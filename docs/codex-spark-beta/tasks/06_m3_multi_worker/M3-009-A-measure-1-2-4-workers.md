---
task_id: M3-009-A
parent_task: M3-009
milestone: M3
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M3.md
commit_required: true
---

# M3-009-A — Measure 1/2/4 workers

## Atomic goal

Measure 1/2/4 workers.

## Parent intent

Demonstrate real scaling without hiding queue latency or per-worker RSS.

## Dependencies

- `M3-003-Z` — `tasks/06_m3_multi_worker/M3-003-Z-package-evidence-for-implement-serverless-service-and-throughput-profiles.md`
- `M3-006-Z` — `tasks/06_m3_multi_worker/M3-006-Z-package-evidence-for-implement-adaptive-scale-up-and-scale-down.md`
- `M3-008-Z` — `tasks/06_m3_multi_worker/M3-008-Z-package-evidence-for-add-fairness-and-overload-controls.md`

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
5. Implement exactly this deliverable: Measure 1/2/4 workers.
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
m3-009-a: measure 1 2 4 workers
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M3-009-A) — PASS

- Date: 2026-08-31
- Branch/PR: m3-009-a (squash-merged; see git log for final hash)
- Closes: #420

### Changed files
- `crates/q-bench-support/src/bin/worker_scaling.rs` (new, bin
  `q-worker-scaling`): measures 1/2/4 REAL parallel QuickJS runtimes
  (`spawn_independent`: one thread + one runtime each, ADR-0036 §1/§2)
  behind the M3-002 bounded `Dispatcher` (least-outstanding selection,
  per-worker capacity 1 024). 5 repetitions INTERLEAVED round-robin over
  the worker counts; 3 000 measured requests per repetition after
  100/worker warmup; every response verified host-side (45 000/45 000
  correct).
- `benchmarks/raw/worker-scaling/`: `worker-scaling.jsonl` (48 500 raw
  samples: workers, rep, idx, totalUs, queueWaitUs, correct) +
  `worker-scaling-summary.json` (velqu-worker-scaling-v2).
- `docs/reports/m3-009-a-worker-scaling.md`: generated report with
  artifact hashes.
- `crates/q-bench-support/Cargo.toml`: bin registration + q-capabilities
  dep. `benchmarks/manifest.json`: refreshed (standard remapped flow).

### Headline results (exact values in the summary/report)
- Throughput medians 705 / 1 589 / 2 752 ops/s for 1/2/4 workers →
  **2.25× / 3.90× median scaling**, service p50 flat (~1.1–1.2 ms).
- Service p99 flat (~2.2–3.0 ms) under full saturation — **no p99
  collapse**; queue-wait reported separately (never hidden).
- Per-worker heap identical across workers/configs/repetitions:
  200 336 B each (W=4 total 801 344 B). Process RSS disclosed
  process-level only.

### Methodology incidents (root-caused, disclosed)
1. Sequential config phases produced impossible >linear ratios on this
   shared host (the W=1 phase ran under heavier load: service p50
   1 645 µs vs ~1 300 µs elsewhere). Fixed by interleaving repetitions
   round-robin (format bumped to v2). Residual 2.25× at W=2 is
   host-scheduling relief (single consumer vs 8 producers + 2 Tokio
   threads at W=1) — documented as "at least near-linear", not a
   precise efficiency claim.
2. The first raw dump contained only warmup samples (measured samples
   never reached the JSONL — caught by line-count check 3 500 vs
   expected 48 500). Fixed; final raw = 48 500 lines.
3. Correctness initially failed 0/9000: `ResponseStrategy::Js` returns
   `BodyOut::JsonText` (engine-stringified), and JS numbers are f64.
   Fixed the check to parse the text and compare as f64.

### Guardrail mapping (parent M3-009)
- 2-worker scaling: measured; **no numeric approved target exists** —
  flagged as an owner decision (tracked with REVIEW_INDEX open items).
- 4-worker memory budgeted: linear, identical per-worker heaps.
- Serverless profile unchanged: no runtime/profile path touched.
- No p99 collapse: service p99 flat across all W under saturation.

### Command results
- `cargo test -p q-engine-quickjs` → 20 + 102 + 1 — 0 failed
- `cargo test -p velqu-runtime` → 7 suites — 0 failed
- `bun test` → 219 pass / 0 fail; `bun run typecheck` → clean
- `cargo fmt --check` clean; `cargo clippy --workspace --all-targets --
  -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS**

### Scope boundary (explicit)
Invocation-boundary measurement of the engine+dispatcher core (what
ADR-0036 scoped for M3-009); the HTTP layer still drives one engine —
multi-engine HTTP wiring is the M3 integration. Percentile report
formatting, C1/C2/C3 workloads, topology recording: M3-009-B/C/D.

### Disclosures
- Standing: CI fails with zero executed steps on every PR since ~#714
  (infrastructure-side); disclosed per PR.
