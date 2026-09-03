# BETA-003-D — Honest Losses

Status: **MEASURED** (mechanically extracted from committed evidence) +
**DRAFT wording** (not approved public positioning).

This report exists so the crossover story cannot be told wins-only. Every
loss or non-win below is *mechanically extracted* from the committed
measured evidence by `benchmarks/harness/losses.ts` (rules: steady-floor
ratio, crossover-never, crossover-lag, no-onset) — the same rules run over
whatever the evidence contains. Generated artifacts:
`benchmarks/raw/ramp/losses.json` + `losses.md` (18 rows at generation
time; run 1788448064944).

## Where Velqu loses (from the ledger)

1. **Steady-state floor, JS-handler work (C2 /js-json):** Velqu's steady
   p50 is 59µs vs a class best of 37µs — **1.59× slower than Elysia2** and
   also behind raw-bun (41µs) and raw-rust (45µs). This is the QuickJS
   handler tax; it is the honest headline: at steady state, warmed JIT
   runtimes execute the JS handler faster than Velqu's interpreter.
2. **Behind raw-rust from request 1:** raw-rust overtakes Velqu
   immediately in C2 (N*=1) and holds a lead for the first 75 requests in
   C0 (Velqu's crossover N* = 76). Velqu is not the fastest native
   transport either — it carries an engine on top of one.
3. **Measurement gaps that are *not* claimed as wins:** Velqu is absent
   from the real-world I/O/payload/CPU matrices (BETA-003-A covers the four
   JS candidates), so there is **no measured QuickJS-vs-JIT scaling data
   under heavy CPU work** — the loopback C2 route is too light to expose
   it. Until measured, no claim is made either way; the gap is the finding.
4. **Context caveats carried into every reuse of these numbers:** single
   run on a shared development host, 3 repetitions, 100-request horizons,
   sequential (c=1) serving, loopback only, Bun 1.4.0 / Node v22.23.2.

## Where the other candidates lose (same ledger, same rules)

- Elysia2 and raw-bun **never overtook** velqu or raw-rust within the
  horizon in either class (their 3.3–14.8ms warmup debt has no
  per-request advantage to amortize with — their steady floors are not
  better).
- raw-rust has the highest C0 steady floor (61µs, 1.79× the class best)
  but the lowest RSS (~3.4MB); raw-bun pays a 3.3–3.9ms first request and
  sits mid-pack at steady state.

## Fairness posture

Losses are defined by fixed rules over committed raw data — the same rules
that produce wins elsewhere. No number in this report is hand-typed; all
quantities trace to `benchmarks/raw/ramp/*` (BETA-003-B/C evidence).
Where a comparison does not exist (Velqu in the real-world matrices; CPU
scaling head-to-head), the gap is declared instead of filled.

## Public wording draft (DRAFT — not approved; do not publish)

> In our measurements, Velqu's pre-compiled handlers start within a few
> hundred microseconds of the first request and reach steady state after
> roughly 26–270 requests, while runtimes that compile at startup paid
> 3–15ms up front and never recouped that debt within our horizon. The
> trade-off is explicit: at steady state, warmed JIT runtimes executed the
> measured JavaScript handlers up to ~1.6× faster than Velqu's interpreter,
> and we have not yet measured heavier CPU workloads head-to-head.
> Single-host measurements, raw data published.

This draft must not be published: it requires quiet-host reruns, the
CPU-scaling head-to-head gap closed, and owner review. No
production-readiness or "fastest" claims are made anywhere in this packet.
