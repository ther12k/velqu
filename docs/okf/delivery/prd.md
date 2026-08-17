---
type: Product Requirements Document
title: Project Q Product Requirements Document
description: Complete product definition, user experience, functional and non-functional
  requirements, proof application, metrics, and delivery acceptance.
tags:
- prd
- product
- quickjs
- rust
- treaty
- cold-start
status: draft
generated:
  by: openai/gpt-5.6-pro
  at: '2026-08-17T19:27:58+07:00'
sources:
- id: elysia-2
  resource: https://elysiajs.com/blog/elysia-20
  title: Elysia 2 beta announcement and AOT design
- id: eden-treaty
  resource: https://elysiajs.com/eden/treaty/overview
  title: Eden Treaty overview
- id: elysia-best-practice
  resource: https://elysiajs.com/essential/best-practice
  title: Elysia best-practice guide
- id: aws-llrt
  resource: https://github.com/awslabs/llrt
  title: AWS LLRT
- id: quickjs
  resource: https://bellard.org/quickjs/quickjs.html
  title: QuickJS documentation
- id: rfc-9457
  resource: https://www.rfc-editor.org/info/rfc9457/
  title: RFC 9457 Problem Details for HTTP APIs
---

# Product Requirements Document — Project Q

## 1. Document purpose

This PRD defines the product to be proven and built: a cold-start-first TypeScript backend framework with a Rust production host, QuickJS-family application engine, Elysia-inspired contract discipline, and Treaty-style type-safe client.

This document is normative for product scope but remains `draft` and unverified until the feasibility milestones close. It intentionally separates:

- desired product behavior;
- architectural working decisions;
- measured evidence;
- future hypotheses.

## 2. Executive summary

Project Q aims to let an application developer write ordinary TypeScript route and business logic while receiving:

- complete cold starts materially faster than a matched Bun/Elysia 2 application;
- a small, bounded production runtime;
- schema-derived request, response, OpenAPI, and client types;
- object-like Treaty client calls with status-narrowed failures;
- deterministic AOT route/policy/schema manifests;
- explicit native capability and compatibility boundaries;
- inspectable per-route execution and security metadata.

Production execution is not Bun. Bun remains the preferred authoring, package, test, and build toolchain. The production artifact is a Rust runtime embedding a QuickJS-family engine.

The first product is not a Node.js replacement and does not attempt to execute Express, Elysia, or arbitrary npm server applications.

## 3. Problem statement

TypeScript backend developers often trade among:

- rich framework ergonomics;
- end-to-end type inference;
- compatibility with a large runtime ecosystem;
- process startup and idle memory;
- deterministic deployment artifacts;
- operational visibility into route/security contracts.

Elysia and Eden Treaty demonstrate strong schema-first and client inference ergonomics. A general Bun runtime, however, carries broader runtime capabilities than a narrowly compiled application needs. A QuickJS-family interpreter can be compact, but naïvely placing it behind Rust may introduce bridge costs that remove the benefit.

Project Q solves this only if it can combine:

```text
small startup/runtime surface
+ static application knowledge
+ typed framework experience
+ acceptable realistic route performance
```

The product is invalid if it achieves low startup only by omitting validation, policy, client typing, source maps, or production correctness.

## 4. Product thesis

> A statically compiled TypeScript contract system can use Rust for infrastructure and QuickJS for trusted business logic to deliver materially better complete cold start and memory behavior than a matched Elysia 2 application, while preserving Treaty-quality end-to-end types and production-grade route governance.

This is a hypothesis, not a claim.

## 5. Goals

### G-001 — Complete cold start

Optimize and measure process-to-first-valid-response for validated and policy-protected routes.

### G-002 — TypeScript-first business development

Ordinary application developers should not need Rust for routes, policies, or domain services.

### G-003 — Treaty-quality end-to-end typing

Client path navigation, request inputs, success values, and status-specific failures derive from the same server contract.

### G-004 — Deterministic release artifacts

Production startup performs no route discovery, schema compilation, plugin graph construction, OpenAPI generation, or TypeScript transpilation.

### G-005 — Native-by-need, not native-by-dogma

Rust owns transport, routing, limits, and capabilities. Parsing, validation, and serialization strategies are selected from measured end-to-end cost.

### G-006 — Small explicit compatibility surface

Unsupported APIs fail during build. Applications pay only for linked capabilities.

### G-007 — Evidence-driven open-source foundation

Benchmarks, conformance, limitations, and negative results are reproducible and visible.

## 6. Non-goals for the first public alpha

