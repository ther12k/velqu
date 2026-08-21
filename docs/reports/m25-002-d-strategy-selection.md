---
type: Evidence Report
title: Codec Strategy Selection (M25-002-D)
status: complete
milestone: M25
---

# Codec Strategy Selection — M25-002-D

This report formalizes the compiler's deterministic strategy selection rules
derived from the empirical benchmark evidence gathered across M25-002-A,
M25-002-B, and M25-002-C.

## 1. Executive Summary and Principles

1. **No Single Strategy Is Forced Globally**: The compiler analyzes each route's
   input schemas (`body`, `query`, `params`) and output response schemas
   independently. Strategy decisions are computed per-route and per-response
   status code.
2. **Evidence-Driven Defaults**:
   - Representable Schema IR v2 inputs default to **native validation**
     (`validationStrategy: "native"`).
   - Structured JSON responses default to **native response serialization**
     (`responseStrategy: "native"`).
3. **Explicit Fallback Paths**: Schemas containing fallback markers (`s_fallback`),
   unsupported transforms (`transform`), or unrepresentable constructs
   deterministically route to the QuickJS engine (`validationStrategy: "js"` /
   `responseStrategy: "js"`) with closed, validated reasons (`unsupported-transform`,
   `unrepresentable`, `measured`, `explicit`).
4. **Inspect-Visible Fallback Cost**: Fallback locations and their measured
   overhead (+latency, +allocations) are recorded in `build-report.json` and
   surfaced directly via `velqu inspect fallbacks`.

---

## 2. Empirical Evidence Base

Strategy decisions synthesize the reproducible benchmark runs from M25-002-A,
M25-002-B, and M25-002-C (60,000 samples per run, 30 cells, 2,000/2,000
correct each, 13th Gen Intel Core i5-13420H, quickjs-ng 0.15.1):

### A. Latency Comparison (μs, p50 / p99)

| Shape | Bytes | QuickJS-JSON p50 | Native (Generic) p50 | Native (Generated) p50 | Native vs QuickJS |
| --- | ---: | ---: | ---: | ---: | ---: |
| `small_user` | 75 | 30.1 / 109.7 | 22.7 / 80.1 | 21.3 / 102.4 | **−29% latency** |
| `nested_order` | 106 | 36.2 / 259.1 | 28.9 / 101.8 | 43.1 / 106.1 | **−20% p50 / −59% p99** |
| `pad_256` | 244 | 43.4 / 115.4 | 21.1 / 92.5 | 28.3 / 138.0 | **−51% latency** |
| `pad_1k` | 1,012 | 45.5 / 320.2 | 27.4 / 120.1 | 22.2 / 224.3 | **−51% latency** |
| `pad_16k` | 16,372 | 85.3 / 489.9 | 38.5 / 137.5 | 44.9 / 165.8 | **−55% latency** |
| `pad_64k` | 65,524 | 277.8 / 911.9 | 70.7 / 314.1 | 45.5 / 138.0 | **−84% latency (6x faster)** |
| `opt_null` | 49 | 37.2 / 282.3 | 21.6 / 102.5 | 37.5 / 159.3 | **−42% latency** |
| `problem_shape` | 156 | 27.5 / 165.0 | 27.7 / 131.5 | 25.1 / 151.3 | **−9% latency / −8% p99** |
| `records100` | 4,976 | 295.1 / 1,186.8 | 327.3 / 1,064.9 | 360.9 / 1,067.0 | +10% (no validation in QuickJS) |
| `records1000` | 52,726 | 2,635.4 / 6,001.8 | 3,231.7 / 5,707.2 | 3,004.1 / 8,309.9 | +14% (no validation in QuickJS) |

### B. Bridge and Allocation Dynamics

- **Bridge Crossing Cost**: `quickjs-json` requires a lazy `ctx.json()` body
  materialization callback over the host bridge on every request. Bridge access
  alone costs 0.13 µs at 75 B, 21.0 µs at 65 KB, and 67.5 µs at 52 KB
  (`records1000`). Host-validated native bodies cross with **0 host bridge calls**
  during handler execution because the pre-validated body is injected directly.
