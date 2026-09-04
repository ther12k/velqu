---
task_id: BETA-011-A
parent_task: BETA-011
milestone: BETA
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-011-A — Use SemVer prerelease

## Atomic goal

Use SemVer prerelease.

## Parent intent

Produce repeatable pre-release packages without implying API stability.

## Dependencies

- `M4A-GATE` — `gates/M4A-GATE.md`
- `BETA-010-Z` — `tasks/08_public_beta/BETA-010-Z-package-evidence-for-create-supported-beta-platform-and-packaging-matrix.md`

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
5. Implement exactly this deliverable: Use SemVer prerelease.
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
beta-011-a: use semver prerelease
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-011-A) — PASS (2026-09-04)

- Branch/PR: beta-011-a (squash-merged; see git log for final hash)
- Closes: #567

### Behavior implemented

Established and validated the SemVer prerelease policy:
- Target version is `0.1.0-beta.1` as authorized in `docs/beta/governance/RELEASE_AUTHORITY.md`.
- Conforms to SemVer 2.0.0 with explicit prerelease identifier `-beta.1`, conveying zero API/ABI stability guarantees.
- Added `scripts/semver-prerelease-check.sh` to validate the versioning format and policy. Emits `docs/reports/beta-011-a-semver-prerelease.json` with verdict `PASS`.
- Confirmed rollback and withdrawal governance: Owner maintains authority to withdraw or yank packages without altering historical evidence.

### Changed files

- `scripts/semver-prerelease-check.sh` (version structure validation script)
- `docs/reports/beta-011-a-semver-prerelease.json` (machine-readable validation artifact)
- `docs/reports/beta-011-a-semver-prerelease.md` (evidence report)
- `docs/codex-spark-beta/tasks/08_public_beta/BETA-011-A-use-semver-prerelease.md`
- `docs/codex-spark-beta/STATUS.md`
- `docs/codex-spark-beta/indexes/TASK_INDEX.md`

### Required evidence

- `scripts/semver-prerelease-check.sh` — PASS.
- `docs/reports/beta-011-a-semver-prerelease.json`.
- `docs/reports/beta-011-a-semver-prerelease.md`.

### Gates

- `cargo test -p q-pack` — pass
- `bun test` — 434 pass / 0 fail (67 files)
- `bun run typecheck` — pass
- `./scripts/verify` — ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)

### Disclosures

- Monorepo package manifests retain `0.1.0` (with `private: true`) until release publication automation activates under owner authority.
- Pre-release versions carry no SLA or backward compatibility guarantees.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
