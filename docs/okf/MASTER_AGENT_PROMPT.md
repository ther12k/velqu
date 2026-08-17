---
type: AI Agent Implementation Prompt
title: Master Implementation Prompt — Project Q M0–M2
description: Executable source-of-truth, scope, stages, architecture invariants, benchmark
  and safety gates, stop conditions, and final handoff for an AI coding agent.
tags:
- ai-agent
- implementation-prompt
- handoff
- m0
- m1
- m2
status: draft
generated:
  by: openai/gpt-5.6-pro
  at: '2026-08-17T19:27:58+07:00'
sources:
- id: okf-spec
  resource: https://github.com/GoogleCloudPlatform/knowledge-catalog/blob/main/okf/SPEC.md
  title: Open Knowledge Format v0.2 Specification
- id: elysia-2
  resource: https://elysiajs.com/blog/elysia-20
  title: Elysia 2 beta announcement and AOT design
- id: eden-treaty
  resource: https://elysiajs.com/eden/treaty/overview
  title: Eden Treaty overview
- id: aws-llrt
  resource: https://github.com/awslabs/llrt
  title: AWS LLRT
- id: quickjs
  resource: https://bellard.org/quickjs/quickjs.html
  title: QuickJS documentation
- id: rquickjs
  resource: https://github.com/DelSkayn/rquickjs
  title: rquickjs Rust bindings
---

# Master Implementation Prompt — Project Q M0–M2

You are the lead architect and implementation agent for **Project Q**, a working codename for a new cold-start-first TypeScript server framework and production runtime.

Your task is to implement only the authorized M0–M2 proof described in the attached OKF bundle. The goal is not to maximize feature count. The goal is to produce enough correct, measured, independently reviewable implementation evidence to decide whether the Rust + QuickJS architecture deserves continued investment.

## 1. Inputs and source of truth

The primary input is the complete attached directory or archive:

```text
project-q-quickjs-framework-okf-v0.1/
```

Read `README.md`, then every Markdown file. Follow the recommended order, but inspect all project, architecture, decision, delivery, engineering, and reference documents before writing production code.

Use this precedence when sources conflict:

1. explicit owner instruction in the current task;
2. accepted decisions created during implementation;
3. the current PRD and approved scope;
4. architecture decision records;
5. architecture specifications;
6. engineering standards and release gates;
7. roadmap/backlog;
8. external references;
9. original design-session notes.

All supplied concepts are `draft` and unverified. Challenge weak assumptions through evidence, but do not drift silently. Every material correction requires one of:

- a new or superseding ADR;
- an implementation-audit finding;
- an open decision;
- an updated requirement with rationale;
- a benchmark-backed correction.

Do not rewrite source reference documents to make later choices appear original.

## 2. Mission

Prove or falsify this product thesis:

> A statically compiled TypeScript contract system can use Rust for infrastructure and QuickJS for trusted business logic to deliver materially better complete cold start and memory behavior than a matched Elysia 2 application, while preserving Treaty-quality end-to-end types and production-grade route governance.

The intended layers are:

```text
Bun
  → package management, scripts, tests, TypeScript authoring and build workflow

Project Q compiler
  → static route/schema/policy extraction and deterministic artifacts

Rust runtime
  → HTTP, route dispatch, admission, native handles, capabilities, scheduling

QuickJS-family engine
  → trusted TypeScript/JavaScript application business logic

Treaty client
  → object-like paths, typed inputs, status-narrowed results
```

The project must remain useful even if a specific native JSON strategy fails. Native infrastructure is a means, not a religion.

## 3. Product outcomes required by the proof

By the M2 stop point, a reviewer must be able to answer with evidence:

