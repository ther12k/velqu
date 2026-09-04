---
task_id: BETA-014-C
parent_task: BETA-014
milestone: BETA
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-014-C — Retain raw data

## Atomic goal

Retain raw data.

## Parent intent

Create an honest comparison for beta users.

## Dependencies

- `BETA-014-B` — `tasks/08_public_beta/BETA-014-B-pin-all-candidates-artifacts.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/BETA.md`
- `context/components/devex-beta.md`
- `context/components/evidence.md`

### Source files

- `AGENTS.md`
- `docs/beta/`
- `scripts/verify`
- `package.json`
- `.github/workflows/verify.yml`
- `benchmarks/harness/`
- `benchmarks/manifest.json`
- `benchmarks/raw/`
- `docs/reports/`
- `scripts/benchmark`
- `scripts/package`
- `scripts/release-packet`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Retain raw data.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Every number links to raw evidence.
- Fixture-specific wording.
- Velqu losses are included.
- No cloud cold-start claim from local process data.

## Targeted commands

```bash
bun test
```
```bash
bun run typecheck
```

## Required evidence for this microtask

- Benchmark report.
- Raw archive.
- Methodology review.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
beta-014-c: retain raw data
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-014-C) — PASS (2026-09-04)

- Branch/PR: beta-014-c (squash-merged; see git log for final hash)
- Closes: #593

### Behavior implemented

Verified and documented the raw benchmark sample retention architecture in `docs/reports/beta-014-c-retain-raw-data.md`:
- Deterministic compression: `deterministicGzip` zeroes volatile metadata to guarantee byte-identical `.jsonl.gz` outputs from identical input rows.
- Lossless verification: exact row counts and raw SHA-256 hashes are verified without drift or truncation.
- Zero cherry-picking: all failed, timed-out, and slow requests are preserved in the raw data files.
- Tested via `benchmarks/real-world/retain.test.ts` (5 passed, 0 failed), `python3 scripts/validate-benchmark-evidence.py` (PASS), and `./scripts/validate-okf` (PASS).

### Changed files

- `docs/reports/beta-014-c-retain-raw-data.md`
- `docs/codex-spark-beta/tasks/08_public_beta/BETA-014-C-retain-raw-data.md`
- `docs/codex-spark-beta/STATUS.md`
- `docs/codex-spark-beta/indexes/TASK_INDEX.md`

### Gates

- `bun test` — 434 pass / 0 fail (67 files)
- `bun run typecheck` — pass
- `cargo fmt --all --check` — pass
- `cargo clippy --workspace --all-targets -- -D warnings` — pass
- `./scripts/validate-okf` — pass
- `./scripts/verify` — ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)

### Disclosures

- Raw data retention verification only; no runtime binary behavior modified.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
