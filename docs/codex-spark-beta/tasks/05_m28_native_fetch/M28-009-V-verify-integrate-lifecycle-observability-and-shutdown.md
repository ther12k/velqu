---
task_id: M28-009-V
parent_task: M28-009
milestone: M28
priority: P1
mode: VERIFY
status: TODO
context_card: context/milestones/M28.md
commit_required: true
---

# M28-009-V — Verify Integrate lifecycle, observability, and shutdown

## Atomic goal

Prove every acceptance criterion for parent task M28-009 without broadening scope.

## Parent intent

Make fetch operationally diagnosable without hot-path logging cost.

## Dependencies

- `M28-009-A` — `tasks/05_m28_native_fetch/M28-009-A-expose-pool-wait-dns-connect-tls-ttfb-body-errors-cancellations.md`
- `M28-009-B` — `tasks/05_m28_native_fetch/M28-009-B-sample-aggregate-metrics.md`
- `M28-009-C` — `tasks/05_m28_native_fetch/M28-009-C-drain-pool-on-shutdown.md`
- `M28-009-D` — `tasks/05_m28_native_fetch/M28-009-D-quarantine-rejects-new-work.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M28.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`
- `context/components/capabilities-fetch.md`

### Source files

- `AGENTS.md`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-http/src/lib.rs`
- `Cargo.toml`
- `crates/q-engine/src/lib.rs`
- `docs/reports/`
- `docs/beta/workstreams/OBSERVABILITY_OPERATIONS.md`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- Metrics are bounded and redacted.
- Shutdown reaches quiescence.
- No task/connection leak after errors.
- Disabled instrumentation overhead is measured.

## Targeted commands

```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p q-http
```
```bash
cargo test -p q-capabilities
```
```bash
cargo test -p velqu-runtime
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
- Shutdown tests.
- Overhead report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m28-009-v: verify integrate lifecycle observability and shutdown
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
