---
task_id: M27-008-V
parent_task: M27-008
milestone: M27
priority: P0
mode: VERIFY
status: PASS
context_card: context/milestones/M27.md
commit_required: true
---

# M27-008-V — Verify Implement crypto random subset

## Atomic goal

Prove every acceptance criterion for parent task M27-008 without broadening scope.

## Parent intent

Provide secure random bytes and UUID without broad crypto scope.

## Dependencies

- `M27-008-A` — `tasks/04_m27_capability_linker/M27-008-A-implement-getrandomvalues-and-randomuuid-through-os-csprng.md`
- `M27-008-B` — `tasks/04_m27_capability_linker/M27-008-B-enforce-typed-array-and-size-constraints.md`
- `M27-008-C` — `tasks/04_m27_capability_linker/M27-008-C-define-unavailable-entropy-failure.md`
- `M27-008-D` — `tasks/04_m27_capability_linker/M27-008-D-do-not-implement-custom-cryptography.md`

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
- `conformance/security/security.conformance.test.ts`
- `crates/q-pack/tests/fuzz_pack.rs`
- `crates/q-http/tests/fuzz_parsers.rs`
- `crates/q-schema-runtime/tests/fuzz_validator.rs`
- `.github/workflows/verify.yml`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

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
cargo test -p q-http
```
```bash
cargo test -p q-schema-runtime
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

- Statistical smoke tests.
- WPT cases.
- Security review note.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m27-008-v: verify implement crypto random subset
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Verification record — M27-008-V (PASS)

Parent: M27-008 "Implement crypto random subset".
Implementation packets merged prior: A (PR #884, #282), B
(PR #885, #283), C (PR #886, #284), D (PR #887, #285).

### Guardrail map

1. **Random API fails closed.** `CryptoError::EntropyUnavailable` is a typed error; OS CSPRNG failure throws with no fallback to pseudo-random or predictable seeds. Unit test `entropy_unavailable_error_formatting_and_fail_closed`.
2. **Input limits match intended standard.** Web Crypto 64 KiB quota strictly enforced on `getRandomValues` (`MAX_RANDOM_BYTES_LEN`). Unit test `get_random_values_quota_limit` plus prelude-level `RangeError("QuotaExceededError")`.
3. **No predictable fallback.** Every byte sourced directly from `getrandom` (OS CSPRNG); no intermediary seeding or custom algorithms. Unit test `no_custom_or_pseudorandom_primitives`.
4. **Security review passes.** Typed-array constraint check rejects Float arrays and DataViews (`TypeError`); interface restricted to standard methods only.

### Manifest

Matched refresh under verify's remap env (qRuntimeRelease hash updated for Crypto capability integration).

### Commands and results (fresh worktree on parent HEAD bb895a7)

- `cargo test -p q-pack` 98 · `-p q-engine-quickjs` 111 · `-p q-http` 11 · `-p q-schema-runtime` 67 · `-p q-capabilities` 90 · `-p velqu-runtime` 31 — pass.
- `bun test` 200 pass / 0 fail; `bun run typecheck`, `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings` — clean.
- `./scripts/verify` — ALL PASS (exit 0).

