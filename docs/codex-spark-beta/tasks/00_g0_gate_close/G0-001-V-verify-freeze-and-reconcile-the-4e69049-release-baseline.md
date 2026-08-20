---
task_id: G0-001-V
parent_task: G0-001
milestone: G0
priority: P0
mode: VERIFY
status: TODO
context_card: context/milestones/G0.md
commit_required: true
---

# G0-001-V — Verify Freeze and reconcile the 4e69049 release baseline

## Atomic goal

Prove every acceptance criterion for parent task G0-001 without broadening scope.

## Parent intent

Establish commit 4e6904951729ea14b48ca39a9564a950cc83e98e as the only working baseline and remove contradictory evidence state.

## Dependencies

- `G0-001-A` — `tasks/00_g0_gate_close/G0-001-A-verify-source-4e69049-zip-and-velqu-4e69049-bundle-against-source-commit-txt-and.md`
- `G0-001-B` — `tasks/00_g0_gate_close/G0-001-B-reconcile-review-index-json-and-evidence-index-json-so-their-commit-milestone-te.md`
- `G0-001-C` — `tasks/00_g0_gate_close/G0-001-C-update-docs-beta-00-current-baseline-md-docs-beta-04-task-ledger-md-and-related.md`
- `G0-001-D` — `tasks/00_g0_gate_close/G0-001-D-capture-compiler-rust-bun-quickjs-ng-rquickjs-os-cpu-load-generator-and-benchmar.md`
- `G0-001-E` — `tasks/00_g0_gate_close/G0-001-E-quarantine-stale-historical-release-metadata-under-an-explicitly-historical-dire.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/G0.md`
- `context/components/devex-beta.md`
- `context/components/evidence.md`

### Source files

- `AGENTS.md`
- `SOURCE-COMMIT.txt`
- `SHA256SUMS.txt`
- `REVIEW_INDEX.json`
- `EVIDENCE_INDEX.json`
- `TASKS.production.json`
- `docs/beta/00_CURRENT_BASELINE.md`
- `docs/beta/04_TASK_LEDGER.md`
- `scripts/release-packet`
- `scripts/validate-production-plan`
- `scripts/validate-okf`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- Git bundle HEAD, source tree, SOURCE-COMMIT, review index, and evidence index identify one commit.
- No current document claims the single-pass benchmark is the required repeated benchmark.
- G0 status is not PASS while any frozen gate remains open.
- Current release metadata contains no unlabeled stale checkpoint.

## Targeted commands

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

- Git bundle verification transcript.
- ZIP-to-commit tree comparison.
- Current environment manifest.
- Corrected REVIEW_INDEX and EVIDENCE_INDEX.
- Clean working-tree proof.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
g0-001-v: verify freeze and reconcile the 4e69049 release baseline
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
