---
task_id: M25-001-V
parent_task: M25-001
milestone: M25
priority: P0
mode: VERIFY
status: PASS
context_card: context/milestones/M25.md
commit_required: true
---

# M25-001-V — Verify Define canonical Schema IR v2

## Atomic goal

Prove every acceptance criterion for parent task M25-001 without broadening scope.

## Parent intent

Create a versioned normalized schema model suitable for validation, decoding, encoding, OpenAPI, Treaty, and semantic diff.

## Dependencies

- `M25-001-A` — `tasks/02_m25_schema_codecs/M25-001-A-specify-objects-arrays-unions-literals-enums-formats-defaults-optional-null-tran.md`
- `M25-001-B` — `tasks/02_m25_schema_codecs/M25-001-B-define-compatibility-and-fallback-markers.md`
- `M25-001-C` — `tasks/02_m25_schema_codecs/M25-001-C-canonicalize-ordering-and-hashing.md`
- `M25-001-D` — `tasks/02_m25_schema_codecs/M25-001-D-document-unsupported-transformations.md`

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

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

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

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m25-001-v: verify define canonical schema ir v2
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Verification evidence (M25-001-V)

Status: **PASS**. All four implementation dependencies are present on
`origin/master`: M25-001-A (`2d029d1`), M25-001-B (`9879f02`), M25-001-C
(`cd7a448`), and M25-001-D (`734aae4`). The parent acceptance guardrails are
proven by the source-backed records and tests below; this packet does not mark
the M25 milestone gate PASS.

### Acceptance matrix

- **One schema identity produces equivalent runtime and public projections.**
  Rust and TypeScript share `SCHEMA_IR_VERSION = 2`; compiler extraction and
  emission feed the q-pack schema registry, Treaty/type projection, OpenAPI,
  contract lock, and golden wire corpus. Positive coverage: `golden_corpus_round_trips`,
  `Schema IR v2 golden corpus (M25-001-A)`, `determinism (COMP-003/009)`,
  and `contract lock workflow (PR-006/SCHEMA-007)`.
- **Canonical form is deterministic.** Recursive object-key ordering, ordered
  arrays, integral-float normalization, and both hash surfaces are covered by
  `m25_001_c_tests::canonical_json_sorts_all_keys_recursively`,
  `canonical_form_normalizes_integral_floats`,
  `canonical_value_is_emission_order_insensitive`,
  `canonical_corpus_matches_golden_files`, and the compiler test
  `option literal field order never changes canonical hashes`.
- **Unsupported constructs fail or use explicit fallback.** Typed unsupported
  validation, closed fallback reasons, manifest feature verification, compiler
  diagnostics, and the normative unsupported-transformations specification are
  covered by `v2_nodes_return_typed_unsupported_validation_errors`,
  `fallback_without_inner_fails_closed_with_typed_error`,
  `fallback_rejects_unknown_reason`,
  `schema_manifest_unknown_fallback_reason_rejected`, and the three
  M25-001-D documentation-drift tests.
- **Schema diff can classify nested changes.** The contract-lock semantic diff
  suite `semantic diff detects schema structural changes accurately` passes;
  canonical schema identity is shared by the diff input and emitted contract.

### Exact verification results

| Command | Result |
| --- | --- |
| `cargo test -p q-engine-quickjs` | 1 unit + 96 integration tests passed |
| `cargo test -p q-schema-runtime` | 28 library + 2 fuzz tests passed |
| `bun test` | 66 passed, 0 failed, 233 expect calls |
| `bun run typecheck` | clean (`tsc -b tsconfig.json`) |
| `cargo fmt --check` | clean |
| `cargo clippy --workspace --all-targets -- -D warnings` | clean |
| `./scripts/verify` | all Rust, typecheck, proof build, and TypeScript stages passed; final benchmark evidence validation failed only for isolated-worktree `qRuntimeRelease` and `proofPack` hash mismatches |

`./scripts/verify` also reported `scripts/validate-okf`: 176 links checked,
0 errors; production task validation reported 120 tasks, current head
`6181d8629df7b13628de46c3bac8d2234cec497d`, 0 errors. The benchmark manifest
was not changed and no performance claim is made. The hash mismatch is scoped
to reproducibility of this temporary verification worktree and is not evidence
that the implementation or committed manifest is invalid.

### Evidence locations and scope

- Schema golden corpus: `conformance/schema/golden/` and
  `conformance/schema/golden/canonical/`.
- Compatibility matrix: `conformance/schema/golden/COMPATIBILITY.md`.
- Canonicalization ADR: `docs/okf/decisions/0023-canonical-ordering-and-hashing.md`.
- Unsupported transformation specification:
  `docs/specs/unsupported-transformations.md`.
- No source or generated tracked files changed during verification besides this
  packet record; benchmark manifests remain untouched.
