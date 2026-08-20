---
task_id: M4A-007-C
parent_task: M4A-007
milestone: M4A
priority: P0
mode: IMPLEMENT
status: TODO
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-007-C — Expose metrics

## Atomic goal

Expose metrics.

## Parent intent

Provide after-response cleanup/best-effort work without pretending it is durable jobs.

## Dependencies

- `M4A-007-B` — `tasks/07_m4a_developer_preview/M4A-007-B-separate-cleanup-from-best-effort-work.md`

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
- `crates/q-engine/src/lib.rs`
- `crates/q-runtime/src/serve.rs`
- `docs/reports/`
- `docs/beta/workstreams/OBSERVABILITY_OPERATIONS.md`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Expose metrics.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Response is not delayed beyond defined handoff.
- Deferred work is bounded.
- Shutdown handles or aborts it deterministically.
- Docs warn against durable-job use.

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

## Required evidence for this microtask

- Lifecycle tests.
- Load/cleanup tests.
- Operational docs.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m4a-007-c: expose metrics
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
