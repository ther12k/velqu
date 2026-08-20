---
task_id: M25-001-Z
parent_task: M25-001
milestone: M25
priority: P0
mode: EVIDENCE
status: TODO
context_card: context/milestones/M25.md
commit_required: true
---

# M25-001-Z — Package evidence for Define canonical Schema IR v2

## Atomic goal

Create source-backed evidence and handoff for parent task M25-001; update status only if verification passed.

## Parent intent

Create a versioned normalized schema model suitable for validation, decoding, encoding, OpenAPI, Treaty, and semantic diff.

## Dependencies

- `M25-001-V` — `tasks/02_m25_schema_codecs/M25-001-V-verify-define-canonical-schema-ir-v2.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M25.md`
- `context/components/engine-scheduler.md`
- `context/components/schema-codecs.md`
- `context/components/evidence.md`

### Source files

- `AGENTS.md`
- `crates/q-schema-runtime/src/lib.rs`
- `crates/q-engine-quickjs/src/convert.rs`
- `packages/schema/src/index.ts`
- `packages/compiler/src/emit.ts`
- `crates/q-schema-runtime/tests/fuzz_validator.rs`
- `conformance/schema/schema.conformance.test.ts`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

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

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m25-001-z: package evidence for define canonical schema ir v2
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
