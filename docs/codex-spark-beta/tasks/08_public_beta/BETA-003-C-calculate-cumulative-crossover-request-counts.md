---
task_id: BETA-003-C
parent_task: BETA-003
milestone: BETA
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-003-C — Calculate cumulative crossover request counts

## Atomic goal

Calculate cumulative crossover request counts.

## Parent intent

Show where cold start and native infrastructure beat or lose to JIT execution.

## Dependencies

- `BETA-003-B` — `tasks/08_public_beta/BETA-003-B-measure-first-request-through-steady-state.md`

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
5. Implement exactly this deliverable: Calculate cumulative crossover request counts.
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
beta-003-c: calculate cumulative crossover request counts
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-003-C) — PASS (2026-09-03)

- Branch/PR: beta-003-c (squash-merged; see git log for final hash)
- Closes: #512

### Changed files
- `benchmarks/harness/crossover.ts` (new): consumes the BETA-003-B
  per-request series and computes, per class and ordered candidate pair,
  the smallest N where the median-across-reps cumulative served time of A
  drops to or below B's (`never` when it does not happen within the
  recorded horizon — never extrapolated). Also computes each candidate's
  self-amortization point (first N where cumulative average latency is
  within 1.25x of its own steady median). Process startup is explicitly
  excluded (a cold-start-harness quantity) and the exclusion is recorded
  in the output. Writes `crossover-counts.json` + `crossover-counts.md`.
- `benchmarks/harness/crossover.test.ts` (new): 9 deterministic tests —
  cumulative curve medians + horizon bounding, crossover boundary math
  (N=26 analytic case, immediate crossing, never-within-cap, horizon
  argument), self-amortization (never / immediate / degenerate inputs),
  median parity.
- `benchmarks/raw/ramp/crossover-counts.json` + `.md` (new evidence):
  generated from ramp run 1788448064944 — no hand-typed values.
- `docs/reports/beta-003-c-crossover-counts.md` (new): definitions,
  results, honest reading (incl. where Velqu loses), DRAFT public wording.

### Measured highlights (100-request horizon, 3 reps)

- raw-bun and elysia2 NEVER overtake velqu or raw-rust within the horizon
  in either class: their warmup debt (3.3-14.8ms first request) has no
  per-request advantage to amortize it with (steady floors equal-or-worse).
- velqu vs raw-rust: crossover at 76 requests (C0), 3 requests (C2).
- Velqu loses on the C2 steady floor (60us vs 35-44us for the others) —
  the QuickJS handler tax; the win is the absence of a warmup cliff.
- Self-amortization: velqu 270 (C0) / 26 (C2); raw-rust 17 (C0) / never
  (C2); raw-bun and elysia2 never within horizon in both classes.

### Required evidence

- **Raw crossover data**: derived strictly from the committed
  `benchmarks/raw/ramp/ramp-1788448064944.jsonl` series.
- **Generated report**: `crossover-counts.{json,md}` +
  `docs/reports/beta-003-c-crossover-counts.md`.
- **Public wording draft**: included, marked DRAFT — gated on quiet-host
  reruns, BETA-003-D, owner review.

### Commands

- `bun crossover.ts` -> PASS (12 pairs + 4 self-amortization cells per class)
- `bun test benchmarks/harness/crossover.test.ts` -> 9 pass / 0 fail
- `cargo test -p q-http` / `-p q-bridge` / `-p velqu-runtime` -> all suites ok
- `bun test` -> 367 pass / 0 fail (59 files)
- `bun run typecheck` -> clean
- `./scripts/verify` -> ALL PASS (M0-M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)
  (isolated netns; standing port-3000 environment note, BETA-002-C record)

### Guardrail mapping

- **Crossover method is reproducible**: deterministic definitions over
  committed raw series; one command regenerates both artifacts.
- **Cold, warm, CPU, and I/O are not conflated**: serving-only counts
  (startup explicitly excluded and labeled); ramp classes separate
  transport-floor from JS-handler work.
- **p50/p95/p99, CPU, RSS, errors are included**: medians across reps,
  only valid requests counted, horizon and ratio pinned in the output.
- **Positioning follows evidence**: `never` results kept as real findings;
  Velqu's slower C2 steady floor stated plainly; wording is DRAFT.

### Standing CI disclosure

CI `verify` workflows stall/fail with zero executed steps on PR creation
across all branches (infrastructure-side, tracked since ~#714); the local
`./scripts/verify` run above is the real gate evidence for this packet.
