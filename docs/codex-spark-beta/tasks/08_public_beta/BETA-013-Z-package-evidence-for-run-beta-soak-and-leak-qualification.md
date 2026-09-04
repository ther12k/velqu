---
task_id: BETA-013-Z
parent_task: BETA-013
milestone: BETA
priority: P0
mode: EVIDENCE
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-013-Z — Package evidence for Run beta soak and leak qualification

## Atomic goal

Create source-backed evidence and handoff for parent task BETA-013; update status only if verification passed.

## Parent intent

Prove no obvious unbounded retention before exposing the runtime publicly.

## Dependencies

- `BETA-013-V` — `tasks/08_public_beta/BETA-013-V-verify-run-beta-soak-and-leak-qualification.md`

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

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

## Parent acceptance guardrails

- No monotonic unbounded growth.
- All resource gauges return near baseline after quiescence.
- No boundary violations.
- Any bounded cache growth is documented.

## Targeted commands

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

- Soak raw data.
- Memory graphs.
- Leak analysis.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
beta-013-z: package evidence for run beta soak and leak qualification
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-013-Z) — PASS (2026-09-04)

- Branch/PR: beta-013-z (squash-merged; see git log for final hash)
- Closes: #590

### Behavior implemented

Evidence packaging and parent task closure for **BETA-013** ("Run beta soak and leak qualification"):
- Flipped parent row `BETA-013` to **PASS** in `docs/beta/04_TASK_LEDGER.md`.
- Consolidated evidence inventory across all child tasks (BETA-013-A through BETA-013-D, and verification BETA-013-V) in `docs/reports/beta-013-z-package-evidence.md`.
- Verified all parent acceptance guardrails: no monotonic unbounded growth; all resource gauges return near baseline after quiescence (0 pending slots/tasks); no boundary violations; bounded allocator retention documented.

### Changed files

- `docs/beta/04_TASK_LEDGER.md`
- `docs/reports/beta-013-z-package-evidence.md`
- `docs/codex-spark-beta/tasks/08_public_beta/BETA-013-Z-package-evidence-for-run-beta-soak-and-leak-qualification.md`
- `docs/codex-spark-beta/STATUS.md`
- `docs/codex-spark-beta/indexes/TASK_INDEX.md`

### Gates

- `cargo fmt --all --check` — pass
- `cargo clippy --workspace --all-targets -- -D warnings` — pass
- `bun run typecheck` — pass
- `./scripts/validate-okf` — pass
- `./scripts/verify` — ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)

### Disclosures

- Evidence packaging and status tracking only; no runtime binary behavior modified.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
