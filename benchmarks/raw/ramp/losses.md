# Honest-Loss Ledger (BETA-003-D)

Every loss or non-win substantiated by the committed measured evidence
(ramp summary + crossover counts). Mechanically extracted — the same
rules run over whatever the evidence contains; nothing is omitted or
hand-selected.

Source run: 1788448064944 (generated 2026-09-03T15:36:11.725Z)

| kind | candidate | class | value | detail |
|---|---|---|---|---|
| steady-floor | raw-rust | C0 | 61µs vs best 34µs (1.79x) | steady p50 is 1.79x the class best |
| steady-floor | raw-bun | C0 | 48µs vs best 34µs (1.41x) | steady p50 is 1.41x the class best |
| steady-floor | elysia2 | C0 | 35µs vs best 34µs (1.03x) | steady p50 is 1.03x the class best |
| steady-floor | velqu | C2 | 59µs vs best 37µs (1.59x) | steady p50 is 1.59x the class best |
| steady-floor | raw-rust | C2 | 45µs vs best 37µs (1.22x) | steady p50 is 1.22x the class best |
| steady-floor | raw-bun | C2 | 41µs vs best 37µs (1.11x) | steady p50 is 1.11x the class best |
| crossover-never | elysia2 | C0 | never (100 requests) | never overtook raw-bun within the 100-request horizon |
| crossover-never | elysia2 | C0 | never (100 requests) | never overtook raw-rust within the 100-request horizon |
| crossover-never | elysia2 | C0 | never (100 requests) | never overtook velqu within the 100-request horizon |
| crossover-never | raw-bun | C0 | never (100 requests) | never overtook raw-rust within the 100-request horizon |
| crossover-never | raw-bun | C0 | never (125 requests) | never overtook velqu within the 125-request horizon |
| crossover-lag | velqu | C0 | lag 75 requests | behind raw-rust for the first 75 requests |
| crossover-never | elysia2 | C2 | never (100 requests) | never overtook raw-bun within the 100-request horizon |
| crossover-never | elysia2 | C2 | never (100 requests) | never overtook raw-rust within the 100-request horizon |
| crossover-never | elysia2 | C2 | never (100 requests) | never overtook velqu within the 100-request horizon |
| crossover-never | raw-bun | C2 | never (100 requests) | never overtook raw-rust within the 100-request horizon |
| crossover-never | raw-bun | C2 | never (100 requests) | never overtook velqu within the 100-request horizon |
| crossover-lag | velqu | C2 | lag 2 requests | behind raw-rust for the first 2 requests |

18 substantiated loss row(s).
