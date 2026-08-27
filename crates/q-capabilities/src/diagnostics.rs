//! Build/inspect diagnostics for linked capabilities (M27-009-C).
//!
//! Read-only snapshot of a linked application's capability surface:
//! the resolved pack [`CapabilityInventory`] joined with registered
//! [`CapabilityMetadata`]. Collection never mutates any lifecycle state;
//! missing or version-mismatched SDK metadata fails closed instead of
//! rendering a partial report.

use std::fmt;

use crate::inventory::CapabilityInventory;
use crate::sdk::CapabilityMetadata;

/// One rendered diagnostic row: explicit `id`, version, and SDK summary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityDiagnostic {
    pub id: String,
    pub version: u32,
    pub summary: String,
}

/// Typed diagnostics-collection failures. Closed set.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiagnosticsError {
    /// The inventory links this id but no SDK capability registered it.
    MissingMetadata { id: String },
    /// The registered metadata version disagrees with the linked inventory
    /// version — versioning must be explicit everywhere (ADR-0029 §2).
    VersionMismatch {
        id: String,
        linked: u32,
        declared: u32,
    },
}

impl fmt::Display for DiagnosticsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DiagnosticsError::MissingMetadata { id } => {
                write!(f, "capability '{id}' is linked but has no SDK metadata")
            }
            DiagnosticsError::VersionMismatch {
                id,
                linked,
                declared,
            } => write!(
                f,
                "capability '{id}' version conflict: linked @{linked}, sdk metadata declares @{declared}"
            ),
        }
    }
}

impl std::error::Error for DiagnosticsError {}

/// Read-only inspection snapshot over what is linked.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CapabilityDiagnostics {
    entries: Vec<CapabilityDiagnostic>,
}

impl CapabilityDiagnostics {
    /// Join a resolved inventory with registered SDK metadata. Fails typed
    /// when any linked capability lacks metadata or its versions disagree.
    pub fn collect(
        inventory: &CapabilityInventory,
        registry: &[CapabilityMetadata],
    ) -> Result<Self, DiagnosticsError> {
        let mut entries = Vec::with_capacity(inventory.entries().len());
        for linked in inventory.entries() {
            let meta = registry.iter().find(|m| m.id == linked.id).ok_or_else(|| {
                DiagnosticsError::MissingMetadata {
                    id: linked.id.to_string(),
                }
            })?;
            if meta.version != linked.version {
                return Err(DiagnosticsError::VersionMismatch {
                    id: linked.id.to_string(),
                    linked: linked.version.0,
                    declared: meta.version.0,
                });
            }
            entries.push(CapabilityDiagnostic {
                id: linked.id.to_string(),
                version: linked.version.0,
                summary: meta.summary.clone(),
            });
        }
        Ok(CapabilityDiagnostics { entries })
    }

    /// Diagnostic rows in inventory order (sorted, unique by construction).
    pub fn entries(&self) -> &[CapabilityDiagnostic] {
        &self.entries
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Human-readable rows, one per linked capability.
    pub fn lines(&self) -> Vec<String> {
        self.entries
            .iter()
            .map(|e| format!("{}@{} — {}", e.id, e.version, e.summary))
            .collect()
    }

    /// One-line header summary for build/inspect output.
    pub fn summary(&self) -> String {
        match self.entries.len() {
            0 => "0 capabilities linked".to_string(),
            n => format!("{n} capabilities linked"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inventory::CapabilityInventory;
    use crate::sdk::{CapabilitySdk as _, ExampleSdkCapability};

    #[test]
    fn collect_joins_inventory_with_sdk_metadata() {
        let cap = ExampleSdkCapability::new().unwrap();
        let inventory =
            CapabilityInventory::from_pairs(&[("runtime:test-sdk".to_string(), 1)]).unwrap();
        let diag = CapabilityDiagnostics::collect(&inventory, &[cap.metadata().clone()]).unwrap();

        assert_eq!(diag.entries().len(), 1);
        assert_eq!(diag.entries()[0].id, "runtime:test-sdk");
        assert_eq!(diag.entries()[0].version, 1);
        assert_eq!(
            diag.lines(),
            vec!["runtime:test-sdk@1 — SDK example capability"]
        );
        assert_eq!(diag.summary(), "1 capabilities linked");
        assert!(!diag.is_empty());
    }

    #[test]
    fn version_mismatch_fails_closed() {
        let cap = ExampleSdkCapability::new().unwrap();
        let inventory =
            CapabilityInventory::from_pairs(&[("runtime:test-sdk".to_string(), 2)]).unwrap();
        let err =
            CapabilityDiagnostics::collect(&inventory, &[cap.metadata().clone()]).unwrap_err();
        assert_eq!(
            err,
            DiagnosticsError::VersionMismatch {
                id: "runtime:test-sdk".to_string(),
                linked: 2,
                declared: 1,
            }
        );
    }

    #[test]
    fn missing_metadata_fails_closed() {
        let inventory =
            CapabilityInventory::from_pairs(&[("runtime:other".to_string(), 1)]).unwrap();
        let err = CapabilityDiagnostics::collect(&inventory, &[]).unwrap_err();
        assert_eq!(
            err,
            DiagnosticsError::MissingMetadata {
                id: "runtime:other".to_string(),
            }
        );
    }

    #[test]
    fn empty_inventory_renders_zero_modules() {
        let inventory = CapabilityInventory::from_pairs(&[]).unwrap();
        let diag = CapabilityDiagnostics::collect(&inventory, &[]).unwrap();
        assert!(diag.is_empty());
        assert!(diag.lines().is_empty());
        assert_eq!(diag.summary(), "0 capabilities linked");
    }
}
