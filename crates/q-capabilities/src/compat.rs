//! Semver/ABI compatibility policy for capabilities (M27-009-D).
//!
//! Policy: a capability version interpreted as a packed semver triple
//! `major*1_000_000 + minor*1_000 + patch` (each component bounded to
//! 0–999, fail closed). The **major version is the ABI revision**:
//!
//! - major bump → ABI breaking; requirements never auto-satisfy across it;
//! - minor bump → additive, backward-compatible surface growth;
//! - patch bump → backward-compatible fixes.
//!
//! Exact integer matching ([`crate::identity`] stays untouched) remains
//! the default and safest requirement form; [`VersionSelector`] offers
//! the explicit, opt-in relaxed form so pack authors declare their
//! intended policy rather than inheriting one.

use std::fmt;

use crate::identity::CapabilityVersion;

/// Largest legal value for each semver component (0–999).
pub const MAX_SEMVER_COMPONENT: u32 = 999;

/// SDK ABI revision implemented by this build of `q-capabilities`.
/// Breaking the public SDK trait shapes (`CapabilitySdk`,
/// `CancellableCapability`) or the lifecycle state machine bumps this.
pub const SDK_ABI_REVISION: u32 = 1;

/// Typed compatibility-policy failures. Closed set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompatError {
    /// A semver component is zero-bound-violating (> [`MAX_SEMVER_COMPONENT`]).
    ComponentOutOfRange { component: &'static str, value: u32 },
    /// A version integer does not decompose into the packed triple
    /// under the component ceiling.
    UnpackableVersion { version: u32 },
}

impl fmt::Display for CompatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompatError::ComponentOutOfRange { component, value } => {
                write!(
                    f,
                    "semver {component} {value} exceeds {}",
                    MAX_SEMVER_COMPONENT
                )
            }
            CompatError::UnpackableVersion { version } => write!(
                f,
                "version {version} does not decompose into major.minor.patch with each part <= {}",
                MAX_SEMVER_COMPONENT
            ),
        }
    }
}

impl std::error::Error for CompatError {}

fn check_component(component: &'static str, value: u32) -> Result<(), CompatError> {
    if value > MAX_SEMVER_COMPONENT {
        return Err(CompatError::ComponentOutOfRange { component, value });
    }
    Ok(())
}

/// A validated semver triple in the packed regime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PackedSemVer {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl PackedSemVer {
    /// Build from components, failing closed on out-of-range parts.
    pub fn from_parts(major: u32, minor: u32, patch: u32) -> Result<Self, CompatError> {
        check_component("major", major)?;
        check_component("minor", minor)?;
        check_component("patch", patch)?;
        Ok(PackedSemVer {
            major,
            minor,
            patch,
        })
    }

    /// Decompose a version integer into the packed triple. Any component
    /// above the ceiling makes the version unpackable — fail closed,
    /// never guess.
    pub fn unpack(version: CapabilityVersion) -> Result<Self, CompatError> {
        let v = version.0;
        let patch = v % 1_000;
        let minor = (v / 1_000) % 1_000;
        let major = v / 1_000_000;
        if major > MAX_SEMVER_COMPONENT {
            return Err(CompatError::UnpackableVersion { version: v });
        }
        Ok(PackedSemVer {
            major,
            minor,
            patch,
        })
    }

    /// Encode back into the pack-format integer (total fits `u32` by
    /// construction after component validation).
    pub fn pack(&self) -> CapabilityVersion {
        CapabilityVersion(self.major * 1_000_000 + self.minor * 1_000 + self.patch)
    }

    /// ABI revision (major) of this version.
    pub fn abi_revision(&self) -> u32 {
        self.major
    }
}

impl fmt::Display for PackedSemVer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// Outcome of comparing two versions under the ABI policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Compatibility {
    /// Same ABI revision: drop-in safe per the policy.
    AbiCompatible,
    /// Different ABI revision: fail closed, re-resolve explicitly.
    AbiBreaking,
}

/// Classify a version change against the policy: differing majors are
/// ABI breaking; everything within a major is compatible (minors add,
/// patches fix — providers owe that discipline).
pub fn classify(from: &PackedSemVer, to: &PackedSemVer) -> Compatibility {
    if from.major != to.major {
        Compatibility::AbiBreaking
    } else {
        Compatibility::AbiCompatible
    }
}

