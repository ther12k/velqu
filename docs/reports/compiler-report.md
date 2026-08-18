---
type: Evidence Report
title: Compiler Report (Static Extraction and Emission)
status: complete
milestone: M2
---

# Compiler report (M2 §12.2 / §12.8)

## Overview

The `@q/compiler` package implements static contract extraction and artifact
emission without application execution (COMP-001..009).

## Evidence & Verifications

| Requirement | Verification | Evidence |
|---|---|---|
| COMP-001 Production route discovery at build time | AST-only extraction of `route()`, `defineModule()`, `defineApp()` | `packages/compiler/src/extract.ts` |
| COMP-002 Zero side-effects / no app dry-run | Trap test with throwing service factory & global counter | `conformance/compiler/compiler.test.ts` (trap test PASS) |
| COMP-003 Deterministic route IDs, paths, schemas | Rebuild comparison of hashes | `conformance/compiler/compiler.test.ts` (determinism PASS) |
| COMP-004 Duplicate/shadow route rejection | Rejection of canonically equivalent routes | `conformance/compiler/compiler.test.ts` (collision PASS) |
| COMP-005 Pack metadata versions & hashes | Integrity verification | `q-pack/src/lib.rs` verify() PASS |
| COMP-006 Unsupported Node/Bun import rejection | AST scan for `node:*`, `bun:*`, `fs`, etc. | `conformance/compiler/compiler.test.ts` (import error PASS) |
| COMP-008 OpenAPI & contract diff emission | Generated OpenAPI 3.1 & semantic diff tool | `packages/compiler/src/emit.ts`, `openapi.json` |
| COMP-009 Reproducible release builds | Bit-identical output on repeated builds | `conformance/compiler/compiler.test.ts` |

## Generated Artifacts

For `examples/proof`, `q build` generates 9 deterministic files in `dist/`:
1. `app.qpack` (40,426 B): executable pack embedding bundle + sourceMap + precompiled route table
2. `route-manifest.json` (3,170 B): route inventory with stages and security posture
3. `schema-manifest.json` (1,487 B): Schema IR v1 registry
4. `capability-manifest.json` (320 B): linked capabilities (timer) and route mappings
5. `contract.json` (3,450 B): compact contract representation
6. `contract.d.ts` (1,908 B): TypeScript published contract type definition
7. `openapi.json` (4,323 B): OpenAPI 3.1.0 specification
8. `contract.lock.json` (3,427 B): baseline for semantic contract diff
9. `build-report.json` / `build-report.md` (6,244 B): human and machine readable build reports

## Build Performance

- Clean build duration for 9-route proof app: **579 ms** (budget ≤ 1,000 ms — PASS)
- Bundling: `Bun.build` with source maps enabled
