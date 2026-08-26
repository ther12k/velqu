//! Capability inventory: the canonical, hashable form of a resolved
//! dependency DAG (M27-002-C).
//!
//! The inventory is what enters the application artifact: one entry
//! per linked capability module, id + exact version, sorted by id.
//! Its canonical byte encoding is unambiguous (length-prefixed, no
//! delimiter collisions) so the compiler (TypeScript), the pack
//! verifier (q-pack), and `velqu inspect` all agree on the same
//! bytes and therefore the same sha256.

use std::fmt;

use sha2::{Digest, Sha256};

use crate::identity::{CapabilityId, CapabilityIdError, CapabilityVersion};
use crate::resolver::DependencyDag;

/// One linked capability in the inventory.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct InventoryEntry {
    pub id: CapabilityId,
    pub version: CapabilityVersion,
}

/// The resolved capability inventory for one application. Entries
/// are kept sorted and unique by id — the pack boundary rejects
/// anything else (ADR-0029 §4's uniqueness pin).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CapabilityInventory {
    entries: Vec<InventoryEntry>,
}

impl CapabilityInventory {
    /// Build from a resolved DAG. The DAG is already id-sorted and
    /// duplicate-free by construction, so the mapping is direct.
    pub fn from_dag(dag: &DependencyDag) -> Self {
        CapabilityInventory {
            entries: dag
                .resolved()
                .iter()
                .map(|d| InventoryEntry {
                    id: d.requirement.id.clone(),
                    version: d.requirement.version,
                })
                .collect(),
        }
    }

    pub fn entries(&self) -> &[InventoryEntry] {
        &self.entries
    }

    /// Checked constructor from raw (id, version) pairs — the path
    /// used when reading an inventory off the wire. Parses every id
    /// with the full ADR-0029 validation, sorts by id, and rejects
    /// duplicates (the uniqueness pin). Never trusts caller ordering
    /// or hygiene.
    pub fn from_pairs(pairs: &[(String, u32)]) -> Result<Self, InventoryError> {
        let mut entries: Vec<InventoryEntry> = Vec::with_capacity(pairs.len());
        for (raw, version) in pairs {
            let id = CapabilityId::parse(raw).map_err(InventoryError::InvalidId)?;
            entries.push(InventoryEntry {
                id,
                version: CapabilityVersion(*version),
            });
        }
        entries.sort_by(|a, b| a.id.cmp(&b.id));
        for w in entries.windows(2) {
            if w[0].id == w[1].id {
                return Err(InventoryError::DuplicateId {
                    id: w[0].id.clone(),
                });
            }
        }
        Ok(CapabilityInventory { entries })
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Canonical bytes: `u32-le entry-count`, then per entry
    /// `u16-le id-length, id utf-8 bytes, u32-le version`. Sorted by
    /// id, so the encoding is deterministic for the same set. The
    /// empty inventory is the 4-byte count prefix — NOT the empty
    /// string — so its hash is distinct from "field absent".
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(4 + self.entries.len() * (2 + 16 + 4));
        out.extend_from_slice(&(self.entries.len() as u32).to_le_bytes());
        for e in &self.entries {
            let id = e.id.as_str().as_bytes();
            out.extend_from_slice(&(id.len() as u16).to_le_bytes());
            out.extend_from_slice(id);
            out.extend_from_slice(&e.version.0.to_le_bytes());
        }
        out
    }

    /// sha256 hex over [`Self::canonical_bytes`]. This is the value
    /// the pack carries as `capability_inventory_sha256`.
    pub fn sha256_hex(&self) -> String {
        let digest = Sha256::digest(self.canonical_bytes());
        let mut hex = String::with_capacity(64);
        for b in digest {
            use std::fmt::Write;
            let _ = write!(hex, "{b:02x}");
        }
        hex
    }
}

/// Typed inventory construction failures (wire path).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InventoryError {
    InvalidId(CapabilityIdError),
    DuplicateId { id: CapabilityId },
}

impl fmt::Display for InventoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InventoryError::InvalidId(e) => write!(f, "inventory entry has an invalid id: {e}"),
            InventoryError::DuplicateId { id } => {
                write!(f, "inventory lists capability {id} more than once")
            }
        }
    }
}

impl std::error::Error for InventoryError {}

