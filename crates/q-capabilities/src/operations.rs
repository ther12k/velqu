//! Native operation owner, deadline, and cancellation state
//! (ADR-0030, M27-001-C).
//!
//! Every operation started by a capability has exactly one owner —
//! the invocation that started it, identified by the worker slot and
//! generation (the same pair the request store checks). Settlements
//! are delivered only to the owning generation or dropped as
//! expired. Every operation declares one of exactly two cancellation
//! classes at creation; deadlines are bounded by a fail-closed
//! ceiling.

use std::fmt;

use crate::CapabilityLifecycle;

/// Fail-closed ceiling for operation deadlines (ADR-0030 §2). An
/// operation asking for more than this is a typed rejection, not a
/// clamped value. Raising it is an ADR-level decision.
pub const MAX_OP_DEADLINE_MS: u64 = 300_000;

/// Identifies the invocation that owns an operation: the same
/// slot/generation pair the request store validates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OpOwner {
    pub slot: usize,
    pub generation: u64,
}

/// The two — and only two — cancellation classes (ADR-0028 §5).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CancellationClass {
    /// The host can physically stop the operation mid-flight.
    /// Cancellation is idempotent.
    Cancellable,
    /// The operation is short, bounded, and completes on its own.
    /// This is an explicit, reviewed declaration — never a default.
    NonCancellable,
}

/// Closed operation-state vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OpState {
    /// Started, awaiting settlement, cancellation, or expiry.
    Pending,
    /// Completed normally. Terminal.
    Settled,
    /// Physically stopped (cancellable class only). Terminal.
    Cancelled,
    /// Deadline reached before settlement. Terminal.
    Expired,
}

impl fmt::Display for OpState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            OpState::Pending => "pending",
            OpState::Settled => "settled",
            OpState::Cancelled => "cancelled",
            OpState::Expired => "expired",
        })
    }
}

/// Typed operation violations. Closed set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpError {
    /// The state change is not legal from the current state
    /// (double settle, cancel after settle, …).
    IllegalOpTransition { from: OpState, to: OpState },
    /// A cancel was attempted on a non-cancellable operation.
    NotCancellable,
    /// A settlement was delivered by a non-owner.
    NotOwner,
    /// The operation start was attempted while the capability is not
    /// in `Ready`.
    NotReady { from: crate::CapabilityPhase },
    /// The requested deadline is zero or exceeds the ceiling.
    InvalidDeadline { ms: u64, max: u64 },
}

impl fmt::Display for OpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpError::IllegalOpTransition { from, to } => {
                write!(f, "illegal operation transition {from} -> {to}")
            }
            OpError::NotCancellable => f.write_str("operation is explicitly non-cancellable"),
            OpError::NotOwner => f.write_str("settlement delivered by a non-owner"),
            OpError::NotReady { from } => {
                write!(f, "operations may only start in ready, not in {from}")
            }
            OpError::InvalidDeadline { ms, max } => {
                write!(f, "operation deadline {ms}ms is zero or exceeds {max}ms")
            }
        }
    }
}

impl std::error::Error for OpError {}

/// A native operation: owner, cancellation class, bounded deadline,
/// and state. Terminal states are final; every illegal change is a
/// typed error and never mutates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NativeOp {
    owner: OpOwner,
    class: CancellationClass,
    deadline_ms: u64,
    state: OpState,
}

impl NativeOp {
    /// Start an operation on behalf of `owner`. Fails typed when the
    /// capability is outside `Ready` (guardrail 1) or the deadline is
    /// zero / over the ceiling (bounded everything).
    pub fn start(
        lifecycle: &CapabilityLifecycle,
        owner: OpOwner,
        class: CancellationClass,
        deadline_ms: u64,
    ) -> Result<Self, OpError> {
        if !lifecycle.can_start_ops() {
            return Err(OpError::NotReady {
                from: lifecycle.phase(),
            });
        }
        if deadline_ms == 0 || deadline_ms > MAX_OP_DEADLINE_MS {
            return Err(OpError::InvalidDeadline {
                ms: deadline_ms,
                max: MAX_OP_DEADLINE_MS,
            });
        }
        Ok(NativeOp {
            owner,
            class,
            deadline_ms,
            state: OpState::Pending,
        })
    }

    pub fn owner(&self) -> OpOwner {
        self.owner
    }

