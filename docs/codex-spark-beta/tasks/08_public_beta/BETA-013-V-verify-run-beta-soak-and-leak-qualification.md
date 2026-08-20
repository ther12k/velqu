---
task_id: BETA-013-V
parent_task: BETA-013
milestone: BETA
priority: P0
mode: VERIFY
status: TODO
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-013-V — Verify Run beta soak and leak qualification

## Atomic goal

Prove every acceptance criterion for parent task BETA-013 without broadening scope.

## Parent intent

Prove no obvious unbounded retention before exposing the runtime publicly.

## Dependencies

- `BETA-013-A` — `tasks/08_public_beta/BETA-013-A-run-at-least-two-hour-mixed-workload-and-at-least-one-million-requests-on-refere.md`
- `BETA-013-B` — `tasks/08_public_beta/BETA-013-B-include-fetch-db-auth-timeouts-cancellation-worker-replacement-and-reload.md`
- `BETA-013-C` — `tasks/08_public_beta/BETA-013-C-track-rss-heap-slots-tasks-queues-pools-and-errors.md`
- `BETA-013-D` — `tasks/08_public_beta/BETA-013-D-analyze-retained-growth.md`

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

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- No monotonic unbounded growth.
- All resource gauges return near baseline after quiescence.
- No boundary violations.
- Any bounded cache growth is documented.

## Targeted commands

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

- Soak raw data.
- Memory graphs.
- Leak analysis.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
beta-013-v: verify run beta soak and leak qualification
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
