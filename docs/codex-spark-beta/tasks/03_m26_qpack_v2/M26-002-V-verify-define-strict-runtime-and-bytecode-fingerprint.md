---
task_id: M26-002-V
parent_task: M26-002
milestone: M26
priority: P0
mode: VERIFY
status: TODO
context_card: context/milestones/M26.md
commit_required: true
---

# M26-002-V — Verify Define strict runtime and bytecode fingerprint

## Atomic goal

Prove every acceptance criterion for parent task M26-002 without broadening scope.

## Parent intent

Prevent loading bytecode or plans under an incompatible engine/runtime build.

## Dependencies

- `M26-002-A` — `tasks/03_m26_qpack_v2/M26-002-A-include-runtime-abi-quickjs-ng-version-build-hash-rquickjs-version-bytecode-form.md`
- `M26-002-B` — `tasks/03_m26_qpack_v2/M26-002-B-fail-closed-on-mismatch.md`
- `M26-002-C` — `tasks/03_m26_qpack_v2/M26-002-C-provide-explicit-source-rebuild-path.md`
- `M26-002-D` — `tasks/03_m26_qpack_v2/M26-002-D-never-silently-fall-back.md`

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

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

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

- Fingerprint tests.
- Cross-build fixtures.
- Upgrade lane documentation.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m26-002-v: verify define strict runtime and bytecode fingerprint
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
