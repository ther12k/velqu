---
type: Knowledge Bundle Guide
title: Project Q Framework Design and Product Handoff
description: Navigation, trust boundary, reviewed thesis, and implementation handoff
  for the Rust and QuickJS cold-start-first TypeScript framework.
tags:
- project-q
- quickjs
- rust
- bun
- elysia
- eden-treaty
- okf
- framework
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
---

# Project Q Design and Product Handoff Bundle

## Purpose

This bundle records the reviewed product and engineering design for **Project Q**, a working codename for a cold-start-first TypeScript server framework.

The intended developer experience is inspired by the strongest parts of Elysia and Eden Treaty, while production execution uses a purpose-built Rust host and a QuickJS-family engine:

```text
Bun-first development workflow
        │
        ├── TypeScript route contracts
        ├── schema-derived types and validation
        ├── Treaty-style client contract
        └── build-time compiler
                    │
                    ▼
Rust HTTP host + QuickJS execution
        │
        ├── native route dispatch
        ├── bounded request bridge
        ├── precompiled route/schema metadata
        ├── typed status-aware responses
        └── capability-linked runtime
```

The goal is not to reproduce Elysia method-for-method and not to create a general Node.js replacement. The product hypothesis is narrower:

> A statically compiled TypeScript contract system can provide materially faster cold starts, lower idle memory, predictable route composition, and Eden Treaty-quality end-to-end typing when Rust owns infrastructure and QuickJS executes business logic.

## Executive review verdict

The direction is coherent enough to justify an implementation spike, but several earlier assumptions were too confident.

### Accepted core decisions

1. Cold start is the primary runtime optimization target.
2. Bun is the preferred package manager, test runner, and TypeScript build interface, not the production JavaScript engine.
3. Rust owns HTTP, route dispatch, limits, scheduling boundaries, and native capabilities.
4. QuickJS-NG is the initial engine candidate behind an internal adapter.
5. Route, schema, policy, response, OpenAPI, and Treaty metadata come from one static contract.
6. Production startup must not discover routes, compile schemas, resolve plugins, or execute application setup.
7. The client must preserve Eden Treaty-style object navigation, autocomplete, typed parameters, and status-narrowed failures.
8. Performance claims remain hypotheses until reproduced against raw Rust, raw Bun, and Elysia 2 AOT.

### Corrections introduced by this review

- Native JSON parsing and validation are **not automatically faster** once Rust-to-QuickJS conversion is counted. The body and response bridge requires a measured design spike.
- A network-free Treaty unit mode cannot be advertised as native-runtime conformance unless it executes the actual Rust pipeline. Fast unit mode and native integration mode are separate.
- Adaptive worker creation improves minimum cold start but can create later tail-latency spikes. Deployment profiles must expose this tradeoff.
- QuickJS resource limits are useful controls, but they do not make same-process execution a sufficient sandbox for hostile code.
- QuickJS bytecode is engine-version-bound and trusted-input-only. Release artifacts must pin and attest the runtime/engine pair.
- Rust and Zig together would increase complexity without evidence. Rust alone is the initial implementation language.

See [Architecture Review and Corrections](architecture/review-and-corrections.md) for the full assessment.

## Status and trust

All product and architecture concepts in this bundle are `draft`. They represent a reviewed proposal, not completed implementation or measured performance.

Normative words such as **MUST**, **SHOULD**, and **MAY** define intended contracts. They do not assert that those contracts already exist.

Performance values are budgets, comparison gates, or kill criteria. They are not benchmark results.

## Recommended reading order

1. [Project Charter](project/charter.md)
2. [Vision and Positioning](project/vision-and-positioning.md)
3. [Architecture Review and Corrections](architecture/review-and-corrections.md)
4. [Product Requirements Document](delivery/prd.md)
5. [Architecture Overview](architecture/overview.md)
6. [Compiler and Build Model](architecture/compiler-and-build.md)
7. [Rust Host Runtime](architecture/rust-host-runtime.md)
8. [QuickJS Engine Integration](architecture/quickjs-engine.md)
9. [Contract Type System](architecture/contract-type-system.md)
10. [Treaty Client](architecture/treaty-client.md)
11. [Minimum Viable Product](delivery/mvp.md)
12. [Performance and Benchmark Strategy](engineering/benchmark-methodology.md)
13. [Risks and Open Questions](delivery/risks-and-open-questions.md)
14. [Master Implementation Prompt](MASTER_AGENT_PROMPT.md)

## Handoff boundary

The attached implementation prompt authorizes an AI development agent to complete the evidence-focused **M0–M2** slice:

```text
M0 — feasibility, contracts, and fair baselines
M1 — one-worker Rust/QuickJS runtime bridge
M2 — minimal static contract compiler and Treaty client
```

It deliberately stops before production claims, broad npm compatibility, multi-worker scaling, databases, WebSockets, or a public release.

## Naming

“Project Q” is a working codename only. This bundle does not select a final project name, package scope, public repository, license, organization, trademark position, or release date.
