---
task_id: M27-001-V
parent_task: M27-001
milestone: M27
priority: P0
mode: VERIFY
status: TODO
context_card: context/milestones/M27.md
commit_required: true
---

# M27-001-V — Verify Define capability ABI and lifecycle state machine

## Atomic goal

Prove every acceptance criterion for parent task M27-001 without broadening scope.

## Parent intent

Specify install, lazy init, invocation ownership, cancellation, drain, shutdown, versioning, and errors for native capabilities.

## Dependencies

- `M27-001-A` — `tasks/04_m27_capability_linker/M27-001-A-accept-adr.md`
- `M27-001-B` — `tasks/04_m27_capability_linker/M27-001-B-define-capabilityid-version-dependencies.md`
- `M27-001-C` — `tasks/04_m27_capability_linker/M27-001-C-define-native-operation-owner-deadline-state.md`
- `M27-001-D` — `tasks/04_m27_capability_linker/M27-001-D-define-lifecycle-phases-and-bounded-shutdown.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M27.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/capabilities-fetch.md`

### Source files

- `AGENTS.md`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `packages/compiler/src/emit.ts`
- `crates/q-router/src/lib.rs`
- `crates/q-pack/src/lib.rs`
- `conformance/routing/routing.conformance.test.ts`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-runtime/src/main.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- No capability can start work outside allowed phase.
- Every op is physically cancellable or explicitly non-cancellable.
- Version conflicts fail before ready.
- Shutdown reaches quiescence or fails closed.

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
cargo test -p q-capabilities
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

- Lifecycle state tests.
- Capability author guide draft.
- Threat review.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m27-001-v: verify define capability abi and lifecycle state machine
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
