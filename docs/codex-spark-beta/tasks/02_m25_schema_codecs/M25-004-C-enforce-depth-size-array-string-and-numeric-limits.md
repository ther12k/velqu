---
task_id: M25-004-C
parent_task: M25-004
milestone: M25
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M25.md
commit_required: true
---

# M25-004-C — Enforce depth, size, array, string, and numeric limits

## Atomic goal

Enforce depth, size, array, string, and numeric limits.

## Parent intent

Parse and validate declared JSON bodies with one route-selected strategy.

## Dependencies

- `M25-004-B` — `tasks/02_m25_schema_codecs/M25-004-B-retain-quickjs-generic-fallback-for-unsupported-transformations.md`

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

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Enforce depth, size, array, string, and numeric limits.
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
m25-004-c: enforce depth size array string and numeric limits
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record (M25-004-C)

Status: **PASS**. Depth, size, array, string, and numeric limits are enforced
boundedly on every decode path:

- **Depth**: `MAX_VALIDATE_DEPTH = 64` guards both the reference validator
  (`validate_node`) and the direct decoder programs (`decode_value_depth`,
  `decode_str_depth`) with a new typed `depth` field problem; both paths agree
  (differential parity test). Parse-level nesting is separately capped by
  serde_json's recursion limit (proven by the 200-deep HTTP test).
- **Size**: body bytes stay bounded by route `limit_bytes` admission (413,
  pre-existing) — unchanged.
- **Array**: `minItems`/`maxItems` boundary tests at exact bound and off-by-one.
- **String**: `minLength`/`maxLength` boundary tests at exact bound and off-by-one.
- **Numeric**: `minimum`/`maximum` boundary tests; non-finite rejects typed.

### Changed files

- `crates/q-schema-runtime/src/lib.rs` — `MAX_VALIDATE_DEPTH` constant,
  `FieldErrorCode::Depth` variant (closed vocabulary grows by one documented
  code), depth threading through `validate_node`.
- `crates/q-schema-runtime/src/decoder.rs` — depth-threaded
  `decode_value_depth`/`decode_str_depth` (public signatures unchanged);
  tests `decode_depth_bounded_with_typed_depth_problem` (over-limit typed
  error with reference-validator parity, within-limit output parity,
  exact-bound decode) and `scalar_limits_enforced_at_exact_boundaries`
  (string/array/numeric boundaries and non-finite rejection).
- `crates/q-runtime/tests/runtime_conformance.rs` —
  `deeply_nested_body_fails_boundedly`: 200-deep nested array POST rejects
  422 with the exact declared problem envelope on both native and js-fallback
  body routes.
- `docs/specs/unsupported-transformations.md` — §2 table documents the
  `depth` code and its bound.
- `conformance/schema/schema.conformance.test.ts` — spec-code test asserts
  `depth` is documented.
- `docs/codex-spark-beta/STATUS.md`, `docs/codex-spark-beta/indexes/TASK_INDEX.md`
  — M25-004-C marked PASS.

### Tests and evidence

- `cargo test -p q-schema-runtime` — 45 unit tests + 3 fuzz tests passed.
- `cargo test -p velqu-runtime` — 17 integration tests passed (new deep-nesting test included).
- `cargo test -p q-engine-quickjs` — 1 unit + 96 integration tests passed.
- `cargo test -p q-http` — 4 tests passed.
- `cargo test -p q-bridge` — 11 passed.
- `bun test` — 69 passed, 0 failed, 297 expect calls.
- `bun run typecheck` — clean.
- `cargo fmt --check` — clean.
- `cargo clippy --workspace --all-targets -- -D warnings` — clean.
- `scripts/validate-okf` — 176 links, 0 errors.

Commit: `6a91b77`.
