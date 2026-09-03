---
task_id: BETA-003-B
parent_task: BETA-003
milestone: BETA
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-003-B — Measure first request through steady state

## Atomic goal

Measure first request through steady state.

## Parent intent

Show where cold start and native infrastructure beat or lose to JIT execution.

## Dependencies

- `BETA-003-A` — `tasks/08_public_beta/BETA-003-A-run-0-1-5-10-25ms-i-o-payload-matrices-and-cpu-operation-levels.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/BETA.md`
- `context/components/ingress-bridge.md`
- `context/components/devex-beta.md`
- `context/components/evidence.md`

### Source files

- `AGENTS.md`
- `docs/beta/`
- `scripts/verify`
- `package.json`
- `.github/workflows/verify.yml`
- `crates/q-http/src/lib.rs`
- `crates/q-bridge/src/lib.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-runtime/tests/runtime_conformance.rs`
- `benchmarks/harness/`
- `benchmarks/manifest.json`
- `benchmarks/raw/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Measure first request through steady state.
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
cargo test -p q-http
```
```bash
cargo test -p q-bridge
```
```bash
cargo test -p velqu-runtime
```
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
beta-003-b: measure first request through steady state
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-003-B) — PASS (2026-09-03)

- Branch/PR: beta-003-b (squash-merged; see git log for final hash)
- Closes: #511

### Changed files
- `benchmarks/harness/ramp.ts` (new): first-request-through-steady-state
  harness. Per sample: fresh process -> TCP-accept poll -> sequential
  byte-validated requests from request #0 -> deterministic steady-state
  criterion (windows of 25; a transition is flat when the window median is
  within [0.8x, 1.25x] of the previous; onset = two consecutive flat
  transitions, with a >=50-request minimum steady tail) -> terminate.
  Wall-clock-independent; a series still decaying/regressing at the
  --max-requests cap is reported as "no onset" (never extrapolated).
  Pinned baseline deps auto-install from the committed lockfile when absent
  (same policy as run-w4.sh). Pure helpers (steadyOnsetIndex, phaseLabel,
  aggregateRamp, median/percentile) are exported and unit-tested.
- `benchmarks/harness/ramp.test.ts` (new): 10 deterministic tests — flat /
  decaying / steep-drop / ever-improving / ever-regressing / short series;
  phase labeling; cross-rep aggregation incl. no-onset reps; cold-start
  percentile conventions.
- `benchmarks/raw/ramp/` (new evidence): per-request phase-tagged JSONL
  (`ramp-1788448064944.jsonl`), `summary.json` (`velqu-ramp-v1`), generated
  `ramp-report.md`.
- `docs/reports/beta-003-b-first-request-steady-state.md` (new): method,
  results table, honest reading (incl. where Velqu loses), DRAFT public
  wording (explicitly not approved for publication).

### Measured highlights (4 candidates x C0/C2 x 3 reps, 0 errors, all onset)

| candidate | first p50 us | steady p50 us | first/steady | onset (req #) | RSS MB |
|---|---|---|---|---|---|
| velqu | 268-270 | 34-59 | 4.6-7.9x | 50-100 | ~9.8 |
| raw-rust | 238-248 | 45-61 | 4.1-5.3x | 150 | ~3.4 |
| raw-bun | 3258-3902 | 41-48 | 68-95x | 150-175 | ~26 |
| elysia2 | 10060-14811 | 35-37 | 287-400x | 75-200 | ~46 |

Honest framing: steady-state floors are equivalent across runtimes; the
differentiator is the warmup cliff. Velqu's advantage is the near-absence
of one (pre-compiled QPack bytecode), not a lower floor.

### Required evidence

- **Raw crossover data**: per-request JSONL rows retained (committed).
- **Generated report**: `ramp-report.md` (in evidence dir) +
  `docs/reports/beta-003-b-first-request-steady-state.md`.
- **Public wording draft**: included, marked DRAFT — gated on quiet-host
  reruns, BETA-003-C/D, owner review.

### Commands

- `bun ramp.ts` -> PASS (8 cells, 0 errors, steady onset found in all)
- `bun test benchmarks/harness/ramp.test.ts` -> 10 pass / 0 fail
- `cargo test -p q-http` -> 12 pass / 0 failed (2 suites)
- `cargo test -p q-bridge` -> 11 pass / 0 failed
- `cargo test -p velqu-runtime` -> 38 pass / 0 failed (3 suites)
- `bun test` -> 358 pass / 0 fail (58 files)
- `bun run typecheck` -> clean
- `./scripts/verify` -> ALL PASS (M0-M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)
  (isolated netns; standing port-3000 environment note, BETA-002-C record)

### Guardrail mapping

- **Crossover method is reproducible**: one command, deterministic criterion,
  pinned deps, committed raw rows.
- **Cold, warm, CPU, and I/O are not conflated**: cold (first request) and
  warming are separated from steady by the onset detector; I/O/CPU cells
  live in BETA-003-A's harness.
- **p50/p95/p99, CPU, RSS, errors are included**: summary carries percentile
  stats for first/steady phases, per-process RSS, and error counts (errors
  fail the run loudly).
- **Positioning follows evidence**: report narrates only measured trends,
  including where Velqu does not win; wording is DRAFT.

### Standing CI disclosure

CI `verify` workflows stall/fail with zero executed steps on PR creation
across all branches (infrastructure-side, tracked since ~#714); the local
`./scripts/verify` run above is the real gate evidence for this packet.
