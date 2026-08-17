---
type: Architecture Decision Record
title: ADR-0015 Native JSON as default input strategy
status: accepted
date: 2026-08-17
supersedes: null
---

# ADR-0015: Native JSON as the default input strategy

## Context

The design review (architecture/review-and-corrections.md) reclassified
native JSON parsing/validation as a HYPOTHESIS because Rust→QuickJS object
conversion might erase native parsing gains. M1 required measurement before
adoption (also performance-budgets.md bridge gate).

## Decision

Adopt strategy B (Rust `serde_json` parse + recursive object construction
into QuickJS) as the compiler default for JSON BODY inputs, and native
conversion as the default RESPONSE strategy. Engine-side JSON (strategy A)
remains available per route.

## Evidence

`benchmarks/raw/bridge/bridge-summary.json` (2000 samples/case, release
build, all cases correctness-asserted): inputs small −34%, nested −11%,
array100 −42%; responses: small ≈equal (22.1 vs 22.6μs), array100 −23%.

## Consequences

- Compiler emits `validationStrategy: "native"` + native response strategy by
  default; any JS fallback must appear in build reports (SCHEMA-005).
- Claim is scope-limited: quickjs-ng 0.15.1 via rquickjs 0.12.2 on the
  reference host. Re-measure if the engine or conversion layer changes.
- rquickjs object construction proved cheap enough that the review's worry
  does not hold for INPUTS on this pair; the review doc's caution remains
  valid guidance for future engines.
