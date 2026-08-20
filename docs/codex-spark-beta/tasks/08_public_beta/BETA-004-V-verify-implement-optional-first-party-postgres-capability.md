---
task_id: BETA-004-V
parent_task: BETA-004
milestone: BETA
priority: P0
mode: VERIFY
status: TODO
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-004-V — Verify Implement optional first-party Postgres capability

## Atomic goal

Prove every acceptance criterion for parent task BETA-004 without broadening scope.

## Parent intent

Provide a real database story without enlarging core.

## Dependencies

- `BETA-004-A` — `tasks/08_public_beta/BETA-004-A-use-capability-abi.md`
- `BETA-004-B` — `tasks/08_public_beta/BETA-004-B-lazy-pool.md`
- `BETA-004-C` — `tasks/08_public_beta/BETA-004-C-parameterized-queries-transactions.md`
- `BETA-004-D` — `tasks/08_public_beta/BETA-004-D-deadline-cancellation-shutdown.md`
- `BETA-004-E` — `tasks/08_public_beta/BETA-004-E-pool-limits-and-observability.md`
- `BETA-004-F` — `tasks/08_public_beta/BETA-004-F-no-orm.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/BETA.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/capabilities-fetch.md`

### Source files

- `AGENTS.md`
- `docs/beta/`
- `scripts/verify`
- `package.json`
- `.github/workflows/verify.yml`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-pack/src/lib.rs`
- `benchmarks/real-world/postgres/`
- `benchmarks/real-world/SPEC.md`
- `packages/capability-postgres/ (create if absent)`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- App without Postgres pays zero dependency/init cost.
- Queries are parameterized.
- Timeout cancels/releases connection safely.
- Pool exhaustion is bounded.
- W1/W2/W3 workloads pass.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p q-capabilities
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

- Capability tests.
- Real-world results.
- Cold/RSS cost report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
beta-004-v: verify implement optional first party postgres capability
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
