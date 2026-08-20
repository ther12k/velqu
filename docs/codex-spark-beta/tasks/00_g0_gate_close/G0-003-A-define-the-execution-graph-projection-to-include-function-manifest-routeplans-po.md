---
task_id: G0-003-A
parent_task: G0-003
milestone: G0
priority: P0
mode: VERIFY_OR_FIX
status: TODO
context_card: context/milestones/G0.md
commit_required: true
---

# G0-003-A — Define the execution graph projection to include function manifest, RoutePlans, policy bindings, schema manifest, capability bindings, and every serialized router node/terminal

## Atomic goal

Define the execution graph projection to include function manifest, RoutePlans, policy bindings, schema manifest, capability bindings, and every serialized router node/terminal.

## Parent intent

Bind and semantically verify the entire execution graph, especially serialized routing and schema identity.

## Dependencies

- `G0-001-Z` — `tasks/00_g0_gate_close/G0-001-Z-package-evidence-for-freeze-and-reconcile-the-4e69049-release-baseline.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/G0.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/schema-codecs.md`

### Source files

- `AGENTS.md`
- `crates/q-pack/src/lib.rs`
- `crates/q-router/src/lib.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `packages/compiler/src/emit.ts`
- `conformance/routing/routing.conformance.test.ts`
- `crates/q-pack/tests/fuzz_pack.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `docs/specs/pack-format-v1.md`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Define the execution graph projection to include function manifest, RoutePlans, policy bindings, schema manifest, capability bindings, and every serialized router node/terminal.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Any execution-critical router or schema mutation changes the hash or fails semantic verification.
- A structurally in-range but semantically wrong router is rejected before socket bind.
- Execution hash changes on internal plan changes while public contract identity remains independent.
- Empty, unreachable, duplicate-edge, or mismatched-method routers are rejected.

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

- Pack tamper tests.
- Canonicalization golden fixtures.
- Hash separation report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
g0-003-a: define the execution graph projection to include function ma
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
