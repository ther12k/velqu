---
task_id: BETA-013-V
parent_task: BETA-013
milestone: BETA
priority: P0
mode: VERIFY
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-013-V — Verify Run beta soak and leak qualification

## Atomic goal

Prove every acceptance criterion for parent task BETA-013 without broadening scope.

## Parent intent

Prove no obvious unbounded retention before exposing the runtime publicly.

## Dependencies

- `BETA-013-A` — `tasks/08_public_beta/BETA-013-A-run-at-least-two-hour-mixed-workload-and-at-least-one-million-requests-on-refere.md`
- `BETA-013-B` — `tasks/08_public_beta/BETA-013-B-include-fetch-db-auth-timeouts-cancellation-worker-replacement-and-reload.md`
- `BETA-013-C` — `tasks/08_public_beta/BETA-013-C-track-rss-heap-slots-tasks-queues-pools-and-errors.md`
- `BETA-013-D` — `tasks/08_public_beta/BETA-013-D-analyze-retained-growth.md`

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

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

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

## Required evidence for this microtask

- Soak raw data.
- Memory graphs.
- Leak analysis.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
beta-013-v: verify run beta soak and leak qualification
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-013-V) — PASS (2026-09-04)

- Branch/PR: beta-013-v (squash-merged; see git log for final hash)
- Closes: #589

### Behavior verified

Verification closure for parent task BETA-013 ("Run beta soak and leak qualification"):
- Validated no monotonic unbounded growth across 2.43M+ requests and continuous chaos (QuickJS heap flat within ~201–206 KiB band, process RSS drift is bounded allocator retention at ~0.298 B/req).
- Verified quiescence: 0 pending slots, 0 live native tasks, 0 pending native ops at shutdown.
- Confirmed zero boundary violations and queue limits strictly bounded at capacity.
- Confirmed comprehensive subsystem coverage (outbound fetch, Postgres DB, JWT auth, timeouts, client cancellation, worker replacement, graceful reload/drain).

### Changed files

- `docs/reports/beta-013-v-verify-soak-leak-qualification.md`
- `docs/codex-spark-beta/tasks/08_public_beta/BETA-013-V-verify-run-beta-soak-and-leak-qualification.md`
- `docs/codex-spark-beta/STATUS.md`
- `docs/codex-spark-beta/indexes/TASK_INDEX.md`

### Gates

- `cargo fmt --all --check` — pass
- `cargo clippy --workspace --all-targets -- -D warnings` — pass
- `bun run typecheck` — pass
- `./scripts/validate-okf` — pass
- `./scripts/verify` — ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)

### Disclosures

- Verification closure only; no runtime binary behavior modified.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
