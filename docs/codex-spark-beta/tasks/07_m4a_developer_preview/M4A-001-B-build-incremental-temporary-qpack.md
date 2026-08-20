---
task_id: M4A-001-B
parent_task: M4A-001
milestone: M4A
priority: P0
mode: IMPLEMENT
status: TODO
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-001-B — Build incremental temporary QPack

## Atomic goal

Build incremental temporary QPack.

## Parent intent

Compile and reload the real QuickJS/QPack runtime with fast feedback and parity.

## Dependencies

- `M4A-001-A` — `tasks/07_m4a_developer_preview/M4A-001-A-watch-source-and-contracts.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M4A.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/devex-beta.md`

### Source files

- `AGENTS.md`
- `packages/cli/src/index.ts`
- `packages/compiler/src/index.ts`
- `packages/treaty/src/index.ts`
- `packages/testing/src/index.ts`
- `crates/q-pack/src/lib.rs`
- `crates/q-pack/tests/fuzz_pack.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `docs/specs/pack-format-v1.md`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Build incremental temporary QPack.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- No Bun-only behavior mismatch by default.
- Failed reload keeps prior healthy app.
- Source maps point to TypeScript.
- Reload is bounded and observable.

## Targeted commands

```bash
cargo test -p q-pack
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

- Reload conformance.
- Failure recovery tests.
- Developer latency measurements.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m4a-001-b: build incremental temporary qpack
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
