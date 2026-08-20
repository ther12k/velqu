---
task_id: M4A-003-V
parent_task: M4A-003
milestone: M4A
priority: P1
mode: VERIFY
status: TODO
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-003-V — Verify Implement project scaffolding

## Atomic goal

Prove every acceptance criterion for parent task M4A-003 without broadening scope.

## Parent intent

Create a minimal correct project without hidden demo credentials or broad dependencies.

## Dependencies

- `M4A-003-A` — `tasks/07_m4a_developer_preview/M4A-003-A-starter-api.md`
- `M4A-003-B` — `tasks/07_m4a_developer_preview/M4A-003-B-treaty-client-example.md`
- `M4A-003-C` — `tasks/07_m4a_developer_preview/M4A-003-C-testing-setup.md`
- `M4A-003-D` — `tasks/07_m4a_developer_preview/M4A-003-D-optional-fetch-profile-choices.md`

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

- Generated project builds/tests/runs.
- Starter follows module/service/contract best practices.
- No database/auth forced into core.
- Dependencies are minimal.

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

- Scaffold snapshot tests.
- Fresh install test.
- Bundle-size report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m4a-003-v: verify implement project scaffolding
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
