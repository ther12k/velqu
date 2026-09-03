# Crossover I/O Latency Matrix (0/1/5/10/25ms) — Candidate Comparison (BETA-003-A)

Every candidate implements the identical contract (BETA-002): same routes,
same response bodies, same posture — verified per candidate by
verify-contract.ts before this run. Same machine, same controlled upstream,
same load generator; raw rows are retained alongside each summary.

| candidate | cell | c | requests | rps | p50 (µs) | p95 (µs) | p99 (µs) | max (µs) | errors | mismatches | server RSS (kB) |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| elysia2 | W4_0ms | 1 | 1277 | 638.5 | 1547 | 1985 | 2399 | 5139 | 0 | 0 | 54648 |
| elysia2 | W4_0ms | 10 | 11534 | 5767 | 1681 | 2276 | 3713 | 12935 | 0 | 0 | 54648 |
| elysia2 | W4_1ms | 1 | 1234 | 617 | 1597 | 1970 | 2224 | 3839 | 0 | 0 | 54648 |
| elysia2 | W4_1ms | 10 | 11570 | 5785 | 1698 | 2204 | 3578 | 5166 | 0 | 0 | 54648 |
| elysia2 | W4_5ms | 1 | 336 | 168 | 5942 | 6434 | 6939 | 7620 | 0 | 0 | 54648 |
| elysia2 | W4_5ms | 10 | 3325 | 1662.5 | 5968 | 6555 | 8912 | 19142 | 0 | 0 | 54648 |
| elysia2 | W4_10ms | 1 | 180 | 90 | 11124 | 11603 | 12757 | 13117 | 0 | 0 | 54648 |
| elysia2 | W4_10ms | 10 | 1780 | 890 | 11306 | 11792 | 12756 | 12875 | 0 | 0 | 54648 |
| elysia2 | W4_25ms | 1 | 77 | 38.5 | 26278 | 27203 | 28781 | 28781 | 0 | 0 | 54648 |
| elysia2 | W4_25ms | 10 | 760 | 380 | 26484 | 27007 | 27945 | 27994 | 0 | 0 | 54648 |
| hono | W4_0ms | 1 | 1271 | 635.5 | 1564 | 1987 | 2235 | 3660 | 0 | 0 | 47932 |
| hono | W4_0ms | 10 | 11282 | 5641 | 1749 | 2187 | 3293 | 5229 | 0 | 0 | 47932 |
| hono | W4_1ms | 1 | 1256 | 628 | 1575 | 1916 | 2107 | 3361 | 0 | 0 | 47932 |
| hono | W4_1ms | 10 | 11369 | 5684.5 | 1730 | 2202 | 3283 | 5406 | 0 | 0 | 47932 |
| hono | W4_5ms | 1 | 338 | 169 | 5926 | 6294 | 6461 | 7266 | 0 | 0 | 47932 |
| hono | W4_5ms | 10 | 3380 | 1690 | 5935 | 6471 | 7117 | 8738 | 0 | 0 | 47932 |
| hono | W4_10ms | 1 | 181 | 90.5 | 11084 | 11455 | 12052 | 12921 | 0 | 0 | 47932 |
| hono | W4_10ms | 10 | 1791 | 895.5 | 11163 | 11669 | 12624 | 13098 | 0 | 0 | 47932 |
| hono | W4_25ms | 1 | 76 | 38 | 26411 | 26797 | 26940 | 26940 | 0 | 0 | 47932 |
| hono | W4_25ms | 10 | 770 | 385 | 26320 | 26860 | 27967 | 28048 | 0 | 0 | 47932 |
| fastify | W4_0ms | 1 | 489 | 244.5 | 4071 | 5045 | 5674 | 22546 | 0 | 0 | 163488 |
| fastify | W4_0ms | 10 | 5386 | 2693 | 3390 | 5990 | 8451 | 13838 | 0 | 0 | 163488 |
| fastify | W4_1ms | 1 | 570 | 285 | 3574 | 4198 | 4761 | 9463 | 0 | 0 | 163488 |
| fastify | W4_1ms | 10 | 5659 | 2829.5 | 3138 | 6305 | 8167 | 10417 | 0 | 0 | 163488 |
| fastify | W4_5ms | 1 | 258 | 129 | 7806 | 8580 | 8982 | 10362 | 0 | 0 | 163488 |
| fastify | W4_5ms | 10 | 2512 | 1256 | 7812 | 10094 | 11145 | 14113 | 0 | 0 | 163488 |
| fastify | W4_10ms | 1 | 153 | 76.5 | 13169 | 13835 | 14271 | 14848 | 0 | 0 | 163488 |
| fastify | W4_10ms | 10 | 1490 | 745 | 13296 | 15701 | 17289 | 20720 | 0 | 0 | 163488 |
| fastify | W4_25ms | 1 | 71 | 35.5 | 28423 | 29245 | 30380 | 30380 | 0 | 0 | 163488 |
| fastify | W4_25ms | 10 | 686 | 343 | 29115 | 33115 | 36608 | 37217 | 0 | 0 | 163488 |
| bun-fetch | W4_0ms | 1 | 1243 | 621.5 | 1589 | 1973 | 2230 | 3494 | 0 | 0 | 45256 |
| bun-fetch | W4_0ms | 10 | 12373 | 6186.5 | 1577 | 2066 | 2938 | 6733 | 0 | 0 | 45256 |
| bun-fetch | W4_1ms | 1 | 1291 | 645.5 | 1506 | 1999 | 2618 | 3995 | 0 | 0 | 45256 |
| bun-fetch | W4_1ms | 10 | 11557 | 5778.5 | 1708 | 2121 | 3279 | 5755 | 0 | 0 | 45256 |
| bun-fetch | W4_5ms | 1 | 340 | 170 | 5879 | 6228 | 6756 | 8071 | 0 | 0 | 45256 |
| bun-fetch | W4_5ms | 10 | 3285 | 1642.5 | 6061 | 6507 | 7630 | 8158 | 0 | 0 | 45256 |
| bun-fetch | W4_10ms | 1 | 182 | 91 | 11059 | 11413 | 11735 | 12353 | 0 | 0 | 45256 |
| bun-fetch | W4_10ms | 10 | 1780 | 890 | 11317 | 11707 | 12285 | 12949 | 0 | 0 | 45256 |
| bun-fetch | W4_25ms | 1 | 77 | 38.5 | 26189 | 26820 | 28155 | 28155 | 0 | 0 | 45256 |
| bun-fetch | W4_25ms | 10 | 760 | 380 | 26422 | 27055 | 28117 | 28835 | 0 | 0 | 45256 |

Tail-latency guardrail: for every candidate and W4 cell, p99 must remain within
50x the nominal upstream latency (a structural sanity bound, not a perf claim).
Non-W4 cells (payload/CPU matrices) are checked for zero errors/mismatches only.

- PASS: all candidates, all cells: 0 errors, 0 mismatches, p99 within 50x nominal.
