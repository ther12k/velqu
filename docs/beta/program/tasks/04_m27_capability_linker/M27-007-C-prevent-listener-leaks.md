---
task_id: M27-007-C
parent_task: M27-007
milestone: M27
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M27.md
commit_required: true
---

# M27-007-C — Prevent listener leaks

## Atomic goal

Prevent listener leaks.

## Parent intent

Create one cancellation primitive shared by fetch and native capabilities.

## Dependencies

- `M27-007-B` — `tasks/04_m27_capability_linker/M27-007-B-bridge-route-deadline-and-explicit-cancellation.md`

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
- `crates/q-pack/src/lib.rs`
- `crates/q-http/src/lib.rs`
- `crates/q-runtime/src/serve.rs`
- `Cargo.toml`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Prevent listener leaks.
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
m27-007-c: prevent listener leaks
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record — M27-007-C (PASS)

Deliverable: prevent listener leaks in `AbortSignal` through bounded listener capacity, automatic listener clearing on dispatch, once-listener cleanup, and source signal unregistration in `AbortSignal.any()`.

### Changed files

- `crates/q-capabilities/src/abort.rs`:
  - Defined `MAX_ABORT_LISTENERS` (1,024).
  - Added `try_add_listener` enforcing listener cap fail-closed.
  - Added `listener_count` inspector.
  - Added unit test `listeners_cleared_after_abort_and_bounded_capacity`.
- `crates/q-engine-quickjs/src/prelude.rs`:
  - `addEventListener`: throws `RangeError` if listener count reaches 1024.
  - `dispatchEvent`: clears `_listeners = []` upon firing to prevent retaining callback references.
  - `AbortSignal.any`: unregisters attached listeners from all other source signals as soon as any signal fires, preventing lingering closures.
- `packages/cli/src/abort-signal.test.ts`:
  - Added test suite `Listener leak prevention (M27-007-C)` testing `removeEventListener`, once-listener auto-removal, and `AbortSignal.any` cross-signal cleanup.
- Bookkeeping: STATUS.md, TASK_INDEX.md.

### Tests

- `cargo test -p q-capabilities` — 84 passed (+1 listener leak prevention test).
- `cargo test -p q-engine-quickjs` — 110 passed.
- `cargo test -p velqu-runtime` — 31 passed.
- `cargo test -p q-http` — 11 passed.
- `cargo test -p q-bridge` — 11 passed.
- `cargo test -p q-pack` — 98 passed.
- `bun test` — 192 passed (+3 listener leak prevention tests), 0 failed.

### Commands (fresh worktree on parent HEAD a564b89 / 9bef0ca)

- `cargo test -p q-pack` 98 · `-p q-engine-quickjs` 110 · `-p q-http` 11 · `-p q-bridge` 11 · `-p q-capabilities` 84 · `-p velqu-runtime` 31 — pass.
- `bun test` 192 pass / 0 fail; `bun run typecheck`, `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings` — clean.

### Notes

- Guardrail mapping:
  - No unbounded memory or listener leaks: listener registrations bounded at 1024; automatic clearing on dispatch and composite signals.
  - No cross-invocation ownership: signal listener cleanup prevents retaining references across invocation lifetimes.

