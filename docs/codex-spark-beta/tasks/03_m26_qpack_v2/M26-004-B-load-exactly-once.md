---
task_id: M26-004-B
parent_task: M26-004
milestone: M26
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M26.md
commit_required: true
---

# M26-004-B — Load exactly once

## Atomic goal

Load exactly once.

## Parent intent

Remove base64 storage/decoding and duplicate production source by default.

## Dependencies

- `M26-004-A` — `tasks/03_m26_qpack_v2/M26-004-A-store-raw-module-bytecode-section.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M26.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`

### Source files

- `AGENTS.md`
- `crates/q-pack/src/lib.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `crates/q-runtime/src/main.rs`
- `packages/compiler/src/emit.ts`
- `crates/q-pack/tests/fuzz_pack.rs`
- `docs/specs/pack-format-v1.md`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Load exactly once.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- No base64 decode at startup.
- No source parse in bytecode production mode.
- Tamper/incompatibility rejects.
- Small-app source mode remains explicit if measured faster.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
cargo test -p q-engine-quickjs
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

- Bytecode integration tests.
- Tamper tests.
- Pack size/startup evidence.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m26-004-b: load exactly once
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record — M26-004-B (PASS)

Deliverable: base64-decode the embedded bytecode exactly once per
production startup. Previously the bytecode was decoded TWICE: once
inside `QPack::verify` (to hash it for the integrity check) and once in
`velqu-runtime` main (to hand bytes to the engine). Now one decode feeds
both.

Changed files:

- `crates/q-pack/src/lib.rs`
- `crates/q-runtime/src/main.rs`
- `crates/q-runtime/tests/runtime_conformance.rs` (fixture initializer
  for the new field only)

Implementation:

- `QPack.decoded_bytecode: Option<Vec<u8>>` — `#[serde(skip)]` cache,
  never serialized, absent on failure/skip.
- `verify()` body refactored to `verify_inner(&self, cache: Option<&mut
  Option<Vec<u8>>>)`; `verify()` wraps with `None` (unchanged behavior);
  new `verify_and_cache_bytecode(&mut self)` passes a slot that the
  bytecode block fills with the SAME decoded buffer used for the sha256
  check — populated only on success.
- `load_and_verify_with`: `BytecodePolicy::Enforce` now calls
  `verify_and_cache_bytecode`; `Skip` path unchanged (never populates).
- `main.rs` handoff: `pack.decoded_bytecode.take()` instead of a second
  `base64_decode` (no re-decode, no copy beyond the take).

Tests (crates/q-pack/src/lib.rs):

- `verify_caches_decoded_bytecode_exactly_once` — host-matching
  bytecode with correct hash verifies and the cache equals the decoded
  bytes; plain `verify()` still works and never populates the field.
- `failed_verify_leaves_no_cached_bytecode` — hash mismatch rejects AND
  leaves the cache empty (a rejected pack cannot hand bytecode to the
  engine).

Commands and results:

- `cargo test -p q-pack` — 73 passed + 2, 0 failed.
- `cargo test -p q-engine-quickjs` — 1 + 97 passed.
- `cargo test -p velqu-runtime` — 26 passed.
- `bun test` — 82 pass / 0 fail / 487 expect().
- `bun run typecheck` — clean.
- `cargo fmt`, `cargo clippy --workspace --all-targets -- -D warnings` — clean.
- `./scripts/verify` — green except the pre-existing documented
  `validate-benchmark-evidence` scoped failure (qRuntimeRelease +
  proofPack manifest hashes; flagged matched-evidence follow-up from
  M26-002-A).

Guardrails: engine still loads bytecode OR source exactly once (worker
either/or unchanged, covered by `bytecode_pack_serves_identically...`);
no source parse in bytecode mode (ADR-0017 path untouched); tamper
rejection strengthened by the failed-verify-no-cache invariant. The v2
section path already carries raw bytes with no base64 (M26-004-A).
