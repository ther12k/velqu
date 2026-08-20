---
task_id: G0-004-V
parent_task: G0-004
milestone: G0
priority: P0
mode: VERIFY
status: TODO
context_card: context/milestones/G0.md
commit_required: true
---

# G0-004-V — Verify Load the serialized router directly

## Atomic goal

Prove every acceptance criterion for parent task G0-004 without broadening scope.

## Parent intent

Trust and load the compiler-emitted serialized automaton directly without runtime semantic reconstruction.

## Dependencies

- `G0-004-A` — `tasks/00_g0_gate_close/G0-004-A-change-the-current-numeric-startup-path-so-router-from-pack-consumes-the-verifie.md`
- `G0-004-B` — `tasks/00_g0_gate_close/G0-004-B-keep-router-build-only-in-the-reference-matcher-compiler-tests-and-explicit-lega.md`
- `G0-004-C` — `tasks/00_g0_gate_close/G0-004-C-return-routeid-plus-capture-ranges-from-the-serialized-matcher-and-derive-404-40.md`
- `G0-004-D` — `tasks/00_g0_gate_close/G0-004-D-add-a-genuine-generated-property-suite-comparing-serialized-matching-with-the-in.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/G0.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`

### Source files

- `AGENTS.md`
- `crates/q-pack/src/lib.rs`
- `crates/q-router/src/lib.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `packages/compiler/src/emit.ts`
- `conformance/routing/routing.conformance.test.ts`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- Current numeric startup performs zero route parsing/collision reconstruction.
- 404/405 and Allow semantics match the reference matcher.
- Route-specific parameter names are preserved.
- Every compiled route is reachable exactly as intended.

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

- Startup instrumentation.
- Router property-test corpus.
- 10,000-route load test.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
g0-004-v: verify load the serialized router directly
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
