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

| Class | Route | p50 (µs) | p95 (µs) | p99 (µs) | Throughput (ops/s) |
|---|---|---:|---:|---:|---:|
| **C0** | `health.live` | 34.0 | 48.0 | 58.0 | 28,450 |
| **C1** | `js.text` | 42.0 | 61.0 | 79.0 | 23,120 |
| **C2** | `js.json` | 59.0 | 82.0 | 104.0 | 16,740 |
| **C3** | `hello.get` | 48.0 | 71.0 | 92.0 | 20,410 |
| **C4** | `users.get` | 64.0 | 95.0 | 125.0 | 15,280 |

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

1. **Steady-State JS Handler Floor (C2 `/js-json`)**:
   - Velqu's steady p50 is 59 µs vs. Elysia 2's steady p50 of 37 µs.
   - **Velqu is 1.59× slower** than Elysia 2 on warmed JS JSON handler execution. This reflects the QuickJS bytecode interpreter tax compared to JIT-compiled JavaScript.
2. **Behind Raw Rust from Request 1**:
   - Raw Rust without QuickJS overtakes Velqu immediately in C2 ($N^* = 1$) and maintains lower steady floor latencies across native endpoints. Velqu is an application runtime with an embedded engine, not bare transport.
3. **High-CPU Workloads**:
   - For compute-bound algorithms (matrix math, cryptographic loops in JS), JIT runtimes will decisively outperform QuickJS. Velqu is optimized for I/O-bound web microservices, not numerical computing.

---

## 6. Cost-Normalized Metrics & Memory Footprint

| Runtime Candidate | Idle RSS (MiB) | Peak RSS under Load (MiB) | Steady Latency (p50) |
|---|---:|---:|---:|
| **Velqu (QuickJS-NG)** | **5.6** | **6.4** | **59 µs** |
| **Raw Rust** | 3.4 | 4.2 | 45 µs |
| **Raw Bun** | 28.5 | 42.1 | 41 µs |
| **Elysia 2 (Bun)** | 34.2 | 48.6 | 37 µs |
| **Node.js (Fastify)** | 38.0 | 55.4 | 68 µs |

Velqu trades raw compute peak throughput for an order-of-magnitude reduction in memory overhead (6.4 MiB vs ~48 MiB) and immediate deterministic readiness (< 5 ms).

---

## Limitations & Invariants

- **Non-SLA**: Velqu `0.1.0-beta.1` is a public beta release carrying no SLA or production-readiness promise.
- **Fixture-Specific**: All reported metrics apply strictly to the committed benchmarks on Linux x86_64 glibc.
- **No Native Direct TLS**: Production deployment requires a reverse proxy (e.g. Nginx, Envoy) for TLS termination.
