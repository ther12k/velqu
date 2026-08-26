//! Capability identity, versioning, and requirements (ADR-0029,
//! M27-001-B).
//!
//! A capability is identified by a validated `namespace:name` string
//! with a closed namespace vocabulary (`runtime:` for built-ins — no
//! `node:`, no arbitrary prefixes). A requirement is satisfied only by
//! an exact version match: there is no implicit compatibility until a
//! semver ABI policy is defined (M27-009-D). Resolution failures are
//! typed and route the capability to `Failed` before it can reach
//! `Ready` (ADR-0028 rule 1).

use std::fmt;

use crate::CapabilityLifecycle;
use crate::CapabilityPhase;
use crate::LifecycleError;

/// Maximum total length of a capability id (`namespace:name`).
pub const MAX_ID_LEN: usize = 64;
/// Maximum length of the name segment.
pub const MAX_NAME_LEN: usize = 48;

/// Validated capability identity, e.g. `runtime:timers`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CapabilityId {
    raw: Box<str>,
}

/// Typed identity-validation failures. Closed set.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CapabilityIdError {
    Empty,
    MissingNamespaceSeparator,
    EmptyNamespace,
    EmptyName,
    /// The namespace is not in the closed vocabulary.
    UnknownNamespace {
        namespace: Box<str>,
    },
    /// Name contains characters outside `[a-z0-9-]`.
    InvalidNameChar {
        name: Box<str>,
        offset: usize,
    },
    NameTooLong {
        len: usize,
        max: usize,
    },
    IdTooLong {
        len: usize,
        max: usize,
    },
}

impl fmt::Display for CapabilityIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CapabilityIdError::Empty => f.write_str("capability id is empty"),
            CapabilityIdError::MissingNamespaceSeparator => {
                f.write_str("capability id must be namespace:name")
            }
            CapabilityIdError::EmptyNamespace => {
                f.write_str("capability id namespace is empty")
            }
            CapabilityIdError::EmptyName => f.write_str("capability id name is empty"),
            CapabilityIdError::UnknownNamespace { namespace } => write!(
                f,
                "unknown capability namespace {namespace:?} (closed set: runtime)"
            ),
            CapabilityIdError::InvalidNameChar { name, offset } => write!(
                f,
                "capability id name {name:?} has an invalid character at offset {offset} (allowed: a-z 0-9 -)"
            ),
            CapabilityIdError::NameTooLong { len, max } => {
                write!(f, "capability id name length {len} exceeds {max}")
            }
            CapabilityIdError::IdTooLong { len, max } => {
                write!(f, "capability id length {len} exceeds {max}")
            }
        }
    }
}

impl std::error::Error for CapabilityIdError {}

impl CapabilityId {
    /// Parse and validate a capability id. Fail-closed: every
    /// malformed input is a typed error, never a repair.
    pub fn parse(s: &str) -> Result<Self, CapabilityIdError> {
        if s.is_empty() {
            return Err(CapabilityIdError::Empty);
        }
        if s.len() > MAX_ID_LEN {
            return Err(CapabilityIdError::IdTooLong {
                len: s.len(),
                max: MAX_ID_LEN,
            });
        }
        let (namespace, name) = s
            .split_once(':')
            .ok_or(CapabilityIdError::MissingNamespaceSeparator)?;
        if namespace.is_empty() {
            return Err(CapabilityIdError::EmptyNamespace);
        }
        // Closed vocabulary: only the built-in namespace exists today.
        // Adding one is an ADR-level decision, not a string tweak.
        if namespace != "runtime" {
            return Err(CapabilityIdError::UnknownNamespace {
                namespace: namespace.into(),
            });
        }
        if name.is_empty() {
            return Err(CapabilityIdError::EmptyName);
        }
        if name.len() > MAX_NAME_LEN {
            return Err(CapabilityIdError::NameTooLong {
                len: name.len(),
                max: MAX_NAME_LEN,
            });
        }
        if let Some(offset) = name
            .chars()
            .position(|c| !c.is_ascii_lowercase() && !c.is_ascii_digit() && c != '-')
        {
            return Err(CapabilityIdError::InvalidNameChar {
                name: name.into(),
                offset,
            });
        }
        Ok(CapabilityId { raw: s.into() })
    }

    pub fn as_str(&self) -> &str {
        &self.raw
    }
}

impl fmt::Display for CapabilityId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.raw)
    }
}

/// Capability ABI version. Newtype so comparisons stay explicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CapabilityVersion(pub u32);

impl fmt::Display for CapabilityVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// What a pack requires: a capability at an exact version.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityRequirement {
    pub id: CapabilityId,
    pub version: CapabilityVersion,
}

/// What the runtime has linked: identity, version, and the module's
/// own requirements. The dependency graph over descriptors is built
/// and cycle-checked by the compiler resolver (M27-002).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityDescriptor {
    pub requirement: CapabilityRequirement,
    pub dependencies: Vec<CapabilityRequirement>,
}

