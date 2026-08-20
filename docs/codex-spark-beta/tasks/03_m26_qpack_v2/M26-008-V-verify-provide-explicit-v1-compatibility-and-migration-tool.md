---
task_id: M26-008-V
parent_task: M26-008
milestone: M26
priority: P1
mode: VERIFY
status: TODO
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
