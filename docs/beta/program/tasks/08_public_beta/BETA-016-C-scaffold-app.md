---
task_id: BETA-016-C
parent_task: BETA-016
milestone: BETA
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-016-C — Scaffold app

## Atomic goal

Scaffold app.

## Parent intent

Confirm a user outside the repository can complete the intended beta journey.

## Dependencies

- `BETA-016-B` — `tasks/08_public_beta/BETA-016-B-install-cli-runtime.md`

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
- `packages/cli/src/index.ts`
- `crates/q-runtime/src/source_map.rs`
- `examples/proof/`
- `README.md`
- `scripts/package`
- `scripts/release-packet`
- `packages/cli/package.json`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Scaffold app.
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
cargo test -p velqu-runtime
```
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
beta-016-c: scaffold app
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result

- Status: PASS. Report: `docs/reports/beta-016-c-scaffold-app.md`.
- Deliverable: `scripts/beta-external/scaffold-app.sh` — external-user
  scaffold verification using the QUICKSTART `create` command verbatim,
  documented `@velqu/{core,schema,treaty}` links, structure checks, and
  `velqu check` (must report 3 routes).
- External transcript: `create hello-velqu --name hello-velqu` produced
  the documented scaffold (package.json, src/app.ts, health + greetings
  modules); `velqu check` → "3 routes — clean"; `SCAFFOLD-OK`;
  uninstall `rm -rf ~/hello-velqu`.
- Issues recorded: (1) `create --help` silently scaffolds a default
  project instead of printing help — diagnostics wart logged for the
  parent rollup, journey unaffected; (2) probe pollution incident
  cleaned and verification re-run pristine.
- Gates: `cargo test -p velqu-runtime` 37+3 pass; `bun test` 434/0
  (netns); `bun run typecheck` pass.
- Standing CI disclosure applies; local gates are the acceptance basis.
