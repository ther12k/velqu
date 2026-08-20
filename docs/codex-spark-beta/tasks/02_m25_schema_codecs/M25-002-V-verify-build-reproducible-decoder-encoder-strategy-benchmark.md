---
task_id: M25-002-V
parent_task: M25-002
milestone: M25
priority: P1
mode: VERIFY
status: TODO
context_card: context/milestones/M25.md
commit_required: true
---

# M25-002-V — Verify Build reproducible decoder/encoder strategy benchmark

## Atomic goal

Prove every acceptance criterion for parent task M25-002 without broadening scope.

## Parent intent

Measure QuickJS and native strategies across realistic payload shapes.

## Dependencies

- `M25-002-A` — `tasks/02_m25_schema_codecs/M25-002-A-compare-quickjs-parse-stringify-generic-rust-conversion-and-generated-schema-awa.md`
- `M25-002-B` — `tasks/02_m25_schema_codecs/M25-002-B-use-256b-1kb-16kb-64kb-nested-objects-arrays-100-1-000-optional-null-and-problem.md`
- `M25-002-C` — `tasks/02_m25_schema_codecs/M25-002-C-capture-cpu-allocation-bridge-time-and-tails.md`
- `M25-002-D` — `tasks/02_m25_schema_codecs/M25-002-D-select-strategies-by-evidence.md`

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
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`
- `crates/q-schema-runtime/tests/fuzz_validator.rs`
- `conformance/schema/schema.conformance.test.ts`
- `packages/cli/src/index.ts`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- Raw and generated results are committed.
- No single strategy is forced globally.
- Compiler decision rules are deterministic.
- Fallback cost is visible in inspect output.

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

- Benchmark raw data.
- Strategy decision report.
- Artifact hashes.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m25-002-v: verify build reproducible decoder encoder strategy benchmark
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
