---
task_id: M25-005-C
parent_task: M25-005
milestone: M25
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M25.md
commit_required: true
---

# M25-005-C — Handle optional/null/union fields

## Atomic goal

Handle optional/null/union fields.

## Parent intent

Fuse output validation and serialization for stable response contracts.

## Dependencies

- `M25-005-B` — `tasks/02_m25_schema_codecs/M25-005-B-read-declared-properties-in-fixed-order.md`

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

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Handle optional/null/union fields.
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
m25-005-c: handle optional null union fields
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record (M25-005-C)

Status: **PASS**. The direct encoder handles optional, nullable, and union
fields with reference parity:

- **Unions** (`crates/q-schema-runtime/src/encoder.rs`): union properties
  now compile into the program when EVERY member is encodable (a union
  carrying any unencodable member — nested object, transform, ... — keeps
  the reference path, since a value could match only that member).
  Encoding is first-match-wins in declared member order, mirroring the
  reference validator: each attempt encodes into a SCRATCH buffer so a
  failed member's partial bytes never reach the response; only the
  winning member's bytes append. A no-match value produces the same typed
  `union` problem (code, path, message) as the reference.
- **Optional/Nullable**: the combination semantics are proven against the
  reference validator — Optional absorbs null into its declared default
  before Nullable ever sees it, Nullable<Optional> null resolves to bare
  null, and nested Optional defaults collapse outermost-first on absence.
  (The A-packet implementation already encoded these cases; C adds the
  combination-matrix evidence.)

### Tests and evidence

- `unions_encode_via_first_matching_member_with_parity` — first member
  matches (byte parity vs reference); first member fails mid-array after
  partial `[1,` bytes and the second member wins (output contains ONLY
  the winning bytes — scratch isolation); no member matches (typed
  `union` error with reference parity).
- `optional_null_combinations_match_reference` — six-case combination
  matrix (Optional<Nullable> with null/absent/value,
  Nullable<Optional> with null, nested Optional<Optional> with
  absent/null), each byte-equal to the reference normalized output.
- `unrepresentable_schemas_compile_to_none` updated — an all-encodable
  union now compiles; a union with an unencodable member stays None.
- `runtime_conformance::native_response_encoder_emits_declared_order`
  extended with a required `uni: Union<Integer, String>` property — the
  live HTTP response encodes the union member correctly inside the
  declared-order bytes; the mismatch twin still 500s.
- `cargo test -p q-schema-runtime` — 54 unit + 3 fuzz passed.
- `cargo test -p q-engine-quickjs` — 1 + 96 passed.
- `cargo test -p velqu-runtime` — 19 integration passed.
- `bun test` — 69 passed, 0 failed, 297 expect calls.
- `bun run typecheck` — clean. `cargo fmt --check` — clean.
- `cargo clippy --workspace --all-targets -- -D warnings` — clean.
- `scripts/validate-okf` — 176 links, 0 errors.
- `./scripts/verify` — all stages pass except the documented
  isolated-worktree `qRuntimeRelease`/`proofPack` manifest hash mismatch
  (known, pre-existing on every packet branch).

No performance claim; benchmark manifest preserved unchanged.

Commit: `872b59b`.
