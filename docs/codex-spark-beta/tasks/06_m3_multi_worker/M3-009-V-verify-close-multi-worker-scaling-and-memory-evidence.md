---
task_id: M3-009-V
parent_task: M3-009
milestone: M3
priority: P1
mode: VERIFY
status: TODO
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
