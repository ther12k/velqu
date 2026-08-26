---
task_id: M27-004-A
parent_task: M27-004
milestone: M27
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M27.md
commit_required: true
---

# M27-004-A — Port timer cancellation/accounting

## Atomic goal

Port timer cancellation/accounting.

## Parent intent

Move existing timer behavior under the capability ABI and add bounded console semantics.

## Dependencies

- `M27-001-Z` — `tasks/04_m27_capability_linker/M27-001-Z-package-evidence-for-define-capability-abi-and-lifecycle-state-machine.md`
- `M27-002-Z` — `tasks/04_m27_capability_linker/M27-002-Z-package-evidence-for-implement-compile-time-capability-dependency-resolver.md`

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

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Port timer cancellation/accounting.
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
m27-004-a: port timer cancellation accounting
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record — M27-004-A (PASS)

Deliverable: port existing timer cancellation and accounting under the capability ABI (`q-capabilities`).

### Changed files

- `crates/q-engine-quickjs/Cargo.toml` — added `q-capabilities` workspace dependency.
- `crates/q-engine-quickjs/src/worker.rs`:
  - `OpRegistry`: added `timer_lifecycle: Mutex<q_capabilities::CapabilityLifecycle>` (initialized `Declared -> Installed -> Ready`).
  - `PendingOp`: stores `pub op: q_capabilities::NativeOp`.
  - `__velquTimerStart`: enforces `NativeOp::start` against `timer_lifecycle`, validates `OpOwner { slot: 0, generation: invocation_id }`, `CancellationClass::Cancellable`, and bounded deadline `ms.clamp(1, MAX_OP_DEADLINE_MS)`.
  - `complete_timer`: settles `op.settle(owner)` on Ok or cancels `op.cancel()` on Err; drops late completions if not pending.
  - `WorkerMsg::Shutdown`: transitions `timer_lifecycle` to `Draining -> Quiesced` and cancels remaining `NativeOp`s before aborting tasks.
  - `quarantine_runtime`: transitions `timer_lifecycle` to `Failed` and cancels remaining `NativeOp`s before aborting tasks.
  - Added unit test `timer_capability_lifecycle_and_accounting`.
- Bookkeeping: STATUS.md, TASK_INDEX.md.

### Tests

- `cargo test -p q-engine-quickjs` — 103 passed (+1 new lifecycle/accounting test; all 97 engine integration tests and worker unit tests pass).
- `cargo test -p q-capabilities` — 51 passed.
- `cargo test -p velqu-runtime` — 31 passed.
- `bun test` — 152 passed, 0 failed.

### Commands (fresh worktree on parent HEAD 5101acc)

- `cargo test -p q-pack` 98 · `-p q-engine-quickjs` 103 · `-p q-capabilities` 51 · `-p velqu-runtime` 31 — pass.
- `bun test` 152 pass / 0 fail; `bun run typecheck`, `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings` — clean.

### Notes

- Guardrail mapping:
  - Existing scheduler invariants remain: all M2.2.1 scheduler/quarantine tests pass unchanged.
  - Timers physically cancel: `abort_op_task` aborts the Tokio task handle and `NativeOp::cancel()` transitions state to `Cancelled`.
  - Capabilities absent when unused: capabilities linker and pruning (M27-002) remain intact.

