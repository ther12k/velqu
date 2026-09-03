# CPU Operation Levels (deterministic in-handler loop) — Candidate Comparison (BETA-003-A)

Every candidate implements the identical contract (BETA-002): same routes,
same response bodies, same posture — verified per candidate by
verify-contract.ts before this run. Same machine, same controlled upstream,
same load generator; raw rows are retained alongside each summary.

| candidate | cell | c | requests | rps | p50 (µs) | p95 (µs) | p99 (µs) | max (µs) | errors | mismatches | server RSS (kB) |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| elysia2 | CPU_0 | 1 | 47444 | 23722 | 30 | 94 | 140 | 3253 | 0 | 0 | 54648 |
| elysia2 | CPU_0 | 10 | 228947 | 114473.5 | 74 | 201 | 272 | 2299 | 0 | 0 | 54648 |
| elysia2 | CPU_100 | 1 | 46894 | 23447 | 30 | 95 | 134 | 3989 | 0 | 0 | 54648 |
| elysia2 | CPU_100 | 10 | 202779 | 101389.5 | 75 | 240 | 318 | 4390 | 0 | 0 | 54648 |
| elysia2 | CPU_1000 | 1 | 30971 | 15485.5 | 60 | 125 | 183 | 2173 | 0 | 0 | 54648 |
| elysia2 | CPU_1000 | 10 | 130198 | 65099 | 125 | 366 | 438 | 4341 | 0 | 0 | 54648 |
| elysia2 | CPU_10000 | 1 | 23228 | 11614 | 68 | 168 | 229 | 2199 | 0 | 0 | 54648 |
| elysia2 | CPU_10000 | 10 | 45046 | 22523 | 392 | 785 | 827 | 4018 | 0 | 0 | 54648 |
| hono | CPU_0 | 1 | 42421 | 21210.5 | 33 | 107 | 149 | 1417 | 0 | 0 | 47932 |
| hono | CPU_0 | 10 | 197011 | 98505.5 | 77 | 268 | 387 | 2345 | 0 | 0 | 47932 |
| hono | CPU_100 | 1 | 41211 | 20605.5 | 34 | 109 | 162 | 2494 | 0 | 0 | 47932 |
| hono | CPU_100 | 10 | 214277 | 107138.5 | 78 | 183 | 366 | 7061 | 0 | 0 | 47932 |
| hono | CPU_1000 | 1 | 35471 | 17735.5 | 42 | 119 | 180 | 4281 | 0 | 0 | 47932 |
| hono | CPU_1000 | 10 | 139133 | 69566.5 | 101 | 289 | 496 | 5756 | 0 | 0 | 47932 |
| hono | CPU_10000 | 1 | 18151 | 9075.5 | 84 | 214 | 283 | 4276 | 0 | 0 | 47932 |
| hono | CPU_10000 | 10 | 43624 | 21812 | 405 | 814 | 866 | 2762 | 0 | 0 | 47932 |
| fastify | CPU_0 | 1 | 52229 | 26114.5 | 31 | 71 | 122 | 1579 | 0 | 0 | 163488 |
| fastify | CPU_0 | 10 | 155120 | 77560 | 104 | 214 | 358 | 3709 | 0 | 0 | 163488 |
| fastify | CPU_100 | 1 | 42496 | 21248 | 36 | 99 | 163 | 2565 | 0 | 0 | 163488 |
| fastify | CPU_100 | 10 | 149384 | 74692 | 110 | 220 | 397 | 3812 | 0 | 0 | 163488 |
| fastify | CPU_1000 | 1 | 36973 | 18486.5 | 40 | 108 | 179 | 3162 | 0 | 0 | 163488 |
| fastify | CPU_1000 | 10 | 118492 | 59246 | 142 | 287 | 480 | 3750 | 0 | 0 | 163488 |
| fastify | CPU_10000 | 1 | 19556 | 9778 | 80 | 192 | 366 | 2131 | 0 | 0 | 163488 |
| fastify | CPU_10000 | 10 | 42164 | 21082 | 419 | 838 | 884 | 3159 | 0 | 0 | 163488 |
| bun-fetch | CPU_0 | 1 | 40612 | 20306 | 36 | 105 | 151 | 1719 | 0 | 0 | 45256 |
| bun-fetch | CPU_0 | 10 | 184006 | 92003 | 80 | 263 | 351 | 3526 | 0 | 0 | 45256 |
| bun-fetch | CPU_100 | 1 | 44456 | 22228 | 32 | 99 | 141 | 3766 | 0 | 0 | 45256 |
| bun-fetch | CPU_100 | 10 | 191501 | 95750.5 | 81 | 267 | 341 | 4258 | 0 | 0 | 45256 |
| bun-fetch | CPU_1000 | 1 | 36852 | 18426 | 39 | 118 | 170 | 3252 | 0 | 0 | 45256 |
| bun-fetch | CPU_1000 | 10 | 144130 | 72065 | 102 | 264 | 448 | 6301 | 0 | 0 | 45256 |
| bun-fetch | CPU_10000 | 1 | 23211 | 11605.5 | 69 | 156 | 216 | 2400 | 0 | 0 | 45256 |
| bun-fetch | CPU_10000 | 10 | 43519 | 21759.5 | 401 | 808 | 890 | 3652 | 0 | 0 | 45256 |

Tail-latency guardrail: for every candidate and W4 cell, p99 must remain within
50x the nominal upstream latency (a structural sanity bound, not a perf claim).
Non-W4 cells (payload/CPU matrices) are checked for zero errors/mismatches only.

- PASS: all candidates, all cells: 0 errors, 0 mismatches, p99 within 50x nominal.
