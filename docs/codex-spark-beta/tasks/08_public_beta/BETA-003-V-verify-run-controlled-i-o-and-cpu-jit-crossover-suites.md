---
task_id: BETA-003-V
parent_task: BETA-003
milestone: BETA
priority: P1
mode: VERIFY
status: TODO
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-003-V — Verify Run controlled I/O and CPU/JIT crossover suites

## Atomic goal

Prove every acceptance criterion for parent task BETA-003 without broadening scope.

## Parent intent

Show where cold start and native infrastructure beat or lose to JIT execution.

## Dependencies

- `BETA-003-A` — `tasks/08_public_beta/BETA-003-A-run-0-1-5-10-25ms-i-o-payload-matrices-and-cpu-operation-levels.md`
- `BETA-003-B` — `tasks/08_public_beta/BETA-003-B-measure-first-request-through-steady-state.md`
- `BETA-003-C` — `tasks/08_public_beta/BETA-003-C-calculate-cumulative-crossover-request-counts.md`
- `BETA-003-D` — `tasks/08_public_beta/BETA-003-D-report-losses-honestly.md`

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

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

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

## Required evidence for this microtask

- Raw crossover data.
- Generated report.
- Public wording draft.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
beta-003-v: verify run controlled i o and cpu jit crossover suites
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
