---
task_id: M3-001-V
parent_task: M3-001
milestone: M3
priority: P0
mode: VERIFY
status: TODO
context_card: context/milestones/M3.md
commit_required: true
---

# M3-001-V — Verify Freeze independent-worker state semantics

## Atomic goal

Prove every acceptance criterion for parent task M3-001 without broadening scope.

## Parent intent

Define what JavaScript and native state is per worker versus shared.

## Dependencies

- `M3-001-A` — `tasks/06_m3_multi_worker/M3-001-A-accept-adr.md`
- `M3-001-B` — `tasks/06_m3_multi_worker/M3-001-B-document-module-level-state-replication.md`
- `M3-001-C` — `tasks/06_m3_multi_worker/M3-001-C-forbid-jsvalue-sharing.md`
- `M3-001-D` — `tasks/06_m3_multi_worker/M3-001-D-define-service-capability-shared-handles-and-thread-safety.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M3.md`
- `context/components/engine-scheduler.md`
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

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- Each runtime has one owner thread.
- Cross-worker mutable state is explicit.
- Initialization is deterministic.
- Developer docs describe per-worker globals.

## Targeted commands

```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p velqu-runtime
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

- ADR.
- Concurrency model tests plan.
- State examples.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m3-001-v: verify freeze independent worker state semantics
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
