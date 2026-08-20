---
task_id: M24-004-D
parent_task: M24-004
milestone: M24
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M24.md
commit_required: true
---

# M24-004-D — Materialize JS strings lazily

## Atomic goal

Materialize JS strings lazily.

## Parent intent

Avoid allocating parameter strings until validation or JavaScript access requires them.

## Dependencies

- `M24-004-C` — `tasks/01_m24_zero_copy_ingress/M24-004-C-validate-numeric-uuid-formats-directly-from-bytes-where-possible.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M24.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`
- `context/components/schema-codecs.md`

### Source files

- `AGENTS.md`
- `crates/q-http/src/lib.rs`
- `crates/q-bridge/src/lib.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-schema-runtime/src/lib.rs`
- `crates/q-schema-runtime/tests/fuzz_validator.rs`
- `crates/q-engine-quickjs/src/convert.rs`
- `packages/schema/src/index.ts`
- `conformance/schema/schema.conformance.test.ts`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Materialize JS strings lazily.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Parameterized routes preserve exact names and values.
- No owned parameter string on an unread path.
- Percent-decoding policy is explicit and tested.
- Invalid encodings fail consistently.

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

- Allocation test.
- Reference router parity.
- Encoding edge-case corpus.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m24-004-d: materialize js strings lazily
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record

- Status: **PASS**
- Deliverable: per-key lazy JS parameter strings. `q_engine::RequestMeta` now carries `param_specs: Vec<ParamSpec {name, start, end}>` — byte ranges against the stored request path — instead of owned `(name, value)` pairs; parameter VALUE strings do not exist until JavaScript reads them. The prelude builds `ctx.params` (and the policy `req.params`) as an object of per-key lazy getters: touching `ctx.params.k` calls the new `__velquReqParam(slot, gen, key)` native, which materializes exactly one value by slicing the stored path (`__velquReqParamNames` supplies the declared key list at 0 data cost). Whole-field access (`__velquReqRaw("params")`) now also builds from specs + path on demand. Counters charge the exact materialized value length, so laziness evidence stays precise. `serve.rs` builds specs from the borrowed interned names + capture ranges; no value string is allocated at admission.
- Changed files:
  - `crates/q-engine/src/lib.rs` (`ParamSpec`; `RequestMeta.param_specs`)
  - `crates/q-engine-quickjs/src/worker.rs` (`meta_path_slice`; `__velquReqParamNames`; `__velquReqParam` with exact byte accounting; `__velquReqRaw("params")` from specs)
  - `crates/q-engine-quickjs/src/prelude.rs` (`__velquMakeLazyParams` per-key getters for ctx and policy req)
  - `crates/q-runtime/src/serve.rs` (admission builds name+range specs; no owned param values)
  - `crates/q-engine-quickjs/tests/engine.rs` (`params.lazyb` handler; per-key laziness proof; spec-based fixture)
  - `docs/codex-spark-beta/tasks/01_m24_zero_copy_ingress/M24-004-D-materialize-js-strings-lazily.md`, `docs/codex-spark-beta/STATUS.md`, `docs/codex-spark-beta/indexes/TASK_INDEX.md`
- Tests: new `params_materialize_one_key_per_access` — three declared params, handler reads only `ctx.params.b`: response `"BB"`, counters show exactly 1 materialized field / 2 bytes, slot settles to 0. Existing proofs green: `lazy_ctx_touches_nothing` (0 host calls), `lazy_query_and_body_materialize_on_access`, `microtask_retains_valid_request_context` (fixture migrated to specs), `field_free_invocation_skips_request_store_slot`; engine suite 91/91.
- Verification: `cargo test -p q-engine-quickjs` PASS (1 + 91); `cargo test -p q-http` PASS (2 + 3); `cargo test -p q-bridge` PASS (9); `cargo test -p q-schema-runtime` PASS (unit + fuzz); `cargo test -p velqu-runtime` PASS (13); `cargo test -p q-router` PASS (15); `bun run typecheck` PASS; `bun test` PASS (35/0 after proof-pack + runtime-binary build in this fresh worktree — prior failures were binary-not-found environmental errors); `cargo fmt --check` PASS; `cargo clippy --workspace --all-targets -- -D warnings` PASS. Raw log: `/tmp/m24-004-d-bun.log`.
- Acceptance criteria proven: names and values preserved (per-key proof + full HTTP conformance over `/hello/:name` and policy routes); no owned parameter string on an unread path (specs-only admission; unread = zero value strings; counters prove per-access allocation); percent-decoding policy unchanged (raw path bytes sliced; M24-004-A corpus); invalid encodings fail consistently (slices stay on '/' char boundaries; malformed UTF-8 cannot enter — path is a Rust `&str`).
- Remaining risk / deferred by design: query/header lazy field IDs are M24-005/M24-006; response prototypes sharing is M24-008.
- Next dependency-ready task: M24-004-V (verify Capture path parameters as byte ranges).

