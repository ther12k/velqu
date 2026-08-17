---
type: Requirements Specification
title: Project Q Requirements
description: Traceable product, compiler, runtime, schema, Treaty, performance, security,
  operations, and developer-experience requirements.
tags:
- requirements
- traceability
- compiler
- runtime
- treaty
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
- id: quickjs
  resource: https://bellard.org/quickjs/quickjs.html
  title: QuickJS documentation
- id: rfc-9457
  resource: https://www.rfc-editor.org/info/rfc9457/
  title: RFC 9457 Problem Details for HTTP APIs
---

# Requirement conventions

Requirements use stable IDs for traceability.

Priority:

- **P0** — required to validate the product thesis;
- **P1** — required for the first credible alpha;
- **P2** — useful after the thesis passes.

Status in this document is proposed until implementation evidence closes it.

# Product requirements

| ID | Priority | Requirement |
|---|---|---|
| PR-001 | P0 | The product SHALL optimize process-to-first-validated-response rather than quote engine initialization alone. |
| PR-002 | P0 | The developer SHALL write TypeScript business logic without Rust for ordinary routes. |
| PR-003 | P0 | The client SHALL provide object-like route navigation, autocomplete, and status-aware result narrowing. |
| PR-004 | P0 | The release compiler SHALL reject unsupported dynamic route or schema definitions with actionable diagnostics. |
| PR-005 | P0 | The runtime SHALL expose its compatibility and capability surface explicitly. |
| PR-006 | P1 | A separately published compact contract SHALL support client repositories that do not import server source. |
| PR-007 | P1 | Route and policy manifests SHALL support security and semantic API checks in CI. |

# Compiler requirements

| ID | Priority | Requirement |
|---|---|---|
| COMP-001 | P0 | Production route discovery SHALL happen at build time. |
| COMP-002 | P0 | The compiler SHALL NOT execute service initialization, open sockets, or connect to external systems to discover the app. |
| COMP-003 | P0 | Route IDs, methods, canonical paths, schemas, policies, and responses SHALL be deterministic. |
| COMP-004 | P0 | Duplicate, canonically equivalent, and shadowed routes SHALL fail the build. |
| COMP-005 | P0 | The compiler SHALL emit runtime/engine/schema/manifest versions and a contract hash. |
| COMP-006 | P0 | Unsupported imports and Node/Bun runtime APIs SHALL fail before deployment. |
| COMP-007 | P1 | Unused capabilities and schema backends SHOULD be removed from release artifacts. |
| COMP-008 | P1 | The compiler SHOULD emit OpenAPI and semantic contract-diff metadata from the same route contract. |
| COMP-009 | P1 | Builds SHOULD be reproducible for identical inputs and pinned toolchains. |

# Runtime requirements

| ID | Priority | Requirement |
|---|---|---|
| RUN-001 | P0 | The initial runtime SHALL use Rust and a QuickJS-family engine. |
| RUN-002 | P0 | The HTTP host SHALL route before invoking JavaScript. |
| RUN-003 | P0 | Handler functions SHALL be cached after application load. |
| RUN-004 | P0 | Unused request fields SHALL not be materialized into JavaScript objects. |
| RUN-005 | P0 | Body size, JavaScript heap, stack, execution time, pending work, and request queue SHALL be bounded. |
| RUN-006 | P0 | Cancellation and timeout SHALL propagate across HTTP, native operations, and JavaScript promises where supported. |
| RUN-007 | P0 | Unexpected exceptions SHALL become redacted internal problem responses in production. |
| RUN-008 | P0 | The runtime SHALL support graceful shutdown and deterministic cleanup. |
| RUN-009 | P1 | Static responses SHOULD bypass QuickJS. |
| RUN-010 | P1 | Service and multi-worker profiles SHALL make eager versus lazy startup explicit. |

# Schema and contract requirements

