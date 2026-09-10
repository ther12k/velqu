---
type: Evidence Report
title: Cold-Start Report (process → first valid response)
status: in_progress
milestone: M0–M2.3
---

# Cold-start report

## Current gate evidence

Run `cold-1789054622405` is a Velqu-only gate run with 60 fresh-process samples per class (28 cells, 1680 rows, zero failures/timeouts). Raw JSONL: `benchmarks/raw/cold-start/cold-1789054622405.jsonl`.

| Class | Route | p50 total (ms) | p95 total (ms) | p99 total (ms) | failures |
|---|---|---:|---:|---:|---:|
| C0 | health.live | 6.609 | 9.329 | 10.868 | 0 |
| C1 | js.text | 6.855 | 11.512 | 14.672 | 0 |
| C2 | js.json | 6.681 | 9.938 | 11.194 | 0 |
| C3 | hello.get | 6.664 | 11.863 | 12.736 | 0 |
| C3b | users.create | 6.441 | 13.604 | 14.736 | 0 |
| C4 | users.get | 7.522 | 11.685 | 14.782 | 0 |
| C5 | async.timer | 18.121 | 21.205 | 21.878 | 0 |
| C0 | health.live | 2.357 | 4.062 | 6.844 | 0 |
| C1 | js.text | 2.240 | 3.711 | 4.364 | 0 |
| C2 | js.json | 2.475 | 3.261 | 4.234 | 0 |
| C3 | hello.get | 2.373 | 3.476 | 3.953 | 0 |
| C3b | users.create | 2.402 | 4.234 | 5.559 | 0 |
| C4 | users.get | 2.401 | 3.764 | 4.462 | 0 |
| C5 | async.timer | 13.679 | 15.027 | 15.581 | 0 |
| C0 | health.live | 9.958 | 15.747 | 17.180 | 0 |
| C1 | js.text | 9.191 | 12.886 | 17.429 | 0 |
| C2 | js.json | 9.439 | 16.901 | 21.560 | 0 |
| C3 | hello.get | 10.014 | 19.688 | 23.172 | 0 |
| C3b | users.create | 10.774 | 16.770 | 20.793 | 0 |
| C4 | users.get | 10.248 | 17.694 | 20.655 | 0 |
| C5 | async.timer | 19.811 | 27.264 | 28.937 | 0 |
| C0 | health.live | 108.117 | 157.443 | 177.255 | 0 |
| C1 | js.text | 101.150 | 137.201 | 142.953 | 0 |
| C2 | js.json | 106.241 | 189.213 | 198.947 | 0 |
| C3 | hello.get | 146.741 | 194.017 | 210.490 | 0 |
| C3b | users.create | 145.437 | 229.361 | 234.076 | 0 |
| C4 | users.get | 153.773 | 198.361 | 230.871 | 0 |
| C5 | async.timer | 163.229 | 228.887 | 244.793 | 0 |

This run is gate evidence for repeatability and correctness, not a fresh competitor cold-start comparison.

## Startup profile

The 10,000-route startup profile is recorded at `benchmarks/raw/profiles/startup-10000.json`. The generated fixture contains 10,001 routes because it retains the health route plus 10,000 generated routes. The ready-line-bounded capture reports 0.0 ms total: pack.load 0.0 ms, serialized router load 0.0 ms, engine.spawn 0.000 ms, bundle.load 0.0 ms, and listen 0.000 ms. Allocator instrumentation captured 7755106 mallocs, 1 callocs, 280332 reallocs, and 7755108 frees. Linux `perf` counters were unavailable because the host sets `perf_event_paranoid=4`; allocator counts are startup instrumentation, not a general allocator benchmark.

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

The route-count suite uses 40 fresh processes per cell, randomized candidate/size order (seed 3881366700, run route-count-1789054694139), and reports failures per cell. Raw and summary artifacts: `benchmarks/raw/route-count/route-count-1789054919962.jsonl` and `benchmarks/raw/route-count/summary.json`.

| Candidate | 25 routes p50 | 100 routes p50 | 1,000 routes p50 | 5,000 routes p50 | 10,000 routes p50 | 10,000 p95 | 10,000 RSS |
|---|---:|---:|---:|---:|---:|---:|---:|
| velqu (bytecode) | 7.87ms | 15.351ms | 113.791ms | 584.054ms | 1281.557ms | 1507.425ms | 306.1 MB |
| velqu (source) | 9.061ms | 16.506ms | 121.678ms | 590.901ms | 1307.276ms | 1650.327ms | 306.5 MB |
| raw-bun | 9.984ms | 9.043ms | 8.596ms | 9.082ms | 9.474ms | 20.193ms | 19.5 MB |
| elysia2 | 151.82ms | 174.944ms | 191.962ms | 284.447ms | 401.954ms | 527.404ms | 95.9 MB |

These are observations for this host and fixture, not universal performance claims. Binary QPack v2 remains the planned lever for reducing JSON-pack parsing cost.

## Scope

These numbers describe only this host, pinned versions, release builds, loopback HTTP/1.1, and the frozen fixture workloads. G0 remains IN_PROGRESS while allocation profiling, report parity automation, and the commit-bound release packet are completed.
