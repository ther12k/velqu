//! Bounded capability shutdown and quiescence (ADR-0031, M27-001-D).
//!
//! Shutdown moves a capability from `Ready` to `Draining`, cancels
//! pending cancellable operations, awaits pending non-cancellable
//! ones, and reaches `Quiesced` only if everything settles within the
//! budget. A missed budget expires the remaining operations and
//! routes the lifecycle to `Failed` — the runtime never exits with
//! silently abandoned operations. The model is deterministic: the
//! host tells it whether the deadline fired; it never reads a clock.

use std::fmt;

use crate::operations::CancellationClass;
use crate::operations::NativeOp;
use crate::operations::OpState;
use crate::CapabilityLifecycle;
use crate::CapabilityPhase;
use crate::LifecycleError;

/// Fail-closed shutdown budget (ADR-0031 §1). Reaching quiescence
/// after this is a `DeadlineExceeded` failure, not a late success.
/// Moving the ceiling is an ADR-level decision.
pub const SHUTDOWN_BUDGET_MS: u64 = 5_000;

/// Typed shutdown violations. Closed set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShutdownError {
    /// Wraps lifecycle violations (e.g. draining a capability that
    /// never served, or draining twice).
    Lifecycle(LifecycleError),
}

impl fmt::Display for ShutdownError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShutdownError::Lifecycle(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for ShutdownError {}

/// Result of a completed shutdown attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrainOutcome {
    /// Everything settled within the budget. Accounting names what
    /// happened to each operation.
    Quiesced {
        cancelled: usize,
        settled: usize,
        expired: usize,
    },
    /// The budget fired with operations still pending. Fail-closed:
    /// the remaining operations are expired and the lifecycle is
    /// `Failed`. `pending` counts what was abandoned.
    DeadlineExceeded { pending: usize },
}

impl fmt::Display for DrainOutcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DrainOutcome::Quiesced {
                cancelled,
                settled,
                expired,
            } => write!(
                f,
                "quiesced (cancelled {cancelled}, settled {settled}, expired {expired})"
            ),
            DrainOutcome::DeadlineExceeded { pending } => {
                write!(
                    f,
                    "shutdown budget exceeded with {pending} pending operation(s); failed closed"
                )
            }
        }
    }
}

/// Begin shutdown: `Ready -> Draining`. New operations are refused
/// from this point (the lifecycle guard rejects every start).
pub fn begin_shutdown(
    lifecycle: &mut CapabilityLifecycle,
) -> Result<CapabilityPhase, ShutdownError> {
    lifecycle.begin_drain().map_err(ShutdownError::Lifecycle)
}

/// One drain step over the operation set: every pending cancellable
/// operation is cancelled immediately; pending non-cancellable
/// operations are left to complete on their own (the host settles or
/// expires them before `finish_shutdown`). Returns the number of
/// still-pending operations — the await set.
pub fn drain_step(ops: &mut [NativeOp]) -> usize {
    let mut pending = 0;
    for op in ops.iter_mut() {
        if op.state() != OpState::Pending {
            continue;
        }
        match op.class() {
            CancellationClass::Cancellable => {
                // Pending + cancellable: cancellation is always legal
                // here by construction.
                let _ = op.cancel();
            }
            CancellationClass::NonCancellable => pending += 1,
        }
    }
    pending
}

