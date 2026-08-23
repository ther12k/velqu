//! q-pack — versioned application pack reader/verifier (velqu.qpack v1).
//!
//! The pack is the single production artifact (see `docs/specs/pack-format-v1.md`).
//! Everything here is load-and-verify only: no compilation, no discovery.
//!
//! M26-005-C unsafe policy: this crate contains exactly ONE `unsafe`
//! block — the read-only mmap in `qpack2::reader::PackBytes::open` —
//! audited in place with a full SAFETY block. Everything the reader
//! does with mapped bytes is bounds-checked safe code (checked range
//! arithmetic, M26-005-B). `unsafe_op_in_unsafe_fn` is denied so any
//! future unsafe must be an explicit, reviewable block.
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// Numeric pack-format modes (ADR-0024). `formatVersion` is a mode
/// selector, not a minor revision: the runtime implements a closed set of
/// modes and fails closed on anything outside it. Exactly one mode is
/// CURRENT at a time; older supported modes load only through their named
/// adapter; unknown versions never fall back or guess.
pub const PACK_FORMAT_LEGACY_V1: u32 = 1;

/// M26-002-C: bytecode loading policy. `Enforce` is the default (all
/// bytecode fingerprint checks run); `Skip` is the explicit
/// source-rebuild path (`--no-bytecode`): the verified source bundle
/// evaluates instead of the embedded bytecode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BytecodePolicy {
    Enforce,
    Skip,
}
/// The current numeric mode. Legacy v1 until M26-003 flips producer and
/// runtime to binary QPack v2 (ADR-0024 migration rule); the flip is one
/// constant change plus a native adapter, never a silent in-place rewrite.
pub const PACK_FORMAT_CURRENT: u32 = PACK_FORMAT_LEGACY_V1;
/// Workspace-wide alias: "the version the current producer emits".
pub const PACK_FORMAT_VERSION: u32 = PACK_FORMAT_CURRENT;
pub const RUNTIME_ABI: u32 = 1;
pub const SCHEMA_IR_VERSION: u32 = 2;
pub const CONTRACT_VERSION: u32 = 1;
/// Engine this runtime build embeds (quickjs-ng vendored by rquickjs =0.12.2).
pub const ENGINE_NAME: &str = "quickjs-ng";
pub const ENGINE_VERSION: &str = "0.15.1";
/// M26-002-A: the rquickjs binding version pinned by the workspace
/// (rquickjs =0.12.2, AGENTS.md constraint 1).
pub const RQUICKJS_VERSION: &str = "0.12.2";

/// M26-002-A: deterministic runtime build fingerprint — the runtime
/// identity tuple (ABI, engine, version, rquickjs, binding). A binary-level
/// reproducible-build hash replaces this constant when the release pipeline
/// embeds one; the tuple hash is the build identity until then.
/// M26-002-A: canonical capability-set hash — sha256 over the sorted,
/// newline-joined capability names (empty set hashes the empty string).
pub fn capability_hash(caps: &[String]) -> String {
    let mut sorted = caps.to_vec();
    sorted.sort();
    hex(&Sha256::digest(sorted.join("\n").as_bytes()))
}

pub fn runtime_build_hash() -> String {
    let tuple = format!(
        "abi={}:engine={}:version={}:rquickjs={}:binding={}",
        RUNTIME_ABI, ENGINE_NAME, ENGINE_VERSION, RQUICKJS_VERSION, ENGINE_BINDING
    );
    hex(&Sha256::digest(tuple.as_bytes()))
}
pub const ENGINE_BINDING: &str = "rquickjs-0.12.2";

#[derive(Debug, thiserror::Error)]
pub enum PackError {
    #[error("pack io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("pack is not valid JSON: {0}")]
    Malformed(String),
    #[error("pack rejected: {0}")]
    Rejected(String),
}

/// Which adapter decodes and validates a pack (ADR-0024). One variant per
/// numeric mode; no variant carries a cross-mode handler table, and a
/// current-mode pack never silently reuses legacy semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackFormatMode {
    /// UTF-8 JSON pack, frozen v1 shape (`docs/specs/pack-format-v1.md`),
    /// loaded only through this named adapter.
    LegacyV1,
}

/// Resolve a numeric `formatVersion` to its adapter. Unknown versions fail
/// closed: no fallback, no best-effort parse of a newer/older layout.
pub fn detect_pack_format_mode(format_version: u32) -> Result<PackFormatMode, PackError> {
    match format_version {
        PACK_FORMAT_LEGACY_V1 => Ok(PackFormatMode::LegacyV1),
        other => Err(PackError::Rejected(format!(
            "pack formatVersion {other} not supported (supported modes: \
             {PACK_FORMAT_LEGACY_V1} = legacy-v1 JSON adapter); unknown versions fail closed"
        ))),
    }
}

/// Mode-2 binary layout constants (ADR-0025,
/// `docs/specs/pack-format-v2.md`). The encoder and native decoder land
/// with M26-003; these constants exist so spec/code drift fails tests
/// first. Nothing here changes legacy-v1 behavior.
/// ADR-0027 debug source sidecar (`<pack>.sources.json`): advisory
/// tooling file bound to exactly one pack via `pack_sha256`. The runtime
/// NEVER reads sidecars — no load path or fallback consults them
/// (`verification_is_independent_of_debug_sidecars`); verification here
/// serves DEVELOPMENT TOOLING ONLY (symbolizers, inspect) and confers no
/// authenticity (ADR-0026).
pub mod sources_sidecar {
    use serde::{Deserialize, Serialize};

    pub const SIDECAR_FORMAT_VERSION: u32 = 1;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct SidecarModule {
        pub id: String,
        pub file: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct SourcesSidecar {
        pub format_version: u32,
        /// sha256 (hex) of the exact pack file bytes this sidecar belongs to.
        pub pack_sha256: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub bundle_source: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub source_map: Option<String>,
        #[serde(default)]
        pub modules: Vec<SidecarModule>,
    }

    impl SourcesSidecar {
        /// Tool-side advisory check: the sidecar must declare the exact
        /// pack file hash it belongs to. A mismatch is an ergonomic warning
        /// for tooling, never a runtime behavior change.
        pub fn verify_against(&self, pack_sha256: &str) -> Result<(), String> {
            if self.format_version != SIDECAR_FORMAT_VERSION {
                return Err(format!(
                    "sources sidecar format version {} != {} (ADR-0024: unknown versions fail closed)",
                    self.format_version, SIDECAR_FORMAT_VERSION
                ));
            }
            if self.pack_sha256 != pack_sha256 {
                return Err(
                    "sources sidecar belongs to a different pack (packSha256 mismatch)".to_string(),
                );
            }
            Ok(())
        }
    }
}

pub mod qpack2 {
    /// File magic at offset 0 (ASCII).
    pub const MAGIC: &[u8; 8] = b"VELQUQPK";
    /// Numeric mode this layout belongs to.
    pub const FORMAT_VERSION: u32 = 2;
    /// Fixed header size in bytes.
    pub const HEADER_SIZE: u64 = 64;
    /// Fixed directory-entry stride in bytes.
    pub const DIR_ENTRY_SIZE: u64 = 64;
    /// Required alignment for every section start.
    pub const SECTION_ALIGN: u64 = 8;
    /// Directory entry flag bit: the adapter may tolerate this section's
    /// absence; presence is always fully validated.
    pub const FLAG_OPTIONAL: u16 = 0x0001;

    /// M26-003-C: file-level offsets and bounds checks. Parses the 64-byte
    /// header and the section directory with every spec §2/§3 rule
    /// enforced BEFORE any section content is interpreted:
    /// magic exact; header_size == 64; total_size == actual length;
    /// reserved zero; entries unique by id; offsets >= header+directory;
    /// 8-aligned; len > 0; ranges disjoint; within the file; content
    /// sha256 verified at read time (integrity only — ADR-0026).
    /// Required ids must all be present; unknown ids reject even when
    /// flagged optional.
    pub mod reader {
        use super::{section, DIR_ENTRY_SIZE, FLAG_OPTIONAL, HEADER_SIZE, MAGIC, SECTION_ALIGN};

        /// Pack file bytes: mmap'd READ-ONLY where supported (unix),
        /// owned `Vec<u8>` otherwise (M26-005-A). Deref to `&[u8]` so
        /// directory validation borrows zero-copy section views out of
        /// the mapping — no owned copy of the pack is reconstructed.
        pub enum PackBytes {
            #[cfg(unix)]
            Mapped(memmap2::Mmap),
            /// Pack bytes compiled into a standalone binary
            /// (`include_bytes!`-style, M26-009-B wiring). Zero-copy by
            /// construction: validation borrows straight out of the
            /// executable image; nothing is copied or reconstructed.
            Embedded(&'static [u8]),
            Owned(Vec<u8>),
        }

        impl std::ops::Deref for PackBytes {
            type Target = [u8];
            fn deref(&self) -> &[u8] {
                match self {
                    #[cfg(unix)]
                    PackBytes::Mapped(m) => m.as_ref(),
                    PackBytes::Embedded(b) => b,
                    PackBytes::Owned(v) => v.as_slice(),
                }
            }
        }

        impl PackBytes {
            /// Open a pack file read-only. Prefer a read-only mapping on
            /// unix; empty/unmappable/other platforms fall back to a
            /// bounded owned read. Validation rejects malformed lengths
            /// identically on either path (same `&[u8]` consumer).
            pub fn open(path: &std::path::Path) -> Result<PackBytes, String> {
                let file =
                    std::fs::File::open(path).map_err(|e| format!("open pack failed: {e}"))?;
                #[cfg(unix)]
                {
                    let len = file
                        .metadata()
                        .map_err(|e| format!("pack metadata failed: {e}"))?
                        .len();
                    if len > 0 {
                        // SAFETY (M26-005-C audit — the crate's ONLY unsafe):
                        // 1. `Mmap::map` requests a PROT_READ, MAP_SHARED
                        //    mapping; `Mmap` owns the fd for its lifetime,
                        //    so the mapping outlives `file` correctly.
                        // 2. Consumers deref to `&[u8]` and NEVER write —
                        //    write-through would fault (read-only map).
                        // 3. Residual hazard (inherent to mmap, accepted):
                        //    ANOTHER process truncating the file after
                        //    mapping raises SIGBUS on access. Deployed
                        //    packs are immutable build artifacts; the
                        //    owned fallback covers empty/unmappable files.
                        // 4. All reads through the mapping go through the
                        //    checked-bounds reader (M26-005-B): no slice
                        //    is derived without a validated range.
                        let map = unsafe { memmap2::Mmap::map(&file) }
                            .map_err(|e| format!("mmap pack failed: {e}"))?;
                        return Ok(PackBytes::Mapped(map));
                    }
                }
                let bytes = std::fs::read(path).map_err(|e| format!("read pack failed: {e}"))?;
                Ok(PackBytes::Owned(bytes))
            }
        }

        #[derive(Debug)]
        pub struct Header {
            pub total_size: u64,
            pub section_count: u32,
        }

        #[derive(Debug)]
        pub struct DirEntry {
            pub section_id: u16,
            pub flags: u16,
            pub offset: u64,
            pub len: u64,
            pub content_sha256: [u8; 32],
        }

        pub fn parse_header(bytes: &[u8]) -> Result<Header, String> {
            if bytes.len() < HEADER_SIZE as usize {
                return Err("file shorter than the fixed header".to_string());
            }
            if &bytes[0..8] != MAGIC {
                return Err("magic mismatch (not a v2 pack)".to_string());
            }
            let version = u32::from_le_bytes(bytes[8..12].try_into().unwrap());
            if version != super::FORMAT_VERSION {
                return Err(format!(
                    "format version {version} != {}",
                    super::FORMAT_VERSION
                ));
            }
            let header_size = u32::from_le_bytes(bytes[12..16].try_into().unwrap());
            if header_size as u64 != HEADER_SIZE {
                return Err(format!("header_size {header_size} != {HEADER_SIZE}"));
            }
            let total_size = u64::from_le_bytes(bytes[16..24].try_into().unwrap());
            if total_size != bytes.len() as u64 {
                return Err(format!(
                    "total_size {total_size} != actual length {}",
                    bytes.len()
                ));
            }
            let section_count = u32::from_le_bytes(bytes[24..28].try_into().unwrap());
            let reserved = u32::from_le_bytes(bytes[28..32].try_into().unwrap());
            if reserved != 0 {
                return Err("header reserved field is non-zero".to_string());
            }
            if bytes[32..64].iter().any(|b| *b != 0) {
                return Err("header reserved bytes are non-zero".to_string());
            }
            Ok(Header {
                total_size,
                section_count,
            })
        }

        pub fn parse_directory(bytes: &[u8]) -> Result<Vec<DirEntry>, String> {
            // M26-003-D: both header sizes are legal — 64 (plain) and 96
            // (execution-integrity binding in the extension area)
            let stored_hs = u32::from_le_bytes(
                bytes
                    .get(12..16)
                    .ok_or("file shorter than the fixed header")?
                    .try_into()
                    .unwrap(),
            );
            let hs = match stored_hs {
                64 => HEADER_SIZE,
                96 => EXTENDED_HEADER_SIZE,
                other => return Err(format!("header_size {other} is not 64 or 96")),
            };
            parse_directory_of_size(bytes, hs as u32)
        }

        /// Full validation: header + directory + catalog rules (required
        /// ids present; unknown ids reject even when optional) + per-section
        /// content sha256. Returns validated entries with their slices.
        pub fn validate(bytes: &[u8]) -> Result<Vec<(DirEntry, &[u8])>, String> {
            let entries = parse_directory(bytes)?;
            for e in &entries {
                let known = matches!(
                    e.section_id,
                    section::STRINGS
                        | section::ROUTES
                        | section::ROUTE_PLANS
                        | section::SCHEMA_MANIFEST
                        | section::POLICIES
                        | section::CAPABILITIES
                        | section::BUNDLE_BYTECODE
                        | section::CONTRACT_SUMMARY
                );
                if !known {
                    return Err(format!(
                        "unknown section id {:#x} — outside the registered catalog; no skip-and-continue (spec §5)",
                        e.section_id
                    ));
                }
                let body = &bytes[e.offset as usize..(e.offset + e.len) as usize];
                use sha2::Digest;
                let hash = sha2::Sha256::digest(body);
                if hash[..] != e.content_sha256[..] {
                    return Err(format!(
                        "section {:#x}: content sha256 mismatch (integrity failure)",
                        e.section_id
                    ));
                }
            }
            for required in section::REQUIRED {
                if !entries.iter().any(|e| e.section_id == *required) {
                    return Err(format!(
                        "required section {:#x} missing (spec §6)",
                        required
                    ));
                }
            }
            Ok(entries
                .into_iter()
                .map(|e| {
                    let body = &bytes[e.offset as usize..(e.offset + e.len) as usize];
                    (e, body)
                })
                .collect())
        }

        /// M26-003-D: execution-integrity binding. The aggregate is
        /// sha256 over the canonical directory form (entries sorted by
        /// section_id: id, flags, offset, len, content_sha256). It pins
        /// the ENTIRE execution graph — any section byte, any directory
        /// field, any reordering changes it. Stored in the header
        /// extension area (spec §2 growth path: header grows via
        /// header_size, never by reinterpreting fields).
        pub const EXTENDED_HEADER_SIZE: u64 = 96;
        pub const EXECUTION_HASH_OFFSET: usize = 64;

        pub fn compute_execution_hash(entries: &[DirEntry]) -> [u8; 32] {
            use sha2::Digest;
            let mut sorted: Vec<&DirEntry> = entries.iter().collect();
            sorted.sort_by_key(|e| e.section_id);
            let mut canon = Vec::new();
            for e in sorted {
                canon.extend_from_slice(&e.section_id.to_le_bytes());
                canon.extend_from_slice(&e.flags.to_le_bytes());
                canon.extend_from_slice(&e.offset.to_le_bytes());
                canon.extend_from_slice(&e.len.to_le_bytes());
                canon.extend_from_slice(&e.content_sha256);
            }
            sha2::Sha256::digest(&canon).into()
        }

        /// Directory parse accepting BOTH header sizes: 64 (no binding)
        /// and 96 (execution hash present and verified).
        pub fn parse_directory_with_binding(bytes: &[u8]) -> Result<Vec<DirEntry>, String> {
            let entries = parse_directory_of_size(bytes, EXTENDED_HEADER_SIZE as u32)?;
            let stored: [u8; 32] = bytes[EXECUTION_HASH_OFFSET..EXECUTION_HASH_OFFSET + 32]
                .try_into()
                .unwrap();
            let computed = compute_execution_hash(&entries);
            if stored != computed {
                return Err(
                    "execution-integrity hash mismatch: the directory or any section content changed after build"
                        .to_string(),
                );
            }
            Ok(entries)
        }

        fn parse_directory_of_size(
            bytes: &[u8],
            header_size: u32,
        ) -> Result<Vec<DirEntry>, String> {
            if bytes.len() < header_size as usize {
                return Err("file shorter than the header".to_string());
            }
            if &bytes[0..8] != MAGIC {
                return Err("magic mismatch (not a v2 pack)".to_string());
            }
            let version = u32::from_le_bytes(bytes[8..12].try_into().unwrap());
            if version != super::FORMAT_VERSION {
                return Err("format version mismatch".to_string());
            }
            let stored_hs = u32::from_le_bytes(bytes[12..16].try_into().unwrap());
            if stored_hs != header_size {
                return Err(format!("header_size {stored_hs} != expected {header_size}"));
            }
            let total_size = u64::from_le_bytes(bytes[16..24].try_into().unwrap());
            if total_size != bytes.len() as u64 {
                return Err("total_size != actual length".to_string());
            }
            let section_count = u32::from_le_bytes(bytes[24..28].try_into().unwrap());
            if u32::from_le_bytes(bytes[28..32].try_into().unwrap()) != 0 {
                return Err("header reserved field is non-zero".to_string());
            }
            if bytes[32..EXECUTION_HASH_OFFSET].iter().any(|b| *b != 0) {
                return Err("header reserved bytes are non-zero".to_string());
            }
            let dir_base = header_size as usize;
            let dir_end = dir_base as u64 + DIR_ENTRY_SIZE * section_count as u64;
            if dir_end > bytes.len() as u64 {
                return Err("directory extends past end of file".to_string());
            }
            let mut entries: Vec<DirEntry> = Vec::new();
            for i in 0..section_count as usize {
                let base = dir_base + (DIR_ENTRY_SIZE * i as u64) as usize;
                let e = DirEntry {
                    section_id: u16::from_le_bytes(bytes[base..base + 2].try_into().unwrap()),
                    flags: u16::from_le_bytes(bytes[base + 2..base + 4].try_into().unwrap()),
                    offset: u64::from_le_bytes(bytes[base + 8..base + 16].try_into().unwrap()),
                    len: u64::from_le_bytes(bytes[base + 16..base + 24].try_into().unwrap()),
                    content_sha256: bytes[base + 24..base + 56].try_into().unwrap(),
                };
                if e.flags & !FLAG_OPTIONAL != 0 {
                    return Err(format!(
                        "directory entry {i}: unknown flag bits {:#x}",
                        e.flags
                    ));
                }
                if u32::from_le_bytes(bytes[base + 4..base + 8].try_into().unwrap()) != 0 {
                    return Err(format!("directory entry {i}: reserved field is non-zero"));
                }
                if e.offset < dir_end {
                    return Err(format!(
                        "directory entry {i} (id {:#x}): offset overlaps header/directory",
                        e.section_id
                    ));
                }
                if !e.offset.is_multiple_of(SECTION_ALIGN) {
                    return Err(format!(
                        "directory entry {i} (id {:#x}): offset not 8-aligned",
                        e.section_id
                    ));
                }
                if e.len == 0 {
                    return Err(format!(
                        "directory entry {i} (id {:#x}): zero length",
                        e.section_id
                    ));
                }
                // M26-005-B: offset/len are raw file-controlled u64s —
                // every range computation is checked BEFORE any slicing
                // so malformed lengths can neither panic (debug overflow)
                // nor wrap past the file (release).
                let Some(range_end) = e.offset.checked_add(e.len) else {
                    return Err(format!(
                        "directory entry {i} (id {:#x}): offset+len overflows u64",
                        e.section_id
                    ));
                };
                if range_end > bytes.len() as u64 {
                    return Err(format!(
                        "directory entry {i} (id {:#x}): range past end of file",
                        e.section_id
                    ));
                }
                if entries.iter().any(|p| p.section_id == e.section_id) {
                    return Err(format!("duplicate section id {:#x}", e.section_id));
                }
                let overlaps = entries.iter().any(|p| {
                    p.offset
                        .checked_add(p.len)
                        .is_some_and(|p_end| range_end > p.offset && p_end > e.offset)
                });
                if overlaps {
                    return Err(format!(
                        "directory entry {i} (id {:#x}): range overlaps another section",
                        e.section_id
                    ));
                }
                entries.push(e);
            }
            Ok(entries)
        }

        /// Build a v2 file WITH the execution-integrity binding: a 96-byte
        /// header whose extension area carries the aggregate hash over
        /// the final directory.
        pub fn build_file_bound(payloads: &[(u16, &[u8])]) -> Vec<u8> {
            use sha2::Digest;
            let section_count = payloads.len() as u32;
            let dir_end = EXTENDED_HEADER_SIZE + DIR_ENTRY_SIZE * payloads.len() as u64;
            let mut out = Vec::new();
            out.extend_from_slice(MAGIC);
            out.extend_from_slice(&super::FORMAT_VERSION.to_le_bytes());
            out.extend_from_slice(&(EXTENDED_HEADER_SIZE as u32).to_le_bytes());
            out.extend_from_slice(&0u64.to_le_bytes()); // total patched
            out.extend_from_slice(&section_count.to_le_bytes());
            out.extend_from_slice(&0u32.to_le_bytes());
            out.extend_from_slice(&[0u8; 32]);
            out.extend_from_slice(&[0u8; 32]); // binding patched
            let mut bodies = Vec::new();
            let mut offset = dir_end;
            for (_, body) in payloads {
                while !offset.is_multiple_of(SECTION_ALIGN) {
                    offset += 1;
                }
                bodies.push((offset, body));
                offset += body.len() as u64;
            }
            out[16..24].copy_from_slice(&offset.to_le_bytes());
            let mut entries = Vec::new();
            for (i, (id, body)) in payloads.iter().enumerate() {
                let base = (EXTENDED_HEADER_SIZE + DIR_ENTRY_SIZE * i as u64) as usize;
                out.extend_from_slice(&vec![0u8; base - out.len()]);
                out.extend_from_slice(&id.to_le_bytes());
                out.extend_from_slice(&0u16.to_le_bytes());
                out.extend_from_slice(&0u32.to_le_bytes());
                let (sec_off, _) = bodies[i];
                out.extend_from_slice(&sec_off.to_le_bytes());
                out.extend_from_slice(&(body.len() as u64).to_le_bytes());
                let hash: [u8; 32] = sha2::Sha256::digest(body).into();
                out.extend_from_slice(&hash);
                entries.push(DirEntry {
                    section_id: *id,
                    flags: 0,
                    offset: sec_off,
                    len: body.len() as u64,
                    content_sha256: hash,
                });
            }
            for (sec_off, body) in &bodies {
                while (out.len() as u64) < *sec_off {
                    out.push(0);
                }
                out.extend_from_slice(body);
            }
            let binding = compute_execution_hash(&entries);
            out[EXECUTION_HASH_OFFSET..EXECUTION_HASH_OFFSET + 32].copy_from_slice(&binding);
            out
        }

        /// Build a v2 file from section payloads (writer for tests and
        /// the future encoder): lays out sections 8-aligned after the
        /// directory and computes every content hash.
        pub fn build_file(payloads: &[(u16, &[u8])]) -> Vec<u8> {
            use sha2::Digest;
            let section_count = payloads.len() as u32;
            let dir_end = HEADER_SIZE + DIR_ENTRY_SIZE * payloads.len() as u64;
            let mut out = Vec::new();
            out.extend_from_slice(MAGIC);
            out.extend_from_slice(&super::FORMAT_VERSION.to_le_bytes());
            out.extend_from_slice(&(HEADER_SIZE as u32).to_le_bytes());
            out.extend_from_slice(&0u64.to_le_bytes()); // total_size patched later
            out.extend_from_slice(&section_count.to_le_bytes());
            out.extend_from_slice(&0u32.to_le_bytes());
            out.extend_from_slice(&[0u8; 32]);
            // section bodies first to learn offsets
            let mut bodies = Vec::new();
            let mut offset = dir_end;
            for (_, body) in payloads {
                while !offset.is_multiple_of(SECTION_ALIGN) {
                    offset += 1;
                }
                bodies.push((offset, body));
                offset += body.len() as u64;
            }
            let total = offset;
            out[16..24].copy_from_slice(&total.to_le_bytes());
            for (i, (id, body)) in payloads.iter().enumerate() {
                let base = (HEADER_SIZE + DIR_ENTRY_SIZE * i as u64) as usize;
                out.extend_from_slice(&vec![0u8; base - out.len()]);
                out.extend_from_slice(&id.to_le_bytes());
                out.extend_from_slice(&0u16.to_le_bytes()); // flags
                out.extend_from_slice(&0u32.to_le_bytes()); // reserved
                let (sec_off, _) = bodies[i];
                out.extend_from_slice(&sec_off.to_le_bytes());
                out.extend_from_slice(&(body.len() as u64).to_le_bytes());
                out.extend_from_slice(&sha2::Sha256::digest(body));
            }
            for (sec_off, body) in &bodies {
                while (out.len() as u64) < *sec_off {
                    out.push(0);
                }
                out.extend_from_slice(body);
            }
            out
        }
    }

    /// M26-003-B: graph sections. Stores the verified runtime graph —
    /// router nodes/edges/terminals (0x0002), RoutePlans (0x0003), the
    /// schema manifest (0x0004), policy plans (0x0005), the function
    /// manifest, and capabilities — over a shared strings table.
    /// Property-equivalent: decoding reproduces the exact encoded
    /// structures. The full file wrapper (header/directory/integrity)
    /// lands in M26-003-C/D.
    pub mod graph {
        use super::super::{
            FieldNeeds, RoutePlanDecl, SchemaDecl, SerializedRouter, SerializedRouterNode,
            SerializedStaticEdge, SerializedTerminal, Strategy,
        };

        /// Sanity bound on router sizes (malformed input rejects before
        /// any large allocation).
        pub const MAX_NODES: u32 = 1 << 20;

        pub fn opt_u32(v: Option<u32>) -> u32 {
            v.unwrap_or(super::NONE_REF)
        }

