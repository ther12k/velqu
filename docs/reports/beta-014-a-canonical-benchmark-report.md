# BETA-014-A — Canonical Beta Benchmark Report

## Overview

This report provides the canonical, honest benchmark comparison for Velqu public beta (`0.1.0-beta.1`), adhering to the core principle that performance claims must be fixture-specific, fully grounded in raw measured data, and include Velqu losses alongside wins.

Every number below traces to committed raw artifacts indexed in `benchmarks/manifest.json`.

---

## 1. Cold-Start Categories (Process Exec → First Valid Response)

Raw data: `benchmarks/raw/cold-start/g0-cold-1787214119.jsonl` and `benchmarks/raw/cold-start/summary.json`.  
Executed on Linux x86_64 reference host (5 fresh-process samples per class, sequential c=1, loopback).

| Class | Route / Workload | p50 (ms) | p95 (ms) | p99 (ms) | Failures | Notes |
|---|---|---:|---:|---:|---:|---|
| **C0** | `health.live` (Native liveness) | 4.42 | 4.78 | 4.78 | 0 | Native Rust dispatch, zero JS execution |
| **C1** | `js.text` (JS plaintext) | 4.49 | 4.68 | 4.68 | 0 | QuickJS handler return string |
| **C2** | `js.json` (JS JSON response) | 4.17 | 4.87 | 4.87 | 0 | Handler object serialization |
| **C3** | `hello.get` (Validated path param) | 4.99 | 5.18 | 5.18 | 0 | Native route parameter extraction + JS |
| **C3b**| `users.create` (Body validation) | 4.91 | 5.37 | 5.37 | 0 | Ingress JSON schema check + handler |
| **C4** | `users.get` (Policy + validation) | 5.03 | 5.65 | 5.65 | 0 | Session policy + path validation |
| **C5** | `async.timer` (Async timer I/O) | 15.54 | 15.89 | 15.89 | 0 | 10 ms native timer delay + resolution |

*Guardrail Note*: These measurements reflect local process execution time (`fork/exec` to first response). They must **not** be extrapolated into cloud cold-start promises (e.g. AWS Lambda / Cloud Run container provisioning), where network virtualization, storage pulling, and container initialization dominate.

---

## 2. Warm Microbenchmarks (Steady-State Throughput & Latency)

Raw data: `benchmarks/raw/warm/g0-warm-1787214167.jsonl` and `benchmarks/raw/warm/summary.json`.

The table below reports the **median across five repetitions at concurrency 1** from the raw rows. The current warm fixture contains C0–C3 only; it does not contain a C4 `users.get` cell. RSS is the median `rssKb` for the same rows.

| Candidate | Class | p50 (µs) | p95 (µs) | p99 (µs) | Throughput (ops/s) | RSS (MiB) |
|---|---|---:|---:|---:|---:|---:|
| Velqu | C0 | 28.4 | 75.6 | 123.0 | 25,616 | 6.34 |
| Velqu | C1 | 140.9 | 290.3 | 582.9 | 6,368 | 6.50 |
| Velqu | C2 | 37.7 | 121.2 | 214.5 | 17,648 | 6.40 |
| Velqu | C3 | 113.1 | 276.2 | 600.3 | 6,748 | 6.46 |
| Raw Rust | C0 | 27.9 | 67.5 | 111.8 | 26,920 | 3.38 |
| Raw Rust | C1 | 32.0 | 96.3 | 180.3 | 22,514 | 3.44 |
| Raw Rust | C2 | 34.8 | 86.4 | 180.9 | 21,915 | 3.46 |
| Raw Rust | C3 | 35.6 | 78.1 | 125.7 | 22,705 | 3.39 |
| Raw Bun | C0 | 51.1 | 138.4 | 315.6 | 15,178 | 53.32 |
| Raw Bun | C1 | 33.7 | 116.6 | 218.9 | 19,002 | 50.90 |
| Raw Bun | C2 | 39.3 | 109.5 | 270.3 | 17,977 | 54.01 |
| Raw Bun | C3 | 33.9 | 120.9 | 237.7 | 17,513 | 55.87 |
| Elysia 2 | C0 | 55.1 | 160.5 | 424.7 | 13,352 | 89.88 |
| Elysia 2 | C1 | 41.2 | 134.6 | 310.0 | 16,096 | 93.94 |
| Elysia 2 | C2 | 34.7 | 132.8 | 292.7 | 17,511 | 93.59 |
| Elysia 2 | C3 | 45.0 | 149.0 | 305.6 | 15,300 | 110.46 |

