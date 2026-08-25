# Real-World Benchmark Report

Generated from `../raw/real-world/smoke/raw.jsonl` at 2026-08-25T14:51:18.912Z.

Scope: candidate `http://127.0.0.1:8791`, 2s cells, concurrency 1/10, per velqu-realworld-summary-v1.

## W1

| c | requests | rps | p50 (µs) | p95 (µs) | p99 (µs) | max (µs) | errors | status mismatches |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 21754 | 10877 | 66 | 231 | 396 | 3603 | 0 | 21754 |
| 10 | 112402 | 56201 | 139 | 390 | 676 | 4860 | 0 | 112402 |

## W2

| c | requests | rps | p50 (µs) | p95 (µs) | p99 (µs) | max (µs) | errors | status mismatches |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 19534 | 9767 | 75 | 235 | 427 | 6794 | 0 | 19534 |
| 10 | 84719 | 42359.5 | 187 | 543 | 1015 | 14277 | 0 | 84719 |

## W3

| c | requests | rps | p50 (µs) | p95 (µs) | p99 (µs) | max (µs) | errors | status mismatches |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 22865 | 11432.5 | 59 | 239 | 435 | 3716 | 0 | 22865 |
| 10 | 92493 | 46246.5 | 166 | 507 | 867 | 6692 | 0 | 92493 |

## W4_1ms

| c | requests | rps | p50 (µs) | p95 (µs) | p99 (µs) | max (µs) | errors | status mismatches |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 1476 | 738 | 1306 | 1667 | 2026 | 4227 | 0 | 0 |
| 10 | 13942 | 6971 | 1353 | 1900 | 2936 | 6898 | 0 | 0 |

## W4_5ms

| c | requests | rps | p50 (µs) | p95 (µs) | p99 (µs) | max (µs) | errors | status mismatches |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 362 | 181 | 5450 | 5874 | 7329 | 8436 | 0 | 0 |
| 10 | 3510 | 1755 | 5506 | 6627 | 8442 | 19547 | 0 | 0 |

## W4_10ms

| c | requests | rps | p50 (µs) | p95 (µs) | p99 (µs) | max (µs) | errors | status mismatches |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 189 | 94.5 | 10519 | 11141 | 11955 | 12959 | 0 | 0 |
| 10 | 1880 | 940 | 10523 | 11493 | 12535 | 13300 | 0 | 0 |

## W4_25ms

| c | requests | rps | p50 (µs) | p95 (µs) | p99 (µs) | max (µs) | errors | status mismatches |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 79 | 39.5 | 25487 | 26213 | 26851 | 26851 | 0 | 0 |
| 10 | 780 | 390 | 25594 | 26410 | 28032 | 28234 | 0 | 0 |

## Retained failures

- W1 c=1: 0 errors, 21754 status mismatches out of 21754 requests (raw rows retained).
- W1 c=10: 0 errors, 112402 status mismatches out of 112402 requests (raw rows retained).
- W2 c=1: 0 errors, 19534 status mismatches out of 19534 requests (raw rows retained).
- W2 c=10: 0 errors, 84719 status mismatches out of 84719 requests (raw rows retained).
- W3 c=1: 0 errors, 22865 status mismatches out of 22865 requests (raw rows retained).
- W3 c=10: 0 errors, 92493 status mismatches out of 92493 requests (raw rows retained).

## Protocol

```text
bun 1.4.0
os linux / arch x64
commit 31ad65d5e1a485e32c58b6581536f9e802381858
spec sha256 bfe25d3d6e102f644fb3c4c3e83ffca1561e0b5f97a99334c0dcab65dde99fdf
workloads sha256 88a33a052f7efaed2e878cce2a57cd4c2bb3c9aef1f97d29e62e2ff2c9f51e33
schema sha256 b7d77b4348271d2922ed75f3c5ed4c43261a713fa4091bca2149ccdeed2dab58
seed sha256 b1f51d068a42bf1af080418f645ecc712f0076610fb9454a429f1be9ba4ba75e
versions sha256 7e852d4b887e9beca826a5fe96cd835d30c70317b27fd80ad67eeb48e606c3a8
```
