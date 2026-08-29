# W4 Controlled-Upstream Latency — Candidate Comparison (M28-011-A)

Every candidate implements the identical proxy contract: `GET /api/bench/io?ms=N`
relayed through the runtime's native fetch to the controlled upstream
(`GET /io?ms=N`). Same machine, same upstream, same load generator.

| candidate | cell | c | requests | rps | p50 (µs) | p95 (µs) | p99 (µs) | max (µs) | errors | mismatches |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| elysia2 | W4_1ms | 1 | 2089 | 696.33 | 1411 | 1616 | 2227 | 12073 | 0 | 0 |
| elysia2 | W4_1ms | 10 | 19482 | 6494 | 1495 | 1862 | 2802 | 5755 | 0 | 0 |
| elysia2 | W4_5ms | 1 | 538 | 179.33 | 5560 | 5745 | 6193 | 6957 | 0 | 0 |
| elysia2 | W4_5ms | 10 | 4084 | 1361.33 | 5905 | 14409 | 21752 | 28922 | 0 | 0 |
| elysia2 | W4_10ms | 1 | 229 | 76.33 | 11419 | 20979 | 31565 | 65517 | 0 | 0 |
| elysia2 | W4_10ms | 10 | 1495 | 498.33 | 15229 | 42453 | 57802 | 72577 | 0 | 0 |
| elysia2 | W4_25ms | 1 | 96 | 32 | 28978 | 45942 | 54110 | 54110 | 0 | 0 |
| elysia2 | W4_25ms | 10 | 1141 | 380.33 | 25759 | 29520 | 32662 | 35304 | 0 | 0 |
| hono | W4_1ms | 1 | 2045 | 681.67 | 1433 | 1723 | 2420 | 9042 | 0 | 0 |
| hono | W4_1ms | 10 | 18730 | 6243.33 | 1506 | 2085 | 3182 | 37441 | 0 | 0 |
| hono | W4_5ms | 1 | 534 | 178 | 5592 | 5842 | 6736 | 7662 | 0 | 0 |
| hono | W4_5ms | 10 | 5290 | 1763.33 | 5635 | 5969 | 6766 | 7759 | 0 | 0 |
| hono | W4_10ms | 1 | 283 | 94.33 | 10605 | 10772 | 11142 | 13063 | 0 | 0 |
| hono | W4_10ms | 10 | 2790 | 930 | 10674 | 11483 | 13489 | 14846 | 0 | 0 |
| hono | W4_25ms | 1 | 117 | 39 | 25613 | 26010 | 26436 | 29410 | 0 | 0 |
| hono | W4_25ms | 10 | 1170 | 390 | 25685 | 26189 | 27202 | 28429 | 0 | 0 |
| fastify | W4_1ms | 1 | 1214 | 404.67 | 2308 | 3382 | 5230 | 60578 | 0 | 0 |
| fastify | W4_1ms | 10 | 7785 | 2595 | 3230 | 7539 | 11602 | 35629 | 0 | 0 |
| fastify | W4_5ms | 1 | 482 | 160.67 | 6129 | 6752 | 8331 | 12372 | 0 | 0 |
| fastify | W4_5ms | 10 | 4699 | 1566.33 | 6138 | 7612 | 11942 | 25179 | 0 | 0 |
| fastify | W4_10ms | 1 | 270 | 90 | 11021 | 11353 | 14176 | 18837 | 0 | 0 |
| fastify | W4_10ms | 10 | 2593 | 864.33 | 11459 | 12801 | 15449 | 19917 | 0 | 0 |
| fastify | W4_25ms | 1 | 113 | 37.67 | 26239 | 29116 | 30874 | 30920 | 0 | 0 |
| fastify | W4_25ms | 10 | 1113 | 371 | 26594 | 29382 | 33484 | 39635 | 0 | 0 |
| bun-fetch | W4_1ms | 1 | 1531 | 510.33 | 1664 | 3669 | 5566 | 15016 | 0 | 0 |
| bun-fetch | W4_1ms | 10 | 11450 | 3816.67 | 1954 | 5996 | 9989 | 41864 | 0 | 0 |
| bun-fetch | W4_5ms | 1 | 527 | 175.67 | 5623 | 6009 | 7640 | 9389 | 0 | 0 |
| bun-fetch | W4_5ms | 10 | 5197 | 1732.33 | 5680 | 6398 | 7992 | 9847 | 0 | 0 |
| bun-fetch | W4_10ms | 1 | 276 | 92 | 10665 | 12326 | 14058 | 15937 | 0 | 0 |
| bun-fetch | W4_10ms | 10 | 2616 | 872 | 11160 | 13299 | 16070 | 17138 | 0 | 0 |
| bun-fetch | W4_25ms | 1 | 116 | 38.67 | 25755 | 27173 | 30690 | 33868 | 0 | 0 |
| bun-fetch | W4_25ms | 10 | 1170 | 390 | 25727 | 26314 | 27434 | 28773 | 0 | 0 |

Tail-latency guardrail: for every candidate and cell, p99 must remain within
50x the nominal upstream latency (a structural sanity bound, not a perf claim).

- PASS: all candidates, all cells: 0 errors, 0 mismatches, p99 within 50x nominal.
