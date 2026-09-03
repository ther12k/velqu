# CPU Operation Levels (deterministic in-handler loop) — Candidate Comparison (BETA-003-A)

Every candidate implements the identical contract (BETA-002): same routes,
same response bodies, same posture — verified per candidate by
verify-contract.ts before this run. Same machine, same controlled upstream,
same load generator; raw rows are retained alongside each summary.

| candidate | cell | c | requests | rps | p50 (µs) | p95 (µs) | p99 (µs) | max (µs) | errors | mismatches | server RSS (kB) |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| elysia2 | CPU_0 | 1 | 67742 | 22580.67 | 33 | 93 | 132 | 2206 | 0 | 0 | 55272 |
| elysia2 | CPU_0 | 10 | 363371 | 121123.67 | 73 | 177 | 261 | 4997 | 0 | 0 | 55272 |
| elysia2 | CPU_100 | 1 | 74162 | 24720.67 | 29 | 90 | 129 | 3889 | 0 | 0 | 55272 |
| elysia2 | CPU_100 | 10 | 332572 | 110857.33 | 73 | 216 | 300 | 7209 | 0 | 0 | 55272 |
| elysia2 | CPU_1000 | 1 | 60334 | 20111.33 | 35 | 108 | 148 | 5891 | 0 | 0 | 55272 |
| elysia2 | CPU_1000 | 10 | 270821 | 90273.67 | 87 | 194 | 381 | 5033 | 0 | 0 | 55272 |
| elysia2 | CPU_10000 | 1 | 30985 | 10328.33 | 72 | 190 | 238 | 5958 | 0 | 0 | 55272 |
| elysia2 | CPU_10000 | 10 | 67500 | 22500 | 393 | 787 | 830 | 2933 | 0 | 0 | 55272 |
| hono | CPU_0 | 1 | 67490 | 22496.67 | 32 | 90 | 129 | 2114 | 0 | 0 | 48480 |
| hono | CPU_0 | 10 | 235627 | 78542.33 | 90 | 310 | 405 | 3386 | 0 | 0 | 48480 |
| hono | CPU_100 | 1 | 74243 | 24747.67 | 28 | 85 | 125 | 5219 | 0 | 0 | 48480 |
| hono | CPU_100 | 10 | 316422 | 105474 | 80 | 184 | 370 | 8797 | 0 | 0 | 48480 |
| hono | CPU_1000 | 1 | 66892 | 22297.33 | 32 | 103 | 151 | 3924 | 0 | 0 | 48480 |
| hono | CPU_1000 | 10 | 217750 | 72583.33 | 102 | 253 | 470 | 5036 | 0 | 0 | 48480 |
| hono | CPU_10000 | 1 | 30748 | 10249.33 | 70 | 201 | 252 | 2004 | 0 | 0 | 48480 |
| hono | CPU_10000 | 10 | 63899 | 21299.67 | 405 | 816 | 908 | 4164 | 0 | 0 | 48480 |
| fastify | CPU_0 | 1 | 61029 | 20343 | 38 | 102 | 164 | 2556 | 0 | 0 | 164660 |
| fastify | CPU_0 | 10 | 235919 | 78639.67 | 102 | 209 | 413 | 4294 | 0 | 0 | 164660 |
| fastify | CPU_100 | 1 | 72994 | 24331.33 | 32 | 84 | 131 | 2505 | 0 | 0 | 164660 |
| fastify | CPU_100 | 10 | 226322 | 75440.67 | 106 | 215 | 380 | 5269 | 0 | 0 | 164660 |
| fastify | CPU_1000 | 1 | 67763 | 22587.67 | 37 | 79 | 120 | 3456 | 0 | 0 | 164660 |
| fastify | CPU_1000 | 10 | 187646 | 62548.67 | 133 | 262 | 376 | 5613 | 0 | 0 | 164660 |
| fastify | CPU_10000 | 1 | 37991 | 12663.67 | 70 | 128 | 187 | 3059 | 0 | 0 | 164660 |
| fastify | CPU_10000 | 10 | 61115 | 20371.67 | 430 | 862 | 964 | 3765 | 0 | 0 | 164660 |
| bun-fetch | CPU_0 | 1 | 56495 | 18831.67 | 44 | 108 | 151 | 2050 | 0 | 0 | 43472 |
| bun-fetch | CPU_0 | 10 | 309190 | 103063.33 | 75 | 247 | 313 | 4162 | 0 | 0 | 43472 |
| bun-fetch | CPU_100 | 1 | 75332 | 25110.67 | 28 | 92 | 127 | 3886 | 0 | 0 | 43472 |
| bun-fetch | CPU_100 | 10 | 283072 | 94357.33 | 81 | 274 | 351 | 3943 | 0 | 0 | 43472 |
| bun-fetch | CPU_1000 | 1 | 55734 | 18578 | 40 | 114 | 161 | 5629 | 0 | 0 | 43472 |
| bun-fetch | CPU_1000 | 10 | 246471 | 82157 | 94 | 219 | 413 | 6350 | 0 | 0 | 43472 |
| bun-fetch | CPU_10000 | 1 | 34741 | 11580.33 | 68 | 162 | 220 | 2681 | 0 | 0 | 43472 |
| bun-fetch | CPU_10000 | 10 | 66978 | 22326 | 396 | 795 | 839 | 3290 | 0 | 0 | 43472 |

Tail-latency guardrail: for every candidate and W4 cell, p99 must remain within
50x the nominal upstream latency (a structural sanity bound, not a perf claim).
Non-W4 cells (payload/CPU matrices) are checked for zero errors/mismatches only.

- PASS: all candidates, all cells: 0 errors, 0 mismatches, p99 within 50x nominal.
