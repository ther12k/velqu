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
| C0 native liveness | **2.9 / 4.8** | 2.1 / 3.3 | 14.2 / 23.9 | 101.7 / 131.6 |
| C1 JS plaintext | **3.0 / 6.1** | 2.1 / 3.4 | 13.6 / 21.9 | 99.8 / 134.3 |
| C2 JS small JSON | **3.0 / 4.9** | 2.1 / 3.6 | 14.9 / 21.1 | 98.8 / 129.9 |
| C3 validated path (hello) | **3.1 / 5.1** | 2.1 / 3.3 | 13.6 / 20.4 | 137.1 / 167.7 |
| C3 validated body (users) | **3.1 / 5.0** | 2.2 / 5.4 | 14.2 / 21.9 | 141.0 / 156.6 |
| C4 policy + validation | **3.3 / 6.0** | 2.2 / 3.6 | 14.0 / 22.2 | 139.0 / 160.6 |
| C5 async (10ms timer) | 14.3 / 18.8 | 13.4 / 15.9 | 24.8 / 32.3 | 155.5 / 192.5 |

(C5 includes the 10ms deliberate timer wait; startup itself is C0–C4-like.)

## Comparative gate (performance budgets)

> Velqu C3 and C4 p95 ≤ 60% of matched Elysia 2 AOT p95

- C3 p95: velqu 5.1ms vs elysia 167.7ms → **3.0%** — PASS (33× lower)
- C4 p95: velqu 6.0ms vs elysia 160.6ms → **3.7%** — PASS (27× lower)
- velqu C3 p95 vs raw-bun 20.4ms → 25% — PASS

## Absolute budgets (aspirational, this host)

| Budget | Target | Observed | Status |
|---|---:|---:|---|
| C0 p95 | ≤8ms | 4.8ms | PASS |
| C1 p95 | ≤12ms | 6.1ms | PASS |
| C2 p95 | ≤15ms | 4.9ms | PASS |
| C3 p95 | ≤18ms | 5.1ms | PASS |
| C4 p95 | ≤22ms | 6.0ms | PASS |
| failures/timeouts | 0 | **0 of 1680** | PASS |

## Startup decomposition (velqu, from ready-line stages)

Typical: pack.load 1.2ms → router.build 0.05ms → engine.spawn 0.09ms →
bundle.load 1.1ms → listen 0.04ms; total ≈ 2.5ms. First-request adds ~0.3ms
(handler cache hit). Zero runtime route/schema/OpenAPI compilation (segments and
IR are pre-compiled in the pack).

## Route-count scaling (PERF-005) — sync handlers & bytecode scaling

Measured route `GET /res7/item/7` (`benchmarks/raw/route-count/`):

| Candidate | 25 routes p50 | 1,000 routes p50 | 1,000 routes p95 | Δ (p50) |
|---|---:|---:|---:|---:|
| velqu (source) | 3.31ms | 20.20ms | 24.47ms | +509.8% |
| velqu (bytecode, ADR-0017) | 3.21ms | 17.83ms | 22.48ms | +455.0% (−2.38ms vs source) |
| raw-bun | 14.82ms | 14.01ms | 21.92ms | −5.5% |
| elysia2 | 153.93ms | 185.07ms | 216.22ms | +20.2% |

Bytecode compilation (`velqu-bytecode embed`, ADR-0017) saves **2.38 ms**
at 1,000 routes by eliminating QuickJS source tokenization and AST parsing at load time.
Absolute velqu bytecode cold start (17.83 ms) is **~10.4× faster** than the matched Elysia
candidate (185.1 ms) and ~104 MB lighter RSS.

## Scope

These numbers describe ONLY: this host, these pinned versions, the frozen
fixture workloads (C0–C5), release builds, HTTP/1.1 no-TLS no-compression,
loopback. They are not claims about other workloads or environments.