/// Typed resolution failures (ADR-0028: these fail before `Ready`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolveError {
    /// No linked capability carries this id.
    Missing { id: CapabilityId },
    /// A linked capability carries the id at a different version.
    /// Exact match is required; no implicit compatibility.
    VersionConflict {
        id: CapabilityId,
        required: CapabilityVersion,
        linked: CapabilityVersion,
    },
    /// A cycle was detected in the dependency graph. The `path`
    /// names the ids on the cycle in traversal order; the last
    /// entry is the id that was already on the stack when the
    /// cycle edge was followed. Build errors must fail before any
    /// pack is produced (ADR-0028: version conflicts fail before
    /// ready; cycles are similarly a fatal build-time error).
    Cycle { path: Vec<CapabilityId> },
}

impl fmt::Display for ResolveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResolveError::Missing { id } => write!(f, "capability {id} is not linked"),
            ResolveError::VersionConflict {
                id,
                required,
                linked,
            } => write!(
                f,
                "capability {id} version conflict: required {required}, linked {linked}"
            ),
            ResolveError::Cycle { path } => {
                let ids: Vec<&str> = path.iter().map(|id| id.as_str()).collect();
                write!(f, "capability dependency cycle: {}", ids.join(" -> "))
            }
        }
    }
}

impl std::error::Error for ResolveError {}

/// Resolve one requirement against the linked set. The first
/// descriptor carrying the id decides: exact version match satisfies,
/// anything else is a typed conflict. Linked-set uniqueness itself is
/// pinned by the compiler inventory (M27-002-C).
pub fn resolve_requirement(
    linked: &[CapabilityDescriptor],
    req: &CapabilityRequirement,
) -> Result<(), ResolveError> {
    match linked.iter().find(|d| d.requirement.id == req.id) {
        None => Err(ResolveError::Missing { id: req.id.clone() }),
        Some(d) => {
            if d.requirement.version == req.version {
                Ok(())
            } else {
                Err(ResolveError::VersionConflict {
                    id: req.id.clone(),
                    required: req.version,
                    linked: d.requirement.version,
                })
            }
        }
    }
}

/// Typed install failures combining resolution and lifecycle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstallError {
    Resolve(ResolveError),
    Lifecycle(LifecycleError),
}