        /// Shared string table builder: dedups interned strings and
        /// hands out dense refs.
        #[derive(Default)]
        pub struct Strings {
            items: Vec<String>,
        }
        impl Strings {
            pub fn new() -> Self {
                Self::default()
            }
            pub fn intern(&mut self, s: &str) -> u32 {
                if let Some(i) = self.items.iter().position(|x| x == s) {
                    return i as u32;
                }
                self.items.push(s.to_string());
                (self.items.len() - 1) as u32
            }
            pub fn finish(self) -> Vec<String> {
                self.items
            }
        }

        // ---- router section (0x0002): nodes/edges/terminals ----
        pub mod router_section {
            use super::*;

            pub fn encode(nodes: &[SerializedRouterNode], strings: &mut Strings) -> Vec<u8> {
                let mut out = Vec::new();
                out.extend_from_slice(&(nodes.len() as u32).to_le_bytes());
                for n in nodes {
                    out.extend_from_slice(&(n.static_edges.len() as u16).to_le_bytes());
                    out.extend_from_slice(&opt_u32(n.param_edge.map(|v| v as u32)).to_le_bytes());
                    out.extend_from_slice(
                        &opt_u32(n.wildcard_edge.map(|v| v as u32)).to_le_bytes(),
                    );
                    match &n.terminal {
                        Some(t) => {
                            out.push(1);
                            out.extend_from_slice(&t.method_mask.to_le_bytes());
                            for slot in &t.route_by_method {
                                out.extend_from_slice(
                                    &opt_u32(slot.map(|v| v as u32)).to_le_bytes(),
                                );
                            }
                        }
                        None => out.push(0),
                    }
                    for e in &n.static_edges {
                        out.extend_from_slice(&strings.intern(&e.segment).to_le_bytes());
                        out.extend_from_slice(&(e.target_node as u32).to_le_bytes());
                    }
                }
                out
            }

            pub fn decode(bytes: &[u8], strings: &[String]) -> Result<SerializedRouter, String> {
                let u32at = |off: usize| -> Result<u32, String> {
                    bytes
                        .get(off..off + 4)
                        .map(|b| u32::from_le_bytes(b.try_into().unwrap()))
                        .ok_or_else(|| "router section truncated".to_string())
                };
                let u16at = |off: usize| -> Result<u16, String> {
                    bytes
                        .get(off..off + 2)
                        .map(|b| u16::from_le_bytes(b.try_into().unwrap()))
                        .ok_or_else(|| "router section truncated".to_string())
                };
                let none = |v: u32| -> Result<Option<usize>, String> {
                    if v == super::super::NONE_REF {
                        Ok(None)
                    } else if v >= MAX_NODES {
                        Err(format!("router ref {v} out of sane range"))
                    } else {
                        Ok(Some(v as usize))
                    }
                };
                let node_count = u32at(0)?;
                if node_count >= MAX_NODES {
                    return Err("router node count out of sane range".to_string());
                }
                let mut nodes = Vec::with_capacity(node_count as usize);
                let mut pos = 4usize;
                for n_idx in 0..node_count as usize {
                    let static_count = u16at(pos)? as usize;
                    let param_ref = u32at(pos + 2)?;
                    let wildcard_ref = u32at(pos + 6)?;
                    let has_terminal = *bytes
                        .get(pos + 10)
                        .ok_or_else(|| "router section truncated".to_string())?;
                    pos += 11;
                    let mut terminal = None;
                    if has_terminal == 1 {
                        let method_mask = u16at(pos)?;
                        pos += 2;
                        let mut slots = [None::<usize>; 7];
                        for slot in slots.iter_mut() {
                            *slot = none(u32at(pos)?)?;
                            pos += 4;
                        }
                        terminal = Some(SerializedTerminal {
                            method_mask,
                            route_by_method: slots,
                        });
                    } else if has_terminal != 0 {
                        return Err(format!("router node {n_idx}: bad terminal flag"));
                    }
                    let mut static_edges = Vec::new();
                    for _ in 0..static_count {
                        let seg_ref = u32at(pos)?;
                        let target = u32at(pos + 4)?;
                        pos += 8;
                        let segment = strings
                            .get(seg_ref as usize)
                            .ok_or_else(|| {
                                format!("router node {n_idx}: segment ref out of bounds")
                            })?
                            .clone();
                        let target_node = none(target)?
                            .ok_or_else(|| format!("router node {n_idx}: NONE_REF edge target"))?;
                        static_edges.push(SerializedStaticEdge {
                            segment,
                            target_node,
                        });
                    }
                    nodes.push(SerializedRouterNode {
                        static_edges,
                        param_edge: none(param_ref)?,
                        wildcard_edge: none(wildcard_ref)?,
                        terminal,
                    });
                }
                if pos != bytes.len() {
                    return Err("router section has trailing bytes".to_string());
                }
                Ok(SerializedRouter { nodes })
            }
        }

        // ---- RoutePlans section (0x0003) ----
        pub mod plans_section {
            use super::*;

            fn push_ids(out: &mut Vec<u8>, ids: &[u32]) {
                out.extend_from_slice(&(ids.len() as u16).to_le_bytes());
                for id in ids {
                    out.extend_from_slice(&id.to_le_bytes());
                }
            }

            pub fn encode(plans: &[RoutePlanDecl], strings: &mut Strings) -> Vec<u8> {
                let mut out = Vec::new();
                out.extend_from_slice(&(plans.len() as u32).to_le_bytes());
                for p in plans {
                    out.extend_from_slice(&p.route_id.to_le_bytes());
                    out.extend_from_slice(&p.handler_id.to_le_bytes());
                    out.extend_from_slice(&opt_u32(p.policy_id).to_le_bytes());
                    out.extend_from_slice(&opt_u32(p.policy_handler_id).to_le_bytes());
                    out.extend_from_slice(&opt_u32(p.params_schema_id).to_le_bytes());
                    out.extend_from_slice(&opt_u32(p.query_schema_id).to_le_bytes());
                    out.extend_from_slice(&opt_u32(p.headers_schema_id).to_le_bytes());
                    out.extend_from_slice(&opt_u32(p.body_schema_id).to_le_bytes());
                    out.extend_from_slice(&p.default_status.to_le_bytes());
                    out.push(match p.response_strategy {
                        Strategy::Native => 0u8,
                        Strategy::Js => 1u8,
                    });
                    let needs = p.field_needs;
                    let mut flags = 0u8;
                    if needs.params {
                        flags |= 1;
                    }
                    if needs.query {
                        flags |= 2;
                    }
                    if needs.headers {
                        flags |= 4;
                    }
                    if needs.body {
                        flags |= 8;
                    }
                    out.push(flags);
                    out.extend_from_slice(&(p.deadline_ms as u32).to_le_bytes());
                    out.extend_from_slice(
                        &opt_u32(
                            p.validation_fallback_reason
                                .as_ref()
                                .map(|s| strings.intern(s)),
                        )
                        .to_le_bytes(),
                    );
                    out.extend_from_slice(
                        &opt_u32(
                            p.response_fallback_reason
                                .as_ref()
                                .map(|s| strings.intern(s)),
                        )
                        .to_le_bytes(),
                    );
                    out.extend_from_slice(&(p.allowed_statuses.len() as u16).to_le_bytes());
                    for st in &p.allowed_statuses {
                        out.extend_from_slice(&st.to_le_bytes());
                    }
                    push_ids(&mut out, &p.header_name_ids);
                    push_ids(&mut out, &p.query_name_ids);
                    push_ids(&mut out, &p.cookie_name_ids);
                }
                out
            }

            pub fn decode(bytes: &[u8], strings: &[String]) -> Result<Vec<RoutePlanDecl>, String> {
                let u32at = |off: usize| -> Result<u32, String> {
                    bytes
                        .get(off..off + 4)
                        .map(|b| u32::from_le_bytes(b.try_into().unwrap()))
                        .ok_or_else(|| "plans section truncated".to_string())
                };
                let u16at = |off: usize| -> Result<u16, String> {
                    bytes
                        .get(off..off + 2)
                        .map(|b| u16::from_le_bytes(b.try_into().unwrap()))
                        .ok_or_else(|| "plans section truncated".to_string())
                };
                let some = |v: u32, strings: &[String]| -> Result<Option<String>, String> {
                    if v == super::super::NONE_REF {
                        Ok(None)
                    } else {
                        strings
                            .get(v as usize)
                            .map(|s| Some(s.clone()))
                            .ok_or_else(|| "plans: fallback reason ref out of bounds".to_string())
                    }
                };
                let read_ids = |pos: &mut usize| -> Result<Vec<u32>, String> {
                    let n = u16at(*pos)? as usize;
                    *pos += 2;
                    let mut ids = Vec::with_capacity(n.min(1 << 16));
                    for _ in 0..n {
                        ids.push(u32at(*pos)?);
                        *pos += 4;
                    }
                    Ok(ids)
                };
                let count = u32at(0)? as usize;
                if count > 1 << 20 {
                    return Err("plans count out of sane range".to_string());
                }
                let mut out = Vec::with_capacity(count);
                let mut pos = 4usize;
                for i in 0..count {
                    let route_id = u32at(pos)?;
                    let handler_id = u32at(pos + 4)?;
                    let policy_id = u32at(pos + 8)?;
                    let policy_handler_id = u32at(pos + 12)?;
                    let params_schema_id = u32at(pos + 16)?;
                    let query_schema_id = u32at(pos + 20)?;
                    let headers_schema_id = u32at(pos + 24)?;
                    let body_schema_id = u32at(pos + 28)?;
                    let default_status = u16at(pos + 32)?;
                    let strategy = *bytes.get(pos + 34).ok_or("plans section truncated")?;
                    let flags = *bytes.get(pos + 35).ok_or("plans section truncated")?;
                    let deadline_ms = u32at(pos + 36)? as u64;
                    let val_reason_ref = u32at(pos + 40)?;
                    let resp_reason_ref = u32at(pos + 44)?;
                    pos += 48;
                    let status_count = u16at(pos)? as usize;
                    pos += 2;
                    let mut allowed_statuses = Vec::with_capacity(status_count.min(1 << 12));
                    for _ in 0..status_count {
                        allowed_statuses.push(u16at(pos)?);
                        pos += 2;
                    }
                    let header_name_ids = read_ids(&mut pos)?;
                    let query_name_ids = read_ids(&mut pos)?;
                    let cookie_name_ids = read_ids(&mut pos)?;
                    let response_strategy = match strategy {
                        0 => Strategy::Native,
                        1 => Strategy::Js,
                        other => return Err(format!("plan {i}: bad strategy byte {other}")),
                    };
                    let opt = |v: u32| -> Option<u32> {
                        if v == super::super::NONE_REF {
                            None
                        } else {
                            Some(v)
                        }
                    };
                    out.push(RoutePlanDecl {
                        route_id,
                        handler_id,
                        policy_id: opt(policy_id),
                        policy_handler_id: opt(policy_handler_id),
                        params_schema_id: opt(params_schema_id),
                        query_schema_id: opt(query_schema_id),
                        headers_schema_id: opt(headers_schema_id),
                        body_schema_id: opt(body_schema_id),
                        header_name_ids,
                        query_name_ids,
                        cookie_name_ids,
                        default_status,
                        allowed_statuses,
                        field_needs: FieldNeeds {
                            params: flags & 1 != 0,
                            query: flags & 2 != 0,
                            headers: flags & 4 != 0,
                            body: flags & 8 != 0,
                        },
                        response_strategy,
                        validation_fallback_reason: some(val_reason_ref, strings)?,
                        response_fallback_reason: some(resp_reason_ref, strings)?,
                        deadline_ms,
                    });
                }
                if pos != bytes.len() {
                    return Err("plans section has trailing bytes".to_string());
                }
                Ok(out)
            }
        }

        // ---- schema manifest section (0x0004): dense envelope; the IR
        // node itself travels as canonical JSON until the binary IR
        // codec lands (property-equivalent, semantics unchanged) ----
        pub mod schemas_section {
            use super::*;

            pub fn encode(decls: &[SchemaDecl], strings: &mut Strings) -> Vec<u8> {
                let mut out = Vec::new();
                out.extend_from_slice(&(decls.len() as u32).to_le_bytes());
                for d in decls {
                    out.extend_from_slice(&d.id.to_le_bytes());
                    out.extend_from_slice(&strings.intern(&d.key).to_le_bytes());
                    out.extend_from_slice(&(d.features.len() as u16).to_le_bytes());
                    for f in &d.features {
                        out.extend_from_slice(&strings.intern(f).to_le_bytes());
                    }
                    let ir = serde_json::to_string(&d.ir).unwrap_or_default();
                    out.extend_from_slice(&(ir.len() as u32).to_le_bytes());
                    out.extend_from_slice(ir.as_bytes());
                }
                out
            }

            pub fn decode(bytes: &[u8], strings: &[String]) -> Result<Vec<SchemaDecl>, String> {
                let u32at = |off: usize| -> Result<u32, String> {
                    bytes
                        .get(off..off + 4)
                        .map(|b| u32::from_le_bytes(b.try_into().unwrap()))
                        .ok_or_else(|| "schemas section truncated".to_string())
                };
                let u16at = |off: usize| -> Result<u16, String> {
                    bytes
                        .get(off..off + 2)
                        .map(|b| u16::from_le_bytes(b.try_into().unwrap()))
                        .ok_or_else(|| "schemas section truncated".to_string())
                };
                let get = |v: u32| -> Result<&String, String> {
                    strings
                        .get(v as usize)
                        .ok_or_else(|| "schemas: string ref out of bounds".to_string())
                };
                let count = u32at(0)? as usize;
                if count > 1 << 20 {
                    return Err("schemas count out of sane range".to_string());
                }
                let mut out = Vec::with_capacity(count);
                let mut pos = 4usize;
                for _ in 0..count {
                    let id = u32at(pos)?;
                    let key = get(u32at(pos + 4)?)?.clone();
                    pos += 8;
                    let fcount = u16at(pos)? as usize;
                    pos += 2;
                    let mut features = Vec::with_capacity(fcount.min(64));
                    for _ in 0..fcount {
                        features.push(get(u32at(pos)?)?.clone());
                        pos += 4;
                    }
                    let ir_len = u32at(pos)? as usize;
                    pos += 4;
                    let ir_bytes = bytes
                        .get(pos..pos + ir_len)
                        .ok_or_else(|| "schemas: IR blob truncated".to_string())?;
                    pos += ir_len;
                    let ir = serde_json::from_slice(ir_bytes)
                        .map_err(|e| format!("schemas: IR blob is not valid IR JSON: {e}"))?;
                    out.push(SchemaDecl {
                        id,
                        key,
                        features,
                        ir,
                    });
                }
                if pos != bytes.len() {
                    return Err("schemas section has trailing bytes".to_string());
                }
                Ok(out)
            }
        }

        // ---- policy plans (0x0005): PolicyEntry + the dense manifest ----
        pub mod policy_section {
            use super::super::super::{PolicyDecl, PolicyEntry};
            use super::super::policies_table;
            use super::*;

            pub fn encode(
                entries: &[(String, PolicyEntry)],
                manifest: &[super::super::super::PolicyDecl],
                strings: &mut Strings,
            ) -> Vec<u8> {
                let rows: Vec<crate::qpack2::PolicyRow> = entries
                    .iter()
                    .map(|(id, e)| {
                        (
                            strings.intern(id),
                            strings.intern(&e.handler),
                            e.provides
                                .as_ref()
                                .map(|p| strings.intern(p))
                                .unwrap_or(super::super::NONE_REF),
                            e.declared_statuses.clone(),
                        )
                    })
                    .collect();
                let mut out = policies_table::encode(&rows);
                out.extend_from_slice(&(manifest.len() as u32).to_le_bytes());
                for d in manifest {
                    out.extend_from_slice(&d.id.to_le_bytes());
                    out.extend_from_slice(&strings.intern(&d.key).to_le_bytes());
                    out.extend_from_slice(&d.handler_id.to_le_bytes());
                }
                out
            }

            /// Decode rows + manifest: walks the row records to find the
            /// manifest boundary, then decodes each part.
            pub fn decode(
                bytes: &[u8],
                strings: &[String],
            ) -> Result<(Vec<crate::qpack2::PolicyRow>, Vec<PolicyDecl>), String> {
                let u32at = |off: usize| -> Result<u32, String> {
                    bytes
                        .get(off..off + 4)
                        .map(|b| u32::from_le_bytes(b.try_into().unwrap()))
                        .ok_or_else(|| "policy section truncated".to_string())
                };
                let u16at = |off: usize| -> Result<u16, String> {
                    bytes
                        .get(off..off + 2)
                        .map(|b| u16::from_le_bytes(b.try_into().unwrap()))
                        .ok_or_else(|| "policy section truncated".to_string())
                };
                let row_count = u32at(0)? as usize;
                let mut pos = 4usize;
                for _ in 0..row_count {
                    pos += 14; // id, handler, provides, status_count
                    let n = u16at(pos - 2)? as usize;
                    pos += n * 2;
                }
                let rows = policies_table::decode(&bytes[..pos])?;
                let manifest_count = u32at(pos)? as usize;
                pos += 4;
                let mut manifest = Vec::with_capacity(manifest_count.min(1 << 16));
                for i in 0..manifest_count {
                    let id = u32at(pos)?;
                    let key_ref = u32at(pos + 4)?;
                    let handler_id = u32at(pos + 8)?;
                    pos += 12;
                    let key = strings
                        .get(key_ref as usize)
                        .ok_or_else(|| format!("policy manifest {i}: key ref out of bounds"))?
                        .clone();
                    manifest.push(PolicyDecl {
                        id,
                        key,
                        handler_id,
                    });
                }
                if pos != bytes.len() {
                    return Err("policy section has trailing bytes".to_string());
                }
                Ok((rows, manifest))
            }
        }

        // ---- bundle bytecode (0x0007): RAW quickjs-ng module bytecode ----
        //
        // The v1 pack embeds bytecode as a base64 String inside JSON, which
        // the runtime must base64-decode before loading. This section stores
        // the engine-target metadata as string refs plus the bytecode bytes
        // verbatim — no base64 anywhere on the v2 path.
        pub mod bytecode_section {
            use super::super::super::BytecodeTarget;
            use super::*;

            /// Sanity bound on declared bytecode length (constraint 11);
            /// decode rejects larger counts before allocating.
            pub const MAX_CODE_BYTES: u32 = 1 << 28;

            /// Engine-target metadata carried beside the raw bytecode — the
            /// `BundleBytecode` fields minus the base64 `data`.
            #[derive(Debug, Clone, PartialEq, Eq, Default)]
            pub struct BytecodeMeta {
                pub quickjs: String,
                pub binding: String,
                /// "little" | "big"
                pub endianness: String,
                pub target: Option<BytecodeTarget>,
            }

            fn put_ref(out: &mut Vec<u8>, strings: &mut Strings, s: &str) {
                out.extend_from_slice(&strings.intern(s).to_le_bytes());
            }

            fn get_ref(
                bytes: &[u8],
                off: usize,
                strings: &[String],
                what: &str,
            ) -> Result<String, String> {
                let raw = bytes
                    .get(off..off + 4)
                    .map(|b| u32::from_le_bytes(b.try_into().unwrap()))
                    .ok_or_else(|| "bytecode section truncated".to_string())?;
                strings
                    .get(raw as usize)
                    .map(|s| s.as_str())
                    .ok_or_else(|| format!("bytecode section: {what} ref out of bounds"))
                    .map(|s| s.to_string())
            }

            /// Payload layout: quickjs_ref u32, binding_ref u32,
            /// endianness_ref u32, has_target u8 [target: arch_ref u32,
            /// os_ref u32, pointer_width u8, endianness_ref u32],
            /// code_len u32, code bytes verbatim.
            pub fn encode(meta: &BytecodeMeta, code: &[u8], strings: &mut Strings) -> Vec<u8> {
                let mut out = Vec::new();
                put_ref(&mut out, strings, &meta.quickjs);
                put_ref(&mut out, strings, &meta.binding);
                put_ref(&mut out, strings, &meta.endianness);
                match &meta.target {
                    Some(t) => {
                        out.push(1);
                        put_ref(&mut out, strings, &t.arch);
                        put_ref(&mut out, strings, &t.os);
                        out.push(t.pointer_width);
                        put_ref(&mut out, strings, &t.endianness);
                    }
                    None => out.push(0),
                }
                out.extend_from_slice(&(code.len() as u32).to_le_bytes());
                out.extend_from_slice(code);
                out
            }

            pub fn decode(
                bytes: &[u8],
                strings: &[String],
            ) -> Result<(BytecodeMeta, Vec<u8>), String> {
                let quickjs = get_ref(bytes, 0, strings, "quickjs")?;
                let binding = get_ref(bytes, 4, strings, "binding")?;
                let endianness = get_ref(bytes, 8, strings, "endianness")?;
                let has_target = *bytes
                    .get(12)
                    .ok_or_else(|| "bytecode section truncated".to_string())?;
                let mut pos = 13usize;
                let target = if has_target == 1 {
                    let arch = get_ref(bytes, pos, strings, "target arch")?;
                    let os = get_ref(bytes, pos + 4, strings, "target os")?;
                    let pointer_width = *bytes
                        .get(pos + 8)
                        .ok_or_else(|| "bytecode section truncated".to_string())?;
                    let t_endianness = get_ref(bytes, pos + 9, strings, "target endianness")?;
                    if ![4u8, 8].contains(&pointer_width) {
                        return Err(format!(
                            "bytecode target pointer width {pointer_width} is not 4 or 8"
                        ));
                    }
                    pos += 13;
                    Some(BytecodeTarget {
                        arch,
                        os,
                        pointer_width,
                        endianness: t_endianness,
                    })
                } else if has_target == 0 {
                    None
                } else {
                    return Err("bytecode section target flag is neither 0 nor 1".into());
                };
                let code_len = bytes
                    .get(pos..pos + 4)
                    .map(|b| u32::from_le_bytes(b.try_into().unwrap()))
                    .ok_or_else(|| "bytecode section truncated".to_string())?;
                if code_len > MAX_CODE_BYTES {
                    return Err(format!(
                        "bytecode length {code_len} exceeds sane bound {MAX_CODE_BYTES}"
                    ));
                }
                pos += 4;
                let end = pos + code_len as usize;
                let code = bytes
                    .get(pos..end)
                    .ok_or_else(|| "bytecode section truncated (code_len past end)".to_string())?
                    .to_vec();
                if end != bytes.len() {
                    return Err("bytecode section has trailing bytes".to_string());
                }
                Ok((
                    BytecodeMeta {
                        quickjs,
                        binding,
                        endianness,
                        target,
                    },
                    code,
                ))
            }
        }
    }

    // ---- section id catalog (spec §6; ids reserved by M26-001-B) ----
    pub mod section {
        pub const STRINGS: u16 = 0x0001;
        pub const ROUTES: u16 = 0x0002;
        pub const ROUTE_PLANS: u16 = 0x0003;
        pub const SCHEMA_MANIFEST: u16 = 0x0004;
        pub const POLICIES: u16 = 0x0005;
        pub const CAPABILITIES: u16 = 0x0006;
        pub const BUNDLE_BYTECODE: u16 = 0x0007;
        pub const CONTRACT_SUMMARY: u16 = 0x0008;
        /// Required ids: a pack missing any of these rejects (spec §6).
        pub const REQUIRED: &[u16] = &[
            STRINGS,
            ROUTES,
            ROUTE_PLANS,
            SCHEMA_MANIFEST,
            POLICIES,
            CAPABILITIES,
            CONTRACT_SUMMARY,
        ];
    }

    /// Sentinel for "absent" in dense u32 reference fields (string refs,
    /// schema ids, policy ids). Any other u32::MAX-adjacent value is
    /// invalid; dense encodings never use Option.
    pub const NONE_REF: u32 = 0xFFFF_FFFF;

    /// M26-003-A: dense string table (section 0x0001). Layout:
    /// `count: u32` then per string `len: u32` + UTF-8 bytes; offsets are
    /// implicit by running position. Reads are bounds-checked slices.
    pub mod strings_table {
        pub fn encode(items: &[String]) -> Vec<u8> {
            let mut out = Vec::new();
            out.extend_from_slice(&(items.len() as u32).to_le_bytes());
            for s in items {
                out.extend_from_slice(&(s.len() as u32).to_le_bytes());
                out.extend_from_slice(s.as_bytes());
            }
            out
        }

        /// Decode with bounds checks; every length must land inside the
        /// section and every string must be valid UTF-8.
        pub fn decode(bytes: &[u8]) -> Result<Vec<String>, String> {
            let read_u32 = |off: usize| -> Result<u32, String> {
                bytes
                    .get(off..off + 4)
                    .map(|b| u32::from_le_bytes(b.try_into().unwrap()))
                    .ok_or_else(|| "strings table truncated".to_string())
            };
            let count = read_u32(0)? as usize;
            let mut items = Vec::with_capacity(count.min(1 << 16));
            let mut pos = 4usize;
            for _ in 0..count {
                let len = read_u32(pos)? as usize;
                pos += 4;
                let slice = bytes
                    .get(pos..pos + len)
                    .ok_or_else(|| "strings table truncated".to_string())?;
                let s = String::from_utf8(slice.to_vec())
                    .map_err(|_| "strings table entry is not UTF-8".to_string())?;
                items.push(s);
                pos += len;
            }
            if pos != bytes.len() {
                return Err("strings table has trailing bytes".to_string());
            }
            Ok(items)
        }
    }

    /// M26-003-A: dense function manifest (lives inside the routes
    /// section's fixed header): `count: u32` then per function
    /// `id: u32, key: u32 (string ref), kind: u8`.
    pub mod functions_table {
        pub fn encode(fns: &[super::super::FunctionDecl], keys: &[u32]) -> Vec<u8> {
            let mut out = Vec::new();
            out.extend_from_slice(&(fns.len() as u32).to_le_bytes());
            for (f, key) in fns.iter().zip(keys) {
                out.extend_from_slice(&f.id.to_le_bytes());
                out.extend_from_slice(&key.to_le_bytes());
                let kind = match f.kind {
                    super::super::FunctionKind::RouteHandler => 0u8,
                    super::super::FunctionKind::PolicyHandler => 1u8,
                };
                out.push(kind);
            }
            out
        }

        pub fn decode(
            bytes: &[u8],
            strings: &[String],
        ) -> Result<Vec<super::super::FunctionDecl>, String> {
            let read_u32 = |off: usize| -> Result<u32, String> {
                bytes
                    .get(off..off + 4)
                    .map(|b| u32::from_le_bytes(b.try_into().unwrap()))
                    .ok_or_else(|| "functions table truncated".to_string())
            };
            let count = read_u32(0)? as usize;
            let mut out = Vec::with_capacity(count.min(1 << 16));
            let mut pos = 4usize;
            for i in 0..count {
                let id = read_u32(pos)?;
                let key_ref = read_u32(pos + 4)?;
                let kind = *bytes
                    .get(pos + 8)
                    .ok_or_else(|| "functions table truncated".to_string())?;
                pos += 9;
                let key = strings
                    .get(key_ref as usize)
                    .ok_or_else(|| format!("function {i}: key string ref out of bounds"))?
                    .clone();
                let kind = match kind {
                    0 => super::super::FunctionKind::RouteHandler,
                    1 => super::super::FunctionKind::PolicyHandler,
                    _ => return Err(format!("function {i}: unknown kind byte {kind}")),
                };
                out.push(super::super::FunctionDecl { id, key, kind });
            }
            if pos != bytes.len() {
                return Err("functions table has trailing bytes".to_string());
            }
            Ok(out)
        }
    }

