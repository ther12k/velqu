---
task_id: M25-005-V
parent_task: M25-005
milestone: M25
priority: P0
mode: VERIFY
status: TODO
context_card: context/milestones/M25.md
commit_required: true
---

# M25-005-V — Verify Generate status-specific response encoders

## Atomic goal

Prove every acceptance criterion for parent task M25-005 without broadening scope.

## Parent intent

Fuse output validation and serialization for stable response contracts.

## Dependencies

- `M25-005-A` — `tasks/02_m25_schema_codecs/M25-005-A-generate-per-status-encoders.md`
- `M25-005-B` — `tasks/02_m25_schema_codecs/M25-005-B-read-declared-properties-in-fixed-order.md`
- `M25-005-C` — `tasks/02_m25_schema_codecs/M25-005-C-handle-optional-null-union-fields.md`
- `M25-005-D` — `tasks/02_m25_schema_codecs/M25-005-D-keep-quickjs-stringify-or-generic-fallback-when-measured-better.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M25.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`
- `context/components/schema-codecs.md`

### Source files

- `AGENTS.md`
- `crates/q-schema-runtime/src/lib.rs`
- `crates/q-engine-quickjs/src/convert.rs`
- `packages/schema/src/index.ts`
- `packages/compiler/src/emit.ts`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`
- `crates/q-http/src/lib.rs`
- `crates/q-bridge/src/lib.rs`
- `crates/q-runtime/src/serve.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- Undeclared status/body remains a contract violation.
- Output is JSON-equivalent to reference serialization.
- One traversal for generated paths.
- No user JS escapes deadline ownership during conversion.

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
cargo test -p q-schema-runtime
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

- Golden JSON corpus.
- Response mismatch tests.
- Mapping deadline tests.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m25-005-v: verify generate status specific response encoders
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
