//! legacy_v1 — the separate reader/adapter for `formatVersion: 1` packs
//! (ADR-0024, M26-008-A).
//!
//! Role and invariants:
//!
//! - This module is the ONLY sanctioned entry point for loading legacy
//!   JSON packs. Mode dispatch (`detect_pack_format_mode`) happens before
//!   any adapter runs, so legacy structures are constructed exclusively
//!   behind this facade; current-mode (qpack2) hot paths share no types
//!   and no code path with it — they parse borrowed zero-copy views, this
//!   adapter builds owned [`QPack`] trees.
//! - A supported v1 pack either loads through here or is rebuilt/migrated
//!   with tooling (M26-008-B); there is no silent fallback between modes.
//! - An unsupported pack fails closed with an actionable message naming
//!   the supported mode and the migration doc.
//!
//! Deprecation policy: v1 stays loadable through the M2.6 window; actual
//! removal requires an explicit owner-track decision recorded against
//! `docs/specs/pack-format-v1.md`. Until then this adapter is maintained,
//! fuzzed, and fixture-pinned — deprecated, not abandoned.

use crate::{detect_pack_format_mode, PackError, QPack};
use std::path::Path;

/// Load and fully verify a legacy v1 JSON pack from disk.
///
/// Fails closed on anything that is not a well-formed `formatVersion: 1`
/// pack; see [`crate::detect_pack_format_mode`] for the mode gate that
/// runs before this adapter is reached.
pub fn read_and_verify(path: &Path) -> Result<QPack, PackError> {
    // The caller (verify) has already gated the numeric mode; re-checking
    // here keeps this adapter safe when called directly.
    let pack = QPack::load_and_verify(path)?;
    detect_pack_format_mode(pack.format_version)?;
    Ok(pack)
}

/// Parse and verify a legacy v1 pack from raw bytes (fixtures, fuzzing,
/// tooling). Disk IO is the caller's concern.
pub fn read_and_verify_bytes(bytes: &[u8]) -> Result<QPack, PackError> {
    let pack: QPack =
        serde_json::from_slice(bytes).map_err(|e| PackError::Malformed(e.to_string()))?;
    pack.verify()?;
    detect_pack_format_mode(pack.format_version)?;
    Ok(pack)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Compatibility fixture: the committed golden v1 pack must keep
    /// loading byte-for-byte across refactors (public contract).
    #[test]
    fn loads_committed_v1_fixture() {
        let raw = include_bytes!("../tests/fixtures/v1/minimal.json");
        let pack = read_and_verify_bytes(raw).expect("golden v1 fixture verifies");
        assert_eq!(pack.format_version, crate::PACK_FORMAT_LEGACY_V1);
        assert_eq!(pack.app_id, "fixture");
        assert_eq!(pack.routes.len(), 1);
    }

    /// Guardrail: unsupported packs fail with an actionable message that
    /// names both the failure and the way out.
    #[test]
    fn unsupported_version_message_is_actionable() {
        let mut raw = serde_json::to_vec(&crate::minimal_pack_public()).unwrap();
        // Rewrite "formatVersion":1 -> ":7" inside the serialized JSON
        // (needle index 16 is the version digit).
        let needle = b"\"formatVersion\":1";
        let pos = raw
            .windows(needle.len())
            .position(|w| w == needle)
            .expect("fixture serializes formatVersion");
        raw[pos + 16] = b'7';
        let err = read_and_verify_bytes(&raw).unwrap_err().to_string();
        assert!(err.contains("not supported"), "{err}");
        assert!(err.contains("fail closed"), "{err}");
        assert!(
            err.contains("rebuild") && err.contains("migrate"),
            "message must name the way out: {err}"
        );
    }
}
