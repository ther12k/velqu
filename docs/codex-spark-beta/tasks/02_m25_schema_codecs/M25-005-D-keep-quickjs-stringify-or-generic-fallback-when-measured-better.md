---
task_id: M25-005-D
parent_task: M25-005
milestone: M25
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M25.md
commit_required: true
---

# M25-005-D — Keep QuickJS stringify or generic fallback when measured better

## Atomic goal

Keep QuickJS stringify or generic fallback when measured better.

## Parent intent

Fuse output validation and serialization for stable response contracts.

## Dependencies

- `M25-005-C` — `tasks/02_m25_schema_codecs/M25-005-C-handle-optional-null-union-fields.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M25.md`
- `context/components/engine-scheduler.md`
- `context/components/schema-codecs.md`
- `context/components/devex-beta.md`

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
- `crates/q-schema-runtime/tests/fuzz_validator.rs`
- `conformance/schema/schema.conformance.test.ts`
- `packages/treaty/src/index.ts`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Keep QuickJS stringify or generic fallback when measured better.
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
cargo test -p q-schema-runtime
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
m25-005-d: keep quickjs stringify or generic fallback when measured bet
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record (M25-005-D)

Status: **PASS**. The QuickJS stringify fallback and the generic reference
path stay retained and correct next to the generated encoder:

- The selection mechanism already exists end-to-end from M25-002-D:
  `s.fallback("measured", ...)` on a response schema selects the js
  strategy with the measured-cost estimate recorded in
  `build-report.json` (compiler conformance covers it: "explicit fallback
  nodes select js strategy and record estimated overhead" asserts the
  `measured` reason at `response.200`). No default flipped — current
  M25-002 evidence selects native for every representable shape, and no
  new measurement justifies changing it.
- The generic reference validate-then-serialize path remains for native
  routes whose response schema the direct encoder cannot represent
  (M25-005-A compile-to-None behavior, unchanged).
- New runtime evidence `quickjs_stringify_fallback_stays_json_equivalent_
  to_encoder`: twin routes share one declared response schema; the native
  twin encodes through the generated program (declared property order
  bytes), the js twin stringifies in the engine (handler insertion order
  bytes, host validation skipped per the disclosed fallback). Both return
  200 and the bodies are JSON-equal — the retained fallback stays
  selectable (e.g. via the `measured` marker) with zero correctness
  drift. QPack::verify enforces plan/declared strategy agreement (the js
  twin carries `ResponseDecl.strategy: Js`).

### Tests and evidence

- `runtime_conformance::quickjs_stringify_fallback_stays_json_equivalent_
  to_encoder` — twin-route retention proof (see above).
- Golden corpus / response mismatch / mapping deadline evidence:
  unchanged from M25-005-A/B/C and still green (`cargo test -p
  q-schema-runtime` — 54 unit + 3 fuzz).
- `cargo test -p q-engine-quickjs` — 1 + 96 passed.
- `cargo test -p velqu-runtime` — 20 integration passed (new twin test).
- `bun test` — 69 passed, 0 failed, 297 expect calls (compiler strategy
  tests incl. the `measured` fallback assertions).
- `bun run typecheck` — clean. `cargo fmt --check` — clean.
- `cargo clippy --workspace --all-targets -- -D warnings` — clean.
- `scripts/validate-okf` — 176 links, 0 errors.
- `./scripts/verify` — all stages pass except the documented
  isolated-worktree `qRuntimeRelease`/`proofPack` manifest hash mismatch
  (known, pre-existing on every packet branch).

No performance claim; benchmark manifest preserved unchanged.

Commit: `f4efd70`.
