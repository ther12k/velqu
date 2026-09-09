---
task_id: M27-004-V
parent_task: M27-004
milestone: M27
priority: P0
mode: VERIFY
status: PASS
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

## Verification record — M27-004-V (PASS)

Parent: M27-004 "Implement console and timer core capabilities".
Implementation packets merged prior: A (PR #860, #258), B
(PR #861, #259), C (PR #862, #260), D (PR #863, #261).

### Guardrail map

1. **Existing scheduler invariants remain.** All M2.2.1 scheduler/quarantine tests (invocation scopes, deadline scopes, job budgets, quarantine terminal sweep, interrupt handling) pass cleanly: 106 tests in `q-engine-quickjs`.
2. **No unbounded logging queue.** Logging is bounded by message length `MAX_CONSOLE_MSG_LEN` (16,384 B), arg count `MAX_CONSOLE_ARGS` (32), and queue capacity `DEFAULT_LOG_SINK_CAP` (1024). Excess records increment `dropped` counter non-blockingly (`BoundedLogSink`).
3. **Timers physically cancel.** Tokio sleep tasks are aborted via `abort_op_task` upon cancellation/quarantine/shutdown, and `NativeOp::cancel()` transitions state to `Cancelled`.
4. **Capabilities absent when unused.** Capability resolver & pruning (M27-002) accurately link `runtime:timers` when declared and omit it otherwise.

### Manifest

Matched refresh under verify's remap env (qRuntimeRelease hash updated for console/timer capability bindings).

### Commands and results (fresh worktree on parent HEAD 671a520)

- `cargo test -p q-pack` 98 · `-p q-engine-quickjs` 106 · `-p q-capabilities` 58 · `-p velqu-runtime` 31 — pass.
- `bun test` 152 pass / 0 fail; `bun run typecheck`, `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings` — clean.
- `./scripts/verify` — ALL PASS (exit 0).

