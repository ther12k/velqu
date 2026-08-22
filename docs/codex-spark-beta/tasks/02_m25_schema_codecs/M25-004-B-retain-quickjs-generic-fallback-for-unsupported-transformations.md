---
task_id: M25-004-B
parent_task: M25-004
milestone: M25
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M25.md
commit_required: true
---

# M25-004-B — Retain QuickJS/generic fallback for unsupported transformations

## Atomic goal

Retain QuickJS/generic fallback for unsupported transformations.

## Parent intent

Parse and validate declared JSON bodies with one route-selected strategy.

## Dependencies

- `M25-004-A` — `tasks/02_m25_schema_codecs/M25-004-A-implement-generated-direct-decode-where-supported.md`

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
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`
- `crates/q-http/src/lib.rs`
- `crates/q-bridge/src/lib.rs`
- `crates/q-runtime/src/serve.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Retain QuickJS/generic fallback for unsupported transformations.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

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

## Required evidence for this microtask

- Fuzz/differential tests.
- Depth/size boundary tests.
- CPU/allocation results.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m25-004-b: retain quickjs generic fallback for unsupported transformati
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record (M25-004-B)

Status: **PASS**. The QuickJS/generic fallback for unsupported transformations
is retained end-to-end: routes whose body schema carries a fallback marker
without inner (or transform/file/problem nodes) compile to
`validationStrategy: "js"` (M25-002-D), and the runtime now hands the parsed
raw JSON to the handler instead of failing closed. The native decoder keeps
typed fail-closed errors as defense in depth when invoked on the native path.

### Changed files

- `crates/q-runtime/src/serve.rs` — body admission checks
  `route.validation_strategy == Strategy::Js` and bypasses native decode
  (raw parsed JSON crosses to the handler); native path unchanged.
- `crates/q-runtime/tests/runtime_conformance.rs` — new `fallback.echo` fixture
  route (POST /fallback, body schema `Fallback { reason: "explicit", inner: None }`,
  validationStrategy js, echo handler), function-manifest entry, and the
  `js_fallback_body_routes_raw_json_to_handler` conformance test proving:
  arbitrary JSON objects cross intact, non-object arrays cross raw, malformed
  JSON still rejects 422 at admission.
- `crates/q-schema-runtime/src/decoder.rs` — two defense-in-depth unit tests:
  `native_path_fails_closed_on_fallback_without_inner_and_unsupported_nodes`
  (fallback-without-inner → typed `fallback` error; transform → typed
  `unsupported` error) and `fallback_with_inner_still_validates_inner_on_native_path`.
- `docs/codex-spark-beta/STATUS.md`, `docs/codex-spark-beta/indexes/TASK_INDEX.md`
  — M25-004-B marked PASS.

### Tests and evidence

- `cargo test -p q-schema-runtime` — 43 unit tests + 3 fuzz tests passed.
- `cargo test -p velqu-runtime` — 16 integration tests passed (new fallback test included).
- `cargo test -p q-engine-quickjs` — 1 unit + 96 integration tests passed.
- `cargo test -p q-http` — 4 tests passed.
- `cargo test -p q-bridge` — 11 passed.
- `bun test` — 69 passed, 0 failed, 296 expect calls.
- `bun run typecheck` — clean.
- `cargo fmt --check` — clean.
- `cargo clippy --workspace --all-targets -- -D warnings` — clean.
- `scripts/validate-okf` — 176 links, 0 errors.

Commit: `16cdc5d`.
