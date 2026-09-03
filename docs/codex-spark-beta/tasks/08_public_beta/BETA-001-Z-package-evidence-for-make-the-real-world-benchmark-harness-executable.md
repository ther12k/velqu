---
task_id: BETA-001-Z
parent_task: BETA-001
milestone: BETA
priority: P1
mode: EVIDENCE
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-001-Z — Package evidence for Make the real-world benchmark harness executable

## Atomic goal

Create source-backed evidence and handoff for parent task BETA-001; update status only if verification passed.

## Parent intent

Turn the current SPEC/schema/workloads scaffold into deterministic infrastructure.

## Dependencies

- `BETA-001-V` — `tasks/08_public_beta/BETA-001-V-verify-make-the-real-world-benchmark-harness-executable.md`

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
- `benchmarks/harness/`
- `benchmarks/manifest.json`
- `benchmarks/raw/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

## Parent acceptance guardrails

- One command prepares/runs/reports.
- Dataset resets deterministically.
- Candidate failure is retained.
- Protocol records environment and hashes.

## Targeted commands

```bash
cargo test -p q-engine-quickjs
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

- Harness source.
- Smoke results.
- Fairness audit.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
beta-001-z: package evidence for make the real world benchmark harness e
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

---

## Result (BETA-001-Z) — PASS (2026-09-01)

- Branch/PR: beta-001-z (squash-merged; see git log for final hash)
- Closes: #503
- Parent verification: BETA-001-V PASS (PR #1102); this packet packages the
  source-backed evidence across all child packets (A through D) and flips
  parent task BETA-001 to PASS.

### Evidence package

- **Implementation packets (squash-merged):**
  - BETA-001-A (PR #969 / earlier baseline): `compose.yaml` (pinned `postgres:17.5-alpine3.22`),
    `postgres/reset.sql` + `postgres/seed.sql` + `reset.sh` (deterministic SQL reset/seed),
    `upstream.ts` (W4 controlled upstream 0..1000ms), `result-schema.ts` (`velqu-realworld-summary-v1`),
    `load.ts` (fixed-duration concurrency ladder load generator), `report.ts`, and `run.sh`.
  - BETA-001-B: candidate/toolchain version pins (`versions.json`, `versions.test.ts` 7/7 pass).
  - BETA-001-C: fairness audit and reporting (`fairness.ts`, `fairness.test.ts` 9/9 pass).
  - BETA-001-D: raw-sample retention (`retain.ts`, `retain.test.ts` 5/5 pass, `raw.jsonl.gz`).
  - BETA-001-V (PR #1102): verification closure mapping all 4 acceptance guardrails.

### Required evidence

- **Harness source**: `benchmarks/real-world/` with 36/36 passing unit/conformance tests.
- **Smoke results**: `benchmarks/raw/real-world/smoke/{summary.json,report.md,raw.jsonl.gz,RETENTION.md}`
  demonstrating deterministic execution and raw sample preservation.
- **Fairness audit**: `fairness.ts` enforcing contract hash matching, protocol alignment,
  cell parity, and zero hidden errors.

### Parent guardrail proofs

1. **One command prepares/runs/reports** — `run.sh` orchestrates `prepare` -> `smoke` -> `audit` -> `retain`.
2. **Dataset resets deterministically** — `reset.sql` / `seed.sql` use pure modular arithmetic without random functions.
3. **Candidate failure is retained** — `load.ts` / `result-schema.ts` / `report.ts` record failure rows.
4. **Protocol records environment and hashes** — summaries mandate environment details and 5 SHA-256 config hashes.

### Gate results

- `bun test benchmarks/real-world` → **36 pass / 0 fail (6 files)**
- `cargo test -p q-engine-quickjs` → PASS
- `cargo test -p velqu-runtime` → PASS
- `bun test` → **327 pass / 0 fail (55 files)**
- `bun run typecheck`, fmt check, workspace clippy → clean
- `./scripts/verify` → **ALL PASS**
- `./scripts/validate-okf` → **ALL PASS**

### Ledger

- `docs/beta/04_TASK_LEDGER.md`: BETA-001 flipped TODO → **PASS**.
- STATUS.md and TASK_INDEX.md updated to PASS.

### Disclosures

- Evidence-only packet; no production runtime behavior changes.
- Standing: CI `verify` workflows fail with zero executed steps since ~#714
  (infrastructure-side); disclosed per PR. Local
  `./scripts/verify` is the gate evidence.
