//! Rust-side capability SDK traits and metadata (M27-009-A).
//!
//! First-party and external capabilities implement these traits without mutating
//! internal runtime state. All traits compose with `CapabilityLifecycle` (ADR-0028)
//! and enforce explicit versioning, read-only metadata access, and typed lifecycle hooks.

use std::fmt;

use crate::identity::{CapabilityId, CapabilityVersion};
use crate::shutdown::{begin_shutdown, finish_shutdown};
use crate::CapabilityLifecycle;

/// Static metadata describing a linked capability module.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityMetadata {
    pub id: CapabilityId,
    pub version: CapabilityVersion,
    /// Short description of the exposed surface.
    pub summary: String,
}

impl fmt::Display for CapabilityMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}@{} — {}", self.id, self.version, self.summary)
    }
}

impl CapabilityMetadata {
    /// Create new capability metadata from a validated id.
    pub fn new(id: &str, version: u32, summary: &str) -> Result<Self, String> {
        Ok(CapabilityMetadata {
            id: CapabilityId::parse(id).map_err(|e| e.to_string())?,
            version: CapabilityVersion(version),
            summary: summary.to_string(),
        })
    }
}

/// Context provided to a capability's lifecycle operations.
///
/// Read-only; capabilities cannot mutate internal runtime state through this
/// interface. Arbitrary mutable application state is not surfaced here.
#[derive(Debug)]
pub struct LifecycleContext<'a> {
    pub lifecycle: &'a mut CapabilityLifecycle,
}

/// Core trait every capability must implement to participate in the SDK.
pub trait CapabilitySdk: Send + Sync {
    /// Metadata identifying this capability.
    fn metadata(&self) -> &CapabilityMetadata;

    /// Callback invoked on graceful shutdown via ADR-0031 drain protocol.
    /// Returns Err on cleanup failure — shutdown fails closed.
    fn on_shutdown(&self) -> Result<(), String> {
        Ok(())
    }

    /// Callback invoked when this capability is being abandoned during quarantine/drain-deadline expiry.
    /// Returns Err if the resource cleanup itself failed; such failures are reported, never silently swallowed.
    fn on_failure(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Trait for cancellation-aware capabilities implementing a bounded drain path
/// and enforced lifecycle transitions per ADR-0028 §6 / ADR-0031.
pub trait CancellableCapability: CapabilitySdk {
    /// Begin draining. The default implementation drives the bounded-shutdown protocol
    /// over the capability lifecycle with no pending operation set.
    fn begin_shutdown_drain(
        &self,
        lifecycle: &mut CapabilityLifecycle,
        deadline_fired: bool,
    ) -> Result<crate::shutdown::DrainOutcome, String> {
        begin_shutdown(lifecycle).map_err(|e| e.to_string())?;
        let mut ops: [crate::operations::NativeOp; 0] = [];
        let outcome =
            finish_shutdown(lifecycle, &mut ops, deadline_fired).map_err(|e| e.to_string())?;
        self.on_shutdown()?;
        Ok(outcome)
    }
}

/// A simple example implementation used for SDK compatibility testing (kept out of runtime paths).
pub struct ExampleSdkCapability {
    meta: CapabilityMetadata,
}

impl ExampleSdkCapability {
    pub fn new() -> Result<Self, String> {
        Ok(ExampleSdkCapability {
            meta: CapabilityMetadata::new("runtime:test-sdk", 1, "SDK example capability")?,
        })
    }
}

impl Default for ExampleSdkCapability {
    fn default() -> Self {
        Self::new().expect("valid test SDK capability")
    }
}

impl CapabilitySdk for ExampleSdkCapability {
    fn metadata(&self) -> &CapabilityMetadata {
        &self.meta
    }
}

impl CancellableCapability for ExampleSdkCapability {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_capability_metadata_is_explicit() {
        let cap = ExampleSdkCapability::new().unwrap();
        assert_eq!(cap.metadata().id.as_str(), "runtime:test-sdk");
        assert_eq!(cap.metadata().version.0, 1);
        assert_eq!(cap.metadata().summary, "SDK example capability");

        // Versioning is part of Display output
        let display = cap.metadata().to_string();
        assert!(display.contains("runtime:test-sdk@1"), "{display}");
    }

    #[test]
    fn invalid_id_in_metadata_fails_closed() {
        // Invalid namespace rejected
        assert!(CapabilityMetadata::new("node:fs", 1, "x").is_err());
        // Malformed name rejected
        assert!(CapabilityMetadata::new("", 1, "x").is_err());
        assert!(CapabilityMetadata::new("runtime:", 1, "x").is_err());
    }

    #[test]
    fn cancellable_capability_drains_to_quiesced() {
        let mut lc = CapabilityLifecycle::declared();
        lc.install().unwrap();
        lc.activate().unwrap();

        let cap = ExampleSdkCapability::new().unwrap();
        let outcome = cap.begin_shutdown_drain(&mut lc, false).unwrap();
        assert!(matches!(
            outcome,
            crate::shutdown::DrainOutcome::Quiesced { .. }
        ));
        assert_eq!(
            lc.phase(),
            crate::CapabilityPhase::Quiesced,
            "quiescence is required"
        );
    }
}