impl fmt::Display for CapabilityInventory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parts: Vec<String> = self
            .entries
            .iter()
            .map(|e| format!("{}@{}", e.id, e.version))
            .collect();
        write!(f, "[{}]", parts.join(", "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identity::CapabilityRequirement;
    use crate::resolver::resolve_closure;

    fn req(id: &str, version: u32) -> CapabilityRequirement {
        CapabilityRequirement {
            id: CapabilityId::parse(id).unwrap(),
            version: CapabilityVersion(version),
        }
    }

    fn desc(id: &str, version: u32, deps: &[(&str, u32)]) -> crate::identity::CapabilityDescriptor {
        crate::identity::CapabilityDescriptor {
            requirement: req(id, version),
            dependencies: deps.iter().map(|(i, v)| req(i, *v)).collect(),
        }
    }

    #[test]
    fn empty_dag_yields_empty_inventory_with_stable_hash() {
        let dag = resolve_closure(&[], &[desc("runtime:text", 1, &[])]).unwrap();
        let inv = CapabilityInventory::from_dag(&dag);
        assert!(inv.is_empty());
        // canonical bytes = count prefix only; hash is over 4 zero bytes
        assert_eq!(inv.canonical_bytes(), vec![0, 0, 0, 0]);
        assert_eq!(inv.sha256_hex().len(), 64);
        // stable across instances
        assert_eq!(
            inv.sha256_hex(),
            CapabilityInventory::default().sha256_hex()
        );
    }

    #[test]
    fn inventory_from_dag_is_sorted_by_id() {
        let universe = [
            desc("runtime:abort", 1, &[("runtime:text", 2)]),
            desc("runtime:text", 2, &[]),
        ];
        let dag = resolve_closure(&[req("runtime:abort", 1)], &universe).unwrap();
        let inv = CapabilityInventory::from_dag(&dag);
        let ids: Vec<&str> = inv.entries().iter().map(|e| e.id.as_str()).collect();
        assert_eq!(ids, vec!["runtime:abort", "runtime:text"]);
        assert_eq!(inv.entries()[1].version, CapabilityVersion(2));
    }

    #[test]
    fn canonical_bytes_are_deterministic_and_unambiguous() {
        let universe = [
            desc("runtime:abort", 1, &[("runtime:text", 2)]),
            desc("runtime:text", 2, &[]),
        ];
        let dag = resolve_closure(&[req("runtime:abort", 1)], &universe).unwrap();
        let a = CapabilityInventory::from_dag(&dag);
        let b = CapabilityInventory::from_dag(&dag);
        assert_eq!(a.canonical_bytes(), b.canonical_bytes());
        assert_eq!(a.sha256_hex(), b.sha256_hex());
        // encoding internals: count + len-prefixed id + version
        let bytes = a.canonical_bytes();
        assert_eq!(&bytes[0..4], &[2, 0, 0, 0]); // 2 entries
        assert_eq!(&bytes[4..6], &[13, 0]); // "runtime:abort" = 13 bytes
        assert_eq!(&bytes[4 + 2 + 13..4 + 2 + 13 + 4], &[1, 0, 0, 0]); // v1
    }

    #[test]
    fn version_differences_change_the_hash() {
        let d1 = resolve_closure(
            &[req("runtime:timers", 1)],
            &[desc("runtime:timers", 1, &[])],
        )
        .unwrap();
        let d2 = resolve_closure(
            &[req("runtime:timers", 2)],
            &[desc("runtime:timers", 2, &[])],
        )
        .unwrap();
        let h1 = CapabilityInventory::from_dag(&d1).sha256_hex();
        let h2 = CapabilityInventory::from_dag(&d2).sha256_hex();
        assert_ne!(h1, h2, "version must participate in the inventory hash");
    }

    #[test]
    fn different_sets_hash_differently() {
        let d1 = resolve_closure(
            &[req("runtime:timers", 1)],
            &[desc("runtime:timers", 1, &[])],
        )
        .unwrap();
        let d2 =
            resolve_closure(&[req("runtime:text", 1)], &[desc("runtime:text", 1, &[])]).unwrap();
        assert_ne!(
            CapabilityInventory::from_dag(&d1).sha256_hex(),
            CapabilityInventory::from_dag(&d2).sha256_hex()
        );
    }

    #[test]
    fn from_pairs_sorts_parses_and_rejects_duplicates() {
        let inv = CapabilityInventory::from_pairs(&[
            ("runtime:text".to_string(), 2),
            ("runtime:abort".to_string(), 1),
        ])
        .unwrap();
        assert_eq!(
            inv.entries()
                .iter()
                .map(|e| e.id.as_str())
                .collect::<Vec<_>>(),
            vec!["runtime:abort", "runtime:text"]
        );
        // duplicate ids reject with the id named
        let err = CapabilityInventory::from_pairs(&[
            ("runtime:text".to_string(), 1),
            ("runtime:text".to_string(), 2),
        ])
        .unwrap_err();
        assert_eq!(
            err.to_string(),
            "inventory lists capability runtime:text more than once"
        );
        // invalid ids reject with the underlying ADR-0029 error
        assert!(matches!(
            CapabilityInventory::from_pairs(&[("node:fs".to_string(), 1)]),
            Err(InventoryError::InvalidId(_))
        ));
        // reordering does not change the hash: canonical form wins
        let a = CapabilityInventory::from_pairs(&[
            ("runtime:text".to_string(), 1),
            ("runtime:abort".to_string(), 1),
        ])
        .unwrap();
        let b = CapabilityInventory::from_pairs(&[
            ("runtime:abort".to_string(), 1),
            ("runtime:text".to_string(), 1),
        ])
        .unwrap();
        assert_eq!(a.sha256_hex(), b.sha256_hex());
    }

    #[test]
    fn canonical_hash_matches_cross_language_vectors() {
        // Vector pinned against the TypeScript mirror in
        // packages/compiler (capabilityInventoryHash) and the spec:
        // identical bytes from three implementations.
        let empty = CapabilityInventory::default();
        assert_eq!(
            empty.sha256_hex(),
            "df3f619804a92fdb4057192dc43dd748ea778adc52bc498ce80524c014b81119"
        );
        let sample = CapabilityInventory::from_pairs(&[
            ("runtime:abort".to_string(), 1),
            ("runtime:text".to_string(), 2),
        ])
        .unwrap();
        assert_eq!(
            sample.sha256_hex(),
            "3a1b71efeb688d1d032f863fc32c9742fe9d3f54843c377b41cb2c2b5521f69e"
        );
    }

    #[test]
    fn display_is_human_readable() {
        let universe = [
            desc("runtime:abort", 1, &[("runtime:text", 2)]),
            desc("runtime:text", 2, &[]),
        ];
        let dag = resolve_closure(&[req("runtime:abort", 1)], &universe).unwrap();
        let inv = CapabilityInventory::from_dag(&dag);
        assert_eq!(inv.to_string(), "[runtime:abort@1, runtime:text@2]");
    }
}
