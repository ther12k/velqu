---
task_id: BETA-016-A
parent_task: BETA-016
milestone: BETA
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-016-A — Fresh Linux VM/container

## Atomic goal

Fresh Linux VM/container.

## Parent intent

Confirm a user outside the repository can complete the intended beta journey.

## Dependencies

- `BETA-011-Z` — `tasks/08_public_beta/BETA-011-Z-package-evidence-for-automate-beta-publishing-and-versioning.md`
- `BETA-012-Z` — `tasks/08_public_beta/BETA-012-Z-package-evidence-for-complete-beta-documentation-and-limitations.md`
- `BETA-015-Z` — `tasks/08_public_beta/BETA-015-Z-package-evidence-for-generate-beta-release-evidence-sbom-and-checksums.md`

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
- `crates/q-runtime/src/serve.rs`
- `crates/q-runtime/src/main.rs`
- `examples/proof/`
- `scripts/package`
- `scripts/release-packet`
- `packages/cli/package.json`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Fresh Linux VM/container.
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
beta-016-a: fresh linux vm container
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result

- Status: PASS. Report: `docs/reports/beta-016-a-fresh-linux-vm-container.md`.
- Deliverable: `scripts/beta-external/` (pinned Dockerfile, fail-closed
  manifest probe, build script) → image
  `velqu-beta-external:0.1.0-beta.1`
  (digest `sha256:a3df266bf73e…0b3da58`), Debian 12 bookworm x86_64,
  Bun 1.4.0, Rust 1.96.0 minimal (repository lockfile), unprivileged
  `beta` user, zero Velqu material (`fresh=no-velqu-material`).
- External transcript + environment manifest + issues/resolutions are in
  the report; the environment is externally reproducible via
  `scripts/beta-external/build-env.sh`.
- Gates: `cargo test -p velqu-runtime` pass; `bun test` 434/0 (netns);
  `bun run typecheck` pass.
- Standing CI disclosure applies; local gates are the acceptance basis.
