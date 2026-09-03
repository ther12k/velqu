# First Request Through Steady State (BETA-003-B)

One fresh process per sample; sequential validated requests from request #0;
steady onset = start of the window after two consecutive flat window
transitions (window=25, flat = within [0.8x, 1.25x] of the
previous window median); series capped at --max-requests.

Environment: {"bun":"1.4.0","node":"v22.23.2","kernel":"Linux version 7.0.0-30-generic (buildd@lcy02-amd64-067) (x86_64-linux-gnu-gcc-13 (Ubuntu 13.3.0-6ubuntu2~24.04.1) 13.3.0, GNU ld (GNU Binutils for Ubuntu) 2.42) #30~24.04.1-Ubuntu SMP PREEMPT_DYNAMIC Fri Aug  7 13:27:52 UTC 2","cpu":"13th Gen Intel(R) Core(TM) i5-13420H","commit":"b496ffd83fea432d48d801211a2c4144e793adea"}

| candidate | class | first p50 (µs) | steady p50 (µs) | first/steady | steady onset (req #) | errors | RSS (kB) |
|---|---|---:|---:|---:|---:|---:|---:|
| velqu | C0 | 373 | 55 | 6.78 | 100 | 0 | 9628 |
| velqu | C2 | 232 | 26 | 8.92 | 75 | 0 | 9672 |
| raw-rust | C0 | 261 | 24 | 10.88 | 100 | 0 | 3412 |
| raw-rust | C2 | 169 | 29 | 5.83 | 100 | 0 | 3424 |
| raw-bun | C0 | 1979 | 27 | 73.30 | 50 | 0 | 25800 |
| raw-bun | C2 | 1832 | 30 | 61.07 | 75 | 0 | 25780 |
| elysia2 | C0 | 8615 | 26 | 331.35 | 50 | 0 | 49420 |
| elysia2 | C2 | 8862 | 26 | 340.85 | 50 | 0 | 50032 |

Errors > 0 or `none` onset are retained findings, never smoothed away.
