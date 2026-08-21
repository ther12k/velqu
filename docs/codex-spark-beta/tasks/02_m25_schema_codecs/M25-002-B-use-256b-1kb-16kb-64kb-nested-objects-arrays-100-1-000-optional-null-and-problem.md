---
task_id: M25-002-B
parent_task: M25-002
milestone: M25
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M25.md
commit_required: true
---

# M25-002-B — Use 256B, 1KB, 16KB, 64KB, nested objects, arrays 100/1,000, optional/null, and problems

## Atomic goal

Use 256B, 1KB, 16KB, 64KB, nested objects, arrays 100/1,000, optional/null, and problems.

## Parent intent

Measure QuickJS and native strategies across realistic payload shapes.

## Dependencies

- `M25-002-A` — `tasks/02_m25_schema_codecs/M25-002-A-compare-quickjs-parse-stringify-generic-rust-conversion-and-generated-schema-awa.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M25.md`
- `context/components/engine-scheduler.md`
- `context/components/schema-codecs.md`
- `context/components/evidence.md`

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
- `benchmarks/harness/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Use 256B, 1KB, 16KB, 64KB, nested objects, arrays 100/1,000, optional/null, and problems.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Raw and generated results are committed.
- No single strategy is forced globally.
- Compiler decision rules are deterministic.
- Fallback cost is visible in inspect output.

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

- Benchmark raw data.
- Strategy decision report.
- Artifact hashes.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m25-002-b: use 256b 1kb 16kb 64kb nested objects arrays 100 1 000 optio
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record (M25-002-B)

Status: **PASS**. The benchmark corpus now covers the required payload matrix:
`small_user`, `nested_order`, `records100`, `records1000`, approximately
256 B / 1 KB / 16 KB / 64 KB objects, an optional/null-heavy object, and an
RFC 9457-shaped problem object. The generated projection remains benchmark-only;
production codec selection and fallback behavior are unchanged.

### Changed files

- `crates/q-bench-support/src/bin/codec_bench/schemas.rs` — ten-schema corpus,
  deterministic payload generators, optional/default and problem fixtures, and
  invalid differential cases.
- `crates/q-bench-support/src/bin/codec_bench/generator.rs` — M25-002-A/B
  generated decoder metadata and long-function allowance.
- `crates/q-bench-support/src/bin/codec_bench/generated.rs` — regenerated
  ten-schema projection source, locked by `generated_source_is_current`.
- `crates/q-bench-support/src/bin/codec_bench/main.rs` — M25-002-B metadata,
  candidate-aware normalized-output correctness, and ten-case evidence identity.
- `benchmarks/raw/codec/{codec.jsonl,codec-summary.json,evidence.json}` —
  60,000 raw rows, 30 OK cells, and current artifact hashes.
- `docs/reports/m25-002-b-payload-matrix.md` — source-backed matrix report with
  p50/p95/p99 values, fairness limitations, normalization semantics, and hashes.
- `docs/codex-spark-beta/STATUS.md`, `docs/codex-spark-beta/indexes/TASK_INDEX.md`,
  `docs/beta/04_TASK_LEDGER.md` — M25-002-A/B and aggregate M25-002 status.

### Tests and evidence

- `generated_source_is_current`
- `corpus_is_supported_by_generated_decoder`
- `generated_supports_matches_generator_guard`
- `unsupported_schemas_fail_closed_in_supports`
- `differential_decode_matches_generic_validator` (all ten valid fixtures and
  expanded invalid mutations)
- `fallback_without_inner_stays_unavailable`
- `cargo test -p q-bench-support` — 6 codec tests + 1 existing test passed.
- `cargo test -p q-engine-quickjs` — 1 unit + 96 integration tests passed.
- `cargo test -p q-schema-runtime` — 28 library + 2 fuzz tests passed.
- `bun test` — 66 passed, 0 failed, 233 expect calls.
- `bun run typecheck` — clean.
- `cargo fmt --check` — clean.
- `cargo clippy --workspace --all-targets -- -D warnings` — clean.
- `scripts/validate-okf` — 176 links, 0 errors.
- Release benchmark — run `m25-002-b-1787289561`, 30 cells, 2,000/2,000
  correct per cell, 60,000 JSONL samples.

`./scripts/verify` completed all Rust, TypeScript, proof-build, and OKF stages.
Its final benchmark-evidence check reports only the known isolated-worktree
hash mismatches for `qRuntimeRelease` and `proofPack` against the canonical
`benchmarks/manifest.json`; that manifest was intentionally preserved and no
benchmark claim depends on those mismatched artifacts.

Artifact hashes and the complete matrix are recorded in
`docs/reports/m25-002-b-payload-matrix.md` and
`benchmarks/raw/codec/evidence.json`.

Commit: `11b07bf`.
