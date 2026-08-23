---
type: Architecture Decision Record
title: ADR-0024 QPack v2 Numeric Current Mode and Legacy v1 Adapter
status: accepted
date: 2026-08-23
implements: ADR-0018 (M2.6 binary QPack v2 track), ADR-0014 (version-pinned trusted bytecode), ADR-0017 (bytecode embedding)
---

# ADR-0024: QPack v2 Numeric Current Mode and Legacy v1 Adapter

## Context

The production artifact `app.qpack` is today a single UTF-8 JSON file
(`docs/specs/pack-format-v1.md`, frozen). M2.6 replaces JSON/base64
startup reconstruction with binary sections (dense tables, raw
bytecode, reproducible bytes) while v1 packs remain loadable during a
migration window. The runtime needs an explicit, testable policy for
numeric pack versions before any v2 byte is produced.

## Decision

### 1. `formatVersion` is a numeric mode selector

- The set of modes is **closed**. Each supported version maps to exactly
  one named adapter; there is no generic parser.
- **Unknown versions fail closed**: reject at verify, name the supported
  modes in the error, never guess, never best-effort parse a newer or
  older layout.
- Exactly one mode is **CURRENT** per runtime build
  (`PACK_FORMAT_CURRENT`). Until M26-003 lands, CURRENT is legacy v1;
  the flip to v2 is one constant change plus a native adapter —
  reviewed and tested, never a silent rewrite.

```text
formatVersion ──> detect_pack_format_mode()
                    |-- 1 --> PackFormatMode::LegacyV1 (JSON adapter, frozen v1 shape)
                    +-- other -> Rejected("not supported ... fail closed")
   (v2 -> PackFormatMode::NativeV2 is added by M26-003; no producer emits it before then)
```

### 2. Legacy v1 adapter

Today's JSON loader/verifier becomes the **named** legacy-v1 adapter
(`PackFormatMode::LegacyV1`). It keeps enforcing every frozen v1 rule:
kind, exact engine fingerprint, runtime ABI, IR/contract versions,
integrity re-hash, plan strategies with closed-vocabulary fallback
reasons. The adapter is a compatibility boundary, not a second code
path into new features: v2-only capabilities are unrepresentable in v1
packs, and a pack claiming otherwise fails verification.

### 3. Current mode has no legacy handler table

A current-mode pack carries exactly the handler references of its own
mode layout. There is no embedded fallback table consulted when a field
is absent: missing required fields are verification errors. Mode
dispatch happens before every other check, so one adapter's semantics
can never leak into another mode's packs.

### 4. Trust model — untrusted arbitrary bytecode is forbidden

Same-process QuickJS executes **trusted application code only**
(constraint 14); nothing here is a sandbox. Bytecode enters an artifact
only through the compiler-owned rebuild path (`velqu-bytecode embed`,
ADR-0014/0017): version-pinned to the embedding engine, hash-bound in
the pack integrity block. There is no API that accepts arbitrary
bytecode from requests, environment, or third parties at runtime.

### 5. Compatibility matrix

| producer \ runtime | accepts v1 (legacy adapter) | accepts v2 (native) | unknown version |
|---|---|---|---|
| emits v1 (current compiler until M26-003) | loads | n/a — no producer yet | rejected |
| emits v2 (from M26-003) | rejected by pre-M26 runtimes (fail closed) | loads | rejected |
| hand-edited / hostile | rejected (integrity + fingerprint checks) | rejected | rejected |

### 6. Binary layout direction (goals frozen here; exact bytes in M26-003-B)

```text
v1 (today): single UTF-8 JSON object
  { identity+versions, bundle:string, routes[], schemaManifest[],
    routePlans[], nameTables[] }          // parse + canonical re-hash at load

v2 (goal): fixed magic header + section directory + dense sections
  [magic "VQPK"][version u32][abi u32][engine fingerprint]
  [section dir: id u16, offset u64, len u64, sha256]
  sections: strings | routes | plans | schemas | policies | bytecode | ...
  // zero JSON parsing; bounds-checked mmap-friendly reads; reproducible bytes
```

Section ids, alignment, bounds, and optional-section rules are specified
by M26-001-B/M26-003-B and must not be implemented from this sketch.

### 7. Migration rules

1. Compiler keeps emitting v1 until the v2 encoder lands (M26-003);
   runtime keeps accepting v1 via the adapter through the M2.6 window.
2. The CURRENT flip is atomic with the native adapter and its tests;
   both land in the same reviewed packet.
3. Unknown versions always fail closed — during migration and after.
4. v1 support ends only with an explicit, documented deprecation decision
   (owner track); it is never dropped implicitly by a refactor.

## Consequences

- Version handling is now data: one enum, one dispatch point, pinned by
  tests (`legacy_v1_resolves_to_named_adapter`,
  `unknown_versions_fail_closed`, `current_mode_is_pinned_until_native_v2_lands`).
- M26-003 gains a precise insertion point for the native adapter.
- The compatibility story for beta users is explicit: old packs keep
  working, new packs fail loudly on old runtimes.
