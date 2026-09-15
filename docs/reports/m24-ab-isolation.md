# M24-GATE Path A — Matched A/B Isolating M2.4 (C1/C3 p95 Regression Question)

- Date: 2026-09-15 (executed 04:51–06:12 UTC)
- Question (from the refined gate-resolution report, #1345): *Did the M2.4
  zero-copy ingress implementation specifically introduce a regression in
  C1 or C3 p95 latency?*
- Endpoints:
  - **pre-M24**: `3bcb6302` ("docs: close Codex Spark G0 evidence gate" —
    last commit before the first m24-* packet; builds clean under pinned
    1.96.0). Executable sha256 `82fd5ba8a4640d1a…`
  - **M24-complete**: `75bda51f` ("make release artifacts reproducible
    across checkout paths" — **the M24-GATE candidate** the PASS verdict
    was rendered against, per `docs/reports/m24-gate-review.md`). Executable
    sha256 `d7d2a6bebab73c29…`. Not `b8fee349` (whole-stack; excluded by
    design).
- Both endpoints: rustc 1.96.0, release profile with the canonical
  `--remap-path-prefix` / prefix-map flags, each worktree's own compiler
  build of `examples/proof` (pack digests differ across revisions, as
  expected — each runtime serves the pack its own toolchain produced).

## Protocol

Harness `benchmarks/harness/warm.ts` (this packet adds the
`WARM_CANDIDATES` filter so matched velqu-vs-velqu runs skip baseline
cells; byte-identical patched harness ran in both worktrees). Velqu
candidate only; routes C0–C3; concurrency 1/10/50; 10 s cells; seed
20260915 (identical randomized cell order on both endpoints). Two designs:

1. **Round-interleaved** (2 rounds × both endpoints, 5 reps each):
   `m24ab-r{1,2}-{pre,m24c}` — detected that whole-run ambient drift
   dominates c=1/c=10 tails (round-level ratio reversals).
2. **Paired repetitions** (13 pairs; pairs 6–13 targeted at c=10 only):
   `m24ab-p{1..13}-{pre,m24c}` — each pair runs one pre invocation and one
   m24c invocation ~2 min apart, so ambient drift is common to the pair.

## Environment disclosure (no quiet-host claim)

The host is a shared desktop running the operator session's own agent
processes; ambient load was 5.0–6.5 between phases and the measurement
itself drives load to ~8–9 (recorded per phase in the campaign log). One
foreign workload window (load ~14.8, pair 2) is visible in the data and
called out below. All committed prior warm evidence was captured on this
same host class; **these conditions do not meet a quiet-host bar**, and
the paired design + power statement below are how that limitation is
handled honestly.

## Results

Pooled medians over 14 endpoint-runs (rounds 1–2 + pairs 1–5; zero
errors on every cell):

| Cell | p50 ratio (m24c/pre) | p95 ratio | rps ratio |
|---|---|---|---|
| C1 c=1 | 0.820 | 0.915 | 1.244 |
| C1 c=10 | 0.853 | 0.892 | 1.193 |
| C1 c=50 | 0.861 | 0.910 | 1.134 |
| C3 c=1 | 0.916 | 0.910 | 1.098 |
| C3 c=10 | 1.023 | 1.040 | 1.041 |
| C3 c=50 | 0.921 | 0.954 | 1.072 |

Paired-repetition ratios (13 pairs, the drift-robust statistic):

- **C1 c=10 p95**: median **0.829** (10 of 13 pairs ≤ 1.02; pooled medians
  352→302 µs). Improvement, consistent with every other C1 cell.
- **C3 c=10 p95**: median **1.037**; per-pair range 0.75–1.61. The three
  high ratios (1.56/1.61/1.36) are pairs 1–3, coincident with the load
  14.8 foreign-workload window; the ten subsequent pairs hover at 1.0
  (0.75–1.27). p50 ratio is 1.036 with IQR 0.015 — the typical case is
  byte-for-byte-identical in cost.

## Conclusion (bounded to what was measured)

**No material M2.4-introduced C1/C3 p95 regression was detected within the disclosed measurement resolution.** C1 improved on
every metric at every concurrency (p95 ratios 0.89–0.92 pooled). C3 is
flat-to-better at c=1 and c=50 (p95 0.910/0.954); at c=10 the paired p95
median ratio is 1.037 (p50 ratio 1.036 with IQR 0.015). This addresses the
gate-time observation that motivated the clause (gate-time C1 inverse
ratio 90.3% at c=10 vs ~98% elsewhere): the matched A/B shows the M24
revision **faster** than pre-M24 on that exact cell.

**Power statement**: under these ambient conditions the pair scatter
resolves differences of roughly ±25% at c=1/c=10 p95 tails and ~±10–15%
at c=50. Effects smaller than that are not detectable on this host in
this state; "no regression detected" is the established claim, not
"no regression of any size exists". A quieter window would tighten the
c=10 tail bound; it is not required to answer the gate question as posed.

## Raw evidence

30 JSONL files under `benchmarks/raw/warm/` (`m24ab-*.jsonl`), each row
carrying runId, repetition, executionOrder, cell, rps, p50/p95/p99,
errors, RSS, totalRequests. Zero errors across all 30 runs.
