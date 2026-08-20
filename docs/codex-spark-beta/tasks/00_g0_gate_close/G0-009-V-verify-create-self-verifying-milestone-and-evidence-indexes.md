---
task_id: G0-009-V
parent_task: G0-009
milestone: G0
priority: P1
mode: VERIFY
status: TODO
context_card: context/milestones/G0.md
commit_required: true
---

# G0-009-V — Verify Create self-verifying milestone and evidence indexes

## Atomic goal

Prove every acceptance criterion for parent task G0-009 without broadening scope.

## Parent intent

Make release, review, evidence, and task status self-verifying and commit-accurate.

## Dependencies

- `G0-009-A` — `tasks/00_g0_gate_close/G0-009-A-generate-review-index-and-evidence-index-only-after-the-source-commit-is-fixed-a.md`
- `G0-009-B` — `tasks/00_g0_gate_close/G0-009-B-replace-placeholder-pending-commit-references-in-every-pass-task-with-concrete-c.md`
- `G0-009-C` — `tasks/00_g0_gate_close/G0-009-C-extend-the-production-beta-ledger-validator-to-verify-evidence-paths-test-names.md`
- `G0-009-D` — `tasks/00_g0_gate_close/G0-009-D-update-the-beta-baseline-and-g0-task-ledger-so-all-sources-of-truth-agree.md`
- `G0-009-E` — `tasks/00_g0_gate_close/G0-009-E-generate-a-current-only-release-packet-whose-internal-sha256sum-c-passes.md`

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

- `sha256sum -c` equivalent passes for the release packet.
- Every PASS task references existing evidence.
- No stale previous bundle is presented as current.
- Git bundle, source ZIP, and source commit agree.

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

- Review index.
- Evidence index.
- Release packet validation report.
- [ ] Current numeric startup requires semantic function identity and accepts no count-only fallback.
- [ ] Serialized router and schema/function plans are integrity-bound and loaded without runtime semantic reconstruction.
- [ ] RouteId, PolicyId, HandlerId, and SchemaId are operational; names are diagnostic only.
- [ ] Public contract and execution graph hashes are separate and independently verified.
- [ ] Canonical warm/cold evidence meets the frozen protocol and reports match raw data.
- [ ] Release packet is self-verifying and task/evidence state is truthful.
- Warm C0–C3: c=1/10/50, five repetitions.
- Cold: 25/1,000/10,000 routes, fresh processes.
- Allocation and startup-stage profile.
- No regression threshold is silently relaxed.
- No M2.4 request-slab integration.
- No new capability APIs.
- No database implementation.
- No multi-worker changes.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
g0-009-v: verify create self verifying milestone and evidence indexes
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
