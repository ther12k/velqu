---
task_id: M27-004-V
parent_task: M27-004
milestone: M27
priority: P0
mode: VERIFY
status: TODO
context_card: context/milestones/M27.md
commit_required: true
---

# M27-004-V — Verify Implement console and timer core capabilities

## Atomic goal

Prove every acceptance criterion for parent task M27-004 without broadening scope.

## Parent intent

Move existing timer behavior under the capability ABI and add bounded console semantics.

## Dependencies

- `M27-004-A` — `tasks/04_m27_capability_linker/M27-004-A-port-timer-cancellation-accounting.md`
- `M27-004-B` — `tasks/04_m27_capability_linker/M27-004-B-define-console-levels-and-redaction.md`
- `M27-004-C` — `tasks/04_m27_capability_linker/M27-004-C-keep-logs-asynchronous-bounded.md`
- `M27-004-D` — `tasks/04_m27_capability_linker/M27-004-D-support-shutdown-and-quarantine.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M27.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/capabilities-fetch.md`

### Source files

- `AGENTS.md`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `packages/compiler/src/emit.ts`
- `crates/q-pack/src/lib.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- Existing scheduler invariants remain.
- No unbounded logging queue.
- Timers physically cancel.
- Capabilities absent when unused.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p q-capabilities
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

- Regression suite.
- Lifecycle tests.
- Overhead measurement.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m27-004-v: verify implement console and timer core capabilities
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
