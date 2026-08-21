---
type: Architecture Decision Record
title: ADR-0023 Canonical Ordering and Hashing for Schema IR v2 Projections
status: accepted
date: 2026-08-21
implements: ADR-0022 (Schema IR v2), ADR-0012 (evidence discipline)
---

# ADR-0023: Canonical Ordering and Hashing for Schema IR v2 Projections

## Context

Before this decision, the TypeScript compiler computed pack integrity hashes by
serializing the extracted IR with schema-node keys in *source literal order*
(preserving how the developer wrote options), while the Rust runtime hashed its
typed re-serialization in *field declaration order*. Parity held only because
extraction happened to emit fields in declaration order — a convention, not a
guarantee. Two problems followed:

1. **Order sensitivity**: `s.string({ maxLength: 5, minLength: 1 })` and
   `s.string({ minLength: 1, maxLength: 5 })` describe the same schema but
   hashed differently across the boundary, rejecting valid packs.
2. **Number formatting**: Rust serializes the f64 `0.0` as `0.0` while
   JavaScript emits `0`; any schema with integral float bounds risked a
   byte-level hash mismatch.

M25-001-C owns the canonical ordering/hashing algorithm (per ADR-0022's scope
boundary).

## Decision

1. **Canonical JSON form** (JCS-inspired, RFC 8785-adjacent):
   - Object keys are **recursively sorted** (byte/code-unit order) at every
     level — schema IR nodes, route entries, manifests, tables.
   - Arrays keep their order (they are semantically ordered: `required`,
     union members, manifest lists, router edges).
   - Integral finite floats with magnitude ≤ 2^53−1 **normalize to integers**
     (`0.0` → `0`), matching JavaScript's number formatting.
2. **Single implementation per side, shared byte-for-byte**:
   - Rust: `q_schema_runtime::canonical_value` / `canonical_json`.
   - TypeScript: `@velqu/schema` `canonicalValue` / `canonicalJson`; the
     compiler (`sortIR`) and the benchmark fixture builder use the same
     recursive sort.
3. **Both hash surfaces canonicalize the whole view**:
   - Execution graph: `q_pack::routes_canonical_json` /
     `routes_canonical_sha256` serialize the full `Canonical` view to a value,
     canonicalize, then stringify/hash.
   - Public contract: `q_pack::public_contract_canonical_json` applies the
     same treatment; the compiler's `contractHash` mirrors it exactly.
4. **Golden canonical corpus**: `conformance/schema/golden/canonical/*.canonical.json`
   holds the committed canonical string for every corpus node; both language
   suites assert byte-equality against those files, locking cross-language
   parity deterministically.
5. **Hash values change; packs rebuild.** Canonicalization is part of the
   hashed bytes, so existing pack hashes are invalidated and packs must be
   recompiled (benchmark evidence refresh happens at the M25 gate with raw
   samples, per ADR-0012).

## Consequences

- Source literal field order no longer affects any hash; schema identity is
  order-insensitive everywhere both sides agree on canonicalization.
- Semantic diff (M25-008-D) can canonicalize both sides and compare strings.
- Non-integral floats rely on both runtimes' shortest round-trip formatting
  (ryu / ECMAScript), which agree for finite doubles.
- Keys are compared by UTF-8 byte order (Rust) / UTF-16 code units (JS);
  identical for ASCII keys, which all IR fields are. Schema property names
  with astral characters could theoretically diverge; documented, not
  enforced (property keys are developer-controlled ASCII identifiers in
  practice).

## Evidence

- `cargo test -p q-schema-runtime`: `m25_001_c_tests` (sorted keys, number
  normalization, emission-order insensitivity, golden canonical corpus).
- `bun test`: canonicalization suite in `conformance/schema/` + compiler test
  "option literal field order never changes canonical hashes" (two fixture
  apps with reversed option orders compile to identical `routesSha256` and
  `contractHash`).
- End-to-end parity: the Treaty/runtime conformance suites load TS-compiled
  packs into the Rust runtime, where `q-pack` verify() recomputes and compares
  both hashes — 63 bun tests + 155 Rust tests pass on the canonical form.