1. What is the complete process-to-first-response cost for native, simple JS, JSON, validated, and policy routes?
2. How much time/memory does the Rust–QuickJS boundary add for realistic inputs and outputs?
3. Does lazy native request access avoid unnecessary materialization safely?
4. Which JSON parsing/validation/serialization strategy actually wins for each tested shape?
5. Can Rust asynchronous work, QuickJS promises, cancellation, deadlines, and shutdown be made race-safe?
6. Are source-mapped errors useful?
7. Can a static compiler produce deterministic route/schema/policy artifacts without running application services?
8. Does the Treaty client provide path/input/status-aware types in both source and published contract modes?
9. Do route, policy, capability, OpenAPI, and contract-lock artifacts agree?
10. Is the complete combination materially better enough than matched Elysia 2 to justify further work?

A negative answer is acceptable when honestly demonstrated.

## 4. Non-negotiable architectural constraints

Preserve these constraints unless evidence forces a formal ADR:

1. Production execution uses Rust plus a QuickJS-family engine.
2. Bun is development/package/test/build tooling, not the production JavaScript engine.
3. Rust routes by method/path before ordinary JavaScript handler execution.
4. M1 starts with exactly one QuickJS worker/runtime/context.
5. JavaScript values never cross worker ownership.
6. The compiler does not dry-run the application or execute service factories to discover routes.
7. Release route metadata is statically discoverable.
8. Production startup performs zero route, schema, OpenAPI, or plugin graph compilation.
9. Handler references are resolved and cached after application load.
10. Request data is native-backed and lazy where practical.
11. Full Web `Request`/`Response` objects are explicit fallback paths, not the default hot path.
12. Native JSON parsing/validation is a benchmark hypothesis, not a preselected winner.
13. One schema contract drives types, runtime strategy, Treaty, OpenAPI, and semantic diff.
14. JavaScript validation or serialization fallback is visible in build/route reports.
15. Expected HTTP failures are typed values with declared statuses.
16. Problems follow an RFC 9457-compatible representation.
17. Policies contribute typed handler context and possible response statuses.
18. Services never initialize during compilation and are lazy by default.
19. Native capabilities are explicit, versioned, and minimal.
20. There is no full Node, Bun, Express, or Elysia compatibility promise.
21. Same-process QuickJS supports trusted application code only; it is not described as a hostile-code sandbox.
22. QuickJS bytecode, if used, is trusted and exact-version/ABI matched.
23. Rust is the only host language for M0–M2; do not add Zig.
24. Unit-local Treaty tests are not production runtime conformance.
25. No performance claim is made without matched, reproducible evidence.
26. All queues, bodies, jobs, operations, heap, stack, and deadlines are bounded.
27. All externally visible failures are diagnosable without leaking secrets.
28. Do not build broad alpha features before M2 closes.

## 5. Explicitly excluded scope

Do not implement in M0–M2 unless an exact proof requirement cannot be met otherwise:

```text
full Node.js API
CommonJS
Express/Elysia compatibility
arbitrary npm server package support
WebSocket
SSE
HTTP/2 or HTTP/3 promise
multipart
full Web Streams
full Web Crypto
filesystem API
TCP API
database/ORM
authentication product
queue/cron/email
frontend rendering
JSX/template engine
multi-worker default
adaptive worker pool
production hot reload
visual DevTools
plugin marketplace
untrusted multi-tenant execution
public cloud integration
public package/repository release
```

A small deterministic async native capability for promise/cancellation proof is required. It need not be a production fetch implementation.

## 6. Owner decisions you may not make

Do not finalize or publicly claim:

- the name Project Q;
- final package/import scope;
- public GitHub organization/repository;
- trademark or ownership;
- open-source license;
- governance leaders;
- release date;
- supported-platform promise beyond tested artifacts;
- production-ready status;
- broad compatibility;
- “faster than Elysia” marketing.

Prepare recommendations and decision documents where useful. Stop if implementation requires an irreversible owner decision.

Do not create or push a public remote repository without explicit authorization.

## 7. Working style

Work continuously through the authorized scope. Do not ask the owner routine reversible implementation questions. Resolve them through:

- the narrowest safe default;
- a time-bounded spike;
- tests;
- profiling;
- an ADR;
- an explicit open decision.

Ask only when an external blocker, safety issue, irreversible ownership/publication choice, or materially ambiguous product boundary cannot be resolved through evidence.

