---
task_id: BETA-003-V
parent_task: BETA-003
milestone: BETA
priority: P1
mode: VERIFY
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-003-V — Verify Run controlled I/O and CPU/JIT crossover suites

## Atomic goal

Prove every acceptance criterion for parent task BETA-003 without broadening scope.

## Parent intent

Show where cold start and native infrastructure beat or lose to JIT execution.

## Dependencies

- `BETA-003-A` — `tasks/08_public_beta/BETA-003-A-run-0-1-5-10-25ms-i-o-payload-matrices-and-cpu-operation-levels.md`
- `BETA-003-B` — `tasks/08_public_beta/BETA-003-B-measure-first-request-through-steady-state.md`
- `BETA-003-C` — `tasks/08_public_beta/BETA-003-C-calculate-cumulative-crossover-request-counts.md`
- `BETA-003-D` — `tasks/08_public_beta/BETA-003-D-report-losses-honestly.md`

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

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

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

- Raw crossover data.
- Generated report.
- Public wording draft.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
beta-003-v: verify run controlled i o and cpu jit crossover suites
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-003-V) — PASS (2026-09-03)

- Branch/PR: beta-003-v (squash-merged; see git log for final hash)
- Closes: #514

### Acceptance-criterion mapping (parent BETA-003)

1. **Crossover method is reproducible**
   - Source: `benchmarks/harness/{ramp,crossover,losses}.ts` +
     `benchmarks/real-world/run-crossover.sh` — one command each, fixed
     rules, pinned deps, committed raw series.
   - Fresh verification on this branch: `bun ramp.ts` re-run -> 8 cells,
     0 errors, steady onset in all; `bun crossover.ts` + `bun losses.ts`
     regenerate from the new series; `./run-crossover.sh 2 1,10` re-run ->
     all 78 real-world cells 0 errors / 0 mismatches, three comparison
     reports PASS.
   - Run-to-run spread observed (e.g. velqu/C2 steady p50 59us in the
     BETA-003-B run vs 23us in the verification run) — consistent with the
     reports' single-shared-host caveat and reinforces the DRAFT gating.
   - Unit tests: `ramp.test.ts` (10), `crossover.test.ts` (9),
     `losses.test.ts` (6) — all deterministic, no servers.
2. **Cold, warm, CPU, and I/O are not conflated**
   - Ramp separates first/warming/steady phases per request; A's matrices
     keep I/O (controlled upstream), payload, and CPU (zero-I/O bounded
     loop) in distinct cells; startup is excluded from crossover counts and
     the exclusion is recorded in the output.
3. **p50/p95/p99, CPU, RSS, errors are included**
   - Ramp summary: percentile stats for first/steady phases, per-process
     RSS, error counts (errors fail loudly). Real-world comparisons carry
     p50/p95/p99/max/errors/mismatches/RSS per cell.
4. **Positioning follows evidence**
   - `losses.ts` mechanically extracts losses (18 rows in the committed
     ledger, including Velqu's C2 steady floor at 1.59x the class best);
     DRAFT wording leads with the trade-off; reports declare measurement
     gaps (no QuickJS-vs-JIT CPU-scaling data) instead of filling them.

### Commands (fresh on this branch)

- `bun ramp.ts` -> 8/8 cells PASS; `bun crossover.ts` -> regenerated;
  `bun losses.ts` -> regenerated
- `./run-crossover.sh 2 1,10` -> PASS (78 cells, 0 errors/mismatches)
- `bun test` -> 373 pass / 0 fail (60 files)
- `cargo test -p q-http` / `-p q-bridge` / `-p velqu-runtime` -> all suites ok
- `cargo fmt --all --check` -> clean; `cargo clippy --workspace
  --all-targets -- -D warnings` -> clean
- `bun run typecheck` -> clean
- `./scripts/verify` -> ALL PASS (M0-M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)
  (isolated netns; standing port-3000 environment note, BETA-002-C record)

### Changed files

- Task record only (verification-only packet; regenerated evidence was
  restored to the committed canonical A/B runs after verification).

### Disclosures

- Verification-only packet; no production or harness behavior changes.
- Run-to-run variance on the shared host is material; canonical numbers
  require a quiet-host rerun (BETA-014).
- Standing: CI `verify` workflows stall/fail with zero executed steps on
  PR creation across all branches (infrastructure-side, tracked since
  ~#714); local `./scripts/verify` is the real gate evidence.
