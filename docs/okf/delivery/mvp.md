---
type: Delivery Plan
title: MVP and Feasibility Milestones
description: Evidence-gated M0–M2 sequence, exact implementation boundary, tests,
  pass/fail outcomes, and stop point.
tags:
- mvp
- milestones
- feasibility
- runtime
- compiler
status: draft
generated:
  by: openai/gpt-5.6-pro
  at: '2026-08-17T19:27:58+07:00'
---

# Minimum Viable Proof

The MVP is not “a small production framework.” It is the smallest sequence that can falsify or support the product thesis.

# Scope boundary

Authorized initial scope:

```text
M0 — contracts, baseline, and evidence harness
M1 — Rust/QuickJS runtime and bridge feasibility
M2 — compiled typed vertical slice with Treaty
STOP
```

Do not begin broad alpha features before the M2 review.

# M0 — Contracts and evidence baseline

## Objective

Freeze observable behavior, public hypotheses, benchmark methodology, and traceability before implementation optimizations.

## Deliverables

```text
docs/implementation-audit.md
docs/open-decisions.md
docs/m0-m2-traceability.md
benchmarks/manifest.json
benchmarks/fixtures/
examples/proof-contract/
packages/core-type-spike/
packages/treaty-type-spike/
```

## Work

1. Validate the OKF bundle and internal links.
2. Classify every architecture statement as decision, hypothesis, budget, deferred feature, risk, or owner decision.
3. Pin current official tool/framework versions used by the actual implementation.
4. Define identical proof-route semantics.
5. Implement or specify raw Rust, raw Bun, and Elysia 2 AOT baselines.
6. Freeze cold-start parent/child measurement protocol.
7. Freeze bridge input/output fixture shapes.
8. Build TypeScript-only sketches for route, policy, schema, and Treaty inference.
9. Measure TypeScript check behavior at growing synthetic route counts.
10. Create machine-readable raw-result schemas.

## Exit criteria

- all benchmark fixtures validate exact outputs;
- source and published Treaty sketches produce equivalent type tests;
- process-to-ready/first-response definitions are executable;
- Elysia/Bun baselines are fair and pinned;
- no framework performance claim exists;
- M1 bridge choices and kill criteria are explicit.

# M1 — Runtime and bridge feasibility

## Objective

Prove the smallest real Rust + QuickJS path before building a compiler ecosystem.

## Required implementation

```text
runtime/
  config
  pack reader v0
  HTTP/1.1 listener
  minimal native router
  one QuickJS worker
  handler table load/cache
  lazy request handle
  text/JSON result mapping
  timer or deterministic async native operation
  cancellation/deadline
  structured diagnostics
```

Application source can initially be a hand-built bundle and manifest. The full compiler is not required yet.

## Required routes

```text
GET /native-live          native static response
GET /js-text              cached JS string
GET /js-json              cached JS small object
GET /params/:id           path access
POST /json                body parse/echo/validation strategy fixture
GET /async                native promise completion
GET /cancel               cancellation fixture
GET /throw                source-mapped exception fixture
```

## Required strategy comparisons

- QuickJS source versus trusted bytecode load, if bytecode is feasible;
- QuickJS parse/stringify versus Rust conversion for representative JSON;
- native-backed property access versus eager object construction;
- one handler call versus policy + handler call;
- sync and async handler paths.

## Safety tests

- expired handle access;
- wrong generation;
- oversized body/header;
- queue saturation;
- CPU deadline interrupt;
- native completion after cancellation;
- shutdown with active and queued work;
- modified/incompatible pack rejection.

## Exit outcomes

### Pass

Proceed when complete cold-start/memory/bridge evidence shows the architecture can plausibly meet product targets and safety/conformance checks pass.

### Conditional pass

Proceed only with a recorded redesign, such as keeping JSON parsing in QuickJS or narrowing native capabilities.

### Fail

Stop or change the engine/runtime thesis when bridge or application load overhead removes the material advantage, source maps are unusable, or ownership cannot be made safe.

A fail is a valid M1 completion.

# M2 — Compiled typed vertical slice

## Objective

Join the runtime proof with the smallest credible Elysia-inspired authoring and Treaty experience.

## Required packages/components

```text
packages/core
packages/schema
packages/treaty
compiler
runtime
examples/proof
benchmarks
conformance
docs/reports
```

## Required authoring features

- `defineApp`;
- `defineModule`;
- `route`;
- minimal `s` schema;
- `definePolicy`;
- `defineService`;
- `status` and typed problem;
- bounded `defer`, if it does not distract from core gates.

## Required compiler features

- static declarations;
- route canonicalization;
- duplicate/shadow checks;
- schema IR for proof shapes;
- policy graph;
- handler table;
- application pack;
- build report;
- compact Treaty contract;
- minimal OpenAPI;
- contract lock.

## Required proof application

- native liveness;
- validated hello;
- create user;
- policy-protected get user;
- typed 401/404/422;
- in-memory repository;
- one async native operation;
- source and published Treaty tests;
- actual runtime integration tests.

## M2 exit criteria

- one clean command runs format, lint, type checks, Rust tests, compiler tests, unit-local Treaty tests, actual runtime tests, conformance, pack verification, and OKF validation;
- proof artifacts are deterministic;
- no service runs during compilation;
- route/security/capability inventory is accurate;
- source/published Treaty parity passes;
- matched cold-start comparison is reproducible;
- known limitations are explicit;
- all M0–M2 P0 traceability closes or records an authorized waiver;
- final archive and SHA-256 are generated;
- exact stop point is reported.

# Excluded until after M2

```text
WebSocket
SSE
HTTP/2/3 promise
multipart
full Web Streams
full Web Crypto
filesystem
database adapters
authentication product
multi-worker default
adaptive worker pool
hot production reload
full Standard Schema compatibility
code marketplace
visual devtools
frontend framework
server-side templates
Node compatibility
```

# Quality rule

A narrower M2 with strong evidence is better than a broad framework whose central bridge and cold-start claims remain untested.
