---
task_id: M27-008-D
parent_task: M27-008
milestone: M27
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M27.md
commit_required: true
---

# M27-008-D — Do not implement custom cryptography

## Atomic goal

Do not implement custom cryptography.

## Parent intent

Provide secure random bytes and UUID without broad crypto scope.

## Dependencies

- `M27-008-C` — `tasks/04_m27_capability_linker/M27-008-C-define-unavailable-entropy-failure.md`

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
5. Implement exactly this deliverable: Do not implement custom cryptography.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Random API fails closed.
- Input limits match intended standard.
- No predictable fallback.
- Security review passes.

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

- Statistical smoke tests.
- WPT cases.
- Security review note.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m27-008-d: do not implement custom cryptography
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record — M27-008-D (PASS)

Deliverable: verify and enforce that no custom, unproven, or weak cryptographic algorithms are implemented, restricting crypto capability strictly to OS CSPRNG primitives.

### Changed files

- `crates/q-capabilities/src/crypto.rs`:
  - Added unit test `no_custom_or_pseudorandom_primitives`.
- `packages/cli/src/crypto-random.test.ts`:
  - Added test suite `Security constraints (M27-008-D)`.
- Bookkeeping: STATUS.md, TASK_INDEX.md.

### Tests

- `cargo test -p q-capabilities` — 90 passed (+1 security audit test).
- `cargo test -p q-engine-quickjs` — 111 passed.
- `cargo test -p velqu-runtime` — 31 passed.
- `cargo test -p q-http` — 11 passed.
- `cargo test -p q-bridge` — 11 passed.
- `cargo test -p q-pack` — 98 passed.
- `bun test` — 200 passed (+1 security constraint test), 0 failed.

### Commands (fresh worktree on parent HEAD 6ac4b73)

- `cargo test -p q-pack` 98 · `-p q-engine-quickjs` 111 · `-p q-http` 11 · `-p q-bridge` 11 · `-p q-capabilities` 90 · `-p velqu-runtime` 31 — pass.
- `bun test` 200 pass / 0 fail; `bun run typecheck`, `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings` — clean.

### Notes

- Guardrail mapping:
  - Security review passes: no custom ciphers, hashing, or pseudo-random fallbacks implemented; delegates solely to `getrandom`.
  - No predictable fallback: all random operations fail closed immediately on entropy failure.

