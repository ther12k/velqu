---
task_id: M25-004-A
parent_task: M25-004
milestone: M25
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M25.md
commit_required: true
---

# M25-004-A — Implement generated direct decode where supported

## Atomic goal

Implement generated direct decode where supported.

## Parent intent

Parse and validate declared JSON bodies with one route-selected strategy.

## Dependencies

- `M25-001-Z` — `tasks/02_m25_schema_codecs/M25-001-Z-package-evidence-for-define-canonical-schema-ir-v2.md`
- `M24-007-Z` — `tasks/01_m24_zero_copy_ingress/M24-007-Z-package-evidence-for-implement-bounded-read-once-body-admission.md`

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
- `crates/q-http/src/lib.rs`
- `crates/q-bridge/src/lib.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-runtime/tests/runtime_conformance.rs`
- `crates/q-schema-runtime/tests/fuzz_validator.rs`
- `conformance/schema/schema.conformance.test.ts`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Implement generated direct decode where supported.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- One successful decode representation crosses to JS.
- Oversize/deep inputs fail boundedly.
- No semantic drift from schema.
- Fallback is explicit in build report.

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

## Required evidence for this microtask

- Fuzz/differential tests.
- Depth/size boundary tests.
- CPU/allocation results.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m25-004-a: implement generated direct decode where supported
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record (M25-004-A)

Status: **PASS**. Direct JSON body decode was implemented in `q-schema-runtime`
via `FieldSpec::decode_value`, `DecoderProgram::decode_body_value`, and
`DecoderTable::decode_body_value`. Bounded read-once body admission in `q-runtime`
invokes the SchemaId-indexed direct decoder to validate and inject the single
normalized body representation into `InvocationSpec`.

### Changed files

- `crates/q-schema-runtime/src/decoder.rs` — `FieldSpec::decode_value`
  (strict `Source::Body` value validation: exact JSON scalar types, nested
  objects, arrays, optionals with defaults, nullables), `DecoderProgram::decode_body_value`
  (rejects non-object, unknown keys, missing required keys), `DecoderTable::decode_body_value`
  dispatch, unit tests with differential parity check against `validate(Source::Body)`.
- `crates/q-runtime/src/serve.rs` — request pipeline calls `state.decoder_table.decode_body_value`
  for declared JSON bodies.
- `docs/codex-spark-beta/STATUS.md`, `docs/codex-spark-beta/indexes/TASK_INDEX.md`
  — M25-004-A marked PASS.

### Tests and evidence

- `cargo test -p q-schema-runtime` — 39 unit tests + 3 fuzz tests passed.
- `cargo test -p velqu-runtime` — 15 integration tests passed.
- `cargo test -p q-engine-quickjs` — 1 unit + 96 integration tests passed.
- `cargo test -p q-http` — 4 tests passed.
- `cargo test -p q-bridge` — 11 passed.
- `bun test` — 69 passed, 0 failed, 296 expect calls.
- `bun run typecheck` — clean.
- `cargo fmt --check` — clean.
- `cargo clippy --workspace --all-targets -- -D warnings` — clean.
- `scripts/validate-okf` — 176 links, 0 errors.

Commit: `40e4e3a`.
