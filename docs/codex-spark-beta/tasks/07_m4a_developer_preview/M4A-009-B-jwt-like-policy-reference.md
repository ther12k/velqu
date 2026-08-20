---
task_id: M4A-009-B
parent_task: M4A-009
milestone: M4A
priority: P0
mode: IMPLEMENT
status: TODO
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-009-B — JWT-like policy reference

## Atomic goal

JWT-like policy reference.

## Parent intent

Validate 30–50 routes, auth, fetch, validation, errors, pagination, and deployment.

## Dependencies

- `M4A-009-A` — `tasks/07_m4a_developer_preview/M4A-009-A-feature-modules.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M4A.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`
- `context/components/schema-codecs.md`

### Source files

- `AGENTS.md`
- `packages/cli/src/index.ts`
- `packages/compiler/src/index.ts`
- `packages/treaty/src/index.ts`
- `packages/testing/src/index.ts`
- `crates/q-schema-runtime/src/lib.rs`
- `crates/q-schema-runtime/tests/fuzz_validator.rs`
- `crates/q-engine-quickjs/src/convert.rs`
- `packages/schema/src/index.ts`
- `conformance/schema/schema.conformance.test.ts`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-http/src/lib.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: JWT-like policy reference.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Runs entirely on actual runtime.
- No hidden Bun production path.
- All error/status contracts declared.
- Load and failure scenarios pass.

## Targeted commands

```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p q-http
```
```bash
cargo test -p q-schema-runtime
```
```bash
cargo test -p q-capabilities
```
```bash
bun test
```
```bash
bun run typecheck
```

## Required evidence for this microtask

- Proof app source.
- Scenario tests.
- Benchmark report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m4a-009-b: jwt like policy reference
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