/// Finish shutdown. Call after the host either settled the await set
/// (`deadline_fired = false`) or observed the budget expiry
/// (`deadline_fired = true`).
///
/// - No pending operations: `Draining -> Quiesced`, success
///   accounting returned.
/// - Pending operations and the deadline fired: the stragglers are
///   expired and the lifecycle fails closed (`Failed`); the outcome
///   is `DeadlineExceeded` — never a late `Quiesced`.
pub fn finish_shutdown(
    lifecycle: &mut CapabilityLifecycle,
    ops: &mut [NativeOp],
    deadline_fired: bool,
) -> Result<DrainOutcome, ShutdownError> {
    let pending = ops.iter().filter(|o| o.state() == OpState::Pending).count();
    if pending == 0 {
        let cancelled = ops
            .iter()
            .filter(|o| o.state() == OpState::Cancelled)
            .count();
        let settled = ops.iter().filter(|o| o.state() == OpState::Settled).count();
        let expired = ops.iter().filter(|o| o.state() == OpState::Expired).count();
        lifecycle.quiesce().map_err(ShutdownError::Lifecycle)?;
        return Ok(DrainOutcome::Quiesced {
            cancelled,
            settled,
            expired,
        });
    }
    if deadline_fired {
        for op in ops.iter_mut() {
            if op.state() == OpState::Pending {
                let _ = op.expire();
            }
        }
        let _ = lifecycle.fail();
        return Ok(DrainOutcome::DeadlineExceeded { pending });
    }
    // The host must not call finish with pending work and no deadline:
    // that state has no honest outcome.
    Err(ShutdownError::Lifecycle(
        LifecycleError::IllegalTransition {
            from: CapabilityPhase::Draining,
            to: CapabilityPhase::Quiesced,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::operations::OpOwner;

    fn ready_lifecycle() -> CapabilityLifecycle {
        let mut lc = CapabilityLifecycle::declared();
        lc.install().unwrap();
        lc.activate().unwrap();
        lc
    }

    fn owner(n: usize) -> OpOwner {
        OpOwner {
            slot: n,
            generation: n as u64,
        }
    }

    #[test]
    fn all_cancellable_operations_drain_to_quiesced() {
        let mut lc = ready_lifecycle();
        let mut ops = [
            NativeOp::start(&lc, owner(1), CancellationClass::Cancellable, 100).unwrap(),
            NativeOp::start(&lc, owner(2), CancellationClass::Cancellable, 100).unwrap(),
        ];
        assert_eq!(begin_shutdown(&mut lc), Ok(CapabilityPhase::Draining));
        assert_eq!(drain_step(&mut ops), 0);
        let outcome = finish_shutdown(&mut lc, &mut ops, false).unwrap();
        assert_eq!(
            outcome,
            DrainOutcome::Quiesced {
                cancelled: 2,
                settled: 0,
                expired: 0
            }
        );
        assert_eq!(lc.phase(), CapabilityPhase::Quiesced);
        assert_eq!(ops[0].state(), OpState::Cancelled);
        assert_eq!(ops[1].state(), OpState::Cancelled);
    }

    #[test]
    fn non_cancellable_operations_settle_within_budget() {
        let mut lc = ready_lifecycle();
        let mut ops = [
            NativeOp::start(&lc, owner(1), CancellationClass::Cancellable, 100).unwrap(),
            NativeOp::start(&lc, owner(2), CancellationClass::NonCancellable, 100).unwrap(),
            NativeOp::start(&lc, owner(3), CancellationClass::NonCancellable, 100).unwrap(),
        ];
        begin_shutdown(&mut lc).unwrap();
        assert_eq!(drain_step(&mut ops), 2); // the await set
        ops[1].settle(owner(2)).unwrap();
        ops[2].settle(owner(3)).unwrap();
        let outcome = finish_shutdown(&mut lc, &mut ops, false).unwrap();
        assert_eq!(
            outcome,
            DrainOutcome::Quiesced {
                cancelled: 1,
                settled: 2,
                expired: 0
            }
        );
        assert_eq!(lc.phase(), CapabilityPhase::Quiesced);
    }

    #[test]
    fn missed_budget_expires_stragglers_and_fails_closed() {
        let mut lc = ready_lifecycle();
        let mut ops = [
            NativeOp::start(&lc, owner(1), CancellationClass::Cancellable, 100).unwrap(),
            NativeOp::start(&lc, owner(2), CancellationClass::NonCancellable, 100).unwrap(),
        ];
        begin_shutdown(&mut lc).unwrap();
        assert_eq!(drain_step(&mut ops), 1);
        // budget fires before op 2 settles
        let outcome = finish_shutdown(&mut lc, &mut ops, true).unwrap();
        assert_eq!(outcome, DrainOutcome::DeadlineExceeded { pending: 1 });
        assert_eq!(lc.phase(), CapabilityPhase::Failed); // fail closed
        assert_eq!(ops[1].state(), OpState::Expired); // abandoned visibly
        assert_eq!(ops[0].state(), OpState::Cancelled);
        // a late settlement for the expired op drops, quiesce never happens
        assert_eq!(ops[1].deliver_or_drop(owner(2)), Ok(false));
        assert_eq!(
            lc.quiesce(),
            Err(LifecycleError::Terminal {
                from: CapabilityPhase::Failed
            })
        );
    }

    #[test]
    fn empty_operation_set_quiesces_immediately() {
        let mut lc = ready_lifecycle();
        begin_shutdown(&mut lc).unwrap();
        let mut ops: [NativeOp; 0] = [];
        assert_eq!(
            finish_shutdown(&mut lc, &mut ops, false).unwrap(),
            DrainOutcome::Quiesced {
                cancelled: 0,
                settled: 0,
                expired: 0
            }
        );
        assert_eq!(lc.phase(), CapabilityPhase::Quiesced);
    }

    #[test]
    fn draining_refuses_new_operations() {
        let mut lc = ready_lifecycle();
        begin_shutdown(&mut lc).unwrap();
        assert_eq!(
            NativeOp::start(&lc, owner(9), CancellationClass::Cancellable, 100),
            Err(crate::operations::OpError::NotReady {
                from: CapabilityPhase::Draining
            })
        );
    }

    #[test]
    fn shutdown_requires_ready_lifecycle() {
        let mut lc = CapabilityLifecycle::declared();
        lc.install().unwrap();
        assert_eq!(
            begin_shutdown(&mut lc),
            Err(ShutdownError::Lifecycle(
                LifecycleError::IllegalTransition {
                    from: CapabilityPhase::Installed,
                    to: CapabilityPhase::Draining
                }
            ))
        );
        // double shutdown is equally illegal
        let mut lc2 = ready_lifecycle();
        begin_shutdown(&mut lc2).unwrap();
        assert_eq!(
            begin_shutdown(&mut lc2),
            Err(ShutdownError::Lifecycle(
                LifecycleError::IllegalTransition {
                    from: CapabilityPhase::Draining,
                    to: CapabilityPhase::Draining
                }
            ))
        );
    }

    #[test]
    fn finish_with_pending_and_no_deadline_has_no_honest_outcome() {
        let mut lc = ready_lifecycle();
        let mut ops =
            [NativeOp::start(&lc, owner(1), CancellationClass::NonCancellable, 100).unwrap()];
        begin_shutdown(&mut lc).unwrap();
        drain_step(&mut ops);
        // host bug: pending work, budget not observed
        assert!(matches!(
            finish_shutdown(&mut lc, &mut ops, false),
            Err(ShutdownError::Lifecycle(_))
        ));
        assert_eq!(lc.phase(), CapabilityPhase::Draining); // unchanged
    }
}
