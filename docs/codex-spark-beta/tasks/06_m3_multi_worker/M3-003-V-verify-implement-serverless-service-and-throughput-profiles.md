---
task_id: M3-003-V
parent_task: M3-003
milestone: M3
priority: P1
mode: VERIFY
status: TODO
context_card: context/milestones/M3.md
commit_required: true
---

# M3-003-V — Verify Implement serverless, service, and throughput profiles

## Atomic goal

Prove every acceptance criterion for parent task M3-003 without broadening scope.

## Parent intent

Make cold start versus immediate throughput an explicit deployment choice.

## Dependencies

- `M3-003-A` — `tasks/06_m3_multi_worker/M3-003-A-serverless-starts-one-worker-only.md`
- `M3-003-B` — `tasks/06_m3_multi_worker/M3-003-B-service-marks-ready-after-worker-0-and-adds-workers-adaptively.md`
- `M3-003-C` — `tasks/06_m3_multi_worker/M3-003-C-throughput-initializes-configured-workers-before-ready.md`
- `M3-003-D` — `tasks/06_m3_multi_worker/M3-003-D-expose-profile-in-inspect-config.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M3.md`
- `context/components/engine-scheduler.md`
- `context/components/multiworker.md`
- `context/components/evidence.md`

### Source files

- `AGENTS.md`
- `crates/q-runtime/src/main.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-engine-quickjs/src/lib.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`
- `benchmarks/harness/`
- `benchmarks/manifest.json`
- `benchmarks/raw/`
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

- Serverless cold start remains within approved budget.
- Profiles have deterministic readiness.
- No hidden worker creation.
- Profile-specific RSS is reported.

## Targeted commands

```bash
cargo test -p q-engine-quickjs
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

- Profile conformance.
- Cold/RSS report.
- Configuration docs.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m3-003-v: verify implement serverless service and throughput profiles
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
