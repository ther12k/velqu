---
type: Architecture Decision Record
title: ADR-0017 QuickJS Bytecode Embedding and Evaluation
status: accepted
date: 2026-08-18
implements: ADR-0014
---

# ADR-0017: QuickJS Bytecode Embedding and Evaluation

## Context

ADR-0014 proposed trusted version-pinned QuickJS bytecode to reduce cold-start
application load overhead, while requiring strict version matching and tamper
integrity. The 1,000-route cold start profile identified `bundle.load` (evaluating
1,000 function registrations from source text) as taking ~4.15 ms of engine time.

## Decision

1. **Format**: Additive optional `bundleBytecode` object in `velqu.qpack` v1:
   ```json
   "bundleBytecode": {
     "quickjs": "0.15.1",
     "binding": "rquickjs-0.12.2",
     "endianness": "little",
     "data": "<base64>"
   }
   ```
2. **Integrity & Gating**:
   - Pack loader strictly validates: `integrity.bytecodeSha256` matching raw decoded bytes.
   - Exact match required on: `quickjs` version ("0.15.1"), `binding` ("rquickjs-0.12.2"), and target `endianness`. Mismatched or tampered bytecode fails startup before ready (no silent execution).
3. **Engine Loading**:
   - `Module::load(ctx, bytes)` loads bytecode directly via QuickJS C FFI `JS_ReadObject` with `JS_READ_OBJ_BYTECODE | JS_READ_OBJ_ROM_DATA`.
   - The module is evaluated once, populating `globalThis.__velquHandlers` identically to the source-eval path.
4. **Tooling**:
   - `velqu-bytecode embed --pack <path>` compiles bundle source using the pinned `rquickjs` engine build and embeds the bytecode in-place or to `--out`.

## Evidence

Measured across 30 samples/cell with fresh processes (`benchmarks/raw/route-count/`):

| Scale | velqu (source) p50 | velqu (bytecode) p50 | Delta |
|---|---:|---:|---:|
| 25 routes | 3.20 ms | 3.10 ms | −3.1% |
| 1,000 routes | 16.23 ms | 14.49 ms | **−10.7% (−1.74 ms)** |

- At 1,000 routes, bytecode saves ~1.74 ms of cold-start latency by bypassing source lexical parsing and AST construction.
- Conformance test parity verified: 31/31 assertions pass on bytecode-loaded packs identically to source packs.

## Consequences

- Bytecode is compiled only on the target architecture (endianness-checked).
- Source remains in the pack as the human-inspectable baseline and source-map anchor.
- Public packs can be distributed with or without bytecode; runtime supports both transparently.
