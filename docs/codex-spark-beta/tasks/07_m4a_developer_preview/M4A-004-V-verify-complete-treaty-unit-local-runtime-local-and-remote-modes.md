---
task_id: M4A-004-V
parent_task: M4A-004
milestone: M4A
priority: P0
mode: VERIFY
status: TODO
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-004-V — Verify Complete Treaty unit-local, runtime-local, and remote modes

## Atomic goal

Prove every acceptance criterion for parent task M4A-004 without broadening scope.

## Parent intent

Deliver Eden-quality type-safe clients and distinct test fidelity levels.

## Dependencies

- `M4A-004-A` — `tasks/07_m4a_developer_preview/M4A-004-A-unit-local-direct-generated-dispatcher.md`
- `M4A-004-B` — `tasks/07_m4a_developer_preview/M4A-004-B-runtime-local-actual-rust-quickjs-process.md`
- `M4A-004-C` — `tasks/07_m4a_developer_preview/M4A-004-C-remote-fetch-client.md`
- `M4A-004-D` — `tasks/07_m4a_developer_preview/M4A-004-D-exact-method-body-query-status-problem-typing.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M4A.md`
- `context/components/devex-beta.md`

### Source files

- `AGENTS.md`
- `packages/cli/src/index.ts`
- `packages/compiler/src/index.ts`
- `packages/treaty/src/index.ts`
- `packages/testing/src/index.ts`
- `packages/contract/src/index.ts`
- `conformance/treaty/treaty.conformance.test.ts`
- `packages/compiler/src/emit.ts`
- `crates/q-runtime/src/source_map.rs`
- `examples/proof/`
- `README.md`
- `docs/beta/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- No public `any`.
- 2xx data and non-2xx errors narrow correctly.
- Undeclared status is a contract error.
- All modes share the same contract.

## Targeted commands

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

- Negative type tests.
- Mode parity tests.
- Typecheck scale benchmark.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m4a-004-v: verify complete treaty unit local runtime local and remote m
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
