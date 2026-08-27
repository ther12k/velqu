---
task_id: M27-008-Z
parent_task: M27-008
milestone: M27
priority: P0
mode: EVIDENCE
status: PASS
context_card: context/milestones/M27.md
commit_required: true
---

# M27-008-Z — Package evidence for Implement crypto random subset

## Atomic goal

Create source-backed evidence and handoff for parent task M27-008; update status only if verification passed.

## Parent intent

Provide secure random bytes and UUID without broad crypto scope.

## Dependencies

- `M27-008-V` — `tasks/04_m27_capability_linker/M27-008-V-verify-implement-crypto-random-subset.md`

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

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

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

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m27-008-z: package evidence for implement crypto random subset
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Evidence package

- Status: **PASS**. Parent verification M27-008-V merged in PR #888
  at commit `27fdf8236a63c02d9463906ceb3bcafecf1a04fc`; issue #286
  is closed. Based on clean parent HEAD `bb895a7` (queue-regen).
- Parent acceptance matrix: `M27-008-V` maps all four guardrails
  (fail-closed random API, Web Crypto standard limits, no predictable
  fallback to custom algorithms, and security review passing).
- Source-backed implementation records:
  - `M27-008-A` (PR #884, #282 closed): `CryptoRandom::get_random_values`
    and `random_uuid` backed by OS CSPRNG (`getrandom = "0.2"`).
  - `M27-008-B` (PR #885, #283 closed): typed-array constraints rejecting
    Float/DataView with `TypeError`, enforcing 64 KiB quota.
  - `M27-008-C` (PR #886, #284 closed): fail-closed entropy failure behavior.
  - `M27-008-D` (PR #887, #285 closed): no custom cryptography implemented;
    interface restricted to standard methods only.
  - `M27-008-V` (PR #888, #286 closed): verification closure + matched manifest refresh.
- Canonical evidence artifacts:
  - Tests: `q-capabilities` 90 passed (+5 crypto tests), `q-engine-quickjs` 111 passed
    (+2 JS crypto integration tests), `bun test` 200 passed (+7 crypto tests).
  - Manifest: `benchmarks/manifest.json` matched refresh under verify remap environment.
- Exact verification (fresh on this branch): `cargo test` across all crates passes;
  `bun test` 200/0; typecheck, fmt --check, clippy `-D warnings` clean;
  `./scripts/verify` — ALL PASS (exit 0).
- Status bookkeeping: ledger marks M27-008 PASS; TASK_INDEX marks M27-008-Z PASS.
  Queues expose M27-009-A next.
- Remaining scope: M27-009+ (capability SDK & inspection surface), M27-GATE.
