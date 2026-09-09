---
task_id: M24-004-Z
parent_task: M24-004
milestone: M24
priority: P1
mode: EVIDENCE
status: PASS
context_card: context/milestones/M24.md
commit_required: true
---

# M24-004-Z — Package evidence for Capture path parameters as byte ranges

## Atomic goal

Create source-backed evidence and handoff for parent task M24-004; update status only if verification passed.

## Parent intent

Avoid allocating parameter strings until validation or JavaScript access requires them.

## Dependencies

- `M24-004-V` — `tasks/01_m24_zero_copy_ingress/M24-004-V-verify-capture-path-parameters-as-byte-ranges.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M24.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`

### Source files

- `AGENTS.md`
- `crates/q-http/src/lib.rs`
- `crates/q-bridge/src/lib.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-router/src/lib.rs`
- `crates/q-pack/src/lib.rs`
- `packages/compiler/src/emit.ts`
- `conformance/routing/routing.conformance.test.ts`
- `benchmarks/harness/`
- `benchmarks/manifest.json`
- `benchmarks/raw/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

## Parent acceptance guardrails

- Parameterized routes preserve exact names and values.
- No owned parameter string on an unread path.
- Percent-decoding policy is explicit and tested.
- Invalid encodings fail consistently.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
cargo test -p q-router
```
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

## Required evidence for this microtask

- Allocation test.
- Reference router parity.
- Encoding edge-case corpus.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m24-004-z: package evidence for capture path parameters as byte ranges
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record

- Status: **PASS**
- Deliverable: source-backed evidence package for parent task M24-004 (capture path parameters as byte ranges). Implementation commits: `f6cb33f` (A, PR #654 — `MatchResult::param_ranges` byte ranges, offset-tracking `resolve`, `materialize_params` as the single allocation path, serve-side lazy materialization), `e4e488b` (B, PR #655 — interned `param_name_table` + `param_name_ids`, RouteId-bound borrowed `param_names`), `264fa1d` (C, PR #656 — `validate_params_bytes` byte-level numeric/UUID gate, zero allocation on invalid values, parity with the owned validator), `328696e` (D, PR #657 — `RequestMeta.param_specs` name+range storage, `__velquReqParam`/`__velquReqParamNames` per-key lazy natives, prelude per-key getters, exact byte accounting). Verification commit: `a4f4dc7` (V, PR #658 — guardrail mapping plus the honest correction that D's q-bridge/clippy claims were stale; the compile-stale fixture and clippy `manual_inspect` were fixed in V with no behavior change).
- Exact changed files (implementation scope): `crates/q-router/src/lib.rs`, `crates/q-engine/src/lib.rs` (`ParamSpec`, `RequestMeta`), `crates/q-engine-quickjs/src/worker.rs`, `crates/q-engine-quickjs/src/prelude.rs`, `crates/q-schema-runtime/src/lib.rs`, `crates/q-runtime/src/serve.rs`, `crates/q-bridge/src/lib.rs` (fixture), plus packet/status/index documents.
- Evidence index (key tests): q-router — `capture_ranges_defer_string_allocation_and_match_reference_values`, `capture_ranges_encoding_corpus_is_raw_and_panic_free`, `param_names_bind_after_routeid_selection_and_are_borrowed`, generated reference-parity property suite; q-schema-runtime — `validate_params_bytes_rejects_invalid_formats_from_bytes`, `validate_params_bytes_parity_with_owned_validator`; engine — `params_materialize_one_key_per_access`, `lazy_ctx_touches_nothing`, `lazy_query_and_body_materialize_on_access`, `field_free_invocation_skips_request_store_slot`; runtime conformance 13/13 and Bun HTTP conformance 35/35.
- Exact command results (V run, this tree): all targeted suites PASS — engine 1+91, q-http 2+3, q-bridge 9, q-schema-runtime unit+fuzz, velqu-runtime 13, q-router 15, `bun run typecheck`, `bun test` 35/0, `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`. Raw logs: `/tmp/m24-004-v-rust.log`, `/tmp/m24-004-v-verify.log`, `/tmp/m24-004-v-bun.log`. Scoped limitation unchanged: `./scripts/verify` fails only `validate-benchmark-evidence` (fresh-worktree artifact ordering; worktree `qRuntimeRelease` hash mismatch vs canonical manifest); no benchmark manifest or performance claim changed.
- Evidence boundary: BridgeCounters prove laziness/allocation behavior only; aggregate ingress metrics and instrumentation-overhead benchmarks remain M24-009 deliverables.
- Next dependency-ready tasks: M24-005-A (compile header-name IDs into RoutePlan) and M24-006-A (compile query/cookie field IDs, also awaits M24-004-Z — this packet).

