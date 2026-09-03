# Payload Matrix (W3 route, limit 1/10/20/50) — Candidate Comparison (BETA-003-A)

Every candidate implements the identical contract (BETA-002): same routes,
same response bodies, same posture — verified per candidate by
verify-contract.ts before this run. Same machine, same controlled upstream,
same load generator; raw rows are retained alongside each summary.

| candidate | cell | c | requests | rps | p50 (µs) | p95 (µs) | p99 (µs) | max (µs) | errors | mismatches | server RSS (kB) |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| elysia2 | PAYLOAD_1 | 1 | 11715 | 5857.5 | 136 | 380 | 505 | 10478 | 0 | 0 | 54648 |
| elysia2 | PAYLOAD_1 | 10 | 29599 | 14799.5 | 574 | 1165 | 1506 | 5165 | 0 | 0 | 54648 |
| elysia2 | PAYLOAD_10 | 1 | 3368 | 1684 | 535 | 1173 | 1442 | 2941 | 0 | 0 | 54648 |
| elysia2 | PAYLOAD_10 | 10 | 3991 | 1995.5 | 4430 | 8933 | 9834 | 16906 | 0 | 0 | 54648 |
| elysia2 | PAYLOAD_20 | 1 | 1842 | 921 | 937 | 2014 | 2737 | 3341 | 0 | 0 | 54648 |
| elysia2 | PAYLOAD_20 | 10 | 2462 | 1231 | 7363 | 14538 | 16167 | 23014 | 0 | 0 | 54648 |
| elysia2 | PAYLOAD_50 | 1 | 842 | 421 | 2218 | 3151 | 4943 | 5605 | 0 | 0 | 54648 |
| elysia2 | PAYLOAD_50 | 10 | 950 | 475 | 18871 | 37174 | 45122 | 56191 | 0 | 0 | 54648 |
| hono | PAYLOAD_1 | 1 | 17425 | 8712.5 | 89 | 244 | 395 | 5149 | 0 | 0 | 47932 |
| hono | PAYLOAD_1 | 10 | 26725 | 13362.5 | 611 | 1469 | 1874 | 6051 | 0 | 0 | 47932 |
| hono | PAYLOAD_10 | 1 | 3058 | 1529 | 546 | 1309 | 1573 | 4087 | 0 | 0 | 47932 |
| hono | PAYLOAD_10 | 10 | 3813 | 1906.5 | 4772 | 9508 | 10158 | 17650 | 0 | 0 | 47932 |
| hono | PAYLOAD_20 | 1 | 1816 | 908 | 958 | 2059 | 2590 | 3108 | 0 | 0 | 47932 |
| hono | PAYLOAD_20 | 10 | 2399 | 1199.5 | 7546 | 15094 | 16008 | 30585 | 0 | 0 | 47932 |
| hono | PAYLOAD_50 | 1 | 845 | 422.5 | 2258 | 2912 | 4802 | 5163 | 0 | 0 | 47932 |
| hono | PAYLOAD_50 | 10 | 946 | 473 | 19254 | 38321 | 40297 | 43355 | 0 | 0 | 47932 |
| fastify | PAYLOAD_1 | 1 | 19396 | 9698 | 81 | 198 | 422 | 6116 | 0 | 0 | 163488 |
| fastify | PAYLOAD_1 | 10 | 38408 | 19204 | 446 | 899 | 1131 | 5743 | 0 | 0 | 163488 |
| fastify | PAYLOAD_10 | 1 | 5004 | 2502 | 380 | 533 | 825 | 1815 | 0 | 0 | 163488 |
| fastify | PAYLOAD_10 | 10 | 6069 | 3034.5 | 2906 | 5826 | 6409 | 12300 | 0 | 0 | 163488 |
| fastify | PAYLOAD_20 | 1 | 2534 | 1267 | 715 | 1224 | 1661 | 2242 | 0 | 0 | 163488 |
| fastify | PAYLOAD_20 | 10 | 3135 | 1567.5 | 5707 | 11383 | 12104 | 17091 | 0 | 0 | 163488 |
| fastify | PAYLOAD_50 | 1 | 890 | 445 | 2147 | 2780 | 3910 | 5229 | 0 | 0 | 163488 |
| fastify | PAYLOAD_50 | 10 | 1015 | 507.5 | 16951 | 34431 | 44567 | 78049 | 0 | 0 | 163488 |
| bun-fetch | PAYLOAD_1 | 1 | 11928 | 5964 | 125 | 377 | 469 | 2250 | 0 | 0 | 45256 |
| bun-fetch | PAYLOAD_1 | 10 | 29748 | 14874 | 579 | 1165 | 1442 | 3647 | 0 | 0 | 45256 |
| bun-fetch | PAYLOAD_10 | 1 | 3148 | 1574 | 557 | 1261 | 1712 | 2402 | 0 | 0 | 45256 |
| bun-fetch | PAYLOAD_10 | 10 | 4063 | 2031.5 | 4405 | 8762 | 9400 | 10930 | 0 | 0 | 45256 |
| bun-fetch | PAYLOAD_20 | 1 | 1954 | 977 | 915 | 1869 | 2649 | 3539 | 0 | 0 | 45256 |
| bun-fetch | PAYLOAD_20 | 10 | 2442 | 1221 | 7514 | 14741 | 15792 | 16910 | 0 | 0 | 45256 |
| bun-fetch | PAYLOAD_50 | 1 | 776 | 388 | 2337 | 4652 | 6185 | 7867 | 0 | 0 | 45256 |
| bun-fetch | PAYLOAD_50 | 10 | 964 | 482 | 18960 | 37693 | 39384 | 41023 | 0 | 0 | 45256 |

Tail-latency guardrail: for every candidate and W4 cell, p99 must remain within
50x the nominal upstream latency (a structural sanity bound, not a perf claim).
Non-W4 cells (payload/CPU matrices) are checked for zero errors/mismatches only.

- PASS: all candidates, all cells: 0 errors, 0 mismatches, p99 within 50x nominal.
