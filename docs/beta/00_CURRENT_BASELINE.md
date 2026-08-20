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
Commit:          e2b379d775a79e619753aaf39eb9ea5f8a763f15
Source ZIP:      velqu-m0-m2-20260819T141529Z.zip
Source SHA-256:  e66bd2da0d7e74ae277a819df6d38c453a119413eaf939755ccabc97efbcce41
Git bundle:      velqu-m2.3-r3-e2b379d.bundle
Bundle SHA-256:  a5ba061b422e857e1f8f1411ed5ced90c3148e492a6ed950aa418600e91d3554
Checkpoint name: M2.3-r3 implementation checkpoint
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

The M23R2-GATE-CLOSE work package (implemented on top of e2b379d) closes the review items as follows:

| # | Review requirement | Status |
|---|---|---|
| 1 | Semantic function manifest mandatory in numeric mode | **DONE** — count-only `__velquFunctions` fallback removed; missing manifest rejects startup (`numeric_pack_without_semantic_manifest_is_rejected`) |
| 2 | Router/schema/function execution graph integrity-bound and semantically verified | **DONE** — `routes_canonical_sha256` covers functions + schema manifest + serialized router; tamper tests prove hash sensitivity; router bounds/method-slot validation in `QPack::verify` |
| 3 | No `Router::build` semantic reconstruction at startup | **DONE** — `Router::from_pack` consumes the serialized automaton directly; `Router::build` remains only as the legacy no-router fallback |
| 4 | No duplicate legacy handler table or implicit fallback | **DONE** — numeric packs with `handlerTable` are rejected; compiler/bench generators emit handlerTable-free packs |
| 5 | RouteId/PolicyId/HandlerId/SchemaId drive runtime behavior | **DONE for the request path** — router returns route indexes into `CompiledRoute`; params/query/body admission validates through the `SchemaId`-indexed `schema_vector` (zero string lookups); response-schema checking remains on the optional slow path |
| 6 | Public contract and execution graph hashes independently recomputed and verified | **DONE** — `QPack::verify` recomputes and rejects mismatched `contractHash`; `public_contract_sha256` is stable across function reordering |
| 7 | Repeated randomized benchmark protocol incl. 10,000-route cold evidence and allocation data | **PARTIAL** — clean isolated single-pass evidence at 25/1,000/10,000 routes (0 failures) committed; ≥5 randomized repetitions and allocation profiles remain open |
| 8 | Release/task/evidence indexes and checksums current and self-verifying | **DONE** — `TASKS.production.json` carries machine-readable `evidence_refs`; `REVIEW_INDEX.json` + `EVIDENCE_INDEX.json`; `scripts/release-packet` produces a `release/` directory whose `SHA256SUMS.txt` passes `sha256sum -c` internally |

Verification at this revision: 154 Rust tests + 35 TypeScript tests, all passing; `./scripts/verify` ALL PASS.
