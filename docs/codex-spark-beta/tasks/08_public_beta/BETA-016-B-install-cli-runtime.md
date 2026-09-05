---
task_id: BETA-016-B
parent_task: BETA-016
milestone: BETA
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-016-B — Install CLI/runtime

## Atomic goal

Install CLI/runtime.

## Parent intent

Confirm a user outside the repository can complete the intended beta journey.

## Dependencies

- `BETA-016-A` — `tasks/08_public_beta/BETA-016-A-fresh-linux-vm-container.md`

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
5. Implement exactly this deliverable: Install CLI/runtime.
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
beta-016-b: install cli runtime
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result

- Status: PASS. Report: `docs/reports/beta-016-b-install-cli-runtime.md`.
- Deliverable: `scripts/beta-external/install-cli-runtime.sh` (fail-closed
  external install from a source archive, per INSTALL.md Step 1) plus the
  environment forward fix (tooling homes chowned to `beta`; manifest probe
  asserts `tooling_homes_writable=yes`).
- External transcript: user `beta` in `velqu-beta-external:0.1.0-beta.1`
  (digest `sha256:9076de16f6ec…a2f5570`), archive sha256
  `9509435365…cfe85d` at commit `cfe3604`, 6 steps → `INSTALL-OK`;
  runtime + CLI `--help` verified; uninstall = `rm -rf ~/velqu`.
- Issues found and fixed: (1) archive without root directory broke
  extraction → `--prefix=velqu/` + guard; (2) root-owned CARGO_HOME broke
  unprivileged crate downloads → chown + probe writability check.
- Gates: `cargo test -p velqu-runtime` 37+3 pass; `bun test` 434/0 (netns);
  `bun run typecheck` pass.
- Standing CI disclosure applies; local gates are the acceptance basis.
