---
task_id: BETA-014-Z
parent_task: BETA-014
milestone: BETA
priority: P1
mode: EVIDENCE
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-014-Z — Package evidence for Publish canonical beta benchmark report

## Atomic goal

Create source-backed evidence and handoff for parent task BETA-014; update status only if verification passed.

## Parent intent

Create an honest comparison for beta users.

## Dependencies

- `BETA-014-V` — `tasks/08_public_beta/BETA-014-V-verify-publish-canonical-beta-benchmark-report.md`

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

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

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
```bash
cargo fmt --check
```
```bash
cargo clippy --workspace --all-targets -- -D warnings
```
```bash
./scripts/verify
```
```bash
./scripts/validate-okf
```

## Required evidence for this microtask

- Benchmark report.
- Raw archive.
- Methodology review.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
beta-014-z: package evidence for publish canonical beta benchmark report
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-014-Z) — PASS (2026-09-05)

- Branch/PR: beta-014-z (squash-merged; see git log for final hash)
- Closes: #596

### Evidence packaged

Full evidence closure for parent BETA-014; ledger row flipped to PASS in `docs/beta/04_TASK_LEDGER.md`:
- Packet inventory A–D + V with canonical evidence paths and PR numbers (#1197–#1201) in `docs/reports/beta-014-z-package-evidence.md`.
- All four acceptance guardrails mapped to evidence: raw-evidence linkage (validate-benchmark-evidence zero errors), fixture-specific wording, honest losses (2.29× C0 floor, no raw-rust overtake), and no cloud cold-start extrapolation.
- Pins (9 tests) and raw retention (5 tests) passing as part of the full 434-test suite.

### Changed files

- `docs/beta/04_TASK_LEDGER.md` (BETA-014 → PASS)
- `docs/reports/beta-014-z-package-evidence.md`
- `docs/codex-spark-beta/tasks/08_public_beta/BETA-014-Z-package-evidence-for-publish-canonical-beta-benchmark-report.md`
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

- Evidence packaging and status tracking only; no runtime behavior modified.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
