---
task_id: M27-007-D
parent_task: M27-007
milestone: M27
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M27.md
commit_required: true
---

# M27-007-D — Make cancellation idempotent

## Atomic goal

Make cancellation idempotent.

## Parent intent

Create one cancellation primitive shared by fetch and native capabilities.

## Dependencies

- `M27-007-C` — `tasks/04_m27_capability_linker/M27-007-C-prevent-listener-leaks.md`

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
5. Implement exactly this deliverable: Make cancellation idempotent.
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
m27-007-d: make cancellation idempotent
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record — M27-007-D (PASS)

Deliverable: make cancellation idempotent across concurrent threads, async events, and repeated calls in `AbortSignalModel` and `AbortController`.

### Changed files

- `crates/q-capabilities/src/abort.rs`:
  - Added unit test `concurrent_abort_race_notifies_exactly_once` (50 threads racing to call `abort` on the same signal; atomic `swap(true)` ensures exactly one winner, listeners fire exactly once, and initial reason is preserved).
- `packages/cli/src/abort-signal.test.ts`:
  - Added test suite `Idempotency and race resilience (M27-007-D)` testing rapid sequential aborts and concurrent `Promise.all` abort races in JS.
- Bookkeeping: STATUS.md, TASK_INDEX.md.

### Tests

- `cargo test -p q-capabilities` — 85 passed (+1 concurrent abort race test).
- `cargo test -p q-engine-quickjs` — 110 passed.
- `cargo test -p velqu-runtime` — 31 passed.
- `cargo test -p q-bridge` — 11 passed.
- `cargo test -p q-http` — 11 passed.
- `cargo test -p q-pack` — 98 passed.
- `bun test` — 194 passed (+2 idempotency and race tests), 0 failed.

### Commands (fresh worktree on parent HEAD f91fad3 / af704f9)

- `cargo test -p q-pack` 98 · `-p q-engine-quickjs` 110 · `-p q-http` 11 · `-p q-bridge` 11 · `-p q-capabilities` 85 · `-p velqu-runtime` 31 — pass.
- `bun test` 194 pass / 0 fail; `bun run typecheck`, `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings` — clean.

### Notes

- Guardrail mapping:
  - Abort propagates exactly once: atomic exchange guarantees single notification even under heavy concurrent multi-threaded racing.
  - Late listeners follow defined semantics: already-aborted signals execute late listeners immediately with preserved reason.

