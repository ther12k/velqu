---
task_id: M4A-007-Z
parent_task: M4A-007
milestone: M4A
priority: P0
mode: EVIDENCE
status: TODO
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-007-Z — Package evidence for Implement bounded `defer` and lifecycle hooks

## Atomic goal

Create source-backed evidence and handoff for parent task M4A-007; update status only if verification passed.

## Parent intent

Provide after-response cleanup/best-effort work without pretending it is durable jobs.

## Dependencies

- `M4A-007-V` — `tasks/07_m4a_developer_preview/M4A-007-V-verify-implement-bounded-defer-and-lifecycle-hooks.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M4A.md`
- `context/components/devex-beta.md`
- `context/components/evidence.md`

### Source files

- `AGENTS.md`
- `packages/cli/src/index.ts`
- `packages/compiler/src/index.ts`
- `packages/treaty/src/index.ts`
- `packages/testing/src/index.ts`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

## Parent acceptance guardrails

- Response is not delayed beyond defined handoff.
- Deferred work is bounded.
- Shutdown handles or aborts it deterministically.
- Docs warn against durable-job use.

## Targeted commands

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

- Lifecycle tests.
- Load/cleanup tests.
- Operational docs.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m4a-007-z: package evidence for implement bounded defer and lifecycle h
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