impl fmt::Display for InstallError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InstallError::Resolve(e) => write!(f, "{e}"),
            InstallError::Lifecycle(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for InstallError {}

/// ADR-0028 integration point: resolve first, then install. A
/// resolution failure routes the lifecycle to `Failed` — the
/// capability never observes `Ready` (guardrail: version conflicts
/// fail before ready).
pub fn resolve_and_install(
    lifecycle: &mut CapabilityLifecycle,
    linked: &[CapabilityDescriptor],
    req: &CapabilityRequirement,
) -> Result<CapabilityPhase, InstallError> {
    if let Err(e) = resolve_requirement(linked, req) {
        // fail() from Declared is always legal here
        let _ = lifecycle.fail();
        return Err(InstallError::Resolve(e));
    }
    lifecycle.install().map_err(InstallError::Lifecycle)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn descriptor(id: &str, version: u32) -> CapabilityDescriptor {
        CapabilityDescriptor {
            requirement: CapabilityRequirement {
                id: CapabilityId::parse(id).unwrap(),
                version: CapabilityVersion(version),
            },
            dependencies: Vec::new(),
        }
    }

    #[test]
    fn ids_parse_and_round_trip() {
        for ok in [
            "runtime:timers",
            "runtime:url",
            "runtime:crypto",
            "runtime:a-b9",
        ] {
            let id = CapabilityId::parse(ok).unwrap();
            assert_eq!(id.as_str(), ok);
            assert_eq!(id.to_string(), ok);
        }
    }

    #[test]
    fn malformed_ids_fail_closed_with_typed_errors() {
        assert_eq!(CapabilityId::parse(""), Err(CapabilityIdError::Empty));
        assert_eq!(
            CapabilityId::parse("timers"),
            Err(CapabilityIdError::MissingNamespaceSeparator)
        );
        assert_eq!(
            CapabilityId::parse(":timers"),
            Err(CapabilityIdError::EmptyNamespace)
        );
        assert_eq!(
            CapabilityId::parse("runtime:"),
            Err(CapabilityIdError::EmptyName)
        );
        assert_eq!(
            CapabilityId::parse("node:fs"),
            Err(CapabilityIdError::UnknownNamespace {
                namespace: "node".into()
            })
        );
        assert_eq!(
            CapabilityId::parse("runtime:Timers"),
            Err(CapabilityIdError::InvalidNameChar {
                name: "Timers".into(),
                offset: 0
            })
        );
        assert_eq!(
            CapabilityId::parse("runtime:timer_s"),
            Err(CapabilityIdError::InvalidNameChar {
                name: "timer_s".into(),
                offset: 5
            })
        );
        let long_name = "a".repeat(MAX_NAME_LEN + 1);
        assert_eq!(
            CapabilityId::parse(&format!("runtime:{long_name}")),
            Err(CapabilityIdError::NameTooLong {
                len: MAX_NAME_LEN + 1,
                max: MAX_NAME_LEN
            })
        );
        let long_id = format!("runtime:{}", "a".repeat(MAX_ID_LEN));
        assert_eq!(
            CapabilityId::parse(&long_id),
            Err(CapabilityIdError::IdTooLong {
                len: long_id.len(),
                max: MAX_ID_LEN
            })
        );
        // multiple separators are a name charset violation
        assert!(matches!(
            CapabilityId::parse("runtime:text:extra"),
            Err(CapabilityIdError::InvalidNameChar { .. })
        ));
    }

    #[test]
    fn exact_version_match_satisfies_requirement() {
        let linked = [descriptor("runtime:timers", 1)];
        let req = CapabilityRequirement {
            id: CapabilityId::parse("runtime:timers").unwrap(),
            version: CapabilityVersion(1),
        };
        assert_eq!(resolve_requirement(&linked, &req), Ok(()));
    }

    #[test]
    fn version_mismatch_conflicts_with_both_versions_named() {
        let linked = [descriptor("runtime:timers", 2)];
        let req = CapabilityRequirement {
            id: CapabilityId::parse("runtime:timers").unwrap(),
            version: CapabilityVersion(1),
        };
        // no implicit compatibility: newer linked != older required
        assert_eq!(
            resolve_requirement(&linked, &req),
            Err(ResolveError::VersionConflict {
                id: CapabilityId::parse("runtime:timers").unwrap(),
                required: CapabilityVersion(1),
                linked: CapabilityVersion(2),
            })
        );
        // and the message names both sides
        let msg = resolve_requirement(&linked, &req).unwrap_err().to_string();
        assert!(
            msg.contains("required 1") && msg.contains("linked 2"),
            "{msg}"
        );
    }

    #[test]
    fn unlinked_capability_is_missing() {
        let linked = [descriptor("runtime:timers", 1)];
        let req = CapabilityRequirement {
            id: CapabilityId::parse("runtime:fetch").unwrap(),
            version: CapabilityVersion(1),
        };
        assert_eq!(
            resolve_requirement(&linked, &req),
            Err(ResolveError::Missing {
                id: CapabilityId::parse("runtime:fetch").unwrap()
            })
        );
    }

    #[test]
    fn resolve_and_install_installs_on_success() {
        let linked = [descriptor("runtime:timers", 1)];
        let req = CapabilityRequirement {
            id: CapabilityId::parse("runtime:timers").unwrap(),
            version: CapabilityVersion(1),
        };
        let mut lc = CapabilityLifecycle::declared();
        assert_eq!(
            resolve_and_install(&mut lc, &linked, &req),
            Ok(CapabilityPhase::Installed)
        );
        // activation still available: conflict-free path to Ready
        assert_eq!(lc.activate(), Ok(CapabilityPhase::Ready));
    }

    #[test]
    fn resolve_and_install_conflict_fails_lifecycle_before_ready() {
        let linked = [descriptor("runtime:timers", 2)];
        let req = CapabilityRequirement {
            id: CapabilityId::parse("runtime:timers").unwrap(),
            version: CapabilityVersion(1),
        };
        let mut lc = CapabilityLifecycle::declared();
        let err = resolve_and_install(&mut lc, &linked, &req).unwrap_err();
        assert!(matches!(
            err,
            InstallError::Resolve(ResolveError::VersionConflict { .. })
        ));
        assert_eq!(lc.phase(), CapabilityPhase::Failed);
        assert!(!lc.can_start_ops());
        assert_eq!(
            lc.activate(),
            Err(LifecycleError::Terminal {
                from: CapabilityPhase::Failed
            })
        );
    }

    #[test]
    fn resolve_and_install_missing_fails_lifecycle_before_ready() {
        let linked: [CapabilityDescriptor; 0] = [];
        let req = CapabilityRequirement {
            id: CapabilityId::parse("runtime:timers").unwrap(),
            version: CapabilityVersion(1),
        };
        let mut lc = CapabilityLifecycle::declared();
        let err = resolve_and_install(&mut lc, &linked, &req).unwrap_err();
        assert!(matches!(
            err,
            InstallError::Resolve(ResolveError::Missing { .. })
        ));
        assert_eq!(lc.phase(), CapabilityPhase::Failed);
    }

    #[test]
    fn descriptors_carry_validated_dependencies() {
        let deps = vec![
            CapabilityRequirement {
                id: CapabilityId::parse("runtime:text").unwrap(),
                version: CapabilityVersion(1),
            },
            CapabilityRequirement {
                id: CapabilityId::parse("runtime:url").unwrap(),
                version: CapabilityVersion(1),
            },
        ];
        let d = CapabilityDescriptor {
            requirement: CapabilityRequirement {
                id: CapabilityId::parse("runtime:abort").unwrap(),
                version: CapabilityVersion(1),
            },
            dependencies: deps,
        };
        assert_eq!(d.dependencies.len(), 2);
        assert_eq!(d.dependencies[0].id.as_str(), "runtime:text");
    }
}
