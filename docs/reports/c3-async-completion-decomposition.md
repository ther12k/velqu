# C3 async-completion decomposition (2026-09-19, diagnostic)

Owner-set question (post-#1394 assessment): **how much additional work
does Velqu perform to deliver an immediately-resolved async result,
beyond the engine's own equivalent completion work?** This packet
measures exactly that, with equivalent completion boundaries and
operation counts, before any completion fast-path is considered.

## Method (`q-c3-decompose` v3)

Six tiers, every tier ending at the SAME logical boundary
(result-consumed / invocation-outcome-finalized):

| tier | handler | boundary |
|---|---|---|
| A_sync | bare engine, ordinary function | result field read |
| A_async | bare engine, same body `async` | Promise driven via job pump, result field read |
| P_sync | production worker path, diagnostic sync twin | outcome finalized |
| P_async | production worker path, canonical async handler | outcome finalized |
| C_async | full host↔worker channel dispatch, canonical | outcome finalized |
| P_async_repeat | P_async repeated after C | run-order guard |

Timing discipline (per the owner's rules): each tier is ONE inclusive
measurement to its boundary; the binary builds WITHOUT
`bench-instrumentation`, so no nested stage timers exist to
double-count; all derived numbers are differences of measured tiers —
never sums of sub-timers. Counts (handler calls, immediate-vs-promise
results, promise watches, job-queue drains, settlement scans) come from
the engine's shared counters around each timed phase, per completed op.
The sync handler is diagnostic only; the canonical route stays async.

## Results (p50 µs/op; 120 batches; correctness-gated; postconditions OK)

| host | A_sync | A_async | P_sync | P_async | C_async | guard Δ |
|---|---|---|---|---|---|---|
| local | 0.313 | 0.920 | 4.000 | 7.294 | 10.553 | −1.00 |
| Halotec | 0.526 | 1.726 | 9.237 | 13.907 | 84.251 | +1.17 |
| Oracle | 0.557 | 1.562 | 6.309 | 12.275 | 17.095 | +0.08 |

Counts per completed op (identical on all hosts):
**P_sync**: 1 handler call, 1 immediate result, 0 watches, 0 drains,
0 scans. **P_async / C_async**: 1 handler call, 1 promise result,
**1 promise watch, 1 job-queue drain, 1 settlement scan**. A_async:
**0 reaction jobs** — QuickJS fulfills a no-`await` async return inline;
the engine's own async-contract cost is Promise allocation + resolution.

## Derived (differences of measured tiers)

| host | raw async contract (engine) | Velqu async handling (P_async−P_sync) | glue, sync contract | glue, async contract | handoff |
|---|---|---|---|---|---|
| local | 0.61 | **3.29** | 3.69 | 6.37 | 3.26 |
| Halotec | 1.20 | **4.67** | 8.71 | 12.18 | 70.34 |
| Oracle | 1.01 | **5.97** | 5.75 | 10.71 | 4.82 |

Additivity check (holds exactly on every host, by construction):
A_sync + glue_sync + prod_async = P_async — e.g. local
0.313 + 3.687 + 3.294 = 7.294.

## Answers

1. **The owner's question, answered:** Velqu performs **3.3–6.0 µs** of
   additional work per immediately-resolved async invocation, against
   the engine's own async-contract cost of 0.6–1.2 µs. The counts say
   precisely what that work is: one promise-watch registration, one
   job-queue drain pass, and one settlement scan per invocation
   (plus promise-result bookkeeping). No extra operations are scheduled
   beyond those three — the machinery is not proliferating; each unit is
   individually cheap but the async path forces all of them.
2. **Corrected attribution of the earlier "glue" number:** the #1393/#1394
   B−A (7.0/13.2/11.3 µs) matches the ASYNC-contract glue here
   (6.4/12.2/10.7 µs). About half of it is async completion machinery,
   not generic context glue. The prior context-plan negative result is
   thereby fully explained: context work lives inside the 3.7–8.7 µs
   sync-contract glue and was already measured small.
3. **Engine floor, restated with the corrected boundary:** the
   synchronous-consumption floor is 0.31–0.56 µs; the async-contract
   floor is 0.92–1.73 µs. The old "0.19–0.39 µs engine" claim timed
   call-only; these are the consumed-result numbers.
4. **Halotec handoff re-confirmed:** 70.3 µs (C−P), consistent with
   #1393's 60.6 µs under the old async-mixed P. The handoff
   investigation (H1/H2/H3) remains the separate queued packet.

## Hypotheses this selects between (owner's list — no implementation yet)

- The 3.3–6.0 µs async handling = watch registration + one drain pass +
  one settlement scan. A cheaper completion contract for
  compiler-provably-sync handlers would eliminate all three units for
  those routes — the counts now bound the win at exactly that value.
- Remaining sync-contract glue (3.7–8.7 µs) is dispatch + conversion +
  settlement bookkeeping — candidates: fewer repeated outcome
  conversions, simpler settlement records.
- A "sync" inference must preserve JS semantics (a fulfilled Promise
  still schedules a reaction; `ValidatedParamsOnly` proves nothing about
  return contracts) — any fast path keys on a compiler-proven handler
  contract, never on plan alone.

Raw: `benchmarks/raw/c3-decompose-engine/c3d-{local,halotec,oracle}-20260919T08*`.
Diagnostic evidence only (AGENTS constraint 12).