    /// Dense policy row: (id ref, handler ref, provides ref | NONE_REF,
    /// declared statuses).
    pub type PolicyRow = (u32, u32, u32, Vec<u16>);

    /// M26-003-A: dense policy section (0x0005): `count: u32` then per
    /// policy `id: u32 (string ref), handler: u32 (string ref),
    /// provides: u32 (string ref | NONE_REF),
    /// status_count: u16` + `u16` statuses.
    pub mod policies_table {
        pub fn encode(entries: &[(u32, u32, u32, Vec<u16>)]) -> Vec<u8> {
            let mut out = Vec::new();
            out.extend_from_slice(&(entries.len() as u32).to_le_bytes());
            for (id, handler, provides, statuses) in entries {
                out.extend_from_slice(&id.to_le_bytes());
                out.extend_from_slice(&handler.to_le_bytes());
                out.extend_from_slice(&provides.to_le_bytes());
                out.extend_from_slice(&(statuses.len() as u16).to_le_bytes());
                for s in statuses {
                    out.extend_from_slice(&s.to_le_bytes());
                }
            }
            out
        }

        pub fn decode(bytes: &[u8]) -> Result<Vec<super::PolicyRow>, String> {
            let read_u32 = |off: usize| -> Result<u32, String> {
                bytes
                    .get(off..off + 4)
                    .map(|b| u32::from_le_bytes(b.try_into().unwrap()))
                    .ok_or_else(|| "policies table truncated".to_string())
            };
            let read_u16 = |off: usize| -> Result<u16, String> {
                bytes
                    .get(off..off + 2)
                    .map(|b| u16::from_le_bytes(b.try_into().unwrap()))
                    .ok_or_else(|| "policies table truncated".to_string())
            };
            let count = read_u32(0)? as usize;
            let mut out = Vec::with_capacity(count.min(1 << 16));
            let mut pos = 4usize;
            for _ in 0..count {
                let id = read_u32(pos)?;
                let handler = read_u32(pos + 4)?;
                let provides = read_u32(pos + 8)?;
                let n = read_u16(pos + 12)? as usize;
                pos += 14;
                let mut statuses = Vec::with_capacity(n.min(1024));
                for _ in 0..n {
                    statuses.push(read_u16(pos)?);
                    pos += 2;
                }
                out.push((id, handler, provides, statuses));
            }
            if pos != bytes.len() {
                return Err("policies table has trailing bytes".to_string());
            }
            Ok(out)
        }
    }

    /// M26-003-A: dense capability manifest (0x0006): `count: u32` then
    /// per capability `name: u32 (string ref)`. The capability hash
    /// (M26-002-A) is computed over the same sorted names.
    pub mod capabilities_table {
        pub fn encode(name_refs: &[u32]) -> Vec<u8> {
            let mut out = Vec::new();
            out.extend_from_slice(&(name_refs.len() as u32).to_le_bytes());
            for r in name_refs {
                out.extend_from_slice(&r.to_le_bytes());
            }
            out
        }

        pub fn decode(bytes: &[u8]) -> Result<Vec<u32>, String> {
            let read_u32 = |off: usize| -> Result<u32, String> {
                bytes
                    .get(off..off + 4)
                    .map(|b| u32::from_le_bytes(b.try_into().unwrap()))
                    .ok_or_else(|| "capabilities table truncated".to_string())
            };
            let count = read_u32(0)? as usize;
            let mut out = Vec::with_capacity(count.min(1024));
            for i in 0..count {
                out.push(read_u32(4 + i * 4)?);
            }
            if 4 + count * 4 != bytes.len() {
                return Err("capabilities table size mismatch".to_string());
            }
            Ok(out)
        }
    }

    /// M26-003-A: dense contract summary (0x0008): `contract_hash: u32
    /// (string ref), route_count: u32, format_revision: u32`. The hash
    /// string itself lives in the strings table.
    pub mod contract_summary {
        pub fn encode(hash_ref: u32, route_count: u32) -> Vec<u8> {
            let mut out = Vec::new();
            out.extend_from_slice(&hash_ref.to_le_bytes());
            out.extend_from_slice(&route_count.to_le_bytes());
            out.extend_from_slice(&1u32.to_le_bytes());
            out
        }

