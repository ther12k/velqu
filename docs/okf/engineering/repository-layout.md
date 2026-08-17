---
type: Engineering Specification
title: Repository Layout and Boundaries
description: Monorepo structure and strict boundaries among Rust host, engine, bridge,
  TypeScript compiler, Treaty, baselines, conformance, and evidence.
tags:
- repository
- monorepo
- rust
- typescript
- boundaries
status: draft
generated:
  by: openai/gpt-5.6-pro
  at: '2026-08-17T19:27:58+07:00'
---

# Repository layout

The first implementation should be a monorepo because compiler, runtime, contracts, Treaty, fixtures, and evidence must evolve atomically during feasibility work.

```text
/
├── AGENTS.md
├── README.md
├── Cargo.toml
├── Cargo.lock
├── package.json
├── bun.lock
├── q.config.example.ts
│
├── crates/
│   ├── q-runtime/
│   ├── q-engine/
│   ├── q-engine-quickjs/
│   ├── q-http/
│   ├── q-router/
│   ├── q-bridge/
│   ├── q-pack/
│   ├── q-schema-runtime/
│   ├── q-capabilities/
│   └── q-bench-support/
│
├── packages/
│   ├── core/
│   ├── schema/
│   ├── treaty/
│   ├── compiler/
│   ├── cli/
│   ├── testing/
│   └── contract/
│
├── examples/
│   └── proof/
│
├── baselines/
│   ├── raw-rust/
│   ├── raw-bun/
│   └── elysia2/
│
├── conformance/
│   ├── routing/
│   ├── schema/
│   ├── bridge/
│   ├── treaty/
│   ├── lifecycle/
│   └── security/
│
├── benchmarks/
│   ├── harness/
│   ├── fixtures/
│   ├── raw/
│   ├── summaries/
│   └── manifest.json
│
├── docs/
│   ├── okf/
│   ├── reports/
│   ├── implementation-audit.md
│   ├── open-decisions.md
│   └── m0-m2-traceability.md
│
├── scripts/
│   ├── verify
│   ├── benchmark
│   ├── validate-okf
│   └── package
│
└── target/dist/
```

Exact crate/package names remain internal until public naming is approved.

# Boundary rules

## Rust crates

- `q-engine` defines the narrow engine trait.
- `q-engine-quickjs` owns QuickJS/rquickjs FFI and engine-specific code.
- `q-bridge` owns opaque handles, conversion, and async completion.
- `q-runtime` composes components but does not expose low-level FFI.
- `q-pack` parses/verifies the versioned application artifact.
- `q-http` and `q-router` are independently benchmarkable.
- capability implementations cannot mutate route manifests.

## TypeScript packages

- `core` contains static authoring primitives and types, not runtime server code.
- `schema` defines public schema declarations/IR types.
- `compiler` parses/normalizes/emits artifacts and never ships in production runtime.
- `treaty` contains the small remote client and local testing adapters.
- `testing` provides explicit unit-local/runtime-local helpers.
- `cli` orchestrates build/inspect/dev/contract commands.
- `contract` contains language-neutral/generated contract utilities where needed.

# Public versus internal API

A symbol is public only when:

- documented;
- covered by compatibility policy;
- required by proof applications or extension API;
- intentionally exported from a public package.

Do not export internals to avoid relative imports in tests. Use crate-internal tests or test support modules.

# Baseline isolation

Baseline applications may share:

- request/response fixture definitions;
- benchmark process protocol;
- payload files;
- correctness assertions.

They must not share Project Q runtime code.

Every baseline records its own lockfile/build command and remains idiomatic to that stack.

# Generated files

Generated output goes under clearly named paths and is reproducible where expected. Source-controlled generated files require a reason:

- contract type snapshots;
- conformance fixtures;
- application pack golden metadata;
- documentation inventory.

Large benchmark raw results can be archived by milestone rather than rerun during every unit test.

# Documentation

`docs/okf/` is the canonical evolving bundle. Source references are preserved. Implementation reports link exact commits and commands.

The root README is operational; the OKF README is architecture/product navigation.

# Repository invariants

- one command verifies authorized scope;
- lockfiles are committed;
- dependency versions are pinned appropriately;
- no secrets or production values;
- no generated benchmark claim without raw data;
- no public package/repository/license assertion before approval;
- milestone checkpoint leaves a clean tree;
- every P0 requirement maps to code, tests, and evidence.
