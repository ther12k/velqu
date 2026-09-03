# First Request Through Steady State (BETA-003-B)

One fresh process per sample; sequential validated requests from request #0;
steady onset = start of the window after two consecutive flat window
transitions (window=25, flat = within [0.8x, 1.25x] of the
previous window median); series capped at --max-requests.

Environment: {"bun":"1.4.0","node":"v22.23.2","kernel":"Linux version 7.0.0-30-generic (buildd@lcy02-amd64-067) (x86_64-linux-gnu-gcc-13 (Ubuntu 13.3.0-6ubuntu2~24.04.1) 13.3.0, GNU ld (GNU Binutils for Ubuntu) 2.42) #30~24.04.1-Ubuntu SMP PREEMPT_DYNAMIC Fri Aug  7 13:27:52 UTC 2","cpu":"13th Gen Intel(R) Core(TM) i5-13420H","commit":"dcf42646831894231afc43f997076e89656597a6"}

| candidate | class | first p50 (µs) | steady p50 (µs) | first/steady | steady onset (req #) | errors | RSS (kB) |
|---|---|---:|---:|---:|---:|---:|---:|
| velqu | C0 | 268 | 34 | 7.88 | 100 | 0 | 9768 |
| velqu | C2 | 270 | 59 | 4.58 | 50 | 0 | 9792 |
| raw-rust | C0 | 248 | 61 | 4.07 | 150 | 0 | 3456 |
| raw-rust | C2 | 238 | 45 | 5.29 | 150 | 0 | 3404 |
| raw-bun | C0 | 3258 | 48 | 67.88 | 175 | 0 | 26516 |
| raw-bun | C2 | 3902 | 41 | 95.17 | 150 | 0 | 26280 |
| elysia2 | C0 | 10060 | 35 | 287.43 | 75 | 0 | 46296 |
| elysia2 | C2 | 14811 | 37 | 400.30 | 200 | 0 | 46544 |

Errors > 0 or `none` onset are retained findings, never smoothed away.
