---
task_id: M24-004-C
parent_task: M24-004
milestone: M24
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M24.md
commit_required: true
---

# M24-004-C — Validate numeric/UUID formats directly from bytes where possible

## Atomic goal

Validate numeric/UUID formats directly from bytes where possible.

## Parent intent

Avoid allocating parameter strings until validation or JavaScript access requires them.

## Dependencies

- `M24-004-B` — `tasks/01_m24_zero_copy_ingress/M24-004-B-bind-route-specific-parameter-names-after-routeid-selection.md`

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
5. Implement exactly this deliverable: Validate numeric/UUID formats directly from bytes where possible.
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
m24-004-c: validate numeric uuid formats directly from bytes where poss
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record

- Status: **PASS**
- Deliverable: numeric/UUID formats validate directly from the captured path bytes. `q_schema_runtime::validate_params_bytes(ir, &[(&str, &[u8])])` adds a byte-level format gate: Integer and Number schemas parse from the UTF-8 bytes (`str::parse`, no intermediate String) and UUID strings check syntax on raw bytes (`is_uuid_bytes`), so an INVALID value rejects with the full validator's exact error identity before any parameter string is allocated. Values that pass continue through `validate_params`, whose semantics (bounds, length, pattern, email, defaults, coercion) remain the single source of truth — the owned strings it builds are the pre-validated params the engine consumes. `serve.rs` now validates FIRST from borrowed names (`Router::param_names`) zipped with range slices of the path, and materializes owned parameter strings only after validation succeeded (or none is declared) AND the engine/policy reads them.
- Changed files:
  - `crates/q-schema-runtime/src/lib.rs` (`validate_params_bytes`, `byte_format_error`, `is_uuid_bytes`, rejection + parity tests)
  - `crates/q-runtime/src/serve.rs` (byte-first validation order; materialization gated to post-validation needs)
  - `docs/codex-spark-beta/tasks/01_m24_zero_copy_ingress/M24-004-C-validate-numeric-uuid-formats-directly-from-bytes-where-possible.md`, `docs/codex-spark-beta/STATUS.md`, `docs/codex-spark-beta/indexes/TASK_INDEX.md`
- Tests: new `validate_params_bytes_rejects_invalid_formats_from_bytes` (non-numeric/float integer text, bad UUID, invalid UTF-8 bytes — reject without panic, matching code/path) and `validate_params_bytes_parity_with_owned_validator` (valid path returns EXACTLY the owned validator's Value). All M24-004-A/B proofs and end-to-end suites remain green.
- Verification: `cargo test -p q-schema-runtime` PASS (9 unit + 2 fuzz); `cargo test -p q-router` PASS (15); `cargo test -p q-engine-quickjs` PASS (1 + 90); `cargo test -p q-http` PASS (2 + 3); `cargo test -p q-bridge` PASS (9); `cargo test -p velqu-runtime` PASS (13); `bun run typecheck` PASS; `bun test` PASS (35/0 after proof-pack build); `cargo fmt --check` PASS; `cargo clippy --workspace --all-targets -- -D warnings` PASS. Raw log: `/tmp/m24-004-c-bun.log`.
- Acceptance criteria proven: names and values preserved on valid paths (parity test + full conformance); no owned parameter string on an unread path (invalid values allocate zero parameter strings; valid values allocate only the engine-consumed result); percent-decoding policy unchanged (raw bytes; M24-004-A corpus); invalid encodings fail consistently (invalid UTF-8 integer bytes reject with the typed `type` error).
- Remaining risk / deferred by design: lazy JS-side string materialization through the bridge is M24-004-D; email/pattern formats intentionally use the full validator (only numeric/uuid are named by the packet).
- Next dependency-ready task: M24-004-D (materialize JS strings lazily).

