---
task_id: M26-002-B
parent_task: M26-002
milestone: M26
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M26.md
commit_required: true
---

# M26-002-B — Fail closed on mismatch

## Atomic goal

Fail closed on mismatch.

## Parent intent

Prevent loading bytecode or plans under an incompatible engine/runtime build.

## Dependencies

- `M26-002-A` — `tasks/03_m26_qpack_v2/M26-002-A-include-runtime-abi-quickjs-ng-version-build-hash-rquickjs-version-bytecode-form.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M26.md`
- `context/components/qpack-router.md`

### Source files

- `AGENTS.md`
- `crates/q-pack/src/lib.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `crates/q-runtime/src/main.rs`
- `packages/compiler/src/emit.ts`
- `crates/q-pack/tests/fuzz_pack.rs`
- `docs/specs/pack-format-v1.md`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Fail closed on mismatch.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Any fingerprint mismatch rejects before ready.
- Error identifies incompatible dimension.
- Engine upgrades require pack rebuild.
- Cross-target packs are rejected.

## Targeted commands

```bash
cargo test -p q-pack
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

- Fingerprint tests.
- Cross-build fixtures.
- Upgrade lane documentation.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m26-002-b: fail closed on mismatch
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record (M26-002-B)

Status: **PASS**. Fingerprint mismatches fail closed BEFORE ready, with
the incompatible dimensions named:

- `crates/q-pack/src/lib.rs` `verify()`: bytecode packs are checked
  against the HOST target at load — arch, OS, pointer width, and
  endianness each compared; any mismatch rejects as "cross-target pack
  rejected (incompatible dimensions: …)" naming every mismatching
  dimension, the full pack-vs-runtime target description, and the
  "rebuild the pack for this target" remedy (guardrails: fail before
  ready + rebuild requirement). Bytecode WITHOUT a target fingerprint
  cannot prove compatibility and rejects fail-closed (the embed tool
  always stamps one). The pre-existing endianness-only check remains as
  defense in depth behind the richer check.
- Evidence: `cross_target_bytecode_fails_closed_with_dimensions` —
  wrong arch (dimension named + rebuild hint), wrong pointer width,
  flipped endianness, and missing target each reject with the expected
  dimension in the message.

### Tests and evidence

- `cargo test -p q-pack` — 52 + 2 passed (new cross-target suite).
- `cargo test -p q-engine-quickjs` — 1 + 96; `cargo test -p q-http` —
  4 + 6 + 1; `cargo test -p q-schema-runtime` — 58 + 4 + 5;
  `cargo test -p velqu-runtime` — 24 — all passed (the runtime-local
  bytecode suite proves matching-target bytecode still serves).
- `bun test` — 82 passed, 0 failed, 487 expect calls.
- `bun run typecheck` — clean. `cargo fmt --check` — clean.
- `cargo clippy --workspace --all-targets -- -D warnings` — clean.
- `scripts/validate-okf` — clean.
- `./scripts/verify` — all stages pass except the documented
  isolated-worktree benchmark-manifest mismatch (pack bytes changed in
  M26-002-A; canonical proofPack refresh flagged there).

Commit: `b49c9d5`.