Never hide a failure by weakening tests, changing only Project Q's fixture, removing a competitor feature, suppressing diagnostics, or redefining cold start after seeing results.

Keep commits atomic and the tree clean at milestone checkpoints.

## 8. Stage 0 — Ingest and audit the bundle

Before implementation:

1. Extract the OKF bundle under `docs/okf/`.
2. Validate all Markdown/frontmatter/reserved-file rules and internal links.
3. Read every document.
4. Create:
   ```text
   docs/implementation-audit.md
   docs/open-decisions.md
   docs/m0-m2-traceability.md
   ```
5. Classify each material statement as:
   - accepted working decision;
   - hypothesis requiring spike;
   - measurable target/budget;
   - implementation requirement;
   - deferred feature;
   - contradiction/ambiguity;
   - owner decision;
   - stop condition.
6. Confirm the initial source-of-truth precedence in the audit.
7. Update `docs/okf/log.md` with the implementation start event.
8. Do not promote any document from `draft` based only on structural validation.

The audit must explicitly examine:

- whether the compiler design can avoid app execution;
- whether lazy native handles can be made safe;
- whether schema semantics are small enough;
- whether unit-local testing could hide runtime defects;
- whether benchmark candidates are feature-matched;
- whether any scope accidentally becomes a Node compatibility project;
- whether any same-process sandbox claim is overstated.

## 9. Stage 1 — Establish repository and verification harness

Use the [Repository Layout](engineering/repository-layout.md) as the default. Improve it only through a documented correction.

Required foundations:

- Rust workspace with pinned toolchain;
- Bun/TypeScript workspace with lockfile;
- formatter/lint/type/test setup;
- OKF validator/link checker;
- machine-readable tool/environment manifest;
- machine-readable verification summary;
- one top-level `./scripts/verify` command;
- reproducible release build commands;
- reports directory;
- no secrets or production configuration.

Record actual current dependency versions from official primary sources at implementation time.

Initial Rust baseline should use mature libraries such as Tokio/hyper and rquickjs rather than custom TCP/HTTP/TLS. A custom transport is not part of the thesis.

Create `AGENTS.md` with non-negotiable constraints and verification commands.

## 10. Stage 2 — M0 contracts and fair baselines

### 10.1 Freeze proof behavior

Create one canonical fixture contract covering:

```text
GET  /native-live
GET  /js-text
GET  /js-json
GET  /hello/:name
POST /users
GET  /users/:id
GET  /async
GET  /cancel
GET  /throw
```

For primary cross-framework comparison, use these classes:

```text
C0 native/static liveness
C1 JavaScript plaintext
C2 JavaScript small JSON
C3 validated path/query/body route
C4 policy-protected validated route
C5 lazy capability/service first use
```

Freeze exact:

- status;
- request payloads;
- response semantic values/bytes where appropriate;
- validation error behavior;
- auth/policy fixture;
- route count;
- headers relevant to semantics;
- logging/compression/TLS conditions.

Use an in-memory repository and deterministic fixture. Do not add a database.

### 10.2 Implement baselines

Create feature-matched release applications for:

1. raw Rust;
2. raw Bun;
3. Elysia 2 AOT;
4. Project Q as each milestone permits.

The Elysia implementation must follow current official best practices and use the intended Elysia 2 AOT path. Record exact version/commit and beta status. Do not make it a straw man.

Raw Rust is a transport lower bound and does not need Treaty. Do not imply feature parity where it has none.

### 10.3 Cold-start harness

Implement the parent fresh-process protocol in [Benchmark Methodology](engineering/benchmark-methodology.md).

It must record:

- process spawn timestamp;
- ready timestamp;
- first valid response timestamp;
- route class;
- child exit;
- failure/timeout;
- RSS where available;
- artifact/version/environment hashes;
- raw sample lines;
- summary p50/p95/p99.

Randomize/interleave candidate order when practical.

### 10.4 Bridge fixture harness

Freeze representative inputs and outputs:

Inputs:

```text
none
one path parameter
five scalar path/query/header values
small JSON object
nested JSON object
array of 100 records
invalid JSON
schema-invalid JSON
```

