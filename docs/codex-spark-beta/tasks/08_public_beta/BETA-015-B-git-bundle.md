---
task_id: BETA-015-B
parent_task: BETA-015
milestone: BETA
priority: P0
mode: IMPLEMENT
status: TODO
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-015-B — Git bundle

## Atomic goal

Git bundle.

## Parent intent

Create a self-verifying public-beta packet.

## Dependencies

- `BETA-015-A` — `tasks/08_public_beta/BETA-015-A-source-zip.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/BETA.md`
- `context/components/devex-beta.md`
- `context/components/evidence.md`

### Source files

- `AGENTS.md`
- `SOURCE-COMMIT.txt`
- `SHA256SUMS.txt`
- `REVIEW_INDEX.json`
- `EVIDENCE_INDEX.json`
- `TASKS.production.json`
- `docs/beta/00_CURRENT_BASELINE.md`
- `docs/beta/04_TASK_LEDGER.md`
- `scripts/release-packet`
- `scripts/validate-production-plan`
- `scripts/validate-okf`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Git bundle.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Checksums verify from release directory.
- Artifacts map to one source commit.
- SBOM identifies dependencies/licenses.
- No stale historical metadata is current.

## Targeted commands

Run the smallest relevant existing test command for the changed component.

## Required evidence for this microtask

- Release packet.
- Verification transcript.
- Artifact inventory.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
beta-015-b: git bundle
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
