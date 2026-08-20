---
type: Baseline
title: Velqu Current State and Reviewed Baseline
status: draft
tags:
- baseline
- m2.3
- review

---

# Current State and Reviewed Baseline

## Authoritative checkpoint

```text
Commit:          4e6904951729ea14b48ca39a9564a950cc83e98e
Source ZIP:      commit-named release artifact (generated only after a clean candidate commit)
Source SHA-256:  recorded in the candidate release packet
Git bundle:      commit-named release artifact (generated only after a clean candidate commit)
Bundle SHA-256:  recorded in the candidate release packet
Checkpoint name: M23R2-GATE-CLOSE implementation checkpoint
```

## What is already proven

- Rust-hosted QuickJS-NG runtime and TypeScript compilation pipeline.
- Millisecond-scale local process cold start in the pinned fixture.
- QuickJS module bytecode embedding.
- Bounded scheduler, owner-scoped microtasks, deadlines, cancellation, task accounting, quarantine, and readiness.
- Native routing and schema-runtime foundations.
- Typed route contracts, Treaty foundations, OpenAPI/contract-lock direction.
- Dense numeric function dispatch and semantic route/policy function metadata on the primary generated path.
- In-memory method-aware terminal router with route-specific parameter names.
- Strong Rust/TypeScript conformance coverage and reproducible Git/source checkpoints.

## What the latest review did not accept as closed

The M23 production gate remains open until all of the following are proven in the exact source checkpoint:

1. semantic function manifest is mandatory in current numeric mode;
2. router/schema/function execution graph is integrity-bound and semantically verified;
3. current numeric startup performs no `Router::build` semantic reconstruction;
4. numeric packs contain no duplicate legacy handler table or implicit fallback;
5. RouteId, PolicyId, HandlerId, and SchemaId drive runtime behavior rather than string identities;
6. public contract and execution graph hashes are independently recomputed and verified;
7. benchmark evidence meets the repeated randomized protocol, including 10,000-route cold evidence and allocation data;
8. release/task/evidence indexes and checksums are current and self-verifying.

## Current maturity

```text
Runtime research thesis:          substantially proven
Single-worker scheduler:          strong
Numeric runtime implementation:   advanced, gate still open
Developer product:                incomplete
Real I/O/capabilities:            incomplete
Multi-worker service mode:        incomplete
Public beta readiness:            not yet
Production-ready GA:              explicitly out of scope for this plan
```

## Baseline rule

Do not reimplement already proven scheduler work without a reproduced regression. G0 is a finite artifact/router/evidence closure, not permission to reopen unrelated architecture.

---

## Status addendum (2026-08-20, M23R2-GATE-CLOSE revision)

The M23R2-GATE-CLOSE work package (implemented on top of the 4e69049 reviewed checkpoint) closes the code items as follows; evidence-only blockers remain open until the frozen benchmark and release protocol passes:

| # | Review requirement | Status |
|---|---|---|
| 1 | Semantic function manifest mandatory in numeric mode | **DONE** — count-only `__velquFunctions` fallback removed; missing manifest rejects startup (`numeric_pack_without_semantic_manifest_is_rejected`) |
| 2 | Router/schema/function execution graph integrity-bound and semantically verified | **DONE** — `routes_canonical_sha256` covers functions + schema manifest + serialized router; tamper tests prove hash sensitivity; router bounds/method-slot validation in `QPack::verify` |
| 3 | No `Router::build` semantic reconstruction at startup | **DONE** — `Router::from_pack` consumes the serialized automaton directly; `Router::build` remains only as the legacy no-router fallback |
| 4 | No duplicate legacy handler table or implicit fallback | **DONE** — numeric packs with `handlerTable` are rejected; compiler/bench generators emit handlerTable-free packs |
| 5 | RouteId/PolicyId/HandlerId/SchemaId drive runtime behavior | **DONE for the request path** — router returns route indexes into `CompiledRoute`; params/query/body admission validates through the `SchemaId`-indexed `schema_vector` (zero string lookups); response-schema checking remains on the optional slow path |
| 6 | Public contract and execution graph hashes independently recomputed and verified | **DONE** — `QPack::verify` recomputes and rejects mismatched `contractHash`; `public_contract_sha256` is stable across function reordering |
| 7 | Repeated randomized benchmark protocol incl. 10,000-route cold evidence and allocation data | **PARTIAL** — five randomized warm repetitions, five-sample route-count evidence, and Velqu-only cold evidence are captured; allocator counters remain unavailable because the host denies perf events, so no allocation count is claimed |
| 8 | Release/task/evidence indexes and checksums current and self-verifying | **IN_PROGRESS** — current indexes and report parity are reconciled; final clean candidate commit and packet-local checksum generation remain required |

Verification at this revision: 154 Rust tests + 35 TypeScript tests, all passing; `./scripts/verify` ALL PASS.
