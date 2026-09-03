# Honest-Loss Ledger (BETA-003-D)

Every loss or non-win substantiated by the committed measured evidence
(ramp summary + crossover counts). Mechanically extracted — the same
rules run over whatever the evidence contains; nothing is omitted or
hand-selected.

Source run: 1788451334621 (generated 2026-09-03T16:02:15.458Z)

| kind | candidate | class | value | detail |
|---|---|---|---|---|
| steady-floor | velqu | C0 | 55µs vs best 24µs (2.29x) | steady p50 is 2.29x the class best |
| steady-floor | raw-bun | C0 | 27µs vs best 24µs (1.13x) | steady p50 is 1.13x the class best |
| steady-floor | elysia2 | C0 | 26µs vs best 24µs (1.08x) | steady p50 is 1.08x the class best |
| steady-floor | raw-rust | C2 | 29µs vs best 26µs (1.12x) | steady p50 is 1.12x the class best |
| steady-floor | raw-bun | C2 | 30µs vs best 26µs (1.15x) | steady p50 is 1.15x the class best |
| crossover-never | elysia2 | C0 | never (100 requests) | never overtook raw-bun within the 100-request horizon |
| crossover-never | elysia2 | C0 | never (100 requests) | never overtook raw-rust within the 100-request horizon |
| crossover-never | elysia2 | C0 | never (100 requests) | never overtook velqu within the 100-request horizon |
| crossover-never | raw-bun | C0 | never (100 requests) | never overtook raw-rust within the 100-request horizon |
| crossover-lag | raw-bun | C0 | lag 56 requests | behind velqu for the first 56 requests |
| crossover-never | velqu | C0 | never (100 requests) | never overtook raw-rust within the 100-request horizon |
| crossover-never | elysia2 | C2 | never (100 requests) | never overtook raw-bun within the 100-request horizon |
| crossover-never | elysia2 | C2 | never (100 requests) | never overtook raw-rust within the 100-request horizon |
| crossover-never | elysia2 | C2 | never (100 requests) | never overtook velqu within the 100-request horizon |
| crossover-never | raw-bun | C2 | never (100 requests) | never overtook raw-rust within the 100-request horizon |
| crossover-never | raw-bun | C2 | never (100 requests) | never overtook velqu within the 100-request horizon |
| crossover-never | velqu | C2 | never (100 requests) | never overtook raw-rust within the 100-request horizon |

17 substantiated loss row(s).
