---
task_id: M25-001-V
parent_task: M25-001
milestone: M25
priority: P0
mode: VERIFY
status: TODO
context_card: context/milestones/M25.md
commit_required: true
---

# M25-001-V — Verify Define canonical Schema IR v2

## Atomic goal

Prove every acceptance criterion for parent task M25-001 without broadening scope.

## Parent intent

Create a versioned normalized schema model suitable for validation, decoding, encoding, OpenAPI, Treaty, and semantic diff.

## Dependencies

- `M25-001-A` — `tasks/02_m25_schema_codecs/M25-001-A-specify-objects-arrays-unions-literals-enums-formats-defaults-optional-null-tran.md`
- `M25-001-B` — `tasks/02_m25_schema_codecs/M25-001-B-define-compatibility-and-fallback-markers.md`
- `M25-001-C` — `tasks/02_m25_schema_codecs/M25-001-C-canonicalize-ordering-and-hashing.md`
- `M25-001-D` — `tasks/02_m25_schema_codecs/M25-001-D-document-unsupported-transformations.md`

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

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

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

- Schema golden corpus.
- Canonicalization tests.
- Compatibility matrix.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m25-001-v: verify define canonical schema ir v2
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
