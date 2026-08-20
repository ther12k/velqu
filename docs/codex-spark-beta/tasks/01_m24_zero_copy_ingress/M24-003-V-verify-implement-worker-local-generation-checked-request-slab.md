---
task_id: M24-003-V
parent_task: M24-003
milestone: M24
priority: P0
mode: VERIFY
status: TODO
context_card: context/milestones/M24.md
commit_required: true
---

# M24-003-V — Verify Implement worker-local generation-checked request slab

## Atomic goal

Prove every acceptance criterion for parent task M24-003 without broadening scope.

## Parent intent

Eliminate the global request-store mutex and keep lazy request access worker-owned.

## Dependencies

- `M24-003-A` — `tasks/01_m24_zero_copy_ingress/M24-003-A-move-request-slots-into-each-quickjs-worker.md`
- `M24-003-B` — `tasks/01_m24_zero_copy_ingress/M24-003-B-use-slot-plus-generation-handles.md`
- `M24-003-C` — `tasks/01_m24_zero_copy_ingress/M24-003-C-invalidate-at-settlement-timeout-cancellation-quarantine-and-shutdown.md`
- `M24-003-D` — `tasks/01_m24_zero_copy_ingress/M24-003-D-reject-stale-or-cross-worker-handles-deterministically.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M24.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`

### Source files

- `AGENTS.md`
- `crates/q-http/src/lib.rs`
- `crates/q-bridge/src/lib.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`
- `crates/q-runtime/tests/runtime_conformance.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- No process-wide request-store mutex on normal access.
- Stale handles always fail.
- No request slot leaks after terminal paths.
- No JS value or request slot crosses worker ownership.

## Targeted commands

```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p q-http
```
```bash
cargo test -p q-bridge
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

- Race tests.
- Slot accounting metrics.
- Fuzzed stale-handle operations.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m24-003-v: verify implement worker local generation checked request sla
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
