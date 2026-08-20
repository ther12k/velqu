---
type: Evidence Report
title: Warm Performance Report (Throughput and Latency)
status: in_progress
milestone: M1–M2.3
---

# Warm performance report

Current gate source: `benchmarks/raw/warm/summary.json`; raw JSONL: `benchmarks/raw/warm/g0-warm-1787214167.jsonl`. The run uses 1s cells, concurrency 1, 10, 50, 5 independent randomized repetitions, 240 raw cells, and 0 errors.
Environment: 13th Gen Intel Core i5-13420H, Linux 7.0.0-28-generic x86_64. Release builds. Logging disabled across all candidates.

## Current repeated-run evidence

The table reports the median across repetitions for each candidate/route/concurrency cell. The one-second cells are protocol evidence and should not be treated as a replacement for a longer steady-state benchmark.

| Candidate | Route | c | median p50 (μs) | median p95 (μs) | median p99 (μs) | errors |
|---|---|---:|---:|---:|---:|---:|
| elysia2 | C0 | 1 | 55.1 | 160.5 | 424.7 | 0 |
| elysia2 | C0 | 10 | 83.8 | 302.2 | 510.5 | 0 |
| elysia2 | C0 | 50 | 367.7 | 1014.9 | 1533.1 | 0 |
| elysia2 | C1 | 1 | 41.2 | 134.6 | 310.0 | 0 |
| elysia2 | C1 | 10 | 74.5 | 208.7 | 425.0 | 0 |
| elysia2 | C1 | 50 | 358.9 | 987.8 | 1582.1 | 0 |
| elysia2 | C2 | 1 | 34.7 | 132.8 | 292.7 | 0 |
| elysia2 | C2 | 10 | 80.7 | 292.4 | 505.3 | 0 |
| elysia2 | C2 | 50 | 358.0 | 979.4 | 1643.9 | 0 |
| elysia2 | C3 | 1 | 45.0 | 149.0 | 305.6 | 0 |
| elysia2 | C3 | 10 | 84.5 | 277.3 | 543.2 | 0 |
| elysia2 | C3 | 50 | 363.2 | 1010.0 | 1671.2 | 0 |
| raw-bun | C0 | 1 | 51.1 | 138.4 | 315.6 | 0 |
| raw-bun | C0 | 10 | 81.0 | 263.8 | 438.9 | 0 |
| raw-bun | C0 | 50 | 375.1 | 1008.9 | 1618.3 | 0 |
| raw-bun | C1 | 1 | 33.7 | 116.6 | 218.9 | 0 |
| raw-bun | C1 | 10 | 74.4 | 177.0 | 372.4 | 0 |
| raw-bun | C1 | 50 | 356.2 | 950.8 | 1397.2 | 0 |
| raw-bun | C2 | 1 | 39.3 | 109.5 | 270.3 | 0 |
| raw-bun | C2 | 10 | 80.9 | 289.8 | 529.1 | 0 |
| raw-bun | C2 | 50 | 371.7 | 1048.4 | 1824.1 | 0 |
| raw-bun | C3 | 1 | 33.9 | 120.9 | 237.7 | 0 |
| raw-bun | C3 | 10 | 81.2 | 167.9 | 376.7 | 0 |
| raw-bun | C3 | 50 | 354.8 | 1032.0 | 1634.9 | 0 |
| raw-rust | C0 | 1 | 27.9 | 67.5 | 111.8 | 0 |
| raw-rust | C0 | 10 | 77.5 | 226.8 | 331.8 | 0 |
| raw-rust | C0 | 50 | 343.9 | 736.9 | 1003.1 | 0 |
| raw-rust | C1 | 1 | 32.0 | 96.3 | 180.3 | 0 |
| raw-rust | C1 | 10 | 79.5 | 247.0 | 330.6 | 0 |
| raw-rust | C1 | 50 | 353.1 | 707.7 | 1010.2 | 0 |
| raw-rust | C2 | 1 | 34.8 | 86.4 | 180.9 | 0 |
| raw-rust | C2 | 10 | 80.6 | 259.9 | 383.1 | 0 |
| raw-rust | C2 | 50 | 343.0 | 745.0 | 925.0 | 0 |
| raw-rust | C3 | 1 | 35.6 | 78.1 | 125.7 | 0 |
| raw-rust | C3 | 10 | 95.2 | 260.6 | 366.1 | 0 |
| raw-rust | C3 | 50 | 343.8 | 660.5 | 905.9 | 0 |
| velqu | C0 | 1 | 28.4 | 75.6 | 123.0 | 0 |
| velqu | C0 | 10 | 85.8 | 217.4 | 342.8 | 0 |
| velqu | C0 | 50 | 326.5 | 776.7 | 1041.7 | 0 |
| velqu | C1 | 1 | 140.9 | 290.3 | 582.9 | 0 |
| velqu | C1 | 10 | 170.4 | 268.9 | 430.2 | 0 |
| velqu | C1 | 50 | 710.9 | 1266.8 | 1613.0 | 0 |
| velqu | C2 | 1 | 37.7 | 121.2 | 214.5 | 0 |
| velqu | C2 | 10 | 78.3 | 213.4 | 327.7 | 0 |
| velqu | C2 | 50 | 315.0 | 745.9 | 945.6 | 0 |
| velqu | C3 | 1 | 113.1 | 276.2 | 600.3 | 0 |
| velqu | C3 | 10 | 161.1 | 275.3 | 451.3 | 0 |
| velqu | C3 | 50 | 691.6 | 1199.3 | 1541.8 | 0 |

## Historical 10-second comparison

The following figures are retained as historical context from the prior single-pass 10-second run. They are not the current five-repetition gate measurements:

| Candidate | C0 c=10 | C1 c=10 | C2 c=10 | C3 c=10 |
|---|---:|---:|---:|---:|
| velqu | 125,185 req/s | 62,381 req/s | 60,231 req/s | 58,857 req/s |
| raw-rust (prebuilt) | 95,801 req/s | 102,265 req/s | 104,399 req/s | 91,990 req/s |
| raw-bun | 80,132 req/s | 97,322 req/s | 96,746 req/s | 92,672 req/s |
| elysia2 AOT | 72,049 req/s | 80,810 req/s | 81,632 req/s | 48,294 req/s |

## Architecture and scope

Velqu executes on exactly one QuickJS worker for this milestone; multi-worker scaling is scheduled for M3. The repeated run reported zero errors across all cells. These measurements describe only this host, pinned versions, release builds, loopback HTTP/1.1, and the frozen fixture workloads. G0 remains IN_PROGRESS because allocation counters are unavailable and the release packet is not yet generated.