/// How a requirement selects among provider versions. `Exact` mirrors
/// [`crate::identity`] resolution; `CompatibleWith` opts into the semver
/// policy: any provider with the same ABI revision that is at least as
/// new (minor/patch lexicographically) satisfies the requirement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VersionSelector {
    Exact(CapabilityVersion),
    CompatibleWith(PackedSemVer),
}

impl VersionSelector {
    /// Does `provider` satisfy this selector?
    pub fn matches(&self, provider: CapabilityVersion) -> bool {
        match self {
            VersionSelector::Exact(required) => *required == provider,
            VersionSelector::CompatibleWith(required) => {
                let Ok(p) = PackedSemVer::unpack(provider) else {
                    return false;
                };
                p.major == required.major && (p.minor, p.patch) >= (required.minor, required.patch)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parts_pack_and_roundtrip() {
        let v = PackedSemVer::from_parts(1, 4, 2).unwrap();
        assert_eq!(v.pack(), CapabilityVersion(1_004_002));
        assert_eq!(
            PackedSemVer::unpack(CapabilityVersion(1_004_002)).unwrap(),
            v
        );
        assert_eq!(v.to_string(), "1.4.2");
        assert_eq!(v.abi_revision(), 1);
    }

    #[test]
    fn out_of_range_components_fail_closed() {
        assert!(PackedSemVer::from_parts(1_000, 0, 0).is_err());
        assert!(PackedSemVer::from_parts(1, 1_000, 0).is_err());
        assert!(PackedSemVer::from_parts(1, 0, 1_000).is_err());
        assert_eq!(
            PackedSemVer::from_parts(1_000, 0, 0).unwrap_err(),
            CompatError::ComponentOutOfRange {
                component: "major",
                value: 1_000,
            }
        );
    }

    #[test]
    fn unpack_rejects_versions_above_component_ceiling() {
        // 1_234_567_890 decomposes to major 1234 — beyond the ceiling.
        assert_eq!(
            PackedSemVer::unpack(CapabilityVersion(1_234_567_890)).unwrap_err(),
            CompatError::UnpackableVersion {
                version: 1_234_567_890
            }
        );
        // The maximal packed value is fine.
        assert!(PackedSemVer::unpack(CapabilityVersion(999_999_999)).is_ok());
    }

    #[test]
    fn major_changes_break_abi_within_major_is_compatible() {
        let a = PackedSemVer::from_parts(1, 2, 3).unwrap();
        assert_eq!(
            classify(&a, &PackedSemVer::from_parts(1, 3, 0).unwrap()),
            Compatibility::AbiCompatible
        );
        assert_eq!(
            classify(&a, &PackedSemVer::from_parts(1, 2, 4).unwrap()),
            Compatibility::AbiCompatible
        );
        assert_eq!(
            classify(&a, &PackedSemVer::from_parts(2, 0, 0).unwrap()),
            Compatibility::AbiBreaking
        );
    }

    #[test]
    fn exact_selector_matches_only_identical_version() {
        let sel = VersionSelector::Exact(CapabilityVersion(7));
        assert!(sel.matches(CapabilityVersion(7)));
        assert!(!sel.matches(CapabilityVersion(8)));
    }

    #[test]
    fn compatible_selector_follows_semver_policy() {
        // Need 1.2.0-or-newer within ABI major 1.
        let sel = VersionSelector::CompatibleWith(PackedSemVer::from_parts(1, 2, 0).unwrap());
        assert!(sel.matches(CapabilityVersion(1_002_000))); // 1.2.0 itself
        assert!(sel.matches(CapabilityVersion(1_002_001))); // 1.2.1 patch
        assert!(sel.matches(CapabilityVersion(1_003_000))); // 1.3.0 minor
        assert!(sel.matches(CapabilityVersion(1_099_099))); // 1.99.99
        assert!(!sel.matches(CapabilityVersion(1_001_999))); // 1.1.999 too old
        assert!(!sel.matches(CapabilityVersion(2_000_000))); // 2.0.0 other ABI
                                                             // Provider versions that do not unpack never satisfy the policy.
        assert!(!sel.matches(CapabilityVersion(4_294_967_295)));
    }

    #[test]
    fn sdk_abi_revision_is_explicit() {
        // Pin the current revision so an unnoticed bump is impossible.
        assert_eq!(SDK_ABI_REVISION, 1);
    }
}
