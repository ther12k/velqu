---
type: Architecture Decision Record
title: ADR-0016 Product Naming — Velqu / VelquJS
status: accepted
date: 2026-08-18
decided-by: owner
---

# ADR-0016: Product naming — Velqu / VelquJS

## Context

OD-001 (product name) and OD-002 (package scope) were open owner decisions
since ingestion. The owner reviewed the candidates (Velqu vs VelquJS vs
VelquTS) and decided on 2026-08-18.

## Decision

| Slot | Name |
|---|---|
| Brand / framework | **Velqu** |
| Full descriptive name | **VelquJS** |
| CLI command | `velqu` |
| Package scope | `@velqu/*` (core, schema, contract, treaty, compiler, cli, testing) |
| Runtime binary | `velqu-runtime` |
| Compiler | Velqu Compiler |
| Client | Treaty (`@velqu/treaty`) |

Owner rationale (recorded): VelquJS communicates a JavaScript/TypeScript
runtime without locking identity to TypeScript — the runtime executes
JavaScript on QuickJS after compilation; VelquTS reads as a TS library rather
than a runtime; "Velqu" alone is the strongest independent identity with
VelquJS as the searchable long name. Ranking: Velqu > VelquJS > VelquTS.

Tagline: "VelquJS, or simply Velqu, is a cold-start-first TypeScript
framework and runtime powered by Rust and QuickJS."

## Scope of the mechanical rename

- TS workspace packages renamed `@q/*` → `@velqu/*` (imports, manifests,
  tsconfig paths, generated `contract.d.ts` import).
- CLI `q` → `velqu` (`bun run velqu …` / `packages/cli` bin).
- Runtime binary `q-runtime` → `velqu-runtime` ([[bin]] name; crate names in
  `crates/` remain internal).
- Internal Rust crate names (`q-*`) are NOT renamed: they are internal
  implementation units per repository-layout ("exact crate names remain
  internal"), and renaming them adds churn without user-facing value.
- Historical documents (the frozen OKF bundle under `docs/okf/` and evidence
  reports under `docs/reports/`) keep their original `@q/*` / `q-runtime`
  wording — provenance is preserved; this ADR is the mapping.

## Still open (owner)

OD-003 public repository/organization, OD-004 license, OD-005 platform
promise, OD-006 release/governance. The `velqu.dev` problem URNs remain a
placeholder domain pending OD-003.
