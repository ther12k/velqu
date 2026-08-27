---
task_id: M27-008-A
parent_task: M27-008
milestone: M27
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M27.md
commit_required: true
---

# M27-008-A — Implement `getRandomValues` and `randomUUID` through OS CSPRNG

## Atomic goal

Implement `getRandomValues` and `randomUUID` through OS CSPRNG.

## Parent intent

Provide secure random bytes and UUID without broad crypto scope.

## Dependencies

- `M27-001-Z` — `tasks/04_m27_capability_linker/M27-001-Z-package-evidence-for-define-capability-abi-and-lifecycle-state-machine.md`
- `M27-003-Z` — `tasks/04_m27_capability_linker/M27-003-Z-package-evidence-for-introduce-custom-quickjs-context-profiles.md`

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
5. Implement exactly this deliverable: Implement `getRandomValues` and `randomUUID` through OS CSPRNG.
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
m27-008-a: implement getrandomvalues and randomuuid through os csprng
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record — M27-008-A (PASS)

Deliverable: implement `crypto.getRandomValues` and `crypto.randomUUID` backed directly by OS CSPRNG (`getrandom = "0.2"`).

### Changed files

- `Cargo.toml` & `crates/q-capabilities/Cargo.toml` — added `getrandom = "0.2"`.
- `crates/q-capabilities/src/crypto.rs` (new):
  - `CryptoRandom::get_random_values`: fills destination slice with OS CSPRNG entropy; enforces 64 KiB `MAX_RANDOM_BYTES_LEN` limit (`QuotaExceededError`).
  - `CryptoRandom::random_uuid`: generates RFC 4122 v4 UUID with version and variant bits.
  - Unit tests covering non-zero entropy, quota enforcement, and v4 UUID formatting.
- `crates/q-capabilities/src/lib.rs`: exposed `pub mod crypto;` and re-exports.
- `crates/q-engine-quickjs/src/prelude.rs`:
  - Defined global `crypto.getRandomValues` and `crypto.randomUUID` and `native.crypto` capability handle.
- `crates/q-engine-quickjs/src/worker.rs`:
  - Registered `__velquCryptoGetRandomValues` and `__velquCryptoRandomUUID` native bridges.
  - Added unit test `crypto_getrandomvalues_and_randomuuid_in_js_environment`.
- `packages/cli/src/crypto-random.test.ts` (new):
  - 4 conformance tests for Web Crypto random subset.
- Bookkeeping: STATUS.md, TASK_INDEX.md.

### Tests

- `cargo test -p q-capabilities` — 88 passed (+3 crypto random tests).
- `cargo test -p q-engine-quickjs` — 111 passed (+1 JS crypto test).
- `cargo test -p velqu-runtime` — 31 passed.
- `cargo test -p q-http` — 11 passed.
- `cargo test -p q-bridge` — 11 passed.
- `cargo test -p q-pack` — 98 passed.
- `bun test` — 198 passed (+4 new crypto tests), 0 failed.

### Commands (fresh worktree on parent HEAD ca6e1fa)

- `cargo test -p q-pack` 98 · `-p q-engine-quickjs` 111 · `-p q-http` 11 · `-p q-bridge` 11 · `-p q-capabilities` 88 · `-p velqu-runtime` 31 — pass.
- `bun test` 198 pass / 0 fail; `bun run typecheck`, `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings` — clean.

### Notes

- Guardrail mapping:
  - Entropy source is OS CSPRNG (`getrandom`), avoiding custom or weak pseudo-random generation.
  - Web Crypto compatibility: `getRandomValues` adheres to 64 KiB quota; `randomUUID` complies with RFC 4122 v4.

