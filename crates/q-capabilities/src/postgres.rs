//! Postgres capability ABI model (BETA-004-A).
//!
//! First-party database capability (`runtime:postgres`) expressed with
//! the capability ABI: identity, lazy lifecycle, and the bounded
//! operation surface every query/transaction will use. This module is
//! the *ABI contract only* — no wire protocol, no connection pool, and
//! no I/O live here (BETA-004-B/E own the pool; the protocol lives with
//! the native module, outside core). An application that never grants
//! `postgres` never links this capability: nothing here is constructed
//! at pack load, and the capability table of an unrelated app stays
//! empty (G-004, "real database story without enlarging core").
//!
//! Rules inherited from the ABI (ADR-0028/0029/0030):
//! - Work starts only in [`CapabilityPhase::Ready`].
//! - Nothing initializes until first demand after activation.
//! - Every query op declares an owner and a fail-closed deadline
//!   ceiling; cancellation is a reviewed per-op class.
//! - Terminal phases are terminal.

use std::fmt;

use crate::identity::{CapabilityId, CapabilityVersion};
use crate::operations::{CancellationClass, NativeOp, OpError, OpOwner};
use crate::{CapabilityLifecycle, CapabilityPhase, LifecycleError};

/// Capability identity. `runtime` is the only namespace in the closed
/// vocabulary; the database capability is a runtime capability, not an
/// ambient service.
pub const POSTGRES_CAPABILITY_ID: &str = "runtime:postgres";

/// Exact capability version required by packs (ADR-0029: exact-match
/// requirements; no implicit relaxed policy).
pub const POSTGRES_CAPABILITY_VERSION: u32 = 1;

/// Fail-closed ceiling for a single Postgres operation deadline.
/// Stricter than the ABI-wide [`MAX_OP_DEADLINE_MS`]: a database round
/// trip beyond two minutes is a configuration error, not a long query.
pub const MAX_POSTGRES_OP_DEADLINE_MS: u64 = 120_000;

/// Typed Postgres-capability errors. Closed set; the host redacts
/// before anything reaches a response.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PostgresCapabilityError {
    /// Operation start outside `Ready` (ABI lifecycle violation).
    Lifecycle(LifecycleError),
    /// Deadline 0 or above the Postgres ceiling.
    DeadlineOutOfRange { max: u64 },
    /// Op-level typed error from the shared operations model.
    Op(OpError),
}

impl fmt::Display for PostgresCapabilityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PostgresCapabilityError::Lifecycle(e) => write!(f, "postgres capability: {e}"),
            PostgresCapabilityError::DeadlineOutOfRange { max } => {
                write!(f, "postgres capability: deadline exceeds ceiling {max}ms")
            }
            PostgresCapabilityError::Op(e) => write!(f, "postgres capability: {e}"),
        }
    }
}

impl std::error::Error for PostgresCapabilityError {}

/// Parsed, validated `runtime:postgres` identity.
pub fn postgres_capability_id() -> CapabilityId {
    // Unwrappable by construction: the constant is validated by
    // `postgres_identity_parses` below.
    CapabilityId::parse(POSTGRES_CAPABILITY_ID).expect("built-in postgres capability id is valid")
}

/// Exact-match requirement as a pack manifest would carry it.
pub fn postgres_requirement() -> (CapabilityId, CapabilityVersion) {
    (
        postgres_capability_id(),
        CapabilityVersion(POSTGRES_CAPABILITY_VERSION),
    )
}

/// ABI-level state of the `runtime:postgres` capability. Lazy by
/// construction: the host creates the value only on first demand after
/// linking — nothing in this module initializes at pack load or
/// process start.
pub struct PostgresCapability {
    pub lifecycle: CapabilityLifecycle,
}

impl Default for PostgresCapability {
    fn default() -> Self {
        Self::declared()
    }
}

impl PostgresCapability {
    /// Manifest-declared state: nothing linked yet.
    pub fn declared() -> Self {
        PostgresCapability {
            lifecycle: CapabilityLifecycle::declared(),
        }
    }

    pub fn install(&mut self) -> Result<CapabilityPhase, PostgresCapabilityError> {
        self.lifecycle
            .install()
            .map_err(PostgresCapabilityError::Lifecycle)
    }

    /// First demand after activation lazily initializes here.
    pub fn activate(&mut self) -> Result<CapabilityPhase, PostgresCapabilityError> {
        self.lifecycle
            .activate()
            .map_err(PostgresCapabilityError::Lifecycle)
    }

    pub fn begin_drain(&mut self) -> Result<CapabilityPhase, PostgresCapabilityError> {
        self.lifecycle
            .begin_drain()
            .map_err(PostgresCapabilityError::Lifecycle)
    }

    pub fn quiesce(&mut self) -> Result<CapabilityPhase, PostgresCapabilityError> {
        self.lifecycle
            .quiesce()
            .map_err(PostgresCapabilityError::Lifecycle)
    }

    pub fn fail(&mut self) -> Result<CapabilityPhase, PostgresCapabilityError> {
        self.lifecycle
            .fail()
            .map_err(PostgresCapabilityError::Lifecycle)
    }