- full Node.js or Bun compatibility;
- CommonJS;
- Express/Elysia compatibility adapter;
- arbitrary native addons;
- ORM/database built into core;
- built-in authentication product;
- full Web Platform implementation;
- WebSocket, SSE, queues, cron, email, or frontend rendering in the MVP;
- hostile multi-tenant code execution in the same process;
- dynamic production route registration;
- universal performance superiority;
- a final project name, public repository, license, or release date without owner decision.

## 7. Target users

### Primary — performance-sensitive TypeScript backend developer

Needs fast cold start and typed APIs without learning Rust.

### Primary — platform engineer

Needs deterministic binaries/packs, small capability surface, limits, readiness, diagnostics, and semantic API governance.

### Primary — frontend/full-stack developer

Needs Eden-like object calls and status-aware types without importing the server implementation into production browser bundles.

### Secondary — framework/plugin author

Needs stable compiler and native capability extension boundaries.

Detailed personas and journeys are in [Personas and Journeys](../project/personas-and-journeys.md).

## 8. Product principles

1. Measure the complete path.
2. Contracts precede handlers.
3. One contract, many artifacts.
4. No production discovery.
5. Route before JavaScript.
6. Materialize only what is used.
7. Expected failures are typed values.
8. Unsupported compatibility is explicit.
9. Local test shortcuts do not prove runtime conformance.
10. Performance claims follow reproducible evidence.

See [Design Principles](../project/principles.md).

## 9. Core user experience

### 9.1 Create an application

Provisional workflow:

```bash
bunx create-q-app my-api
cd my-api
bun install
bun run dev
```

Generated structure:

```text
src/
├── app.ts
├── modules/
│   └── health/
│       ├── contract.ts
│       ├── service.ts
│       └── routes.ts
└── tests/
    ├── health.unit.test.ts
    └── health.runtime.test.ts
q.config.ts
package.json
```

Scaffolding is P1. M0/M1 may use a checked-in proof fixture.

### 9.2 Define schema and route

```ts
import { defineModule, route, s, status } from "@q/core";

const Greeting = s.object({
  message: s.string()
});

export const hello = route({
  id: "hello.get",
  method: "GET",
  path: "/hello/:name",

  params: s.object({
    name: s.string({ minLength: 1, maxLength: 60 })
  }),

  response: {
    200: Greeting
  },

  handle({ params }) {
    return {
      message: `Hello ${params.name}`
    };
  }
});

export default defineModule({
  id: "hello",
  routes: [hello]
});
```

The handler receives normalized values. The compiler knows route identity, input, output, and handler reference without executing the handler.

### 9.3 Run in development

```bash
bun run q dev
```

Required behavior:

- source-located diagnostics;
- incremental rebuild/restart;
- development source maps;
- route inspection;
- behavior equivalent to release manifest;
- no claim that dev startup equals release cold start.

### 9.4 Build release artifact

```bash
bun run q build --profile serverless
```

Expected outputs:

```text
dist/app.qpack
dist/contract.d.ts
dist/contract.json
dist/openapi.json
dist/contract.lock.json
dist/build-report.json
```

### 9.5 Call through Treaty

```ts
import { treaty } from "@q/treaty";
import type { Api } from "./dist/contract";

const api = treaty<Api>({
  baseUrl: "http://localhost:3000"
});

const result = await api.hello({ name: "Rafi" }).get();

if (result.error) {
  console.error(result.error.status);
} else {
  console.log(result.data.message);
}
```

### 9.6 Inspect the application

```bash
q inspect routes
q inspect route hello.get
q inspect capabilities
q inspect fallbacks
q contract diff --against contract.lock.json
```

Inspection must distinguish native stages, JavaScript stages, wrapper/fallback usage, and expected capabilities.

## 10. Functional requirements

The stable requirement inventory is in [Project Requirements](../project/requirements.md). This section groups it into product capabilities.

### 10.1 Authoring core — P0

The core SHALL provide static declarations for:

- application;
- feature module;
- route;
- schema;
- policy;
- typed status/result;
- typed problem;
- service declaration;
- optional interceptor;
- defer registration.

The public API must remain small. An implementation helper is not public merely because tests need it.

### 10.2 Static compiler — P0

The compiler SHALL:

- resolve the application/module graph;
- extract recognized static route metadata without running services;
- canonicalize methods and paths;
- reject duplicates, collisions, shadow hazards, and unsupported dynamic metadata;
- normalize schema, policy, and capability graphs;
- emit route and handler IDs;
- bundle application JavaScript;
- generate a versioned application pack;
- emit compact Treaty contract and OpenAPI;
- emit build and compatibility reports;
- record hashes and versions.

### 10.3 Rust runtime — P0

The runtime SHALL:

