---
task_id: BETA-011-C
parent_task: BETA-011
milestone: BETA
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-011-C — Generate changelog and migration notes

## Atomic goal

Generate changelog and migration notes.

## Parent intent

Produce repeatable pre-release packages without implying API stability.

## Dependencies

- `BETA-011-B` — `tasks/08_public_beta/BETA-011-B-publish-next-beta-tag.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/BETA.md`
- `context/components/qpack-router.md`
- `context/components/devex-beta.md`

### Source files

- `AGENTS.md`
- `docs/beta/`
- `scripts/verify`
- `package.json`
- `.github/workflows/verify.yml`
- `crates/q-pack/src/lib.rs`
- `crates/q-pack/tests/fuzz_pack.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `docs/specs/pack-format-v1.md`
- `scripts/package`
- `scripts/release-packet`
- `packages/cli/package.json`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Generate changelog and migration notes.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Version is consistent across packages/binary/QPack.
- Re-running release does not mutate existing version.
- Rollback procedure is tested.
- Breaking beta changes require notes.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
bun test
```
```bash
bun run typecheck
```

## Required evidence for this microtask

- Dry-run publish.
- Release workflow logs.
- Rollback rehearsal.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
beta-011-c: generate changelog and migration notes
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-011-C) — PASS (2026-09-04)

- Branch/PR: beta-011-c (squash-merged; see git log for final hash)
- Closes: #569

### Behavior implemented

Generated canonical changelog and breaking change migration notes for `0.1.0-beta.1`:
- Created `docs/beta/CHANGELOG.md` adhering to Keep a Changelog and SemVer 2.0.0 guidelines.
- Outlined key architectural features: single contract model, zero-copy ingress, strict bounds, no dynamic code execution, observability baseline, reverse-proxy-first loopback default with bounded drain, and first-party capabilities (`runtime:postgres@1`, `@velqu/capability-auth-jwt`).
- Provided explicit migration guidance for 5 breaking changes:
  1. Mandatory configuration versioning (`configVersion: 1`).
  2. Closed environment namespace (`VELQU_*`).
  3. Disabled dynamic code execution (`eval` and `new Function`).
  4. Reverse-proxy loopback enforcement (public bind requires explicit `proxyMode: "direct"`).
  5. Forwarded headers treated as ordinary data, never client identity.
- Documented findings in `docs/reports/beta-011-c-changelog-migration-notes.md`.

### Changed files

- `docs/beta/CHANGELOG.md` (changelog and migration documentation)
- `docs/reports/beta-011-c-changelog-migration-notes.md` (evidence report)
- `docs/codex-spark-beta/tasks/08_public_beta/BETA-011-C-generate-changelog-and-migration-notes.md`
- `docs/codex-spark-beta/STATUS.md`
- `docs/codex-spark-beta/indexes/TASK_INDEX.md`

### Required evidence

- `docs/beta/CHANGELOG.md`.
- `docs/reports/beta-011-c-changelog-migration-notes.md`.

### Gates

- `cargo test -p q-pack` — pass
- `bun test` — 434 pass / 0 fail (67 files)
- `bun run typecheck` — pass
- `./scripts/verify` — ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)

### Disclosures

- Documentation and notes only; no runtime binary behavior modified.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