    /// Start a query operation: gated on `Ready` (delegated to
    /// [`NativeOp::start`] via the lifecycle), owner-tagged, bounded by
    /// the stricter Postgres deadline ceiling. Every query is
    /// cancellable: deadlines must be able to stop the round trip and
    /// release the connection safely (BETA-004-D). Parameterization
    /// itself is a query-text property owned by the protocol layer
    /// (BETA-004-C); the ABI only carries the bound.
    pub fn start_query(
        &self,
        owner: OpOwner,
        deadline_ms: u64,
    ) -> Result<NativeOp, PostgresCapabilityError> {
        if deadline_ms == 0 || deadline_ms > MAX_POSTGRES_OP_DEADLINE_MS {
            return Err(PostgresCapabilityError::DeadlineOutOfRange {
                max: MAX_POSTGRES_OP_DEADLINE_MS,
            });
        }
        NativeOp::start(
            &self.lifecycle,
            owner,
            CancellationClass::Cancellable,
            deadline_ms,
        )
        .map_err(PostgresCapabilityError::Op)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::operations::{OpState, MAX_OP_DEADLINE_MS};

    #[test]
    fn postgres_identity_parses() {
        let id = postgres_capability_id();
        assert_eq!(id.as_str(), "runtime:postgres");
    }

    #[test]
    fn requirement_is_exact_version_one() {
        let (id, version) = postgres_requirement();
        assert_eq!(id.as_str(), "runtime:postgres");
        assert_eq!(version.0, POSTGRES_CAPABILITY_VERSION);
        assert_eq!(POSTGRES_CAPABILITY_VERSION, 1);
    }

    #[test]
    fn lazy_until_installed_and_activated() {
        let cap = PostgresCapability::declared();
        assert_eq!(cap.lifecycle.phase(), CapabilityPhase::Declared);
        assert!(!cap.lifecycle.can_start_ops());
        // nothing initialized at declared: a query attempt is a typed
        // NotReady rejection, never a silent init
        let err = cap
            .start_query(
                OpOwner {
                    slot: 0,
                    generation: 1,
                },
                1_000,
            )
            .unwrap_err();
        assert!(matches!(
            err,
            PostgresCapabilityError::Op(OpError::NotReady { .. })
        ));
    }

    #[test]
    fn queries_start_only_in_ready() {
        let mut cap = PostgresCapability::declared();
        cap.install().unwrap();
        let err = cap
            .start_query(
                OpOwner {
                    slot: 0,
                    generation: 1,
                },
                1_000,
            )
            .unwrap_err();
        assert!(matches!(
            err,
            PostgresCapabilityError::Op(OpError::NotReady { .. })
        ));
        cap.activate().unwrap();
        let op = cap
            .start_query(
                OpOwner {
                    slot: 0,
                    generation: 1,
                },
                1_000,
            )
            .unwrap();
        assert_eq!(op.state(), OpState::Pending);
    }

    #[test]
    fn query_deadlines_are_bounded_by_the_postgres_ceiling() {
        let mut cap = PostgresCapability::declared();
        cap.install().unwrap();
        cap.activate().unwrap();
        let err = cap
            .start_query(
                OpOwner {
                    slot: 0,
                    generation: 1,
                },
                0,
            )
            .unwrap_err();
        assert!(matches!(
            err,
            PostgresCapabilityError::DeadlineOutOfRange { .. }
        ));
        let err = cap
            .start_query(
                OpOwner {
                    slot: 0,
                    generation: 1,
                },
                MAX_POSTGRES_OP_DEADLINE_MS + 1,
            )
            .unwrap_err();
        assert!(matches!(
            err,
            PostgresCapabilityError::DeadlineOutOfRange { max: 120_000 }
        ));
        // the Postgres ceiling is stricter than the ABI-wide ceiling
        const {
            assert!(MAX_POSTGRES_OP_DEADLINE_MS < MAX_OP_DEADLINE_MS);
        }
    }

    #[test]
    fn every_query_op_is_cancellable_and_settles_through_the_closed_states() {
        let mut cap = PostgresCapability::declared();
        cap.install().unwrap();
        cap.activate().unwrap();
        let owner = OpOwner {
            slot: 1,
            generation: 2,
        };

        let mut op = cap.start_query(owner, 1_000).unwrap();
        assert_eq!(op.class(), CancellationClass::Cancellable);
        op.cancel().unwrap();
        assert_eq!(op.state(), OpState::Cancelled);
        // terminal: cancelling again is a typed error, not a silent no-op
        assert!(op.cancel().is_err());

        let mut op = cap.start_query(owner, 1_000).unwrap();
        op.expire().unwrap();
        assert_eq!(op.state(), OpState::Expired);

        let mut op = cap.start_query(owner, 1_000).unwrap();
        op.settle(owner).unwrap();
        assert_eq!(op.state(), OpState::Settled);
        // settlement is exactly-once: a second delivery is typed
        assert!(op.settle(owner).is_err());
    }

    #[test]
    fn drain_then_quiesce_then_terminal() {
        let mut cap = PostgresCapability::declared();
        cap.install().unwrap();
        cap.activate().unwrap();
        cap.begin_drain().unwrap();
        // no new operations in draining
        assert!(cap
            .start_query(
                OpOwner {
                    slot: 0,
                    generation: 1
                },
                1_000
            )
            .is_err());
        cap.quiesce().unwrap();
        assert!(cap.fail().is_err());
    }
}
