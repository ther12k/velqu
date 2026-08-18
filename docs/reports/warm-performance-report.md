---
type: Evidence Report
title: Warm Performance Report (Throughput and Latency)
status: complete
milestone: M1–M2
---

# Warm performance report

Source: `benchmarks/raw/warm/summary.json` (1,000 requests per cell, concurrency=10).
Environment: 13th Gen Intel Core i5-13420H, Linux 7.0.0-28-generic x86_64. Release builds.

## Throughput & Latency (concurrency=10, 1,000 requests)

| Route Class | velqu | raw-rust | raw-bun | elysia2 AOT |
|---|---|---|---|---|
| **C0 (health.live)** | 69,066 req/s (p50=92.5μs) | 60,200 req/s (p50=141.4μs) | 63,002 req/s (p50=98.5μs) | 94,727 req/s (p50=88.4μs) |
| **C1 (js.text)** | 53,585 req/s (p50=176.4μs) | 57,026 req/s (p50=111.2μs) | 29,016 req/s (p50=252.1μs) | 69,122 req/s (p50=91.9μs) |
| **C2 (js.json)** | 116,309 req/s (p50=85.0μs) | 73,406 req/s (p50=107.4μs) | 90,621 req/s (p50=91.6μs) | 50,068 req/s (p50=136.0μs) |
| **C3 (hello.get)** | 35,688 req/s (p50=209.2μs) | 63,481 req/s (p50=101.6μs) | 66,047 req/s (p50=103.5μs) | 75,584 req/s (p50=103.3μs) |

## Key Observations

1. **C0/C2 Throughput**: Velqu achieves **116k req/s** on C2 small JSON (p50 = 85μs), outperforming both raw-rust (73k req/s) and Elysia 2 (50k req/s) due to Rust-level pre-serialization of small objects and lightweight QuickJS worker dispatch.
2. **C3 Validated Route**: Velqu serves **35.6k req/s** with a single QuickJS worker, with p50 latency under 210μs. Elysia 2 serves 75.5k req/s using JIT-compiled TypeBox validators under Bun.
3. **Single-Worker Context**: Velqu M1/M2 runs strictly on **one QuickJS worker** (ADR-0008). Multi-worker scaling is deferred to future milestones.
4. **Zero Errors**: Across all runs and candidates, error rates were **0%** under test concurrency.
