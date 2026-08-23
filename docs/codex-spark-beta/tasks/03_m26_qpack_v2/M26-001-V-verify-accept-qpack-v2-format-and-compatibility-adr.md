---
task_id: M26-001-V
parent_task: M26-001
milestone: M26
priority: P0
mode: VERIFY
status: PASS
context_card: context/milestones/M26.md
commit_required: true
---

# M26-001-V — Verify Accept QPack v2 format and compatibility ADR

## Atomic goal

Prove every acceptance criterion for parent task M26-001 without broadening scope.

## Parent intent

Freeze the binary format goals, trust model, compatibility, and migration rules.

## Dependencies

- `M26-001-A` — `tasks/03_m26_qpack_v2/M26-001-A-define-numeric-current-mode-and-legacy-v1-adapter.md`
- `M26-001-B` — `tasks/03_m26_qpack_v2/M26-001-B-specify-section-directory-alignment-bounds-optional-sections-and-versioning.md`
- `M26-001-C` — `tasks/03_m26_qpack_v2/M26-001-C-separate-integrity-from-authenticity.md`
- `M26-001-D` — `tasks/03_m26_qpack_v2/M26-001-D-define-debug-source-sidecar-policy.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M26.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`

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
- `crates/q-http/src/lib.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- Unknown versions fail closed.
- Current mode has no legacy handler table.
- Compatibility policy is explicit.
- Untrusted arbitrary bytecode is forbidden.

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

- ADR.
- Binary layout diagrams.
- Compatibility matrix.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m26-001-v: verify accept qpack v2 format and compatibility adr
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record (M26-001-V)

Status: **PASS**. Every parent M26-001 acceptance guardrail maps to source
and passing tests; all verification commands were run fresh on this branch
(no code changes — verification closure only).

### Guardrail → source → evidence

1. **Unknown versions fail closed.**
   - `detect_pack_format_mode` (q-pack, M26-001-A) is the first dispatch
     in `verify()`; `tests::unknown_versions_fail_closed` drives
     v{0,2,3,u32::MAX} — each rejects with "not supported … fail closed"
     naming the supported adapter; the qpack2 section's
     `mode_two_still_fails_closed_before_native_adapter` pins that v2
     bytes never reach a native adapter by accident.
2. **Current mode has no legacy handler table.**
   - `tests::numeric_pack_with_handler_table_is_rejected` — a numeric
     pack carrying handlerTable rejects ("must not carry handlerTable");
     `numeric_pack_without_compiled_router_is_rejected` and
     `accepts_valid_numeric_pack` pin the current-mode shape.
3. **Compatibility policy is explicit.**
   - ADR-0024 (numeric mode policy, legacy v1 adapter as the named
     compatibility boundary, compatibility matrix, migration rules) with
     the spec header cross-reference; ADR-0025 (section directory) +
     `docs/specs/pack-format-v2.md` (64-byte header, alignment, bounds)
     define the v2 layout; ADR-0026 separates integrity from
     authenticity; ADR-0027 fixes the debug/source sidecar policy with a
     runtime-independence test.
4. **Untrusted arbitrary bytecode is forbidden.**
   - The trust model (ADR-0024): embedded bytecode is compiler-owned and
     integrity-pinned, never a loading path for arbitrary bytes —
     `verify()` rejects `bundleBytecode` without
     `integrity.bytecodeSha256`, any hash mismatch ("tampered or
     corrupt"), and integrity declaring bytecode with no bytecode
     present; engine/ABI/binding mismatches reject
     (`rejects_engine_mismatch`, `rejects_abi_mismatch_and_duplicate_ids`);
     the runtime-local
     `bytecode_pack_serves_identically_and_mismatch_fails_before_ready`
     proves tampered bytecode fails BEFORE ready.

### Command results (this branch, fresh worktree)

- `cargo test -p q-pack` — 48 + 2 passed.
- `cargo test -p q-engine-quickjs` — 1 + 96 passed.
- `cargo test -p q-http` — 4 + 6 + 1 passed.
- `cargo test -p velqu-runtime` — 24 integration passed.
- `bun test` — 81 passed, 0 failed, 481 expect calls.
- `bun run typecheck` — clean. `cargo fmt --check` — clean.
- `cargo clippy --workspace --all-targets -- -D warnings` — clean.
- `scripts/validate-okf` — links checked, 0 errors.
- `./scripts/verify` — ALL stages pass (the benchmark-manifest mismatch
  known from isolated worktrees no longer reproduces — the manifest was
  refreshed on master with matched artifacts).

Changed files: this record, `docs/codex-spark-beta/STATUS.md`,
`docs/codex-spark-beta/indexes/TASK_INDEX.md` (verification closure only).

Commit: `4911842`.
