---
task_id: BETA-016-D
parent_task: BETA-016
milestone: BETA
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-016-D — Run tests/dev/build

## Atomic goal

Run tests/dev/build.

## Parent intent

Confirm a user outside the repository can complete the intended beta journey.

## Dependencies

- `BETA-016-C` — `tasks/08_public_beta/BETA-016-C-scaffold-app.md`

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
5. Implement exactly this deliverable: Run tests/dev/build.
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
beta-016-d: run tests dev build
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result

- Status: PASS. Report: `docs/reports/beta-016-d-run-tests-dev-build.md`.
- External transcript: scaffold `bun run test` passes; `bun run build`
  twice with identical `app.qpack` sha256 `cb00bc37…` (determinism holds
  externally); `bun run check` clean; `velqu dev` on :8084 probed
  (`{"status":"ok"}`, `{"message":"Hello, dev!"}`); production runtime
  on :8081 probed (`{"message":"Hello, world!"}`); `DEVBUILD-OK`.
- Product fixes carried (surfaced by this verification): scaffold
  scripts invoke the CLI via its linked path with `--project .`;
  dev-server resolves the runtime from its install tree + actionable
  error; QUICKSTART link step includes `cli`; test assertions updated.
- Gates: `bun test` 434/0 (netns); `bun run typecheck` pass;
  `cargo test -p velqu-runtime` 37+3; `cargo fmt` clean.
- Standing CI disclosure applies; local gates are the acceptance basis.
