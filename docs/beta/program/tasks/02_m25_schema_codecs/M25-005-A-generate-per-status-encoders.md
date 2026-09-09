---
task_id: M25-005-A
parent_task: M25-005
milestone: M25
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M25.md
commit_required: true
---

# M25-005-A — Generate per-status encoders

## Atomic goal

Generate per-status encoders.

## Parent intent

Fuse output validation and serialization for stable response contracts.

## Dependencies

- `M25-001-Z` — `tasks/02_m25_schema_codecs/M25-001-Z-package-evidence-for-define-canonical-schema-ir-v2.md`
- `M25-002-Z` — `tasks/02_m25_schema_codecs/M25-002-Z-package-evidence-for-build-reproducible-decoder-encoder-strategy-benchmark.md`

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

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Generate per-status encoders.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

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
cargo test -p q-schema-runtime
```
```bash
bun test
```
```bash
bun run typecheck
```

## Required evidence for this microtask

- Golden JSON corpus.
- Response mismatch tests.
- Mapping deadline tests.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m25-005-a: generate per status encoders
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record (M25-005-A)

Status: **PASS**. Generated per-status response encoders fuse output
validation and JSON serialization into one traversal:

- `crates/q-schema-runtime/src/encoder.rs` (new): `EncoderProgram` compiles
  a representable object SchemaIr into a direct encoder; `encode` walks the
  handler value ONCE, producing typed field errors exactly where the
  reference validator produces them and emitting canonical JSON bytes in
  declared property order (defaults inserted, integer-under-Number
  normalized to float form). Leaf bytes delegate to `serde_json::to_writer`
  so escaping/number formatting can never drift from the reference
  serialization. `EncoderTable` is dense by SchemaId, mirroring
  `DecoderTable`. Schemas that are not directly encodable (nested object
  properties, unions, transforms, files, problems, fallback-without-inner,
  non-object top levels) compile to `None` — the runtime keeps the
  reference validate-then-serialize path for those routes instead of
  failing closed.
- `crates/q-runtime/src/serve.rs` + `main.rs`: `encoder_table` and a
  per-route `response_schema_ids` map (status → SchemaId, resolved once at
  startup) in `ServeState`. For `BodyOut::Json` on a declared response
  schema with a program, the response block encodes once and the mapping
  stage writes those bytes — the previous validate pass + Value clone +
  second serialization pass collapse into one traversal. Encoder typed
  errors map to the same controlled 500 `contract.violation.response`
  (detail logged, redacted from the response).
- Guardrails: undeclared status/body stays a contract violation (the
  declared-status gate is unchanged; the existing
  `response_schema_violation_is_a_controlled_500` test now exercises the
  encoder path since its flat schema compiles); output is byte-equal to
  the reference normalized serialization (golden corpus); one traversal
  for generated paths (no intermediate Value, no second serialize); no
  user JS executes during conversion — encode is native host code after
  engine settlement, and recursion is depth-bounded by
  `MAX_VALIDATE_DEPTH` (constraint 11).

### Tests and evidence

- `encoder_matches_reference_serialization_on_golden_corpus` — golden
  corpus (scalars, formats email/uuid, pattern, bounds, optional
  present/absent/default/null, nullable, arrays incl. nested, literals,
  enums, unicode/escapes, integer-under-Number normalization, out-of-order
  handler keys): encoder bytes == `serde_json::to_vec` of the reference
  validator output, case by case.
- `encoder_rejects_mismatches_with_reference_parity` — response mismatch
  matrix (unknown key, missing required, wrong types, bound violations,
  format/pattern/literal/enum misses, empty array, non-object): identical
  typed code+path pairs versus the reference validator.
- `encoder_depth_is_bounded` — mapping deadline evidence: schema+value
  nested past `MAX_VALIDATE_DEPTH` reject with the typed `depth` problem,
  parity with the reference bound; no unbounded stack work.
- `unrepresentable_schemas_compile_to_none` — nested object, union,
  transform, file, fallback-without-inner, non-object top level compile to
  `None` (fallback-with-inner stays encodable).
- `encoder_table_is_dense_by_schema_id` — dense SchemaId indexing.
- `runtime_conformance::native_response_encoder_emits_declared_order` —
  live HTTP: valid handler response arrives byte-exact in declared
  (byte-sorted) property order; the mismatch twin returns a controlled 500
  through the same encoder path with the contract violation logged
  internally and redacted from the wire.
- `cargo test -p q-engine-quickjs` — 1 + 96 passed.
- `cargo test -p q-schema-runtime` — 50 unit + 3 fuzz passed.
- `cargo test -p velqu-runtime` — 19 integration passed.
- `bun test` — 69 passed, 0 failed, 297 expect calls.
- `bun run typecheck` — clean. `cargo fmt --check` — clean.
- `cargo clippy --workspace --all-targets -- -D warnings` — clean.
- `scripts/validate-okf` — 176 links, 0 errors.
- `./scripts/verify` — all stages pass except the documented
  isolated-worktree `qRuntimeRelease`/`proofPack` manifest hash mismatch
  (known, pre-existing on every packet branch).

No performance claim is made in this packet (measurement belongs to the
M25-002 instrumented harness; benchmark manifest preserved unchanged).

Commit: `38e2731`.
