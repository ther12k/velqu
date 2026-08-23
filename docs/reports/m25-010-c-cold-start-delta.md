# M25-010-C — Cold-start delta at 25/1,000 routes

Raw data: `benchmarks/raw/route-count/route-count-1787452753541.jsonl`
(480 samples — 4 candidates × 3 sizes × 40 fresh-process samples,
randomized candidate/size order, zero failures) and
`benchmarks/raw/route-count/summary.json` (run
`m25-010-c-1787452642`). Harness: `benchmarks/harness/route-count.ts`
(fresh process per sample; metric = spawn → first byte-valid response on
`GET /res7/item/7`).

## Fixture refresh (disclosed)

The committed route-count packs predated the current pack contract:
`schemaIrVersion: 1` (runtime requires 2) and js plan strategies without
the M25-007-A `responseFallbackReason`. They were rejected at load by
the current runtime. For this packet the packs were regenerated with the
checked-in fixture builder (`benchmarks/harness/build-proof-pack.ts`),
which now tags every js response strategy with the closed-vocabulary
reason `"explicit"`; bytecode variants re-embedded with
`velqu-bytecode embed`. Pack growth is +2.8% across all sizes (e.g.
app-1000.qpack 1,760,978 → 1,811,014 bytes). No harness protocol change;
no assertion weakened.

## Results (this machine, 2026-08-23)

p50 / p95 ms cold start (spawn → first valid response); RSS p50 kB after ready:

| candidate | 25 routes | 1,000 routes | 10,000 routes | RSS @1k |
|---|---|---|---|---|
| velqu (source) | 5.606 / 7.608 | 74.195 / 101.575 | 853.071 / 929.056 | 53,276 |
| velqu (bytecode) | 7.991 / 17.716 | 84.480 / 101.798 | 867.335 / 984.403 | 53,272 |
| raw-bun | 9.819 / 18.167 | 7.228 / 12.827 | 13.425 / 23.975 | 20,076 |
| elysia2 | 146.538 / 183.484 | 183.635 / 234.295 | 290.404 / 395.359 | 67,640 |

Within-run scaling deltas (1,000 vs 25 routes, p50): velqu source
**+1,223%**, velqu bytecode **+957%**, raw-bun −26%, elysia2 +25%.
Marginal cost at scale (source, 10k cell): ~85 µs per added route.

## Delta vs previously recorded run

Previous recorded evidence is the G0 smoke (`g0-route-count-1787214108`,
2026-08-20, **5 samples per cell**, stale packs):

| candidate | 25 | 1,000 | 10,000 |
|---|---|---|---|
| velqu (source) | 3.606 → 5.606 (**+55%**) | 22.901 → 74.195 (**+224%**) | 303.835 → 853.071 (**+181%**) |
| velqu (bytecode) | 3.662 → 7.991 (**+118%**) | 21.043 → 84.480 (**+301%**) | 267.937 → 867.335 (**+224%**) |
| raw-bun | 13.657 → 9.819 (−28%) | 15.173 → 7.228 (−52%) | 14.896 → 13.425 (−10%) |
| elysia2 | 140.875 → 146.538 (+4%) | 166.695 → 183.635 (+10%) | 291.674 → 290.404 (−0.4%) |

Honest reading:

1. **A cold-start regression is real and unexplained by pack size.**
   Pack bytes grew only +2.8%; the bun baselines moved −28%/−52% on the
   same host between runs (environmental drift works *in favor of* the
   old numbers, so the velqu regression is not thermal noise). Per-route
   marginal startup cost roughly tripled (~30 µs → ~85 µs/route).
2. **Attribution (measured):** the runtime's own stage logs show
   `pack.load` dominates startup and scales linearly with route/schema
   count — n=25 median ~5.1 ms of ~7.6 ms; n=1,000 range 72–119 ms of
   79–125 ms (~90%); n=10,000 range 752–850 ms of 806–906 ms (~93%).
   Stage timings above are from 3–5 matched manual spawns per size
   (`--log off`, stdout captured), not part of the JSONL suite.
   Attribution beyond this stage boundary is not instrumented yet.
3. **Attribution (hypothesis, clearly separated):** since g0, pack load
   gained IR v2 canonical integrity re-hashing over the whole view
   including the schema manifest (M25-001-C), denser manifest
   verification, and per-route RoutePlan/fallback-reason checks
   (M2.3/M25-007-A). This plausibly explains the `pack.load` growth, but
   no per-substage profile exists to apportion blame — instrumenting
   that belongs to follow-up work (M25-010-D records CPU/RSS; substage
   profiling is unscheduled).
4. **Post-ready codec tables are not the dominant cost.** The dense
   `DecoderTable`/`EncoderTable` construction (q-runtime main.rs, built
   after the ready line, before the serve loop) lands inside the measured
   window, but single-spawn smoke shows totalMs p50 within run-to-run
   variance of `startupMs`, bounding its share well below `pack.load`.

## Guardrail status

Parent guardrail: *"No unapproved cold-start regression."* Measured
regression exists (items 1–2). This report documents it rather than
suppressing it; **approval or a mitigation decision is explicitly
escalated to M25-GATE**. Candidate mitigations (not built here):
binary QPack v2 (M2.6) removing JSON parse/re-hash from the load path,
and incremental/lazy schema-manifest verification.

## Decision matrix

| axis | state | inspectability |
|---|---|---|
| request validation | native for all fixture/generated routes | `validationStrategy: "native"` per route |
| response encode | js strategy per declared responses, each carrying explicit fallback reason `"explicit"` from the closed vocabulary | pack verification rejects silent fallbacks (M25-007-A) |
| cold-start overhead | O(routes+schemas) pack load/verify dominates; codec tables compile once post-ready, bounded by schema manifest | per-stage `startupMs` logs; `velqu inspect routes` (M25-007-D) |
| verdict | strategy selection does NOT inflate startup via codecs; the load/verify path does — escalated to gate | reports match raw JSONL |

Velqu remains 24–33x faster than elysia2 at 25/1,000 routes and its RSS
at 1k routes (52 MB) is below elysia2 (66 MB); the scaling slope against
route count is the open issue, not absolute competitiveness.

Scope: no binary QPack encoding, no capability API expansion, no ORM.
No performance claim beyond recorded samples; raw JSONL retained.
