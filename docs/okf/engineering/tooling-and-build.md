---
type: Engineering Specification
title: Tooling and Build System
description: Rust/Bun toolchain, command surface, development/release builds, dependencies,
  CI, side-effect controls, and packaging.
tags:
- tooling
- build
- ci
- bun
- cargo
status: draft
generated:
  by: openai/gpt-5.6-pro
  at: '2026-08-17T19:27:58+07:00'
---

# Toolchain

Initial development toolchain:

```text
Rust stable, pinned through rust-toolchain.toml
Cargo and Cargo.lock
Bun, pinned in environment manifest
TypeScript, pinned
QuickJS-NG/rquickjs, exact dependency revisions or locked releases
Linux reference environment
```

The agent must verify actual current stable versions from official sources at implementation time and record them. This design bundle does not freeze future dependency versions.

# Commands

Target command surface:

```bash
bun run q dev
bun run q build
bun run q inspect routes
bun run q contract build
bun run q contract diff
bun test
cargo test
./scripts/verify
./scripts/benchmark cold-start
./scripts/package
```

One top-level verification command should cover all checks in the authorized milestone.

# Build layers

```text
TypeScript contract compile
→ JavaScript application bundle
→ route/schema/policy/capability manifests
→ application pack
→ optional standalone runtime packaging
```

The Rust runtime is built separately from each application where possible. Single executable packaging composes known artifacts rather than recompiling framework semantics differently.

# Development build

Requirements:

- useful source maps;
- fast enough incremental feedback;
- explicit restart/reload;
- manifest parity with release;
- development diagnostics;
- no dependence on production bytecode.

A development convenience may use source JavaScript and a directory pack.

# Release build

Requirements:

- release Rust optimization;
- deterministic normalized manifests;
- no compiler/dev dependencies in runtime artifact;
- pack integrity metadata;
- explicit engine/ABI versions;
- fallback and capability report;
- contract/OpenAPI artifacts;
- reproducibility report;
- zero hidden route/schema compilation at startup.

# Compiler plugin execution

Compiler plugins are pinned build dependencies. They run with the same trust as the build, but:

- receive versioned APIs;
- cannot mutate arbitrary compiler globals;
- cannot perform unnoticed network access in reproducible mode;
- declare output and non-deterministic inputs;
- appear in the build report.

A strict build mode may deny network and unexpected filesystem access.

# Lint and formatting

Rust:

```text
rustfmt
clippy with warnings denied for production crates
cargo deny/audit or selected equivalent
```

TypeScript:

```text
formatter
lint rules focused on correctness
tsc/type tests
unused and forbidden runtime import checks
```

Generated code has deterministic formatting and is not manually edited.

# Dependency policy

Before adding a dependency:

- identify why standard/current dependencies are insufficient;
- document startup/artifact/security/license impact;
- pin appropriately;
- avoid overlapping libraries;
- keep compiler-only dependencies out of runtime;
- run baseline before/after for hot-path dependencies.

No custom HTTP/TLS/crypto implementation is introduced merely to reduce dependency count.

# Build-time side-effect controls

Release compilation tests set traps so recognized app/service declarations fail if imported execution attempts:

- network;
- database;
- timer;
- process spawn;
- write outside build output;
- environment-dependent route creation.

The preferred architecture avoids importing executable app setup at all.

# Artifact inventory

Each build emits machine-readable metadata:

```json
{
  "inputs": [],
  "outputs": [],
  "versions": {},
  "hashes": {},
  "routes": 0,
  "schemas": 0,
  "capabilities": [],
  "fallbacks": [],
  "reproducible": true
}
```

# CI stages

Recommended:

1. formatting and static checks;
2. OKF/internal-link validation;
3. TypeScript type tests;
4. Rust unit tests;
5. compiler golden/conformance;
6. runtime integration;
7. Treaty unit and runtime modes;
8. security/pack/limit tests;
9. release build;
10. short smoke benchmark;
11. artifact/report validation.

Long cold-start distributions, sanitizer, fuzzing, and cross-platform builds may run in scheduled or release CI.

# Packaging

The package script:

- validates clean tree or records allowed state;
- builds release artifacts;
- runs required gates;
- generates reports;
- copies canonical OKF;
- creates archive;
- calculates SHA-256;
- records commit/tool versions.

It never labels unexecuted tests as passing.
