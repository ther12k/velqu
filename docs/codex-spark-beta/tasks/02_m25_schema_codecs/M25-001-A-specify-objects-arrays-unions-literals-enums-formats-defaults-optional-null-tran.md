---
task_id: M25-001-A
parent_task: M25-001
milestone: M25
priority: P0
mode: IMPLEMENT
status: TODO
context_card: context/milestones/M25.md
commit_required: true
---

# M25-001-A — Specify objects, arrays, unions, literals, enums, formats, defaults, optional/null, transforms, files, and problem schemas

## Atomic goal

Specify objects, arrays, unions, literals, enums, formats, defaults, optional/null, transforms, files, and problem schemas.

## Parent intent

Create a versioned normalized schema model suitable for validation, decoding, encoding, OpenAPI, Treaty, and semantic diff.

## Dependencies

- `M24-GATE` — `gates/M24-GATE.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M25.md`
- `context/components/engine-scheduler.md`
- `context/components/schema-codecs.md`
- `context/components/devex-beta.md`

### Source files

- `AGENTS.md`
- `crates/q-schema-runtime/src/lib.rs`
- `crates/q-engine-quickjs/src/convert.rs`
- `packages/schema/src/index.ts`
- `packages/compiler/src/emit.ts`
- `crates/q-schema-runtime/tests/fuzz_validator.rs`
- `conformance/schema/schema.conformance.test.ts`
- `packages/treaty/src/index.ts`
- `packages/contract/src/index.ts`
- `packages/testing/src/index.ts`
- `conformance/treaty/treaty.conformance.test.ts`
- `scripts/package`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Specify objects, arrays, unions, literals, enums, formats, defaults, optional/null, transforms, files, and problem schemas.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- One schema identity produces equivalent runtime and public projections.
- Canonical form is deterministic.
- Unsupported constructs fail or use explicit fallback.
- Schema diff can classify nested changes.

## Targeted commands

```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p q-schema-runtime
```
```bash
bun test
```
```bash
bun run typecheck
```

## Required evidence for this microtask

- Schema golden corpus.
- Canonicalization tests.
- Compatibility matrix.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m25-001-a: specify objects arrays unions literals enums formats default
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
