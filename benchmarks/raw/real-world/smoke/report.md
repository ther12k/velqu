# Real-World Benchmark Report

Generated from `../raw/real-world/smoke/raw.jsonl` at 2026-08-25T12:55:02.325Z.

Scope: candidate `http://127.0.0.1:8791`, 2s cells, concurrency 1/10, per velqu-realworld-summary-v1.

## W1

| c | requests | rps | p50 (µs) | p95 (µs) | p99 (µs) | max (µs) | errors | status mismatches |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 18362 | 9181 | 71 | 310 | 524 | 3435 | 0 | 18362 |
| 10 | 106141 | 53070.5 | 156 | 404 | 686 | 7534 | 0 | 106141 |

## W2

| c | requests | rps | p50 (µs) | p95 (µs) | p99 (µs) | max (µs) | errors | status mismatches |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 11845 | 5922.5 | 119 | 439 | 736 | 13805 | 0 | 11845 |
| 10 | 92087 | 46043.5 | 165 | 529 | 870 | 5587 | 0 | 92087 |

## W3

| c | requests | rps | p50 (µs) | p95 (µs) | p99 (µs) | max (µs) | errors | status mismatches |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 22236 | 11118 | 57 | 251 | 428 | 5697 | 0 | 22236 |
| 10 | 128928 | 64464 | 108 | 379 | 671 | 6115 | 0 | 128928 |

## W4_1ms

| c | requests | rps | p50 (µs) | p95 (µs) | p99 (µs) | max (µs) | errors | status mismatches |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 1338 | 669 | 1469 | 1770 | 2471 | 4324 | 0 | 0 |
| 10 | 12268 | 6134 | 1465 | 2691 | 4712 | 8994 | 0 | 0 |

## W4_5ms

| c | requests | rps | p50 (µs) | p95 (µs) | p99 (µs) | max (µs) | errors | status mismatches |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 349 | 174.5 | 5642 | 6375 | 7628 | 9823 | 0 | 0 |
| 10 | 3523 | 1761.5 | 5505 | 6661 | 7871 | 13406 | 0 | 0 |

## W4_10ms

| c | requests | rps | p50 (µs) | p95 (µs) | p99 (µs) | max (µs) | errors | status mismatches |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 188 | 94 | 10666 | 10988 | 11807 | 12497 | 0 | 0 |
| 10 | 1880 | 940 | 10594 | 11100 | 11654 | 12087 | 0 | 0 |

## W4_25ms

| c | requests | rps | p50 (µs) | p95 (µs) | p99 (µs) | max (µs) | errors | status mismatches |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 78 | 39 | 25621 | 26123 | 27309 | 27309 | 0 | 0 |
| 10 | 780 | 390 | 25755 | 26663 | 27211 | 27328 | 0 | 0 |

## Retained failures

- W1 c=1: 0 errors, 18362 status mismatches out of 18362 requests (raw rows retained).
- W1 c=10: 0 errors, 106141 status mismatches out of 106141 requests (raw rows retained).
- W2 c=1: 0 errors, 11845 status mismatches out of 11845 requests (raw rows retained).
- W2 c=10: 0 errors, 92087 status mismatches out of 92087 requests (raw rows retained).
- W3 c=1: 0 errors, 22236 status mismatches out of 22236 requests (raw rows retained).
- W3 c=10: 0 errors, 128928 status mismatches out of 128928 requests (raw rows retained).

## Protocol

```text
bun 1.4.0
os linux / arch x64
commit 2b02844d7ac414b97bf0c983df6374c4110ad376
spec sha256 bfe25d3d6e102f644fb3c4c3e83ffca1561e0b5f97a99334c0dcab65dde99fdf
workloads sha256 88a33a052f7efaed2e878cce2a57cd4c2bb3c9aef1f97d29e62e2ff2c9f51e33
schema sha256 b7d77b4348271d2922ed75f3c5ed4c43261a713fa4091bca2149ccdeed2dab58
seed sha256 b1f51d068a42bf1af080418f645ecc712f0076610fb9454a429f1be9ba4ba75e
```
