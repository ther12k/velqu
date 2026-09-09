---
task_id: M3-009-V
parent_task: M3-009
milestone: M3
priority: P1
mode: VERIFY
status: PASS
context_card: context/milestones/M3.md
commit_required: true
---

# M3-009-V — Verify Close multi-worker scaling and memory evidence

## Atomic goal

Prove every acceptance criterion for parent task M3-009 without broadening scope.

## Parent intent

Demonstrate real scaling without hiding queue latency or per-worker RSS.

## Dependencies

- `M3-009-A` — `tasks/06_m3_multi_worker/M3-009-A-measure-1-2-4-workers.md`
- `M3-009-B` — `tasks/06_m3_multi_worker/M3-009-B-report-throughput-p50-p95-p99-queue-time-cpu-rss-errors.md`
- `M3-009-C` — `tasks/06_m3_multi_worker/M3-009-C-run-c1-c2-c3-and-controlled-i-o.md`
- `M3-009-D` — `tasks/06_m3_multi_worker/M3-009-D-record-physical-core-topology.md`

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

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

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
```bash
cargo fmt --check
```
```bash
cargo clippy --workspace --all-targets -- -D warnings
```
```bash
./scripts/verify
```

## Required evidence for this microtask

- Raw scaling data.
- Generated report.
- Artifact/environment hashes.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m3-009-v: verify close multi worker scaling and memory evidence
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M3-009-V) — PASS

- Date: 2026-08-31
- Branch/PR: m3-009-v (squash-merged; see git log for final hash)
- Closes: #424

### Acceptance-criterion mapping (parent M3-009 guardrails)

1. **2 workers achieve approved scaling target or limitation is
   documented** — measured (interleaved repetitions, quiet-host run):
   C1 1.97×/2.09×, C2 2.12×/2.22×, C3 1.99×/1.98× at 2 workers across
   the A/C evidence runs; **no numeric approved target exists in the
   docs** — the limitation/target gap is documented in every report and
   tracked as an owner decision (REVIEW_INDEX open items).
2. **4-worker memory is budgeted** — per-worker heap identical within
   each config (201 339 B CPU/mixed; 204 182 B C3 with the timer op
   table); W=4 total heap 805 356 B / 816 728 B; RSS series stabilizes
   (no unbounded growth over 15 measured runs); `MAX_WORKERS` bounds
   total heap by construction.
3. **Serverless profile remains unchanged** — the measurement harness
   exercises the engine+dispatcher boundary only; no runtime/profile
   path changed (git range A..D touches bench scripts, reports, and a
   topology script — zero runtime source changes).
4. **No p99 collapse under saturation** — service p99 flat across
   worker counts within each workload on every run (quiet: C1
   2.3–2.4 ms, C2 ~2.0 ms, C3 2.6–3.7 ms; loaded v3 run: 5.1–5.5 ms
   flat across W). Queue wait reported separately everywhere.

### Evidence chain (all committed, raw + generated + hashed)
- **A** #1024 (ede475d): worker-scaling bench (v2, interleaved) +
  report; scaling medians 2.25×/3.90× (quiet host).
- **B** #1025 (476b1fb): v3 metrics (CPU secs, wall secs, classified
  errors); consolidated report; loaded-host run documented honestly
  (2.00×/3.54× — ratios held while absolutes dropped).
- **C** #1026 (cff83c0): v4 workload dimension (C1 CPU / C2 mixed /
  C3 controlled-I/O); 21 600/21 600 verified, 0 errors; C2 light p50
  ~23 µs at every W (no starvation under mixing).
- **D** #1027 (bf8c8b0): physical topology captured (8 physical / 12
  logical, hybrid P+E, SMT) and embedded in every summary; final
  evidence regenerated from the final source (C1 2.09×/3.69×, C2
  2.22×/3.87×, C3 1.98×/3.93×; 0 errors).

### Verification runs (this branch, worktree-fresh)
- `cargo test -p q-engine-quickjs` → 20 + 102 + 1 — 0 failed
- `cargo test -p velqu-runtime` → 7 suites — 0 failed
- `cargo test -p q-capabilities` → 7 suites (260 lib + 6 workload +
  7 fuzz + 1 + 4 + 9) — 0 failed
- `bun test` → 219/219; `bun run typecheck` → clean
- `cargo fmt --check` clean; `cargo clippy --workspace --all-targets --
  -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS**

### Disclosures (standing)
- No production code changed in this packet: verification-only closure
  of M3-009-A/B/C/D.
- The M3-009 numeric scaling target remains an owner decision; all
  evidence is reported as measurements with published per-repetition
  values (constraint 12).
- CI fails with zero executed steps on every PR since ~#714
  (infrastructure-side); disclosed per PR. Local evidence above is
  complete.
