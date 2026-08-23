---
type: Architecture Decision Record
title: ADR-0025 QPack v2 Section Directory, Alignment, Bounds, and Versioning
status: accepted
date: 2026-08-23
implements: ADR-0024 (numeric mode policy), ADR-0018 (M2.6 track)
---

# ADR-0025: QPack v2 Section Directory, Alignment, Bounds, and Versioning

## Context

ADR-0024 made `formatVersion` a closed numeric mode set and deferred the
exact binary layout to M26-001-B. Before the M26-003 encoder/decoder can
be written, the directory mechanics must be frozen so encoder, decoder,
fuzzing, and tooling all target one normative document.

## Decision

`docs/specs/pack-format-v2.md` is the normative mode-2 layout:

- Fixed 64-byte header (`"VELQUQPK"` magic, format_version = 2,
  total_size, section_count; reserved zero bytes for future header
  growth via `header_size`).
- 64-byte fixed-stride section directory entries: u16 id, u16 flags
  (bit 0 OPTIONAL), u64 offset, u64 len, SHA-256 content digest.
- 8-byte alignment for all sections; disjointness and containment rules
  validated before any interpretation.
- Optional sections are declared per catalog; unknown section ids fail
  closed even when flagged optional — extensibility is a new numeric
  mode through ADR review, never skip-and-continue.
- No minor revisions inside a mode; layout changes bump `formatVersion`
  and land with their named adapter in one reviewed packet.
- The section-id catalog is reserved here (strings/routes/plans/
  schemas/policies/capabilities/bundle-bytecode/contract-summary);
  concrete encodings arrive with M26-003-B.

## Consequences

- Encoder (M26-003) and decoder implement against one checked-in spec;
  q-pack carries the layout constants so drift fails tests, not reviews.
- Integrity is bound per-section at the directory level now; separating
  integrity from authenticity remains M26-001-C.
- Denial-of-service posture is explicit: every count and range is
  validated against file length before allocation or use.
