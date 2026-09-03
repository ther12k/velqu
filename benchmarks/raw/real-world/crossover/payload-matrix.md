# Payload Matrix (W3 route, limit 1/10/20/50) — Candidate Comparison (BETA-003-A)

Every candidate implements the identical contract (BETA-002): same routes,
same response bodies, same posture — verified per candidate by
verify-contract.ts before this run. Same machine, same controlled upstream,
same load generator; raw rows are retained alongside each summary.

| candidate | cell | c | requests | rps | p50 (µs) | p95 (µs) | p99 (µs) | max (µs) | errors | mismatches | server RSS (kB) |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| elysia2 | PAYLOAD_1 | 1 | 20016 | 6672 | 104 | 340 | 440 | 5364 | 0 | 0 | 55272 |
| elysia2 | PAYLOAD_1 | 10 | 44428 | 14809.33 | 569 | 1157 | 1570 | 3904 | 0 | 0 | 55272 |
| elysia2 | PAYLOAD_10 | 1 | 5257 | 1752.33 | 528 | 831 | 1411 | 2039 | 0 | 0 | 55272 |
| elysia2 | PAYLOAD_10 | 10 | 6227 | 2075.67 | 4326 | 8615 | 9058 | 14977 | 0 | 0 | 55272 |
| elysia2 | PAYLOAD_20 | 1 | 3149 | 1049.67 | 895 | 1373 | 2040 | 3224 | 0 | 0 | 55272 |
| elysia2 | PAYLOAD_20 | 10 | 3884 | 1294.67 | 7037 | 13789 | 14874 | 17666 | 0 | 0 | 55272 |
| elysia2 | PAYLOAD_50 | 1 | 1286 | 428.67 | 2321 | 2746 | 2959 | 4327 | 0 | 0 | 55272 |
| elysia2 | PAYLOAD_50 | 10 | 1344 | 448 | 20366 | 39960 | 44604 | 47616 | 0 | 0 | 55272 |
| hono | PAYLOAD_1 | 1 | 27542 | 9180.67 | 89 | 220 | 376 | 4313 | 0 | 0 | 48480 |
| hono | PAYLOAD_1 | 10 | 44203 | 14734.33 | 581 | 1171 | 1523 | 3784 | 0 | 0 | 48480 |
| hono | PAYLOAD_10 | 1 | 4706 | 1568.67 | 538 | 1271 | 1808 | 2663 | 0 | 0 | 48480 |
| hono | PAYLOAD_10 | 10 | 6178 | 2059.33 | 4362 | 8674 | 9261 | 12610 | 0 | 0 | 48480 |
| hono | PAYLOAD_20 | 1 | 2937 | 979 | 895 | 1892 | 2599 | 4737 | 0 | 0 | 48480 |
| hono | PAYLOAD_20 | 10 | 3719 | 1239.67 | 7340 | 14307 | 15549 | 25080 | 0 | 0 | 48480 |
| hono | PAYLOAD_50 | 1 | 1244 | 414.67 | 2310 | 2909 | 4403 | 9320 | 0 | 0 | 48480 |
| hono | PAYLOAD_50 | 10 | 1477 | 492.33 | 18199 | 36081 | 39932 | 62211 | 0 | 0 | 48480 |
| fastify | PAYLOAD_1 | 1 | 33497 | 11165.67 | 72 | 168 | 299 | 9794 | 0 | 0 | 164660 |
| fastify | PAYLOAD_1 | 10 | 59406 | 19802 | 433 | 873 | 1146 | 5568 | 0 | 0 | 164660 |
| fastify | PAYLOAD_10 | 1 | 7882 | 2627.33 | 373 | 443 | 594 | 1139 | 0 | 0 | 164660 |
| fastify | PAYLOAD_10 | 10 | 9568 | 3189.33 | 2819 | 5626 | 5863 | 11430 | 0 | 0 | 164660 |
| fastify | PAYLOAD_20 | 1 | 3701 | 1233.67 | 723 | 1219 | 1859 | 11626 | 0 | 0 | 164660 |
| fastify | PAYLOAD_20 | 10 | 4777 | 1592.33 | 5661 | 11276 | 11802 | 13214 | 0 | 0 | 164660 |
| fastify | PAYLOAD_50 | 1 | 1421 | 473.67 | 2007 | 2677 | 2880 | 4567 | 0 | 0 | 164660 |
| fastify | PAYLOAD_50 | 10 | 1632 | 544 | 16612 | 33120 | 34467 | 37052 | 0 | 0 | 164660 |
| bun-fetch | PAYLOAD_1 | 1 | 21319 | 7106.33 | 96 | 262 | 383 | 3951 | 0 | 0 | 43472 |
| bun-fetch | PAYLOAD_1 | 10 | 44190 | 14730 | 582 | 1189 | 1471 | 3476 | 0 | 0 | 43472 |
| bun-fetch | PAYLOAD_10 | 1 | 5009 | 1669.67 | 539 | 1114 | 1478 | 2790 | 0 | 0 | 43472 |
| bun-fetch | PAYLOAD_10 | 10 | 5995 | 1998.33 | 4347 | 8780 | 10983 | 24918 | 0 | 0 | 43472 |
| bun-fetch | PAYLOAD_20 | 1 | 2702 | 900.67 | 932 | 2251 | 2937 | 3547 | 0 | 0 | 43472 |
| bun-fetch | PAYLOAD_20 | 10 | 3774 | 1258 | 7300 | 14533 | 15666 | 23117 | 0 | 0 | 43472 |
| bun-fetch | PAYLOAD_50 | 1 | 1230 | 410 | 2334 | 3018 | 5031 | 7177 | 0 | 0 | 43472 |
| bun-fetch | PAYLOAD_50 | 10 | 1494 | 498 | 18148 | 35842 | 39116 | 46634 | 0 | 0 | 43472 |

Tail-latency guardrail: for every candidate and W4 cell, p99 must remain within
50x the nominal upstream latency (a structural sanity bound, not a perf claim).
Non-W4 cells (payload/CPU matrices) are checked for zero errors/mismatches only.

- PASS: all candidates, all cells: 0 errors, 0 mismatches, p99 within 50x nominal.
