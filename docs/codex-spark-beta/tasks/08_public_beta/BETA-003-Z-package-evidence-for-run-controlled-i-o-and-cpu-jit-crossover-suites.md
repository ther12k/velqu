---
task_id: BETA-003-Z
parent_task: BETA-003
milestone: BETA
priority: P1
mode: EVIDENCE
status: TODO
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
