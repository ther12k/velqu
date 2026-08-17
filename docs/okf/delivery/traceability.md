---
type: Traceability Matrix
title: M0–M2 Requirements Traceability
description: Maps product, compiler, runtime, schema, Treaty, performance, security,
  operations, and DX requirements to tests and evidence.
tags:
- traceability
- requirements
- tests
- evidence
status: draft
generated:
  by: openai/gpt-5.6-pro
  at: '2026-08-17T19:27:58+07:00'
---

# Traceability Matrix

This matrix links P0 requirements to milestones, implementation areas, tests, and evidence. The implementation agent must expand it to file-level paths as code is created.

| Requirement | Milestone | Implementation | Verification | Evidence |
|---|---|---|---|---|
| PR-001 | M0–M2 | cold-start harness/runtime startup | fresh-process C3/C4 suite | `benchmark-results.json` |
| PR-002 | M2 | `packages/core`, proof app | no Rust in app route fixture | proof source |
| PR-003 | M0/M2 | Treaty source/published mode | TypeScript positive/negative tests | type-test report |
| PR-004 | M2 | static compiler diagnostics | dynamic route/schema fixtures fail | compiler conformance |
| PR-005 | M2 | compatibility/capability manifests | unsupported import and inspect tests | build report |
| COMP-001 | M2 | route extractor/manifest | runtime contains prebuilt routes | pack inspection |
| COMP-002 | M2 | compiler architecture | trap service factory/network calls | side-effect test |
| COMP-003 | M2 | normalization/hash | rebuild determinism | manifest hash report |
| COMP-004 | M2 | collision analysis | duplicate/shadow fixtures | diagnostics snapshots |
| COMP-005 | M1/M2 | pack metadata | mismatch/tamper tests | pack report |
| COMP-006 | M2 | import compatibility pass | `node:http`, `Bun.serve` fail | diagnostics |
| RUN-001 | M1 | Rust + QuickJS adapter | actual binary test | runtime report |
| RUN-002 | M1 | native router | 404/405/static/handler trace | route trace |
| RUN-003 | M1 | handler cache | startup resolution count | engine report |
| RUN-004 | M1 | lazy request bridge | unread materialization counters | bridge report |
| RUN-005 | M1 | limits/admission | heap/stack/body/queue/deadline tests | security report |
| RUN-006 | M1 | cancellation token/async registry | race matrix | conformance report |
| RUN-007 | M1/M2 | error mapper/redaction | throw route fixture | security report |
| RUN-008 | M1 | shutdown state machine | active/queued shutdown tests | runtime report |
| SCHEMA-001 | M2 | schema IR/generators | type/native/Treaty/OpenAPI fixture | schema report |
| SCHEMA-002 | M2 | source-aware coercion | params/query/body differences | schema conformance |
| SCHEMA-003 | M2 | result constructors/manifest | undeclared status negative tests | type/runtime report |
| SCHEMA-004 | M2 | policy contract composition | session context/401 types | policy tests |
| SCHEMA-005 | M2 | build report | fallback fixture visible | build report |
| TRT-001 | M2 | Treaty encoder/types | path/query/header/body type tests | Treaty report |
| TRT-002 | M2 | Treaty result | declared HTTP error non-throwing | client tests |
| TRT-003 | M2 | status discriminant | switch narrowing tests | type tests |
| TRT-004 | M2 | compact client | bundle/import graph check | size report |
| TRT-005 | M2 | local test APIs | unit versus runtime labels | testing report |
| PERF-001 | M0–M2 | benchmark candidates | matched fixture audit | methodology/report |
| PERF-002 | M0 | statistics pipeline | p50/p95/p99/raw samples | result JSON |
| PERF-003 | M0 | cold route classes | ready/C1–C4 results | result JSON |
| PERF-004 | M1 | bridge harness | input/output strategy matrix | bridge report |
| PERF-005 | M0/M2 | route generators | 25/1,000 apps | cold-start report |
| PERF-006 | Continuous | docs/release gate | claim scanner/manual review | release report |
| SEC-001 | M1/M2 | pack/engine loader | tamper/version tests | security report |
| SEC-002 | Documentation | positioning | no sandbox claim; process isolation deferred | review checklist |
| SEC-003 | M1 | opaque generation handles | expiry/wrong-owner tests | FFI report |
| SEC-004 | M1/M2 | logger/error mapper | secret fixtures | redaction report |
| SEC-005 | M2/P1 | async capability seam | timeout/cancel; SSRF hooks documented | capability report |
| OPS-001 | M1/M2 | logs/manifest IDs | correlation consistency | observability report |
| OPS-002 | M0–M2 | report generators | schema validation | evidence inventory |
| DX-001 | M2 | proof app/scripts | clean-checkout run | handoff report |
| DX-002 | M2 | compiler/runtime diagnostics | source-located snapshots | diagnostics report |
| DX-003 | M2 | example feature modules | structure review | proof app |
| DX-004 | M2 | ordinary service interfaces | unit test without framework context | service tests |

# Required implementation update

When work begins, add columns:

```text
Source file
Test file
Commit
Status
Waiver/decision
```

No requirement is marked complete because a file exists. Completion requires the listed verification and evidence.

# Waivers

A waiver must include:

- requirement ID;
- reason;
- owner/authority;
- date;
- risk;
- compensating control;
- expiration or review milestone.

The implementation agent may propose waivers but cannot silently create owner approval.
