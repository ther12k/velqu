# C1-A — Response-Path Probe and Stage Breakdown (diagnostic, non-canonical)

- Date: 2026-09-16
- Status: diagnostic measurement only — no canonical fixture, gate threshold,
  or benchmark manifest touched. Probe app: `examples/c1-probe/` (never enters
  conformance or gates). Harness: `benchmarks/harness/c1-probe.ts`.
- Host: the soak workstation UNDER the r3 72h soak (co-resident, all
  measurements pinned to two cores). Absolute numbers are depressed and noisy
  relative to canonical evidence; only structural relationships are claimed.
- Raw: `benchmarks/raw/c1-probe/` (stock + instrumented runs, stage dumps).

## Question

Canonical C1 (`/js-text`, text, async) serves ~52-66% of the JSON baselines
while C2 (`/js-json`) leads every framework. Both handlers are async literals.
Hypothesis under test: the divergence is in the post-settlement response path
(text clone + generic validation vs generated encoder).

## Finding 1 — canonical C2 never enters QuickJS

Shutdown stats for one request: `/js-json` → `handler_calls: 0`;
`/js-text` → `handler_calls: 1`. Stage instrumentation on the proof pack:
`/js-text` records the full engine path; `/js-json` records zero everywhere —
including the direct-encoder stage. C2 is served from the **build-time
constant-response path** (native liveness, RUN-009).

The reason is explicit in `packages/compiler/src/extract.ts` (static handler
fold): "Only OBJECT literals become native liveness (JSON semantics).
String/number returns have unknown content-type → served through the engine."

So the canonical C1↔C2 throughput gap measures **the entire QuickJS handler
round-trip** (context construction + handler + Promise settlement + string
extraction + response validation), not a text-vs-JSON response-encoding
difference. The premise "C1 and C2 diverge after settlement" is false: C2 has
no settlement because it has no JS execution.

## Finding 2 — stage breakdown of the canonical-shaped text path

Per-request native-side stages (release build, `bench-instrumentation`
feature, 5k-20k isolated requests per route):

| stage | async text | sync text |
|---|---:|---:|
| handler call (context + handler [+ watch attach]) | 8.4 µs | 5.8 µs |
| Promise settle (wake → outcome) | 1.0-1.25 µs | — |
| JS string → Rust String | 0.16-0.22 µs | 0.16-0.22 µs |
| response validation (String clone → `Value::String` → generic validate) | 0.21-0.26 µs | 0.21-0.26 µs |
| text → HTTP bytes | 0.02 µs | 0.02 µs |
| **total measured native path** | **≈ 9.8 µs** | **≈ 6.3 µs** |

Throughput shape agrees (stock probe medians, n=5): text async is 93%/83%/65%
of text sync at c=1/10/50 — the Promise/settlement tax is real and grows with
saturation.

## Consequence for DirectTextResponsePlan (Packet C1-B)

The plan's target — eliminating the `Value::String` clone + generic validate —
was the 0.21-0.26 µs stage: **~2-2.5% of the C1 engine path**. C1-B implemented
`DirectTextResponsePlan` with borrowed `&str` validation and exact reference error
semantics. The stage instrumentation confirms the validate stage dropped from
0.21–0.26 µs to **0.06 µs** (a ~0.15–0.20 µs saving per request).

As predicted, this is a clean, worthwhile optimization for any dynamic text
response, but does not move C1 to C2.

## Owner Decision (ADR-0044) and Benchmark Taxonomy Reclassification

Per owner review, **Option 1 was adopted**: RUN-009 native liveness is extended to
statically provable string literals when the route declares a constraint-free
`s.string()` response. The object fold was also tightened to require declared-kind
matches.

Following this fold, post-fold measurements confirm:
- `text-async` (now folded): 52.4k req/s at c=10
- `json-async` (folded C2): 50.4k req/s at c=10
- Both constant routes now perform identically.

### Benchmark Taxonomy (ADR-0044)

| Class | Route | What it measures |
|---|---|---|
| C0 | `/health/live` | Native static liveness |
| C1 | `/js-text` | AOT-folded constant text (wire frozen) |
| C2 | `/js-json` | AOT-folded constant JSON (wire frozen) |
| C3 | `/hello/:name` | Validated dynamic request + QuickJS |
| E1 | `/diag/text-engine` | **Explicit JS-boundary plaintext benchmark** (behaviorally verified: `handler_calls > 0`) |

Neither C1 nor C2 may ever again be cited as QuickJS interpreter evidence. E1
serves 27.5k req/s at c=10 under identical conditions, providing the honest,
un-foldable JS engine boundary baseline.

## Instrumentation notes

- `stage_timing` is compiled only under the existing `bench-instrumentation`
  feature (pattern from M25-002-C); production builds carry zero timing.
- Dumper thread starts only when `BENCH_STAGE_DIR` is set; rewrites one JSON
  file per crate every 2 s (process counters reset per spawn — read dumps
  from single-process runs, not the multi-cell probe).
- Stock-vs-instrumented throughput differs by less than run-to-run noise on
  this loaded host (both directions, ±5-15% at c=50).
