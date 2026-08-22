---
task_id: M25-010-B
parent_task: M25-010
milestone: M25
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M25.md
commit_required: true
---

# M25-010-B — Measure generated code/pack size

## Atomic goal

Measure generated code/pack size.

## Parent intent

Prove the selected strategies improve real payloads without inflating startup unacceptably.

## Dependencies

- `M25-010-A` — `tasks/02_m25_schema_codecs/M25-010-A-run-c2-plus-medium-large-json-workloads.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M25.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/schema-codecs.md`

### Source files

- `AGENTS.md`
- `crates/q-schema-runtime/src/lib.rs`
- `crates/q-engine-quickjs/src/convert.rs`
- `packages/schema/src/index.ts`
- `packages/compiler/src/emit.ts`
- `crates/q-pack/src/lib.rs`
- `crates/q-pack/tests/fuzz_pack.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `docs/specs/pack-format-v1.md`
- `crates/q-schema-runtime/tests/fuzz_validator.rs`
- `conformance/schema/schema.conformance.test.ts`
- `benchmarks/harness/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Measure generated code/pack size.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- C2 materially improves or limitation is documented.
- No unapproved cold-start regression.
- Reports match raw data.
- Route-specific strategy is inspectable.

## Targeted commands

```bash
cargo test -p q-pack
```
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

- Raw performance suite.
- Generated report.
- Decision matrix.
- [ ] Canonical Schema IR drives runtime, Treaty, OpenAPI, lock, and diff.
- [ ] Generated decoders/encoders are semantically equivalent and bounded.
- [ ] Fallbacks are explicit and measured.
- [ ] Response errors/problems are exact and redacted correctly.
- [ ] Performance evidence supports route-level strategy selection.
- C2 small JSON.
- 1KB/16KB/64KB dynamic payloads.
- Arrays 100/1,000.
- Request decode and response encode stage timings.
- No binary QPack encoding yet.
- No capability API expansion.
- No ORM.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m25-010-b: measure generated code pack size
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record (M25-010-B)

Status: **PASS**. Generated code/pack size measured with raw evidence:

- Raw evidence: `benchmarks/raw/sizes-m25-010-b/sizes.json` — byte sizes
  + sha256 for every proof-app dist artifact (total 108,731 bytes;
  app.qpack 61,582 bytes ≈ 6,842 bytes/route incl. schemas, router
  tables, bundle), the C2 generated codec module (benchmark-only), and
  the release binaries (velqu-runtime 5,145,976 bytes; raw-rust baseline
  509 KB-class for comparison), environment (rustc version, remap flags)
  recorded.
- Report: `docs/reports/m25-010-b-size.md` — full artifact table,
  per-route math, binary table, decision-matrix impact.
- Key facts (honest): the generated codec programs add ZERO per-route
  pack bytes (decoders/encoders compile at startup from the same schema
  IR the pack already carries) and ZERO binary growth per route (route
  count scales the pack, not the executable). The C2 generated module is
  benchmark-only (production compiles from IR — no checked-in generated
  sources). Sizes are toolchain-dependent: recorded, not normative.
- Root benchmark manifest untouched (new versioned raw directory).

### Tests and evidence

- `cargo test -p q-schema-runtime` — 58 + 4 + 5; `cargo test -p
  q-engine-quickjs` — 1 + 96; `cargo test -p velqu-runtime` — 24;
  `cargo test -p q-pack` — 41 + 2 — all passed.
- `bun test` — 81 passed, 0 failed, 481 expect calls.
- `bun run typecheck` — clean. `cargo fmt --check` — clean.
- `cargo clippy --workspace --all-targets -- -D warnings` — clean.
- `scripts/validate-okf` — 176 links, 0 errors.

Commit: `c669397`.
