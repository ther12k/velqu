---
task_id: BETA-014-V
parent_task: BETA-014
milestone: BETA
priority: P1
mode: VERIFY
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-014-V — Verify Publish canonical beta benchmark report

## Atomic goal

Prove every acceptance criterion for parent task BETA-014 without broadening scope.

## Parent intent

Create an honest comparison for beta users.

## Dependencies

- `BETA-014-A` — `tasks/08_public_beta/BETA-014-A-include-cold-start-categories-warm-microbenchmarks-real-db-auth-i-o-cpu-jit-cros.md`
- `BETA-014-B` — `tasks/08_public_beta/BETA-014-B-pin-all-candidates-artifacts.md`
- `BETA-014-C` — `tasks/08_public_beta/BETA-014-C-retain-raw-data.md`
- `BETA-014-D` — `tasks/08_public_beta/BETA-014-D-have-wording-reviewed.md`

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

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

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

## Required evidence for this microtask

- Benchmark report.
- Raw archive.
- Methodology review.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
beta-014-v: verify publish canonical beta benchmark report
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-014-V) — PASS (2026-09-05)

- Branch/PR: beta-014-v (squash-merged; see git log for final hash)
- Closes: #595

### Verification performed

Every parent BETA-014 acceptance criterion mapped to source and re-confirmed evidence (full matrix in `docs/reports/beta-014-v-verify-canonical-benchmark-report.md`):
- Every number links to raw evidence: all report tables re-derived from committed raw JSON; `validate-benchmark-evidence.py` PASS with zero manifest errors after refresh.
- Fixture-specific wording: report states fixture scope (C0–C3 warm cells, single host, Bun 1.4.0, 5 repetitions, randomized order).
- Velqu losses included: honest-loss section carries the current ramp artifact numbers (2.29× C0 steady floor; no raw-rust overtake in horizon).
- No cloud cold-start claim: guardrail note forbids extrapolation of local process data.
- Pins (9 version tests) and raw retention (5 tests) re-confirmed passing.

No defects found; no new features added.

### Changed files

- `docs/reports/beta-014-v-verify-canonical-benchmark-report.md`
- `docs/codex-spark-beta/tasks/08_public_beta/BETA-014-V-verify-publish-canonical-beta-benchmark-report.md`
- `docs/codex-spark-beta/STATUS.md`
- `docs/codex-spark-beta/indexes/TASK_INDEX.md`

### Gates

- `bun test` — 434 pass / 0 fail (67 files)
- `bun run typecheck` — pass
- `cargo fmt --all --check` — pass
- `cargo clippy --workspace --all-targets -- -D warnings` — pass
- `./scripts/validate-okf` — pass
- `python3 scripts/validate-benchmark-evidence.py` — PASS (errors: [])
- `./scripts/verify` — ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)

### Disclosures

- Verification closure only; no runtime behavior modified.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
