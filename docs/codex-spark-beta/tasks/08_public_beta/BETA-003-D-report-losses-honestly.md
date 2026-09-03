---
task_id: BETA-003-D
parent_task: BETA-003
milestone: BETA
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-003-D — Report losses honestly

## Atomic goal

Report losses honestly.

## Parent intent

Show where cold start and native infrastructure beat or lose to JIT execution.

## Dependencies

- `BETA-003-C` — `tasks/08_public_beta/BETA-003-C-calculate-cumulative-crossover-request-counts.md`

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

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Report losses honestly.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Crossover method is reproducible.
- Cold, warm, CPU, and I/O are not conflated.
- p50/p95/p99, CPU, RSS, errors are included.
- Positioning follows evidence.

## Targeted commands

```bash
bun test
```
```bash
bun run typecheck
```

## Required evidence for this microtask

- Raw crossover data.
- Generated report.
- Public wording draft.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
beta-003-d: report losses honestly
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-003-D) — PASS (2026-09-03)

- Branch/PR: beta-003-d (squash-merged; see git log for final hash)
- Closes: #513

### Changed files
- `benchmarks/harness/losses.ts` (new): honest-loss ledger generator. Reads
  the committed measured evidence (ramp summary BETA-003-B + crossover
  counts BETA-003-C) and mechanically extracts every substantiated loss:
  steady-floor (ratio to class best), crossover-never, crossover-lag,
  no-onset. Nothing hand-selected — the same rules run over whatever the
  evidence contains. Writes `losses.json` + `losses.md`.
- `benchmarks/harness/losses.test.ts` (new): 6 deterministic tests over
  synthetic evidence — floor multiplier, no-onset, never/lag rows, the
  N=1 non-loss case, class-best exclusion, rendering completeness.
- `benchmarks/raw/ramp/losses.json` + `losses.md` (new evidence): 18
  substantiated loss rows generated from run 1788448064944.
- `docs/reports/beta-003-d-honest-losses.md` (new): Velqu's losses stated
  plainly (C2 steady floor 1.59x the class best; behind raw-rust from
  request 1 in C2), declared measurement gaps (no QuickJS-vs-JIT CPU
  scaling data; Velqu absent from the real-world matrices), other
  candidates' losses under the same rules, and the DRAFT public wording
  that leads with the trade-off rather than the win.

### Required evidence

- **Raw crossover data**: losses derived strictly from committed
  `benchmarks/raw/ramp/` evidence (summary.json + crossover-counts.json).
- **Generated report**: `losses.{json,md}` + `docs/reports/
  beta-003-d-honest-losses.md`.
- **Public wording draft**: included, marked DRAFT, and it leads with the
  trade-off ("up to ~1.6x slower at steady state" appears before the
  warmup win); gated on quiet-host reruns, the CPU-scaling gap, owner
  review.

### Commands

- `bun losses.ts` -> 18 substantiated loss rows extracted
- `bun test benchmarks/harness/losses.test.ts` -> 6 pass / 0 fail
- `bun test` -> 373 pass / 0 fail (60 files)
- `bun run typecheck` -> clean
- `./scripts/verify` -> ALL PASS (M0-M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)
  (isolated netns; standing port-3000 environment note, BETA-002-C record)

### Guardrail mapping

- **Crossover method is reproducible**: fixed extraction rules over
  committed raw data; one command regenerates the ledger.
- **Cold, warm, CPU, and I/O are not conflated**: ledger rows are tagged by
  source (ramp cells, crossover pairs) and never mix phases.
- **p50/p95/p99, CPU, RSS, errors are included**: floor ratios computed
  from ramp percentile stats; errors/onset absence become explicit rows.
- **Positioning follows evidence**: the ledger forbids wins-only wording;
  the draft states Velqu's steady-state loss first.

### Standing CI disclosure

CI `verify` workflows stall/fail with zero executed steps on PR creation
across all branches (infrastructure-side, tracked since ~#714); the local
`./scripts/verify` run above is the real gate evidence for this packet.
