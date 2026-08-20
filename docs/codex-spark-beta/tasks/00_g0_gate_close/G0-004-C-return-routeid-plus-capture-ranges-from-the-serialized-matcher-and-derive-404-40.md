---
task_id: G0-004-C
parent_task: G0-004
milestone: G0
priority: P0
mode: VERIFY_OR_FIX
status: TODO
context_card: context/milestones/G0.md
commit_required: true
---

# G0-004-C — Return RouteId plus capture ranges from the serialized matcher and derive 404/405/Allow without rebuilding routes

## Atomic goal

Return RouteId plus capture ranges from the serialized matcher and derive 404/405/Allow without rebuilding routes.

## Parent intent

Trust and load the compiler-emitted serialized automaton directly without runtime semantic reconstruction.

## Dependencies

- `G0-004-B` — `tasks/00_g0_gate_close/G0-004-B-keep-router-build-only-in-the-reference-matcher-compiler-tests-and-explicit-lega.md`

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

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Return RouteId plus capture ranges from the serialized matcher and derive 404/405/Allow without rebuilding routes.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

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

## Required evidence for this microtask

- Startup instrumentation.
- Router property-test corpus.
- 10,000-route load test.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
g0-004-c: return routeid plus capture ranges from the serialized match
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
