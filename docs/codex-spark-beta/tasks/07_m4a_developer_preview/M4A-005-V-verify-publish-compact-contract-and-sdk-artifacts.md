---
task_id: M4A-005-V
parent_task: M4A-005
milestone: M4A
priority: P1
mode: VERIFY
status: TODO
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-005-V — Verify Publish compact contract and SDK artifacts

## Atomic goal

Prove every acceptance criterion for parent task M4A-005 without broadening scope.

## Parent intent

Support separate frontend repositories without importing server implementation.

## Dependencies

- `M4A-005-A` — `tasks/07_m4a_developer_preview/M4A-005-A-generate-d-ts-client-openapi-contract-lock.md`
- `M4A-005-B` — `tasks/07_m4a_developer_preview/M4A-005-B-tree-shakable-client.md`
- `M4A-005-C` — `tasks/07_m4a_developer_preview/M4A-005-C-version-and-public-contract-hash.md`
- `M4A-005-D` — `tasks/07_m4a_developer_preview/M4A-005-D-package-verification.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M4A.md`
- `context/components/qpack-router.md`
- `context/components/devex-beta.md`

### Source files

- `AGENTS.md`
- `packages/cli/src/index.ts`
- `packages/compiler/src/index.ts`
- `packages/treaty/src/index.ts`
- `packages/testing/src/index.ts`
- `crates/q-pack/src/lib.rs`
- `crates/q-pack/tests/fuzz_pack.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `docs/specs/pack-format-v1.md`
- `packages/contract/src/index.ts`
- `conformance/treaty/treaty.conformance.test.ts`
- `packages/compiler/src/emit.ts`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- Client package contains no server runtime.
- Types remain responsive at large route counts.
- Version mismatch is diagnosable.
- Published artifact is deterministic.

## Targeted commands

```bash
cargo test -p q-pack
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

- Package content test.
- Type-scale report.
- Reproducibility check.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m4a-005-v: verify publish compact contract and sdk artifacts
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
