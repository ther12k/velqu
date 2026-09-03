---
task_id: BETA-001-V
parent_task: BETA-001
milestone: BETA
priority: P1
mode: VERIFY
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-001-V — Verify Make the real-world benchmark harness executable

## Atomic goal

Prove every acceptance criterion for parent task BETA-001 without broadening scope.

## Parent intent

Turn the current SPEC/schema/workloads scaffold into deterministic infrastructure.

## Dependencies

- `BETA-001-A` — `tasks/08_public_beta/BETA-001-A-add-postgres-compose-seed-reset-controlled-upstream-result-schema-load-generator.md`
- `BETA-001-B` — `tasks/08_public_beta/BETA-001-B-pin-candidate-versions.md`
- `BETA-001-C` — `tasks/08_public_beta/BETA-001-C-define-fairness-checks.md`
- `BETA-001-D` — `tasks/08_public_beta/BETA-001-D-keep-raw-samples.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/BETA.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/schema-codecs.md`

### Source files

- `AGENTS.md`
- `docs/beta/`
- `scripts/verify`
- `package.json`
- `.github/workflows/verify.yml`
- `crates/q-pack/src/lib.rs`
- `crates/q-pack/tests/fuzz_pack.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `docs/specs/pack-format-v1.md`
- `crates/q-schema-runtime/src/lib.rs`
- `crates/q-schema-runtime/tests/fuzz_validator.rs`
- `crates/q-engine-quickjs/src/convert.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- One command prepares/runs/reports.
- Dataset resets deterministically.
- Candidate failure is retained.
- Protocol records environment and hashes.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p q-schema-runtime
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

- Harness source.
- Smoke results.
- Fairness audit.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
beta-001-v: verify make the real world benchmark harness executable
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

---

## Result (BETA-001-V) — PASS (2026-09-01)

- Branch/PR: beta-001-v (squash-merged; see git log for final hash)
- Closes: #500

### Acceptance-criterion mapping

1. **One command prepares/runs/reports** — `benchmarks/real-world/run.sh`
   implements the unified execution script (`prepare` -> `smoke` -> `audit` -> `retain`)
   orchestrating compose health, database reset/seed, load generation, result validation,
   and report output.
2. **Dataset resets deterministically** — `benchmarks/real-world/postgres/reset.sql`
   and `seed.sql` generate predictable fixture rows (1,000 users, 500 products, 10,000 reviews)
   using pure modular arithmetic and `generate_series` with zero non-deterministic random calls.
3. **Candidate failure is retained** — `load.ts`, `result-schema.ts`, and `report.ts`
   record and preserve raw failure rows and errors; tested by `fairness.test.ts`
   ("retained failures fail the audit and name the candidate and cell") and `report.test.ts`
   ("failures are retained in a dedicated section, not dropped").
4. **Protocol records environment and hashes** — `load.ts` and `result-schema.ts`
   mandate runtime environment (bun, node, OS, arch, commit) and SHA-256 config hashes
   (`spec`, `workloads`, `schema`, `seed`, `versions`).

### Evidence

- `bun test benchmarks/real-world` → **36 pass / 0 fail (6 files)**
- `cargo test -p q-pack` → PASS (100 passed)
- `cargo test -p q-engine-quickjs` → PASS (113 passed)
- `cargo test -p q-schema-runtime` → PASS (58 passed)
- `bun test` → **327 pass / 0 fail (55 files)**
- `bun run typecheck`, fmt check, workspace clippy → clean
- `./scripts/verify` → **ALL PASS**

### Changed files

- `docs/codex-spark-beta/tasks/08_public_beta/BETA-001-V-verify-make-the-real-world-benchmark-harness-executable.md`

### Disclosures

- Verification-only packet; no production runtime behavior changes.
- Standing: CI `verify` workflows fail with zero executed steps on every PR
  since ~#714 (infrastructure-side); disclosed per PR. Local
  `./scripts/verify` is the gate evidence.
