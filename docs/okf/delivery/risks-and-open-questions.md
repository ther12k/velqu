---
type: Risk Register
title: Risks, Open Questions, and Stop Conditions
description: Technical, product, security, maintenance, and governance risks with
  milestones and explicit stop conditions.
tags:
- risks
- open-questions
- kill-criteria
- governance
status: draft
generated:
  by: openai/gpt-5.6-pro
  at: '2026-08-17T19:27:58+07:00'
---

# Risk register

Likelihood and impact are initial judgments, not measured probabilities.

| ID | Risk | Likelihood | Impact | Current response | Gate |
|---|---|---:|---:|---|---|
| R-001 | Rust–QuickJS conversion erases runtime advantage | High | Critical | Bridge A/B/C microbenchmarks; lazy handles; kill criteria | M1 |
| R-002 | Complete cold start is not materially better than Elysia 2 AOT | Medium | Critical | Matched C3/C4 process harness | M1/M2 |
| R-003 | QuickJS CPU performance is inadequate for common handlers | Medium | High | Representative logic fixtures; document workload boundary | M1 |
| R-004 | Compiler static subset frustrates developers | Medium | High | Small explicit syntax; diagnostics; generated-source escape hatch | M2 |
| R-005 | Compiler accidentally executes application side effects | Medium | Critical | Architecture separation and side-effect traps in tests | M2 |
| R-006 | Source and published Treaty modes diverge | Medium | High | Shared canonical contract and parity fixtures | M2 |
| R-007 | TypeScript inference becomes slow at 1,000 routes | Medium | High | Compact declaration generation and type-check benchmarks | M0/M2 |
| R-008 | Source maps across bundle/QuickJS/native errors are poor | Medium | High | M1 source-map spike before broad compiler work | M1 |
| R-009 | QuickJS limits are mistaken for secure sandboxing | Medium | Critical | Trusted-code positioning; explicit non-goal; future process isolation | Continuous |
| R-010 | Capability surface grows into Node clone | High | High | Capability ADR, size/security gate per addition | Continuous |
| R-011 | Lazy workers/services shift cold cost into tail latency | Medium | High | Separate expansion/lazy-first measurements | M3/M4 |
| R-012 | Bytecode locks runtime/application too tightly | Medium | Medium | Exact version metadata; source mode; pair rollback | M2/M3 |
| R-013 | Native validators differ from Treaty/OpenAPI semantics | Medium | Critical | Shared schema conformance corpus | M2 |
| R-014 | Elysia/Bun baseline is a straw man | Medium | Critical | Best-practice review, pinned official version, fairness audit | M0 |
| R-015 | Optimizing static/native routes distorts product claims | High | High | C3/C4 primary metrics; C0 separately labeled | M0 |
| R-016 | FFI use-after-free or cross-worker access | Medium | Critical | Opaque generations, Rust audit, sanitizer/fuzz tests | M1 |
| R-017 | Native async completion races cancellation/shutdown | High | Critical | Deterministic race suite and bounded operation registry | M1 |
| R-018 | Maintainer burden exceeds adoption value | Medium | High | Narrow M0–M2; no compatibility expansion; stop gate | M2 |
| R-019 | Dependency/license constraints affect distribution | Low/Medium | High | Pin and review early; SBOM/license report | M0/M2 |
| R-020 | Final branding/package names collide | Medium | Medium | Keep Project Q provisional; no public claim | Owner |

# Open technical questions

## OQ-001 — QuickJS-NG or upstream QuickJS?

Decision method:

- same application source;
- same bridge semantics;
- source/bytecode load;
- handler calls;
- async jobs;
- memory;
- maintenance/license assessment.

M1 may proceed with QuickJS-NG while keeping the adapter.

## OQ-002 — What is the simplest application artifact?

Candidates:

- directory pack with source;
- archive with source;
- version-pinned bytecode;
- embedded standalone executable.

Use the directory pack until evidence shows packaging I/O dominates.

## OQ-003 — Which JSON path wins?

No default answer. Select per [Request and Response Bridge](../architecture/request-response-bridge.md).

## OQ-004 — How much schema output validation belongs in production?

Candidates:

- always;
- generated constructor trust;
- development-only;
- per-route strict mode.

Correctness and cost evidence needed.

## OQ-005 — What static extraction technology?

Candidates:

- TypeScript compiler API;
- SWC/Oxc parser;
- generated TypeScript declarations;
- a constrained data file plus handler imports.

Evaluate diagnostics, semantic resolution, build size, contributor burden, and reproducibility.

## OQ-006 — How should development reload work?

Candidates:

- process restart;
- runtime application reload;
- separate dev host.

Release semantics must remain deterministic; stale native handles/module state must not leak.

## OQ-007 — Is `hyper`/Tokio sufficiently small and fast at cold start?

Use as correctness baseline. Profile before considering custom listener/event loop.

## OQ-008 — When should Web Request/Response be supported?

Only after the normal native-backed contract path is stable. It may be P1 if essential package interoperability requires it.

## OQ-009 — What is the minimum Web API surface?

P0 should remain tiny. Every additional global has conformance, security, artifact, startup, and maintenance cost.

## OQ-010 — How are client route-name conflicts escaped?

The Treaty path grammar needs fixtures for reserved names such as `then`, `get`, `post`, `$url`, and dynamic segments.

# Open product/owner decisions

| ID | Decision | Needed by |
|---|---|---|
| OD-001 | final product name | public release preparation |
| OD-002 | package namespace/import path | public package publication |
| OD-003 | public repository and organization | publication |
| OD-004 | open-source license | before external distribution |
| OD-005 | governance/maintainer authority | public alpha |
| OD-006 | supported production platforms | alpha |
| OD-007 | public compatibility promise | alpha |
| OD-008 | public performance messaging | after evidence review |
| OD-009 | whether to pursue serverless platform integrations | post-M2 |
| OD-010 | whether Oracle/native DB adapter is an early ecosystem priority | post-core proof |

# Stop conditions

Implementation must stop and report rather than broaden scope when:

- the bridge cannot be made memory-safe under required semantics;
- complete C3/C4 cold start has no material advantage and no compensating product value;
- M1 empty/small-object overhead fails kill criteria;
- source maps cannot provide usable diagnostics;
- static compilation requires executing arbitrary app setup;
- conformance can pass only by weakening the baseline or tests;
- a public ownership/license/repository decision is required.

A stop is not a failed handoff. It is an evidence-backed project decision.