| ID | Priority | Requirement |
|---|---|---|
| SCHEMA-001 | P0 | One schema SHALL drive TypeScript inference, validation instructions, client inputs, responses, and OpenAPI metadata. |
| SCHEMA-002 | P0 | Coercion SHALL be explicit and source-aware. |
| SCHEMA-003 | P0 | Response status and body combinations SHALL be declared and type checked. |
| SCHEMA-004 | P0 | Policies SHALL contribute typed context and possible response statuses to route contracts. |
| SCHEMA-005 | P0 | JavaScript fallback validation SHALL be explicit in build output. |
| SCHEMA-006 | P1 | Third-party schema adapters MAY be accepted only when their semantics can be represented or clearly marked as fallback. |
| SCHEMA-007 | P1 | Semantic API diff SHALL classify breaking, compatible, and policy-sensitive changes. |

# Treaty requirements

| ID | Priority | Requirement |
|---|---|---|
| TRT-001 | P0 | The client SHALL infer path, query, header, and body inputs. |
| TRT-002 | P0 | The client SHALL represent success and HTTP failure separately without throwing by default. |
| TRT-003 | P0 | Failure values SHALL narrow by status code. |
| TRT-004 | P0 | The remote client runtime SHOULD remain small and independent of server implementation code. |
| TRT-005 | P0 | Fast local unit mode SHALL be labeled separately from native-runtime conformance. |
| TRT-006 | P1 | Published contract mode SHALL support independent repositories and package versioning. |
| TRT-007 | P1 | Client/server contract hash mismatch SHOULD be diagnosable. |

# Performance requirements

| ID | Priority | Requirement |
|---|---|---|
| PERF-001 | P0 | Benchmarks SHALL compare matched raw Rust, raw Bun, Elysia 2 AOT, and Project Q applications. |
| PERF-002 | P0 | Measurements SHALL report p50, p95, p99 or confidence intervals as appropriate, not a single best run. |
| PERF-003 | P0 | Cold-start measurements SHALL distinguish process-to-ready, first plaintext, first JSON, and first validated/policy route. |
| PERF-004 | P0 | The bridge suite SHALL isolate handler call, request access, JSON input, async completion, and response serialization costs. |
| PERF-005 | P0 | Route-count tests SHALL include at least 25 and 1,000 routes. |
| PERF-006 | P0 | Performance marketing SHALL be prohibited until release gates pass. |
| PERF-007 | P1 | TypeScript check and editor/client inference cost SHOULD be measured at 100, 500, and 1,000 routes. |

# Security and operations requirements

| ID | Priority | Requirement |
|---|---|---|
| SEC-001 | P0 | QuickJS bytecode SHALL be loaded only from trusted, version-matched build artifacts. |
| SEC-002 | P0 | Same-process QuickJS SHALL not be described as sufficient hostile-code isolation. |
| SEC-003 | P0 | Native handles SHALL have explicit ownership and lifetime rules. |
| SEC-004 | P0 | Logs and errors SHALL redact secrets, authorization values, cookies, and configured sensitive fields. |
| SEC-005 | P0 | Outbound network capability SHALL have timeout, cancellation, and future policy hooks for SSRF control. |
| OPS-001 | P0 | Build and runtime logs SHALL identify route ID, stage, and contract/runtime version. |
| OPS-002 | P0 | Machine-readable benchmark, compatibility, route, schema, and capability manifests SHALL be produced. |
| OPS-003 | P1 | Stage-level tracing SHOULD expose native time, JavaScript time, queue time, and materialization bytes. |

# Developer-experience requirements

| ID | Priority | Requirement |
|---|---|---|
| DX-001 | P0 | A new proof app SHALL build, test, and run through documented commands. |
| DX-002 | P0 | Diagnostics SHALL include source location and a corrective suggestion when practical. |
| DX-003 | P0 | Feature modules SHOULD separate contract/model, service, policy, and route adaptation. |
| DX-004 | P0 | Domain services SHOULD accept ordinary values rather than the entire framework context. |
| DX-005 | P1 | Development mode SHOULD preserve rapid reload without weakening release determinism. |
| DX-006 | P1 | The CLI SHOULD inspect routes, policies, capabilities, contract changes, and unsupported APIs. |

See [Traceability](../delivery/traceability.md) for planned evidence.
