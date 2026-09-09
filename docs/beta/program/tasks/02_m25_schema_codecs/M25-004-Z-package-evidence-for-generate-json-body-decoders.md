---
task_id: M25-004-Z
parent_task: M25-004
milestone: M25
priority: P0
mode: EVIDENCE
status: PASS
context_card: context/milestones/M25.md
commit_required: true
---

# M25-004-Z — Package evidence for Generate JSON body decoders

## Atomic goal

Create source-backed evidence and handoff for parent task M25-004; update status only if verification passed.

## Parent intent

Parse and validate declared JSON bodies with one route-selected strategy.

## Dependencies

- `M25-004-V` — `tasks/02_m25_schema_codecs/M25-004-V-verify-generate-json-body-decoders.md`

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
- `benchmarks/harness/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

## Parent acceptance guardrails

- One successful decode representation crosses to JS.
- Oversize/deep inputs fail boundedly.
- No semantic drift from schema.
- Fallback is explicit in build report.

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

- Fuzz/differential tests.
- Depth/size boundary tests.
- CPU/allocation results.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m25-004-z: package evidence for generate json body decoders
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Evidence package

- Status: **PASS**. Parent verification M25-004-V merged in PR #736 at commit
  `5dd765a4f5e57364a3b9d72dda4405849c325098`; issue #142 is closed. The evidence
  package is based on clean parent HEAD `d936b22` before this commit.
- Parent acceptance matrix: `M25-004-V` maps all four guardrails to source and
  named tests:
  1. One successful decode representation crosses to JS:
     `crates/q-runtime/src/serve.rs` body stage (`DecoderTable::decode_body_value`
     native / raw-JSON js strategy); `full_runtime_conformance` POST /users 201 +
     typed 422 `errors[0].path == "email"`;
     `decoder_program_decodes_and_validates_json_body_value` (reference parity);
     `js_fallback_body_routes_raw_json_to_handler`.
  2. Oversize/deep inputs fail boundedly: 413 admission bounds
     (`content_length_over_limit_rejects_before_body_poll`,
     `body_and_header_limits_reject_oversize`, bun security 413 test); depth
     (`deeply_nested_body_fails_boundedly`,
     `decode_depth_bounded_with_typed_depth_problem`); scalar exact bounds
     (`scalar_limits_enforced_at_exact_boundaries`); time
     (`body_read_deadline_cancels_stalled_transfer`).
  3. No semantic drift from schema:
     `decoder_program_matches_reference_validator_on_mixed_corpus` (differential),
     `native_path_fails_closed_on_fallback_without_inner_and_unsupported_nodes`,
     `fallback_with_inner_still_validates_inner_on_native_path`,
     `fuzz_validator.rs` (3 fuzz tests), `conformance/schema/schema.conformance.test.ts`.
  4. Fallback is explicit in build report: `packages/compiler/src/strategy.ts`
     per-route `validationStrategy` + fallback costs in `build-report.json`;
     compiler tests "explicit fallback nodes select js strategy and record
     estimated overhead" / "strategy decisions are deterministic across repeated
     builds"; `velqu inspect fallbacks`.
- Source-backed implementation records:
  - `M25-004-A` (PR #732, #138 closed): strict JSON body direct decode
    (`DecoderProgram::decode_body_value` — unknown-key and missing-required
    rejection, optional defaults inserted).
  - `M25-004-B` (PR #733, #139 closed): retained QuickJS/generic fallback for
    unsupported transformations (validationStrategy "js" routes the raw parsed
    JSON across; the native path fails closed as defense in depth).
  - `M25-004-C` (PR #734, #140 closed): depth, size, array, string, and numeric
    limits (`MAX_VALIDATE_DEPTH = 64` typed `depth` problem on both validation
    paths; exact-boundary scalar tests; 200-deep HTTP bound proof).
  - `M25-004-D` (PR #735, #141 closed): cancellation and request deadlines
    (deadline anchored at route match, `timeout_at`-bounded body read with 504
    `timeout` settlement, anchored deadline propagated to the worker).
- CPU/allocation evidence: the decode path is the M25-002-C instrumented path
  (feature-gated `bench-instrumentation`, raw samples retained under
  `benchmarks/raw/codec-c/`); this packet adds no new measurement and makes no
  new performance claim. The benchmark manifest is preserved unchanged.
- Exact verification (fresh on this branch): `cargo test -p q-schema-runtime`
  (45 unit + 3 fuzz pass); `cargo test -p velqu-runtime` (18 pass);
  `cargo test -p q-engine-quickjs` (1 + 96 pass); `cargo test -p q-http`
  (4 + 6 + 1 pass); `cargo test -p q-bridge` (11 pass); `bun test` (69 passed,
  0 failed, 297 expect calls); `bun run typecheck` clean; `cargo fmt --check`
  clean; `cargo clippy --workspace --all-targets -- -D warnings` clean;
  `scripts/validate-okf` (176 links, 0 errors).
- Full `./scripts/verify` completed all Rust, typecheck, proof-build, and
  TypeScript stages. Its final benchmark check reports only the known
  isolated-worktree hash mismatches for `qRuntimeRelease` and `proofPack`
  against `benchmarks/manifest.json` (release binary embeds checkout-absolute
  OUT_DIR paths; identical on every packet branch this session). The canonical
  root manifest and historical raw benchmarks were preserved.
- Status bookkeeping: `docs/beta/04_TASK_LEDGER.md` marks M25-004 PASS; the
  beta checklist and task index mark this Z packet PASS. The generated Spark
  queues now expose M25-005-A (#144) as the next dependency-ready packet.
- Remaining scope: `M25-005`–`M25-010` and `M25-GATE` remain TODO until
  implemented and evidenced.

Commit: `4ef0ec4`.
