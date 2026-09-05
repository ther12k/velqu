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
#[cfg(feature = "native")]
use std::path::Path;

/// Load and fully verify a legacy v1 JSON pack from disk.
///
/// Fails closed on anything that is not a well-formed `formatVersion: 1`
/// pack; see [`crate::detect_pack_format_mode`] for the mode gate that
/// runs before this adapter is reached.
#[cfg(feature = "native")]
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
    crate::reject_mixed_mode_bytes(bytes)?;
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

    /// M26-008-C: a v1 JSON pack carrying mode-2-reserved top-level
    /// fields is a mixed-mode artifact and is rejected BY NAME — serde
    /// would otherwise silently drop the unknown key.
    #[test]
    fn mixed_mode_sections_key_is_rejected() {
        let raw = include_bytes!("../tests/fixtures/v1/mixed-mode-sections.json");
        let err = read_and_verify_bytes(raw).unwrap_err().to_string();
        assert!(err.contains("mixed-mode"), "{err}");
        assert!(err.contains("'sections'"), "{err}");
    }

    /// M26-008-C: a binary mode-2 container presented as a JSON pack is
    /// caught by magic before any parse attempt.
    #[test]
    fn binary_container_presented_as_json_is_rejected() {
        let mut bytes = crate::qpack2::MAGIC.to_vec();
        bytes.extend_from_slice(&[0u8; 64]);
        let err = read_and_verify_bytes(&bytes).unwrap_err().to_string();
        assert!(
            err.contains("mixed-mode") && err.contains("VELQUQPK"),
            "{err}"
        );
    }

    /// M26-008-D: unsupported legacy features fail DETERMINISTICALLY —
    /// the same fixture always produces the same rejection substring, and
    /// two runs produce byte-identical errors (no addresses, counters, or
    /// environment-dependent text), with no fallback path succeeding.
    #[test]
    fn unsupported_legacy_features_fail_deterministically() {
        let cases: [(&str, &str); 4] = [
            (
                // pre-M25 producer: Schema IR v1 is not supported by this runtime
                "../tests/fixtures/v1/unsupported/schema-ir-v1.json",
                "schema IR version 1 not supported",
            ),
            (
                // engine fingerprint mismatch (SEC-001 exact match)
                "../tests/fixtures/v1/unsupported/engine-mismatch.json",
                "engine mismatch",
            ),
            (
                // legacy prelude declaration without bytecode
                "../tests/fixtures/v1/unsupported/prelude-without-bytecode.json",
                "requires bundleBytecode",
            ),
            (
                // future/unknown runtime ABI
                "../tests/fixtures/v1/unsupported/runtime-abi.json",
                "runtime ABI",
            ),
        ];
        for (fixture, expected) in cases {
            // Fixtures are committed; load by name from the unsupported dir.
            let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("tests/fixtures/v1/unsupported")
                .join(fixture.rsplit('/').next().unwrap());
            let raw =
                std::fs::read(&path).unwrap_or_else(|e| panic!("fixture {path:?} missing: {e}"));
            let e1 = read_and_verify_bytes(&raw).unwrap_err().to_string();
            assert!(
                e1.contains(expected),
                "{fixture}: expected '{expected}' in '{e1}'"
            );
            let e2 = read_and_verify_bytes(&raw).unwrap_err().to_string();
            assert_eq!(e1, e2, "{fixture}: rejection must be deterministic");
            assert!(!e1.contains("0x"), "no addresses in rejections: {e1}");
        }
    }
}