These values are fixture-specific and are not a universal ranking. Candidate order was randomized; the run used Bun 1.4.0, five repetitions, one-second cells, and concurrency levels 1, 10, and 50.

---

## 3. Real-World I/O and Subsystems (DB, Auth, Multi-Worker)

Data sourced from `benchmarks/raw/worker-scaling/soak-summary.json` and `docs/reports/m3-010-a-soak.md`:
- **Workload Mix**: 60% light + 25% CPU + 15% controlled 1 ms native timer I/O.
- **Sustained Multi-Worker Throughput**: 2,448 to 2,672 ops/s across 2 workers under continuous closed-loop load.
- **Reliability & Settlement**: 2,407,340 requests completed and verified (100% of admitted slots settled).
- **Postgres Capability**: zero-leak connection pooling with cancellation on timeout.
- **JWT Auth**: 5 fail-closed verification gates with timing-safe HMAC-SHA-256 verification.

---

## 4. CPU/JIT Crossover Analysis

Raw data: `benchmarks/raw/ramp/` and `docs/reports/beta-003-a-crossover-matrices.md`.

In low-request or bursty scenarios, Velqu's near-zero startup overhead yields cumulative latency advantages over JIT runtimes (which pay 3.3–14.8 ms JIT compilation/warmup debt). However:
- As request volume increases on CPU-intensive logic, JIT-optimized machine code amortizes the initial warmup penalty.
- The crossover point ($N^*$) defines where JIT cumulative execution time overtakes Velqu.
- In pure CPU calculation loops, JIT compilation (V8 / JavaScriptCore) achieves superior per-request instruction throughput.

---

## 5. Honest Losses (Where Velqu Loses)

Sourced mechanically from `docs/reports/beta-003-d-honest-losses.md`:

1. **Steady-State Native Health Floor (C0)**:
   - In the ramp fixture, Velqu's steady p50 is 55 µs versus the class-best 24 µs (`raw-rust`), or **2.29× the class best**.
   - This is a serving-floor comparison, not a universal runtime ranking.
2. **No Overtake of Raw Rust in the Recorded Horizon**:
   - In the ramp fixture, Velqu never overtook `raw-rust` within the recorded 100-request horizon for C0 or C2.
   - Velqu is an application runtime with an embedded engine, not bare transport.
3. **High-CPU Workloads**:
   - For compute-bound algorithms (matrix math, cryptographic loops in JS), JIT runtimes may achieve higher per-request instruction throughput. Velqu is optimized for I/O-bound web microservices, not numerical computing.

The prior BETA-003-D report contains additional historical loss rows; this canonical report uses the current `ramp-1788451334621` loss artifact and does not carry unsupported C2 `59 µs vs 37 µs` or `$N^*=1` claims.

---

## 6. Cost-Normalized Metrics & Memory Footprint

The warm fixture records per-cell RSS rather than a separate idle/peak experiment. The table below uses the **median RSS at concurrency 1 across five repetitions** for each candidate's C0–C3 cells (see the raw warm summary); it must not be read as a memory ceiling.

| Candidate | Median RSS across C0–C3 (MiB) | C0 p50 (µs) | C2 p50 (µs) |
|---|---:|---:|---:|
| **Velqu (QuickJS-NG)** | 6.45 | 28.4 | 37.7 |
| **Raw Rust** | 3.42 | 27.9 | 34.8 |
| **Raw Bun** | 53.52 | 51.1 | 39.3 |
| **Elysia 2 (Bun)** | 96.85 | 55.1 | 34.7 |

These are measured fixture RSS snapshots, not cost or capacity predictions. A cost-normalized rate requires a separately pinned pricing model and is intentionally not claimed here. Velqu's trade-off is visible in this fixture: lower measured RSS than the Bun-based candidates, while raw Rust has the lowest footprint and warmed JIT candidates can have lower handler p50 on some cells.

---

## Limitations & Invariants

- **Non-SLA**: Velqu `0.1.0-beta.1` is a public beta release carrying no SLA or production-readiness promise.
- **Fixture-Specific**: All reported metrics apply strictly to the committed benchmarks on Linux x86_64 glibc.
- **No Native Direct TLS**: Production deployment requires a reverse proxy (e.g. Nginx, Envoy) for TLS termination.
- **No cost claim**: No cloud or host pricing model is included; memory and latency figures are measurements, not cost-normalized capacity predictions.
- **Candidate coverage**: The current warm fixture contains Velqu, raw Rust, raw Bun, and Elysia 2 at C0–C3. Fastify is pinned for the real-world matrix but has no row in this warm fixture, so this report makes no Fastify warm-latency claim.
