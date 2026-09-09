---
task_id: BETA-016-E
parent_task: BETA-016
milestone: BETA
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-016-E — Deploy proof service

## Atomic goal

Deploy proof service.

## Parent intent

Confirm a user outside the repository can complete the intended beta journey.

## Dependencies

- `BETA-016-D` — `tasks/08_public_beta/BETA-016-D-run-tests-dev-build.md`

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
- `scripts/package`
- `scripts/release-packet`
- `packages/cli/package.json`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Deploy proof service.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- No local unpublished dependency.
- Tutorial succeeds verbatim.
- Failures produce actionable diagnostics.
- Artifacts can be rolled back/uninstalled.

## Targeted commands

```bash
bun test
```
```bash
bun run typecheck
```

## Required evidence for this microtask

- External transcript.
- Environment manifest.
- Issues and resolutions.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
beta-016-e: deploy proof service
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result

- Status: PASS. Report: `docs/reports/beta-016-e-deploy-proof-service.md`.
- External transcript: proof pack built from the installed tree; runtime
  serves it on `127.0.0.1:3000` in reverse-proxy mode (asserted from the
  startup log); nginx edge on `127.0.0.1:8080` (operator provisioning);
  through-edge probes match INSTALL.md verbatim (`{"status":"ok"}`,
  `{"message":"Hello beta"}`, ready) — `VERIFY-OK`; rollback removes the
  edge, SIGTERM-stops the service (exit enforced), removes artifacts —
  `ROLLBACK-OK`; post-rollback verify fails closed.
- Deliverable: `scripts/beta-external/deploy-proof-service.sh`
  (app|edge|verify|rollback subcommands).
- Gates: `bun test` 434/0 (netns); `bun run typecheck` pass.
- Standing CI disclosure applies; local gates are the acceptance basis.
