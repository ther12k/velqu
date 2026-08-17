---
type: Evidence Report
title: Cold-Start Report (process → first valid response)
status: complete
milestone: M0–M2
---

# Cold-start report

Protocol: parent fresh-process harness (`benchmarks/harness/cold-start.ts`):
monotonic timer before spawn → TCP-accept poll (0.5ms interval) → route-class
request → byte-exact validation (`checkFirstResponse` semantics) → terminate.
Randomized/interleaved sample order. 60 samples per candidate×class
(4 candidates × 7 classes = 1680 samples; **0 failures, 0 timeouts**).
Raw: `benchmarks/raw/cold-start/cold-*.jsonl`; summary:
`benchmarks/raw/cold-start/summary.json`.

Environment: 13th Gen Intel Core i5-13420H, Linux 7.0.0-28-generic x86_64,
Bun 1.3.4 (harness), rustc 1.96.0, elysia 2.0.0-beta.4, quickjs-ng 0.15.1 via
rquickjs 0.12.2. Release builds for all candidates. 2026-08-17.

## Primary result — process-to-first-valid-response (p50 / p95, ms)

| Class | velqu | raw-rust (lower bound) | raw-bun | elysia2 AOT |
|---|---|---|---|---|
| C0 native liveness | **2.8 / 4.4** | 2.1 / 3.1 | 14.0 / 20.3 | 96.6 / 112.8 |
| C1 JS plaintext | **2.9 / 4.8** | 2.1 / 2.9 | 14.1 / 18.5 | 96.8 / 116.8 |
| C2 JS small JSON | **3.0 / 4.2** | 2.1 / 3.0 | 14.4 / 19.8 | 94.3 / 112.3 |
| C3 validated path (hello) | **2.9 / 4.4** | 2.1 / 2.7 | 14.2 / 18.3 | 132.6 / 152.0 |
| C3 validated body (users) | **3.2 / 4.7** | 2.2 / 3.4 | 14.4 / 22.2 | 135.0 / 146.8 |
| C4 policy + validation | **3.3 / 5.0** | 2.2 / 3.8 | 14.3 / 21.2 | 133.3 / 149.9 |
| C5 async (10ms timer) | 14.4 / 15.7 | 13.4 / 15.0 | 24.8 / 30.8 | 151.0 / 163.9 |

(C5 includes the 10ms deliberate timer wait; startup itself is C0–C4-like.)

## Comparative gate (performance budgets)

> Project Q C3 and C4 p95 ≤ 60% of matched Elysia 2 AOT p95

- C3 p95: velqu 4.4ms vs elysia 152.0ms → **2.9%** — PASS (34× lower)
- C4 p95: velqu 5.0ms vs elysia 149.9ms → **3.3%** — PASS (30× lower)
- velqu C3 p95 vs raw-bun 18.3ms → 24% — PASS

## Absolute budgets (aspirational, this host)

| Budget | Target | Observed | Status |
|---|---:|---:|---|
| C0 p95 | ≤8ms | 4.4ms | PASS |
| C1 p95 | ≤12ms | 4.8ms | PASS |
| C2 p95 | ≤15ms | 4.2ms | PASS |
| C3 p95 | ≤18ms | 4.4ms | PASS |
| C4 p95 | ≤22ms | 5.0ms | PASS |
| failures/timeouts | 0 | **0 of 1680** | PASS |

## Startup decomposition (velqu, from ready-line stages)

Typical: pack.load 1.5ms → router.build 0.07ms → engine.spawn 0.1ms →
bundle.load 1.4ms → listen 0.06ms; total ≈ 3.1ms. First-request adds ~0.3ms
(handler cache hit). No runtime route/schema/OpenAPI compilation (segments and
IR are pre-compiled in the pack).

## Route-count scaling (PERF-005) — honest negative

40 samples per cell, same protocol; measured route `GET /res7/item/7`
(`benchmarks/raw/route-count/`):

| Candidate | 25 routes p50 | 1,000 routes p50 | Δ |
|---|---:|---:|---:|
| velqu | 3.08ms | 15.7ms | **+409%** (budget ≤20% — **FAIL**) |
| raw-bun | 13.0ms | 13.8ms | +6.0% |
| elysia2 | 138.0ms | 161.2ms | +16.8% |

Absolute velqu 1000-route cold start (15.7ms) is still ~10× faster than the
matched Elysia candidate and ~1.2MB RSS lighter, but the relative scaling
budget FAILS. Profile attribution: 871KB pack JSON parse + 1000-function
bundle eval dominate. Remediation hypotheses (M2+, each needs before/after
benchmark): binary/chunked pack format, bytecode (ADR-0014), lazy handler
table. Recorded as a failed budget — not silently redefined.

## Scope

These numbers describe ONLY: this host, these pinned versions, the frozen
fixture workloads (C0–C5), release builds, HTTP/1.1 no-TLS no-compression,
loopback. They are not claims about other workloads or environments.
