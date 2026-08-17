---
type: Architecture Review
title: 'Reviewed Architecture: Verdict, Corrections, and Open Tensions'
description: Critical review of the Rust, QuickJS, Bun, Elysia-inspired, Treaty-style
  framework design.
tags:
- architecture
- review
- quickjs
- cold-start
- tradeoffs
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
- id: aws-llrt
  resource: https://github.com/awslabs/llrt
  title: AWS LLRT
- id: quickjs
  resource: https://bellard.org/quickjs/quickjs.html
  title: QuickJS documentation
- id: quickjs-ng
  resource: https://quickjs-ng.github.io/quickjs/diff/
  title: QuickJS-NG differences and goals
- id: rquickjs
  resource: https://github.com/DelSkayn/rquickjs
  title: rquickjs Rust bindings
---

# Executive verdict

The concept is worth pursuing, but only as a **bounded runtime/compiler experiment with explicit kill criteria**. The strongest product combination is coherent:

```text
fast complete cold start
+ TypeScript-first authoring
+ Elysia-inspired contract discipline
+ Treaty-style client inference
+ native Rust infrastructure
+ QuickJS-family business-logic execution
```

The design becomes weak when it treats every native implementation as automatically faster, implies broad npm compatibility, or attempts to implement a complete framework surface before measuring the Rust–QuickJS boundary.

# Reviewed scorecard

| Area | Assessment | Decision |
|---|---|---|
| Product differentiation | Strong | Keep cold start + contract integrity + small runtime as the thesis. |
| Runtime choice | Plausible, unproven | Keep Rust + QuickJS behind an engine adapter and benchmark immediately. |
| Bun role | Clear after correction | Bun is development tooling, not the production JavaScript engine. |
| Native HTTP/routing | Strong | Keep route lookup and transport in Rust. |
| Native JSON/schema path | Uncertain | Treat as a measured strategy choice, not an axiom. |
| Eden-style client | Strong | Make status-aware Treaty ergonomics a first-class product feature. |
| Elysia 2 reuse | Valuable | Adopt ideas and best practices, not implementation coupling or API cloning. |
| Node compatibility | Dangerous scope | Explicitly reject full Node, Bun, Express, or Elysia compatibility. |
| Worker model | Reasonable later | Begin with one worker; introduce multiple/adaptive workers only after evidence. |
| Isolation claim | Needs correction | QuickJS limits are resource controls, not complete hostile-code isolation. |
| Compiler ambition | High risk | Build the smallest static manifest compiler before broad AST magic. |
| Performance claims | Premature | Publish methodology and raw evidence before comparison language. |

# Corrections incorporated into the baseline

## C-001 — Cold start means complete first useful response

The relevant metric is not bare QuickJS initialization. The product SHALL measure:

```text
process spawn
+ native runtime initialization
+ manifest/application loading
+ handler cache creation
+ socket readiness
+ first request dispatch
+ first valid response
```

Separate numbers SHALL be reported for process-to-ready, first plaintext, first JSON, first validated route, and first policy-protected route.

## C-002 — Native JSON is a hypothesis

Three paths MUST be implemented or simulated in the bridge spike:

1. QuickJS `JSON.parse` / `JSON.stringify`;
2. Rust parse/validate followed by JavaScript object materialization;
3. generated direct decoders/serializers for a restricted schema subset.

The winning path is selected per shape or route class only when end-to-end measurements include parsing, validation, allocation, conversion, handler access, and response serialization.

A Rust parser that produces a Rust value and then performs an expensive deep object conversion is not a performance win merely because its parser is fast.

## C-003 — Native-backed request values must be lazy

The hot-path request abstraction SHALL expose native handles and lazy access. Full Web `Request`, `Headers`, `URL`, or response wrappers MAY be constructed as explicit compatibility fallbacks. Unread fields SHALL not be copied into the QuickJS heap.

## C-004 — Local Treaty is not runtime conformance

A network-free local Treaty dispatcher is valuable for unit tests and developer speed, but it can bypass the Rust host, QuickJS bridge, native validation, cancellation, and serialization behavior.