- validate configuration and application pack;
- initialize exactly one QuickJS worker for M1/M2;
- load source or trusted version-compatible bytecode;
- cache handlers;
- bind HTTP/1.1;
- route natively;
- enforce header/body/queue/time/memory limits;
- expose lazy request handles;
- invoke policies and handlers;
- map declared results;
- write responses;
- handle cancellation and graceful shutdown;
- produce structured diagnostics.

### 10.4 Routing — P0

Support:

- static route;
- named parameter;
- terminal wildcard;
- 404;
- 405 with `Allow`;
- `HEAD` behavior;
- stable route IDs;
- canonical collision detection;
- native static liveness response.

### 10.5 Request inputs — P0

Support:

- path parameters;
- query values;
- selected headers;
- JSON body;
- text body;
- bytes body;
- route body limits;
- explicit content-type behavior.

Cookies and form encoding are P1 unless required by the proof application.

### 10.6 Responses — P0

Support:

- declared default success;
- declared alternate success status;
- typed problem response;
- JSON;
- text;
- bytes;
- empty response;
- safe headers;
- redacted unexpected 500.

Streaming and raw Web response are P1 after the normal structured path is stable.

### 10.7 Schema — P0

The initial schema IR SHALL support the documented scalar, object, array, optional, nullable, enum/literal, and bounded union subset.

The compiler SHALL expose the chosen validation/serialization strategy per route.

### 10.8 Policies — P0 vertical slice

At least one authentication-like policy SHALL:

- declare needed header;
- call a service or fixture;
- provide typed session context;
- return a declared 401 problem;
- propagate 401 into Treaty types;
- appear in route security inventory.

A complete auth product is not required.

### 10.9 Treaty — P0

Remote Treaty SHALL:

- build method/path/query/header/body from contract;
- type-check inputs;
- return `data` or status-narrowed `error`;
- distinguish network/abort from HTTP failure;
- avoid throwing HTTP errors by default.

Unit-local mode and actual runtime-local mode SHALL be named and tested separately.

### 10.10 Services/capabilities — P0/P1

M0/M1 require a minimal console/timer capability and one asynchronous native-operation proof. P1 adds outbound fetch and crypto subset.

A lazy fake/database-like service fixture proves service initialization is not part of unrelated cold start.

### 10.11 OpenAPI and contract governance — P1

Generate:

- OpenAPI;
- compact contract;
- contract lock;
- semantic diff;
- route/security/capability inventory.

M2 may generate a minimal valid subset before full OpenAPI polish.

### 10.12 Observability — P0

Provide:

- request ID;
- route ID;
- stage-aware errors;
- structured request completion log;
- startup stage timings;
- bridge metrics in benchmark builds;
- source-mapped JavaScript errors;
- secret redaction.

## 11. Non-functional requirements

### 11.1 Performance

Normative methodology is in [Benchmark Methodology](../engineering/benchmark-methodology.md).

P0 gates:

- zero runtime route compilation;
- zero runtime schema compilation;
- one initial worker;
- cached handler references;
- 25- and 1,000-route cold-start fixtures;
- matched raw Rust, raw Bun, Elysia 2 AOT, and Project Q baselines;
- bridge strategy comparison;
- p50/p95/p99 and failures;
- idle and loaded memory;
- artifact sizes;
- TypeScript check cost.

Targets are in [Performance Budgets](../engineering/performance-budgets.md).

### 11.2 Reliability

- malformed packs fail before ready;
- malformed requests fail safely;
- queues and buffers are bounded;
- cancellation cannot access expired handles;
- graceful shutdown is deterministic;
- undeclared statuses are controlled contract failures;
- no silent fallback.

### 11.3 Security

- trusted code only in same-process QuickJS;
- exact bytecode/runtime matching;
- application pack integrity;
- no dynamic native library loading;
- no ambient filesystem/network authority;
- secret redaction;
- outbound capability deadlines and future SSRF policy seam;
- `unsafe` limited to reviewed FFI boundary;
- fuzzing of externally controlled native parsers.

### 11.4 Maintainability

- one native host language initially;
- internal APIs hidden;
- feature-based TypeScript example structure;
- ADR for material design changes;
- requirements-to-test traceability;
- pinned dependencies and licenses;
- no performance optimization without benchmark before/after.

### 11.5 Compatibility/versioning

- runtime ABI version;
- application pack version;
- schema IR version;
- contract format version;
- engine and bytecode ABI;
- semantic framework packages;
- machine-readable compatibility report.

## 12. Product metrics

### 12.1 Thesis metrics

