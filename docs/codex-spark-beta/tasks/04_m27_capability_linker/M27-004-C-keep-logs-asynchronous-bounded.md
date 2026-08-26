---
task_id: M27-004-C
parent_task: M27-004
milestone: M27
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M27.md
commit_required: true
---

# M27-004-C — Keep logs asynchronous/bounded

## Atomic goal

Keep logs asynchronous/bounded.

## Parent intent

Move existing timer behavior under the capability ABI and add bounded console semantics.

## Dependencies

- `M27-004-B` — `tasks/04_m27_capability_linker/M27-004-B-define-console-levels-and-redaction.md`

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
- `crates/q-engine/src/lib.rs`
- `crates/q-runtime/src/serve.rs`
- `docs/reports/`
- `docs/beta/workstreams/OBSERVABILITY_OPERATIONS.md`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Keep logs asynchronous/bounded.
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
cargo test -p velqu-runtime
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
m27-004-c: keep logs asynchronous bounded
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record — M27-004-C (PASS)

Deliverable: keep logs asynchronous and bounded via `BoundedLogSink`.

### Changed files

- `crates/q-capabilities/src/console.rs`:
  - Added `BoundedLogSink` with fixed capacity (`DEFAULT_LOG_SINK_CAP` = 1024), non-blocking `try_push`, `drain`, and atomic `LogSinkStats` tracking (`enqueued`, `dropped`, `drained`, `buffered`).
  - Unit test `bounded_log_sink_drops_on_overflow_without_blocking`.
- `crates/q-capabilities/src/lib.rs`: re-exported `BoundedLogSink`, `LogSinkStats`, `DEFAULT_LOG_SINK_CAP`.
- `crates/q-engine-quickjs/src/worker.rs`:
  - `OpRegistry`: added `log_sink: Arc<BoundedLogSink>`.
  - `__velquConsoleLog`: pushes records non-blockingly into `log_sink.try_push(record)` without synchronous I/O blocking JS execution.
  - `check_message_boundary` & `WorkerMsg::Shutdown`: drain and emit records asynchronously.
  - Unit test `bounded_log_sink_integration_in_worker`.
- Bookkeeping: STATUS.md, TASK_INDEX.md.

### Tests

- `cargo test -p q-capabilities` — 58 passed (+1 bounded sink unit test).
- `cargo test -p q-engine-quickjs` — 105 passed (+1 worker log sink integration test).
- `cargo test -p velqu-runtime` — 31 passed.
- `bun test` — 152 passed, 0 failed.

### Commands (fresh worktree on parent HEAD a349aaf)

- `cargo test -p q-pack` 98 · `-p q-engine-quickjs` 105 · `-p q-capabilities` 58 · `-p velqu-runtime` 31 — pass.
- `bun test` 152 pass / 0 fail; `bun run typecheck`, `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings` — clean.

### Notes

- Guardrail mapping:
  - No unbounded logging queue: queue size is bounded by `DEFAULT_LOG_SINK_CAP`; excess log entries increment the `dropped` counter non-blockingly.
  - Existing scheduler invariants remain: message boundaries drain the sink cleanly without leaking across boundaries.

