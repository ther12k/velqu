---
task_id: M25-003-Z
parent_task: M25-003
milestone: M25
priority: P0
mode: EVIDENCE
status: PASS
context_card: context/milestones/M25.md
commit_required: true
---

# M25-003-Z — Package evidence for Generate params/query/header decoders

## Atomic goal

Create source-backed evidence and handoff for parent task M25-003; update status only if verification passed.

## Parent intent

Fuse field extraction, coercion, and validation for non-body inputs.

## Dependencies

- `M25-003-V` — `tasks/02_m25_schema_codecs/M25-003-V-verify-generate-params-query-header-decoders.md`

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
- `crates/q-runtime/src/main.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

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

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m25-003-z: package evidence for generate params query header decoders
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Evidence package

- Status: **PASS**. Parent verification M25-003-V merged in PR #730 at commit
  `f0f70ef3b57db3c9bcb53b54041f8c0ba7685708`; issue #136 is closed. The evidence
  package is based on clean parent HEAD `821de3e` before this commit.
- Parent acceptance matrix: `M25-003-V` maps all four guardrails to source and
  named tests:
  1. Exact declared envelopes: `crates/q-schema-runtime/src/decoder.rs`
     (`FieldErrorCode`, `FieldError::typed`), `crates/q-runtime/src/problems.rs`
     (`problems::body("validation", ...)`), `crates/q-runtime/src/serve.rs`.
  2. No duplicate parse pass: `DecoderProgram::decode_params_ranges`,
     `DecoderProgram::decode_headers`, `DecoderProgram::decode_query_pairs`.
  3. Treaty and OpenAPI types agree: Schema IR v2 builders in `@velqu/schema`,
     `packages/compiler/src/emit.ts`, `packages/treaty/`.
  4. Decoder programs bounded and fuzzable: `fuzz_validator.rs` (20,000 fuzz runs),
     `decoder.rs` unit tests.
- Source-backed implementation records:
  - `M25-003-A` (PR #726, #132 closed): direct decoder programs (`DecoderProgram`,
    `DecoderTable`) keyed by SchemaId in `q-schema-runtime` and `q-runtime`.
  - `M25-003-B` (PR #727, #133 closed): byte-range slicing without String allocations,
    case-insensitive header decoding.
  - `M25-003-C` (PR #728, #134 closed): typed RFC 9457 problem codes (`FieldErrorCode`).
  - `M25-003-D` (PR #729, #135 closed): scalar, array, nullable, optional coercion
    preservation and differential checks.
- Exact verification: `cargo test -p q-schema-runtime` (38 unit + 3 fuzz pass);
  `cargo test -p velqu-runtime` (15 pass); `cargo test -p q-engine-quickjs`
  (1 + 96 pass); `cargo test -p q-http` (4 pass); `cargo test -p q-bridge` (11 pass);
  `bun test` (69 passed, 0 failed, 296 expect calls); `bun run typecheck` clean;
  `cargo fmt --check` clean; `cargo clippy --workspace --all-targets -- -D warnings`
  clean; `scripts/validate-okf` (176 links, 0 errors).
- Full `./scripts/verify` completed all Rust, typecheck, proof-build, and TypeScript
  stages. Its final benchmark check reports only the known isolated-worktree hash
  mismatches for `qRuntimeRelease` and `proofPack` against `benchmarks/manifest.json`.
  The canonical root manifest and historical raw benchmarks were preserved.
- Status bookkeeping: `docs/beta/04_TASK_LEDGER.md` marks M25-003 PASS; the
  beta checklist and task index mark this Z packet PASS. The generated Spark
  queues now expose M25-004-A (#138) as the next dependency-ready packet.
- Remaining scope: `M25-GATE` remains TODO and future M25 packets (M25-004+)
  remain TODO until implemented and evidenced.

Commit: `7bba2af`.
