---
task_id: M25-001-B
parent_task: M25-001
milestone: M25
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M25.md
commit_required: true
---

# M25-001-B — Define compatibility and fallback markers

## Atomic goal

Define compatibility and fallback markers.

## Parent intent

Create a versioned normalized schema model suitable for validation, decoding, encoding, OpenAPI, Treaty, and semantic diff.

## Dependencies

- `M25-001-A` — `tasks/02_m25_schema_codecs/M25-001-A-specify-objects-arrays-unions-literals-enums-formats-defaults-optional-null-tran.md`

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
5. Implement exactly this deliverable: Define compatibility and fallback markers.
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
m25-001-b: define compatibility and fallback markers
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record (M25-001-B)

Status: **PASS**. Compatibility and fallback markers defined end to end:

- **Fallback marker**: `SchemaIr::Fallback { reason, inner? }` with a closed
  reason vocabulary `FALLBACK_REASONS` = unsupported-transform |
  unrepresentable | measured | explicit (Rust + TS). With `inner`, native
  validation applies the best-effort shape (marker transparent); without
  `inner`, a typed `fallback` field error fails closed until the generic
  codec path lands (M25-004-B). No silent downgrade (ADR-0009).
- **Compatibility markers**: every schema manifest entry now carries
  `features` (sorted tags: fallback/file/problem/transform) derived by a
  shared walker (`features_of` Rust / `featuresOf` TS). `q-pack` verify
  fails closed when declared features ≠ derived features or when any
  fallback reason is outside the vocabulary.

### Changed files

- `crates/q-schema-runtime/src/lib.rs` — Fallback variant, FALLBACK_REASONS,
  `features_of`, `fallback_reasons`, `is_valid_fallback_reason`, Fallback
  validation arm, `m25_001_b_tests` (5 tests).
- `crates/q-schema-runtime/tests/fuzz_validator.rs` — 3 fallback corpus IRs.
- `crates/q-pack/src/lib.rs` — `SchemaDecl.features`, fail-closed
  feature/reason verification, 3 rejection tests.
- `crates/q-runtime/tests/runtime_conformance.rs` — fixture pack now uses
  `q_pack::SCHEMA_IR_VERSION` (latent v1 staleness from M25-001-A exposed by
  running this suite; fixed here), manifest builder derives features.
- `packages/schema/src/index.ts`, `index.d.ts`, `index.js` — `s_fallback`,
  `FALLBACK_REASONS`, `featuresOf`, `FEATURE_TAGS`, Schema union member,
  mirrors synced.
- `packages/compiler/src/extract.ts` — `fallback` extraction (literal reason
  from vocabulary, optional inner).
- `packages/compiler/src/emit.ts` — manifest `features` in Rust field order;
  `tsTypeOfIr` + OpenAPI fallback projections (`x-fallback: reason`).
- `benchmarks/harness/build-proof-pack.ts` — manifest entries carry features.
- `conformance/schema/schema.conformance.test.ts` — marker tests (5).
- `conformance/schema/golden.conformance.test.ts` — fallback corpus entries.
- `conformance/schema/golden/` — `fallback-with-inner.json`,
  `fallback-minimal.json`, COMPATIBILITY.md marker rows + projection row.

### Evidence

| Command | Result |
| --- | --- |
| `cargo test -p q-engine-quickjs` | 1 + 96 passed |
| `cargo test -p q-schema-runtime` | 24 lib + 2 fuzz passed |
| `cargo test -p q-pack` | 40 + 2 passed |
| `cargo test -p velqu-runtime` | 15 passed |
| `bun test` (full) | 59 passed |
| `bun run typecheck` | clean |
| `cargo fmt --check` | clean |
| `cargo clippy --workspace --all-targets -- -D warnings` | clean |
| `./scripts/validate-okf` | 175 links, 0 errors |

Test names (selection): `m25_001_b_tests::fallback_serde_round_trip_with_and_without_inner`,
`m25_001_b_tests::fallback_with_inner_validates_against_inner`,
`m25_001_b_tests::fallback_without_inner_fails_closed_with_typed_error`,
`m25_001_b_tests::fallback_rejects_unknown_reason`,
`m25_001_b_tests::features_are_derived_sorted_and_deduplicated`,
`golden_corpus_feature_expectations`,
`schema_manifest_features_mismatch_rejected`,
`schema_manifest_fallback_feature_must_be_declared`,
`schema_manifest_unknown_fallback_reason_rejected`,
"Fallback and compatibility markers (M25-001-B, ADR-0009)" (5 tests).

### Scope boundaries honored

- Canonical ordering/hashing algorithm unchanged (M25-001-C).
- Unsupported-transform documentation not claimed (M25-001-D).
- Generic fallback codec execution not built (M25-004-B owns wiring; the
  marker fails closed without inner until then).
- No performance claims made.
