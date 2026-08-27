//! Example SDK capability (M27-009-B).
//!
//! Demonstrates external-style capability authoring against the public SDK
//! surface only: explicit versioned metadata, fail-closed lifecycle hooks,
//! and the M27-009-B harness batteries. This compiles as a cargo example —
//! never part of the core library or any production runtime path.

use q_capabilities::harness::{run_expired_drain, run_full_lifecycle};
use q_capabilities::sdk::{CancellableCapability, CapabilityMetadata, CapabilitySdk};

/// A first-party/external capability implementing the SDK traits without
/// touching internal runtime state.
struct GreeterCapability {
    meta: CapabilityMetadata,
}

impl GreeterCapability {
    fn new() -> Result<Self, String> {
        Ok(GreeterCapability {
            meta: CapabilityMetadata::new("runtime:example", 1, "example greeter capability")?,
        })
    }
}

impl CapabilitySdk for GreeterCapability {
    fn metadata(&self) -> &CapabilityMetadata {
        &self.meta
    }

    /// Cleanup on graceful shutdown: close handles, flush buffers. An error
    /// here fails shutdown closed — it is reported, never swallowed.
    fn on_shutdown(&self) -> Result<(), String> {
        Ok(())
    }
}

impl CancellableCapability for GreeterCapability {}

fn main() {
    let cap = GreeterCapability::new().expect("valid example capability metadata");
    println!("metadata: {}", cap.metadata());

    let graceful = run_full_lifecycle(&cap).expect("graceful lifecycle battery");
    println!("graceful: {graceful:?}");
    assert_eq!(graceful.id, "runtime:example");
    assert_eq!(graceful.version, 1);
    assert_eq!(graceful.drain_outcome, "quiesced");
    assert_eq!(
        graceful.terminal_phase,
        q_capabilities::CapabilityPhase::Quiesced
    );

    let expired = run_expired_drain(&cap).expect("expired drain battery");
    println!("expired:  {expired:?}");
    assert_eq!(expired.drain_outcome, "deadline-exceeded");
    assert_eq!(
        expired.terminal_phase,
        q_capabilities::CapabilityPhase::Failed
    );

    q_capabilities::harness::assert_ops_gate_fails_closed()
        .expect("operations outside Ready must fail typed");
    println!("ops gate: fails closed outside Ready ✓");

    // M27-009-C: build/inspect diagnostics — join the resolved inventory
    // with SDK metadata into a read-only snapshot; no runtime mutation.
    let inventory =
        q_capabilities::CapabilityInventory::from_pairs(&[("runtime:example".to_string(), 1)])
            .expect("valid example inventory");
    let registry = vec![cap.metadata().clone()];
    let diag = q_capabilities::CapabilityDiagnostics::collect(&inventory, &registry)
        .expect("inspect surface resolves");
    println!("inspect: {}", diag.summary());
    for line in diag.lines() {
        println!("inspect: {line}");
    }
}
