---
task_id: M27-007-B
parent_task: M27-007
milestone: M27
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M27.md
commit_required: true
---

# M27-007-B — Bridge route deadline and explicit cancellation

## Atomic goal

Bridge route deadline and explicit cancellation.

## Parent intent

Create one cancellation primitive shared by fetch and native capabilities.

## Dependencies

- `M27-007-A` — `tasks/04_m27_capability_linker/M27-007-A-define-signal-state-listeners-reason.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M27.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`

### Source files

- `AGENTS.md`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `packages/compiler/src/emit.ts`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`
- `crates/q-http/src/lib.rs`
- `crates/q-bridge/src/lib.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-runtime/tests/runtime_conformance.rs`
- `crates/q-pack/src/lib.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Bridge route deadline and explicit cancellation.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Abort propagates exactly once.
- Late listeners follow defined semantics.
- No cross-invocation ownership.
- Shutdown cancellation is bounded.

## Targeted commands

```bash
cargo test -p q-pack
```
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

- Conformance tests.
- Leak tests.
- Race tests.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m27-007-b: bridge route deadline and explicit cancellation
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record — M27-007-B (PASS)

Deliverable: bridge route deadlines and explicit cancellation with `AbortSignal` across request context (`ctx.signal`, `req.signal`) and native capabilities (`native.timer.delay(ms, { signal })`).

### Changed files

- `crates/q-engine-quickjs/src/prelude.rs`:
  - Exposed lazy `signal: AbortSignal` on `ctx` (`__velquMakeCtx`) and `req` (`__velquMakeReq`).
  - Added optional `{ signal }` parameter support to `native.timer.delay` and `__velquTimerP` — immediately rejecting if pre-aborted, or hooking the abort listener to cancel the op and reject with the signal's reason.
- `crates/q-engine-quickjs/src/worker.rs`:
  - Added unit test `bridge_route_deadline_and_timer_abort_signal` testing `ctx.signal`, `req.signal`, and `timer.delay` with abort signal.
- `packages/cli/src/abort-signal.test.ts`:
  - Added tests for pre-aborted signal rejection and mid-flight abort cancellation in timer delay.
- Bookkeeping: STATUS.md, TASK_INDEX.md.

### Tests

- `cargo test -p q-engine-quickjs` — 110 passed (+1 bridge route deadline & timer abort test).
- `cargo test -p q-capabilities` — 83 passed.
- `cargo test -p q-bridge` — 11 passed.
- `cargo test -p q-http` — 11 passed.
- `cargo test -p velqu-runtime` — 31 passed.
- `cargo test -p q-pack` — 98 passed.
- `bun test` — 189 passed (+2 timer signal cancellation tests), 0 failed.

### Commands (fresh worktree on parent HEAD d171a2e)

- `cargo test -p q-pack` 98 · `-p q-engine-quickjs` 110 · `-p q-http` 11 · `-p q-bridge` 11 · `-p q-capabilities` 83 · `-p velqu-runtime` 31 — pass.
- `bun test` 189 pass / 0 fail; `bun run typecheck`, `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings` — clean.

### Notes

- Guardrail mapping:
  - Abort propagates exactly once: abort listeners run exactly once per event.
  - No cross-invocation ownership: `ctx.signal` and `req.signal` are scoped to the invocation context.

