# Cumulative Crossover Request Counts (1788448064944)

Source: per-request series from `ramp.ts` (BETA-003-B), valid requests,
median across reps of cumulative served time. Startup time is excluded
(owned by the cold-start harness). `never` = A never reached B within the
recorded horizon — never extrapolated.

## Class C0

| pair (A vs B) | N* (requests) | horizon |
|---|---:|---:|
Self-amortization (elysia2): steady median 38µs -> request never
Self-amortization (raw-bun): steady median 49µs -> request never
Self-amortization (raw-rust): steady median 60µs -> request 17
Self-amortization (velqu): steady median 36µs -> request 270
| elysia2 vs raw-bun | never | 100 |
| elysia2 vs raw-rust | never | 100 |
| elysia2 vs velqu | never | 100 |
| raw-bun vs elysia2 | 1 | 100 |
| raw-bun vs raw-rust | never | 100 |
| raw-bun vs velqu | never | 125 |
| raw-rust vs elysia2 | 1 | 100 |
| raw-rust vs raw-bun | 1 | 100 |
| raw-rust vs velqu | 1 | 100 |
| velqu vs elysia2 | 1 | 100 |
| velqu vs raw-bun | 1 | 125 |
| velqu vs raw-rust | 76 | 100 |

## Class C2

| pair (A vs B) | N* (requests) | horizon |
|---|---:|---:|
Self-amortization (elysia2): steady median 37µs -> request never
Self-amortization (raw-bun): steady median 43µs -> request never
Self-amortization (raw-rust): steady median 44µs -> request never
Self-amortization (velqu): steady median 60µs -> request 26
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
| velqu vs raw-rust | 3 | 100 |

