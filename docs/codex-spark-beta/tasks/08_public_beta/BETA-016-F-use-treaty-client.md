---
task_id: BETA-016-F
parent_task: BETA-016
milestone: BETA
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-016-F — Use Treaty client

## Atomic goal

Use Treaty client.

## Parent intent

Confirm a user outside the repository can complete the intended beta journey.

## Dependencies

- `BETA-016-E` — `tasks/08_public_beta/BETA-016-E-deploy-proof-service.md`

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
- `packages/treaty/src/index.ts`
- `packages/contract/src/index.ts`
- `packages/testing/src/index.ts`
- `conformance/treaty/treaty.conformance.test.ts`
- `packages/compiler/src/emit.ts`
- `packages/cli/src/index.ts`
- `crates/q-runtime/src/source_map.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Use Treaty client.
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
beta-016-f: use treaty client
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result

- Status: PASS. Report: `docs/reports/beta-016-f-use-treaty-client.md`.
- External transcript: dev server on 127.0.0.1:3000 with an identity
  precheck; typed Treaty calls live (`Health OK`, `Created greeting`,
  `Message: Greetings from Treaty!`); scaffold contract tests 5 pass /
  0 fail **without skipping**; teardown releases the port; second run
  repeats cleanly.
- Corrections carried: BETA-016-E's rollback script resolved `$HOME` as
  root and silently skipped the service stop (claim corrected; lifecycle
  re-run end to end with owner-aware paths, pidfile fail-closed, port
  release assertions); `kill -0` liveness replaced by kernel state reads
  (zombie-vs-live) and behavioral port checks; `/proc`-scan teardown
  replaces absent `pkill`.
- Gates: `cargo test -p velqu-runtime` 37+3; `bun test` 434/0 (netns);
  `bun run typecheck` pass.
- Standing CI disclosure applies; local gates are the acceptance basis.
