---
type: Evidence Report
title: Warm Performance Report (Throughput and Latency)
status: in_progress
milestone: M1–M2.3
---

# Warm performance report

Current gate source: `benchmarks/raw/warm/summary.json`; raw JSONL: `benchmarks/raw/warm/warm-1789054920075.jsonl`. The run uses 10s cells, concurrency 1, 10, 50, 5 independent randomized repetitions, 240 raw cells, and 0 errors.
Environment: 13th Gen Intel Core i5-13420H, Linux 7.0.0-28-generic x86_64. Release builds. Logging disabled across all candidates.

## Current repeated-run evidence

The table reports the median across repetitions for each candidate/route/concurrency cell. The one-second cells are protocol evidence and should not be treated as a replacement for a longer steady-state benchmark.

| Candidate | Route | c | median p50 (μs) | median p95 (μs) | median p99 (μs) | errors |
|---|---|---:|---:|---:|---:|---:|
| elysia2 | C0 | 1 | 63.8 | 269.5 | 449.5 | 0 |
| elysia2 | C0 | 10 | 175.0 | 481.4 | 1002.9 | 0 |
| elysia2 | C0 | 50 | 909.0 | 2119.3 | 3461.3 | 0 |
| elysia2 | C1 | 1 | 61.8 | 251.0 | 470.0 | 0 |
| elysia2 | C1 | 10 | 184.1 | 543.6 | 1119.0 | 0 |
| elysia2 | C1 | 50 | 666.8 | 1801.6 | 3052.2 | 0 |
| elysia2 | C2 | 1 | 54.9 | 195.4 | 399.2 | 0 |
| elysia2 | C2 | 10 | 163.5 | 502.0 | 926.4 | 0 |
| elysia2 | C2 | 50 | 919.7 | 2113.7 | 3537.9 | 0 |
| elysia2 | C3 | 1 | 64.3 | 270.8 | 508.6 | 0 |
| elysia2 | C3 | 10 | 155.7 | 505.4 | 1059.9 | 0 |
| elysia2 | C3 | 50 | 875.0 | 2080.1 | 3325.3 | 0 |
| raw-bun | C0 | 1 | 67.6 | 268.5 | 531.5 | 0 |
| raw-bun | C0 | 10 | 179.0 | 431.7 | 1022.5 | 0 |
| raw-bun | C0 | 50 | 800.9 | 1724.8 | 2685.5 | 0 |
| raw-bun | C1 | 1 | 45.6 | 220.1 | 399.2 | 0 |
| raw-bun | C1 | 10 | 158.0 | 455.3 | 964.6 | 0 |
| raw-bun | C1 | 50 | 946.3 | 2279.2 | 3277.9 | 0 |
| raw-bun | C2 | 1 | 57.5 | 232.2 | 399.6 | 0 |
| raw-bun | C2 | 10 | 151.5 | 416.3 | 801.1 | 0 |
| raw-bun | C2 | 50 | 883.6 | 2156.3 | 3220.8 | 0 |
| raw-bun | C3 | 1 | 59.7 | 220.7 | 451.4 | 0 |
| raw-bun | C3 | 10 | 194.8 | 580.2 | 1117.0 | 0 |
| raw-bun | C3 | 50 | 1088.9 | 2551.9 | 3646.1 | 0 |
| raw-rust | C0 | 1 | 47.5 | 161.5 | 341.3 | 0 |
| raw-rust | C0 | 10 | 158.0 | 428.0 | 1088.5 | 0 |
| raw-rust | C0 | 50 | 490.8 | 1209.7 | 2083.2 | 0 |
| raw-rust | C1 | 1 | 56.0 | 181.3 | 524.5 | 0 |
| raw-rust | C1 | 10 | 136.6 | 392.9 | 762.0 | 0 |
| raw-rust | C1 | 50 | 595.2 | 1486.2 | 2216.8 | 0 |
| raw-rust | C2 | 1 | 41.8 | 164.3 | 329.1 | 0 |
| raw-rust | C2 | 10 | 136.2 | 397.1 | 702.6 | 0 |
| raw-rust | C2 | 50 | 623.3 | 1596.6 | 2300.6 | 0 |
| raw-rust | C3 | 1 | 47.2 | 172.1 | 329.3 | 0 |
| raw-rust | C3 | 10 | 139.6 | 395.2 | 746.9 | 0 |
| raw-rust | C3 | 50 | 548.6 | 1331.5 | 2294.1 | 0 |
| velqu | C0 | 1 | 48.3 | 154.8 | 307.1 | 0 |
| velqu | C0 | 10 | 129.5 | 330.2 | 684.4 | 0 |
| velqu | C0 | 50 | 490.9 | 1384.2 | 2213.0 | 0 |
| velqu | C1 | 1 | 103.5 | 339.3 | 645.2 | 0 |
| velqu | C1 | 10 | 181.1 | 608.7 | 1373.6 | 0 |
| velqu | C1 | 50 | 791.6 | 1776.9 | 3117.8 | 0 |
| velqu | C2 | 1 | 120.3 | 435.7 | 779.1 | 0 |
| velqu | C2 | 10 | 179.3 | 362.5 | 834.0 | 0 |
| velqu | C2 | 50 | 805.6 | 1693.1 | 3150.5 | 0 |
| velqu | C3 | 1 | 136.0 | 553.8 | 976.2 | 0 |
| velqu | C3 | 10 | 249.3 | 503.5 | 1217.4 | 0 |
| velqu | C3 | 50 | 1124.7 | 2091.1 | 3396.2 | 0 |

## Historical 10-second comparison

The following figures are retained as historical context from the prior single-pass 10-second run. They are not the current five-repetition gate measurements:

| Candidate | C0 c=10 | C1 c=10 | C2 c=10 | C3 c=10 |
|---|---:|---:|---:|---:|
| velqu | 125,185 req/s | 62,381 req/s | 60,231 req/s | 58,857 req/s |
| raw-rust (prebuilt) | 95,801 req/s | 102,265 req/s | 104,399 req/s | 91,990 req/s |
| raw-bun | 80,132 req/s | 97,322 req/s | 96,746 req/s | 92,672 req/s |
| elysia2 AOT | 72,049 req/s | 80,810 req/s | 81,632 req/s | 48,294 req/s |

## Architecture and scope

Velqu executes on exactly one QuickJS worker for this milestone; multi-worker scaling is scheduled for M3. The repeated run reported zero errors across all cells. These measurements describe only this host, pinned versions, release builds, loopback HTTP/1.1, and the frozen fixture workloads. G0 remains IN_PROGRESS until the current evidence packet is regenerated from the final clean commit.