    pub fn class(&self) -> CancellationClass {
        self.class
    }

    pub fn deadline_ms(&self) -> u64 {
        self.deadline_ms
    }

    pub fn state(&self) -> OpState {
        self.state
    }

    fn is_terminal(&self) -> bool {
        !matches!(self.state, OpState::Pending)
    }

    /// Deliver a settlement to this operation. Only the owner may
    /// settle; only `Pending` operations accept settlement.
    /// Double-delivery after a cancel/expire is an illegal
    /// transition (typed), keeping settlement exactly-once.
    pub fn settle(&mut self, owner: OpOwner) -> Result<(), OpError> {
        if owner != self.owner {
            return Err(OpError::NotOwner);
        }
        if self.is_terminal() {
            return Err(OpError::IllegalOpTransition {
                from: self.state,
                to: OpState::Settled,
            });
        }
        self.state = OpState::Settled;
        Ok(())
    }

    /// Cancel a pending cancellable operation. Idempotency is an
    /// error, not a no-op: a second cancel is an illegal transition
    /// so double-cancellation is visible in tests and logs.
    pub fn cancel(&mut self) -> Result<(), OpError> {
        if self.class != CancellationClass::Cancellable {
            return Err(OpError::NotCancellable);
        }
        if self.is_terminal() {
            return Err(OpError::IllegalOpTransition {
                from: self.state,
                to: OpState::Cancelled,
            });
        }
        self.state = OpState::Cancelled;
        Ok(())
    }

    /// Mark the operation expired: the deadline fired before
    /// settlement. Applies to both cancellation classes — deadlines
    /// bound every operation.
    pub fn expire(&mut self) -> Result<(), OpError> {
        if self.is_terminal() {
            return Err(OpError::IllegalOpTransition {
                from: self.state,
                to: OpState::Expired,
            });
        }
        self.state = OpState::Expired;
        Ok(())
    }

