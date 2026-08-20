---
type: Workstream
title: CLI, Developer Experience, and Documentation
status: draft
tags:
- cli
- devex
- docs

---

# CLI, Developer Experience, and Documentation

## Beta CLI

```text
velqu dev
velqu build
velqu inspect
velqu contract diff
velqu test
velqu pack inspect
velqu pack migrate
velqu new
```

Every command has stable exit codes, human-readable diagnostics, and optional machine-readable output.

## Development parity

`velqu dev` runs the actual Rust/QuickJS runtime. A Bun-local dispatcher is allowed only as an explicitly named unit-test mode.

## Documentation set

- Installation and quickstart.
- Project/module/service/contract structure.
- Schemas, policies, problems, and Treaty.
- Fetch, capabilities, Postgres, and auth reference.
- Runtime profiles and deployment.
- Debugging/source maps/inspect.
- Performance methodology and bytecode-versus-JIT explanation.
- Limits, unsupported APIs, trust model, and beta stability.

## Documentation quality gate

All commands and code samples execute in CI. No sample depends on unpublished local packages. Links are checked. Measured results and engineering targets are visually separated.
