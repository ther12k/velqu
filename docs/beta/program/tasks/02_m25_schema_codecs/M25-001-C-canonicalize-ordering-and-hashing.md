---
task_id: M25-001-C
parent_task: M25-001
milestone: M25
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M25.md
commit_required: true
---

# M25-001-C — Canonicalize ordering and hashing

## Atomic goal

Canonicalize ordering and hashing.

## Parent intent

Create a versioned normalized schema model suitable for validation, decoding, encoding, OpenAPI, Treaty, and semantic diff.

## Dependencies

- `M25-001-B` — `tasks/02_m25_schema_codecs/M25-001-B-define-compatibility-and-fallback-markers.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M25.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/schema-codecs.md`

### Source files

- `AGENTS.md`
- `crates/q-schema-runtime/src/lib.rs`
- `crates/q-engine-quickjs/src/convert.rs`
- `packages/schema/src/index.ts`
- `packages/compiler/src/emit.ts`
- `crates/q-pack/src/lib.rs`
- `crates/q-pack/tests/fuzz_pack.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `docs/specs/pack-format-v1.md`
- `crates/q-schema-runtime/tests/fuzz_validator.rs`
- `conformance/schema/schema.conformance.test.ts`
- `packages/treaty/src/index.ts`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Canonicalize ordering and hashing.
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
cargo test -p q-pack
```
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
m25-001-c: canonicalize ordering and hashing
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record (M25-001-C)

Status: **PASS**. Canonical ordering and hashing implemented per ADR-0023:
recursively sorted object keys, ordered arrays, integral-float normalization
(0.0 → 0). Both hash surfaces — the execution-graph hash
(`routes_canonical_sha256` / `routes_canonical_json`) and the public-contract
hash (`public_contract_canonical_json` / TS `contractHash`) — canonicalize
their whole view. Source literal field order no longer affects any hash.

### Changed files

- `crates/q-schema-runtime/src/lib.rs` — `canonical_value`, `canonical_json`,
  `m25_001_c_tests` (4 tests incl. golden canonical corpus).
- `crates/q-pack/src/lib.rs` — both canonical surfaces pass through
  `canonical_value`; hashing path matches `routes_canonical_json` bytes.
- `packages/schema/src/index.ts`, `index.d.ts`, `index.js` — `canonicalValue`,
  `canonicalJson` exports (shared canonical form for diff/hashing).
- `packages/compiler/src/emit.ts` — `sortIR` now full recursive key sort;
  `rustCanonical` and `publicContractCanonical` canonicalize whole views.
- `benchmarks/harness/build-proof-pack.ts` — fixture canonicalization mirrors
  the same form (bundle/contract hashes stay runtime-verified).
- `conformance/schema/golden/canonical/*.canonical.json` — committed canonical
  corpus (8 nodes), byte-exact expectation for both languages.
- `conformance/schema/schema.conformance.test.ts` — canonical form suite (3).
- `conformance/compiler/compiler.test.ts` + fixtures `order-a-app.ts` /
  `order-b-app.ts` — reversed option orders compile to identical
  `routesSha256`/`contractHash`.
- `docs/okf/decisions/0023-canonical-ordering-and-hashing.md` (+ index),
  `docs/specs/pack-format-v1.md` (canonicalization note),
  `conformance/schema/golden/COMPATIBILITY.md` (matrix updated).

### Evidence

| Command | Result |
| --- | --- |
| `cargo test -p q-schema-runtime` | 28 lib + 2 fuzz passed |
| `cargo test -p q-pack` | 40 + 2 passed |
| `cargo test -p q-engine-quickjs` | 1 + 96 passed |
| `cargo test -p velqu-runtime` | 15 passed |
| `bun test` (full) | 63 passed |
| `bun run typecheck` | clean |
| `cargo fmt --check` / `clippy -D warnings` | clean |
| `./scripts/validate-okf` | 0 errors |

Test names (selection): `m25_001_c_tests::canonical_json_sorts_all_keys_recursively`,
`m25_001_c_tests::canonical_form_normalizes_integral_floats`,
`m25_001_c_tests::canonical_value_is_emission_order_insensitive`,
`m25_001_c_tests::canonical_corpus_matches_golden_files`,
"option literal field order never changes canonical hashes" (compiler),
"canonical corpus matches committed golden canonical files" (schema).

### Notes

- Pack hash values change by design (canonical bytes are hashed); packs
  rebuild. Benchmark evidence refresh with raw samples happens at the M25
  gate, not in this packet (ADR-0012 discipline).
- No performance claims made.