    /// Settlement delivery for a possibly-expired generation
    /// (ADR-0028 §4): the host routes completion through this; if
    /// the receiving side is no longer pending the result is dropped
    /// as expired and the drop is an expected, successful outcome.
    pub fn deliver_or_drop(&mut self, owner: OpOwner) -> Result<bool, OpError> {
        if owner != self.owner {
            return Err(OpError::NotOwner);
        }
        if self.is_terminal() {
            return Ok(false); // dropped as expired/cancelled/settled
        }
        self.state = OpState::Settled;
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CapabilityPhase;

    fn ready() -> CapabilityLifecycle {
        let mut lc = CapabilityLifecycle::declared();
        lc.install().unwrap();
        lc.activate().unwrap();
        lc
    }

    fn owner(slot: usize, generation: u64) -> OpOwner {
        OpOwner { slot, generation }
    }

    #[test]
    fn start_requires_ready_phase() {
        for phase in [
            CapabilityPhase::Declared,
            CapabilityPhase::Installed,
            CapabilityPhase::Draining,
            CapabilityPhase::Quiesced,
            CapabilityPhase::Failed,
        ] {
            let mut lc = CapabilityLifecycle::declared();
            lc.phase = phase;
            assert_eq!(
                NativeOp::start(&lc, owner(1, 1), CancellationClass::Cancellable, 100),
                Err(OpError::NotReady { from: phase }),
                "phase {phase}"
            );
        }
    }

    #[test]
    fn deadlines_are_bounded_fail_closed() {
        let lc = ready();
        assert_eq!(
            NativeOp::start(&lc, owner(1, 1), CancellationClass::Cancellable, 0),
            Err(OpError::InvalidDeadline {
                ms: 0,
                max: MAX_OP_DEADLINE_MS
            })
        );
        assert_eq!(
            NativeOp::start(
                &lc,
                owner(1, 1),
                CancellationClass::Cancellable,
                MAX_OP_DEADLINE_MS + 1
            ),
            Err(OpError::InvalidDeadline {
                ms: MAX_OP_DEADLINE_MS + 1,
                max: MAX_OP_DEADLINE_MS
            })
        );
        assert!(NativeOp::start(
            &lc,
            owner(1, 1),
            CancellationClass::Cancellable,
            MAX_OP_DEADLINE_MS
        )
        .is_ok());
    }

    #[test]
    fn settle_only_by_owner_only_when_pending() {
        let lc = ready();
        let mut op =
            NativeOp::start(&lc, owner(3, 7), CancellationClass::Cancellable, 100).unwrap();
        assert_eq!(op.settle(owner(3, 8)), Err(OpError::NotOwner));
        assert_eq!(op.settle(owner(4, 7)), Err(OpError::NotOwner));
        assert_eq!(op.settle(owner(3, 7)), Ok(()));
        assert_eq!(op.state(), OpState::Settled);
        // exactly-once: second settle is a typed illegal transition
        assert_eq!(
            op.settle(owner(3, 7)),
            Err(OpError::IllegalOpTransition {
                from: OpState::Settled,
                to: OpState::Settled
            })
        );
    }

    #[test]
    fn cancel_only_cancellable_only_pending() {
        let lc = ready();
        let mut op =
            NativeOp::start(&lc, owner(1, 1), CancellationClass::Cancellable, 100).unwrap();
        assert_eq!(op.cancel(), Ok(()));
        assert_eq!(op.state(), OpState::Cancelled);
        // idempotency is visible: second cancel is typed, not silent
        assert_eq!(
            op.cancel(),
            Err(OpError::IllegalOpTransition {
                from: OpState::Cancelled,
                to: OpState::Cancelled
            })
        );
        assert_eq!(
            op.settle(owner(1, 1)),
            Err(OpError::IllegalOpTransition {
                from: OpState::Cancelled,
                to: OpState::Settled
            })
        );

        let mut nc =
            NativeOp::start(&lc, owner(1, 1), CancellationClass::NonCancellable, 100).unwrap();
        assert_eq!(nc.cancel(), Err(OpError::NotCancellable));
        assert_eq!(nc.state(), OpState::Pending); // unchanged
    }

    #[test]
    fn expiry_applies_to_both_classes() {
        let lc = ready();
        for class in [
            CancellationClass::Cancellable,
            CancellationClass::NonCancellable,
        ] {
            let mut op = NativeOp::start(&lc, owner(1, 1), class, 100).unwrap();
            assert_eq!(op.expire(), Ok(()));
            assert_eq!(op.state(), OpState::Expired);
            assert_eq!(
                op.settle(owner(1, 1)),
                Err(OpError::IllegalOpTransition {
                    from: OpState::Expired,
                    to: OpState::Settled
                })
            );
            assert_eq!(
                op.expire(),
                Err(OpError::IllegalOpTransition {
                    from: OpState::Expired,
                    to: OpState::Expired
                })
            );
        }
    }

    #[test]
    fn expired_generation_settlement_drops_as_expected_outcome() {
        let lc = ready();
        let mut op =
            NativeOp::start(&lc, owner(2, 5), CancellationClass::Cancellable, 100).unwrap();
        // generation expired before settlement arrived
        op.expire().unwrap();
        // host still routes the late completion: dropped, not an error
        assert_eq!(op.deliver_or_drop(owner(2, 5)), Ok(false));
        // wrong owner remains a bug even on the drop path
        assert_eq!(op.deliver_or_drop(owner(2, 6)), Err(OpError::NotOwner));
        // pending delivery settles
        let mut op2 =
            NativeOp::start(&lc, owner(2, 6), CancellationClass::Cancellable, 100).unwrap();
        assert_eq!(op2.deliver_or_drop(owner(2, 6)), Ok(true));
        assert_eq!(op2.state(), OpState::Settled);
    }

    #[test]
    fn terminal_states_reject_every_state_change() {
        let lc = ready();
        let states = [OpState::Settled, OpState::Cancelled, OpState::Expired];
        for target in states {
            let mut op =
                NativeOp::start(&lc, owner(9, 9), CancellationClass::Cancellable, 100).unwrap();
            op.state = target;
            assert_eq!(
                op.settle(owner(9, 9)),
                Err(OpError::IllegalOpTransition {
                    from: target,
                    to: OpState::Settled
                })
            );
            assert_eq!(
                op.cancel(),
                Err(OpError::IllegalOpTransition {
                    from: target,
                    to: OpState::Cancelled
                })
            );
            assert_eq!(
                op.expire(),
                Err(OpError::IllegalOpTransition {
                    from: target,
                    to: OpState::Expired
                })
            );
            assert_eq!(op.state(), target); // unchanged
        }
    }
}