- p95 process-to-C3 validated response versus matched Elysia 2;
- p95 process-to-C4 policy response versus matched Elysia 2;
- idle RSS;
- 1,000-route startup delta versus 25 routes;
- empty/small-object bridge overhead versus raw Rust;
- runtime failure rate across cold samples.

### 12.2 Developer metrics

- TypeScript check time at 100/500/1,000 routes;
- compact contract declaration size;
- number of steps from route edit to Treaty autocomplete;
- diagnostics fixture quality;
- release build duration;
- local unit and actual runtime test duration.

### 12.3 Contract/operations metrics

- routes with unknown auth posture: zero;
- duplicate canonical routes accepted: zero;
- unreported JavaScript fallbacks: zero;
- undeclared status fixtures accepted: zero;
- compiler/runtime code present in client bundle: zero.

## 13. Proof application

The first vertical slice contains:

```text
GET  /health/live
GET  /hello/:name
POST /users
GET  /users/:id
```

Behavior:

- health uses native static response;
- hello proves path validation and QuickJS JSON;
- create user proves JSON body, response status 201, validation problem;
- get user uses session policy, typed 401, typed 404, and lazy in-memory service;
- all routes appear in Treaty, OpenAPI, manifest, and contract lock;
- a fake slow native operation proves promise/cancellation;
- no external database obscures framework results.

The same observable application is implemented in raw Bun and Elysia 2 AOT for comparisons. Raw Rust is a transport/bridge lower bound and cannot supply Treaty parity.

## 14. Acceptance criteria by first delivery

### M0 — Contracts and benchmark foundation

Accepted when:

- OKF/design bundle is ingested and validated;
- decisions/hypotheses/open questions are classified;
- matched benchmark fixtures are specified and minimally runnable;
- raw data format and cold-start definition are frozen;
- schema/route/Treaty public sketch type-checks;
- no performance claim is published.

### M1 — Runtime and bridge feasibility

Accepted when:

- Rust host loads one QuickJS worker;
- actual binary serves cached handlers;
- route-before-JS is proven;
- lazy request handle and expiry tests pass;
- text/small JSON/path/JSON input/promise/cancel paths work;
- JSON and response strategies are measured;
- memory and cold-start raw evidence is recorded;
- architecture passes or fails explicit gates.

### M2 — Compiled contract vertical slice

Accepted when:

- static compiler produces application pack and route manifest without app dry-run;
- initial schema IR validates proof routes;
- status-aware policy and problems work;
- remote Treaty and unit/runtime local modes work;
- OpenAPI/contract lock/build report are generated;
- proof app and matched Elysia/Bun comparisons run;
- all P0 traceability items for M0–M2 close or are explicitly waived;
- repository is clean and reviewable.

M2 is the first handoff stop. It is not called production-ready or public alpha.

## 15. Release criteria for a future public alpha

The alpha requires later milestones and is not authorized by the initial agent prompt.

At minimum:

- multi-worker service profile or documented single-worker limitation;
- outbound fetch and crypto capability;
- robust source maps;
- development workflow;
- cookie/form support if promised;
- fuzzing/security report;
- Linux x86_64 and aarch64 artifacts;
- semantic diff stability;
- compatibility documentation;
- honest benchmark report;
- examples and migration-free tutorial;
- owner decisions on name, license, repository, and governance.

## 16. Dependencies

Engineering dependencies:

- Rust stable toolchain;
- Tokio and hyper baseline;
- rquickjs;
- QuickJS-NG initial engine;
- Bun development/build/test tooling;
- TypeScript compiler API or selected parser/bundler;
- benchmark and process harness.

External sources are pinned in [Source Register](../references/source-register.md).

## 17. Risks

Top product risks:

1. Rust–QuickJS conversion overwhelms gains.
2. Static compiler becomes too magical or fragile.
3. QuickJS JavaScript performance is inadequate for realistic handlers.
4. Treaty types become slow at scale.
5. source/bytecode diagnostics are poor.
6. compatibility limitations block useful libraries.
7. cold-start gains disappear once real capabilities are linked.
8. framework scope expands before the thesis is proven.

See [Risks and Open Questions](risks-and-open-questions.md).

## 18. Product decisions held open

The owner has not yet finalized:

- public product name;
- package scope/import path;
- public repository;
- license;
- organization/trademark ownership;
- first platform support promise;
- public release date;
- governance model.

The implementation may use internal placeholders only.

## 19. Definition of done for this PRD

The PRD is satisfied as a handoff when the agent can:

- identify exact M0–M2 scope;
- trace P0 requirements to code/tests/evidence;
- distinguish target from measured result;
- know what not to build;
- stop on failed feasibility gates;
- report all verification honestly;
- deliver source, updated OKF, reports, archive, hashes, and clean commit state.
