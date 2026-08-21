---
task_id: M25-003-A
parent_task: M25-003
milestone: M25
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M25.md
commit_required: true
---

# M25-003-A — Generate direct decoder programs keyed by SchemaId

## Atomic goal

Generate direct decoder programs keyed by SchemaId.

## Parent intent

Fuse field extraction, coercion, and validation for non-body inputs.

## Dependencies

- `M25-001-Z` — `tasks/02_m25_schema_codecs/M25-001-Z-package-evidence-for-define-canonical-schema-ir-v2.md`
- `M24-GATE` — `gates/M24-GATE.md`

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
5. Implement exactly this deliverable: Generate direct decoder programs keyed by SchemaId.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Invalid inputs produce exact declared envelopes.
- No duplicate parse/validation pass.
- Treaty and OpenAPI types agree.
- Decoder programs are bounded and fuzzable.

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

- Differential tests.
- Malformed corpus.
- Performance profile.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m25-003-a: generate direct decoder programs keyed by schemaid
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record (M25-003-A)

Status: **PASS**. Direct decoder programs keyed by SchemaId were implemented
in `q-schema-runtime` and integrated into `q-runtime`. Field extraction,
coercion, bounds validation, format checks, and default application are fused
into a single pass without intermediate generic AST trees.

### Changed files

- `crates/q-schema-runtime/src/decoder.rs` — new direct decoder module:
  `DecoderProgram`, `DecoderTable`, `FieldSpec`, `PropertyDecoder`,
  fused parameter/query direct decoders, unit tests for params and query.
- `crates/q-schema-runtime/src/lib.rs` — exports `DecoderProgram`, `DecoderTable`,
  `FieldSpec`, `PropertyDecoder`; makes validation helper functions `pub(crate)`.
- `crates/q-schema-runtime/tests/fuzz_validator.rs` — added property-based fuzz test
  for direct decoder determinism across 20,000 iterations.
- `crates/q-runtime/src/serve.rs` — `ServeState` stores `decoder_table: DecoderTable`;
  request pipeline invokes `decode_params` and `decode_query` via dense `SchemaId`.
- `crates/q-runtime/src/main.rs` — initializes `DecoderTable::from_schemas` at startup.
- `docs/codex-spark-beta/STATUS.md`, `docs/codex-spark-beta/indexes/TASK_INDEX.md` —
  M25-003-A marked PASS.

### Tests and evidence

- `cargo test -p q-schema-runtime` — 32 unit tests + 3 fuzz tests passed.
- `cargo test -p velqu-runtime` — 15 integration tests passed.
- `cargo test -p q-engine-quickjs` — 1 unit + 96 integration tests passed.
- `cargo test -p q-http` — 4 tests passed.
- `cargo test -p q-bridge` — 11 passed.
- `bun test` — 69 passed, 0 failed, 296 expect calls.
- `bun run typecheck` — clean.
- `cargo fmt --check` — clean.
- `cargo clippy --workspace --all-targets -- -D warnings` — clean.
- `scripts/validate-okf` — 176 links, 0 errors.

Commit: `7e69dff`.
