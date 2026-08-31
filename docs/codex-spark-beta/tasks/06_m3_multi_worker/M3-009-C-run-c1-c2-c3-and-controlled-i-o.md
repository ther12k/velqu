---
task_id: M3-009-C
parent_task: M3-009
milestone: M3
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M3.md
commit_required: true
---

# M3-009-C — Run C1/C2/C3 and controlled I/O

## Atomic goal

Run C1/C2/C3 and controlled I/O.

## Parent intent

Demonstrate real scaling without hiding queue latency or per-worker RSS.

## Dependencies

- `M3-009-B` — `tasks/06_m3_multi_worker/M3-009-B-report-throughput-p50-p95-p99-queue-time-cpu-rss-errors.md`

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
5. Implement exactly this deliverable: Run C1/C2/C3 and controlled I/O.
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
m3-009-c: run c1 c2 c3 and controlled i o
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M3-009-C) — PASS

- Date: 2026-08-31
- Branch/PR: m3-009-c (squash-merged; see git log for final hash)
- Closes: #422

### Changed files
- `crates/q-bench-support/src/bin/worker_scaling.rs`: the bench gains a
  WORKLOAD dimension (format v4) with frozen definitions:
  - **C1 — CPU-bound**: 100 % `cpu.work` (verified exactly);
  - **C2 — mixed**: 80 % `light.work` + 20 % `cpu.work` by the
    deterministic rule `id.is_multiple_of(5)` (consumer verifies every
    response against the known kind);
  - **C3 — I/O-bound**: 100 % `io.delay` — one 1 ms native timer op per
    invocation; CONTROLLED I/O (deterministic, no external network;
    1 200 requests per repetition).
  9 configs (3 workloads × 1/2/4 workers) × 3 interleaved repetitions.
- `benchmarks/raw/worker-scaling/`: v4 evidence (71 100 raw samples).
- `docs/reports/m3-009-c-controlled-workloads.md` (new): report with
  artifact hashes.
- `benchmarks/manifest.json`: refreshed (standard remapped flow).

### Headline (exact values in the summary/report; 21 600/21 600 verified, 0 errors)
- **C1 CPU**: 802 / 1 583 / 2 835 ops/s → 1.97× / 3.53×.
- **C2 mixed**: 3 545 / 7 508 / 13 911 ops/s → 2.12× / 3.92×; light
  p50 ~23 µs at EVERY worker count while the CPU tail runs — no
  light-class starvation (M3-008 fairness posture measured in situ).
- **C3 controlled I/O**: 438 / 871 / 1 688 ops/s → 1.99× / 3.85×,
  tightest repetition spread; CPU-per-op collapses (timers don't burn
  CPU while waiting).
- Service p99 in-band across worker counts within each workload — no
  p99 collapse. Per-worker heap: 201 339 B (C1/C2) / 204 182 B (C3,
  timer op table), stable across repetitions.

### Command results
- `cargo test -p q-engine-quickjs` → 20 + 102 + 1 — 0 failed
- `cargo test -p velqu-runtime` → 7 suites — 0 failed
- `bun test` → 219/219; `bun run typecheck` → clean
- `cargo fmt --check` clean; workspace clippy -D warnings → exit 0
  (one `manual_is_multiple_of` lint fixed post-fmt; evidence then
  REGENERATED from the final source so hashes match the shipped
  binary)
- `./scripts/verify` → **ALL PASS**

### Disclosures
- C3's controlled I/O uses native timers (deterministic, no network);
  external-network I/O remains outside the controlled-workload scope.
- Standing: CI fails with zero executed steps on every PR since ~#714
  (infrastructure-side); disclosed per PR.
