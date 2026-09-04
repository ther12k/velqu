---
task_id: BETA-011-E
parent_task: BETA-011
milestone: BETA
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-011-E — Support yanking/rollback

## Atomic goal

Support yanking/rollback.

## Parent intent

Produce repeatable pre-release packages without implying API stability.

## Dependencies

- `BETA-011-D` — `tasks/08_public_beta/BETA-011-D-create-github-style-release-packet.md`

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
5. Implement exactly this deliverable: Support yanking/rollback.
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
beta-011-e: support yanking rollback
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-011-E) — PASS (2026-09-04)

- Branch/PR: beta-011-e (squash-merged; see git log for final hash)
- Closes: #571

### Behavior implemented

Rehearsed and validated the release yank/rollback procedure (dry-run only, Owner-gated execution):
- Added `scripts/yank-rollback-rehearsal.sh` implementing the withdrawal governance of `docs/beta/governance/RELEASE_AUTHORITY.md`.
- Covers all four Owner withdrawal triggers: incomplete evidence, checksum mismatch, security withdrawal, beta-limits violation.
- Rehearsed actions: `npm dist-tag rm/add` (channel repoint to last stable prerelease), `npm yank`, GitHub release unpublish, and packet-level withdrawal record `{withdrawnVersion, reason, recordedAt, evidenceRewritten: false}`.
- Enforced invariant: withdrawal appends a record and never rewrites historical evidence.
- Emits `docs/reports/beta-011-e-yank-rollback-rehearsal.json` with verdict `PASS`; script exits non-zero if any invariant regresses.

### Changed files

- `scripts/yank-rollback-rehearsal.sh`
- `docs/reports/beta-011-e-yank-rollback-rehearsal.json`
- `docs/reports/beta-011-e-yank-rollback.md`
- `docs/codex-spark-beta/tasks/08_public_beta/BETA-011-E-support-yanking-rollback.md`
- `docs/codex-spark-beta/STATUS.md`
- `docs/codex-spark-beta/indexes/TASK_INDEX.md`

### Required evidence mapping

- Dry-run publish: BETA-011-B (`scripts/publish-tag-dryrun.sh`).
- Release workflow: BETA-011-D (`scripts/release-packet` rehearsal at clean commit, SHA256SUMS all OK).
- Rollback rehearsal: this packet (`scripts/yank-rollback-rehearsal.sh` — PASS).

### Gates

- `cargo test -p q-pack` — pass
- `bun test` — 434 pass / 0 fail (67 files)
- `bun run typecheck` — pass
- `./scripts/verify` — ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)
- `./scripts/yank-rollback-rehearsal.sh` — PASS

### Disclosures

- Dry-run only; actual yank/withdrawal requires an authorized published release and an Owner decision.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
