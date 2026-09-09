---
task_id: BETA-003-A
parent_task: BETA-003
milestone: BETA
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-003-A — Run 0/1/5/10/25ms I/O, payload matrices, and CPU operation levels

## Atomic goal

Run 0/1/5/10/25ms I/O, payload matrices, and CPU operation levels.

## Parent intent

Show where cold start and native infrastructure beat or lose to JIT execution.

## Dependencies

- `BETA-001-Z` — `tasks/08_public_beta/BETA-001-Z-package-evidence-for-make-the-real-world-benchmark-harness-executable.md`
- `M28-GATE` — `gates/M28-GATE.md`
- `M3-GATE` — `gates/M3-GATE.md`

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
5. Implement exactly this deliverable: Run 0/1/5/10/25ms I/O, payload matrices, and CPU operation levels.
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
beta-003-a: run 0 1 5 10 25ms i o payload matrices and cpu operation lev
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-003-A) — PASS (2026-09-03)

- Branch/PR: beta-003-a (squash-merged; see git log for final hash)
- Closes: #510

### Changed files

Harness extensions:
- `benchmarks/real-world/workloads.json`: +9 cells — `W4_0ms` (0ms I/O: the
  overhead-floor cell), `PAYLOAD_1/10/20/50` (W3 route at bounded limits:
  serialization-size scaling), `CPU_0/100/1000/10000` (deterministic
  in-handler loop levels; 22 workloads total).
- `benchmarks/real-world/candidates/matched.ts` + `matched.cjs`: shared
  bounded `cpuWork(ops)` loop (identical JS in every candidate, so only the
  execution engine varies) + `validateOps` (regex, cap 100000) — placed in
  the side-effect-free contract module; `shared.ts`/`shared.cjs` re-export.
- All four candidates (`hono.ts`, `elysia.ts`, `bun-fetch.ts`, `fastify.js`):
  new `GET /api/bench/cpu?ops=N` -> 400 `{error:"invalid ops"}` |
  200 `{ops, checksum}` with the shared `cpuWork`.
- `benchmarks/real-world/contract-fixtures.ts`: +2 fixtures (`cpu.200.ops-1000`
  with checksum oracle = the same shared `cpuWork`, `cpu.400.invalid-ops`);
  matrix is now 20 fixtures x 4 candidates.
- `benchmarks/real-world/load.ts`: `--server-pid N` samples the candidate's
  `/proc/<pid>/status` VmRSS at end of run into `summary.serverRssKb`
  (supplementary: null-safe, never fabricated); `readVmRssKb` exported.
- `benchmarks/real-world/result-schema.ts`: optional `serverRssKb` (positive
  integer when present).
- `benchmarks/real-world/compare-w4.ts`: generalized with `--workloads`,
  `--concurrency`, `--out`, `--title` (M28-011-A invocation unchanged by
  defaults); server-RSS column; W4 tail guardrail now handles the 0ms cell
  (absolute 50ms p99 floor bound — a multiplicative bound on 0 is always 0)
  and checks non-W4 cells for zero errors/mismatches.
- `benchmarks/real-world/run-crossover.sh` (new): one command runs all 13
  cells x 4 candidates (upstream + fresh candidate per run + per-candidate
  RSS + raw-sample retention via `retain.ts` + three generated comparison
  reports).
- `benchmarks/real-world/workloads.test.ts`: +2 tests (payload and CPU cell
  contracts, bounded); W4 matrix updated to [0,1,5,10,25]; cpu path regex.
- `benchmarks/real-world/verify-contract.test.ts`: fixture coverage extended
  to the `cpu.` prefix.
- `benchmarks/real-world/README.md` + `SPEC.md`: crossover runner + fixture
  matrix documentation.
- `.gitignore`: crossover local-only files (uncompressed rows, logs) ignored;
  retained .gz + summaries + reports tracked.

Evidence:
- `benchmarks/raw/real-world/crossover/`: per-candidate `summary.json`
  (incl. sampled RSS), retained byte-reproducible `raw.jsonl.gz`,
  `RETENTION.md`, and generated `w4-latency.md`, `payload-matrix.md`,
  `cpu-matrix.md` (all 78 cells: 0 errors, 0 status mismatches; W4 tail
  guardrail PASS).
- `docs/reports/beta-003-a-crossover-matrices.md` (new): methodology,
  environment (Bun 1.4.0, Node v22.23.2, linux/x64, commit b04e72a),
  cold/warm/CPU/I/O separation, measured results, and the DRAFT public
  wording (explicitly marked not-approved).

### Measured highlights (single run, shared dev host; see report caveats)

- I/O >= 5ms: candidates indistinguishable (upstream-dominated), by design.
- 0ms floor cell: p50 ~30-40us (Bun candidates), ~40-60us (Node/Fastify).
- CPU scaling c=1, CPU_0 -> CPU_10000: throughput -55% (hono, elysia2 2.2x),
  -38% (bun-fetch, fastify 1.6x).
- Payload c=1, limit 1 -> 50: ~an order of magnitude throughput drop for
  every candidate (serialization-dominated).
- RSS sampled: Bun candidates 43-55 MB, Node/Fastify ~165 MB.

### Required evidence

- **Raw crossover data**: retained per-candidate archives + summaries
  (committed).
- **Generated report**: `docs/reports/beta-003-a-crossover-matrices.md` +
  the three generated comparison reports in the evidence dir.
- **Public wording draft**: included and marked DRAFT — requires quiet-host
  reruns, BETA-003-B/C/D, and owner review; no claims published.

### Commands

- `./run-crossover.sh 3 1,10` -> PASS (retained evidence, 0 errors/mismatches)
- `bun verify-contract.ts` -> PASS (20 fixtures x 4 candidates)
- `bun test benchmarks/real-world benchmarks/real-world/candidates` -> 57 pass / 0 fail
- `bun test` -> 348 pass / 0 fail (57 files)
- `bun run typecheck` -> clean
- `./scripts/verify` -> ALL PASS (M0-M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)
  (isolated netns; standing port-3000 environment note, BETA-002-C record)

### Guardrail mapping

- **Crossover method is reproducible**: one command (`run-crossover.sh`),
  pinned candidate deps, hash-pinned contract inputs, retained raw rows.
- **Cold, warm, CPU, and I/O are not conflated**: separate cells; CPU cells
  have zero I/O; cold-start is BETA-003-B, not here.
- **p50/p95/p99, CPU, RSS, errors are included**: all in summaries +
  comparison reports (RSS sampled per candidate process).
- **Positioning follows evidence**: report narrates only directly-visible
  trends; public wording is DRAFT and explicitly unpublished.

### Standing CI disclosure

CI `verify` workflows stall/fail with zero executed steps on PR creation
across all branches (infrastructure-side, tracked since ~#714); the local
`./scripts/verify` run above is the real gate evidence for this packet.
