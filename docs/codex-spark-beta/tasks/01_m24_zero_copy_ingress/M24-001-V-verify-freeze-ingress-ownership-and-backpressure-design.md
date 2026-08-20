---
task_id: M24-001-V
parent_task: M24-001
milestone: M24
priority: P0
mode: VERIFY
status: TODO
context_card: context/milestones/M24.md
commit_required: true
---

# M24-001-V — Verify Freeze ingress ownership and backpressure design

## Atomic goal

Prove every acceptance criterion for parent task M24-001 without broadening scope.

## Parent intent

Define ownership from Hyper ingress through routing, worker queue, slab lifetime, cancellation, and response completion.

## Dependencies

- `M24-001-A` — `tasks/01_m24_zero_copy_ingress/M24-001-A-accept-an-adr-with-ownership-diagrams-and-terminal-invariants.md`
- `M24-001-B` — `tasks/01_m24_zero_copy_ingress/M24-001-B-specify-body-ownership-queue-admission-disconnect-cancellation-and-request-slot.md`
- `M24-001-C` — `tasks/01_m24_zero_copy_ingress/M24-001-C-define-no-copy-and-bounded-copy-boundaries.md`
- `M24-001-D` — `tasks/01_m24_zero_copy_ingress/M24-001-D-define-overload-responses-and-metrics.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M24.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`

### Source files

- `AGENTS.md`
- `crates/q-http/src/lib.rs`
- `crates/q-bridge/src/lib.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-router/src/lib.rs`
- `crates/q-pack/src/lib.rs`
- `packages/compiler/src/emit.ts`
- `conformance/routing/routing.conformance.test.ts`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- No request data is borrowed beyond its owner lifetime.
- Queue/body limits are explicit.
- Cancellation has one owner.
- Design supports one and multiple workers.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
cargo test -p q-router
```
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

## Required evidence for this microtask

- ADR.
- State-machine tests plan.
- Threat/ownership review.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m24-001-v: verify freeze ingress ownership and backpressure design
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
