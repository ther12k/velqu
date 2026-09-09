---
task_id: BETA-003-Z
parent_task: BETA-003
milestone: BETA
priority: P1
mode: EVIDENCE
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-003-Z — Package evidence for Run controlled I/O and CPU/JIT crossover suites

## Atomic goal

Create source-backed evidence and handoff for parent task BETA-003; update status only if verification passed.

## Parent intent

Show where cold start and native infrastructure beat or lose to JIT execution.

## Dependencies

- `BETA-003-V` — `tasks/08_public_beta/BETA-003-V-verify-run-controlled-i-o-and-cpu-jit-crossover-suites.md`

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

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

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
```bash
./scripts/validate-okf
```

## Required evidence for this microtask

- Raw crossover data.
- Generated report.
- Public wording draft.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
beta-003-z: package evidence for run controlled i o and cpu jit crossove
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-003-Z) — PASS (2026-09-03)

- Branch/PR: beta-003-z (squash-merged; see git log for final hash)
- Closes: #515
- Parent verification: BETA-003-V PASS (PR #1114); this packet packages the
  source-backed evidence across all child packets (A through D + V) and
  flips parent task BETA-003 to PASS in `docs/beta/04_TASK_LEDGER.md`.

### Evidence package

- **Implementation packets (squash-merged):**
  - BETA-003-A (PR #1110): controlled crossover cells — 0/1/5/10/25ms I/O,
    payload matrix (PAYLOAD_1/10/20/50), CPU operation levels (CPU_0/100/
    1000/10000 with the shared bounded `cpuWork`), RSS sampling
    (`load.ts --server-pid`), `run-crossover.sh` one-command runner,
    generalized comparison reports; caught its own bun-fetch insertion bug
    via the contract gate before any timing.
  - BETA-003-B (PR #1111): `ramp.ts` — first request through steady state
    with a deterministic flatness-window onset criterion (never
    extrapolated); 25 deterministic unit tests across the three harnesses.
  - BETA-003-C (PR #1112): `crossover.ts` — cumulative crossover request
    counts with honest `never` results; self-amortization points.
  - BETA-003-D (PR #1113): `losses.ts` — mechanical honest-loss ledger
    (18 rows, including Velqu's 1.59x C2 steady-floor loss).
  - BETA-003-V (PR #1114): verification closure; fresh re-runs reproduce;
    run-to-run spread recorded.

### Required evidence (regenerated fresh on this branch, self-consistent)

- `benchmarks/raw/ramp/`: `ramp-*.jsonl` (phase-tagged per-request rows),
  `summary.json` (`velqu-ramp-v1`), generated `ramp-report.md`,
  `crossover-counts.{json,md}`, `losses.{json,md}` — all from one fresh
  run (8/8 cells 0 errors, onset in all).
- `benchmarks/raw/real-world/crossover/`: per-candidate summaries + RSS,
  retained `raw.jsonl.gz` + RETENTION manifests, three generated comparison
  reports — 78 cells, 0 errors / 0 mismatches (`./run-crossover.sh 2 1,10`).
- Reports: `docs/reports/beta-003-{a,b,c,d}-*.md` with DRAFT public wording
  (explicitly not approved; loss-ledger framing).

### Parent guardrail proofs

1. **Crossover method is reproducible** — one command per suite, fixed
   rules, hash-pinned inputs, committed raw rows; fresh re-runs on this
   branch reproduce.
2. **Cold, warm, CPU, and I/O are not conflated** — phase-tagged ramp
   rows; distinct cell kinds; startup excluded from crossover counts and
   labeled.
3. **p50/p95/p99, CPU, RSS, errors are included** — in summaries,
   comparison reports, and the loss ledger.
4. **Positioning follows evidence** — losses mechanically extracted; gaps
   declared; all public wording DRAFT and unpublished.

### Gate results (fresh on this branch)

- `bun ramp.ts` + `bun crossover.ts` + `bun losses.ts` -> regenerated, PASS
- `./run-crossover.sh 2 1,10` -> PASS (78 cells, 0 errors/0 mismatches)
- `cargo test -p q-engine-quickjs` -> 113 pass; `cargo test -p velqu-runtime` -> all suites ok
- `cargo fmt --all --check` -> clean; clippy `-D warnings` -> clean
- `bun test` -> 373 pass / 0 fail (60 files); `bun run typecheck` -> clean
- `./scripts/verify` -> ALL PASS (M0-M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)
- `./scripts/validate-okf` -> PASS
  (verify run inside an isolated netns; standing port-3000 environment
  note, BETA-002-C record. No test weakened.)

### Ledger

- `docs/beta/04_TASK_LEDGER.md`: BETA-003 flipped TODO -> **PASS**.
- STATUS.md and TASK_INDEX.md updated to PASS (BETA-003-Z row).
