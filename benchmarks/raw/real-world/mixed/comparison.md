# Mixed Outcome Comparison (M28-011-C)

Deterministic upstream outcomes per request: success (200 relay),
timeout (500ms upstream vs 100ms client deadline -> typed 504), malformed
(200 + garbage body -> typed 502). Error handling under load, not error
recovery.

| candidate | mode | c | requests | rps | p50 (µs) | p95 (µs) | p99 (µs) | errors | mismatches |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
| elysia2 | SUCCESS | 1 | 537 | 179 | 5561 | 5691 | 6017 | 0 | 0 |
| elysia2 | SUCCESS | 10 | 5330 | 1776.67 | 5600 | 5905 | 6423 | 0 | 0 |
| elysia2 | TIMEOUT | 1 | 30 | 10 | 100615 | 100901 | 101156 | 0 | 0 |
| elysia2 | TIMEOUT | 10 | 300 | 100 | 100710 | 103223 | 104927 | 0 | 0 |
| elysia2 | MALFORMED | 1 | 13071 | 4357 | 198 | 400 | 852 | 0 | 0 |
| elysia2 | MALFORMED | 10 | 69280 | 23093.33 | 384 | 785 | 1431 | 0 | 0 |
| hono | SUCCESS | 1 | 534 | 178 | 5592 | 5817 | 5980 | 0 | 0 |
| hono | SUCCESS | 10 | 5310 | 1770 | 5623 | 5944 | 6586 | 0 | 0 |
| hono | TIMEOUT | 1 | 30 | 10 | 100643 | 101060 | 101241 | 0 | 0 |
| hono | TIMEOUT | 10 | 300 | 100 | 100632 | 101116 | 101991 | 0 | 0 |
| hono | MALFORMED | 1 | 20161 | 6720.33 | 142 | 249 | 336 | 0 | 0 |
| hono | MALFORMED | 10 | 64168 | 21389.33 | 372 | 984 | 2044 | 0 | 0 |
| fastify | SUCCESS | 1 | 458 | 152.67 | 6385 | 7053 | 8121 | 0 | 0 |
| fastify | SUCCESS | 10 | 4392 | 1464 | 6566 | 8827 | 11682 | 0 | 0 |
| fastify | TIMEOUT | 1 | 30 | 10 | 101123 | 101789 | 103407 | 0 | 0 |
| fastify | TIMEOUT | 10 | 300 | 100 | 101231 | 103118 | 105988 | 0 | 0 |
| fastify | MALFORMED | 1 | 5753 | 1917.67 | 518 | 865 | 1094 | 0 | 0 |
| fastify | MALFORMED | 10 | 18224 | 6074.67 | 1385 | 3525 | 5774 | 0 | 0 |
| bun-fetch | SUCCESS | 1 | 541 | 180.33 | 5534 | 5679 | 5918 | 0 | 0 |
| bun-fetch | SUCCESS | 10 | 5350 | 1783.33 | 5583 | 5784 | 6463 | 0 | 0 |
| bun-fetch | TIMEOUT | 1 | 30 | 10 | 100597 | 101023 | 101100 | 0 | 0 |
| bun-fetch | TIMEOUT | 10 | 300 | 100 | 100651 | 101080 | 102022 | 0 | 0 |
| bun-fetch | MALFORMED | 1 | 18783 | 6261 | 144 | 286 | 459 | 0 | 0 |
| bun-fetch | MALFORMED | 10 | 64673 | 21557.67 | 380 | 949 | 1834 | 0 | 0 |
- PASS: all candidates: every mode maps to its exact typed status, 0 errors, 0 mismatches, bounded handling overhead.
