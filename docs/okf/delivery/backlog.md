---
type: Product Backlog
title: Prioritized Project Q Backlog
description: Implementation epics and acceptance-focused work items from evidence
  foundation through M2 and alpha.
tags:
- backlog
- epics
- acceptance
- implementation
status: draft
generated:
  by: openai/gpt-5.6-pro
  at: '2026-08-17T19:27:58+07:00'
---

# Prioritized Backlog

This backlog uses:

```text
P0 — M0–M2 thesis/vertical-slice blocker
P1 — first credible private alpha
P2 — later ecosystem or optimization
```

# Epic E0 — Repository and evidence foundation

| ID | Pri | Item | Acceptance |
|---|---:|---|---|
| E0-01 | P0 | Establish Rust/Bun workspace and pinned tool versions | Clean build on documented environment; lockfiles committed. |
| E0-02 | P0 | Add OKF validator and link checker | Fails malformed frontmatter/reserved files/broken links. |
| E0-03 | P0 | Add one `verify` command | Runs all currently authorized checks. |
| E0-04 | P0 | Add machine-readable environment manifest | Records OS, CPU, tool/dependency versions, flags. |
| E0-05 | P0 | Add reports directory and evidence schema | Raw and summarized results are separated. |

# Epic E1 — Benchmark baselines

| ID | Pri | Item | Acceptance |
|---|---:|---|---|
| E1-01 | P0 | Freeze proof-route semantics | Exact request/response fixtures shared by candidates. |
| E1-02 | P0 | Raw Rust baseline | Correctness tests and release build. |
| E1-03 | P0 | Raw Bun baseline | Same route classes and outputs. |
| E1-04 | P0 | Elysia 2 AOT baseline | Best-practice matched implementation, pinned version. |
| E1-05 | P0 | Parent cold-start harness | Fresh process/sample, p50/p95/p99, failures, raw JSON. |
| E1-06 | P0 | Warm and memory harness | Idle/loaded RSS and matched request tests. |
| E1-07 | P0 | Fairness audit | Detects asymmetric features/config/build modes. |

# Epic E2 — Rust host

| ID | Pri | Item | Acceptance |
|---|---:|---|---|
| E2-01 | P0 | Configuration and fail-closed validation | Invalid production values stop before bind. |
| E2-02 | P0 | HTTP/1.1 listener | Keep-alive and shutdown fixtures pass. |
| E2-03 | P0 | Native route table | Static/param/wildcard/404/405 pass. |
| E2-04 | P0 | Admission limits | Header/body/queue limits pass without unbounded allocation. |
| E2-05 | P0 | Application pack reader v0 | Hash/version/tamper fixtures pass. |
| E2-06 | P0 | Structured startup/request logging | Stable IDs and redaction tests pass. |

# Epic E3 — QuickJS integration

| ID | Pri | Item | Acceptance |
|---|---:|---|---|
| E3-01 | P0 | Engine adapter skeleton | Runtime code does not expose engine-specific API upward. |
| E3-02 | P0 | One worker load and handler cache | Exact handler table verified once. |
| E3-03 | P0 | Sync handler invocation | String/number/object results pass. |
| E3-04 | P0 | Promise/job integration | Deterministic async fixture passes. |
| E3-05 | P0 | Heap/stack/interrupt controls | Limit fixtures fail safely. |
| E3-06 | P0 | Source-map exception | TypeScript source location is useful. |
| E3-07 | P1 | Trusted bytecode experiment | Exact ABI/tamper/source parity evidence. |

# Epic E4 — Native bridge

| ID | Pri | Item | Acceptance |
|---|---:|---|---|
| E4-01 | P0 | Opaque request handle with generation | Expired/wrong-owner access rejected. |
| E4-02 | P0 | Lazy params/header/query access | Unread fields are not materialized. |
| E4-03 | P0 | Bounded body access | Oversize/cancel/read-once behavior specified and tested. |
| E4-04 | P0 | JSON strategy benchmark | A/B/C results include conversions and allocations. |
| E4-05 | P0 | Response strategy benchmark | Correctness and cost for object/string/bytes/problems. |
| E4-06 | P0 | Cancellation propagation | All completion/cancel races pass. |
| E4-07 | P1 | Explicit Web Request/Response fallback | Labeled in manifest and conformance-tested. |

# Epic E5 — Authoring and compiler

