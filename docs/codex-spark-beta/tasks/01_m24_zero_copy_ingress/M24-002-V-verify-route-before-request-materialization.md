---
task_id: M24-002-V
parent_task: M24-002
milestone: M24
priority: P0
mode: VERIFY
status: TODO
context_card: context/milestones/M24.md
commit_required: true
---

# M24-002-V — Verify Route before request materialization

## Atomic goal

Prove every acceptance criterion for parent task M24-002 without broadening scope.

## Parent intent

Avoid query/header/body work for routes that do not declare it.

## Dependencies

- `M24-002-A` — `tasks/01_m24_zero_copy_ingress/M24-002-A-keep-method-uri-headermap-and-body-stream-in-native-forms.md`
- `M24-002-B` — `tasks/01_m24_zero_copy_ingress/M24-002-B-match-routeid-using-method-path-before-creating-request-metadata.md`
- `M24-002-C` — `tasks/01_m24_zero_copy_ingress/M24-002-C-read-fieldneeds-from-routeplan.md`
- `M24-002-D` — `tasks/01_m24_zero_copy_ingress/M24-002-D-bypass-request-object-creation-for-policy-free-routes-that-need-no-request-field.md`

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
- `crates/q-runtime/tests/runtime_conformance.rs`
- `packages/treaty/src/index.ts`
- `packages/contract/src/index.ts`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- C0/C1 perform no query parse, header clone, cookie parse, or body collect.
- 404/405 does not materialize request bodies.
- Malformed oversized inputs fail within coarse ingress budgets.
- Routing behavior remains contract-equivalent.

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

- Admission counters.
- Negative body/header budget tests.
- Perf stage timings.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m24-002-v: verify route before request materialization
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
