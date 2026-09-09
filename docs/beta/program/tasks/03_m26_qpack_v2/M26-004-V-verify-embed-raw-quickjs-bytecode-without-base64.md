---
task_id: M26-004-V
parent_task: M26-004
milestone: M26
priority: P0
mode: VERIFY
status: PASS
context_card: context/milestones/M26.md
commit_required: true
---

# M26-004-V — Verify Embed raw QuickJS bytecode without base64

## Atomic goal

Prove every acceptance criterion for parent task M26-004 without broadening scope.

## Parent intent

Remove base64 storage/decoding and duplicate production source by default.

## Dependencies

- `M26-004-A` — `tasks/03_m26_qpack_v2/M26-004-A-store-raw-module-bytecode-section.md`
- `M26-004-B` — `tasks/03_m26_qpack_v2/M26-004-B-load-exactly-once.md`
- `M26-004-C` — `tasks/03_m26_qpack_v2/M26-004-C-make-source-optional-sidecar-development-section.md`
- `M26-004-D` — `tasks/03_m26_qpack_v2/M26-004-D-include-prelude-and-handler-manifest-in-the-compiled-module.md`

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

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

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

- Bytecode integration tests.
- Tamper tests.
- Pack size/startup evidence.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m26-004-v: verify embed raw quickjs bytecode without base64
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Verification record — M26-004-V (PASS)

Parent: M26-004 "Embed raw QuickJS bytecode without base64" (remove
base64 storage/decoding and duplicate production source by default).
All four implementation dependencies (M26-004-A/B/C/D) merged on master
before this branch (PRs #793/#794/#795/#796; issues #198–#201 closed).

### Acceptance criterion mapping

1. **No base64 decode at startup.**
   v2 path: `qpack2::graph::bytecode_section` stores bytecode verbatim —
   `bytecode_section_round_trips_raw_bytes` proves bytes outside the
   base64 alphabet survive byte-for-byte (structural no-transform
   proof); `bytecode_section_in_bound_file_and_tamper_rejected` proves
   tamper rejection in the bound file.
   v1/transition path: production startup decodes exactly ONCE —
   `verify_and_cache_bytecode` shares one buffer between the integrity
   hash and the engine handoff; `main.rs` takes
   `pack.decoded_bytecode.take()`. Tests:
   `verify_caches_decoded_bytecode_exactly_once`,
   `failed_verify_leaves_no_cached_bytecode`.

2. **No source parse in bytecode production mode.**
   `worker.rs`: spawn skips `ctx.eval(PRELUDE)` when
   `embedded_prelude` (the compiled module carries prelude + handler
   manifest, M26-004-D); the bytecode branch of `load()` feeds bytes
   straight to `Module::load` (no parse). Test:
   `embedded_prelude_pack_serves_identically_and_source_recovery_works`
   — serving identical (C0/C3/JS-JSON) from module-only bytecode;
   legacy marker-less bytecode path also green
   (`bytecode_pack_serves_identically_and_mismatch_fails_before_ready`).

3. **Tamper/incompatibility rejects.**
   Bytecode sha256 mismatch exits non-zero before ready (step 3 of the
   legacy bytecode test); fingerprint dimensions fail closed
   (`rejects_engine_mismatch`, `rejects_rquickjs_mismatch_with_dimension`,
   `rejects_build_hash_mismatch_with_dimension`,
   `cross_target_bytecode_fails_closed_with_dimensions`,
   `hash_valid_garbage_bytecode_rejects_before_ready`); marker
   vocabulary closed (`bundle_prelude_marker_rules`; worker rejects
   embedded-without-bytecode); v2 tamper with repaired content hash
   still caught by the execution-integrity binding
   (`bytecode_section_in_bound_file_and_tamper_rejected`).

4. **Small-app source mode remains explicit if measured faster.**
   `--no-bytecode` is the explicit, sanctioned source path:
   `no_bytecode_flag_recovers_cross_target_packs_from_source` boots
   embedded-prelude packs from source with the host prelude (also in
   the M26-004-D test); `verify_without_bytecode` clears layout
   markers; no silent fallback anywhere (`BytecodePolicy` two-variant
   pinning, `source_rebuild_path_loads_cross_target_bytecode_packs`).

### Changed files

- This task record only. Verification packet: no production-code change
  was required; no unrelated findings needing follow-up tasks.

### Commands and results (fresh worktree on parent HEAD)

- `cargo test -p q-pack` — 75 + 2 passed.
- `cargo test -p q-router` — 15 passed.
- `cargo test -p q-engine-quickjs` — 1 + 97 passed.
- `cargo test -p velqu-runtime` — 28 passed.
- `bun test` — 83 pass / 0 fail / 487 expect().
- `bun run typecheck` — clean.
- `cargo fmt --check` — clean.
- `cargo clippy --workspace --all-targets -- -D warnings` — clean.
- `./scripts/verify` — all gates green except the pre-existing
  documented `validate-benchmark-evidence` scoped failure
  (qRuntimeRelease + proofPack manifest hashes; flagged matched-evidence
  follow-up from M26-002-A, not altered here).
