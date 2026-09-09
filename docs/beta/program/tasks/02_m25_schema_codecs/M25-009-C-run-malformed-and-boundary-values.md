---
task_id: M25-009-C
parent_task: M25-009
milestone: M25
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M25.md
commit_required: true
---

# M25-009-C — Run malformed and boundary values

## Atomic goal

Run malformed and boundary values.

## Parent intent

Prove generated codecs match reference semantics and remain memory-safe.

## Dependencies

- `M25-009-B` — `tasks/02_m25_schema_codecs/M25-009-B-compare-generated-output-with-standards-reference-json-behavior.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M25.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`

### Source files

- `AGENTS.md`
- `crates/q-schema-runtime/src/lib.rs`
- `crates/q-engine-quickjs/src/convert.rs`
- `packages/schema/src/index.ts`
- `packages/compiler/src/emit.ts`
- `crates/q-schema-runtime/tests/fuzz_validator.rs`
- `conformance/schema/schema.conformance.test.ts`
- `conformance/security/security.conformance.test.ts`
- `crates/q-pack/tests/fuzz_pack.rs`
- `crates/q-http/tests/fuzz_parsers.rs`
- `.github/workflows/verify.yml`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Run malformed and boundary values.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- No panic, hang, unbounded output, or semantic mismatch.
- All fuzz findings are triaged.
- Coverage targets are recorded.
- Generated code is deterministic.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p q-http
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

- Fuzz summaries.
- Regression corpus.
- Differential report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m25-009-c: run malformed and boundary values
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record (M25-009-C)

Status: **PASS**. The malformed/boundary corpus ran — and found and fixed
a second real divergence:

- **Fuzz/corpus finding (triaged + fixed)**: on a union total miss, the
  direct DECODER leaked the last member's internal error (e.g.
  `type: expected string`) while the reference validator always reports
  the canonical `union` problem. Fixed in `decoder.rs` at both union
  sites (`decode_value_depth`, `decode_str_depth`): the decoder now
  emits the canonical typed `union` error on a total miss — typed-code
  parity with the reference restored. No existing test relied on the
  leaked member error (all suites green unchanged).
- **Corpus** (`malformed_and_boundary_corpus` in
  `codec_standards_corpus.rs`): 31 malformed entries — wrong types at
  every declared position (string/float/bool/array/null for integers,
  string/bool for numbers, number/array for strings), u64::MAX beyond
  i64, malformed emails (no local part) and UUIDs (truncated), list
  wrong-item-type, non-member enums, union misses (bool and negative
  int), whole-value non-objects/scalars, bound violations just past
  every line (ints, numerics, strings, arrays, missing required,
  unknown keys) — each asserting decoder/reference/encoder TYPED-CODE
  parity and no panic. Plus 11 boundary entries accepted everywhere
  with byte/output parity: exact min/max ints and numerics, exact
  string/list bounds, single-item lists, exact enum members, union
  first-member boundary (0), valid email/UUID.

### Tests and evidence

- `codec_standards_corpus` — 4 passed (3 prior + the new corpus).
- `cargo test -p q-schema-runtime` — 58 unit + 4 fuzz + 4 standards —
  all passed.
- `cargo test -p q-engine-quickjs` — 1 + 96; `cargo test -p q-pack` —
  41 + 2; `cargo test -p velqu-runtime` — 24 — all passed.
- `bun test` — 81 passed, 0 failed, 481 expect calls.
- `bun run typecheck` — clean. `cargo fmt --check` — clean.
- `cargo clippy --workspace --all-targets -- -D warnings` — clean.
- `scripts/validate-okf` — 176 links, 0 errors.
- `./scripts/verify` — all stages pass except the documented
  isolated-worktree `qRuntimeRelease`/`proofPack` manifest hash mismatch
  (known, pre-existing on every packet branch).

Commit: `2aea65f`.
