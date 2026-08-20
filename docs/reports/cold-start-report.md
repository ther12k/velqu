---
type: Evidence Report
title: Cold-Start Report (process → first valid response)
status: complete
milestone: M0–M2.3
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
rquickjs 0.12.2. Release builds for all candidates. 2026-08-19.

## Primary result — process-to-first-valid-response (p50 / p95, ms)

| Class | velqu | raw-rust (lower bound) | raw-bun | elysia2 AOT |
|---|---|---|---|---|
| C0: native liveness | **3.2 / 5.8** | 2.2 / 3.1 | 16.8 / 23.8 | 106.0 / 141.8 |
| C1: JS plaintext | **3.1 / 4.2** | 2.4 / 4.3 | 14.8 / 21.6 | 115.7 / 155.7 |
| C2: JS small JSON | **3.3 / 5.5** | 2.2 / 5.1 | 15.7 / 36.5 | 107.1 / 136.6 |
| C3: validated path (hello) | **3.4 / 5.0** | 2.2 / 3.3 | 14.3 / 23.2 | 150.2 / 180.2 |
| C3b: validated body (users) | **3.3 / 15.6** | 2.3 / 4.0 | 16.1 / 33.9 | 149.3 / 199.3 |
| C4: policy + validation | **3.6 / 5.6** | 2.2 / 2.8 | 16.7 / 29.1 | 144.8 / 173.3 |
| C5: async (10ms timer) | **14.6 / 18.3** | 13.5 / 15.3 | 26.3 / 49.3 | 167.4 / 192.9 |

(C5 includes the 10ms deliberate timer wait; startup itself is C0–C4-like.)

## Comparative gate (performance budgets)

> Velqu C3 and C4 p95 ≤ 60% of matched Elysia 2 AOT p95

- C3 p95: velqu 5.0ms vs elysia 180.2ms → **2.8%** — PASS (36× lower)
- C4 p95: velqu 5.6ms vs elysia 173.3ms → **3.2%** — PASS (31× lower)
- velqu C3 p95 vs raw-bun 23.2ms → 22% — PASS

## Absolute budgets (aspirational, this host)

| Budget | Target | Observed | Status |
|---|---:|---:|---|
| C0 p95 | ≤8ms | 5.8ms | PASS |
| C1 p95 | ≤12ms | 4.2ms | PASS |
| C2 p95 | ≤15ms | 5.5ms | PASS |
| C3 p95 | ≤18ms | 5.0ms | PASS |
| C4 p95 | ≤22ms | 5.6ms | PASS |
| failures/timeouts | 0 | **0 of 840** | PASS |

## Startup decomposition (velqu, from ready-line stages)

Typical: pack.load 1.2ms → router.build 0.05ms → engine.spawn 0.09ms →
bundle.load 1.1ms → listen 0.04ms; total ≈ 2.5ms. First-request adds ~0.3ms
(handler cache hit). Zero runtime route/schema/OpenAPI compilation (segments and
IR are pre-compiled in the pack).

## Route-count scaling (PERF-005) — sync handlers & bytecode scaling

Measured route `GET /res7/item/7` (`benchmarks/raw/route-count/`, 20 samples per cell, 0 failures):

| Candidate | 25 routes p50 | 1,000 routes p50 | 10,000 routes p50 | 10,000 p95 | 10,000 RSS |
|---|---:|---:|---:|---:|---:|
| velqu (source) | 3.68ms | 26.48ms | 170.20ms | 211.93ms | 85.1 MB |
| velqu (bytecode, ADR-0017) | 3.27ms | 20.84ms | 150.61ms | 211.10ms | 84.7 MB |
| raw-bun | 16.13ms | 17.49ms | 16.55ms | 19.65ms | 37.8 MB |
| elysia2 | 150.15ms | 172.82ms | 311.66ms | 365.44ms | 159.3 MB |

At 10,000 routes, velqu bytecode cold start (150.6 ms) is **2.1× faster** than the matched
Elysia candidate (311.7 ms) with **76 MB less RSS**. Startup decomposition at 10,000 routes
shows `pack.load` (JSON parse of the 17.5 MB pack) dominating at ~120 ms; `router.build`
consumes the serialized automaton in 5.8 ms with zero route parsing. Eliminating JSON pack
parsing is the M2.6 binary QPack v2 target and is the dominant remaining lever.

## Scope

These numbers describe ONLY: this host, these pinned versions, the frozen
fixture workloads (C0–C5), release builds, HTTP/1.1 no-TLS no-compression,
loopback. They are not claims about other workloads or environments.
