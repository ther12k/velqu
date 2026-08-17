---
type: Reference
title: Elysia 2 and Eden Treaty Design Notes
description: Transferable design ideas, Project Q differences, best-practice module
  structure, Treaty extensions, and attribution boundary.
tags:
- elysia
- elysia2
- eden
- treaty
- reference
status: stable
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
- id: elysia-lifecycle
  resource: https://elysiajs.com/essential/life-cycle
  title: Elysia lifecycle
- id: elysia-plugin
  resource: https://elysiajs.com/essential/plugin
  title: Elysia plugin and scope model
- id: elysia-validation
  resource: https://elysiajs.com/essential/validation
  title: Elysia validation and schema model
---

# Why these sources matter

Project Q is not an Elysia fork. Elysia 2 and Eden Treaty are reference designs for developer experience, typed contracts, lifecycle separation, and ahead-of-time optimization.

# Elysia 2 ideas used as inspiration

The Elysia 2 beta describes a rewritten, modular foundation with focus on memory, startup, bundle size, adapters, validation modularity, and AOT build behavior.

Relevant transferable ideas:

- precompute route/schema/default work before production;
- remove unused runtime modules;
- keep schema libraries modular;
- preserve clear route contract before handler;
- make lifecycle responsibilities explicit;
- treat application organization and plugin scope as first-class;
- support status-aware typed responses;
- use after-response work intentionally.

Project Q differs by:

- using Rust + QuickJS instead of Bun/JavaScriptCore in production;
- requiring native route dispatch;
- using a static extraction form that avoids application dry-run;
- rejecting general Elysia compatibility;
- exposing native/JavaScript pipeline boundaries.

# Elysia best-practice ideas used

The best-practice guide encourages feature grouping and separating service/domain logic from framework-heavy controller context. Project Q adopts:

```text
contract/schema
service/domain logic
policy/request-dependent context
route/HTTP adapter
module composition
```

A business service should be testable with ordinary values and interfaces.

# Eden Treaty ideas used

Eden Treaty demonstrates:

- object-like route navigation;
- inferred parameters/query/body;
- server response typing;
- status-aware error handling;
- a local application invocation/testing path.

Project Q extends this into:

- source contract mode;
- compact published contract mode;
- non-throwing status-narrowed results;
- explicit unit-local versus actual runtime-local tests;
- client/server contract hash diagnostics;
- language-neutral contract metadata.

# Ideas deliberately not copied

- exact method names or public API surface;
- dependency on Elysia application types;
- Bun runtime implementation;
- dynamic runtime plugin model;
- any implementation detail that conflicts with static compilation or QuickJS constraints.

# Attribution and independent implementation

External design inspiration should be cited in documentation. Code must be independently implemented and license-reviewed. Naming should avoid implying affiliation with Elysia unless a compatibility package is later explicitly approved.

# Verification boundary

These notes summarize source ideas. Actual dependency versions and current behavior must be verified from official documentation at implementation and benchmark time.
