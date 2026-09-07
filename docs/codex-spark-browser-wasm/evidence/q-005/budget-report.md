# BWASM-Q-005 Budget Verification & Regression Gate Report

Suite: `bwasm-q-005-budgets`
Date: `2026-09-07`
Runner: `scripts/browser-budgets-rehearsal.py`
Overall Status: **ALL PASS**

## Behavioral & Performance Metrics

| Metric | Target / Budget | Measured Value | Environment | Result |
|---|---|---|---|---|
| **Base WASM Kernel Size** | <= 512,000 bytes (Brotli-11) | **400,229 B** (390.8 KiB) | Brotli-11 quality | **PASS** |
| **Runtime JS Glue Size** | <= 51,200 bytes (Brotli-11) | **2,420 B** (2.3 KiB) | Brotli-11 quality | **PASS** |
| **Total Initial Transfer** | <= 1,048,576 bytes (Brotli-11) | **453,771 B** (443.1 KiB) | Base + glue + app + manifests | **PASS** |
| **Cold Start to Ready** | <= 2,000 ms | **1,938.4 ms** | Chromium clean context, loopback | **PASS** |
| **Warm Start to Ready** | <= 500 ms | **127.66 ms** | Chromium reload with primed SW cache | **PASS** |
| **Kernel Latency Overhead (p50)** | <= 5.0 ms | **0.3 ms** | 100 requests via in-page runtime | **PASS** |
| **Kernel Latency Overhead (p99)** | <= 15.0 ms | **1.4 ms** | 100 requests via in-page runtime | **PASS** |
| **Memory Repeated Lifecycle (Soak)** | Bounded heap growth (< 20 MB) | **+556 KiB** | 100 mixed cycles (requests, 404, 422, abort, KV) | **PASS** |
| **Optional Assets Policy** | 0 unneeded SQL/engine downloads | **0 forbidden requests** | Core project deployment | **PASS** |

## Raw Latency Sample Statistics

- Total samples: 100
- Min: 0.0 ms
- Mean: 0.28 ms
- p50: 0.3 ms
- p90: 0.6 ms
- p95: 0.8 ms
- p99: 1.4 ms
- Max: 3.8 ms

## Automated Gates

- Automated size gate: `packages/browser-runtime/test/budgets.test.ts` (enforces size budgets in `bun test` and `./scripts/verify`).
- Behavioral rehearsal: `scripts/browser-budgets-rehearsal.py` (real Chromium E2E measurement).
