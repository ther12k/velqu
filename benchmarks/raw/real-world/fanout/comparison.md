# Fan-out Comparison (M28-011-B)

Each request issues n PARALLEL upstream calls (ms=5) via Promise.all-style
fan-out and aggregates. Parallelism proof: p50(n=4) < 4 x p50(n=1).

| candidate | n | c | requests | rps | p50 (µs) | p95 (µs) | p99 (µs) | errors | mismatches |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| elysia2 | 1 | 1 | 480 | 160 | 5854 | 7728 | 12515 | 0 | 0 |
| elysia2 | 1 | 10 | 4930 | 1643.33 | 5880 | 7287 | 8936 | 0 | 0 |
| elysia2 | 2 | 1 | 495 | 165 | 5812 | 7132 | 11174 | 0 | 0 |
| elysia2 | 2 | 10 | 4746 | 1582 | 6029 | 7958 | 10440 | 0 | 0 |
| elysia2 | 4 | 1 | 501 | 167 | 5811 | 6925 | 8613 | 0 | 0 |
| elysia2 | 4 | 10 | 4426 | 1475.33 | 6362 | 9261 | 12979 | 0 | 0 |
- OK elysia2/c1: p50(n=4)=5811µs < 4x p50(n=1)=23416µs (parallelism proven)
- OK elysia2/c10: p50(n=4)=6362µs < 4x p50(n=1)=23520µs (parallelism proven)
| hono | 1 | 1 | 501 | 167 | 5835 | 6839 | 8462 | 0 | 0 |
| hono | 1 | 10 | 4833 | 1611 | 5964 | 7481 | 9738 | 0 | 0 |
| hono | 2 | 1 | 492 | 164 | 5803 | 7900 | 10411 | 0 | 0 |
| hono | 2 | 10 | 4812 | 1604 | 5966 | 7831 | 9423 | 0 | 0 |
| hono | 4 | 1 | 508 | 169.33 | 5795 | 6561 | 7832 | 0 | 0 |
| hono | 4 | 10 | 4453 | 1484.33 | 6216 | 9287 | 15759 | 0 | 0 |
- OK hono/c1: p50(n=4)=5795µs < 4x p50(n=1)=23340µs (parallelism proven)
- OK hono/c10: p50(n=4)=6216µs < 4x p50(n=1)=23856µs (parallelism proven)
| fastify | 1 | 1 | 419 | 139.67 | 6801 | 8456 | 10480 | 0 | 0 |
| fastify | 1 | 10 | 3621 | 1207 | 7799 | 12465 | 16667 | 0 | 0 |
| fastify | 2 | 1 | 451 | 150.33 | 6516 | 7694 | 8805 | 0 | 0 |
| fastify | 2 | 10 | 3576 | 1192 | 7907 | 12048 | 18060 | 0 | 0 |
| fastify | 4 | 1 | 446 | 148.67 | 6542 | 7779 | 9210 | 0 | 0 |
| fastify | 4 | 10 | 2889 | 963 | 9727 | 15974 | 25908 | 0 | 0 |
- OK fastify/c1: p50(n=4)=6542µs < 4x p50(n=1)=27204µs (parallelism proven)
- OK fastify/c10: p50(n=4)=9727µs < 4x p50(n=1)=31196µs (parallelism proven)
| bun-fetch | 1 | 1 | 500 | 166.67 | 5817 | 7166 | 9402 | 0 | 0 |
| bun-fetch | 1 | 10 | 4634 | 1544.67 | 6058 | 8681 | 11166 | 0 | 0 |
| bun-fetch | 2 | 1 | 510 | 170 | 5780 | 6663 | 7508 | 0 | 0 |
| bun-fetch | 2 | 10 | 4740 | 1580 | 6057 | 7898 | 9419 | 0 | 0 |
| bun-fetch | 4 | 1 | 499 | 166.33 | 5817 | 6974 | 8867 | 0 | 0 |
| bun-fetch | 4 | 10 | 4273 | 1424.33 | 6397 | 10825 | 15033 | 0 | 0 |
- OK bun-fetch/c1: p50(n=4)=5817µs < 4x p50(n=1)=23268µs (parallelism proven)
- OK bun-fetch/c10: p50(n=4)=6397µs < 4x p50(n=1)=24232µs (parallelism proven)
- PASS: all candidates: 0 errors, 0 mismatches, fan-out parallelism proven.
