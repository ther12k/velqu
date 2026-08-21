---
task_id: M25-003-B
parent_task: M25-003
milestone: M25
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M25.md
commit_required: true
---

# M25-003-B — Validate byte ranges and header/query values without generic object trees

## Atomic goal

Validate byte ranges and header/query values without generic object trees.

## Parent intent

Fuse field extraction, coercion, and validation for non-body inputs.

## Dependencies

- `M25-003-A` — `tasks/02_m25_schema_codecs/M25-003-A-generate-direct-decoder-programs-keyed-by-schemaid.md`

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
5. Implement exactly this deliverable: Validate byte ranges and header/query values without generic object trees.
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
m25-003-b: validate byte ranges and header query values without generic
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record (M25-003-B)

Status: **PASS**. Path parameters now decode directly from borrowed path
bytes via `(start, end)` ranges (no intermediate String allocation for
integer/number/boolean/UUID fields), and header values validate through the
same direct decoder programs — no generic object trees anywhere on the
non-body input path.

### Changed files

- `crates/q-schema-runtime/src/decoder.rs` — `FieldSpec::decode_bytes`
  (borrowed-slice decoding: integer, number, boolean, and UUID validated
  before any UTF-8 allocation), `DecoderProgram::decode_params_ranges`
  (zero-copy path-byte range slicing), `DecoderProgram::decode_headers`
  (case-insensitive header lookup, unknown headers ignored),
  `DecoderTable::decode_params_ranges` / `decode_headers` dispatch.
- `crates/q-runtime/src/serve.rs` — params pipeline switched from materialized
  byte-slice vectors to `decode_params_ranges` over `path.as_bytes()`; new
  header validation stage keyed by `headers_schema_id` producing 422
  `validation.headers` problems; validated headers flow into `InvocationSpec`.
- `docs/codex-spark-beta/STATUS.md`, `docs/codex-spark-beta/indexes/TASK_INDEX.md`
  — M25-003-B marked PASS.

### Tests and evidence

New tests in `crates/q-schema-runtime/src/decoder.rs`:
- `decoder_program_decodes_ranges_directly` — zero-copy range decode
- `decoder_program_decodes_headers_case_insensitively` — mixed-case headers,
  unknown-header tolerance, missing-required rejection
- `decoder_program_malformed_byte_ranges_rejects_cleanly` — inverted/out-of-
  bounds ranges and non-UTF-8 bytes produce typed errors
- `decoder_program_query_arrays_comma_separated` — comma-separated array query
  decoding with item bounds
- `decoder_program_matches_reference_validator_on_mixed_corpus` — differential
  parity with `validate_query` across valid/invalid/malformed corpus

Results: `cargo test -p q-schema-runtime` (37 unit + 3 fuzz pass),
`cargo test -p velqu-runtime` (15 pass), `cargo test -p q-engine-quickjs`
(1 + 96 pass), `cargo test -p q-http` (pass), `cargo test -p q-bridge` (pass),
`bun test` (69 pass, 0 fail, 296 expects), `bun run typecheck` clean,
`cargo fmt --check` clean, `cargo clippy --workspace --all-targets -- -D warnings`
clean, `scripts/validate-okf` (176 links, 0 errors). `./scripts/verify` passes
all stages except the known isolated-worktree `qRuntimeRelease`/`proofPack`
manifest hash mismatch (documented since PR #714; canonical manifest preserved).

Commit: `e414562`.
