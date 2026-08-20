---
task_id: M24-007-V
parent_task: M24-007
milestone: M24
priority: P0
mode: VERIFY
status: TODO
context_card: context/milestones/M24.md
commit_required: true
---

# M24-007-V — Verify Implement bounded read-once body admission

## Atomic goal

Prove every acceptance criterion for parent task M24-007 without broadening scope.

## Parent intent

Collect or stream request bodies only when declared and under route/global limits.

## Dependencies

- `M24-007-A` — `tasks/01_m24_zero_copy_ingress/M24-007-A-drive-body-behavior-from-routeplan-not-http-method.md`
- `M24-007-B` — `tasks/01_m24_zero_copy_ingress/M24-007-B-use-bytes-and-avoid-bytes-to-vec-copies.md`
- `M24-007-C` — `tasks/01_m24_zero_copy_ingress/M24-007-C-enforce-content-length-and-streaming-limits.md`
- `M24-007-D` — `tasks/01_m24_zero_copy_ingress/M24-007-D-cache-one-decoded-representation-and-reject-incompatible-second-reads.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M24.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`
- `context/components/capabilities-fetch.md`

### Source files

- `AGENTS.md`
- `crates/q-http/src/lib.rs`
- `crates/q-bridge/src/lib.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-runtime/tests/runtime_conformance.rs`
- `packages/treaty/src/index.ts`
- `packages/contract/src/index.ts`
- `packages/testing/src/index.ts`
- `conformance/treaty/treaty.conformance.test.ts`
- `packages/compiler/src/emit.ts`
- `crates/q-capabilities/src/lib.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- POST with no body contract does not collect body.
- DELETE/body routes work when declared.
- Oversize/slow bodies cancel cleanly.
- Client disconnect releases body work.

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

- Body-limit tests.
- Slowloris/partial-body tests.
- Cancellation metrics.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m24-007-v: verify implement bounded read once body admission
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
