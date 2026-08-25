# Real-World Benchmark Report

Generated from `../raw/real-world/smoke/raw.jsonl` at 2026-08-25T15:05:31.612Z.

Scope: candidate `http://127.0.0.1:8791`, 2s cells, concurrency 1/10, per velqu-realworld-summary-v1.

## W1

| c | requests | rps | p50 (µs) | p95 (µs) | p99 (µs) | max (µs) | errors | status mismatches |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 25686 | 12843 | 51 | 217 | 454 | 3973 | 0 | 25686 |
| 10 | 85273 | 42636.5 | 180 | 533 | 1194 | 9171 | 0 | 85273 |

## W2

| c | requests | rps | p50 (µs) | p95 (µs) | p99 (µs) | max (µs) | errors | status mismatches |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 21284 | 10642 | 70 | 232 | 412 | 2786 | 0 | 21284 |
| 10 | 80007 | 40003.5 | 196 | 560 | 1226 | 8935 | 0 | 80007 |

## W3

| c | requests | rps | p50 (µs) | p95 (µs) | p99 (µs) | max (µs) | errors | status mismatches |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 15845 | 7922.5 | 83 | 316 | 760 | 5210 | 0 | 15845 |
| 10 | 75435 | 37717.5 | 197 | 631 | 1344 | 7706 | 0 | 75435 |

## W4_1ms

| c | requests | rps | p50 (µs) | p95 (µs) | p99 (µs) | max (µs) | errors | status mismatches |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 1204 | 602 | 1460 | 2710 | 5313 | 14164 | 0 | 0 |
| 10 | 12287 | 6143.5 | 1443 | 2780 | 3809 | 7207 | 0 | 0 |

## W4_5ms

| c | requests | rps | p50 (µs) | p95 (µs) | p99 (µs) | max (µs) | errors | status mismatches |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 340 | 170 | 5606 | 7801 | 8838 | 10833 | 0 | 0 |
| 10 | 3310 | 1655 | 5706 | 7982 | 9258 | 21695 | 0 | 0 |

## W4_10ms

| c | requests | rps | p50 (µs) | p95 (µs) | p99 (µs) | max (µs) | errors | status mismatches |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 182 | 91 | 10601 | 13040 | 15704 | 22221 | 0 | 0 |
| 10 | 1804 | 902 | 10683 | 13340 | 15133 | 18793 | 0 | 0 |

## W4_25ms

| c | requests | rps | p50 (µs) | p95 (µs) | p99 (µs) | max (µs) | errors | status mismatches |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 78 | 39 | 25531 | 26873 | 28109 | 28109 | 0 | 0 |
| 10 | 770 | 385 | 25719 | 28109 | 30866 | 31073 | 0 | 0 |

## Retained failures

- W1 c=1: 0 errors, 25686 status mismatches out of 25686 requests (raw rows retained).
- W1 c=10: 0 errors, 85273 status mismatches out of 85273 requests (raw rows retained).
- W2 c=1: 0 errors, 21284 status mismatches out of 21284 requests (raw rows retained).
- W2 c=10: 0 errors, 80007 status mismatches out of 80007 requests (raw rows retained).
- W3 c=1: 0 errors, 15845 status mismatches out of 15845 requests (raw rows retained).
- W3 c=10: 0 errors, 75435 status mismatches out of 75435 requests (raw rows retained).

## Protocol

```text
bun 1.4.0
os linux / arch x64
commit 23a98026a1305aac7c4425d9368e7ba05539db8c
spec sha256 bfe25d3d6e102f644fb3c4c3e83ffca1561e0b5f97a99334c0dcab65dde99fdf
workloads sha256 88a33a052f7efaed2e878cce2a57cd4c2bb3c9aef1f97d29e62e2ff2c9f51e33
schema sha256 b7d77b4348271d2922ed75f3c5ed4c43261a713fa4091bca2149ccdeed2dab58
seed sha256 b1f51d068a42bf1af080418f645ecc712f0076610fb9454a429f1be9ba4ba75e
versions sha256 7e852d4b887e9beca826a5fe96cd835d30c70317b27fd80ad67eeb48e606c3a8
```
