---
task_id: M25-001-A
parent_task: M25-001
milestone: M25
priority: P0
mode: IMPLEMENT
status: PASS
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

## Completion record (M25-001-A)

Status: **PASS**. Schema IR v2 specified end to end: Rust + TypeScript share one
versioned wire model (`SCHEMA_IR_VERSION = 2`), with declarative `transform`,
`file`, and `problem` nodes; `q-pack` fails closed on any other schema IR
version. Decision record: `docs/okf/decisions/0022-schema-ir-v2.md`.

### Changed files

- `crates/q-schema-runtime/src/lib.rs` — v2 variants (camelCase serde, absent
  options omitted), `SCHEMA_IR_VERSION`, typed `unsupported` validation errors
  for codec-deferred nodes, object null-member fix, array coercion propagation,
  `m25_001_a_tests` (serde round-trips, version boundary, validation semantics,
  golden corpus round-trip).
- `crates/q-schema-runtime/tests/fuzz_validator.rs` — v2 nodes added to the
  no-panic/determinism corpus.
- `crates/q-pack/src/lib.rs` — `SCHEMA_IR_VERSION = 2` (load-time version gate).
- `packages/schema/src/index.ts`, `index.d.ts`, `index.js` — v2 builders
  (`s_transform`/`s_file`/`s_problem`) with builder-time bounds, `JsonLiteral`
  literal domain, `SCHEMA_IR_VERSION`, `MAX_FILE_BYTES`; mirrors synced.
- `packages/compiler/src/extract.ts` — v2 extraction with source-located
  bounds diagnostics and canonical field order (Rust declaration parity).
- `packages/compiler/src/emit.ts` — `schemaIrVersion: 2`; `tsTypeOfIr` and
  OpenAPI projections for transform/file/problem (+ `union` → `oneOf` fix).
- `benchmarks/harness/build-proof-pack.ts` — fixture packs emit v2.
- `conformance/schema/schema.conformance.test.ts` — v2 builder/bounds/
  composition tests.
- `conformance/schema/golden/` — golden corpus (6 nodes incl. full nested
  composition) + `COMPATIBILITY.md` compatibility matrix.
- `conformance/schema/golden.conformance.test.ts` — builder-vs-corpus
  byte-identity and key-order stability.

### Evidence

| Command | Result |
| --- | --- |
| `cargo test -p q-engine-quickjs` | 97 passed |
| `cargo test -p q-schema-runtime` | 18 lib + 2 fuzz passed |
| `cargo test -p q-pack` | 37 + 2 passed |
| `bun test` (full) | 52 passed |
| `bun run typecheck` | clean |
| `cargo fmt --check` | clean |
| `cargo clippy --workspace --all-targets -- -D warnings` | clean |
| `./scripts/validate-okf` | 175 links checked, 0 errors |

Test names (selection): `m25_001_a_tests::schema_ir_version_is_two`,
`m25_001_a_tests::transform_serde_round_trip_camel_case`,
`m25_001_a_tests::file_serde_round_trip_omits_absent_content_type`,
`m25_001_a_tests::problem_serde_round_trip_camel_case`,
`m25_001_a_tests::v2_nodes_return_typed_unsupported_validation_errors`,
`m25_001_a_tests::object_rejects_null_for_non_nullable_member`,
`m25_001_a_tests::object_accepts_null_for_nullable_and_optional_members`,
`m25_001_a_tests::query_array_items_coerce_strings_consistently`,
`m25_001_a_tests::golden_corpus_round_trips`,
`validator_never_panics_and_is_deterministic` (extended),
"Schema IR v2 nodes (SCHEMA-001, IR v2)" (9 tests),
"Schema IR v2 golden corpus (M25-001-A)" (7 tests).

### Scope boundaries honored

- Compatibility/fallback markers: **not built** (M25-001-B).
- Canonical ordering/hash algorithm: **unchanged**; only structural parity
  fixtures shipped (M25-001-C owns the algorithm).
- Unsupported-transform documentation: **not claimed** (M25-001-D).
- No benchmark or performance claims made.