- **Allocation Profile**: Padded scalar payloads allocate ~272 KB requested
  allocation bytes at 64 KB across both strategies (dominated by the JSON parse
  tree). On large record arrays, host candidates allocate ~1.89 MB per sample
  versus ~1.32 MB for engine JSON.parse due to intermediate AST construction.

---

## 3. Decision Matrix

| Input / Response Characteristic | Selected Strategy | Rationale & Evidence |
|---|---|---|
| **Standard Representable Object / Scalar** | `native` | 20–50% lower p50, 2–6x faster on medium/large bodies, zero runtime bridge calls. |
| **Padded / Large Payloads (16 KB – 64 KB)** | `native` | Native parsing and validation scales significantly better (45.5 µs vs 277.8 µs at 64 KB). |
| **Optional / Null-Heavy Shapes** | `native` | Correctness-enforced default injection and null normalization runs efficiently in native host (21.6 µs p50). |
| **Large Arrays (>1000 items) with unvalidated pass-through** | `js` (via `s_fallback("measured")`) | When schema validation is intentionally bypassed, engine JSON.parse saves ~400 µs and ~500 KB AST allocations. |
| **Unsupported Transform (`s_transform`)** | `js` (fallback: `unsupported-transform`) | No native transform codec available in M25-002; explicit fallback prevents runtime failure. |
| **Unrepresentable Schema Constructs** | `js` (fallback: `unrepresentable`) | Dynamic or non-serializable shapes route explicitly to QuickJS. |
| **Explicit Developer Override** | `js` (fallback: `explicit`) | Honor explicit developer escape hatch without silent downgrade. |
| **Structured JSON Response (`200`, `201`, etc.)** | `native` | Native traversal eliminates engine `JSON.stringify` jitter and avoids JS bridge callbacks. |
| **Raw Text / Binary Response** | `js` / `native-liveness` | Static or raw payloads use dedicated native liveness or direct text/bytes. |

---

## 4. Implementation Details

1. **Compiler Strategy Module (`packages/compiler/src/strategy.ts`)**:
   - `selectRouteStrategies(route: RouteInfo)` evaluates `bodyIr`, `queryIr`,
     `paramsIr`, and `responses` to determine `validationStrategy` and
     `responseStrategies`.
   - `evaluateAppStrategies(routes: RouteInfo[])` aggregates decisions, computes
     overall distribution (`native`, `hybrid`, `js`), and builds the structured
     report payload.
2. **Pack Emission (`packages/compiler/src/emit.ts`)**:
   - Emits `validationStrategy` per route and sets `plan.responseStrategy` on the
     compiled RoutePlan.
   - Emits individual response status strategies under `responses[status].strategy`.
3. **CLI Inspection (`packages/cli/src/index.ts`)**:
   - `velqu inspect routes` displays `val=<strategy>` and `resp=<strategy>` per route.
   - `velqu inspect fallbacks` reports active fallbacks, reasons, and estimated
     overhead (+µs latency, +bytes allocations).

---

## 5. Artifact Hashes and References

| Evidence Artifact | SHA-256 |
| --- | --- |
| `benchmarks/raw/codec/evidence.json` (M25-002-B) | `c20e23fc77eb3f952019dd92214ec143900fdf9bd4264fa2fbbdc8f1cd7e549f` |
| `benchmarks/raw/codec-c/evidence.json` (M25-002-C) | `09a4cb9f8a1abfb34f7e7b869acb913658e6e5e544eb1cd8c6d01d9dd4ca26d0` |
| `docs/reports/m25-002-a-strategy-comparison.md` | `9846feea00713b194f4c28f731a5ebbcda07d7274092b77c5c2d334547963fc2` |
| `docs/reports/m25-002-b-payload-matrix.md` | `a90623d38692dafb5e44cb89d98cb83ef282f6460619a9244ee1d674b62bfd7c` |
| `docs/reports/m25-002-c-cpu-allocation-bridge-tails.md` | `a45cbb668bc5f22e703957a41261d76378e9bce6f9037c8651a029cf05d658c1` |