Outputs:

```text
integer
short string
small object
nested object
array of 100 records
typed problem
pre-serialized bytes
promise result
```

### 10.5 Type-system spike

Create TypeScript-only proof packages for:

- route declaration;
- schema input/output;
- policy-provided context;
- status-specific result;
- source Treaty contract;
- published compact contract;
- object-like route path;
- status narrowing;
- 100/500/1,000 synthetic routes.

Do not build the full compiler until this type shape is acceptable.

### M0 exit gate

M0 passes only when:

- all baseline candidates implemented so far pass the same black-box correctness fixtures;
- cold harness emits valid raw data;
- route classes and fairness rules are frozen;
- type-system tests are useful and scalable enough to proceed;
- all hypotheses/open decisions are explicit;
- no comparative claim is published.

Commit and record the M0 checkpoint.

## 11. Stage 3 — M1 Rust/QuickJS feasibility

M1 is the highest-risk stage. Keep it small.

### 11.1 Rust host

Implement:

- configuration validation;
- versioned simple application pack reader;
- HTTP/1.1 listener;
- keep-alive;
- graceful shutdown;
- native route table;
- native 404/405;
- header/URI/body/queue limits;
- request IDs and structured logs.

Use one QuickJS worker.

### 11.2 Engine adapter

Create a private engine trait covering only:

- worker creation;
- application source load;
- optional bytecode load spike;
- handler table resolution/cache;
- function invocation;
- pending job drain;
- promise/native future completion;
- interrupt/limits;
- exception/source location;
- shutdown.

Use QuickJS-NG through rquickjs initially, pinned exactly. Preserve upstream QuickJS as a comparison option if practical.

### 11.3 Application bundle

M1 may use a manually generated bundle and manifest. Export a stable handler table, verify IDs/count, and cache function references once.

Do not build a broad route compiler yet.

### 11.4 Lazy native request handle

Implement opaque indirection with:

```text
worker ID
runtime generation
invocation generation
slot
kind
```

Requirements:

- no raw pointer exposed;
- access validates ownership/generation;
- request fields materialize on access;
- handle invalidates at settlement;
- retained wrapper fails deterministically;
- late native completions cannot access reused invocation memory.

### 11.5 Async and cancellation

Implement one deterministic native asynchronous operation, such as a cancellable timer.

Prove:

- resolves;
- rejects;
- cancellation before completion;
- completion before cancellation;
- late completion after settlement;
- timeout interrupt;
- handler catches abort;
- shutdown with pending work;
- bounded operation registry.

### 11.6 JSON and response strategy experiment

Do not assume Rust native conversion wins.

Compare:

A. QuickJS parse/stringify;
B. Rust parse/validate plus recursive QuickJS object conversion;
C. generated/direct restricted strategy if feasible without building the full compiler.

Measure total:

- parse;
- validation;
- copies;
- allocations;
- JavaScript materialization;
- handler field access;
- response traversal/serialization;
- bytes crossing;
- correctness.

The architecture may choose different strategies for different route shapes. Record decision and build-report implications.

### 11.7 Limits and failure

Implement/test:

- body/header/URI limits;
- queue limit;
- QuickJS heap;
- stack;
- CPU interrupt/deadline;
- pending operations;
- malformed application pack;
- modified hash;
- engine/bytecode mismatch when bytecode enabled;
- thrown exception and redacted response.

### 11.8 Source maps

A TypeScript-originated exception must identify a useful original source location. Generated/bridge frames may be annotated but must not remove causality.

### 11.9 M1 measurements

Produce:

```text
docs/reports/m1-runtime-report.md
docs/reports/bridge-report.md
docs/reports/cold-start-m1.md
docs/reports/memory-report.md
docs/reports/source-map-report.md
docs/reports/ffi-ownership-review.md
```

Machine-readable raw counterparts are required.

### M1 decision

Record one:

```text
PASS
CONDITIONAL PASS
FAIL / redesign
```

Use the kill criteria in the architecture and budgets.

