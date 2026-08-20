---
task_id: M28-006-V
parent_task: M28-006
milestone: M28
priority: P0
mode: VERIFY
status: TODO
context_card: context/milestones/M28.md
commit_required: true
---

# M28-006-V — Verify Implement streaming and strict backpressure

## Atomic goal

Prove every acceptance criterion for parent task M28-006 without broadening scope.

## Parent intent

Support large bodies without unbounded buffering.

## Dependencies

- `M28-006-A` — `tasks/05_m28_native_fetch/M28-006-A-bound-read-write-buffers.md`
- `M28-006-B` — `tasks/05_m28_native_fetch/M28-006-B-propagate-downstream-backpressure.md`
- `M28-006-C` — `tasks/05_m28_native_fetch/M28-006-C-cancel-on-consumer-stop-disconnect.md`
- `M28-006-D` — `tasks/05_m28_native_fetch/M28-006-D-define-maximum-body-helper-sizes.md`

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
- `crates/q-bridge/src/lib.rs`
- `crates/q-runtime/tests/runtime_conformance.rs`
- `Cargo.toml`
- `scripts/package`
- `scripts/release-packet`
- `packages/cli/package.json`
- `package.json`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- Large response does not allocate full body unless requested.
- Slow upstream/downstream remains bounded.
- Cancellation releases buffers/connections.
- Streaming errors are typed.

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

- Streaming load tests.
- Slow consumer tests.
- Memory profile.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m28-006-v: verify implement streaming and strict backpressure
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
