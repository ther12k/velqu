---
task_id: M28-005-V
parent_task: M28-005
milestone: M28
priority: P0
mode: VERIFY
status: TODO
context_card: context/milestones/M28.md
commit_required: true
---

# M28-005-V — Verify Propagate AbortSignal and route deadlines

## Atomic goal

Prove every acceptance criterion for parent task M28-005 without broadening scope.

## Parent intent

Ensure request cancellation physically stops outbound work and keeps ownership correct.

## Dependencies

- `M28-005-A` — `tasks/05_m28_native_fetch/M28-005-A-combine-explicit-abort-route-deadline-disconnect-shutdown-and-quarantine.md`
- `M28-005-B` — `tasks/05_m28_native_fetch/M28-005-B-use-one-terminal-state-for-each-operation.md`
- `M28-005-C` — `tasks/05_m28_native_fetch/M28-005-C-cancel-dns-connect-body-streaming.md`
- `M28-005-D` — `tasks/05_m28_native_fetch/M28-005-D-map-failures-deterministically.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M28.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`

### Source files

- `AGENTS.md`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-http/src/lib.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`
- `crates/q-bridge/src/lib.rs`
- `crates/q-runtime/tests/runtime_conformance.rs`
- `crates/q-pack/src/lib.rs`
- `benchmarks/harness/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- No outbound task survives terminal invocation without defer ownership.
- Timeout counted once.
- Cancellation latency is bounded.
- Worker remains reusable.

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
cargo test -p q-bridge
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

## Required evidence for this microtask

- Race tests.
- Task accounting.
- Timeout/cancel conformance.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m28-005-v: verify propagate abortsignal and route deadlines
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
