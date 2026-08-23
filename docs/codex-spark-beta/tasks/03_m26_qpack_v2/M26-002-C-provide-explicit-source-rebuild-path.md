---
task_id: M26-002-C
parent_task: M26-002
milestone: M26
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M26.md
commit_required: true
---

# M26-002-C — Provide explicit source rebuild path

## Atomic goal

Provide explicit source rebuild path.

## Parent intent

Prevent loading bytecode or plans under an incompatible engine/runtime build.

## Dependencies

- `M26-002-B` — `tasks/03_m26_qpack_v2/M26-002-B-fail-closed-on-mismatch.md`

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
5. Implement exactly this deliverable: Provide explicit source rebuild path.
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
m26-002-c: provide explicit source rebuild path
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record (M26-002-C)

Status: **PASS**. The explicit source-rebuild path exists end to end:

- **q-pack**: `BytecodePolicy::{Enforce, Skip}` —
  `load_and_verify_with(path, Skip)` verifies the pack with the embedded
  bytecode IGNORED (`verify_without_bytecode`: bytecode nulled, its
  integrity slot unused; every OTHER fingerprint dimension — ABI,
  engine, rquickjs, build hash, integrity, cross-target-free — still
  enforced). The cross-target rejection message now names BOTH recovery
  paths: rebuild the pack for this target, or start with `--no-bytecode`
  to run from source.
- **Runtime**: new `--no-bytecode` flag — loads with `Skip` policy and
  never hands bytecode to the engine (the verified source bundle
  evaluates; ADR-0017's bytecode fast path simply doesn't engage).
- **Evidence**:
  - `source_rebuild_path_loads_cross_target_bytecode_packs` (q-pack) —
    a cross-target bytecode pack rejects with the `--no-bytecode` and
    rebuild hints; the SAME pack verifies on the source path.
  - `no_bytecode_flag_recovers_cross_target_packs_from_source`
    (runtime-local) — embed real bytecode via velqu-bytecode, flip the
    target arch to a foreign machine: normal startup rejects (both
    recovery paths in the message); the SAME pack serves 200 from
    source under `--no-bytecode`.

### Tests and evidence

- `cargo test -p q-pack` — 53 + 2; `cargo test -p q-engine-quickjs` —
  1 + 96; `cargo test -p q-http` — 4 + 6 + 1; `cargo test -p
  q-schema-runtime` — 58 + 4 + 5; `cargo test -p velqu-runtime` — 25 —
  all passed.
- `bun test` — 82 passed, 0 failed, 487 expect calls.
- `bun run typecheck` — clean. `cargo fmt --check` — clean.
- `cargo clippy --workspace --all-targets -- -D warnings` — clean.
- `scripts/validate-okf` — clean.
- `./scripts/verify` — all stages pass except the documented
  isolated-worktree benchmark-manifest mismatch (canonical proofPack
  refresh flagged in M26-002-A).

Commit: `23c8cb7`.
