---
task_id: BETA-002-Z
parent_task: BETA-002
milestone: BETA
priority: P1
mode: EVIDENCE
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-002-Z — Package evidence for Implement matched competitor candidates

## Atomic goal

Create source-backed evidence and handoff for parent task BETA-002; update status only if verification passed.

## Parent intent

Provide Raw Rust, Elysia 2, Hono/Bun, and Fastify/Node implementations of identical contracts.

## Dependencies

- `BETA-002-V` — `tasks/08_public_beta/BETA-002-V-verify-implement-matched-competitor-candidates.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/BETA.md`
- `context/components/engine-scheduler.md`
- `context/components/devex-beta.md`
- `context/components/evidence.md`

### Source files

- `AGENTS.md`
- `docs/beta/`
- `scripts/verify`
- `package.json`
- `.github/workflows/verify.yml`
- `crates/q-runtime/src/main.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-engine-quickjs/src/lib.rs`
- `crates/q-engine-quickjs/src/worker.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

## Parent acceptance guardrails

- Candidates are semantically equivalent.
- No framework receives hidden advantages.
- All outputs pass contract fixtures.
- Version/hash metadata is captured.

## Targeted commands

```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p velqu-runtime
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

- Candidate source.
- Parity tests.
- Fairness report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
beta-002-z: package evidence for implement matched competitor candidates
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-002-Z) — PASS (2026-09-03)

- Branch/PR: beta-002-z (squash-merged; see git log for final hash)
- Closes: #509
- Parent verification: BETA-002-V PASS (PR #1108); this packet packages the
  source-backed evidence across all child packets (A through D + V) and
  flips parent task BETA-002 to PASS in `docs/beta/04_TASK_LEDGER.md`.

### Evidence package

- **Implementation packets (squash-merged):**
  - BETA-002-A (PR #1104): shared matched contract — `matched.ts`/`matched.cjs`
    (identical SQL, pool 20/5s/30s, JWT HS256 + benchmark token + 401 matrix,
    5s/100ms timeouts, logging off, compression off, loopback keep-alive
    single worker) + all four candidates extended to W1..W4 +
    `parity.test.ts` (7 tests).
  - BETA-002-B (PR #1105): version pins locked — `versions.json` hono pin
    aligned to 4.13.5; `versions.test.ts` 9/9 (registry = package.json =
    frozen bun.lock, exact).
  - BETA-002-C (PR #1106): contract-response verification gate —
    `contract-fixtures.ts` (18-fixture matrix, store-derived expectations),
    `verify-contract.ts` (boots every candidate, per-fixture PASS/FAIL),
    `run.sh contracts` phase, `verify-contract.test.ts` (8 tests); caught
    and fixed 2 real drifts (Elysia default 404 body, bun-fetch extra
    `path` field).
  - BETA-002-D (PR #1107): `DIFFERENCES.md` (unavoidable differences,
    declared uncompensated) — sha256-pinned per run
    (`configHashes.differences`), required by `result-schema.ts`, compared
    by `fairness.ts` (+2 drift tests).
  - BETA-002-V (PR #1108): verification closure mapping all 4 acceptance
    guardrails with fresh evidence.

### Required evidence

- **Candidate source**: `benchmarks/real-world/candidates/{matched.ts,
  matched.cjs,bun-fetch.ts,hono.ts,elysia.ts,fastify.js}` (+ `.cjs` twin),
  pins in `versions.json` + frozen `bun.lock`.
- **Parity tests**: `parity.test.ts` 7/7, `versions.test.ts` 9/9,
  `verify-contract.test.ts` 8/8, fairness/result-schema suites — combined
  `bun test benchmarks/real-world benchmarks/real-world/candidates` ->
  55 pass / 0 fail (8 files), re-run fresh on this branch.
- **Fairness report**: live contract verification re-run on this branch ->
  PASS (4 candidates x 18 fixtures, retained at
  `/tmp/beta-002-z-contract-verification.md` during the run); fairness
  audit additionally enforces hash parity incl. the differences document.

### Parent guardrail proofs

1. **Candidates are semantically equivalent** — one shared contract module +
   per-fixture live response identity on every candidate.
2. **No framework receives hidden advantages** — identical posture pinned in
   `MATCHED_CONFIG` and asserted; exact-pin versions; declared differences
   hash-pinned and uncompensated.
3. **All outputs pass contract fixtures** — 18/18 fixtures x 4 candidates
   PASS (fresh run on this branch).
4. **Version/hash metadata is captured** — `configHashes` (spec/workloads/
   schema/seed/versions/differences) validated hex64 and cross-compared.

### Gate results (fresh on this branch)

- `bun test benchmarks/real-world benchmarks/real-world/candidates` -> 55 pass / 0 fail (8 files)
- `bun verify-contract.ts` -> PASS
- `cargo test -p q-engine-quickjs` -> 113 passed / 0 failed
- `cargo test -p velqu-runtime` -> 4 passed / 0 failed (2 suites)
- `cargo fmt --all --check` -> clean; `cargo clippy --workspace --all-targets -- -D warnings` -> clean
- `bun test` -> 346 pass / 0 fail (57 files); `bun run typecheck` -> clean
- `./scripts/verify` -> ALL PASS (M0-M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)
- `./scripts/validate-okf` -> PASS
  (verify run inside an isolated netns; standing environment note: unrelated
  user process on 127.0.0.1:3000 trips the M4A-010-A scaffold live-Treaty
  skip predicate on the shared host. No test weakened.)

### Ledger

- `docs/beta/04_TASK_LEDGER.md`: BETA-002 flipped TODO -> **PASS**.
- STATUS.md and TASK_INDEX.md updated to PASS (BETA-002-Z row).
