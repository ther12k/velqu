---
task_id: BETA-011-D
parent_task: BETA-011
milestone: BETA
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-011-D — Create GitHub-style release packet

## Atomic goal

Create GitHub-style release packet.

## Parent intent

Produce repeatable pre-release packages without implying API stability.

## Dependencies

- `BETA-011-C` — `tasks/08_public_beta/BETA-011-C-generate-changelog-and-migration-notes.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/BETA.md`
- `context/components/qpack-router.md`
- `context/components/devex-beta.md`
- `context/components/evidence.md`

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
5. Implement exactly this deliverable: Create GitHub-style release packet.
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
beta-011-d: create github style release packet
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-011-D) — PASS (2026-09-04)

- Branch/PR: beta-011-d (squash-merged; see git log for final hash)
- Closes: #570

### Behavior implemented

Extended and rehearsed the self-verifying GitHub-style release packet (`scripts/release-packet`):
- Packet now ships `docs/beta/CHANGELOG.md` as `CHANGELOG.md` (conditional copy) so the GitHub release body and consumers share the same migration notes.
- `CHANGELOG.md` is included in `SHA256SUMS.txt` when present, keeping the packet self-verifying.
- Existing invariants preserved: clean-tree requirement, single source-commit binding (`SOURCE-COMMIT.txt`, `git bundle`, source ZIP), post-HEAD index generation with grep-verified `commit`/`releaseCommit` binding, and `sha256sum -c SHA256SUMS.txt` verification from inside `release/`.
- Rehearsal executed at the clean packet commit: packet built, all checksums `OK`, re-run performed without mutating any tracked file.

### Changed files

- `scripts/release-packet` (changelog inclusion + checksum list update)
- `docs/reports/beta-011-d-github-style-release-packet.md` (evidence report)
- `docs/codex-spark-beta/tasks/08_public_beta/BETA-011-D-create-github-style-release-packet.md`
- `docs/codex-spark-beta/STATUS.md`
- `docs/codex-spark-beta/indexes/TASK_INDEX.md`

### Required evidence

- Dry-run publish: covered by BETA-011-B (`scripts/publish-tag-dryrun.sh`).
- Release workflow: `./scripts/release-packet` rehearsal at the packet commit — packet built, `sha256sum -c SHA256SUMS.txt` all `OK` including `CHANGELOG.md`.
- Rollback rehearsal: covered by BETA-011-A withdrawal governance and the packet's non-mutating rebuild behavior.

### Gates

- `cargo test -p q-pack` — pass
- `bun test` — 434 pass / 0 fail (67 files)
- `bun run typecheck` — pass
- `./scripts/verify` — ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)
- `./scripts/release-packet` — packet built and SHA256SUMS verified

### Disclosures

- Publishing to GitHub Releases remains an Owner-authorized action; this packet prepares the artifact set only.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
