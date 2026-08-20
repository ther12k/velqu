---
task_id: BETA-013-D
parent_task: BETA-013
milestone: BETA
priority: P0
mode: IMPLEMENT
status: TODO
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-013-D — Analyze retained growth

## Atomic goal

Analyze retained growth.

## Parent intent

Prove no obvious unbounded retention before exposing the runtime publicly.

## Dependencies

- `BETA-013-C` — `tasks/08_public_beta/BETA-013-C-track-rss-heap-slots-tasks-queues-pools-and-errors.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/BETA.md`
- `context/components/devex-beta.md`

### Source files

- `AGENTS.md`
- `docs/beta/`
- `scripts/verify`
- `package.json`
- `.github/workflows/verify.yml`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Analyze retained growth.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- No monotonic unbounded growth.
- All resource gauges return near baseline after quiescence.
- No boundary violations.
- Any bounded cache growth is documented.

## Targeted commands

Run the smallest relevant existing test command for the changed component.

## Required evidence for this microtask

- Soak raw data.
- Memory graphs.
- Leak analysis.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
beta-013-d: analyze retained growth
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
