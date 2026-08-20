---
task_id: M3-002-V
parent_task: M3-002
milestone: M3
priority: P0
mode: VERIFY
status: TODO
context_card: context/milestones/M3.md
commit_required: true
---

# M3-002-V — Verify Implement bounded worker dispatcher

## Atomic goal

Prove every acceptance criterion for parent task M3-002 without broadening scope.

## Parent intent

Route matched requests to workers without unbounded queues or shared engine mutexes.

## Dependencies

- `M3-002-A` — `tasks/06_m3_multi_worker/M3-002-A-use-bounded-per-worker-queues.md`
- `M3-002-B` — `tasks/06_m3_multi_worker/M3-002-B-select-worker-using-outstanding-load-strategy.md`
- `M3-002-C` — `tasks/06_m3_multi_worker/M3-002-C-define-admission-and-overload-response.md`
- `M3-002-D` — `tasks/06_m3_multi_worker/M3-002-D-preserve-routeid-routeplan-before-dispatch.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M3.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`
- `context/components/multiworker.md`

### Source files

- `AGENTS.md`
- `crates/q-runtime/src/main.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-engine-quickjs/src/lib.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`
- `crates/q-http/src/lib.rs`
- `crates/q-bridge/src/lib.rs`
- `crates/q-runtime/tests/runtime_conformance.rs`
- `benchmarks/harness/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- Queue capacity is configurable and bounded.
- Overload fails quickly and observably.
- No head-of-line lock across workers.
- Per-worker queue latency is measured.

## Targeted commands

```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p q-http
```
```bash
cargo test -p q-bridge
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

- Dispatcher tests.
- Overload load test.
- Metrics.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m3-002-v: verify implement bounded worker dispatcher
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