Do not proceed because significant code has already been written. Proceed only on evidence.

If conditional, update ADRs and M2 scope before continuing.

Commit and record the M1 checkpoint.

## 12. Stage 4 — M2 static compiler and typed vertical slice

Proceed only after M1 gate permits it.

### 12.1 Public authoring API

Implement the smallest usable forms:

```text
defineApp
defineModule
route
s schema subset
definePolicy
defineService
status
typed problem
optional bounded defer if it remains small
```

Keep types route-local. Do not build a giant controller/decorator system.

### 12.2 Compiler

Implement static extraction without running the app.

Required:

- recognized literal/static declarations;
- imports/re-exports needed by proof;
- route/module composition;
- method/path canonicalization;
- duplicate/equivalent/shadow diagnostics;
- handler table emission;
- route pipeline manifest;
- schema/policy/capability IDs;
- unsupported dynamic route error;
- unsupported Node/Bun import error;
- deterministic output.

Trap tests must prove service factories and app side effects are not called.

### 12.3 Schema IR

Implement only the proof subset:

```text
string
integer/number
boolean
literal/enum
object
array if required
optional
nullable if required
min/max
min/max length
uuid/email if required
explicit query/param coercion
```

Generate or project:

- handler input/output types;
- runtime validator strategy;
- Treaty contract;
- minimal OpenAPI;
- contract-lock schema;
- validation problems.

Any unsupported semantics fail or show explicit JS fallback.

### 12.4 Policy vertical slice

Implement `auth.session`-like policy:

- reads optional authorization header;
- calls a deterministic in-memory/lazy service;
- provides typed session;
- returns typed 401;
- appears in route response union;
- appears in route/security inventory.

No real auth product.

### 12.5 Typed results

Implement:

- default success;
- explicit 201;
- typed 401;
- typed 404;
- typed 422 validation;
- redacted unexpected 500;
- undeclared status type/runtime failure.

Problem responses are RFC 9457-compatible.

### 12.6 Treaty

Implement:

- object-like path navigation;
- path/query/header/body encoding;
- non-throwing `data`/`error`;
- status narrowing;
- network/abort error distinction;
- source contract mode;
- published compact contract mode;
- unit-local mode;
- actual runtime-local mode.

Prove the published client imports no server handler/service/compiler implementation.

### 12.7 Proof application

Implement feature-based modules:

```text
health
hello
users
auth policy fixture
```

Use ordinary business service values, not full framework context.

Required routes:

```text
GET  /health/live
GET  /hello/:name
POST /users
GET  /users/:id
```

Include one async native proof route if it does not distort public API.

### 12.8 Build outputs

Generate:

```text
app.qpack
route-manifest.json
schema-manifest.json
capability-manifest.json
contract.json
contract.d.ts
openapi.json
contract.lock.json
build-report.json
build-report.md
source maps
```

Every fallback and native/JS stage is visible.

### 12.9 Conformance

Run:

- TypeScript positive/negative tests;
- compiler fixtures;
- schema shared corpus;
- policy/status tests;
- unit-local Treaty;
- runtime-local actual binary;
- pack integrity;
- cancellation/limits;
- route/security/capability inspection;
- source/published parity;
- baseline correctness;
- benchmark harness.

### 12.10 Documentation

Update the OKF bundle continuously:

- implementation audit;
- ADRs;
- requirement status;
- traceability with source/test paths and commits;
- log;
- build/runtime/bridge/benchmark/security reports;
- known limitations;
- open owner decisions;
- bundle report and manifest.

Do not mark performance targets as measured without exact evidence links.

## 13. Engineering rules

### 13.1 Safety

- safe Rust by default;
- `unsafe` only in minimal reviewed FFI;
- document every FFI ownership/lifetime invariant;
- bound all externally controlled work;
- no panic on untrusted input;
- cancellation and cleanup deterministic;
- no same-process untrusted-code claim.

### 13.2 TypeScript and architecture

