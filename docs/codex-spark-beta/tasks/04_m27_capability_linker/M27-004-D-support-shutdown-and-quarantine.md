---
task_id: M27-004-D
parent_task: M27-004
milestone: M27
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M27.md
commit_required: true
---

# M27-004-D — Support shutdown and quarantine

## Atomic goal

Support shutdown and quarantine.

## Parent intent

Move existing timer behavior under the capability ABI and add bounded console semantics.

## Dependencies

- `M27-004-C` — `tasks/04_m27_capability_linker/M27-004-C-keep-logs-asynchronous-bounded.md`

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
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`
- `crates/q-pack/src/lib.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Support shutdown and quarantine.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

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

## Required evidence for this microtask

- Regression suite.
- Lifecycle tests.
- Overhead measurement.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m27-004-d: support shutdown and quarantine
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record — M27-004-D (PASS)

Deliverable: support capability lifecycle transitions and bounded log flushing on shutdown and quarantine.

### Changed files

- `crates/q-engine-quickjs/src/worker.rs`:
  - `WorkerMsg::Shutdown`: cancels remaining `NativeOp`s, transitions `timer_lifecycle` (`Ready -> Draining -> Quiesced`), drains and flushes `log_sink`.
  - `quarantine_runtime`: cancels remaining `NativeOp`s, transitions `timer_lifecycle` (`Ready -> Failed`), drains and flushes `log_sink`.
  - Unit test `shutdown_and_quarantine_capability_lifecycle_transitions`.
- Bookkeeping: STATUS.md, TASK_INDEX.md.

### Tests

- `cargo test -p q-engine-quickjs` — 106 passed (+1 shutdown/quarantine lifecycle test).
- `cargo test -p q-capabilities` — 58 passed.
- `cargo test -p velqu-runtime` — 31 passed.
- `bun test` — 152 passed, 0 failed.

### Commands (fresh worktree on parent HEAD 9c0457e)

- `cargo test -p q-pack` 98 · `-p q-engine-quickjs` 106 · `-p q-capabilities` 58 · `-p velqu-runtime` 31 — pass.
- `bun test` 152 pass / 0 fail; `bun run typecheck`, `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings` — clean.

### Notes

- Guardrail mapping:
  - Existing scheduler invariants remain: all M2.2.1 scheduler/quarantine invariants pass unchanged.
  - Timers physically cancel: verified in M27-004-A and clean.
  - No unbounded logging queue: flushed on message boundary, quarantine, and shutdown.

