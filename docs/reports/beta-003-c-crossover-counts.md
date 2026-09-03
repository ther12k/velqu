# BETA-003-C — Cumulative Crossover Request Counts

Status: **MEASURED** (derived from BETA-003-B raw per-request series) +
**DRAFT wording** (not approved public positioning).

## Definition (deterministic, startup excluded)

For each route class and ordered candidate pair (A, B): N* is the smallest
request count N where the **median across reps of A's cumulative served
time** for requests 1..N drops to or below B's. A pair that never crosses
within the recorded horizon is reported `never` — never extrapolated.
Process startup (spawn → ready → first response) is deliberately excluded;
it is a separate cold-start-harness quantity. Also reported: each
candidate's **self-amortization point** — the first N where its cumulative
average latency falls within 1.25× of its own steady-phase median.

Source run: `benchmarks/raw/ramp/` (BETA-003-B run 1788448064944, 3 reps,
100-request horizon per rep after criterion; Bun 1.4.0 / Node v22.23.2 /
i5-13420H / commit dcf4264). Generated artifacts:
`crossover-counts.json` + `crossover-counts.md` (no hand-typed values).

## Results

### Class C0 (native transport floor, /health/live)

| pair (A vs B) | N* (requests) |
|---|---:|
| elysia2 vs raw-bun / raw-rust / velqu | never |
| raw-bun vs elysia2 | 1 |
| raw-bun vs raw-rust | never |
| raw-bun vs velqu | never |
| raw-rust vs elysia2 / raw-bun | 1 |
| raw-rust vs velqu | 76 |
| velqu vs elysia2 | 1 |
| velqu vs raw-bun | 1 |
| velqu vs raw-rust | 76 |

Self-amortization: elysia2 never (within horizon), raw-bun never (within
horizon), raw-rust 17, velqu 270. Steady medians: elysia2 38µs, raw-bun
49µs, raw-rust 60µs, velqu 36µs.

### Class C2 (JS handler + JSON, /js-json)

| pair (A vs B) | N* (requests) |
|---|---:|
| elysia2 vs raw-bun / raw-rust / velqu | never |
| raw-bun vs elysia2 | 1 |
| raw-bun vs raw-rust / velqu | never |
| raw-rust vs elysia2 / raw-bun | 1 |
| raw-rust vs velqu | 1 |
| velqu vs elysia2 / raw-bun | 1 |
| velqu vs raw-rust | 3 |

Self-amortization: elysia2 / raw-bun / raw-rust never (within horizon);
velqu 26. Steady medians: elysia2 37µs, raw-bun 43µs, raw-rust 44µs,
velqu 60µs.

## Honest reading (measured-run scope, shared host, 100-request horizon)

1. **The warmup-debt candidates never recover within the horizon**: after
   paying 3.3–14.8ms on the first request, raw-bun and elysia2 do not
   overtake velqu or raw-rust within 100 requests in any class — their
   steady-state floors are equal-or-worse, so there is no per-request
   advantage to amortize the debt with. `never` here is a real result, not
   missing data.
2. **Velqu crosses raw-rust at 76 requests (C0) and 3 requests (C2)** —
   i.e., essentially immediately on JS-handler work and within ~76 requests
   on bare transport. Where Velqu **loses** is the reverse rows and its own
   floor: raw-rust's steady floor is at or below Velqu's in both classes,
   and Velqu's C2 steady median (60µs) is the slowest of the four — the
   QuickJS handler tax is real; the win is the absence of a warmup cliff.
3. **Self-amortization asymmetry**: velqu amortizes within 26–270 requests
   of its own steady floor; the JIT candidates' first-request debt keeps
   their cumulative average above 1.25× steady beyond the whole horizon.
   This is the quantitative basis for the "requests to break even" framing
   BETA-003-D will carry into the public wording.

## Public wording draft (DRAFT — not approved; do not publish)

> Measured in our harness, frameworks that compile code at startup paid
> 3–15ms on their first request and did not recoup that debt within the
> first 100 requests of serving, because their steady-state per-request
> cost was not lower than Velqu's. Velqu's pre-compiled handlers reached
> within a few requests of steady state immediately. Single-host
> measurements with published raw per-request data; not a
> production-environment claim.

This draft must not be published: it requires quiet-host reruns, the
BETA-003-D honest-losses framing pass (notably Velqu's slower C2 steady
floor), and owner review. No production-readiness or "fastest" claims are
made anywhere in this packet.