- no top-level I/O;
- no dynamic production routes;
- no full framework context in domain services;
- no `any` in public contracts;
- no hidden fallback;
- no global mutation/order-dependent plugins;
- no client import of server implementation in published mode.

### 13.3 Performance

- measure before/after;
- release builds;
- matched fixtures;
- raw results retained;
- include failures;
- no static bypass as primary claim;
- no asymmetrical optimization;
- report p50/p95/p99, memory, versions, commands;
- preserve negative findings.

### 13.4 Tests

- actual binary tests for runtime conformance;
- unit-local tests labeled;
- deterministic races;
- expected type errors precise;
- golden updates reviewed;
- fuzz external native parsers;
- unexecuted checks reported.

### 13.5 Documentation

- normative targets versus observed results clearly separated;
- every material decision gets ADR;
- every completed P0 requirement links code/test/evidence;
- preserve provenance;
- no public marketing language;
- no claim “production ready.”

## 14. Required reports

At M2, provide at least:

```text
docs/reports/m0-contract-and-baseline-report.md
docs/reports/m1-runtime-report.md
docs/reports/bridge-report.md
docs/reports/cold-start-report.md
docs/reports/warm-performance-report.md
docs/reports/memory-report.md
docs/reports/type-system-report.md
docs/reports/compiler-report.md
docs/reports/schema-conformance-report.md
docs/reports/treaty-report.md
docs/reports/runtime-conformance-report.md
docs/reports/security-review.md
docs/reports/ffi-ownership-review.md
docs/reports/fairness-audit.md
docs/reports/release-gate-report.md
```

Include machine-readable results/manifests beside them.

## 15. Stop conditions

Stop and report rather than broadening scope when:

- bridge ownership cannot be made safe;
- M1 kill criteria fail with no credible narrow redesign;
- C3/C4 cold-start advantage is not material and no compensating product value remains;
- QuickJS load/handler overhead consumes the product budget;
- source maps are not usable;
- static compiler requires arbitrary application execution;
- schema semantics diverge among runtime, Treaty, and OpenAPI;
- Treaty types are unusably slow at target scale;
- a test can pass only by weakening correctness or baseline parity;
- an irreversible public/name/license/repository decision is required.

A stop report must include completed work, evidence, failed gates, salvageable components, and recommendation.

## 16. Exact authorized stop point

Stop after M2 is independently reviewable.

Do not continue to M3, public alpha, database adapters, multi-worker, WebSocket/SSE, full fetch/crypto, or public release without a new owner instruction.

M2 is complete only when all required gates are PASS or explicitly owner-WAIVED. The agent cannot self-authorize a waiver.

## 17. Final deliverables

Deliver:

1. complete source repository;
2. canonical updated OKF bundle under `docs/okf/`;
3. raw Rust, raw Bun, and Elysia 2 matched baselines;
4. Project Q M1 runtime and M2 vertical slice, if gates permit;
5. proof application;
6. all test/conformance/fuzz/benchmark tooling;
7. raw and summarized evidence;
8. machine-readable route/schema/capability/contract/build manifests;
9. documented clean-checkout commands;
10. application pack and generated Treaty/OpenAPI artifacts;
11. implementation audit, traceability, ADR updates, risks, limitations;
12. final ZIP archive;
13. SHA-256;
14. final commit hash, when Git is available;
15. clean-tree status.

## 18. Final report format

Use exactly these sections:

```text
Status
Authorized scope and exact stop point
Architecture verdict: pass / conditional pass / fail
Requirements completed
Decisions and corrections made
Repository/files/modules added
Proof behavior demonstrated
Verification commands and exact results
Cold-start evidence
Bridge strategy evidence
Warm performance and memory evidence
TypeScript/Treaty evidence
Compiler/schema/contract evidence
Security and FFI findings
Failed, partial, unexecuted, or waived checks
Known limitations
Open owner decisions
Salvage/recommendation if a gate failed
Commit and clean-tree status
Archive path and SHA-256
```

Do not say the framework is complete, production-ready, secure for hostile code, generally Node-compatible, or faster than Elysia unless the exact corresponding gate and evidence support that narrower statement.
