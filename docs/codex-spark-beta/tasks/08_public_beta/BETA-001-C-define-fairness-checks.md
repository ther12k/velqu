---
task_id: BETA-001-C
parent_task: BETA-001
milestone: BETA
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-001-C — Define fairness checks

## Atomic goal

Define fairness checks.

## Parent intent

Turn the current SPEC/schema/workloads scaffold into deterministic infrastructure.

## Dependencies

- `BETA-001-B` — `tasks/08_public_beta/BETA-001-B-pin-candidate-versions.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/BETA.md`
- `context/components/engine-scheduler.md`
- `context/components/schema-codecs.md`
- `context/components/devex-beta.md`

### Source files

- `AGENTS.md`
- `docs/beta/`
- `scripts/verify`
- `package.json`
- `.github/workflows/verify.yml`
- `crates/q-schema-runtime/src/lib.rs`
- `crates/q-schema-runtime/tests/fuzz_validator.rs`
- `crates/q-engine-quickjs/src/convert.rs`
- `packages/schema/src/index.ts`
- `conformance/schema/schema.conformance.test.ts`
- `crates/q-runtime/src/main.rs`
- `crates/q-runtime/src/serve.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Define fairness checks.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

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
cargo test -p q-schema-runtime
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

- Harness source.
- Smoke results.
- Fairness audit.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
beta-001-c: define fairness checks
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
## Completion record

- Status: **PASS**
- Deliverable: fairness checks defined and enforced by `benchmarks/real-world/fairness.ts` (`auditFairness` + `renderFairnessReport`): a run set is fair only when every candidate shares identical contract hashes (spec/workloads/schema/seed/versions), identical protocol shape (durationSec, concurrency levels), identical host class (os/arch), an identical workload x concurrency cell grid, and zero errors/status mismatches in any cell (SPEC's 0%-error bar; failures are retained and named, never hidden). `./run.sh audit` (RUNS=dirA,dirB,...) writes `fairness.md` with a PASS/FAIL verdict table; exit code 1 on any FAIL.
- Live check: audit run against the committed smoke summaries correctly FAILS with `failures.retained` naming the candidate and cell (smoke drives W1–W3 at the bare upstream by design) — the audit refuses to bless that as a fair comparison, exactly the guard BETA-001 requires before cross-candidate claims.
- Changed files: `benchmarks/real-world/{fairness.ts,fairness.test.ts,run.sh,README.md}` + `.gitignore` (generated `fairness.md` stays scratch).
- Tests and exact results: `bun test benchmarks/real-world` 31/31 (9 new fairness tests: identical-contracts pass, single-run-set fail, seed/versions pin drift, duration/concurrency drift, environment-class drift, cell-grid parity, retained-failure naming, deterministic report verdict); full `bun test` 120/120; `bun run typecheck` PASS; `cargo test -p q-engine-quickjs` PASS (98); `cargo test -p q-schema-runtime` PASS (67); `cargo test -p velqu-runtime` PASS (28/28).
- Fairness audit evidence: the audit itself, its tests, and the live failing verdict on the non-comparable smoke run.
- Remaining risk / deferred by design: candidate-vs-candidate audits execute once BETA-002 implementations exist; deep response-body equality beyond status verification belongs to BETA-002 parity tests.
- Next dependency-ready task: BETA-001-D (Keep raw samples) — #499.
- Working tree clean: yes after commit.
