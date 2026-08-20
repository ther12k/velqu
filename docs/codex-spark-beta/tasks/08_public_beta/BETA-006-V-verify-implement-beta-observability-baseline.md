---
task_id: BETA-006-V
parent_task: BETA-006
milestone: BETA
priority: P0
mode: VERIFY
status: TODO
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-006-V — Verify Implement beta observability baseline

## Atomic goal

Prove every acceptance criterion for parent task BETA-006 without broadening scope.

## Parent intent

Expose bounded metrics and structured logs sufficient to operate beta services.

## Dependencies

- `BETA-006-A` — `tasks/08_public_beta/BETA-006-A-request-route-status-duration.md`
- `BETA-006-B` — `tasks/08_public_beta/BETA-006-B-worker-queues-quarantine-replacements.md`
- `BETA-006-C` — `tasks/08_public_beta/BETA-006-C-fetch-and-db-pools.md`
- `BETA-006-D` — `tasks/08_public_beta/BETA-006-D-memory-tasks-slots.md`
- `BETA-006-E` — `tasks/08_public_beta/BETA-006-E-optional-trace-integration-or-trace-ids.md`
- `BETA-006-F` — `tasks/08_public_beta/BETA-006-F-redaction.md`

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
- `packages/cli/src/index.ts`
- `crates/q-runtime/src/source_map.rs`
- `examples/proof/`
- `README.md`
- `crates/q-engine/src/lib.rs`
- `crates/q-runtime/src/serve.rs`
- `docs/reports/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- Disabled overhead measured.
- Enabled overhead budgeted.
- Cardinality is bounded.
- No secrets/PII by default.
- Dashboards/examples exist.

## Targeted commands

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

## Required evidence for this microtask

- Metrics schema.
- Overhead benchmark.
- Redaction audit.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
beta-006-v: verify implement beta observability baseline
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