Therefore testing has three named levels:

```text
unit-local       generated/local dispatcher, fastest
runtime-local    actual production binary over loopback
contract-remote  separately started server and remote Treaty client
```

Only the latter two can prove native runtime behavior.

## C-005 — One worker comes before adaptive workers

Lazy worker creation can improve initial readiness but may create latency spikes later. M1 therefore uses exactly one JavaScript worker. Adaptive and per-core pools are a later milestone with queueing, warm-up, tail-latency, memory, and fairness evidence.

## C-006 — Same-process QuickJS is not a hostile-code sandbox

Heap, stack, time, and interrupt limits are necessary controls. They do not replace process, operating-system, container, namespace, seccomp, or virtual-machine isolation for untrusted tenant code.

The initial product supports trusted application code. Untrusted execution is a non-goal until a separate isolation design is approved.

## C-007 — Static compilation must not execute application services

The compiler SHALL derive routes from a constrained static form and SHALL NOT run database setup, Redis connections, network requests, timers, or service factories to discover the application.

Dynamic route construction is rejected in release mode rather than silently evaluated.

## C-008 — The schema system needs a narrow native core

Arbitrary schema transformations, effects, custom refinements, and user callbacks are not directly representable in a safe compact native validator.

The first native schema IR therefore supports a documented subset. Unsupported semantics either:

- fail compilation; or
- use an explicit JavaScript fallback shown in route inspection and build reports.

## C-009 — Treaty requires two contract modes

Source mode should provide the best monorepo feedback loop. Published mode should provide a compact contract package that does not import the server implementation and scales to independent repositories.

Neither mode should require a production server framework type graph to be shipped to the browser.

## C-010 — The first runtime should use one host language

Rust is selected for the first host. Zig is not included merely as a theoretical optimization. It may be introduced only for an isolated, measured component through a later ADR.

# Decisions retained from the prior design

The following remain sound:

- Rust owns HTTP parsing, native routing, limits, and selected capabilities.
- QuickJS-family execution is used for ordinary TypeScript business logic.
- Bun remains the authoring, package, testing, and build toolchain.
- Release artifacts contain no TypeScript compiler, route registration, plugin discovery, OpenAPI generation, or runtime schema compilation.
- Status-specific responses are represented in server and client types.
- Errors use an RFC 9457-compatible problem model.
- Feature modules separate contracts, route adaptation, policies, and business services.
- Policies contribute typed context and possible errors.
- `defer()` is an after-response facility, not a durable job system.
- Node/Bun compatibility is limited and explicit.
- Performance claims require matched fixtures and reproducible evidence.

# Design tensions that remain intentionally open

| Tension | Current handling |
|---|---|
| QuickJS-NG versus upstream QuickJS | Start with QuickJS-NG through an adapter; benchmark upstream where practical. |
| `hyper`/Tokio versus a narrower custom event loop | Use mature Rust primitives for the first proof; optimize only with profiles. |
| Source extraction versus generated route modules | Start with compiler-friendly static declarations; preserve a future generated DSL option. |
| Native schema DSL versus Standard Schema adapters | Native subset first; adapters compile to IR or show fallback. |
| Persistent source JavaScript versus engine bytecode | Development uses source/source maps; release may use version-pinned bytecode. |
| Web API compatibility versus minimal bridge | Native lazy API is primary; Web objects are opt-in fallback. |
| One executable versus application pack | Use a versioned application pack first; add single-binary embedding after the format is stable. |

# Go/no-go principle

Project Q proceeds beyond the feasibility milestones only if evidence shows a defensible combination of:

- materially better complete cold start than matched Bun/Elysia;
- lower idle memory or stronger predictable resource limits;
- acceptable bridge overhead for realistic API routes;
- Treaty-quality developer experience;
- deterministic build and contract governance;
- maintainable implementation complexity.

If the QuickJS boundary erases the cold-start or memory advantage, the correct outcome is to narrow, redesign, or stop—not to hide the result behind synthetic static-response benchmarks.
