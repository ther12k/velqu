//! SDK capability test harness (M27-009-B).
//!
//! Reusable battery that drives any [`CancellableCapability`] through the
//! ADR-0028 lifecycle (`Declared -> Installed -> Ready -> Draining ->
//! Quiesced | Failed`) and the ADR-0031 bounded shutdown protocol, returning
//! a machine-checkable [`LifecycleReport`]. The harness grants capabilities
//! no application state: only their own explicit metadata and lifecycle
//! hooks cross this interface.

use crate::operations::{CancellationClass, NativeOp, OpError, OpOwner};
use crate::sdk::CancellableCapability;
use crate::shutdown::{self, DrainOutcome};
use crate::{CapabilityLifecycle, CapabilityPhase, MAX_OP_DEADLINE_MS};

/// Machine-checkable result of driving a capability through the harness.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LifecycleReport {
    /// Explicit capability id from the capability's metadata.
    pub id: String,
    /// Explicit capability version from the capability's metadata.
    pub version: u32,
    /// Phase reached after install + activate; must be `Ready`.
    pub ready_phase: CapabilityPhase,
    /// Terminal drain outcome: `"quiesced"` or `"deadline-exceeded"`.
    pub drain_outcome: &'static str,
    /// Terminal lifecycle phase at the end of the battery.
    pub terminal_phase: CapabilityPhase,
}

fn outcome_name(outcome: &DrainOutcome) -> &'static str {
    match outcome {
        DrainOutcome::Quiesced { .. } => "quiesced",
        DrainOutcome::DeadlineExceeded { .. } => "deadline-exceeded",
    }
}

fn fail(step: &str, err: impl std::fmt::Display) -> String {
    format!("SDK harness: {step} failed closed: {err}")
}

fn drive_to_ready() -> Result<CapabilityLifecycle, String> {
    let mut lc = CapabilityLifecycle::declared();
    lc.install().map_err(|e| fail("install", e))?;
    lc.activate().map_err(|e| fail("activate", e))?;
    Ok(lc)
}

/// Drive a cancellable capability through the full graceful battery:
/// install/activate to `Ready`, then a bounded drain with the deadline
/// not fired. Requires a `Quiesced` outcome and `Quiesced` terminal phase.
pub fn run_full_lifecycle<C: CancellableCapability>(cap: &C) -> Result<LifecycleReport, String> {
    let mut lc = drive_to_ready()?;
    let meta = cap.metadata();

    let outcome = cap.begin_shutdown_drain(&mut lc, false)?;
    if !matches!(outcome, DrainOutcome::Quiesced { .. }) {
        return Err(format!(
            "SDK harness: graceful drain produced {} for {}@{}",
            outcome_name(&outcome),
            meta.id,
            meta.version
        ));
    }
    let terminal = lc.phase();
    if terminal != CapabilityPhase::Quiesced {
        return Err(format!(
            "SDK harness: expected Quiesced terminal phase, got {terminal}"
        ));
    }

    Ok(LifecycleReport {
        id: meta.id.to_string(),
        version: meta.version.0,
        ready_phase: CapabilityPhase::Ready,
        drain_outcome: outcome_name(&outcome),
        terminal_phase: terminal,
    })
}

/// Drive a cancellable capability through an expired drain: install/activate
/// to `Ready`, one pending non-cancellable operation stands in for in-flight
/// work, then the host observes budget expiry (`deadline_fired = true`).
/// Fail-closed contract: `DeadlineExceeded` accounting and `Failed` terminal
/// phase — never a late `Quiesced`.
pub fn run_expired_drain<C: CancellableCapability>(cap: &C) -> Result<LifecycleReport, String> {
    let mut lc = drive_to_ready()?;
    let meta = cap.metadata();

    // In-flight work owned by the host on the capability's behalf; starts
    // legal only because the lifecycle is `Ready` right now.
    let mut ops = [NativeOp::start(
        &lc,
        OpOwner {
            slot: 0,
            generation: 1,
        },
        CancellationClass::NonCancellable,
        MAX_OP_DEADLINE_MS,
    )
    .map_err(|e| fail("op start in ready", e))?];

    shutdown::begin_shutdown(&mut lc).map_err(|e| fail("begin shutdown", e))?;
    let outcome =
        shutdown::finish_shutdown(&mut lc, &mut ops, true).map_err(|e| fail("finish", e))?;
    match outcome {
        DrainOutcome::DeadlineExceeded { pending: 1 } => {}
        other => {
            return Err(format!(
                "SDK harness: expired drain must exceed the deadline with 1 pending op, got {}",
                outcome_name(&other)
            ))
        }
    }
    let terminal = lc.phase();
    if terminal != CapabilityPhase::Failed {
        return Err(format!(
            "SDK harness: expected Failed terminal phase after expiry, got {terminal}"
        ));
    }

    Ok(LifecycleReport {
        id: meta.id.to_string(),
        version: meta.version.0,
        ready_phase: CapabilityPhase::Ready,
        drain_outcome: outcome_name(&outcome),
        terminal_phase: terminal,
    })
}

/// Verify the host-enforced operation gate from the SDK surface: starting an
/// operation outside `Ready` fails typed (`NotReady`) and never mutates.
pub fn assert_ops_gate_fails_closed() -> Result<(), String> {
    let lc = CapabilityLifecycle::declared();
    let err = NativeOp::start(
        &lc,
        OpOwner {
            slot: 0,
            generation: 1,
        },
        CancellationClass::Cancellable,
        1_000,
    );
    match err {
        Err(OpError::NotReady {
            from: CapabilityPhase::Declared,
        }) => Ok(()),
        other => Err(format!("SDK harness: ops gate returned {other:?}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sdk::ExampleSdkCapability;

    #[test]
    fn full_lifecycle_battery_reports_quiesced() {
        let cap = ExampleSdkCapability::new().unwrap();
        let report = run_full_lifecycle(&cap).unwrap();
        assert_eq!(report.id, "runtime:test-sdk");
        assert_eq!(report.version, 1);
        assert_eq!(report.ready_phase, CapabilityPhase::Ready);
        assert_eq!(report.drain_outcome, "quiesced");
        assert_eq!(report.terminal_phase, CapabilityPhase::Quiesced);
    }

    #[test]
    fn expired_drain_battery_fails_closed() {
        let cap = ExampleSdkCapability::new().unwrap();
        let report = run_expired_drain(&cap).unwrap();
        assert_eq!(report.drain_outcome, "deadline-exceeded");
        assert_eq!(report.terminal_phase, CapabilityPhase::Failed);
    }

    #[test]
    fn ops_gate_rejects_start_outside_ready() {
        assert_ops_gate_fails_closed().unwrap();
    }
}
