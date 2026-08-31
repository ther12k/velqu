---
task_id: M3-009-B
parent_task: M3-009
milestone: M3
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M3.md
commit_required: true
---

# M3-009-B — Report throughput, p50/p95/p99, queue time, CPU, RSS, errors

## Atomic goal

Report throughput, p50/p95/p99, queue time, CPU, RSS, errors.

## Parent intent

Demonstrate real scaling without hiding queue latency or per-worker RSS.

## Dependencies

- `M3-009-A` — `tasks/06_m3_multi_worker/M3-009-A-measure-1-2-4-workers.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M3.md`
- `context/components/engine-scheduler.md`
- `context/components/multiworker.md`
- `context/components/evidence.md`

### Source files

- `AGENTS.md`
- `crates/q-runtime/src/main.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-engine-quickjs/src/lib.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`
- `benchmarks/harness/`
- `benchmarks/manifest.json`
- `benchmarks/raw/`
- `docs/reports/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Report throughput, p50/p95/p99, queue time, CPU, RSS, errors.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- 2 workers achieve approved scaling target or limitation is documented.
- 4-worker memory is budgeted.
- Serverless profile remains unchanged.
- No p99 collapse under saturation.

## Targeted commands

```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p velqu-runtime
```
```bash
bun test
```
```bash
bun run typecheck
```

## Required evidence for this microtask

- Raw scaling data.
- Generated report.
- Artifact/environment hashes.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m3-009-b: report throughput p50 p95 p99 queue time cpu rss errors
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M3-009-B) — PASS

- Date: 2026-08-31
- Branch/PR: m3-009-b (squash-merged; see git log for final hash)
- Closes: #421

### Changed files
- `crates/q-bench-support/src/bin/worker_scaling.rs`: the summary format
  gains the parent's full metric set — `processCpuSecsPerRepetition` +
  `processCpuSecsPerOp` (getrusage user+sys, process-level, attribution
  disclosed), `wallSecsPerRepetition`, and classified error counters
  (`errors.byClass`: `timeout` = no outcome in budget, `mismatch` = wrong
  status/body/value; every dispatched request is sampled or counted,
  none dropped). Format bumped to velqu-worker-scaling-v3.
- `benchmarks/raw/worker-scaling/`: regenerated v3 evidence
  (48 500 raw samples, 45 000/45 000 verified, 0 errors).
- `docs/reports/m3-009-b-multiworker-metrics.md` (new): the consolidated
  metrics report with artifact hashes.

### Headline (exact values in the summary/report)
- Scaling ratios on a LOADED host: 2.00× (2 workers), 3.54× (4 workers)
  — the interleaved repetition design keeps ratios honest even when the
  host is busy (absolute throughput was ~30% below the quieter v2 run;
  ratios held). Quieter-run comparison: 2.25× / 4.03× (v2 summary in
  M3-009-A's evidence).
- Service p99 flat ACROSS worker counts (5.1–5.5 ms at every W on the
  loaded host) — no p99 collapse; queue wait reported alongside.
- CPU-per-op falls ~4× from W=1 to W=4 (0.0098 → 0.0023 s/op,
  process-level incl. the benchmark's own spin — bounds, not pins).
- RSS stabilizes across repetitions; per-worker heap identical
  (200 336 B each).
- Errors: 0 across 45 000 measured requests; classification wired.

### Command results
- `cargo test -p q-engine-quickjs` → 20 + 102 + 1 — 0 failed
- `cargo test -p velqu-runtime` → 7 suites — 0 failed
- `bun test` → 0 fail; `cargo fmt --check` clean; workspace clippy
  -D warnings → exit 0 (one dead-code fix: the error class is now
  tallied per class, not discarded)
- `./scripts/verify` → **ALL PASS**

### Disclosures
- The committed evidence is from the FINAL source on a loaded host; the
  report carries both runs (quiet v2 vs loaded v3) and states the
  claims as ratios/flatness, per constraint 12.
- Standing: CI fails with zero executed steps on every PR since ~#714
  (infrastructure-side); disclosed per PR.
