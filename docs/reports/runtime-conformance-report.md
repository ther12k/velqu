---
type: Evidence Report
title: Runtime Conformance Report
status: complete
milestone: M1–M2
---

# Runtime conformance report (M1/M2 §12.9)

## Overview

The runtime host (`q-runtime`) was validated through 45 Rust unit/integration
tests and 21 TypeScript conformance suites executing against the actual
compiled release binary over HTTP.

## Test Results Summary

| Suite | Category | Tests | Status |
|---|---|---|---|
| `q-pack` | Pack format validation & tamper detection | 6 | PASS |
| `q-schema-runtime` | Schema IR types, coercion & regex | 7 | PASS |
| `q-router` | Route matching, collisions, 404/405/HEAD | 7 | PASS |
| `q-bridge` | Opaque handle generation & counters | 4 | PASS |
| `q-engine-quickjs` | Engine lifecycle, promises, cancel, timers | 12 | PASS |
| `q-runtime` integration | HTTP limits, source maps, shutdown, liveness | 8 | PASS |
| `q-bench-support` | Timer capability in multi-thread runtime | 1 | PASS |
| **Rust Total** | | **45** | **PASS** |
| `conformance/compiler` | Static AST extraction & trap tests | 6 | PASS |
| `conformance/treaty` | Treaty client & runtime-local execution | 3 | PASS |
| `conformance/routing` | Static, param, wildcard, 404/405 | 1 | PASS |
| `conformance/schema` | Schema IR v1 builders & coercion | 6 | PASS |
| `conformance/bridge` | Lazy handle access & timer promises | 2 | PASS |
| `conformance/lifecycle` | Policy injection & lazy service (C5) | 1 | PASS |
| `conformance/security` | 500 error redaction & payload limits | 2 | PASS |
| **TypeScript Total** | | **21** | **PASS** |
| **Combined Conformance** | | **66** | **ALL PASS** |

## Key Runtime Behaviors Verified

1. **Native Route Before JavaScript (RUN-002)**: `/health/live` served natively (logged as `stage: native`, `x-velqu-stage: native` header).
2. **Handle Invalidation on Settle (SEC-003)**: Retained handle wrappers throw `Error: request handle expired` when accessed after response settlement.
3. **Cancellation & Timers (RUN-006)**: Client disconnect aborts pending timers and settles request slots cleanly.
4. **Error Redaction (RUN-007)**: `throw new Error("secret-boom")` produces a generic 500 RFC 9457 response with zero secret leakage; origin file & line logged to server stderr via embedded source map.
5. **Limits Admission (RUN-005)**: Header blocks > 32 KiB return 431, bodies > 65,536 B return 413, queue saturation returns 503.
