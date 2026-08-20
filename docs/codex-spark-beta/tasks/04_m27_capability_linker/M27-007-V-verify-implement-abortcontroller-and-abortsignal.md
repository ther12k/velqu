---
task_id: M27-007-V
parent_task: M27-007
milestone: M27
priority: P0
mode: VERIFY
status: TODO
context_card: context/milestones/M27.md
commit_required: true
---

# M27-007-V — Verify Implement AbortController and AbortSignal

## Atomic goal

Prove every acceptance criterion for parent task M27-007 without broadening scope.

## Parent intent

Create one cancellation primitive shared by fetch and native capabilities.

## Dependencies

- `M27-007-A` — `tasks/04_m27_capability_linker/M27-007-A-define-signal-state-listeners-reason.md`
- `M27-007-B` — `tasks/04_m27_capability_linker/M27-007-B-bridge-route-deadline-and-explicit-cancellation.md`
- `M27-007-C` — `tasks/04_m27_capability_linker/M27-007-C-prevent-listener-leaks.md`
- `M27-007-D` — `tasks/04_m27_capability_linker/M27-007-D-make-cancellation-idempotent.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M27.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`

### Source files

- `AGENTS.md`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `packages/compiler/src/emit.ts`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`
- `crates/q-pack/src/lib.rs`
- `crates/q-http/src/lib.rs`
- `crates/q-runtime/src/serve.rs`
- `Cargo.toml`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- Abort propagates exactly once.
- Late listeners follow defined semantics.
- No cross-invocation ownership.
- Shutdown cancellation is bounded.

## Targeted commands

```bash
cargo test -p q-pack
```
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

- Conformance tests.
- Leak tests.
- Race tests.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m27-007-v: verify implement abortcontroller and abortsignal
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
