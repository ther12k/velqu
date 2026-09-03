# Cumulative Crossover Request Counts (1788451334621)

Source: per-request series from `ramp.ts` (BETA-003-B), valid requests,
median across reps of cumulative served time. Startup time is excluded
(owned by the cold-start harness). `never` = A never reached B within the
recorded horizon — never extrapolated.

## Class C0

| pair (A vs B) | N* (requests) | horizon |
|---|---:|---:|
Self-amortization (elysia2): steady median 26µs -> request never
Self-amortization (raw-bun): steady median 27µs -> request never
Self-amortization (raw-rust): steady median 23µs -> request 57
Self-amortization (velqu): steady median 52µs -> request never
| elysia2 vs raw-bun | never | 100 |
| elysia2 vs raw-rust | never | 100 |
| elysia2 vs velqu | never | 100 |
| raw-bun vs elysia2 | 1 | 100 |
| raw-bun vs raw-rust | never | 100 |
| raw-bun vs velqu | 57 | 100 |
| raw-rust vs elysia2 | 1 | 100 |
| raw-rust vs raw-bun | 1 | 100 |
| raw-rust vs velqu | 1 | 100 |
| velqu vs elysia2 | 1 | 100 |
| velqu vs raw-bun | 1 | 100 |
| velqu vs raw-rust | never | 100 |

## Class C2

| pair (A vs B) | N* (requests) | horizon |
|---|---:|---:|
Self-amortization (elysia2): steady median 27µs -> request never
Self-amortization (raw-bun): steady median 27µs -> request never
Self-amortization (raw-rust): steady median 28µs -> request 46
Self-amortization (velqu): steady median 27µs -> request never
| elysia2 vs raw-bun | never | 100 |
| elysia2 vs raw-rust | never | 100 |
| elysia2 vs velqu | never | 100 |
| raw-bun vs elysia2 | 1 | 100 |
| raw-bun vs raw-rust | never | 100 |
| raw-bun vs velqu | never | 100 |
| raw-rust vs elysia2 | 1 | 100 |
| raw-rust vs raw-bun | 1 | 100 |
| raw-rust vs velqu | 1 | 100 |
| velqu vs elysia2 | 1 | 100 |
| velqu vs raw-bun | 1 | 100 |
| velqu vs raw-rust | never | 100 |

