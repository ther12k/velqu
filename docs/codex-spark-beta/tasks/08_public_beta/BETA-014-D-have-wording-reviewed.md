---
task_id: BETA-014-D
parent_task: BETA-014
milestone: BETA
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-014-D — Have wording reviewed

## Atomic goal

Have wording reviewed.

## Parent intent

Create an honest comparison for beta users.

## Dependencies

- `BETA-014-C` — `tasks/08_public_beta/BETA-014-C-retain-raw-data.md`

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
5. Implement exactly this deliverable: Have wording reviewed.
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
beta-014-d: have wording reviewed
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-014-D) — PASS (2026-09-05)

- Branch/PR: beta-014-d (squash-merged; see git log for final hash)
- Closes: #594

### Behavior implemented

Completed a source-backed wording review and corrected `docs/reports/beta-014-a-canonical-benchmark-report.md`:
- Replaced unsupported warm benchmark values with medians computed from five raw repetitions in `benchmarks/raw/warm/summary.json`.
- Corrected the honest-loss section to the current ramp artifact: Velqu C0 steady p50 55 µs vs 24 µs class best (2.29×), with no Velqu overtake of raw-rust in the recorded 100-request C0/C2 horizons.
- Removed unsupported idle/peak RSS, Node/Fastify, and cost claims; added measured same-cell RSS snapshots and explicit cost-claim limitations.
- Preserved fixture-specific wording, raw archive references, loss reporting, and the prohibition on cloud cold-start extrapolation from local process data.

### Changed files

- `docs/reports/beta-014-a-canonical-benchmark-report.md`
- `docs/reports/beta-014-d-wording-review.md`
- `docs/codex-spark-beta/tasks/08_public_beta/BETA-014-D-have-wording-reviewed.md`
- `docs/codex-spark-beta/STATUS.md`
- `docs/codex-spark-beta/indexes/TASK_INDEX.md`

### Gates

- `bun test` — 434 pass / 0 fail (67 files)
- `bun test benchmarks/real-world/retain.test.ts` — 5 pass / 0 fail
- `bun test benchmarks/real-world/versions.test.ts` — 9 pass / 0 fail
- `bun run typecheck` — pass
- `cargo fmt --all --check` — pass
- `cargo clippy --workspace --all-targets -- -D warnings` — pass
- `./scripts/validate-okf` — pass
- `./scripts/verify` — ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)

### Disclosures

- Documentation/evidence correction only; no runtime binary behavior modified.
- The benchmark report makes no universal performance, production-readiness, SLA, cloud cold-start, or cost-normalized claim.
