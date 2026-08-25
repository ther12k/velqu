---
task_id: M26-008-V
parent_task: M26-008
milestone: M26
priority: P1
mode: VERIFY
status: PASS
context_card: context/milestones/M26.md
commit_required: true
---

# M26-008-V — Verify Provide explicit v1 compatibility and migration tool

## Atomic goal

Prove every acceptance criterion for parent task M26-008 without broadening scope.

## Parent intent

Keep old packs supportable without contaminating current hot paths.

## Dependencies

- `M26-008-A` — `tasks/03_m26_qpack_v2/M26-008-A-implement-separate-v1-reader-adapter.md`
- `M26-008-B` — `tasks/03_m26_qpack_v2/M26-008-B-provide-velqu-pack-migrate-or-rebuild-guidance.md`
- `M26-008-C` — `tasks/03_m26_qpack_v2/M26-008-C-deprecate-mixed-mode-packs.md`
- `M26-008-D` — `tasks/03_m26_qpack_v2/M26-008-D-test-deterministic-failures-for-unsupported-legacy-features.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M26.md`
- `context/components/qpack-router.md`
- `context/components/devex-beta.md`

### Source files

- `AGENTS.md`
- `crates/q-pack/src/lib.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `crates/q-runtime/src/main.rs`
- `packages/compiler/src/emit.ts`
- `crates/q-pack/tests/fuzz_pack.rs`
- `docs/specs/pack-format-v1.md`
- `packages/treaty/src/index.ts`
- `packages/contract/src/index.ts`
- `packages/testing/src/index.ts`
- `conformance/treaty/treaty.conformance.test.ts`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- Current runtime path allocates no legacy structures.
- Supported v1 pack either migrates or loads through adapter.
- Unsupported pack fails with actionable message.
- Migration does not change public contract.

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

- Compatibility fixtures.
- Migration tests.
- Deprecation documentation.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m26-008-v: verify provide explicit v1 compatibility and migration tool
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record (M26-008-V)

Status: **PASS**. Verification closure only — no code changes. All four
parent guardrails verified against source, tests, and live CLI behavior.

### Guardrail → source → evidence

1. **Current runtime path allocates no legacy structures.**
   `q-runtime/src/main.rs:71` loads via `QPack::load_and_verify_with`
   with an explicit `BytecodePolicy` — the current producer format IS
   mode 1 (`PACK_FORMAT_CURRENT == PACK_FORMAT_LEGACY_V1`, pinned by
   `current_mode_is_pinned_until_native_v2_lands`), so there is exactly
   one allocation path and no adapter double-construction. The qpack2
   zero-copy reader (`PackBytes::open`) returns borrowed/mmap views
   (`Deref to &[u8]`) sharing no types with owned legacy `QPack` trees;
   `legacy_v1` is the only funnel that builds them.
2. **Supported v1 pack either migrates or loads through the adapter.**
   Golden fixture loads through `legacy_v1::read_and_verify_bytes`
   (`loads_committed_v1_fixture`); live end-to-end check this session:
   `velqu pack migrate <golden>` exits 0 with rebuild guidance
   (deterministic per M26-007).
3. **Unsupported packs fail with actionable messages.**
   `unsupported_version_message_is_actionable` + the M26-008-C/D matrix
   (mixed-mode named rejections; determinism pinned byte-identical
   across runs, no addresses/host state). Live check: missing file →
   exit 1 "pack not found".
4. **Migration does not change public contract.** `git diff 44fdaba..HEAD`
   on q-pack shows zero removed public items (`detect_pack_format_mode`
   moved, same signature); runtime and bytecode-tool call sites
   unchanged; all new surface is additive (`legacy_v1`,
   `reject_mixed_mode_bytes`, `MODE2_RESERVED_JSON_KEYS`, fixtures,
   CLI command).

### Command results (fresh on branch m26-008-v)

- `cargo test -p q-pack` — 93 passed; `cargo test -p velqu-runtime` —
  28 passed; `bun test` — 89 passed / 0 fail / 531 expect(); typecheck
  clean. `./scripts/verify` — ALL PASS (exit 0).
