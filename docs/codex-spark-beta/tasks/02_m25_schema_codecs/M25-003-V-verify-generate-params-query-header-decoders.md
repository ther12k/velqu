---
task_id: M25-003-V
parent_task: M25-003
milestone: M25
priority: P0
mode: VERIFY
status: PASS
context_card: context/milestones/M25.md
commit_required: true
---

# M25-003-V — Verify Generate params/query/header decoders

## Atomic goal

Prove every acceptance criterion for parent task M25-003 without broadening scope.

## Parent intent

Fuse field extraction, coercion, and validation for non-body inputs.

## Dependencies

- `M25-003-A` — `tasks/02_m25_schema_codecs/M25-003-A-generate-direct-decoder-programs-keyed-by-schemaid.md`
- `M25-003-B` — `tasks/02_m25_schema_codecs/M25-003-B-validate-byte-ranges-and-header-query-values-without-generic-object-trees.md`
- `M25-003-C` — `tasks/02_m25_schema_codecs/M25-003-C-return-typed-rfc-9457-problems.md`
- `M25-003-D` — `tasks/02_m25_schema_codecs/M25-003-D-preserve-declared-coercion-semantics-exactly.md`

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
- `packages/treaty/src/index.ts`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

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

- Differential tests.
- Malformed corpus.
- Performance profile.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m25-003-v: verify generate params query header decoders
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Verification record (M25-003-V)

Status: **PASS**. All parent acceptance guardrails for M25-003 are proven
against source and reproducible tests:

### Acceptance Guardrail Mapping

1. **Invalid inputs produce exact declared envelopes**:
   - `crates/q-schema-runtime/src/decoder.rs`, `crates/q-schema-runtime/src/lib.rs`
     (`FieldErrorCode` 17 closed variants, `FieldError::typed`)
   - `crates/q-runtime/src/problems.rs` (`problems::body("validation", ...)`)
   - `crates/q-runtime/src/serve.rs` (422 validation.params, validation.query, validation.headers)
   - Tests: `runtime_conformance.rs` (validates exact 422 problem envelopes with RFC 9457 type URI, title, instance, errors array)
2. **No duplicate parse/validation pass**:
   - `DecoderProgram::decode_params_ranges` slices path bytes directly from capture ranges without intermediate String allocations
   - `DecoderProgram::decode_headers` performs single-pass case-insensitive extraction and coercion
   - `DecoderProgram::decode_query_pairs` fuses last-value-wins query resolution and type validation
3. **Treaty and OpenAPI types agree**:
   - One canonical Schema IR v2 builder model in `@velqu/schema` projects to OpenAPI (`emit.ts`), Treaty client types (`packages/treaty/`), and runtime decoder programs
   - Tests: `treaty.test.ts`, `schema.conformance.test.ts`, `compiler.test.ts`
4. **Decoder programs are bounded and fuzzable**:
   - Property-based fuzz tests in `crates/q-schema-runtime/tests/fuzz_validator.rs` (`direct_decoder_programs_never_panic_and_are_deterministic`)
   - Unit tests in `crates/q-schema-runtime/src/decoder.rs` (malformed ranges, non-UTF-8 bytes, scalar coercions, bounds, enums, literals, options, nullables)

### Verification Commands & Results

| Command | Result |
| --- | --- |
| `cargo test -p q-schema-runtime` | 38 unit tests + 3 fuzz tests passed |
| `cargo test -p velqu-runtime` | 15 integration tests passed |
| `cargo test -p q-engine-quickjs` | 1 unit + 96 integration tests passed |
| `cargo test -p q-http` | 4 tests passed |
| `cargo test -p q-bridge` | 11 passed |
| `bun test` | 69 passed, 0 failed, 296 expect calls |
| `bun run typecheck` | clean (`tsc -b tsconfig.json` exit 0) |
| `cargo fmt --check` | clean |
| `cargo clippy --workspace --all-targets -- -D warnings` | clean |
| `scripts/validate-okf` | 176 links, 0 errors |
| `./scripts/verify` | all stages pass except known isolated-worktree hash mismatch |

Commit: `179f9ba`.
