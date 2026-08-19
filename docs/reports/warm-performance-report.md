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

| Route Class | velqu | raw-rust (prebuilt) | raw-bun | elysia2 AOT |
|---|---|---|---|---|
| **C0 (health.live)** | 110,674 req/s (p50=79.5μs, p95=161.1μs) | 106,346 req/s (p50=76.9μs, p95=169.7μs) | 83,884 req/s (p50=84.0μs, p95=305.8μs) | 88,192 req/s (p50=80.6μs, p95=291.4μs) |
| **C1 (js.text)** | 61,961 req/s (p50=167.2μs, p95=216.0μs) | 112,091 req/s (p50=74.6μs, p95=156.6μs) | 88,558 req/s (p50=81.3μs, p95=289.6μs) | 91,368 req/s (p50=81.3μs, p95=281.2μs) |
| **C2 (js.json)** | 59,113 req/s (p50=165.9μs, p95=255.5μs) | 106,643 req/s (p50=77.0μs, p95=171.6μs) | 83,639 req/s (p50=84.2μs, p95=308.7μs) | 80,167 req/s (p50=84.3μs, p95=300.4μs) |
| **C3 (hello.get)** | 58,658 req/s (p50=161.7μs, p95=242.7μs) | 103,049 req/s (p50=80.1μs, p95=187.5μs) | 99,619 req/s (p50=81.9μs, p95=203.8μs) | 85,694 req/s (p50=83.0μs, p95=313.4μs) |

### Concurrency = 50 (High Load)

| Route Class | velqu | raw-rust | raw-bun | elysia2 AOT |
|---|---|---|---|---|
| **C0 (health.live)** | 121,633 req/s (p50=332.2μs) | 128,856 req/s (p50=347.3μs) | 102,971 req/s (p50=376.6μs) | 93,121 req/s (p50=394.8μs) |
| **C1 (js.text)** | 67,644 req/s (p50=634.0μs) | 131,562 req/s (p50=341.4μs) | 95,856 req/s (p50=402.5μs) | 84,151 req/s (p50=431.2μs) |
| **C2 (js.json)** | 66,660 req/s (p50=645.9μs) | 124,744 req/s (p50=350.5μs) | 100,381 req/s (p50=387.7μs) | 114,276 req/s (p50=360.5μs) |
| **C3 (hello.get)** | 69,253 req/s (p50=628.6μs) | 121,892 req/s (p50=354.2μs) | 85,710 req/s (p50=420.6μs) | 113,460 req/s (p50=352.2μs) |

### Concurrency = 1 (Baseline Latency)

| Route Class | velqu | raw-rust | raw-bun | elysia2 AOT |
|---|---|---|---|---|
| **C0 (health.live)** | 19,531 req/s (p50=38.3μs) | 22,183 req/s (p50=33.6μs) | 12,782 req/s (p50=60.9μs) | 15,199 req/s (p50=50.8μs) |
| **C1 (js.text)** | 7,764 req/s (p50=102.4μs) | 20,608 req/s (p50=33.8μs) | 14,843 req/s (p50=54.9μs) | 19,830 req/s (p50=33.9μs) |
| **C2 (js.json)** | 6,895 req/s (p50=118.1μs) | 23,161 req/s (p50=30.8μs) | 20,342 req/s (p50=32.1μs) | 12,931 req/s (p50=56.4μs) |
| **C3 (hello.get)** | 7,374 req/s (p50=111.0μs) | 23,330 req/s (p50=31.5μs) | 13,084 req/s (p50=58.1μs) | 11,475 req/s (p50=56.4μs) |

## Key Findings & Single-Worker Architecture

1. **C0 Native Transport Parity**: On native static responses (C0), Velqu achieves **110.7k req/s** at c=10 and **121.6k req/s** at c=50, demonstrating that Hyper + Tokio HTTP transport matches the prebuilt raw Rust baseline (106k / 128k req/s).
2. **Direct Vector Dispatch & Automaton Router**: Under M2.3 direct function vector indexing ($O(1)$) and cached prelude stringify, Velqu serves **59.1k–62.0k req/s** at c=10 and reaches **66.7k–69.3k req/s** at c=50 on a single QuickJS worker, with tail latency p95 under 260μs at c=10.
3. **Single-Worker Context**: Velqu executes strictly on **one QuickJS worker** (ADR-0008). Multi-worker scaling is scheduled for M3.
4. **Zero Errors**: Across all test configurations and candidates, error rates were **0%**.