        pub fn decode(bytes: &[u8]) -> Result<(u32, u32, u32), String> {
            if bytes.len() != 12 {
                return Err("contract summary must be exactly 12 bytes".to_string());
            }
            let g = |off: usize| u32::from_le_bytes(bytes[off..off + 4].try_into().unwrap());
            Ok((g(0), g(4), g(8)))
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        // Spec §2/§3 pins: header and directory strides are exactly 64
        // bytes and everything downstream stays 8-aligned. Changing either
        // is a new numeric mode, never an in-place patch.
        #[test]
        fn layout_constants_match_spec() {
            assert_eq!(MAGIC, b"VELQUQPK");
            assert_eq!(FORMAT_VERSION, 2);
            assert_eq!(HEADER_SIZE % SECTION_ALIGN, 0);
            assert_eq!(DIR_ENTRY_SIZE % SECTION_ALIGN, 0);
            let dir_end = HEADER_SIZE + DIR_ENTRY_SIZE * 7;
            assert_eq!(dir_end % SECTION_ALIGN, 0);
            assert_eq!(FLAG_OPTIONAL, 1);
        }

        // ---- M26-003-A: dense section schema evidence ----

        fn demo_tables() -> (
            Vec<String>,
            Vec<crate::FunctionDecl>,
            Vec<PolicyRow>,
            Vec<u32>,
        ) {
            let strings = vec![
                "health.live".to_string(),
                "auth.session.check".to_string(),
                "orders.cancel".to_string(),
                "cc17c22537562df74b3179148edffc5b".to_string(),
                "timer".to_string(),
            ];
            let functions = vec![
                crate::FunctionDecl {
                    id: 0,
                    key: "health.live".into(),
                    kind: crate::FunctionKind::RouteHandler,
                },
                crate::FunctionDecl {
                    id: 1,
                    key: "auth.session.check".into(),
                    kind: crate::FunctionKind::PolicyHandler,
                },
            ];
            let policies = vec![(0u32, 1u32, super::NONE_REF, vec![401u16])];
            let caps = vec![4u32];
            (strings, functions, policies, caps)
        }

        #[allow(clippy::type_complexity)]
        fn encode_all() -> [Vec<u8>; 6] {
            let (strings, functions, policies, _caps) = demo_tables();
            let string_refs_of = |names: &[&str], table: &[String]| -> Vec<u32> {
                names
                    .iter()
                    .map(|n| table.iter().position(|s| s == n).unwrap() as u32)
                    .collect()
            };
            let fn_keys: Vec<u32> =
                string_refs_of(&["health.live", "auth.session.check"], &strings);
            let pol: Vec<(u32, u32, u32, Vec<u16>)> = policies
                .iter()
                .map(|(_, h, p, st)| (0u32, *h, *p, st.clone()))
                .collect();
            let _ = &pol;
            [
                strings_table::encode(&strings),
                functions_table::encode(&functions, &fn_keys),
                policies_table::encode(&[(0, 1, super::NONE_REF, vec![401u16])]),
                capabilities_table::encode(&_caps),
                contract_summary::encode(3, 9),
                vec![1u8, 2, 3, 4, 5, 6, 7, 8], // placeholder 8-aligned payload
            ]
        }

        // Round-trip: every dense section decodes back to what was encoded.
        #[test]
        fn dense_sections_round_trip() {
            let [strings_b, fns_b, pol_b, caps_b, contract_b, _] = encode_all();
            let strings = strings_table::decode(&strings_b).unwrap();
            assert_eq!(strings.len(), 5);
            assert_eq!(strings[2], "orders.cancel");

            let fns = functions_table::decode(&fns_b, &strings).unwrap();
            assert_eq!(fns.len(), 2);
            assert_eq!(fns[1].key, "auth.session.check");
            assert_eq!(fns[1].kind, crate::FunctionKind::PolicyHandler);

            let pols = policies_table::decode(&pol_b).unwrap();
            assert_eq!(pols.len(), 1);
            assert_eq!(pols[0].3, vec![401u16]);
            assert_eq!(pols[0].2, super::NONE_REF);

            let caps = capabilities_table::decode(&caps_b).unwrap();
            assert_eq!(caps, vec![4u32]);

            let (hash_ref, routes, rev) = contract_summary::decode(&contract_b).unwrap();
            assert_eq!((hash_ref, routes, rev), (3, 9, 1));
        }

        // Property: empty tables are legal and round-trip.
        #[test]
        fn dense_sections_empty_round_trip() {
            assert_eq!(
                strings_table::decode(&strings_table::encode(&[])).unwrap(),
                Vec::<String>::new()
            );
            assert_eq!(
                functions_table::decode(&functions_table::encode(&[], &[]), &[]).unwrap(),
                Vec::<crate::FunctionDecl>::new()
            );
            assert_eq!(
                policies_table::decode(&policies_table::encode(&[])).unwrap(),
                Vec::<(u32, u32, u32, Vec<u16>)>::new()
            );
            assert_eq!(
                capabilities_table::decode(&capabilities_table::encode(&[])).unwrap(),
                Vec::<u32>::new()
            );
        }

        // Mutation fuzz: single-byte corruption of any section either
        // fails decode or is caught by strict trailing/UTF-8 checks —
        // never a panic, never silent success with different content
        // (deterministic Rng, no external corpus).
        #[test]
        fn dense_sections_never_panic_under_mutation() {
            struct Rng(u64);
            impl Rng {
                fn next(&mut self) -> u64 {
                    let mut x = self.0;
                    x ^= x << 13;
                    x ^= x >> 7;
                    x ^= x << 17;
                    self.0 = x;
                    x
                }
            }
            let [strings_b, fns_b, pol_b, caps_b, contract_b, _] = encode_all();
            let mut rng = Rng(0x00D_3EEE);
            let mut accepted_same = 0usize;
            let mut rejected = 0usize;
            for round in 0..2_000 {
                let pick = rng.next() % 5;
                let mut bytes = match pick {
                    0 => strings_b.clone(),
                    1 => fns_b.clone(),
                    2 => pol_b.clone(),
                    3 => caps_b.clone(),
                    _ => contract_b.clone(),
                };
                if bytes.is_empty() {
                    continue;
                }
                let idx = (rng.next() as usize) % bytes.len();
                bytes[idx] ^= 1u8 << (rng.next() % 8);
                let strings = strings_table::decode(if pick == 0 { &bytes } else { &strings_b })
                    .unwrap_or_default();
                let result = match pick {
                    0 => strings_table::decode(&bytes).map(|_| ()),
                    1 => functions_table::decode(&bytes, &strings).map(|_| ()),
                    2 => policies_table::decode(&bytes).map(|_| ()),
                    3 => capabilities_table::decode(&bytes).map(|_| ()),
                    _ => contract_summary::decode(&bytes).map(|_| ()),
                };
                match result {
                    Ok(()) => accepted_same += 1,
                    Err(_) => rejected += 1,
                }
                // note: a mutation that decodes OK must still be flagged by
                // the SECTION content hash at read time (spec §3 rule 6) —
                // this fuzz only proves totality (no panic, bounded work)
                let _ = round;
            }
            assert!(
                rejected > 100,
                "mutations must overwhelmingly reject: {rejected}"
            );
            assert!(accepted_same + rejected == 2_000);
        }

        // Section-size report: dense encodings are smaller than the v1
        // JSON equivalents for the demo corpus (the normative comparison
        // lands with the full encoder in M26-003-B).
        #[test]
        fn dense_section_size_report() {
            // Section-size REPORT (required evidence): measured sizes for
            // every dense section vs its v1 JSON equivalent on a
            // 25-record corpus. Honest findings:
            // - RECORD tables (functions, policies): dense is strictly
            //   smaller — fixed-width records + string refs beat JSON's
            //   repeated keys and field names.
            // - STRING tables: size-NEUTRAL for plain ASCII (JSON spends
            //   ~3 bytes/string on quotes+comma, dense spends 4 on the
            //   length prefix) — the dense string table's value is the
            //   shared-reference model, not per-string bytes. Escape-heavy
            //   content (quotes/backslashes in names) flips it decisively.
            let strings: Vec<String> = (0..25)
                .map(|i| format!("routes.orders.items.batch-{i}.handler.v{i}"))
                .collect();
            let json_strings = serde_json::to_string(&strings).unwrap().len();
            let dense_strings = strings_table::encode(&strings).len();

            let mut functions = Vec::new();
            let mut fn_keys = Vec::new();
            for i in 0..25u32 {
                functions.push(crate::FunctionDecl {
                    id: i,
                    key: format!("orders.items.batch-{i}.handler"),
                    kind: crate::FunctionKind::RouteHandler,
                });
                fn_keys.push(i);
            }
            let json_fns = serde_json::to_string(&functions).unwrap().len();
            let dense_fns = functions_table::encode(&functions, &fn_keys).len();

            let mut policies: Vec<super::PolicyRow> = Vec::new();
            let mut json_src = Vec::new();
            for i in 0..25u32 {
                policies.push((i, i, super::NONE_REF, vec![401u16]));
                json_src.push(crate::PolicyEntry {
                    id: format!("policy-{i}"),
                    handler: format!("policy-{i}.check"),
                    declared_statuses: vec![401],
                    provides: None,
                });
            }
            let json_pols = serde_json::to_string(&json_src).unwrap().len();
            let dense_pols = policies_table::encode(&policies).len();

            eprintln!(
                "section-size report (25 records): strings dense={dense_strings} json={json_strings} | functions dense={dense_fns} json={json_fns} | policies dense={dense_pols} json={json_pols}"
            );

            // record tables: strictly smaller (the dense win)
            assert!(
                dense_fns < json_fns,
                "functions dense {dense_fns} vs json {json_fns}"
            );
            assert!(
                dense_pols < json_pols,
                "policies dense {dense_pols} vs json {json_pols}"
            );
            // strings: parity within the 4-byte/string prefix overhead
            let per_string_overhead = 4i64 * strings.len() as i64;
            assert!(
                (dense_strings as i64 - json_strings as i64) <= per_string_overhead,
                "strings dense {dense_strings} vs json {json_strings}"
            );
        }

        // ---- M26-003-B: graph section round-trip/fuzz/report ----

        use super::super::{PolicyDecl, PolicyEntry, RoutePlanDecl, SchemaDecl};
        use super::graph::{plans_section, router_section, schemas_section, Strings};

        #[allow(clippy::type_complexity)]
        fn graph_fixture() -> (
            super::super::SerializedRouter,
            Vec<RoutePlanDecl>,
            Vec<SchemaDecl>,
            Vec<(String, PolicyEntry)>,
            Vec<PolicyDecl>,
        ) {
            // a three-node router: root -> static "users" -> param edge,
            // terminal on the param node claiming GET
            let router = super::super::SerializedRouter {
                nodes: vec![
                    super::super::SerializedRouterNode {
                        static_edges: vec![super::super::SerializedStaticEdge {
                            segment: "users".into(),
                            target_node: 1,
                        }],
                        param_edge: Some(2),
                        wildcard_edge: None,
                        terminal: None,
                    },
                    super::super::SerializedRouterNode::default(),
                    super::super::SerializedRouterNode {
                        static_edges: vec![],
                        param_edge: None,
                        wildcard_edge: None,
                        terminal: Some(super::super::SerializedTerminal {
                            method_mask: 0b1,
                            route_by_method: [Some(0), None, None, None, None, None, None],
                        }),
                    },
                ],
            };
            let plans = vec![RoutePlanDecl {
                route_id: 0,
                handler_id: 0,
                policy_id: Some(0),
                policy_handler_id: Some(1),
                params_schema_id: Some(3),
                query_schema_id: None,
                headers_schema_id: None,
                body_schema_id: Some(4),
                header_name_ids: vec![0, 1],
                query_name_ids: vec![],
                cookie_name_ids: vec![],
                default_status: 200,
                allowed_statuses: vec![200, 422],
                field_needs: super::super::FieldNeeds {
                    params: true,
                    query: false,
                    headers: false,
                    body: true,
                },
                response_strategy: super::super::Strategy::Native,
                validation_fallback_reason: None,
                response_fallback_reason: Some("measured".into()),
                deadline_ms: 5000,
            }];
            let schemas = vec![SchemaDecl {
                id: 3,
                key: "sch:users.get.params".into(),
                features: vec!["object".into(), "string".into()],
                ir: serde_json::from_str(r#"{"kind":"object","properties":{"id":{"kind":"string","pattern":"^usr_[0-9]+$"}},"required":["id"]}"#).unwrap(),
            }];
            let policies = vec![(
                "auth.session".to_string(),
                PolicyEntry {
                    id: "auth.session".into(),
                    handler: "auth.session.check".into(),
                    declared_statuses: vec![401],
                    provides: Some("session".into()),
                },
            )];
            let manifest = vec![PolicyDecl {
                id: 0,
                key: "auth.session".into(),
                handler_id: 1,
            }];
            (router, plans, schemas, policies, manifest)
        }

        #[test]
        fn graph_sections_round_trip() {
            let (router, plans, schemas, policies, manifest) = graph_fixture();
            let mut strings = Strings::new();
            let router_b = router_section::encode(&router.nodes, &mut strings);
            let plans_b = plans_section::encode(&plans, &mut strings);
            let schemas_b = schemas_section::encode(&schemas, &mut strings);
            let table = strings.finish();
            // policy section interns into its own pass seeded with the
            // same table so refs stay compatible
            let mut strings2 = Strings::new();
            for s in &table {
                strings2.intern(s);
            }
            let policy_b =
                super::graph::policy_section::encode(&policies, &manifest, &mut strings2);
            let table = strings2.finish();

            let r2 = router_section::decode(&router_b, &table).unwrap();
            assert_eq!(r2, router);
            let p2 = plans_section::decode(&plans_b, &table).unwrap();
            assert_eq!(p2, plans);
            let s2 = schemas_section::decode(&schemas_b, &table).unwrap();
            assert_eq!(s2, schemas);
            // policy rows + manifest decode as one section
            let (rows, manifest) = super::graph::policy_section::decode(&policy_b, &table).unwrap();
            assert_eq!(rows.len(), 1);
            assert_eq!(rows[0].3, vec![401u16]);
            assert_eq!(
                manifest,
                vec![PolicyDecl {
                    id: 0,
                    key: "auth.session".into(),
                    handler_id: 1
                }]
            );
        }

        #[test]
        fn binary_and_transitional_representations_agree() {
            // G2-p3: the v2 binary sections and the transitional JSON/serde
            // form must carry the identical verified graph — same structures
            // out of both decode paths, and the transitional form round-trips.
            let (router, plans, schemas, policies, manifest) = graph_fixture();
            let tj: super::super::SerializedRouter =
                serde_json::from_str(&serde_json::to_string(&router).unwrap()).unwrap();
            let tp: Vec<RoutePlanDecl> =
                serde_json::from_str(&serde_json::to_string(&plans).unwrap()).unwrap();
            let ts: Vec<SchemaDecl> =
                serde_json::from_str(&serde_json::to_string(&schemas).unwrap()).unwrap();
            assert_eq!(tj, router);
            assert_eq!(tp, plans);
            assert_eq!(ts, schemas);

            let mut strings = Strings::new();
            let router_b = router_section::encode(&router.nodes, &mut strings);
            let plans_b = plans_section::encode(&plans, &mut strings);
            let schemas_b = schemas_section::encode(&schemas, &mut strings);
            let table = strings.finish();
            let mut strings2 = Strings::new();
            for s in &table {
                strings2.intern(s);
            }
            let _policy_b =
                super::graph::policy_section::encode(&policies, &manifest, &mut strings2);
            let table = strings2.finish();

            let bj: super::super::SerializedRouter =
                router_section::decode(&router_b, &table).unwrap();
            let bp = plans_section::decode(&plans_b, &table).unwrap();
            let bs = schemas_section::decode(&schemas_b, &table).unwrap();
            assert_eq!(bj, tj);
            assert_eq!(bp, tp);
            assert_eq!(bs, ts);
        }

        #[test]
        fn graph_sections_mutation_never_panics() {
            struct Rng(u64);
            impl Rng {
                fn next(&mut self) -> u64 {
                    let mut x = self.0;
                    x ^= x << 13;
                    x ^= x >> 7;
                    x ^= x << 17;
                    self.0 = x;
                    x
                }
            }
            let (router, plans, schemas, _policies, _manifest) = graph_fixture();
            let mut rng = Rng(0xB00CA);
            let mut rejected = 0usize;
            let rounds = 3_000;
            for _ in 0..rounds {
                let mut strings = Strings::new();
                let (bytes, which) = match rng.next() % 3 {
                    0 => (router_section::encode(&router.nodes, &mut strings), 0),
                    1 => (plans_section::encode(&plans, &mut strings), 1),
                    _ => (schemas_section::encode(&schemas, &mut strings), 2),
                };
                let table = strings.finish();
                let mut bytes = bytes;
                let idx = (rng.next() as usize) % bytes.len();
                bytes[idx] ^= 1u8 << (rng.next() % 8);
                let result = match which {
                    0 => router_section::decode(&bytes, &table).map(|_| ()),
                    1 => plans_section::decode(&bytes, &table).map(|_| ()),
                    _ => schemas_section::decode(&bytes, &table).map(|_| ()),
                };
                if result.is_err() {
                    rejected += 1;
                }
            }
            // every mutation either rejects or decodes (some bit flips are
            // semantically legal values); no panic, bounded work — and the
            // section content hash catches legal-value flips at read time
            assert!(
                rejected > 500,
                "mutation must overwhelmingly reject: {rejected}"
            );
        }

        #[test]
        fn graph_section_size_report() {
            let (router, plans, schemas, policies, manifest) = graph_fixture();
            let mut strings = Strings::new();
            let router_b = router_section::encode(&router.nodes, &mut strings);
            let plans_b = plans_section::encode(&plans, &mut strings);
            let schemas_b = schemas_section::encode(&schemas, &mut strings);
            drop(strings.finish());
            let json_router = serde_json::to_string(&router).unwrap();
            let json_plans = serde_json::to_string(&plans).unwrap();
            let json_schemas = serde_json::to_string(&schemas).unwrap();
            let json_policies = serde_json::to_string(&policies).unwrap();
            eprintln!(
                "graph section sizes: router dense={} json={} | plans dense={} json={} | schemas dense={} json={} | policies json={}",
                router_b.len(), json_router.len(),
                plans_b.len(), json_plans.len(),
                schemas_b.len(), json_schemas.len(),
                json_policies.len(),
            );
            // structural report only: sizes recorded; the fixed-width
            // record win scales with record count (see M26-003-A report)
            assert!(!router_b.is_empty() && !plans_b.is_empty() && !schemas_b.is_empty());
            let _ = (policies, manifest);
        }

        // ---- M26-004-A: raw bytecode section ----

        use super::graph::bytecode_section::{self, BytecodeMeta};

        fn bytecode_payload() -> Vec<u8> {
            // binary-heavy bytes including values outside the base64
            // alphabet: if any base64 encode/decode happened on this path,
            // these bytes could not survive a round trip.
            (0..768u32)
                .map(|i| ((i * 37 + (i >> 4)) & 0xff) as u8)
                .collect()
        }

        fn bytecode_meta(with_target: bool) -> BytecodeMeta {
            BytecodeMeta {
                quickjs: "0.15.1".into(),
                binding: "sha256:abcd0123".into(),
                endianness: "little".into(),
                target: with_target.then(|| super::super::BytecodeTarget {
                    arch: "x86_64".into(),
                    os: "linux".into(),
                    pointer_width: 8,
                    endianness: "little".into(),
                }),
            }
        }

        #[test]
        fn bytecode_section_round_trips_raw_bytes() {
            for with_target in [true, false] {
                let meta = bytecode_meta(with_target);
                let code = bytecode_payload();
                let mut strings = Strings::new();
                let payload = bytecode_section::encode(&meta, &code, &mut strings);
                let table = strings.finish();
                let (m2, c2) = bytecode_section::decode(&payload, &table).unwrap();
                assert_eq!(m2, meta);
                assert_eq!(c2, code, "raw bytecode bytes must survive verbatim");
                // and the payload itself contains the raw bytes untransformed
                assert!(payload.windows(16).any(|w| w.iter().any(|b| *b > 127)));
            }
        }

        #[test]
        fn bytecode_section_rejects_drift_and_truncation() {
            let meta = bytecode_meta(true);
            let code = bytecode_payload();
            let mut strings = Strings::new();
            let payload = bytecode_section::encode(&meta, &code, &mut strings);
            let table = strings.finish();

            // truncated at every header boundary
            for cut in [0usize, 4, 8, 12, 13, 26, 30] {
                assert!(
                    bytecode_section::decode(&payload[..cut.min(payload.len())], &table).is_err(),
                    "cut at {cut} must reject"
                );
            }
            // trailing bytes
            let mut trailing = payload.clone();
            trailing.push(0);
            assert!(bytecode_section::decode(&trailing, &table).is_err());
            // code_len drift: shrink the declared length
            let mut drifted = payload.clone();
            let len_at = payload.len() - code.len() - 4;
            drifted[len_at..len_at + 4].copy_from_slice(&((code.len() as u32 - 1).to_le_bytes()));
            assert!(bytecode_section::decode(&drifted, &table).is_err());
            // bad target flag
            let mut flag = payload.clone();
            flag[12] = 2;
            assert!(bytecode_section::decode(&flag, &table).is_err());
            // out-of-bounds string ref
            let mut oob = payload.clone();
            oob[0..4].copy_from_slice(&0xffff_ff00u32.to_le_bytes());
            assert!(bytecode_section::decode(&oob, &table).is_err());
            // bad pointer width
            let mut pw = payload.clone();
            pw[13 + 8] = 7;
            assert!(bytecode_section::decode(&pw, &table).is_err());
            // declared code_len beyond the sane bound rejects before use
            let mut huge = payload.clone();
            huge[len_at..len_at + 4].copy_from_slice(&0xffff_ffffu32.to_le_bytes());
            assert!(bytecode_section::decode(&huge, &table).is_err());
        }

        #[test]
        fn bytecode_section_in_bound_file_and_tamper_rejected() {
            let meta = bytecode_meta(true);
            let code = bytecode_payload();
            let mut strings = Strings::new();
            let bc_payload = bytecode_section::encode(&meta, &code, &mut strings);
            let table = strings.finish();
            let strings_payload: Vec<u8> = table.join("\0").into_bytes();
            let payloads: Vec<(u16, &[u8])> = vec![
                (section::STRINGS, &strings_payload),
                (section::ROUTES, b"router-graph-payload"),
                (section::ROUTE_PLANS, b"plans-payload-xxxx"),
                (section::SCHEMA_MANIFEST, b"schemas-payload-xxxx"),
                (section::POLICIES, b"policies-payload-xxx"),
                (section::CAPABILITIES, b"caps-payload-bytes-xx"),
                (section::CONTRACT_SUMMARY, b"contract-summary12"),
                (section::BUNDLE_BYTECODE, &bc_payload),
            ];
            let file = reader::build_file_bound(&payloads);
            let entries = reader::parse_directory_with_binding(&file).unwrap();
            let sections = reader::validate(&file).unwrap();
            assert_eq!(sections.len(), 8);
            assert!(entries
                .iter()
                .any(|e| e.section_id == section::BUNDLE_BYTECODE));
            let bc = sections
                .iter()
                .find(|(e, _)| e.section_id == section::BUNDLE_BYTECODE)
                .unwrap();
            let (m2, c2) = bytecode_section::decode(bc.1, &table).unwrap();
            assert_eq!(m2, meta);
            assert_eq!(c2, code);

            // any payload byte change rejects via per-section sha256...
            let mut tampered = file.clone();
            let body_off = bc.0.offset as usize + bc.0.len as usize - 1;
            tampered[body_off] ^= 0xff;
            assert!(reader::validate(&tampered).is_err());
            // ...and even with the content hash repaired, the M26-003-D
            // execution-integrity binding still rejects
            let repaired = repair_section_hash(&tampered, &entries, section::BUNDLE_BYTECODE);
            assert!(reader::validate(&repaired).is_err());
            assert!(reader::parse_directory_with_binding(&repaired).is_err());
        }

        #[test]
        fn bytecode_base64_vs_raw_size_report() {
            // honest structural report: v1 stores base64 text in JSON
            // (~4/3 inflation plus JSON string quotes); the v2 section
            // stores the bytes verbatim plus a fixed metadata header.
            let code = bytecode_payload();
            let mut strings = Strings::new();
            let payload = bytecode_section::encode(&bytecode_meta(true), &code, &mut strings);
            drop(strings.finish());
            let b64_len = code.len().div_ceil(3) * 4;
            let v1_stored = b64_len + 2; // JSON string quotes
            let v2_stored = payload.len();
            eprintln!(
                "bytecode storage: raw={} base64-in-json={} v2-section={} (header={})",
                code.len(),
                v1_stored,
                v2_stored,
                v2_stored - code.len()
            );
            assert!(
                v2_stored < v1_stored,
                "raw section must be smaller than base64 text"
            );
            assert_eq!(v2_stored - code.len(), 30, "fixed metadata header size");
        }

        // ---- M26-005-C: unsafe confinement + platform smoke ----

        #[test]
        fn pack_bytes_open_works_on_write_protected_files() {
            // Platform smoke: production packs are read-only artifacts;
            // the mapped path must open files without any write permit
            // (unix mode 0444) and validate through the read-only map.
            let payloads: Vec<(u16, &[u8])> = vec![
                (section::STRINGS, b"strings-payload-bytes"),
                (section::ROUTES, b"router-graph-payload"),
                (section::ROUTE_PLANS, b"plans-payload-xxxx"),
                (section::SCHEMA_MANIFEST, b"schemas-payload-xxxx"),
                (section::POLICIES, b"policies-payload-xxx"),
                (section::CAPABILITIES, b"caps-payload-bytes-xx"),
                (section::CONTRACT_SUMMARY, b"contract-summary12"),
            ];
            let file = reader::build_file_bound(&payloads);
            let dir = std::env::temp_dir().join("velqu-m26-005-c");
            let _ = std::fs::create_dir_all(&dir);
            let path = dir.join("readonly.qpk2");
            std::fs::write(&path, &file).unwrap();
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o444)).unwrap();
            }
            let bytes = reader::PackBytes::open(&path)
                .expect("read-only file opens through the mapped path");
            assert!(reader::validate(&bytes).is_ok());
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o644)).unwrap();
            }
            let _ = std::fs::remove_dir_all(&dir);
        }

        // ---- M26-005-B: bounds before access ----

        #[test]
        fn overflowing_directory_values_reject_without_panic() {
            // Directory offset/len are raw file-controlled u64s. Before
            // M26-005-B, `offset + len` could overflow: panic in debug,
            // wrap past every bound in release. Every shape here must
            // return a typed error (tests run in debug, so an unchecked
            // addition would abort the suite).
            let file = valid_file();
            let base = super::HEADER_SIZE as usize;
            let patch = |f: &mut Vec<u8>, off: u64, len: u64| {
                f[base + 8..base + 16].copy_from_slice(&off.to_le_bytes());
                f[base + 16..base + 24].copy_from_slice(&len.to_le_bytes());
            };
            // aligned offset near u64::MAX + len → checked_add overflows
            let mut f = file.clone();
            patch(&mut f, u64::MAX - 7, 16);
            let err = reader::validate(&f).unwrap_err();
            assert!(err.contains("overflows u64"), "{err}");
            // len = u64::MAX from the first legal body offset (dir end)
            let mut f = file.clone();
            patch(&mut f, 512, u64::MAX);
            let err = reader::validate(&f).unwrap_err();
            assert!(
                err.contains("overflows u64") || err.contains("past end of file"),
                "{err}"
            );
            // offset = u64::MAX, len = 1 (unaligned MAX rejects at the
            // alignment rule — still a typed error, never a panic)
            let mut f = file.clone();
            patch(&mut f, u64::MAX, 1);
            let err = reader::validate(&f).unwrap_err();
            assert!(!err.is_empty());
            // both huge
            let mut f = file.clone();
            patch(&mut f, u64::MAX - 7, u64::MAX - 7);
            let err = reader::validate(&f).unwrap_err();
            assert!(err.contains("overflows u64"), "{err}");
            // the untouched file still validates
            assert!(reader::validate(&file).is_ok());
        }

        #[test]
        fn bounds_checks_precede_any_section_access() {
            // A file whose FIRST entry is fine but whose SECOND entry is
            // past-end must reject WITHOUT hashing or slicing section 0:
            // patching section bodies after the fact cannot change the
            // error — structural bounds run before content work.
            let file = valid_file();
            let base = super::HEADER_SIZE as usize;
            let mut f = file.clone();
            // entry 1 offset far past EOF but aligned (dir_end small)
            f[base + 64 + 8..base + 64 + 16].copy_from_slice(&(1u64 << 40).to_le_bytes());
            let err = reader::validate(&f).unwrap_err();
            assert!(err.contains("past end of file"), "{err}");
            // corrupt section 0's BODY bytes too: the bounds error is
            // identical (bounds ran first; the hash was never reached)
            let mut f2 = f.clone();
            let e0_off = u64::from_le_bytes(f2[base + 8..base + 16].try_into().unwrap()) as usize;
            let e0_len = u64::from_le_bytes(f2[base + 16..base + 24].try_into().unwrap()) as usize;
            f2[e0_off..e0_off + e0_len].fill(0xAA);
            let err2 = reader::validate(&f2).unwrap_err();
            assert_eq!(err, err2, "bounds rejection must precede content hashing");
        }

        // ---- M26-005-A: mmap/read-only pack bytes ----

        #[test]
        fn pack_bytes_mapped_and_owned_validate_identically_zero_copy() {
            let payloads: Vec<(u16, &[u8])> = vec![
                (section::STRINGS, b"strings-payload-bytes"),
                (section::ROUTES, b"router-graph-payload"),
                (section::ROUTE_PLANS, b"plans-payload-xxxx"),
                (section::SCHEMA_MANIFEST, b"schemas-payload-xxxx"),
                (section::POLICIES, b"policies-payload-xxx"),
                (section::CAPABILITIES, b"caps-payload-bytes-xx"),
                (section::CONTRACT_SUMMARY, b"contract-summary12"),
            ];
            let file = reader::build_file_bound(&payloads);
            let dir = std::env::temp_dir().join("velqu-m26-005-a");
            let _ = std::fs::create_dir_all(&dir);
            let path = dir.join("mapped.qpk2");
            std::fs::write(&path, &file).unwrap();

            let mapped = reader::PackBytes::open(&path).unwrap();
            let owned = reader::PackBytes::Owned(file.clone());
            // standalone-binary carrier: static bytes, zero-copy by
            // construction (leaked here to mimic include_bytes!)
            let embedded = reader::PackBytes::Embedded(Box::leak(file.clone().into_boxed_slice()));

            let m = reader::validate(&mapped).expect("mapped file validates");
            let o = reader::validate(&owned).expect("owned file validates");
            let e = reader::validate(&embedded).expect("embedded bytes validate");
            assert_eq!(m.len(), e.len());
            assert_eq!(m.len(), o.len());
            for ((me, mb), (oe, ob)) in m.iter().zip(o.iter()) {
                assert_eq!(me.section_id, oe.section_id);
                assert_eq!(me.content_sha256, oe.content_sha256);
                assert_eq!(mb, ob, "mapped and owned views must carry the same bytes");
            }
            // zero-copy proof: every mapped section body is a view INSIDE
            // the mapping (no owned reconstruction of section bytes)
            let base = mapped.as_ptr() as usize;
            for (e, body) in &m {
                let p = body.as_ptr() as usize;
                assert!(
                    p >= base && p + body.len() <= base + mapped.len(),
                    "section {:#x} body is not a view into the pack bytes",
                    e.section_id
                );
            }
            // embedded carrier is zero-copy too: bodies are views into
            // the static bytes, not copies
            let ebase = embedded.as_ptr() as usize;
            for (e_ent, e_body) in &e {
                let p = e_body.as_ptr() as usize;
                assert!(
                    p >= ebase && p + e_body.len() <= ebase + embedded.len(),
                    "embedded section {:#x} body is not a view into the static bytes",
                    e_ent.section_id
                );
            }
            let _ = std::fs::remove_dir_all(&dir);
        }

        #[test]
        fn pack_bytes_rejects_missing_and_malformed_without_panic() {
            // missing file
            assert!(reader::PackBytes::open(std::path::Path::new(
                "/nonexistent/definitely-missing.qpk2"
            ))
            .is_err());
            // empty file: owned fallback, then header validation rejects
            let dir = std::env::temp_dir().join("velqu-m26-005-a-empty");
            let _ = std::fs::create_dir_all(&dir);
            let empty = dir.join("empty.qpk2");
            std::fs::write(&empty, b"").unwrap();
            let bytes = reader::PackBytes::open(&empty).unwrap();
            assert!(reader::validate(&bytes).is_err());
            // junk bytes: mapped path on unix, must reject (never panic)
            let junk = dir.join("junk.qpk2");
            std::fs::write(&junk, vec![0x41u8; 4096]).unwrap();
            let bytes = reader::PackBytes::open(&junk).unwrap();
            assert!(reader::validate(&bytes).is_err());
            assert!(reader::parse_directory_with_binding(&bytes).is_err());
            let _ = std::fs::remove_dir_all(&dir);
        }

        /// Rebuild a tampered file with one section's content sha256
        /// recomputed so ONLY the aggregate binding can catch the change.
        fn repair_section_hash(
            file: &[u8],
            entries: &[super::reader::DirEntry],
            id: u16,
        ) -> Vec<u8> {
            use sha2::Digest;
            let mut out = file.to_vec();
            let (i, e) = entries
                .iter()
                .enumerate()
                .find(|(_, e)| e.section_id == id)
                .unwrap();
            let dir_off =
                super::reader::EXTENDED_HEADER_SIZE as usize + i * super::DIR_ENTRY_SIZE as usize;
            let body = &file[e.offset as usize..(e.offset + e.len) as usize];
            let hash: [u8; 32] = sha2::Sha256::digest(body).into();
            out[dir_off + 32..dir_off + 64].copy_from_slice(&hash);
            out
        }

        // ---- M26-003-C: offsets and bounds checks ----
        use super::reader;
        use super::section;

        fn valid_file() -> Vec<u8> {
            let payloads: Vec<(u16, &[u8])> = vec![
                (section::STRINGS, b"strings-payload-bytes"),
                (section::ROUTES, b"router-graph-payload"),
                (section::ROUTE_PLANS, b"plans-payload-xxxx"),
                (section::SCHEMA_MANIFEST, b"schemas-payload-xxxx"),
                (section::POLICIES, b"policies-payload-xxx"),
                (section::CAPABILITIES, b"caps-payload-bytes-xx"),
                (section::CONTRACT_SUMMARY, b"contract-summary12"),
            ];
            reader::build_file(&payloads)
        }

        #[test]
        fn header_and_directory_round_trip() {
            let file = valid_file();
            let entries = reader::validate(&file).expect("valid file validates");
            assert_eq!(entries.len(), 7);
            // every offset is 8-aligned and past the directory
            for (e, body) in &entries {
                assert!(e.offset % super::SECTION_ALIGN == 0);
                assert!(e.len > 0);
                assert_eq!(body.len() as u64, e.len);
            }
            // ids are exactly the required catalog
            let ids: std::collections::BTreeSet<u16> =
                entries.iter().map(|(e, _)| e.section_id).collect();
            assert_eq!(ids, section::REQUIRED.iter().copied().collect());
        }

        #[test]
        fn every_directory_rule_violation_rejects() {
            let set16 = |file: &mut Vec<u8>, base: usize, off: usize, v: u16| {
                file[base + off..base + off + 2].copy_from_slice(&v.to_le_bytes());
            };
            let set64 = |file: &mut Vec<u8>, base: usize, off: usize, v: u64| {
                file[base + off..base + off + 8].copy_from_slice(&v.to_le_bytes());
            };
            let dir_base =
                |i: usize| (super::HEADER_SIZE + super::DIR_ENTRY_SIZE * i as u64) as usize;

            // magic mismatch
            let mut f = valid_file();
            f[0] = b'X';
            assert!(reader::parse_header(&f).unwrap_err().contains("magic"));

            // header_size wrong
            let mut f = valid_file();
            f[12..16].copy_from_slice(&32u32.to_le_bytes());
            assert!(reader::parse_header(&f)
                .unwrap_err()
                .contains("header_size"));

            // total_size wrong
            let mut f = valid_file();
            let wrong_total = f.len() as u64 + 1;
            f[16..24].copy_from_slice(&wrong_total.to_le_bytes());
            assert!(reader::parse_header(&f).unwrap_err().contains("total_size"));

            // reserved non-zero
            let mut f = valid_file();
            f[28] = 1;
            assert!(reader::parse_header(&f).unwrap_err().contains("reserved"));

            // offset overlaps the directory
            let mut f = valid_file();
            set64(&mut f, dir_base(0), 8, 64);
            assert!(reader::parse_directory(&f)
                .unwrap_err()
                .contains("header/directory"));

            // misaligned offset
            let mut f = valid_file();
            let base = dir_base(0);
            let off = u64::from_le_bytes(f[base + 8..base + 16].try_into().unwrap());
            set64(&mut f, base, 8, off + 1);
            assert!(reader::parse_directory(&f).unwrap_err().contains("aligned"));

            // zero length
            let mut f = valid_file();
            set64(&mut f, dir_base(0), 16, 0);
            assert!(reader::parse_directory(&f)
                .unwrap_err()
                .contains("zero length"));

            // range past end of file
            let mut f = valid_file();
            let huge_len = f.len() as u64 * 4;
            set64(&mut f, dir_base(0), 16, huge_len);
            assert!(reader::parse_directory(&f)
                .unwrap_err()
                .contains("past end"));

            // duplicate section id
            let mut f = valid_file();
            let id1 = u16::from_le_bytes(f[dir_base(1)..dir_base(1) + 2].try_into().unwrap());
            set16(&mut f, dir_base(0), 0, id1);
            assert!(reader::parse_directory(&f)
                .unwrap_err()
                .contains("duplicate"));

            // content hash mismatch (mutate a body byte, hash stays stale)
            let mut f = valid_file();
            let last = f.len() - 1;
            f[last] ^= 0xFF;
            assert!(reader::validate(&f)
                .unwrap_err()
                .contains("sha256 mismatch"));

            // unknown section id rejects even when required ids all present
            let payloads: Vec<(u16, &[u8])> = vec![
                (section::STRINGS, b"s"),
                (section::ROUTES, b"r"),
                (section::ROUTE_PLANS, b"p"),
                (section::SCHEMA_MANIFEST, b"s"),
                (section::POLICIES, b"p"),
                (section::CAPABILITIES, b"c"),
                (section::CONTRACT_SUMMARY, b"c"),
                (0x7777, b"experiment"),
            ];
            let f = reader::build_file(&payloads);
            assert!(reader::validate(&f)
                .unwrap_err()
                .contains("unknown section id"));

            // missing required section rejects
            let payloads: Vec<(u16, &[u8])> = vec![
                (section::STRINGS, b"s"),
                (section::ROUTES, b"r"),
                (section::ROUTE_PLANS, b"p"),
                (section::SCHEMA_MANIFEST, b"s"),
                (section::POLICIES, b"p"),
                (section::CAPABILITIES, b"c"),
                // CONTRACT_SUMMARY missing
            ];
            let f = reader::build_file(&payloads);
            assert!(reader::validate(&f)
                .unwrap_err()
                .contains("required section"));
        }

        #[test]
        fn header_directory_mutation_never_panics() {
            struct Rng(u64);
            impl Rng {
                fn next(&mut self) -> u64 {
                    let mut x = self.0;
                    x ^= x << 13;
                    x ^= x >> 7;
                    x ^= x << 17;
                    self.0 = x;
                    x
                }
            }
            let file = valid_file();
            // only the header + directory bytes are fuzzed here; section
            // bodies are covered by the M26-003-B section fuzz and the
            // content-hash check above
            let fuzz_len = (super::HEADER_SIZE + super::DIR_ENTRY_SIZE * 7) as usize;
            let mut rng = Rng(0xB0_0505);
            let mut rejected = 0usize;
            for _ in 0..4_000 {
                let mut bytes = file[..fuzz_len].to_vec();
                bytes.extend_from_slice(&file[fuzz_len..]);
                let idx = (rng.next() as usize) % fuzz_len;
                bytes[idx] ^= 1u8 << (rng.next() % 8);
                if reader::validate(&bytes).is_err() {
                    rejected += 1;
                }
            }
            // most header/directory mutations reject; survivors are flips
            // inside offset/len fields that land on legal values and the
            // single legal flag bit — those are caught by the sha256
            // range checks only when they move ranges; no panic, bounded
            // work either way (content hashes cover the bodies)
            assert!(
                rejected > 3_300,
                "mutation must overwhelmingly reject: {rejected}"
            );
        }

        // ---- M26-003-D: execution-integrity binding ----

        fn valid_payloads() -> Vec<(u16, &'static [u8])> {
            vec![
                (section::STRINGS, b"strings-payload-bytes"),
                (section::ROUTES, b"router-graph-payload"),
                (section::ROUTE_PLANS, b"plans-payload-xxxx"),
                (section::SCHEMA_MANIFEST, b"schemas-payload-xxxx"),
                (section::POLICIES, b"policies-payload-xxx"),
                (section::CAPABILITIES, b"caps-payload-bytes-xx"),
                (section::CONTRACT_SUMMARY, b"contract-summary12"),
            ]
        }

        #[test]
        fn bound_file_round_trips_and_binds_every_section() {
            let file = reader::build_file_bound(&valid_payloads());
            let entries = reader::parse_directory_with_binding(&file)
                .expect("bound file validates with its binding");
            assert_eq!(entries.len(), 7);
            // header_size reflects the extension
            let hs = u32::from_le_bytes(file[12..16].try_into().unwrap());
            assert_eq!(hs as u64, reader::EXTENDED_HEADER_SIZE);
        }

        #[test]
        fn any_section_byte_change_breaks_the_binding() {
            // mutate one body byte AND repair that section's content hash
            // so only the BINDING catches it (content checks alone would
            // pass — proving the aggregate pins the whole graph)
            let file = reader::build_file_bound(&valid_payloads());
            let entries = reader::parse_directory(&file).unwrap();
            let last = entries.last().unwrap();
            let mut f = file.clone();
            let body_off = last.offset as usize;
            f[body_off] ^= 0x01;
            use sha2::Digest;
            let new_hash: [u8; 32] =
                sha2::Sha256::digest(&f[last.offset as usize..(last.offset + last.len) as usize])
                    .into();
            // patch the directory entry's content hash to match
            let dir_base = (reader::EXTENDED_HEADER_SIZE
                + super::DIR_ENTRY_SIZE * (entries.len() - 1) as u64)
                as usize;
            f[dir_base + 24..dir_base + 56].copy_from_slice(&new_hash);
            // per-section integrity now passes...
            let patched_entries = reader::parse_directory(&f).unwrap();
            let _ = patched_entries;
            // ...but the execution-integrity binding rejects: the graph
            // changed after the pack was built
            let err = reader::parse_directory_with_binding(&f).unwrap_err();
            assert!(err.contains("execution-integrity hash mismatch"), "{err}");
        }

        #[test]
        fn directory_field_change_breaks_the_binding() {
            // flip a directory offset into another legal value: content
            // hashes still match their (moved) bodies; only the binding
            // catches the directory change
            let payloads = valid_payloads();
            let file = reader::build_file_bound(&payloads);
            let entries = reader::parse_directory(&file).unwrap();
            let mut f = file.clone();
            // swap two sections' offsets (both stay aligned/in-range)
            let a = entries[0].offset;
            let b = entries[1].offset;
            let dir_a = (reader::EXTENDED_HEADER_SIZE) as usize;
            let dir_b = (reader::EXTENDED_HEADER_SIZE + super::DIR_ENTRY_SIZE) as usize;
            f[dir_a + 8..dir_a + 16].copy_from_slice(&b.to_le_bytes());
            f[dir_b + 8..dir_b + 16].copy_from_slice(&a.to_le_bytes());
            // ranges overlap-check may catch it; if it parses, the
            // binding must reject
            match reader::parse_directory_with_binding(&f) {
                Err(e) => {
                    assert!(
                        e.contains("execution-integrity")
                            || e.contains("overlaps")
                            || e.contains("sha256"),
                        "unexpected error: {e}"
                    );
                }
                Ok(_) => panic!("directory change must not validate"),
            }
        }

        // Mode dispatch still rejects 2 until the native adapter lands:
        // no producer emits v2 before M26-003 (ADR-0024 rule 1).
        #[test]
        fn mode_two_still_fails_closed_before_native_adapter() {
            let err = crate::detect_pack_format_mode(FORMAT_VERSION).unwrap_err();
            assert!(err.to_string().contains("fail closed"));
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EngineRef {
    pub name: String,
    pub version: String,
    pub binding: String,
    /// M26-002-A: rquickjs binding version (fingerprint dimension).
    #[serde(default)]
    pub rquickjs: String,
    /// M26-002-A: runtime build fingerprint — sha256 over the runtime
    /// identity tuple (see `runtime_build_hash`). Engine upgrades change
    /// it, so packs require a rebuild (M26-002 guardrail).
    #[serde(default)]
    pub build_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BuiltBy {
    pub compiler: String,
    #[serde(default)]
    pub typescript: String,
    #[serde(default)]
    pub bun: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SegKind {
    Static,
    Param,
    Wildcard,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PathSegment {
    pub kind: SegKind,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Strategy {
    #[serde(rename = "native")]
    Native,
    #[default]
    #[serde(rename = "js")]
    Js,
}

impl Strategy {
    pub fn as_str(&self) -> &'static str {
        match self {
            Strategy::Native => "native",
            Strategy::Js => "js",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceBinding {
    /// schema registry key, or null when the route declares no schema for this source
    #[serde(default)]
    pub schema: Option<String>,
    /// path | query (coercion source)
    #[serde(default)]
    pub coerce: Option<String>,
    #[serde(default)]
    pub content_type: Option<String>,
    #[serde(default = "default_body_limit")]
    pub limit_bytes: u64,
}

fn default_body_limit() -> u64 {
    65_536
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseDecl {
    #[serde(default)]
    pub schema: Option<String>,
    #[serde(default)]
    pub strategy: Strategy,
    #[serde(default)]
    pub problem: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LivenessSpec {
    pub status: u16,
    pub content_type: String,
    pub body: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct FieldNeeds {
    #[serde(default)]
    pub params: bool,
    #[serde(default)]
    pub query: bool,
    #[serde(default)]
    pub headers: bool,
    #[serde(default)]
    pub body: bool,
}

/// M24-005-D: sentinel header-name id marking the EXPLICIT full-Headers
/// escape hatch — allowed only for a route whose headers binding declares
/// no schema (opt-in to copying every header, bounded by the transport
/// header admission limits; that bounded copy is the documented cost).
pub const FULL_HEADERS_ID: u32 = u32::MAX;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RoutePlanDecl {
    pub route_id: u32,
    pub handler_id: u32,
    #[serde(default)]
    pub policy_id: Option<u32>,
    #[serde(default)]
    pub policy_handler_id: Option<u32>,
    #[serde(default)]
    pub params_schema_id: Option<u32>,
    #[serde(default)]
    pub query_schema_id: Option<u32>,
    #[serde(default)]
    pub headers_schema_id: Option<u32>,
    #[serde(default)]
    pub body_schema_id: Option<u32>,
    /// M24-005-A: dense ids into the pack's headerNameTable for exactly the
    /// header names this route (or its policy) declares — security scheme
    /// headers plus headers-binding schema properties.
    #[serde(default)]
    pub header_name_ids: Vec<u32>,
    /// M24-006-A: dense ids into queryNameTable for declared query fields.
    #[serde(default)]
    pub query_name_ids: Vec<u32>,
    /// M24-006-A: dense ids into cookieNameTable for declared cookie fields.
    #[serde(default)]
    pub cookie_name_ids: Vec<u32>,
    #[serde(default)]
    pub default_status: u16,
    #[serde(default)]
    pub allowed_statuses: Vec<u16>,
    #[serde(default)]
    pub field_needs: FieldNeeds,
    #[serde(default)]
    pub response_strategy: Strategy,
    /// M25-007-A: why validation takes the generic path — a value from the
    /// closed FALLBACK_REASONS vocabulary. Present iff the route's
    /// validation strategy is js; native plans carry none. Fallback never
    /// activates silently: verify fails closed on drift.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validation_fallback_reason: Option<String>,
    /// M25-007-A: why the plan's response strategy is the engine path.
    /// Present iff `response_strategy` is js; native plans carry none.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_fallback_reason: Option<String>,
    #[serde(default = "default_deadline")]
    pub deadline_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SchemaDecl {
    pub id: u32,
    pub key: String,
    /// Compatibility marker (M25-001-B): v2 feature tags derived from `ir`.
    /// Verified against `features_of(&ir)` at load; spoofing fails closed.
    #[serde(default)]
    pub features: Vec<String>,
    pub ir: q_schema_runtime::SchemaIr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecurityReq {
    pub scheme: String,
    pub header: String,
    #[serde(default)]
    pub problem_status: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteEntry {
    pub id: String,
    pub module_id: String,
    pub method: String,
    pub path: String,
    /// Pre-compiled segments; if absent the loader rejects the pack (no runtime parsing).
    pub path_segments: Vec<PathSegment>,
    pub handler: String,
    #[serde(default)]
    pub policy: Option<String>,
    #[serde(default)]
    pub params: Option<SourceBinding>,
    #[serde(default)]
    pub query: Option<SourceBinding>,
    #[serde(default)]
    pub body: Option<SourceBinding>,
    #[serde(default)]
    pub headers: Option<SourceBinding>,
    pub responses: BTreeMap<String, ResponseDecl>,
    #[serde(default)]
    pub validation_strategy: Strategy,
    #[serde(default)]
    pub native_liveness: Option<LivenessSpec>,
    #[serde(default)]
    pub security: Vec<SecurityReq>,
    #[serde(default)]
    pub capabilities: Vec<String>,
    #[serde(default = "default_deadline")]
    pub deadline_ms: u64,
    /// M2.3: Precompiled numeric route plan for $O(1)$ dispatch without string parsing
    #[serde(default)]
    pub plan: Option<RoutePlanDecl>,
}

fn default_deadline() -> u64 {
    5_000
}

pub use q_engine::{FunctionDecl, FunctionKind};

/// Canonical HTTP method → terminal slot index (mirrors q-router's METHOD_* map).
pub const METHOD_SLOTS: [&str; 7] = ["GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS", "HEAD"];

#[inline]
pub fn method_index(method: &str) -> Option<usize> {
    METHOD_SLOTS
        .iter()
        .position(|m| *m == method.to_ascii_uppercase())
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SerializedStaticEdge {
    pub segment: String,
    pub target_node: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SerializedTerminal {
    pub method_mask: u16,
    pub route_by_method: [Option<usize>; 7],
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SerializedRouterNode {
    #[serde(default)]
    pub static_edges: Vec<SerializedStaticEdge>,
    #[serde(default)]
    pub param_edge: Option<usize>,
    #[serde(default)]
    pub wildcard_edge: Option<usize>,
    #[serde(default)]
    pub terminal: Option<SerializedTerminal>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SerializedRouter {
    pub nodes: Vec<SerializedRouterNode>,
}

impl SerializedRouter {
    /// Full semantic verification of the compiled automaton against the
    /// declared routes (G0-r1): non-empty node array with root at 0, in-range
    /// and unique edges, methodMask exactly matching populated slots, every
    /// slot holding a route whose method matches the slot, every route
    /// reachable at the terminal its own pathSegments walk to (proving path
    /// shape), no slot double-claimed, and pathSegments agreeing with
    /// route.path. Fail-before-ready: called from QPack::verify.
    pub fn verify_against(&self, routes: &[RouteEntry]) -> Result<(), String> {
        if self.nodes.is_empty() {
            return Err("serialized router has no nodes (root node 0 required)".into());
        }
        let node_count = self.nodes.len();
        for (n_idx, node) in self.nodes.iter().enumerate() {
            let mut seen = std::collections::BTreeSet::new();
            for e in &node.static_edges {
                if e.target_node >= node_count {
                    return Err(format!(
                        "router node {n_idx} static edge target {} out of range ({node_count})",
                        e.target_node
                    ));
                }
                if !seen.insert(e.segment.as_str()) {
                    return Err(format!(
                        "router node {n_idx} has duplicate static edge '{}'",
                        e.segment
                    ));
                }
            }
            for (label, target) in [("param", node.param_edge), ("wildcard", node.wildcard_edge)] {
                if let Some(t) = target {
                    if t >= node_count {
                        return Err(format!(
                            "router node {n_idx} {label} edge target {t} out of range ({node_count})"
                        ));
                    }
                }
            }
        }

        // Walk every route's declared path through the graph; the terminal
        // reached must hold this route in this route's method slot.
        let mut claimed: Vec<Option<(usize, usize)>> = vec![None; routes.len()];
        for (r_idx, route) in routes.iter().enumerate() {
            // pathSegments must agree with route.path
            let declared: Vec<&str> = route.path.split('/').filter(|s| !s.is_empty()).collect();
            if declared.len() != route.path_segments.len() {
                return Err(format!(
                    "route {} pathSegments do not match route.path",
                    route.id
                ));
            }
            for (seg, decl) in route.path_segments.iter().zip(&declared) {
                let ok = match seg.kind {
                    SegKind::Static => seg.value == *decl,
                    SegKind::Param => {
                        decl.len() > 1 && decl.starts_with(':') && decl[1..] == seg.value
                    }
                    SegKind::Wildcard => *decl == "*",
                };
                if !ok {
                    return Err(format!(
                        "route {} path segment {:?} disagrees with path text {:?}",
                        route.id, seg.value, decl
                    ));
                }
            }
            let Some(m_idx) = method_index(&route.method) else {
                return Err(format!(
                    "route {} has unsupported method {}",
                    route.id, route.method
                ));
            };
            let mut curr = 0usize;
            for seg in &route.path_segments {
                match seg.kind {
                    SegKind::Static => {
                        let Some(edge) = self.nodes[curr]
                            .static_edges
                            .iter()
                            .find(|e| e.segment == seg.value)
                        else {
                            return Err(format!(
                                "route {} path not represented in router (missing static '{}')",
                                route.id, seg.value
                            ));
                        };
                        curr = edge.target_node;
                    }
                    SegKind::Param => {
                        let Some(t) = self.nodes[curr].param_edge else {
                            return Err(format!(
                                "route {} path not represented in router (missing param edge)",
                                route.id
                            ));
                        };
                        curr = t;
                    }
                    SegKind::Wildcard => {
                        let Some(t) = self.nodes[curr].wildcard_edge else {
                            return Err(format!(
                                "route {} path not represented in router (missing wildcard edge)",
                                route.id
                            ));
                        };
                        curr = t;
                    }
                }
            }
            let Some(terminal) = &self.nodes[curr].terminal else {
                return Err(format!(
                    "route {} path ends at a non-terminal router node",
                    route.id
                ));
            };
            match terminal.route_by_method[m_idx] {
                Some(target) if target == r_idx => {}
                Some(other) => {
                    return Err(format!(
                        "route {} terminal slot points at route {} (misrouted or shadowed)",
                        route.id, other
                    ))
                }
                None => {
                    return Err(format!(
                        "route {} not reachable in router (its terminal method slot is empty)",
                        route.id
                    ))
                }
            }
            claimed[r_idx] = Some((curr, m_idx));
        }

        // Terminals: mask == populated slots exactly; slot contents must be
        // claimed by the route whose walk lands here (kills stray/duplicate
        // terminals and in-range-but-wrong mappings).
        let mut slot_claims = std::collections::BTreeSet::new();
        for c in claimed.iter().flatten() {
            if !slot_claims.insert(*c) {
                return Err("two routes claim the same router terminal slot (collision)".into());
            }
        }
        for (n_idx, node) in self.nodes.iter().enumerate() {
            let Some(t) = &node.terminal else { continue };
            let mut expected_mask = 0u16;
            for (m_idx, slot) in t.route_by_method.iter().enumerate() {
                let Some(r_idx) = *slot else { continue };
                if r_idx >= routes.len() {
                    return Err(format!(
                        "router node {n_idx} terminal slot {m_idx} route index {r_idx} out of range ({})",
                        routes.len()
                    ));
                }
                let route = &routes[r_idx];
                let Some(rm) = method_index(&route.method) else {
                    return Err(format!("route {} has unsupported method", route.id));
                };
                if rm != m_idx {
                    return Err(format!(
                        "router node {n_idx} slot {m_idx} holds route {} whose method maps to slot {rm}",
                        route.id
                    ));
                }
                if claimed[r_idx] != Some((n_idx, m_idx)) {
                    return Err(format!(
                        "route {} attached at router node {n_idx} slot {m_idx} but its own path routes elsewhere",
                        route.id
                    ));
                }
                expected_mask |= 1 << m_idx;
            }
            if t.method_mask != expected_mask {
                return Err(format!(
                    "router node {n_idx} methodMask {:b} != populated slots {:b}",
                    t.method_mask, expected_mask
                ));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PolicyEntry {
    pub id: String,
    pub handler: String,
    pub declared_statuses: Vec<u16>,
    #[serde(default)]
    pub provides: Option<String>,
}

/// QuickJS module bytecode, produced at BUILD time by the exact engine build
/// (ADR-0014/ADR-0017). The runtime loads it only on an exact version match;
/// any mismatch or absence falls back to evaluating `bundle` source.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BytecodeTarget {
    pub arch: String,
    pub os: String,
    pub pointer_width: u8,
    pub endianness: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BundleBytecode {
    pub quickjs: String,
    pub binding: String,
    /// "little" | "big" — bytecode is not endian-portable
    pub endianness: String,
    /// Compilation target fingerprint (arch, OS, pointer width, endianness)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<BytecodeTarget>,
    /// base64-encoded module bytecode
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Integrity {
    pub algorithm: String,
    pub bundle_sha256: String,
    pub routes_sha256: String,
    /// required when bundleBytecode is present
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bytecode_sha256: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QPack {
    pub format_version: u32,
    pub kind: String,
    pub runtime_abi: u32,
    pub engine: EngineRef,
    pub schema_ir_version: u32,
    pub contract_version: u32,
    #[serde(default)]
    pub contract_hash: String,
    #[serde(default)]
    pub built_by: BuiltBy,
    pub app_id: String,
    pub modules: Vec<String>,
    pub entry: String,
    /// "script" (register protocol, M1 form) or "module" (named-export form).
    /// Absent = "script" (backward compatible).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bundle_form: Option<String>,
    pub bundle: String,
    #[serde(default)]
    pub source_map: Option<String>,
    /// Optional build-time bytecode (module form only); see BundleBytecode.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bundle_bytecode: Option<BundleBytecode>,
    /// "numeric" (current) or "legacy". Numeric packs MUST declare it
    /// explicitly (G0-r1): no more inferring mode from `functions`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_mode: Option<String>,
    /// M26-004-D: "embedded" when the compiled module bytecode contains
    /// the prelude and handler manifest (startup then performs zero
    /// prelude source evaluation). Absent = host-evaluated prelude.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bundle_prelude: Option<String>,
    /// Decoded bytecode bytes cached by `verify_and_cache_bytecode` so
    /// production startup base64-decodes exactly once: the same buffer
    /// feeds the integrity hash and the engine handoff (M26-004-B).
    /// Not serialized; absent when verification fails or bytecode is
    /// skipped/absent.
    #[serde(skip)]
    pub decoded_bytecode: Option<Vec<u8>>,
    pub routes: Vec<RouteEntry>,
    /// schema IR registry: key -> IR node (q-schema-runtime types)
    #[serde(default)]
    pub schemas: BTreeMap<String, q_schema_runtime::SchemaIr>,
    #[serde(default)]
    pub policies: BTreeMap<String, PolicyEntry>,
    #[serde(default)]
    pub capabilities: Vec<String>,
    /// M26-002-A: sha256 over the sorted capability names. Optional for
    /// legacy-v1 packs (absent = unchecked); present-but-wrong rejects
    /// with the dimension named.
    #[serde(default)]
    pub capability_hash: String,
    #[serde(default)]
    pub functions: Vec<FunctionDecl>,
    #[serde(default)]
    pub schema_manifest: Vec<SchemaDecl>,
    /// M24-005-A: canonical (sorted, deduped) header-name table referenced by
    /// RoutePlanDecl.header_name_ids; verify() proves it is exactly the set
    /// derivable from the (hashed) routes.
    #[serde(default)]
    pub header_name_table: Vec<String>,
    /// M24-006-A: canonical query field-name table.
    #[serde(default)]
    pub query_name_table: Vec<String>,
    /// M24-006-A: canonical cookie field-name table.
    #[serde(default)]
    pub cookie_name_table: Vec<String>,
    /// Dense numeric policy manifest (G0-r1): PolicyId → policy key + HandlerId.
    #[serde(default)]
    pub policy_manifest: Vec<PolicyDecl>,
    #[serde(default)]
    pub router: Option<SerializedRouter>,
    /// Legacy v1 only. Numeric packs omit the field entirely; carrying a
    /// non-empty table in numeric mode is rejected.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub handler_table: BTreeMap<String, String>,
    pub integrity: Integrity,
}

/// Dense numeric policy identity: PolicyId ↔ policy key ↔ pre-resolved HandlerId.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PolicyDecl {
    pub id: u32,
    pub key: String,
    pub handler_id: u32,
}

impl QPack {
    /// Load + fully verify a pack. Fails before any serving can happen.
    pub fn load_and_verify(path: &std::path::Path) -> Result<QPack, PackError> {
        Self::load_and_verify_with(path, BytecodePolicy::Enforce)
    }

    /// M26-002-C: the explicit source-rebuild path. `Skip` ignores the
    /// embedded bytecode entirely (no bytecode fingerprint checks, no
    /// bytecode load — the verified SOURCE bundle evaluates instead);
    /// every other fingerprint dimension still enforces. This is the
    /// sanctioned recovery path for cross-target bytecode: rebuild the
    /// pack, or start with `--no-bytecode` to run from source.
    pub fn load_and_verify_with(
        path: &std::path::Path,
        policy: BytecodePolicy,
    ) -> Result<QPack, PackError> {
        let bytes = std::fs::read(path)?;
        let mut pack: QPack =
            serde_json::from_slice(&bytes).map_err(|e| PackError::Malformed(e.to_string()))?;
        match policy {
            BytecodePolicy::Enforce => pack.verify_and_cache_bytecode()?,
            BytecodePolicy::Skip => pack.verify_without_bytecode()?,
        }
        Ok(pack)
    }

    /// M26-002-C: verify with the embedded bytecode ignored (source path).
    pub fn verify_without_bytecode(&self) -> Result<(), PackError> {
        if self.bundle_bytecode.is_some() || self.bundle_prelude.is_some() {
            let mut source_only = self.clone();
            source_only.bundle_bytecode = None;
            // integrity.bytecodeSha256 without bytecode normally rejects;
            // on the source path it is simply unused
            source_only.integrity.bytecode_sha256 = None;
            // M26-004-D: the embedded marker describes the BYTECODE module
            // layout; the source path always evaluates the host prelude,
            // so bytecode layout markers no longer apply.
            source_only.bundle_prelude = None;
            return source_only.verify();
        }
        self.verify()
    }

    pub fn verify(&self) -> Result<(), PackError> {
        self.verify_inner(None)
    }

    /// Verify and cache the decoded bytecode so production startup
    /// base64-decodes exactly once (M26-004-B): the integrity hash and
    /// the engine handoff share one decode. The cache is populated only
    /// on success; a rejected pack leaves it empty.
    pub fn verify_and_cache_bytecode(&mut self) -> Result<(), PackError> {
        let mut slot = self.decoded_bytecode.take();
        let result = self.verify_inner(Some(&mut slot));
        if result.is_ok() {
            self.decoded_bytecode = slot;
        }
        result
    }

    fn verify_inner(&self, cache: Option<&mut Option<Vec<u8>>>) -> Result<(), PackError> {
        let reject = |msg: String| Err(PackError::Rejected(msg));
        if self.kind != "velqu.qpack" {
            return reject(format!("unexpected kind {:?}", self.kind));
        }
        // ADR-0024: mode dispatch happens before any other check; the
        // adapter chosen here owns every later interpretation of the pack.
        detect_pack_format_mode(self.format_version)?;
        if self.runtime_abi != RUNTIME_ABI {
            return reject(format!(
                "runtime ABI {} != pack {}",
                RUNTIME_ABI, self.runtime_abi
            ));
        }
        if self.schema_ir_version != SCHEMA_IR_VERSION {
            return reject(format!(
                "schema IR version {} not supported",
                self.schema_ir_version
            ));
        }
        if self.contract_version != CONTRACT_VERSION {
            return reject(format!(
                "contract version {} not supported",
                self.contract_version
            ));
        }
        if self.engine.name != ENGINE_NAME
            || self.engine.version != ENGINE_VERSION
            || self.engine.binding != ENGINE_BINDING
        {
            return reject(format!(
                "engine mismatch: pack wants {} {} via {}, runtime embeds {} {} via {} (SEC-001 exact match)",
                self.engine.name, self.engine.version, self.engine.binding,
                ENGINE_NAME, ENGINE_VERSION, ENGINE_BINDING
            ));
        }
        // M26-002-A: fingerprint dimensions each reject with the
        // incompatible dimension named (guardrail: the error identifies
        // what is incompatible)
        if self.engine.rquickjs != RQUICKJS_VERSION {
            return reject(format!(
                "rquickjs version mismatch (incompatible dimension: binding): pack wants {}, runtime embeds {}",
                self.engine.rquickjs, RQUICKJS_VERSION
            ));
        }
        if self.engine.build_hash != runtime_build_hash() {
            return reject(format!(
                "runtime build hash mismatch (incompatible dimension: runtime build): pack wants {}, runtime is {} — engine upgrades require a pack rebuild",
                self.engine.build_hash,
                runtime_build_hash()
            ));
        }
        // integrity
        let bundle_hash = hex(&Sha256::digest(self.bundle.as_bytes()));
        if bundle_hash != self.integrity.bundle_sha256 {
            return reject(
                "integrity failure: bundle sha256 mismatch (tampered or corrupt pack)".into(),
            );
        }
        if let Some(bc) = &self.bundle_bytecode {
            if bc.quickjs != ENGINE_VERSION || bc.binding != ENGINE_BINDING {
                return reject(format!(
                    "bytecode engine mismatch: pack wants quickjs {} via {}, runtime embeds {} via {} (SEC-001)",
                    bc.quickjs, bc.binding, ENGINE_VERSION, ENGINE_BINDING
                ));
            }
            let expect_endianness = if cfg!(target_endian = "big") {
                "big"
            } else {
                "little"
            };
            if bc.endianness != expect_endianness {
                return reject(format!(
                    "bytecode endianness mismatch: {} vs host {}",
                    bc.endianness, expect_endianness
                ));
            }
            let data = base64_decode(&bc.data)
                .ok_or_else(|| PackError::Rejected("bytecode is not valid base64".into()))?;
            let bc_hash = hex(&Sha256::digest(&data));
            let want = self.integrity.bytecode_sha256.as_ref().ok_or_else(|| {
                PackError::Rejected(
                    "bundleBytecode present without integrity.bytecodeSha256".into(),
                )
            })?;
            if bc_hash != *want {
                return reject(
                    "integrity failure: bytecode sha256 mismatch (tampered or corrupt)".into(),
                );
            }
            if let Some(slot) = cache {
                *slot = Some(data);
            }
        } else if self.integrity.bytecode_sha256.is_some() {
            return reject(
                "integrity declares bytecodeSha256 but no bundleBytecode present".into(),
            );
        }
        match self.bundle_prelude.as_deref() {
            None => {}
            Some("embedded") => {
                if self.bundle_bytecode.is_none() {
                    return reject(
                        "bundlePrelude \"embedded\" requires bundleBytecode (source packs evaluate the host prelude)".into(),
                    );
                }
            }
            Some(other) => {
                return reject(format!(
                    "unknown bundlePrelude value {other:?} (only \"embedded\" is defined)"
                ));
            }
        }
        let routes_hash = self.routes_canonical_sha256();
        if routes_hash != self.integrity.routes_sha256 {
            return reject("integrity failure: execution graph sha256 mismatch".into());
        }

        // M24-005-A: header-name ids compiled into RoutePlans must be exactly
        // the names each route declares (security scheme headers + headers
        // binding schema properties); the pack table must be the canonical
        // sorted/deduped union, so a tampered table or ids cannot pass.
        {
            let mut expected_table: Vec<String> = Vec::new();
            for route in &self.routes {
                let mut names: Vec<String> = route
                    .security
                    .iter()
                    .map(|sec| sec.header.clone())
                    .collect();
                let mut escape_hatch = false;
                if let Some(binding) = &route.headers {
                    match &binding.schema {
                        // M24-005-D: a schema-less headers binding is the
                        // EXPLICIT full-Headers escape hatch
                        None => escape_hatch = true,
                        Some(key) => {
                            if let Some(q_schema_runtime::SchemaIr::Object { properties, .. }) =
                                self.schemas.get(key)
                            {
                                names.extend(properties.keys().cloned());
                            }
                        }
                    }
                }
                if escape_hatch {
                    names.clear();
                }
                names.sort();
                names.dedup();
                let ids: Vec<u32> = if escape_hatch {
                    vec![FULL_HEADERS_ID]
                } else {
                    names
                        .iter()
                        .map(|n| match expected_table.binary_search(n) {
                            Ok(pos) => pos as u32,
                            Err(pos) => {
                                expected_table.insert(pos, n.clone());
                                pos as u32
                            }
                        })
                        .collect()
                };
                if let Some(plan) = &route.plan {
                    if plan.header_name_ids != ids {
                        return reject(format!(
                            "route {}: headerNameIds {:?} do not match declared header names {:?}",
                            route.id, plan.header_name_ids, ids
                        ));
                    }
                    if plan.field_needs.headers && ids.is_empty() {
                        return reject(format!(
                            "route {}: fieldNeeds.headers is true but no header names are declared",
                            route.id
                        ));
                    }
                }
            }
            if self.header_name_table != expected_table {
                return reject(format!(
                    "headerNameTable mismatch: pack declares {:?}, routes derive {:?}",
                    self.header_name_table, expected_table
                ));
            }
        }
        // M24-006-A: query/cookie field IDs are canonical dense tables. Query
        // names derive from object-schema properties; cookie bindings are not
        // yet authorable, so any cookie IDs/table entries fail closed.
        {
            let mut expected_query_table = Vec::<String>::new();
            for route in &self.routes {
                let names = route
                    .query
                    .as_ref()
                    .and_then(|binding| binding.schema.as_ref())
                    .and_then(|key| self.schemas.get(key))
                    .and_then(|ir| match ir {
                        q_schema_runtime::SchemaIr::Object { properties, .. } => {
                            Some(properties.keys().cloned().collect::<Vec<_>>())
                        }
                        _ => None,
                    })
                    .unwrap_or_default();
                let mut names = names;
                names.sort();
                names.dedup();
                let ids: Vec<u32> = names
                    .iter()
                    .map(|name| match expected_query_table.binary_search(name) {
                        Ok(pos) => pos as u32,
                        Err(pos) => {
                            expected_query_table.insert(pos, name.clone());
                            pos as u32
                        }
                    })
                    .collect();
                if let Some(plan) = &route.plan {
                    if plan.query_name_ids != ids {
                        return reject(format!(
                            "route {}: queryNameIds {:?} do not match declared query names {:?}",
                            route.id, plan.query_name_ids, ids
                        ));
                    }
                    if !plan.cookie_name_ids.is_empty() {
                        return reject(format!(
                            "route {}: cookieNameIds are unsupported before cookie bindings",
                            route.id
                        ));
                    }
                }
            }
            if self.query_name_table != expected_query_table {
                return reject(format!(
                    "queryNameTable mismatch: pack declares {:?}, routes derive {:?}",
                    self.query_name_table, expected_query_table
                ));
            }
            if !self.cookie_name_table.is_empty() {
                return reject("cookieNameTable is unsupported before cookie bindings".into());
            }
        }
        if !self.contract_hash.is_empty() {
            let expected_contract_hash = &self.public_contract_sha256()[..32];
            if self.contract_hash != expected_contract_hash {
                return reject(format!(
                    "contract hash mismatch: pack declares {}, calculated {}",
                    self.contract_hash, expected_contract_hash
                ));
            }
        }
        if self.integrity.algorithm != "sha256" {
            return reject(format!(
                "unsupported integrity algorithm {}",
                self.integrity.algorithm
            ));
        }
        // function manifest validation (M2.3 numeric mode) + current-pack artifact rules
        if !self.functions.is_empty() {
            let mut seen_keys = std::collections::BTreeSet::new();
            for (idx, fn_decl) in self.functions.iter().enumerate() {
                if fn_decl.id != idx as u32 {
                    return reject(format!(
                        "function manifest id {} does not match index {idx} (must be dense 0..N)",
                        fn_decl.id
                    ));
                }
                if !seen_keys.insert(&fn_decl.key) {
                    return reject(format!("duplicate function key {}", fn_decl.key));
                }
            }
            // G0-r1: numeric mode must be EXPLICIT — no inference from field presence
            if self.execution_mode.as_deref() != Some("numeric") {
                return reject(
                    "numeric pack must declare executionMode: \"numeric\" explicitly".into(),
                );
            }
            // Numeric current packs: no legacy handler table, no implicit map fallback
            if !self.handler_table.is_empty() {
                return reject(
                    "numeric pack must not carry handlerTable (legacy dispatch metadata)".into(),
                );
            }
            // Numeric current packs require the compiler-emitted router automaton
            if self.router.is_none() {
                return reject(
                    "numeric pack requires the compiled router automaton (pack.router)".into(),
                );
            }
            // G0-r1: dense policy manifest — every declared policy present exactly
            // once, handler_id resolving to a PolicyHandler function with the
            // matching key.
            {
                let mut seen = std::collections::BTreeSet::new();
                for (idx, pd) in self.policy_manifest.iter().enumerate() {
                    if pd.id != idx as u32 {
                        return reject(format!(
                            "policy manifest id {} does not match index {idx} (must be dense 0..N)",
                            pd.id
                        ));
                    }
                    if !seen.insert(pd.key.clone()) {
                        return reject(format!("duplicate policy manifest key {}", pd.key));
                    }
                    let Some(entry) = self.policies.get(&pd.key) else {
                        return reject(format!(
                            "policy manifest key {} not found in policies table",
                            pd.key
                        ));
                    };
                    let Some(f) = self.functions.get(pd.handler_id as usize) else {
                        return reject(format!(
                            "policy manifest entry {} handler_id {} out of range",
                            pd.key, pd.handler_id
                        ));
                    };
                    if f.kind != FunctionKind::PolicyHandler || f.key != entry.handler {
                        return reject(format!(
                            "policy manifest entry {} handler_id {} does not resolve to its PolicyHandler function",
                            pd.key, pd.handler_id
                        ));
                    }
                }
                let policy_keys: std::collections::BTreeSet<&String> =
                    self.policies.keys().collect();
                let manifest_keys: std::collections::BTreeSet<&String> =
                    self.policy_manifest.iter().map(|p| &p.key).collect();
                if policy_keys != manifest_keys {
                    return reject(format!(
                        "numeric policy manifest must cover every policy ({:?} vs {:?})",
                        manifest_keys, policy_keys
                    ));
                }
            }
            // Whenever schemas exist, the numeric schema manifest must cover them completely
            if !self.schemas.is_empty() {
                let manifest_keys: std::collections::BTreeSet<&String> =
                    self.schema_manifest.iter().map(|s| &s.key).collect();
                let schema_keys: std::collections::BTreeSet<&String> =
                    self.schemas.keys().collect();
                if manifest_keys != schema_keys {
                    return reject(format!(
                        "numeric schema manifest must cover every schema (manifest {:?} vs schemas {:?})",
                        manifest_keys, schema_keys
                    ));
                }
            }
            // G0-r1 (C): numeric mode requires a declared, verified public contract hash
            if self.contract_hash.is_empty() {
                return reject(
                    "numeric pack must declare contractHash (public contract verification)".into(),
                );
            }
        } else if self.execution_mode.as_deref() == Some("numeric") {
            return reject(
                "executionMode \"numeric\" declared but function manifest is empty".into(),
            );
        }

        // schema manifest validation (M2.3-r2/r3 numeric mode)
        if !self.schema_manifest.is_empty() {
            let mut seen_keys = std::collections::BTreeSet::new();
            for (idx, schema_decl) in self.schema_manifest.iter().enumerate() {
                if schema_decl.id != idx as u32 {
                    return reject(format!(
                        "schema manifest id {} does not match index {idx} (must be dense 0..N)",
                        schema_decl.id
                    ));
                }
                if !seen_keys.insert(&schema_decl.key) {
                    return reject(format!("duplicate schema key {}", schema_decl.key));
                }
                let Some(actual_ir) = self.schemas.get(&schema_decl.key) else {
                    return reject(format!(
                        "schema manifest key {} not found in schemas table",
                        schema_decl.key
                    ));
                };
                if *actual_ir != schema_decl.ir {
                    return reject(format!(
                        "schema manifest entry {} ({}) IR does not match declared schema IR",
                        schema_decl.id, schema_decl.key
                    ));
                }
                // compatibility markers (M25-001-B): declared features must
                // equal the derived ones — no hiding fallback/schema usage
                let derived = q_schema_runtime::features_of(&schema_decl.ir);
                if schema_decl.features != derived {
                    return reject(format!(
                        "schema manifest entry {} ({}) features {:?} do not match derived features {:?}",
                        schema_decl.id, schema_decl.key, schema_decl.features, derived
                    ));
                }
                if let Some(r) = schema_decl
                    .ir
                    .fallback_reasons()
                    .into_iter()
                    .find(|r| !q_schema_runtime::is_valid_fallback_reason(r))
                {
                    return reject(format!(
                        "schema manifest entry {} ({}) has unknown fallback reason {}",
                        schema_decl.id, schema_decl.key, r
                    ));
                }
            }
        }

        // Serialized-router semantic verification (G0-r1): full graph↔routes
        // equivalence — replaces the previous bounds-only check
        if let Some(ref r) = self.router {
            if let Err(e) = r.verify_against(&self.routes) {
                return reject(format!("serialized router rejected: {e}"));
            }
        }

        // handler table sanity
        if self.handler_table.is_empty() && self.functions.is_empty() {
            return reject("empty handler table and empty function manifest".into());
        }
        let mut seen = std::collections::BTreeSet::new();
        for (route_idx, route) in self.routes.iter().enumerate() {
            if !seen.insert(route.id.clone()) {
                return reject(format!("duplicate route id {}", route.id));
            }
            if !self.handler_table.is_empty() && !self.handler_table.contains_key(&route.handler) {
                return reject(format!(
                    "route {} references unknown handler table key {}",
                    route.id, route.handler
                ));
            }
            if let Some(p) = &route.policy {
                let Some(policy_entry) = self.policies.get(p) else {
                    return reject(format!(
                        "route {} references unknown policy {}",
                        route.id, p
                    ));
                };
                if !self.handler_table.is_empty()
                    && !self.handler_table.contains_key(&policy_entry.handler)
                {
                    return reject(format!(
                        "policy {} references unknown handler {}",
                        p, policy_entry.handler
                    ));
                }
            }
            for binding in [&route.params, &route.query, &route.body, &route.headers]
                .into_iter()
                .flatten()
            {
                if let Some(key) = &binding.schema {
                    if !self.schemas.contains_key(key) {
                        return reject(format!(
                            "route {} references unknown schema {}",
                            route.id, key
                        ));
                    }
                }
            }
            if route.responses.is_empty() {
                return reject(format!("route {} declares no responses", route.id));
            }
            let mut declared_statuses = std::collections::BTreeSet::new();
            for status_str in route.responses.keys() {
                let s_num: u16 = status_str.parse().map_err(|_| {
                    PackError::Rejected(format!(
                        "route {} has invalid response status code {status_str}",
                        route.id
                    ))
                })?;
                if !(100..=599).contains(&s_num) {
                    return reject(format!(
                        "route {} declared response status code {s_num} outside valid range 100..=599",
                        route.id
                    ));
                }
                declared_statuses.insert(s_num);
            }

            // Exact RoutePlan Equivalence (M2.3-r2)
            if let Some(plan) = &route.plan {
                if plan.route_id != route_idx as u32 {
                    return reject(format!(
                        "route {} plan.route_id {} does not match route index {route_idx}",
                        route.id, plan.route_id
                    ));
                }
                if plan.deadline_ms != route.deadline_ms {
                    return reject(format!(
                        "route {} plan.deadline_ms {} does not match route.deadline_ms {}",
                        route.id, plan.deadline_ms, route.deadline_ms
                    ));
                }

                // Check allowed_statuses uniqueness and validity
                let mut planned_statuses = std::collections::BTreeSet::new();
                for &s in &plan.allowed_statuses {
                    if !(100..=599).contains(&s) {
                        return reject(format!(
                            "route {} plan.allowed_statuses contains invalid HTTP status code {s}",
                            route.id
                        ));
                    }
                    if !planned_statuses.insert(s) {
                        return reject(format!(
                            "route {} plan.allowed_statuses contains duplicate status code {s}",
                            route.id
                        ));
                    }
                }

                // Exact bidirectional equivalence: declared == planned
                if declared_statuses != planned_statuses {
                    return reject(format!(
                        "route {} plan.allowed_statuses {:?} does not match declared response statuses {:?}",
                        route.id, plan.allowed_statuses, declared_statuses
                    ));
                }

                // Default status must be in declared responses
                if !declared_statuses.contains(&plan.default_status) {
                    return reject(format!(
                        "route {} plan.default_status {} is not in declared response statuses {:?}",
                        route.id, plan.default_status, declared_statuses
                    ));
                }

                // Expected response strategy
                let expected_strategy =
                    if let Some(decl) = route.responses.get(&plan.default_status.to_string()) {
                        decl.strategy
                    } else {
                        route
                            .responses
                            .values()
                            .next()
                            .map(|d| d.strategy)
                            .unwrap_or(Strategy::Js)
                    };
                if plan.response_strategy != expected_strategy {
                    return reject(format!(
                        "route {} plan.response_strategy {:?} != declared response strategy {:?}",
                        route.id, plan.response_strategy, expected_strategy
                    ));
                }

                // M25-007-A: fallback never activates silently — every js
                // plan strategy must carry a reason from the closed
                // FALLBACK_REASONS vocabulary, and a native plan must not
                // carry one.
                let check_reason = |axis: &str,
                                    is_js: bool,
                                    reason: &Option<String>|
                 -> Result<(), String> {
                    match (is_js, reason) {
                        (true, Some(r)) if q_schema_runtime::is_valid_fallback_reason(r) => {
                            Ok(())
                        }
                        (true, Some(r)) => Err(format!(
                            "route {} plan.{} {} is not in the closed FALLBACK_REASONS vocabulary",
                            route.id, axis, r
                        )),
                        (true, None) => Err(format!(
                            "route {} plan.{} is the engine path without a fallback reason (silent fallback)",
                            route.id, axis
                        )),
                        (false, Some(_)) => Err(format!(
                            "route {} plan.{} is native but carries a fallback reason",
                            route.id, axis
                        )),
                        (false, None) => Ok(()),
                    }
                };
                if let Err(msg) = check_reason(
                    "validation",
                    route.validation_strategy == Strategy::Js,
                    &plan.validation_fallback_reason,
                ) {
                    return reject(msg);
                }
                if let Err(msg) = check_reason(
                    "response",
                    plan.response_strategy == Strategy::Js,
                    &plan.response_fallback_reason,
                ) {
                    return reject(msg);
                }

                // Exact FieldNeeds equivalence
                let expected_field_needs = FieldNeeds {
                    params: route.params.is_some(),
                    query: route.query.is_some(),
                    body: route.body.is_some(),
                    headers: route.headers.is_some() || !route.security.is_empty(),
                };
                if plan.field_needs != expected_field_needs {
                    return reject(format!(
                        "route {} plan.field_needs {:?} != declared needs {:?}",
                        route.id, plan.field_needs, expected_field_needs
                    ));
                }

                // Schema ID validation (if schema manifest is present)
                if !self.schema_manifest.is_empty() {
                    let check_schema = |opt_binding: Option<&SourceBinding>,
                                        opt_id: Option<u32>,
                                        source_name: &str|
                     -> Result<(), PackError> {
                        match (opt_binding.and_then(|b| b.schema.as_ref()), opt_id) {
                            (Some(key), Some(id)) => {
                                if (id as usize) >= self.schema_manifest.len() {
                                    return reject(format!(
                                        "route {} plan.{}SchemaId {} out of range",
                                        route.id, source_name, id
                                    ));
                                }
                                if self.schema_manifest[id as usize].key != *key {
                                    return reject(format!(
                                        "route {} plan.{}SchemaId {} ({}) != declared schema key {}",
                                        route.id,
                                        source_name,
                                        id,
                                        self.schema_manifest[id as usize].key,
                                        key
                                    ));
                                }
                            }
                            (None, Some(id)) => {
                                return reject(format!(
                                    "route {} has no {} schema but declares plan.{}SchemaId {}",
                                    route.id, source_name, source_name, id
                                ));
                            }
                            (Some(key), None) => {
                                return reject(format!(
                                    "route {} declares {} schema {} but plan.{}SchemaId is None",
                                    route.id, source_name, key, source_name
                                ));
                            }
                            (None, None) => {}
                        }
                        Ok(())
                    };
                    check_schema(route.params.as_ref(), plan.params_schema_id, "params")?;
                    check_schema(route.query.as_ref(), plan.query_schema_id, "query")?;
                    check_schema(route.body.as_ref(), plan.body_schema_id, "body")?;
                    check_schema(route.headers.as_ref(), plan.headers_schema_id, "headers")?;
                }

                if !self.functions.is_empty() {
                    if (plan.handler_id as usize) >= self.functions.len() {
                        return reject(format!(
                            "route {} plan.handler_id {} out of range (functions count: {})",
                            route.id,
                            plan.handler_id,
                            self.functions.len()
                        ));
                    }
                    let handler_decl = &self.functions[plan.handler_id as usize];
                    if handler_decl.key != route.handler {
                        return reject(format!(
                            "route {} plan.handler_id {} ({}) does not match route.handler ({})",
                            route.id, plan.handler_id, handler_decl.key, route.handler
                        ));
                    }
                    if handler_decl.kind != FunctionKind::RouteHandler {
                        return reject(format!(
                            "route {} plan.handler_id {} points to non-route function kind {:?}",
                            route.id, plan.handler_id, handler_decl.kind
                        ));
                    }
                    if let Some(p) = &route.policy {
                        let policy_entry = self.policies.get(p).unwrap();
                        // G0-r1: policyId must resolve through the dense policy
                        // manifest to this exact policy, and agree with the
                        // pre-resolved policy handler ID.
                        if !self.policy_manifest.is_empty() {
                            let Some(pd) = plan
                                .policy_id
                                .and_then(|pid| self.policy_manifest.get(pid as usize))
                            else {
                                return reject(format!(
                                    "route {} declares policy {} but plan.policyId {:?} does not resolve in the policy manifest",
                                    route.id, p, plan.policy_id
                                ));
                            };
                            if pd.key != *p {
                                return reject(format!(
                                    "route {} plan.policyId {} ({}) does not match route.policy ({})",
                                    route.id, plan.policy_id.unwrap(), pd.key, p
                                ));
                            }
                            if Some(pd.handler_id) != plan.policy_handler_id {
                                return reject(format!(
                                    "route {} plan.policyId {} handler_id {} disagrees with plan.policy_handler_id {:?}",
                                    route.id, plan.policy_id.unwrap(), pd.handler_id, plan.policy_handler_id
                                ));
                            }
                        }
                        let Some(p_fn_id) = plan.policy_handler_id else {
                            return reject(format!(
                                "route {} declares policy {} but plan.policy_handler_id is None",
                                route.id, p
                            ));
                        };
                        if (p_fn_id as usize) >= self.functions.len() {
                            return reject(format!(
                                "route {} plan.policy_handler_id {} out of range (functions count: {})",
                                route.id, p_fn_id, self.functions.len()
                            ));
                        }
                        let policy_fn_decl = &self.functions[p_fn_id as usize];
                        if policy_fn_decl.key != policy_entry.handler {
                            return reject(format!(
                                "route {} policy {} handler {} does not match plan.policy_handler_id {} ({})",
                                route.id, p, policy_entry.handler, p_fn_id, policy_fn_decl.key
                            ));
                        }
                        if policy_fn_decl.kind != FunctionKind::PolicyHandler {
                            return reject(format!(
                                "route {} policy {} plan.policy_handler_id {} points to non-policy function kind {:?}",
                                route.id, p, p_fn_id, policy_fn_decl.kind
                            ));
                        }
                    } else if plan.policy_handler_id.is_some() {
                        return reject(format!(
                            "route {} has no policy but declares plan.policy_handler_id {:?}",
                            route.id, plan.policy_handler_id
                        ));
                    }
                }
            } else if !self.functions.is_empty() {
                return reject(format!(
                    "route {} missing required plan in numeric pack mode",
                    route.id
                ));
            }
        }
        for (policy_id, policy) in &self.policies {
            if policy.id != *policy_id {
                return reject(format!("policy key {policy_id} != entry id {}", policy.id));
            }
            if !self.handler_table.is_empty() && !self.handler_table.contains_key(&policy.handler) {
                return reject(format!(
                    "policy {policy_id} references unknown handler {}",
                    policy.handler
                ));
            }
            if !self.functions.is_empty() {
                let found = self
                    .functions
                    .iter()
                    .any(|f| f.key == policy.handler && f.kind == FunctionKind::PolicyHandler);
                if !found {
                    return reject(format!(
                        "policy {policy_id} handler {} not found in function manifest as PolicyHandler",
                        policy.handler
                    ));
                }
            }
        }
        for cap in &self.capabilities {
            if !["timer", "raw-response", "full-request"].contains(&cap.as_str()) {
                return reject(format!("unknown capability {} declared", cap));
            }
        }
        // M26-002-B: bytecode fails closed on cross-target mismatch —
        // the guardrail is enforced BEFORE ready, never at eval time.
        // Bytecode without a target cannot prove compatibility and
        // rejects (the embed tool always stamps one).
        if let Some(bc) = &self.bundle_bytecode {
            let host_endianness = if cfg!(target_endian = "big") {
                "big"
            } else {
                "little"
            };
            let target = bc.target.as_ref().ok_or_else(|| {
                PackError::Rejected(
                    "bytecode present without a target fingerprint (incompatible dimension: target triple) — cannot verify cross-target compatibility, fail closed".into(),
                )
            })?;
            let mismatches: Vec<&str> = [
                (target.arch != std::env::consts::ARCH, "arch"),
                (target.os != std::env::consts::OS, "os"),
                (
                    target.pointer_width as usize != std::mem::size_of::<usize>() * 8,
                    "pointer width",
                ),
                (target.endianness != host_endianness, "endianness"),
            ]
            .into_iter()
            .filter(|(mismatch, _)| *mismatch)
            .map(|(_, dim)| dim)
            .collect();
            if !mismatches.is_empty() {
                return reject(format!(
                    "cross-target pack rejected (incompatible dimensions: {}): bytecode targets {}/{} {}-bit {} endian, runtime is {}/{} {}-bit {} endian — rebuild the pack for this target, or start with --no-bytecode to run from source",
                    mismatches.join(", "),
                    target.arch, target.os, target.pointer_width, target.endianness,
                    std::env::consts::ARCH, std::env::consts::OS,
                    std::mem::size_of::<usize>() * 8, host_endianness,
                ));
            }
        }
        // M26-002-A: when the pack declares a capability hash it must match
        // the declared set (dimension: capabilities)
        if !self.capability_hash.is_empty()
            && self.capability_hash != capability_hash(&self.capabilities)
        {
            return reject(format!(
                "capability hash mismatch (incompatible dimension: capabilities): pack declares {}, computed {}",
                self.capability_hash,
                capability_hash(&self.capabilities)
            ));
        }
        for route in &self.routes {
            // M25-007-B: a raw-response route's handler bypasses response
            // validation by design — a declared response schema would be a
            // contract claim the runtime never enforces, so it rejects.
            if route.capabilities.iter().any(|c| c == "raw-response") {
                for (status, decl) in &route.responses {
                    if decl.schema.is_some() {
                        return reject(format!(
                            "route {} declares the raw-response capability but status {} carries a response schema (unenforceable contract claim)",
                            route.id, status
                        ));
                    }
                }
            }
        }
        Ok(())
    }

    /// Canonical JSON over the execution graph (routes with plans, schemas, policies,
    /// capabilities, function manifest, schema manifest, serialized router).
    /// M25-001-C: the whole view passes through `canonical_value` (sorted keys,
    /// normalized numbers) so the hash is independent of field emission order.
    pub fn routes_canonical_json(&self) -> String {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Canonical<'a> {
            routes: &'a [RouteEntry],
            schemas: &'a BTreeMap<String, q_schema_runtime::SchemaIr>,
            policies: &'a BTreeMap<String, PolicyEntry>,
            capabilities: &'a [String],
            functions: &'a [FunctionDecl],
            schema_manifest: &'a [SchemaDecl],
            policy_manifest: &'a [PolicyDecl],
            router: &'a Option<SerializedRouter>,
        }
        let c = Canonical {
            routes: &self.routes,
            schemas: &self.schemas,
            policies: &self.policies,
            capabilities: &self.capabilities,
            functions: &self.functions,
            schema_manifest: &self.schema_manifest,
            policy_manifest: &self.policy_manifest,
            router: &self.router,
        };
        let v = serde_json::to_value(&c).expect("canonical serialization cannot fail");
        q_schema_runtime::canonical_value(&v).to_string()
    }

    /// Verification path: hash the canonical form through a Writer adapter —
    /// semantically identical to hashing `routes_canonical_json()` without
    /// materializing the (potentially ~500 KiB) string.
    pub fn routes_canonical_sha256(&self) -> String {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Canonical<'a> {
            routes: &'a [RouteEntry],
            schemas: &'a BTreeMap<String, q_schema_runtime::SchemaIr>,
            policies: &'a BTreeMap<String, PolicyEntry>,
            capabilities: &'a [String],
            functions: &'a [FunctionDecl],
            schema_manifest: &'a [SchemaDecl],
            policy_manifest: &'a [PolicyDecl],
            router: &'a Option<SerializedRouter>,
        }
        let c = Canonical {
            routes: &self.routes,
            schemas: &self.schemas,
            policies: &self.policies,
            capabilities: &self.capabilities,
            functions: &self.functions,
            schema_manifest: &self.schema_manifest,
            policy_manifest: &self.policy_manifest,
            router: &self.router,
        };
        // hash the canonical (sorted-key, number-normalized) form, matching
        // routes_canonical_json byte-for-byte
        let v = serde_json::to_value(&c).expect("canonical serialization cannot fail");
        let mut hasher = Sha256::new();
        hasher.update(q_schema_runtime::canonical_value(&v).to_string().as_bytes());
        hex(&hasher.finalize())
    }

    /// Canonical JSON over the PUBLIC API contract only (G0-r1):
    /// method, path, request schemas + coercion + content types + limits,
    /// declared response statuses/body schemas/public problems, security.
    /// Excludes: function keys/IDs, policy implementation handler, response
    /// serializer strategy, router layout, capability indexes, and schemas not
    /// reachable from a public binding/response.
    /// Stable across internal implementation reordering.
    pub fn public_contract_canonical_json(&self) -> String {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct PublicBinding<'a> {
            schema: Option<&'a str>,
            coerce: Option<&'a str>,
            content_type: Option<&'a str>,
            limit_bytes: u64,
        }
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct PublicResponse<'a> {
            schema: Option<&'a str>,
            problem: Option<&'a str>,
        }
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct PublicRoute<'a> {
            id: &'a str,
            method: &'a str,
            path: &'a str,
            params: Option<PublicBinding<'a>>,
            query: Option<PublicBinding<'a>>,
            headers: Option<PublicBinding<'a>>,
            body: Option<PublicBinding<'a>>,
            responses: BTreeMap<&'a str, PublicResponse<'a>>,
            security: &'a [SecurityReq],
        }
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct PublicPolicy<'a> {
            declared_statuses: &'a [u16],
            provides: Option<&'a str>,
        }
        fn project(b: &SourceBinding) -> PublicBinding<'_> {
            PublicBinding {
                schema: b.schema.as_deref(),
                coerce: b.coerce.as_deref(),
                content_type: b.content_type.as_deref(),
                limit_bytes: b.limit_bytes,
            }
        }
        // Reachability: only schemas referenced by a public binding or response
        let mut used: std::collections::BTreeSet<&str> = Default::default();
        let routes: Vec<PublicRoute> = self
            .routes
            .iter()
            .map(|r| {
                for b in [&r.params, &r.query, &r.body, &r.headers]
                    .into_iter()
                    .flatten()
                {
                    if let Some(k) = b.schema.as_deref() {
                        used.insert(k);
                    }
                }
                let mut responses = BTreeMap::new();
                for (status, decl) in &r.responses {
                    if let Some(k) = decl.schema.as_deref() {
                        used.insert(k);
                    }
                    responses.insert(
                        status.as_str(),
                        PublicResponse {
                            schema: decl.schema.as_deref(),
                            problem: decl.problem.as_deref(),
                        },
                    );
                }
                PublicRoute {
                    id: &r.id,
                    method: &r.method,
                    path: &r.path,
                    params: r.params.as_ref().map(project),
                    query: r.query.as_ref().map(project),
                    headers: r.headers.as_ref().map(project),
                    body: r.body.as_ref().map(project),
                    responses,
                    security: &r.security,
                }
            })
            .collect();
        let schemas: BTreeMap<&str, &q_schema_runtime::SchemaIr> = self
            .schemas
            .iter()
            .filter(|(k, _)| used.contains(k.as_str()))
            .map(|(k, v)| (k.as_str(), v))
            .collect();
        // Policies projected WITHOUT the implementation handler key
        let policies: BTreeMap<&str, PublicPolicy> = self
            .policies
            .iter()
            .map(|(k, p)| {
                (
                    k.as_str(),
                    PublicPolicy {
                        declared_statuses: &p.declared_statuses,
                        provides: p.provides.as_deref(),
                    },
                )
            })
            .collect();
        let v = serde_json::to_value((&routes, &schemas, &policies))
            .expect("canonical serialization cannot fail");
        // M25-001-C: same canonical form as the execution-graph hash
        q_schema_runtime::canonical_value(&v).to_string()
    }

    pub fn public_contract_sha256(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.public_contract_canonical_json().as_bytes());
        hex(&hasher.finalize())
    }
}

pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

const B64_ALPHA: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// Standard base64 encoder (RFC 4648, padded).
pub fn base64_encode(data: &[u8]) -> String {
    let mut out = String::with_capacity(data.len().div_ceil(3) * 4);
    for chunk in data.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = chunk.get(1).copied().unwrap_or(0) as u32;
        let b2 = chunk.get(2).copied().unwrap_or(0) as u32;
        let n = (b0 << 16) | (b1 << 8) | b2;
        out.push(B64_ALPHA[((n >> 18) & 63) as usize] as char);
        out.push(B64_ALPHA[((n >> 12) & 63) as usize] as char);
        out.push(if chunk.len() > 1 {
            B64_ALPHA[((n >> 6) & 63) as usize] as char
        } else {
            '='
        });
        out.push(if chunk.len() > 2 {
            B64_ALPHA[(n & 63) as usize] as char
        } else {
            '='
        });
    }
    out
}

/// Standard base64 decoder (RFC 4648, padded).
pub fn base64_decode(s: &str) -> Option<Vec<u8>> {
    let lookup = |c: u8| -> Option<u8> {
        match c {
            b'A'..=b'Z' => Some(c - b'A'),
            b'a'..=b'z' => Some(c - b'a' + 26),
            b'0'..=b'9' => Some(c - b'0' + 52),
            b'+' => Some(62),
            b'/' => Some(63),
            _ => None,
        }
    };
    let bytes = s.trim().as_bytes();
    let mut out = Vec::with_capacity(bytes.len() * 3 / 4);
    for chunk in bytes.chunks(4) {
        if chunk.len() < 2 {
            return None;
        }
        let c0 = lookup(chunk[0])? as u32;
        let c1 = lookup(chunk[1])? as u32;
        out.push(((c0 << 2) | (c1 >> 4)) as u8);

        if chunk.len() > 2 && chunk[2] != b'=' {
            let c2 = lookup(chunk[2])? as u32;
            out.push((((c1 & 0x0f) << 4) | (c2 >> 2)) as u8);
            if chunk.len() > 3 && chunk[3] != b'=' {
                let c3 = lookup(chunk[3])? as u32;
                out.push((((c2 & 0x03) << 6) | c3) as u8);
            }
        }
    }
    Some(out)
}

/// Test-support pack used by the fuzz integration test (deterministic,
/// integrity-hashed). Not part of the public API contract.
#[doc(hidden)]
pub fn minimal_pack_public() -> QPack {
    use sha2::{Digest, Sha256};
    let route = RouteEntry {
        id: "health.live".into(),
        module_id: "health".into(),
        method: "GET".into(),
        path: "/health/live".into(),
        path_segments: vec![
            PathSegment {
                kind: SegKind::Static,
                value: "health".into(),
            },
            PathSegment {
                kind: SegKind::Static,
                value: "live".into(),
            },
        ],
        handler: "health.live".into(),
        policy: None,
        params: None,
        query: None,
        body: None,
        headers: None,
        responses: BTreeMap::from([(
            "200".into(),
            ResponseDecl {
                schema: None,
                strategy: Strategy::Js,
                problem: None,
            },
        )]),
        validation_strategy: Strategy::Native,
        native_liveness: None,
        security: vec![],
        capabilities: vec![],
        deadline_ms: 5000,
        plan: None,
    };
    let mut pack = QPack {
        bundle_prelude: None,
        decoded_bytecode: None,
        header_name_table: Vec::new(),
        query_name_table: Vec::new(),
        cookie_name_table: Vec::new(),
        format_version: PACK_FORMAT_VERSION,
        kind: "velqu.qpack".into(),
        runtime_abi: RUNTIME_ABI,
        engine: EngineRef {
            name: ENGINE_NAME.into(),
            version: ENGINE_VERSION.into(),
            binding: ENGINE_BINDING.into(),
            rquickjs: RQUICKJS_VERSION.into(),
            build_hash: runtime_build_hash(),
        },
        schema_ir_version: SCHEMA_IR_VERSION,
        contract_version: CONTRACT_VERSION,
        contract_hash: String::new(),
        built_by: BuiltBy {
            compiler: "0.1.0".into(),
            typescript: String::new(),
            bun: String::new(),
        },
        app_id: "fuzz".into(),
        modules: vec!["health".into()],
        entry: "app.js".into(),
        bundle_form: None,
        execution_mode: None,
        bundle: "function h(){} __velquRegister('health.live', h);".into(),
        source_map: None,
        bundle_bytecode: None,
        routes: vec![route],
        schemas: BTreeMap::new(),
        policies: BTreeMap::new(),
        capabilities: vec![],
        capability_hash: String::new(),
        functions: vec![],
        schema_manifest: vec![],
        policy_manifest: vec![],
        router: None,
        handler_table: BTreeMap::from([("health.live".into(), "health.live".into())]),
        integrity: Integrity {
            algorithm: "sha256".into(),
            bundle_sha256: String::new(),
            routes_sha256: String::new(),
            bytecode_sha256: None,
        },
    };
    pack.integrity.bundle_sha256 = hex(&Sha256::digest(pack.bundle.as_bytes()));
    pack.integrity.routes_sha256 = hex(&Sha256::digest(pack.routes_canonical_json().as_bytes()));
    pack
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn minimal_pack() -> QPack {
        let route = RouteEntry {
            id: "health.live".into(),
            module_id: "health".into(),
            method: "GET".into(),
            path: "/health/live".into(),
            path_segments: vec![
                PathSegment {
                    kind: SegKind::Static,
                    value: "health".into(),
                },
                PathSegment {
                    kind: SegKind::Static,
                    value: "live".into(),
                },
            ],
            handler: "health.live".into(),
            policy: None,
            params: None,
            query: None,
            body: None,
            headers: None,
            responses: BTreeMap::from([(
                "200".into(),
                ResponseDecl {
                    schema: None,
                    strategy: Strategy::Js,
                    problem: None,
                },
            )]),
            validation_strategy: Strategy::Native,
            native_liveness: Some(LivenessSpec {
                status: 200,
                content_type: "application/json".into(),
                body: "{\"status\":\"ok\"}".into(),
            }),
            security: vec![],
            capabilities: vec![],
            deadline_ms: 5000,
            plan: None,
        };
        let mut pack = QPack {
            bundle_prelude: None,
            decoded_bytecode: None,
        header_name_table: Vec::new(),
            query_name_table: Vec::new(),
            cookie_name_table: Vec::new(),
            format_version: PACK_FORMAT_VERSION,
            kind: "velqu.qpack".into(),
            runtime_abi: RUNTIME_ABI,
            engine: EngineRef {
                name: ENGINE_NAME.into(),
                version: ENGINE_VERSION.into(),
                binding: ENGINE_BINDING.into(),
                rquickjs: RQUICKJS_VERSION.into(),
                build_hash: runtime_build_hash(),
            },
            schema_ir_version: SCHEMA_IR_VERSION,
            contract_version: CONTRACT_VERSION,
            contract_hash: String::new(),
            built_by: BuiltBy {
                compiler: "0.1.0".into(),
                typescript: String::new(),
                bun: String::new(),
            },
            app_id: "test".into(),
            modules: vec!["health".into()],
            entry: "app.js".into(),
            bundle_form: None,
            execution_mode: None,
            bundle: "function health_live(){return {status:'ok'}} __velquRegister('health.live', health_live);".into(),
            source_map: None,
            bundle_bytecode: None,
            routes: vec![route],
            schemas: BTreeMap::new(),
            policies: BTreeMap::new(),
            capabilities: vec![],
            capability_hash: String::new(),
            functions: vec![],
            schema_manifest: vec![],
            policy_manifest: vec![],
            router: None,
            handler_table: BTreeMap::from([("health.live".into(), "health_live".into())]),
            integrity: Integrity { algorithm: "sha256".into(), bundle_sha256: String::new(), routes_sha256: String::new(), bytecode_sha256: None },
        };
        pack.integrity.bundle_sha256 = hex(&Sha256::digest(pack.bundle.as_bytes()));
        pack.integrity.routes_sha256 =
            hex(&Sha256::digest(pack.routes_canonical_json().as_bytes()));
        pack
    }

    #[test]
    fn verifies_minimal_pack() {
        minimal_pack().verify().expect("valid pack");
    }

    // M26-001-A (ADR-0024): formatVersion is a closed numeric mode set.
    // Legacy v1 resolves to the named adapter; every other version fails
    // closed with no fallback path.
    #[test]
    fn legacy_v1_resolves_to_named_adapter() {
        assert_eq!(
            detect_pack_format_mode(PACK_FORMAT_LEGACY_V1).unwrap(),
            PackFormatMode::LegacyV1
        );
        assert_eq!(PACK_FORMAT_CURRENT, PACK_FORMAT_LEGACY_V1);
        minimal_pack().verify().expect("v1 pack loads via adapter");
    }

    #[test]
    fn unknown_versions_fail_closed() {
        for v in [0u32, 2, 3, u32::MAX] {
            let err = detect_pack_format_mode(v).unwrap_err();
            let msg = err.to_string();
            assert!(msg.contains("not supported"), "v{v}: {msg}");
            assert!(msg.contains("fail closed"), "v{v}: {msg}");
            // The rejection names the supported adapter instead of guessing.
            assert!(msg.contains("legacy-v1"), "v{v}: {msg}");
        }
        let mut p = minimal_pack();
        p.format_version = 2;
        let err = p.verify().unwrap_err().to_string();
        assert!(
            err.contains("not supported") && err.contains("fail closed"),
            "{err}"
        );
    }

    // ADR-0024: while M26-003 has not landed, the current numeric mode IS
    // legacy v1. This pin forces a conscious flip (constant + native
    // adapter) rather than an accidental drift.
    #[test]
    fn current_mode_is_pinned_until_native_v2_lands() {
        assert_eq!(PACK_FORMAT_CURRENT, 1);
        assert_eq!(PACK_FORMAT_VERSION, PACK_FORMAT_CURRENT);
    }

    // M25-007-A: fallback never activates silently — a js plan strategy
    // must carry a reason from the closed vocabulary, and a native plan
    // must not carry one.
    fn rehash(p: &mut QPack) {
        use sha2::{Digest, Sha256};
        p.integrity.bundle_sha256 = hex(&Sha256::digest(p.bundle.as_bytes()));
        p.integrity.routes_sha256 = hex(&Sha256::digest(p.routes_canonical_json().as_bytes()));
    }

    #[test]
    fn rejects_silent_fallback_and_invalid_reasons() {
        // js response strategy without a reason
        let mut p = numeric_pack();
        if let Some(plan) = p.routes[0].plan.as_mut() {
            plan.response_strategy = Strategy::Js;
            plan.response_fallback_reason = None;
        }
        p.routes[0].responses.get_mut("200").unwrap().strategy = Strategy::Js;
        rehash(&mut p);
        assert!(
            matches!(p.verify(), Err(PackError::Rejected(m)) if m.contains("without a fallback reason")),
            "silent fallback must reject"
        );

        // js validation strategy with an out-of-vocabulary reason
        let mut p = numeric_pack();
        p.routes[0].validation_strategy = Strategy::Js;
        if let Some(plan) = p.routes[0].plan.as_mut() {
            plan.validation_fallback_reason = Some("because-i-said-so".into());
        }
        rehash(&mut p);
        assert!(
            matches!(p.verify(), Err(PackError::Rejected(m)) if m.contains("closed FALLBACK_REASONS vocabulary")),
            "invalid reason must reject"
        );

        // native plan carrying a reason
        let mut p = numeric_pack();
        if let Some(plan) = p.routes[0].plan.as_mut() {
            plan.response_strategy = Strategy::Native;
            plan.response_fallback_reason = Some("explicit".into());
        }
        p.routes[0].responses.get_mut("200").unwrap().strategy = Strategy::Native;
        rehash(&mut p);
        assert!(
            matches!(p.verify(), Err(PackError::Rejected(m)) if m.contains("native but carries a fallback reason")),
            "native with reason must reject"
        );

        // a valid js strategy with a valid reason verifies
        let mut p = numeric_pack();
        if let Some(plan) = p.routes[0].plan.as_mut() {
            plan.response_strategy = Strategy::Js;
            plan.response_fallback_reason = Some("explicit".into());
        }
        p.routes[0].responses.get_mut("200").unwrap().strategy = Strategy::Js;
        rehash(&mut p);
        p.verify().expect("tagged fallback verifies");
    }

    #[test]
    fn rejects_tampered_bundle() {
        let mut p = minimal_pack();
        p.bundle.push(' ');
        assert!(
            matches!(p.verify(), Err(PackError::Rejected(m)) if m.contains("bundle sha256 mismatch"))
        );
    }

    #[test]
    fn rejects_tampered_routes() {
        let mut p = minimal_pack();
        p.routes[0].path = "/tampered".into();
        assert!(
            matches!(p.verify(), Err(PackError::Rejected(m)) if m.contains("execution graph sha256 mismatch"))
        );
    }

    // M26-001-C (ADR-0026): integrity and authenticity are different
    // questions. In-band digests prove bit-fidelity against the pack's own
    // integrity block — nothing more. A writer who can rewrite the pack
    // can also rewrite its digests, so a self-consistent pack verifies
    // WITHOUT any trust anchor. Authenticity ("who authorized these
    // bytes") is therefore out-of-band by design: detached signatures or
    // build provenance at deployment time; the runtime has no key store,
    // accepts no authenticity-by-declaration field, and forbids untrusted
    // arbitrary bytecode (compiler-owned embed path only).
    #[test]
    fn self_consistent_digests_verify_without_trust_anchor() {
        let mut p = minimal_pack();
        p.bundle.push(' '); // attacker rewrites content...
        rehash(&mut p); // ...and recomputes in-band digests to match
        p.verify().expect("integrity alone cannot detect this");
        // The corollary is policy, not code: origin authorization MUST come
        // from outside the pack (ADR-0026). No verify() input can express it.
    }

    // M26-001-D (ADR-0027): debug/source sidecars live outside the pack.
    // Verification neither requires nor consults them — a pack verifies
    // identically with or without embedded source-map text (legacy v1's
    // frozen optional field), and no verify() input can point at one.
    #[test]
    fn verification_is_independent_of_debug_sidecars() {
        let with_map = minimal_pack();
        assert!(with_map.source_map.is_none());
        with_map.verify().expect("production-style pack verifies");

        let mut debug_build = minimal_pack();
        debug_build.source_map = Some("{\"version\":3,\"sources\":[]}".into());
        debug_build
            .verify()
            .expect("debug-annotated v1 pack still verifies");
    }

    #[test]
    fn sources_sidecar_binds_to_one_pack_and_tooling_checks_are_advisory() {
        use sources_sidecar::{SidecarModule, SourcesSidecar, SIDECAR_FORMAT_VERSION};
        let sidecar = SourcesSidecar {
            format_version: SIDECAR_FORMAT_VERSION,
            pack_sha256: hex(&Sha256::digest(b"exact-pack-bytes")),
            bundle_source: Some("globalThis.__velquFunctionManifest = [];".into()),
            source_map: Some("{\"version\":3,\"sources\":[]}".into()),
            modules: vec![SidecarModule {
                id: "app.ts".into(),
                file: "app.ts".into(),
            }],
        };
        // round trip through the sidecar JSON form
        let json = serde_json::to_string(&sidecar).unwrap();
        let back: SourcesSidecar = serde_json::from_str(&json).unwrap();
        assert_eq!(back, sidecar);
        // tool-side binding: exact hash verifies, drift rejects
        let want = hex(&Sha256::digest(b"exact-pack-bytes"));
        assert_eq!(back.verify_against(&want), Ok(()));
        assert!(back
            .verify_against(&hex(&Sha256::digest(b"other-pack")))
            .is_err());
        // unknown sidecar format versions fail closed for tooling too
        let mut future = sidecar.clone();
        future.format_version = 2;
        assert!(future.verify_against(&want).is_err());
    }

    #[test]
    fn rejects_rquickjs_mismatch_with_dimension() {
        let mut p = minimal_pack();
        p.engine.rquickjs = "0.11.0".into();
        let err = p.verify().unwrap_err();
        assert!(err.to_string().contains("rquickjs version mismatch"));
        assert!(err.to_string().contains("incompatible dimension: binding"));
    }

    #[test]
    fn rejects_build_hash_mismatch_with_dimension() {
        let mut p = minimal_pack();
        p.engine.build_hash = "deadbeef".into();
        let err = p.verify().unwrap_err();
        assert!(err.to_string().contains("runtime build hash mismatch"));
        assert!(err
            .to_string()
            .contains("incompatible dimension: runtime build"));
        assert!(err.to_string().contains("pack rebuild"));
    }

    #[test]
    fn capability_hash_present_must_match_and_absent_is_v1_compatible() {
        // absent (legacy v1): loads unchecked
        let p = minimal_pack();
        assert!(p.capability_hash.is_empty());
        p.verify()
            .expect("absent capability hash keeps v1 packs loading");

        // present + correct: verifies
        let mut p = minimal_pack();
        p.capability_hash = capability_hash(&p.capabilities);
        p.verify().expect("matching capability hash verifies");

        // present + wrong: rejects with the dimension named
        let mut p = minimal_pack();
        p.capability_hash = "00".repeat(32);
        let err = p.verify().unwrap_err();
        assert!(err.to_string().contains("capability hash mismatch"));
        assert!(err
            .to_string()
            .contains("incompatible dimension: capabilities"));
    }

    #[test]
    fn cross_target_bytecode_fails_closed_with_dimensions() {
        let bc_target = |arch: &str, os: &str, width: u8| BytecodeTarget {
            arch: arch.into(),
            os: os.into(),
            pointer_width: width,
            endianness: if cfg!(target_endian = "big") {
                "big".into()
            } else {
                "little".into()
            },
        };

        // wrong arch: rejected, dimension named, rebuild hint
        let mut p = minimal_pack();
        p.bundle_bytecode = Some(BundleBytecode {
            quickjs: ENGINE_VERSION.into(),
            binding: ENGINE_BINDING.into(),
            endianness: if cfg!(target_endian = "big") {
                "big".into()
            } else {
                "little".into()
            },
            target: Some(bc_target("msp430", std::env::consts::OS, 64)),
            data: String::new(),
        });
        p.integrity.bytecode_sha256 = Some(hex(&Sha256::digest(&b""[..])));
        let err = p.verify().unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("cross-target pack rejected"), "{msg}");
        assert!(msg.contains("arch"), "{msg}");
        assert!(msg.contains("rebuild the pack for this target"), "{msg}");

        // wrong pointer width
        let mut p = minimal_pack();
        p.bundle_bytecode = Some(BundleBytecode {
            quickjs: ENGINE_VERSION.into(),
            binding: ENGINE_BINDING.into(),
            endianness: if cfg!(target_endian = "big") {
                "big".into()
            } else {
                "little".into()
            },
            target: Some(bc_target(std::env::consts::ARCH, std::env::consts::OS, 16)),
            data: String::new(),
        });
        p.integrity.bytecode_sha256 = Some(hex(&Sha256::digest(&b""[..])));
        let msg = p.verify().unwrap_err().to_string();
        assert!(msg.contains("pointer width"), "{msg}");

        // wrong endianness
        let mut p = minimal_pack();
        let flipped = if cfg!(target_endian = "big") {
            "little"
        } else {
            "big"
        };
        p.bundle_bytecode = Some(BundleBytecode {
            quickjs: ENGINE_VERSION.into(),
            binding: ENGINE_BINDING.into(),
            endianness: flipped.into(),
            target: Some(BytecodeTarget {
                arch: std::env::consts::ARCH.into(),
                os: std::env::consts::OS.into(),
                pointer_width: 64,
                endianness: flipped.into(),
            }),
            data: String::new(),
        });
        p.integrity.bytecode_sha256 = Some(hex(&Sha256::digest(&b""[..])));
        let msg = p.verify().unwrap_err().to_string();
        assert!(msg.contains("endianness"), "{msg}");

        // bytecode WITHOUT a target: cannot prove compatibility, fail closed
        let mut p = minimal_pack();
        p.bundle_bytecode = Some(BundleBytecode {
            quickjs: ENGINE_VERSION.into(),
            binding: ENGINE_BINDING.into(),
            endianness: if cfg!(target_endian = "big") {
                "big".into()
            } else {
                "little".into()
            },
            target: None,
            data: String::new(),
        });
        p.integrity.bytecode_sha256 = Some(hex(&Sha256::digest(&b""[..])));
        let msg = p.verify().unwrap_err().to_string();
        assert!(msg.contains("without a target fingerprint"), "{msg}");
        assert!(msg.contains("fail closed"), "{msg}");
    }

    #[test]
    fn source_rebuild_path_loads_cross_target_bytecode_packs() {
        // a cross-target bytecode pack rejects under the default policy,
        // with the message pointing at BOTH recovery paths
        let mut p = minimal_pack();
        p.bundle_bytecode = Some(BundleBytecode {
            quickjs: ENGINE_VERSION.into(),
            binding: ENGINE_BINDING.into(),
            endianness: if cfg!(target_endian = "big") {
                "big".into()
            } else {
                "little".into()
            },
            target: Some(BytecodeTarget {
                arch: "bogus-arch".into(),
                os: std::env::consts::OS.into(),
                pointer_width: (std::mem::size_of::<usize>() * 8) as u8,
                endianness: if cfg!(target_endian = "big") {
                    "big".into()
                } else {
                    "little".into()
                },
            }),
            data: String::new(),
        });
        p.integrity.bytecode_sha256 = Some(hex(&Sha256::digest(&b""[..])));
        let msg = p.verify().unwrap_err().to_string();
        assert!(msg.contains("--no-bytecode"), "{msg}");
        assert!(msg.contains("rebuild the pack"), "{msg}");

        // the explicit source path verifies the SAME pack (bytecode
        // ignored; every other fingerprint dimension still enforced)
        p.verify_without_bytecode()
            .expect("source path verifies cross-target bytecode pack");
    }

    #[test]
    fn verify_caches_decoded_bytecode_exactly_once() {
        // host-matching bytecode with a correct hash: the ONE decode done
        // for the integrity check must also serve the engine handoff
        let code: &[u8] = b"qjsbc-module-bytes-payload";
        let mut p = minimal_pack();
        p.bundle_bytecode = Some(BundleBytecode {
            quickjs: ENGINE_VERSION.into(),
            binding: ENGINE_BINDING.into(),
            endianness: if cfg!(target_endian = "big") {
                "big".into()
            } else {
                "little".into()
            },
            target: Some(BytecodeTarget {
                arch: std::env::consts::ARCH.into(),
                os: std::env::consts::OS.into(),
                pointer_width: (std::mem::size_of::<usize>() * 8) as u8,
                endianness: if cfg!(target_endian = "big") {
                    "big".into()
                } else {
                    "little".into()
                },
            }),
            data: base64_encode(code),
        });
        p.integrity.bytecode_sha256 = Some(hex(&Sha256::digest(code)));
        assert!(p.decoded_bytecode.is_none());
        p.verify_and_cache_bytecode()
            .expect("valid host bytecode verifies and caches");
        assert_eq!(p.decoded_bytecode.as_deref(), Some(code));
        // plain verify() (no cache request) never populates the field
        let mut q = p.clone();
        q.decoded_bytecode = None;
        q.verify().expect("plain verify still works");
        assert!(q.decoded_bytecode.is_none());
    }

    #[test]
    fn bundle_prelude_marker_rules() {
        // embedded marker without bytecode rejects (source packs evaluate
        // the host prelude; the marker describes the bytecode module)
        let mut p = minimal_pack();
        p.bundle_prelude = Some("embedded".into());
        let msg = p.verify().unwrap_err().to_string();
        assert!(msg.contains("requires bundleBytecode"), "{msg}");
        // unknown marker values fail closed
        p.bundle_prelude = Some("inline".into());
        let msg = p.verify().unwrap_err().to_string();
        assert!(msg.contains("unknown bundlePrelude"), "{msg}");
        // the explicit source path clears the marker with the bytecode
        p.bundle_prelude = Some("embedded".into());
        p.verify_without_bytecode()
            .expect("source path verifies embedded-prelude packs (host prelude)");
    }

    #[test]
    fn failed_verify_leaves_no_cached_bytecode() {
        let mut p = minimal_pack();
        p.bundle_bytecode = Some(BundleBytecode {
            quickjs: ENGINE_VERSION.into(),
            binding: ENGINE_BINDING.into(),
            endianness: "little".into(),
            target: None,
            data: base64_encode(b"tampered-payload"),
        });
        p.integrity.bytecode_sha256 = Some(hex(&Sha256::digest(b"different-bytes")));
        assert!(p.verify_and_cache_bytecode().is_err());
        assert!(
            p.decoded_bytecode.is_none(),
            "a rejected pack must not hand bytecode to the engine"
        );
    }

    #[test]
    fn rejects_engine_mismatch() {
        let mut p = minimal_pack();
        p.engine.version = "0.99.0".into();
        assert!(matches!(p.verify(), Err(PackError::Rejected(m)) if m.contains("engine mismatch")));
    }

    #[test]
    fn rejects_abi_mismatch_and_duplicate_ids() {
        let mut p = minimal_pack();
        p.runtime_abi = 99;
        assert!(p.verify().is_err());
        // structural diagnostics model a (buggy) compiler output: integrity is
        // recomputed so the structural check itself is what fires
        let mut p = minimal_pack();
        p.routes.push(p.routes[0].clone());
        p.integrity.routes_sha256 = hex(&Sha256::digest(p.routes_canonical_json().as_bytes()));
        assert!(
            matches!(p.verify(), Err(PackError::Rejected(m)) if m.contains("duplicate route id"))
        );
    }

    #[test]
    fn rejects_unknown_handler_reference() {
        let mut p = minimal_pack();
        p.routes[0].handler = "missing".into();
        p.integrity.routes_sha256 = hex(&Sha256::digest(p.routes_canonical_json().as_bytes()));
        assert!(
            matches!(p.verify(), Err(PackError::Rejected(m)) if m.contains("unknown handler table key"))
        );
    }

    /// M2.2.1-r2 (fail closed): a policy entry whose declared handler is not
    /// in handler_table must be rejected at pack verification — otherwise the
    /// engine would face a protected route with an unresolvable policy.
    #[test]
    fn rejects_policy_with_unknown_handler() {
        let mut p = minimal_pack();
        p.routes[0].policy = Some("auth.session".into());
        p.policies.insert(
            "auth.session".into(),
            PolicyEntry {
                id: "auth.session".into(),
                handler: "auth.session.missing".into(),
                declared_statuses: vec![401],
                provides: Some("session".into()),
            },
        );
        // integrity must cover the mutated routes and policy table so the
        // structural check is what fires, not the digest check
        p.integrity.routes_sha256 = hex(&Sha256::digest(p.routes_canonical_json().as_bytes()));
        let err = p.verify().expect_err("policy handler gap must be rejected");
        assert!(matches!(&err, PackError::Rejected(m) if m.contains("unknown handler")));
    }

    /// A policy whose handler IS present must verify cleanly.
    #[test]
    fn accepts_policy_with_resolvable_handler() {
        let mut p = minimal_pack();
        p.bundle = format!(
            "{} __velquRegister('auth.session', function(){{}});",
            p.bundle
        );
        p.integrity.bundle_sha256 = hex(&Sha256::digest(p.bundle.as_bytes()));
        p.handler_table
            .insert("auth.session".into(), "auth.session".into());
        p.routes[0].policy = Some("auth.session".into());
        p.policies.insert(
            "auth.session".into(),
            PolicyEntry {
                id: "auth.session".into(),
                handler: "auth.session".into(),
                declared_statuses: vec![401],
                provides: Some("session".into()),
            },
        );
        p.integrity.routes_sha256 = hex(&Sha256::digest(p.routes_canonical_json().as_bytes()));
        p.verify()
            .expect("policy with resolvable handler must verify");
    }

    /// Convert minimal_pack into a structurally valid NUMERIC pack:
    /// dense function manifest, empty handler table, compiled router present,
    /// and a matching single-route plan. Tests then mutate one field at a time.
    fn numeric_pack() -> QPack {
        let mut p = minimal_pack();
        p.functions = vec![FunctionDecl {
            id: 0,
            key: "health.live".into(),
            kind: FunctionKind::RouteHandler,
        }];
        p.handler_table.clear();
        p.router = Some(SerializedRouter {
            nodes: vec![SerializedRouterNode {
                static_edges: vec![SerializedStaticEdge {
                    segment: "health".into(),
                    target_node: 1,
                }],
                param_edge: None,
                wildcard_edge: None,
                terminal: None,
            }],
        });
        // node 1: "live" edge; node 2: GET terminal pointing at route 0
        if let Some(ref mut r) = p.router {
            r.nodes.push(SerializedRouterNode {
                static_edges: vec![SerializedStaticEdge {
                    segment: "live".into(),
                    target_node: 2,
                }],
                param_edge: None,
                wildcard_edge: None,
                terminal: None,
            });
            r.nodes.push(SerializedRouterNode {
                static_edges: vec![],
                param_edge: None,
                wildcard_edge: None,
                terminal: Some(SerializedTerminal {
                    method_mask: 1, // GET
                    route_by_method: [Some(0), None, None, None, None, None, None],
                }),
            });
        }
        p.routes[0].plan = Some(RoutePlanDecl {
            route_id: 0,
            handler_id: 0,
            policy_id: None,
            policy_handler_id: None,
            params_schema_id: None,
            query_schema_id: None,
            headers_schema_id: None,
            body_schema_id: None,
            header_name_ids: vec![],
            query_name_ids: vec![],
            cookie_name_ids: vec![],
            default_status: 200,
            allowed_statuses: vec![200],
            field_needs: FieldNeeds::default(),
            response_strategy: Strategy::Js,
            validation_fallback_reason: None,
            response_fallback_reason: Some("explicit".into()),
            deadline_ms: 5000,
        });
        p.execution_mode = Some("numeric".into());
        p.policy_manifest = vec![]; // no policies declared
        p.contract_hash = p.public_contract_sha256()[..32].to_string();
        p.integrity.routes_sha256 = hex(&Sha256::digest(p.routes_canonical_json().as_bytes()));
        p
    }

    #[test]
    fn query_name_ids_are_canonical_and_cookie_table_is_bounded() {
        let mut pack = numeric_pack();
        pack.schemas.insert(
            "sch:health.query".into(),
            q_schema_runtime::SchemaIr::Object {
                properties: [
                    (
                        "z".into(),
                        Box::new(q_schema_runtime::SchemaIr::String {
                            min_length: None,
                            max_length: None,
                            pattern: None,
                            format: None,
                        }),
                    ),
                    (
                        "a".into(),
                        Box::new(q_schema_runtime::SchemaIr::String {
                            min_length: None,
                            max_length: None,
                            pattern: None,
                            format: None,
                        }),
                    ),
                ]
                .into_iter()
                .collect(),
                required: vec![],
            },
        );
        pack.schema_manifest.push(SchemaDecl {
            id: 0,
            key: "sch:health.query".into(),
            features: q_schema_runtime::features_of(&pack.schemas["sch:health.query"]),
            ir: pack.schemas["sch:health.query"].clone(),
        });
        pack.routes[0].query = Some(SourceBinding {
            schema: Some("sch:health.query".into()),
            coerce: Some("query".into()),
            content_type: None,
            limit_bytes: 0,
        });
        let plan = pack.routes[0].plan.as_mut().unwrap();
        plan.query_name_ids = vec![0, 1];
        plan.query_schema_id = Some(0);
        plan.field_needs.query = true;
        pack.query_name_table = vec!["a".into(), "z".into()];
        pack.contract_hash = pack.public_contract_sha256()[..32].to_string();
        pack.integrity.routes_sha256 =
            hex(&Sha256::digest(pack.routes_canonical_json().as_bytes()));
        pack.verify()
            .expect("query names derive to sorted dense ids");

        let mut bad = pack.clone();
        bad.routes[0].plan.as_mut().unwrap().query_name_ids = vec![1, 0];
        bad.integrity.routes_sha256 = hex(&Sha256::digest(bad.routes_canonical_json().as_bytes()));
        let err = bad.verify().unwrap_err();
        assert!(err.to_string().contains("queryNameIds"), "{err}");
    }

    #[test]
    fn accepts_valid_numeric_pack() {
        numeric_pack()
            .verify()
            .expect("valid numeric pack must verify");
    }

    #[test]
    fn numeric_pack_with_handler_table_is_rejected() {
        let mut p = numeric_pack();
        p.handler_table
            .insert("health.live".into(), "health.live".into());
        p.integrity.routes_sha256 = hex(&Sha256::digest(p.routes_canonical_json().as_bytes()));
        let err = p.verify().unwrap_err();
        assert!(matches!(err, PackError::Rejected(m) if m.contains("must not carry handlerTable")));
    }

    #[test]
    fn numeric_pack_without_compiled_router_is_rejected() {
        let mut p = numeric_pack();
        p.router = None;
        p.integrity.routes_sha256 = hex(&Sha256::digest(p.routes_canonical_json().as_bytes()));
        let err = p.verify().unwrap_err();
        assert!(
            matches!(err, PackError::Rejected(m) if m.contains("requires the compiled router automaton"))
        );
    }

    #[test]
    fn numeric_pack_with_incomplete_schema_manifest_is_rejected() {
        let mut p = numeric_pack();
        p.schemas.insert(
            "sch:health.query".into(),
            q_schema_runtime::SchemaIr::Object {
                properties: BTreeMap::new(),
                required: vec![],
            },
        );
        p.integrity.routes_sha256 = hex(&Sha256::digest(p.routes_canonical_json().as_bytes()));
        let err = p.verify().unwrap_err();
        assert!(matches!(err, PackError::Rejected(m) if m.contains("must cover every schema")));
    }

    #[test]
    fn rejects_non_dense_function_manifest() {
        let mut p = numeric_pack();
        p.functions = vec![FunctionDecl {
            id: 1, // should be 0
            key: "health.live".into(),
            kind: FunctionKind::RouteHandler,
        }];
        p.integrity.routes_sha256 = hex(&Sha256::digest(p.routes_canonical_json().as_bytes()));
        let err = p.verify().unwrap_err();
        assert!(matches!(err, PackError::Rejected(m) if m.contains("must be dense 0..N")));
    }

    #[test]
    fn rejects_out_of_range_handler_id() {
        let mut p = numeric_pack();
        p.routes[0].plan = Some(RoutePlanDecl {
            route_id: 0,
            handler_id: 5, // out of range
            policy_id: None,
            policy_handler_id: None,
            params_schema_id: None,
            query_schema_id: None,
            headers_schema_id: None,
            body_schema_id: None,
            header_name_ids: vec![],
            query_name_ids: vec![],
            cookie_name_ids: vec![],
            default_status: 200,
            allowed_statuses: vec![200],
            field_needs: FieldNeeds::default(),
            response_strategy: Strategy::Js,
            validation_fallback_reason: None,
            response_fallback_reason: Some("explicit".into()),
            deadline_ms: 5000,
        });
        p.integrity.routes_sha256 = hex(&Sha256::digest(p.routes_canonical_json().as_bytes()));
        let err = p.verify().unwrap_err();
        assert!(matches!(err, PackError::Rejected(m) if m.contains("out of range")));
    }

    #[test]
    fn rejects_mismatched_handler_id() {
        let mut p = numeric_pack();
        p.functions = vec![
            FunctionDecl {
                id: 0,
                key: "other.handler".into(),
                kind: FunctionKind::RouteHandler,
            },
            FunctionDecl {
                id: 1,
                key: "health.live".into(),
                kind: FunctionKind::RouteHandler,
            },
        ];
        // plan.handler_id 0 still points at functions[0] = other.handler,
        // while route.handler is health.live
        if let Some(ref mut plan) = p.routes[0].plan {
            plan.handler_id = 0;
        }
        p.integrity.routes_sha256 = hex(&Sha256::digest(p.routes_canonical_json().as_bytes()));
        let err = p.verify().unwrap_err();
        assert!(
            matches!(err, PackError::Rejected(m) if m.contains("does not match route.handler"))
        );
    }

    #[test]
    fn rejects_wrong_function_kind() {
        let mut p = numeric_pack();
        p.functions[0].kind = FunctionKind::PolicyHandler; // wrong kind for route handler
        p.integrity.routes_sha256 = hex(&Sha256::digest(p.routes_canonical_json().as_bytes()));
        let err = p.verify().unwrap_err();
        assert!(matches!(err, PackError::Rejected(m) if m.contains("non-route function kind")));
    }

    #[test]
    fn rejects_undeclared_default_status() {
        let mut p = numeric_pack();
        p.routes[0].plan = Some(RoutePlanDecl {
            route_id: 0,
            handler_id: 0,
            policy_id: None,
            policy_handler_id: None,
            params_schema_id: None,
            query_schema_id: None,
            headers_schema_id: None,
            body_schema_id: None,
            header_name_ids: vec![],
            query_name_ids: vec![],
            cookie_name_ids: vec![],
            default_status: 201, // 201 not in allowed_statuses [200]
            allowed_statuses: vec![200],
            field_needs: FieldNeeds::default(),
            response_strategy: Strategy::Js,
            validation_fallback_reason: None,
            response_fallback_reason: Some("explicit".into()),
            deadline_ms: 5000,
        });
        p.integrity.routes_sha256 = hex(&Sha256::digest(p.routes_canonical_json().as_bytes()));
        let err = p.verify().unwrap_err();
        assert!(
            matches!(err, PackError::Rejected(m) if m.contains("not in declared response statuses"))
        );
    }

    #[test]
    fn rejects_unmapped_response_status() {
        let mut p = numeric_pack();
        // Route only declares 200, but plan specifies [200, 418]
        p.routes[0].plan = Some(RoutePlanDecl {
            route_id: 0,
            handler_id: 0,
            policy_id: None,
            policy_handler_id: None,
            params_schema_id: None,
            query_schema_id: None,
            headers_schema_id: None,
            body_schema_id: None,
            header_name_ids: vec![],
            query_name_ids: vec![],
            cookie_name_ids: vec![],
            default_status: 200,
            allowed_statuses: vec![200, 418],
            field_needs: FieldNeeds::default(),
            response_strategy: Strategy::Js,
            validation_fallback_reason: None,
            response_fallback_reason: Some("explicit".into()),
            deadline_ms: 5000,
        });
        p.integrity.routes_sha256 = hex(&Sha256::digest(p.routes_canonical_json().as_bytes()));
        let err = p.verify().unwrap_err();
        assert!(
            matches!(err, PackError::Rejected(m) if m.contains("does not match declared response statuses"))
        );
    }

    #[test]
    fn rejects_mismatched_field_needs() {
        let mut p = numeric_pack();
        // Route has no query binding, but plan claims query: true
        p.routes[0].plan = Some(RoutePlanDecl {
            route_id: 0,
            handler_id: 0,
            policy_id: None,
            policy_handler_id: None,
            params_schema_id: None,
            query_schema_id: None,
            headers_schema_id: None,
            body_schema_id: None,
            header_name_ids: vec![],
            query_name_ids: vec![],
            cookie_name_ids: vec![],
            default_status: 200,
            allowed_statuses: vec![200],
            field_needs: FieldNeeds {
                params: false,
                query: true,
                headers: false,
                body: false,
            },
            response_strategy: Strategy::Js,
            validation_fallback_reason: None,
            response_fallback_reason: Some("explicit".into()),
            deadline_ms: 5000,
        });
        p.integrity.routes_sha256 = hex(&Sha256::digest(p.routes_canonical_json().as_bytes()));
        let err = p.verify().unwrap_err();
        assert!(matches!(err, PackError::Rejected(m) if m.contains("plan.field_needs")));
    }

    #[test]
    fn rejects_mismatched_deadline() {
        let mut p = numeric_pack();
        p.routes[0].deadline_ms = 5000;
        p.routes[0].plan = Some(RoutePlanDecl {
            route_id: 0,
            handler_id: 0,
            policy_id: None,
            policy_handler_id: None,
            params_schema_id: None,
            query_schema_id: None,
            headers_schema_id: None,
            body_schema_id: None,
            header_name_ids: vec![],
            query_name_ids: vec![],
            cookie_name_ids: vec![],
            default_status: 200,
            allowed_statuses: vec![200],
            field_needs: FieldNeeds::default(),
            response_strategy: Strategy::Js,
            validation_fallback_reason: None,
            response_fallback_reason: Some("explicit".into()),
            deadline_ms: 1000, // differs from route.deadline_ms
        });
        p.integrity.routes_sha256 = hex(&Sha256::digest(p.routes_canonical_json().as_bytes()));
        let err = p.verify().unwrap_err();
        assert!(matches!(err, PackError::Rejected(m) if m.contains("deadline_ms")));
    }

    #[test]
    fn rejects_schema_id_mismatch() {
        let mut p = numeric_pack();
        p.schemas.insert(
            "sch:health.query".into(),
            q_schema_runtime::SchemaIr::Object {
                properties: BTreeMap::new(),
                required: vec![],
            },
        );
        p.schema_manifest = vec![SchemaDecl {
            id: 0,
            key: "sch:health.query".into(),
            features: vec![],
            ir: q_schema_runtime::SchemaIr::Object {
                properties: BTreeMap::new(),
                required: vec![],
            },
        }];
        p.routes[0].query = Some(SourceBinding {
            schema: Some("sch:health.query".into()),
            coerce: Some("query".into()),
            content_type: None,
            limit_bytes: 0,
        });
        // manifest covers the schema so the completeness check passes and the
        // plan-level querySchemaId gap is what fires
        p.schema_manifest = vec![SchemaDecl {
            id: 0,
            key: "sch:health.query".into(),
            features: vec![],
            ir: q_schema_runtime::SchemaIr::Object {
                properties: BTreeMap::new(),
                required: vec![],
            },
        }];
        // Plan has querySchemaId = None while route declares query schema
        p.routes[0].plan = Some(RoutePlanDecl {
            route_id: 0,
            handler_id: 0,
            policy_id: None,
            policy_handler_id: None,
            params_schema_id: None,
            query_schema_id: None,
            headers_schema_id: None,
            body_schema_id: None,
            header_name_ids: vec![],
            query_name_ids: vec![],
            cookie_name_ids: vec![],
            default_status: 200,
            allowed_statuses: vec![200],
            field_needs: FieldNeeds {
                params: false,
                query: true,
                headers: false,
                body: false,
            },
            response_strategy: Strategy::Js,
            validation_fallback_reason: None,
            response_fallback_reason: Some("explicit".into()),
            deadline_ms: 5000,
        });
        p.contract_hash = p.public_contract_sha256()[..32].to_string();
        p.integrity.routes_sha256 = hex(&Sha256::digest(p.routes_canonical_json().as_bytes()));
        let err = p.verify().unwrap_err();
        assert!(matches!(err, PackError::Rejected(m) if m.contains("querySchemaId is None")));
    }

    #[test]
    fn policy_handler_rename_keeps_public_contract_hash() {
        let mut p1 = minimal_pack();
        let mut p2 = minimal_pack();
        // both declare the same policy; p2 renames ONLY the implementation handler key
        for p in [&mut p1, &mut p2] {
            p.policies.insert(
                "auth.session".into(),
                PolicyEntry {
                    id: "auth.session".into(),
                    handler: "impl.old".into(),
                    declared_statuses: vec![401],
                    provides: None,
                },
            );
        }
        p2.policies.get_mut("auth.session").unwrap().handler = "impl.renamed".into();
        assert_eq!(p1.public_contract_sha256(), p2.public_contract_sha256());
    }

    #[test]
    fn serializer_strategy_change_keeps_public_contract_hash() {
        let mut p1 = minimal_pack();
        let mut p2 = minimal_pack();
        p1.routes[0].responses.get_mut("200").unwrap().strategy = Strategy::Js;
        p2.routes[0].responses.get_mut("200").unwrap().strategy = Strategy::Native;
        assert_eq!(p1.public_contract_sha256(), p2.public_contract_sha256());
    }

    #[test]
    fn header_contract_change_changes_public_contract_hash() {
        let p1 = minimal_pack();
        let mut p2 = minimal_pack();
        p2.routes[0].headers = Some(SourceBinding {
            schema: Some("sch:h".into()),
            coerce: None,
            content_type: None,
            limit_bytes: 0,
        });
        assert_ne!(p1.public_contract_sha256(), p2.public_contract_sha256());
    }

    #[test]
    fn body_content_type_change_changes_public_contract_hash() {
        let mut p1 = minimal_pack();
        let mut p2 = minimal_pack();
        p1.routes[0].body = Some(SourceBinding {
            schema: None,
            coerce: None,
            content_type: Some("application/json".into()),
            limit_bytes: 64,
        });
        p2.routes[0].body = Some(SourceBinding {
            schema: None,
            coerce: None,
            content_type: Some("text/plain".into()),
            limit_bytes: 64,
        });
        assert_ne!(p1.public_contract_sha256(), p2.public_contract_sha256());
    }

    #[test]
    fn empty_contract_hash_in_numeric_mode_is_rejected() {
        let mut p = numeric_pack();
        p.contract_hash = String::new();
        p.integrity.routes_sha256 = hex(&Sha256::digest(p.routes_canonical_json().as_bytes()));
        let err = p.verify().unwrap_err();
        assert!(matches!(err, PackError::Rejected(m) if m.contains("must declare contractHash")));
    }

    #[test]
    fn public_contract_hash_is_stable_when_function_ids_are_reordered() {
        let mut p1 = minimal_pack();
        p1.functions = vec![
            FunctionDecl {
                id: 0,
                key: "health.live".into(),
                kind: FunctionKind::RouteHandler,
            },
            FunctionDecl {
                id: 1,
                key: "other.route".into(),
                kind: FunctionKind::RouteHandler,
            },
        ];
        let mut p2 = minimal_pack();
        p2.functions = vec![
            FunctionDecl {
                id: 0,
                key: "other.route".into(),
                kind: FunctionKind::RouteHandler,
            },
            FunctionDecl {
                id: 1,
                key: "health.live".into(),
                kind: FunctionKind::RouteHandler,
            },
        ];
        // Public contract hash depends on public API routes/schemas, NOT internal function ordering
        assert_eq!(p1.public_contract_sha256(), p2.public_contract_sha256());
        // Execution graph hash DOES change because internal layout changed
        assert_ne!(p1.routes_canonical_sha256(), p2.routes_canonical_sha256());
    }

    #[test]
    fn router_empty_nodes_rejected_before_ready() {
        let mut p = numeric_pack();
        p.router = Some(SerializedRouter { nodes: vec![] });
        p.integrity.routes_sha256 = hex(&Sha256::digest(p.routes_canonical_json().as_bytes()));
        let err = p.verify().unwrap_err();
        assert!(
            matches!(err, PackError::Rejected(m) if m.contains("serialized router rejected") && m.contains("no nodes"))
        );
    }

    #[test]
    fn router_method_slot_tamper_is_rejected() {
        // in-range but wrong-method slot: POST slot holds the GET route
        let mut p = numeric_pack();
        if let Some(ref mut r) = p.router {
            let term = r.nodes.last_mut().unwrap().terminal.as_mut().unwrap();
            term.route_by_method = [None, Some(0), None, None, None, None, None];
            term.method_mask = 1 << 1;
        }
        p.integrity.routes_sha256 = hex(&Sha256::digest(p.routes_canonical_json().as_bytes()));
        let err = p.verify().unwrap_err();
        assert!(matches!(err, PackError::Rejected(m) if m.contains("serialized router rejected")));
    }

    #[test]
    fn router_path_shape_tamper_is_rejected() {
        // route's path no longer walks to its terminal: repoint the static edge
        let mut p = numeric_pack();
        if let Some(ref mut r) = p.router {
            r.nodes[0].static_edges[0].segment = "wrong".into();
        }
        p.integrity.routes_sha256 = hex(&Sha256::digest(p.routes_canonical_json().as_bytes()));
        let err = p.verify().unwrap_err();
        assert!(matches!(err, PackError::Rejected(m) if m.contains("not represented in router")));
    }

    #[test]
    fn router_method_mask_mismatch_is_rejected() {
        let mut p = numeric_pack();
        if let Some(ref mut r) = p.router {
            let term = r.nodes.last_mut().unwrap().terminal.as_mut().unwrap();
            term.method_mask = 0b11; // claims GET+POST but only GET populated
        }
        p.integrity.routes_sha256 = hex(&Sha256::digest(p.routes_canonical_json().as_bytes()));
        let err = p.verify().unwrap_err();
        assert!(matches!(err, PackError::Rejected(m) if m.contains("methodMask")));
    }

    fn refresh_integrity(pack: &mut QPack) {
        pack.integrity.routes_sha256 = pack.routes_canonical_sha256();
        if !pack.contract_hash.is_empty() {
            pack.contract_hash = pack.public_contract_sha256()[..32].to_string();
        }
    }

    /// M24-005-A: header-name ids are compiled into RoutePlans and verified —
    /// a tampered table or tampered ids cannot load.
    #[test]
    fn header_name_table_and_ids_are_verified() {
        // numeric fixture with a security requirement on the planned route
        let mut pack = numeric_pack();
        pack.routes[0].security = vec![SecurityReq {
            scheme: "bearer".into(),
            header: "authorization".into(),
            problem_status: 401,
        }];
        pack.header_name_table = vec!["authorization".into()];
        let plan = pack.routes[0].plan.as_mut().unwrap();
        plan.header_name_ids = vec![0];
        plan.field_needs.headers = true;
        refresh_integrity(&mut pack);
        pack.verify().expect("consistent table verifies");

        // tampered table entry → rejected (does not match derived set)
        {
            let mut bad = pack.clone();
            bad.header_name_table.push("x-tampered".into());
            refresh_integrity(&mut bad);
            let err = bad.verify().unwrap_err();
            assert!(err.to_string().contains("headerNameTable"), "{err}");
        }
        // tampered ids on the security route → rejected
        {
            let mut bad = pack.clone();
            bad.routes[0].plan.as_mut().unwrap().header_name_ids = vec![9];
            refresh_integrity(&mut bad);
            let err = bad.verify().unwrap_err();
            assert!(err.to_string().contains("headerNameIds"), "{err}");
        }
        // fieldNeeds.headers true with no declared names → rejected
        {
            let mut bad = pack.clone();
            bad.routes[0].security.clear();
            bad.header_name_table.clear();
            let plan = bad.routes[0].plan.as_mut().unwrap();
            let mut needs = plan.field_needs;
            needs.headers = true;
            plan.field_needs = needs;
            plan.header_name_ids.clear();
            refresh_integrity(&mut bad);
            let err = bad.verify().unwrap_err();
            assert!(err.to_string().contains("fieldNeeds.headers"), "{err}");
        }
    }

    /// M24-005-D: the full-Headers escape hatch is explicit and verified —
    /// only a schema-less headers binding yields the sentinel id, and any
    /// other route carrying it is rejected.
    #[test]
    fn full_headers_escape_hatch_is_explicit_and_verified() {
        let mut pack = numeric_pack();
        // convert the planned route into an escape-hatch route: schema-less
        // headers binding authorizes the sentinel id
        pack.routes[0].headers = Some(SourceBinding {
            schema: None,
            coerce: None,
            content_type: None,
            limit_bytes: 0,
        });
        pack.header_name_table = Vec::new();
        let plan = pack.routes[0].plan.as_mut().unwrap();
        plan.header_name_ids = vec![FULL_HEADERS_ID];
        let mut needs = plan.field_needs;
        needs.headers = true;
        plan.field_needs = needs;
        refresh_integrity(&mut pack);
        pack.verify()
            .expect("schema-less headers binding authorizes the sentinel id");

        // the same sentinel WITHOUT the escape-hatch binding is rejected
        let mut bad = pack.clone();
        bad.routes[0].headers = None;
        refresh_integrity(&mut bad);
        let err = bad.verify().unwrap_err();
        assert!(err.to_string().contains("headerNameIds"), "{err}");
    }

    #[test]
    fn router_terminal_target_tamper_breaks_execution_hash() {
        let mut p = minimal_pack();
        p.router = Some(SerializedRouter {
            nodes: vec![SerializedRouterNode {
                static_edges: vec![],
                param_edge: None,
                wildcard_edge: None,
                terminal: Some(SerializedTerminal {
                    method_mask: 1,
                    route_by_method: [Some(0), None, None, None, None, None, None],
                }),
            }],
        });
        let h1 = p.routes_canonical_sha256();
        p.router.as_mut().unwrap().nodes[0]
            .terminal
            .as_mut()
            .unwrap()
            .route_by_method[0] = Some(1);
        let h2 = p.routes_canonical_sha256();
        assert_ne!(
            h1, h2,
            "tampering with router terminal target MUST change execution hash"
        );
    }

    #[test]
    fn schema_manifest_tamper_breaks_execution_hash() {
        let mut p = minimal_pack();
        p.schema_manifest = vec![SchemaDecl {
            id: 0,
            key: "sch:test".into(),
            features: vec![],
            ir: q_schema_runtime::SchemaIr::String {
                min_length: None,
                max_length: None,
                pattern: None,
                format: None,
            },
        }];
        let h1 = p.routes_canonical_sha256();
        p.schema_manifest[0].key = "sch:tampered".into();
        let h2 = p.routes_canonical_sha256();
        assert_ne!(
            h1, h2,
            "tampering with schema manifest MUST change execution hash"
        );
    }

    /// M25-001-B: compatibility markers fail closed — declared features must
    /// equal derived features; spoofing or omission rejects the pack.
    #[test]
    fn schema_manifest_features_mismatch_rejected() {
        let mut p = minimal_pack();
        p.schemas.insert(
            "sch:test".into(),
            q_schema_runtime::SchemaIr::String {
                min_length: None,
                max_length: None,
                pattern: None,
                format: None,
            },
        );
        p.schema_manifest = vec![SchemaDecl {
            id: 0,
            key: "sch:test".into(),
            // claims a fallback the IR does not use
            features: vec!["fallback".into()],
            ir: q_schema_runtime::SchemaIr::String {
                min_length: None,
                max_length: None,
                pattern: None,
                format: None,
            },
        }];
        p.integrity.routes_sha256 = hex(&Sha256::digest(p.routes_canonical_json().as_bytes()));
        let err = p.verify().unwrap_err();
        assert!(
            matches!(err, PackError::Rejected(m) if m.contains("features") && m.contains("derived"))
        );
    }

    #[test]
    fn schema_manifest_fallback_feature_must_be_declared() {
        let mut p = minimal_pack();
        let fallback_ir = q_schema_runtime::SchemaIr::Fallback {
            reason: "explicit".into(),
            inner: None,
        };
        p.schemas.insert("sch:test".into(), fallback_ir.clone());
        p.schema_manifest = vec![SchemaDecl {
            id: 0,
            key: "sch:test".into(),
            // hides the fallback marker the IR uses
            features: vec![],
            ir: fallback_ir,
        }];
        p.integrity.routes_sha256 = hex(&Sha256::digest(p.routes_canonical_json().as_bytes()));
        let err = p.verify().unwrap_err();
        assert!(
            matches!(err, PackError::Rejected(m) if m.contains("do not match derived features"))
        );
    }

    #[test]
    fn schema_manifest_unknown_fallback_reason_rejected() {
        let mut p = minimal_pack();
        let fallback_ir = q_schema_runtime::SchemaIr::Fallback {
            reason: "trust-me".into(),
            inner: None,
        };
        p.schemas.insert("sch:test".into(), fallback_ir.clone());
        p.schema_manifest = vec![SchemaDecl {
            id: 0,
            key: "sch:test".into(),
            features: q_schema_runtime::features_of(&fallback_ir),
            ir: fallback_ir,
        }];
        p.integrity.routes_sha256 = hex(&Sha256::digest(p.routes_canonical_json().as_bytes()));
        let err = p.verify().unwrap_err();
        assert!(matches!(err, PackError::Rejected(m) if m.contains("unknown fallback reason")));
    }

    #[test]
    fn schema_manifest_ir_mismatch_rejected() {
        let mut p = minimal_pack();
        p.schemas.insert(
            "sch:test".into(),
            q_schema_runtime::SchemaIr::String {
                min_length: Some(5),
                max_length: None,
                pattern: None,
                format: None,
            },
        );
        p.schema_manifest = vec![SchemaDecl {
            id: 0,
            key: "sch:test".into(),
            features: vec![],
            ir: q_schema_runtime::SchemaIr::String {
                min_length: Some(10), // mismatch with pack.schemas
                max_length: None,
                pattern: None,
                format: None,
            },
        }];
        p.integrity.routes_sha256 = hex(&Sha256::digest(p.routes_canonical_json().as_bytes()));
        let err = p.verify().unwrap_err();
        assert!(
            matches!(err, PackError::Rejected(m) if m.contains("IR does not match declared schema IR"))
        );
    }
}