| ID | Pri | Item | Acceptance |
|---|---:|---|---|
| E5-01 | P0 | Route/module static declarations | Type tests and extraction fixture pass. |
| E5-02 | P0 | AST/static extractor | Does not execute app/services. |
| E5-03 | P0 | Canonical collision/shadow diagnostics | Source-located failure fixtures pass. |
| E5-04 | P0 | Handler table/bundle emitter | Runtime loads generated output. |
| E5-05 | P0 | Versioned route/pipeline manifest | Stable IDs and deterministic hash. |
| E5-06 | P0 | Unsupported API/import diagnostics | Node/Bun fixtures fail at build. |
| E5-07 | P1 | Incremental development build | Release-manifest parity suite passes. |

# Epic E6 — Schema and contracts

| ID | Pri | Item | Acceptance |
|---|---:|---|---|
| E6-01 | P0 | Core schema IR | Scalar/object/array/optional constraints pass shared fixtures. |
| E6-02 | P0 | Explicit source coercions | Params/query/body semantics differ only as documented. |
| E6-03 | P0 | Validation problem output | Safe paths/codes and typed 422. |
| E6-04 | P0 | Status-specific response typing | Undeclared status fails type/runtime tests. |
| E6-05 | P0 | Policy input/context/error composition | 401 flows into route and Treaty. |
| E6-06 | P1 | OpenAPI projection | Proof app spec validates. |
| E6-07 | P1 | Contract lock and semantic diff | Breaking/compatible fixtures pass. |
| E6-08 | P1 | Third-party adapter interface | Unsupported semantics report fallback/failure. |

# Epic E7 — Treaty

| ID | Pri | Item | Acceptance |
|---|---:|---|---|
| E7-01 | P0 | Object path proxy/builder | Static/param/nested routes pass. |
| E7-02 | P0 | Typed request encoding | Path/query/header/body tests pass. |
| E7-03 | P0 | Status-narrowed result | 200/401/404/422 type tests pass. |
| E7-04 | P0 | Network error model | Abort/network distinct from HTTP failure. |
| E7-05 | P0 | Unit-local dispatcher | Clearly labeled and fast. |
| E7-06 | P0 | Runtime-local integration helper | Spawns real binary and validates contract. |
| E7-07 | P0 | Compact published contract | Client fixture has no server implementation dependency. |
| E7-08 | P1 | Contract hash diagnostics | Mismatch observable without false incompatibility. |

# Epic E8 — Services, policy, and lifecycle

| ID | Pri | Item | Acceptance |
|---|---:|---|---|
| E8-01 | P0 | Lazy application service fixture | Unrelated cold route does not initialize service. |
| E8-02 | P0 | Session-like policy | Typed context and 401. |
| E8-03 | P0 | Deterministic shutdown lifecycle | Initialized service closes once. |
| E8-04 | P1 | Bounded `defer` | Runs post-response, times out, not durable. |
| E8-05 | P1 | Interceptor proof | Depth/cost visible in build report. |
| E8-06 | P1 | Capability manifest and one async capability | Build/startup compatibility and cancellation pass. |

# Epic E9 — Security and hardening

| ID | Pri | Item | Acceptance |
|---|---:|---|---|
| E9-01 | P0 | FFI ownership audit | Written invariant and tests for every handle. |
| E9-02 | P0 | Redaction suite | Secrets absent from logs/errors. |
| E9-03 | P0 | Pack/bytecode trust checks | Modified/mismatched artifact rejected. |
| E9-04 | P0 | Fuzz route/manifest/bridge parsers | Corpus and reproducible command. |
| E9-05 | P1 | Dependency/SBOM/license report | Machine-readable and reviewed. |
| E9-06 | P1 | Outbound fetch SSRF policy seam | Timeouts/cancellation/allow rules testable. |

# Epic E10 — Documentation and handoff

| ID | Pri | Item | Acceptance |
|---|---:|---|---|
| E10-01 | P0 | Keep OKF decisions and evidence current | Status changes link evidence. |
| E10-02 | P0 | Generate build/benchmark/security reports | Raw data and methodology included. |
| E10-03 | P0 | Proof app tutorial | Commands work from clean checkout. |
| E10-04 | P0 | Final M2 review archive | ZIP, SHA-256, commit, exact stop point. |
| E10-05 | P1 | Public docs site | Deferred until naming/repo/license authorization. |
