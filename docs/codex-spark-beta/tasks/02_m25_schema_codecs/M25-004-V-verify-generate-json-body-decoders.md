---
task_id: M25-004-V
parent_task: M25-004
milestone: M25
priority: P0
mode: VERIFY
status: PASS
context_card: context/milestones/M25.md
commit_required: true
---

# M25-004-V — Verify Generate JSON body decoders

## Atomic goal

Prove every acceptance criterion for parent task M25-004 without broadening scope.

## Parent intent

Parse and validate declared JSON bodies with one route-selected strategy.

## Dependencies

- `M25-004-A` — `tasks/02_m25_schema_codecs/M25-004-A-implement-generated-direct-decode-where-supported.md`
- `M25-004-B` — `tasks/02_m25_schema_codecs/M25-004-B-retain-quickjs-generic-fallback-for-unsupported-transformations.md`
- `M25-004-C` — `tasks/02_m25_schema_codecs/M25-004-C-enforce-depth-size-array-string-and-numeric-limits.md`
- `M25-004-D` — `tasks/02_m25_schema_codecs/M25-004-D-propagate-cancellation-and-request-deadlines.md`

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

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

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

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m25-004-v: verify generate json body decoders
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record (M25-004-V)

Status: **PASS**. Every parent M25-004 acceptance guardrail maps to source
and at least one passing test; all verification commands were run fresh on
this branch (no code changes — verification closure only).

### Guardrail → source → evidence

1. **One successful decode representation crosses to JS.**
   - Source: `crates/q-runtime/src/serve.rs` body stage — native strategy
     decodes via `DecoderTable::decode_body_value` and crosses one
     structured value; js strategy crosses the raw parsed JSON once
     (M25-004-B).
   - `runtime_conformance::full_runtime_conformance` — POST /users 201 with
     the decoded body reaching the handler; schema-invalid email rejects 422
     with `errors[0].path == "email"` (decoder-produced typed field error).
   - `decoder::decoder_program_decodes_and_validates_json_body_value` —
     unit proof incl. `assert_eq!(res, ref_valid)` parity against the
     reference validator.
   - `runtime_conformance::js_fallback_body_routes_raw_json_to_handler` —
     fallback raw JSON crosses verbatim; malformed JSON still rejects 422 at
     admission (parse precedes strategy).
2. **Oversize/deep inputs fail boundedly.**
   - `runtime_conformance::content_length_over_limit_rejects_before_body_poll`
     and `body_and_header_limits_reject_oversize`; bun security conformance
     "body limit rejects payload > 65536 bytes with 413".
   - `runtime_conformance::deeply_nested_body_fails_boundedly` — 200-deep
     body rejects 422 on both native and js-fallback routes.
   - `decoder::decode_depth_bounded_with_typed_depth_problem` —
     `MAX_VALIDATE_DEPTH = 64` typed `depth` problem with over/within
     parity between decoder and reference validator.
   - `decoder::scalar_limits_enforced_at_exact_boundaries` — exact-bound and
     off-by-one string/array/numeric limits; non-finite rejects typed.
   - `runtime_conformance::body_read_deadline_cancels_stalled_transfer` —
     a stalled body stream settles 504 at the route deadline (M25-004-D);
     time is bounded, not just bytes and depth.
3. **No semantic drift from schema.**
   - `decoder::decoder_program_matches_reference_validator_on_mixed_corpus`
     — differential parity on a mixed corpus.
   - `decoder::native_path_fails_closed_on_fallback_without_inner_and_unsupported_nodes`
     and `fallback_with_inner_still_validates_inner_on_native_path` — the
     native path never silently drifts: unrepresentable shapes fail closed.
   - `tests/fuzz_validator.rs` — 3 fuzz tests (differential).
   - `conformance/schema/schema.conformance.test.ts` — TypeScript-side
     contract incl. the closed error-code vocabulary with `depth`.
4. **Fallback is explicit in build report.**
   - `conformance/compiler/compiler.test.ts` — "explicit fallback nodes
     select js strategy and record estimated overhead" and "strategy
     decisions are deterministic across repeated builds".
   - `packages/compiler/src/strategy.ts`/`emit.ts` emit per-route
     `validationStrategy` + fallback cost estimates into
     `build-report.json`; `velqu inspect fallbacks` surfaces them (M25-002-D).

### Command results (this branch, fresh worktree)

- `cargo test -p q-engine-quickjs` — 1 + 96 passed.
- `cargo test -p q-http` — 4 + 6 + 1 passed.
- `cargo test -p q-bridge` — 11 passed.
- `cargo test -p q-schema-runtime` — 45 unit + 3 fuzz passed.
- `cargo test -p velqu-runtime` — 18 integration passed.
- `bun test` — 69 passed, 0 failed, 297 expect calls.
- `bun run typecheck` — clean. `cargo fmt --check` — clean.
- `cargo clippy --workspace --all-targets -- -D warnings` — clean.
- `scripts/validate-okf` — 176 links, 0 errors.
- `./scripts/verify` — all stages pass except the documented
  isolated-worktree `qRuntimeRelease`/`proofPack` manifest hash mismatch
  (release binary embeds checkout-absolute OUT_DIR paths; identical on every
  packet branch this session).

CPU/allocation evidence is not re-measured here: the decode path is the
M25-002-C instrumented path (raw samples under `benchmarks/raw/codec-c/`);
this packet changes no decode semantics and makes no new performance claim.

Changed files: this record, `docs/codex-spark-beta/STATUS.md`,
`docs/codex-spark-beta/indexes/TASK_INDEX.md` (verification closure only).

Commit: `46cb1fd`.
