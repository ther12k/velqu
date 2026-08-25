---
type: Evidence Report
title: Cold-Start Report (process → first valid response)
status: in_progress
milestone: M0–M2.3
---

# Cold-start report

## Current gate evidence

Run `g0-cold-1787214119` is a Velqu-only gate run with 5 fresh-process samples per class (7 cells, 35 rows, zero failures/timeouts). Raw JSONL: `benchmarks/raw/cold-start/g0-cold-1787214119.jsonl`.

| Class | Route | p50 total (ms) | p95 total (ms) | p99 total (ms) | failures |
|---|---|---:|---:|---:|---:|
| C0 | health.live | 4.424 | 4.775 | 4.775 | 0 |
| C1 | js.text | 4.490 | 4.683 | 4.683 | 0 |
| C2 | js.json | 4.173 | 4.870 | 4.870 | 0 |
| C3 | hello.get | 4.994 | 5.182 | 5.182 | 0 |
| C3b | users.create | 4.912 | 5.374 | 5.374 | 0 |
| C4 | users.get | 5.027 | 5.653 | 5.653 | 0 |
| C5 | async.timer | 15.542 | 15.886 | 15.886 | 0 |

This run is gate evidence for repeatability and correctness, not a fresh competitor cold-start comparison.

## Startup profile

The 10,000-route startup profile is recorded at `benchmarks/raw/profiles/startup-10000.json`. The generated fixture contains 10,001 routes because it retains the health route plus 10,000 generated routes. The ready-line-bounded capture reports 433.7 ms total: pack.load 341.6 ms, serialized router load 6.8 ms, engine.spawn 0.027 ms, bundle.load 85.0 ms, and listen 0.058 ms. Allocator instrumentation captured 1197 mallocs, 288 callocs, 193 reallocs, and 1843 frees. Linux `perf` counters were unavailable because the host sets `perf_event_paranoid=4`; allocator counts are startup instrumentation, not a general allocator benchmark.

## Historical competitor comparison

The following earlier comparison remains historical context only and is not part of the current repeated gate run. It must not be read as a fresh competitor sample set:

| Class | Velqu p95 (ms) | Raw Rust p95 (ms) | Raw Bun p95 (ms) | Elysia 2 AOT p95 (ms) |
|---|---:|---:|---:|---:|
| C0 native liveness | 5.8 | 3.1 | 23.8 | 141.8 |
| C1 JS plaintext | 4.2 | 4.3 | 21.6 | 155.7 |
| C2 JS small JSON | 5.5 | 5.1 | 36.5 | 136.6 |
| C3 validated path | 5.0 | 3.3 | 23.2 | 180.2 |
| C4 policy + validation | 5.6 | 2.8 | 29.1 | 173.3 |

## Route-count scaling

The route-count suite uses 100 fresh processes per cell, randomized candidate/size order (seed 823816920, run m26-010-c), and reports failures per cell. Raw and summary artifacts: `benchmarks/raw/route-count/route-count-1787684273631.jsonl` and `benchmarks/raw/route-count/summary.json`.

| Candidate | 25 routes p50 | 100 routes p50 | 1,000 routes p50 | 5,000 routes p50 | 10,000 routes p50 | 10,000 p95 | 10,000 RSS |
|---|---:|---:|---:|---:|---:|---:|---:|
| velqu (source) | 6.44ms | 12.548ms | 83.845ms | 436.867ms | 948.156ms | 983.084ms | 305.3 MB |
| raw-bun | 7.868ms | 7.87ms | 8.458ms | 8.131ms | 7.766ms | 9.808ms | 19.7 MB |
| elysia2 | 109.644ms | 111.921ms | 129.065ms | 185.441ms | 251.51ms | 278.821ms | 96.9 MB |
| velqu (bytecode) | 5.952ms | 12.286ms | 82.693ms | 431.309ms | 926.173ms | 963.91ms | 304.8 MB |

These are observations for this host and fixture, not universal performance claims. Binary QPack v2 remains the planned lever for reducing JSON-pack parsing cost.

## Scope

These numbers describe only this host, pinned versions, release builds, loopback HTTP/1.1, and the frozen fixture workloads. G0 remains IN_PROGRESS while allocation profiling, report parity automation, and the commit-bound release packet are completed.
