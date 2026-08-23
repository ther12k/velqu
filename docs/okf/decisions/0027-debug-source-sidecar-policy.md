---
type: Architecture Decision Record
title: ADR-0027 Debug Source Sidecar Policy
status: accepted
date: 2026-08-23
implements: ADR-0024 (numeric mode policy), ADR-0025 (mode-2 layout), ADR-0017 (bytecode embedding)
---

# ADR-0027: Debug Source Sidecar Policy

## Context

Legacy v1 packs optionally carry `sourceMap` (and always carry the
bundle source string) inside the artifact. Binary QPack v2 aims for
minimal, reproducible production bytes — raw bytecode and dense tables,
no JavaScript text. Developers still need sources and maps for stack
traces, profiling, and `velqu inspect`-style tooling. The two needs are
separated by moving debug material out of the artifact entirely.

## Decision

### 1. Production packs carry no debug/source content

- **Mode 2**: there is no source or source-map section in the catalog
  (ADR-0025 §6). Bundle bytecode (0x0007) is OPTIONAL precisely so a
  source-only rebuild path remains possible via the compiler, while
  production packs ship bytecode only.
- **Mode 1 (legacy)**: `sourceMap` and `bundle` stay as frozen v1
  fields for compatibility, but producers SHOULD omit `sourceMap` for
  production builds (`emit({ sourceMap: false })`). The producer
  default flip to debug-free output lands atomically with the M26-003
  encoder — one reviewed change, not a silent drift.

### 2. The sidecar is a separate, untrusted, non-executable file

```text
deploy/
  app.qpack                    <- production artifact (no sources/maps)
  app.qpack.sources.json       <- sidecar: {formatVersion, packSha256,
                                   bundleSource?, sourceMap?,
                                   modules: [{id, file}] }
```

- Named `<pack>.sources.json`, sitting NEXT TO the pack, never inside
  any numeric mode's layout.
- Binds to exactly one pack via its SHA-256 (`packSha256`) — advisory
  metadata for humans and tooling, verified by the TOOL, not by the
  runtime.
- The runtime NEVER reads sidecars: no load path, no fallback, no
  env-var can make it consult one (pinned by test
  `verification_is_independent_of_debug_sidecars`). A missing or wrong
  sidecar changes nothing about serving.
- Tooling (dev server, inspect, profilers) may read sidecars to
  symbolize traces; sidecars are untrusted input there and must never
  influence route/schema/policy interpretation.

### 3. Trust model

Sidecars inherit ADR-0026: they are integrity-referenced
(`packSha256`) but confer no authenticity. Shipping a sidecar grants no
privilege;   shipping a WRONG sidecar corrupts only developer
ergonomics, never behavior. Stack-trace fidelity is a debugging aid,
not a security control.

### 4. Compatibility matrix

| pack \ sidecar | absent | present & matching | present & mismatched |
|---|---|---|---|
| production (no sources) | serves normally | tooling symbolizes; serves normally | serves normally; tooling warns |
| debug build w/ sources (legacy v1 only) | serves normally; unsymbolized traces | serves normally; tooling symbolizes | serves normally; tooling warns |
| unknown formatVersion | rejected (ADR-0024) | rejected | rejected |

## Consequences

- Mode-2 packs become minimal and reproducible; byte-for-byte rebuilds
  no longer embed timestamps-laden maps.
- Symbolization quality depends on operators shipping sidecars with
  their deployments when they want traceable stacks (beta devex docs).
- The producer-side flip (stop embedding maps by default; write the
  sidecar instead) is scheduled with M26-003-B's encoder so both sides
  of the format change land together.
