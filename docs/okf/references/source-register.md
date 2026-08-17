---
type: Reference Register
title: External Source Register and Precedence
description: Primary official references, their design role, implementation source
  precedence, and citation/versioning policy.
tags:
- sources
- references
- provenance
- official
status: stable
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
- id: rfc-9457
  resource: https://www.rfc-editor.org/info/rfc9457/
  title: RFC 9457 Problem Details for HTTP APIs
- id: bun-docs
  resource: https://bun.sh/docs
  title: Bun documentation
- id: hyper
  resource: https://docs.rs/hyper
  title: hyper Rust HTTP library
- id: tokio
  resource: https://tokio.rs/
  title: Tokio asynchronous runtime
---

# External source register

| ID | Source | Role in design |
|---|---|---|
| `okf-spec` | Google Cloud Platform OKF v0.2 specification | Bundle structure, frontmatter, reserved resources, provenance/trust conventions |
| `elysia-2` | Official Elysia 2 beta announcement | AOT, modularity, startup/memory/bundle design context |
| `eden-treaty` | Official Eden Treaty documentation | Object-like typed client and status-aware response inspiration |
| `elysia-best-practice` | Official Elysia best-practice guide | Feature modules and service/controller separation |
| `elysia-lifecycle` | Official lifecycle guide | Lifecycle responsibility reference |
| `elysia-plugin` | Official plugin guide | Scope, dependency, and modularity reference |
| `elysia-validation` | Official validation guide | Schema-first contract reference |
| `aws-llrt` | AWS Labs LLRT repository | Rust + QuickJS selected native runtime reference |
| `quickjs` | Upstream QuickJS documentation | Engine, limits, interrupts, bytecode constraints |
| `quickjs-ng` | QuickJS-NG documentation | Initial engine candidate and optimization context |
| `rquickjs` | rquickjs repository | Rust binding/async integration candidate |
| `rfc-9457` | RFC 9457 | HTTP API Problem Details model |
| `bun-docs` | Official Bun documentation | Development/package/test/build toolchain context |
| `hyper` | hyper documentation | Initial Rust HTTP implementation baseline |
| `tokio` | Tokio documentation | Initial asynchronous runtime baseline |

# Source-of-truth precedence

For implementation:

1. explicit owner instruction in the current task;
2. accepted Project Q ADR;
3. current PRD and architecture;
4. engineering standards and delivery plans;
5. external source behavior verified at implementation time;
6. this design-session record.

When an external project changes, do not rewrite old benchmark evidence. Record a new pinned baseline.

# Citation policy

- cite official sources for behavior and versions;
- prefer primary documentation/repositories;
- avoid copying implementation code without license review;
- do not claim Project Q compatibility solely from conceptual similarity;
- distinguish external observed claims from Project Q targets;
- preserve source title, URL, version/date/commit where available.

# Offline bundle note

External URLs may change. Implementation work should capture exact version/commit metadata and, where licensing permits, store small derived notes or hashes—not copied full documentation.
