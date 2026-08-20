---
task_id: G0-001-Z
parent_task: G0-001
milestone: G0
priority: P0
mode: EVIDENCE
status: TODO
context_card: context/milestones/G0.md
commit_required: true
---

# G0-001-Z — Package evidence for Freeze and reconcile the 4e69049 release baseline

## Atomic goal

Create source-backed evidence and handoff for parent task G0-001; update status only if verification passed.

## Parent intent

Establish commit 4e6904951729ea14b48ca39a9564a950cc83e98e as the only working baseline and remove contradictory evidence state.

## Dependencies

- `G0-001-V` — `tasks/00_g0_gate_close/G0-001-V-verify-freeze-and-reconcile-the-4e69049-release-baseline.md`

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

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

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
```bash
./scripts/validate-production-plan
```
```bash
./scripts/validate-okf
```

## Required evidence for this microtask

- Git bundle verification transcript.
- ZIP-to-commit tree comparison.
- Current environment manifest.
- Corrected REVIEW_INDEX and EVIDENCE_INDEX.
- Clean working-tree proof.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
g0-001-z: package evidence for freeze and reconcile the 4e69049 releas
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
