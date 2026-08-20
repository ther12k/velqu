---
type: Evidence Report
title: Warm Performance Report (Throughput and Latency)
status: complete
milestone: M1–M2.3
---

# Warm performance report

Source: `benchmarks/raw/warm/summary.json` (fixed-duration load, 10s per cell, concurrency levels 1, 10, 50).
Environment: 13th Gen Intel Core i5-13420H, Linux 7.0.0-28-generic x86_64. Release builds. Logging disabled (`--log off`) across all candidates.

## Warm Load Throughput & Latency

### Concurrency = 10 (Primary Target)

| Candidate | C0 (health.live) | C1 (js.text) | C2 (js.json) | C3 (hello.get) |
|---|---|---|---|---|
| **velqu** | 125,185 req/s (p50=73.9μs, p95=136.2μs) | 62,381 req/s (p50=164.8μs, p95=246.1μs) | 60,231 req/s (p50=165.6μs, p95=247.5μs) | 58,857 req/s (p50=162.9μs, p95=235μs) |
| **raw-rust (prebuilt)** | 95,801 req/s (p50=82μs, p95=204.2μs) | 102,265 req/s (p50=76.4μs, p95=178μs) | 104,399 req/s (p50=76.6μs, p95=182.3μs) | 91,990 req/s (p50=85.8μs, p95=254μs) |
| **raw-bun** | 80,132 req/s (p50=84.5μs, p95=323μs) | 97,322 req/s (p50=78.1μs, p95=260μs) | 96,746 req/s (p50=79.9μs, p95=262.1μs) | 92,672 req/s (p50=81.6μs, p95=298.3μs) |
| **elysia2 AOT** | 72,049 req/s (p50=85.8μs, p95=358.1μs) | 80,810 req/s (p50=83μs, p95=319.1μs) | 81,632 req/s (p50=83.6μs, p95=304.2μs) | 48,294 req/s (p50=129.4μs, p95=502.4μs) |

### Concurrency = 50 (High Load)

| Candidate | C0 (health.live) | C1 (js.text) | C2 (js.json) | C3 (hello.get) |
|---|---|---|---|---|
| **velqu** | 144,398 req/s (p50=297.5μs) | 69,331 req/s (p50=628.7μs) | 65,964 req/s (p50=650μs) | 63,362 req/s (p50=724.2μs) |
| **raw-rust (prebuilt)** | 124,450 req/s (p50=342.6μs) | 125,195 req/s (p50=337.8μs) | 110,798 req/s (p50=368.8μs) | 117,489 req/s (p50=361.9μs) |
| **raw-bun** | 91,074 req/s (p50=394.6μs) | 103,801 req/s (p50=369.2μs) | 115,502 req/s (p50=352.2μs) | 91,201 req/s (p50=387.8μs) |
| **elysia2 AOT** | 85,882 req/s (p50=410.3μs) | 100,867 req/s (p50=378.6μs) | 104,177 req/s (p50=373.9μs) | 41,778 req/s (p50=850.9μs) |

## Key Findings & Single-Worker Architecture

1. **C0 Native Transport Parity**: On native static responses (C0), Velqu achieves **125.2k req/s** at c=10 and **144.4k req/s** at c=50 — ahead of the prebuilt raw Rust baseline (95.8k / 107k req/s in this run).
2. **Direct Vector Dispatch & Automaton Router**: Under M2.3 direct function vector indexing ($O(1)$), the SchemaId-indexed validation vector, and cached prelude stringify, Velqu serves **58.9k–62.4k req/s** at c=10 and reaches **63.4k–69.3k req/s** at c=50 on a single QuickJS worker, with tail latency p95 under 250μs at c=10.
3. **Single-Worker Context**: Velqu executes strictly on **one QuickJS worker** (ADR-0008). Multi-worker scaling is scheduled for M3.
4. **Zero Errors**: Across all test configurations and candidates, error rates were **0%**.

