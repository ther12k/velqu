---
type: Evidence Report
title: Runtime Conformance Report
status: complete
milestone: M1–M2
---

# Runtime conformance report (M1/M2 §12.9)

## Overview

The runtime host (`velqu-runtime`) was validated through 57 Rust unit/integration
tests and 30 TypeScript conformance suites executing against the actual
compiled release binary over HTTP.

## Test Results Summary

| Suite | Category | Tests | Status |
|---|---|---|---|
| `q-pack` | Pack format validation & tamper detection | 6 | PASS |
| `q-pack` fuzz | Pack parser robustness + mutation tamper detection | 2 | PASS |
| `q-schema-runtime` | Schema IR types, coercion & regex | 7 | PASS |
| `q-schema-runtime` fuzz | Validator robustness + determinism | 2 | PASS |
| `q-router` | Route matching, collisions, 404/405/HEAD | 7 | PASS |
| `q-bridge` | Opaque handle generation & counters | 4 | PASS |
| `q-engine-quickjs` | Engine lifecycle, promises, cancel, timers, runaway-continuation interrupt, envelope regression | 14 | PASS |
| `q-http` fuzz | Query/percent-decode robustness | 3 | PASS |
| `q-runtime` integration | HTTP limits, source maps, shutdown, liveness, response-schema violation, bytecode pack | 10 | PASS |
| `q-bench-support` | Timer capability in multi-thread runtime | 2 | PASS |
| **Rust Total** | | **57** | **PASS** |
| `conformance/compiler` | Static AST extraction, traps, lock workflow | 7 | PASS |
| `conformance/treaty` | Treaty client & runtime-local execution | 3 | PASS |
| `conformance/routing` | Static, param, wildcard, 404/405 | 1 | PASS |
| `conformance/schema` | Schema IR v1 builders & coercion | 6 | PASS |
| `conformance/bridge` | Lazy handle access & timer promises | 2 | PASS |
| `conformance/lifecycle` | Policy injection & lazy service (C5) | 1 | PASS |
| `conformance/security` | 500 error redaction & payload limits | 2 | PASS |
| `packages/` unit tests | Schema builders, core constructors, treaty client | 7 | PASS |
| `examples/proof` unit | Health module unit test | 1 | PASS |
| **TypeScript Total** | | **30** | **PASS** |
| **Combined Conformance** | | **87** | **ALL PASS** |

## Key Runtime Behaviors Verified

1. **Native Route Before JavaScript (RUN-002)**: `/health/live` served natively (logged as `stage: native`, `x-velqu-stage: native` header).
2. **Handle Invalidation on Settle (SEC-003)**: Retained handle wrappers throw `Error: request handle expired` when accessed after response settlement.
3. **Cancellation & Timers (RUN-006)**: Client disconnect aborts pending timers and settles request slots cleanly.
4. **Error Redaction (RUN-007)**: `throw new Error("secret-boom")` produces a generic 500 RFC 9457 response with zero secret leakage; origin file & line logged to server stderr via embedded source map.
5. **Limits Admission (RUN-005)**: Header blocks > 32 KiB return 431, bodies > 65,536 B return 413, queue saturation returns 503.
6. **Response-Body Contract (SCHEMA-003, enhanced)**: declared response schemas are validated at runtime; a violating body becomes a controlled 500 with an internal `contract.violation.response` log (test `response_schema_violation_is_a_controlled_500`).
7. **Runaway Promise Continuations (enhanced)**: drain-time interrupt arming kills infinite loops inside `.then()` continuations; the worker survives (test `runaway_promise_continuation_is_interruptible_and_worker_survives`).
8. **Typed Result Envelopes (enhanced)**: only TAGGED `{__ok}`/`{__problem}` objects are result envelopes; business objects containing `status`/`value` fields are bodies (test `business_object_with_status_and_value_fields_is_a_body_not_an_envelope`).
9. **Bytecode Execution & Tamper Rejection (ADR-0017)**: `velqu-bytecode` compiled packs serve identically to source packs with ~10.7% faster cold start at 1,000 routes; tampered bytecode is rejected before ready.
