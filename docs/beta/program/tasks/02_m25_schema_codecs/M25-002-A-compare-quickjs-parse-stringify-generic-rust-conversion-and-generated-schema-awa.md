---
task_id: M25-002-A
parent_task: M25-002
milestone: M25
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M25.md
commit_required: true
---

# M25-002-A — Compare QuickJS parse/stringify, generic Rust conversion, and generated schema-aware codecs

## Atomic goal

Compare QuickJS parse/stringify, generic Rust conversion, and generated schema-aware codecs.

## Parent intent

Measure QuickJS and native strategies across realistic payload shapes.

## Dependencies

- `M25-001-Z` — `tasks/02_m25_schema_codecs/M25-001-Z-package-evidence-for-define-canonical-schema-ir-v2.md`

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
5. Implement exactly this deliverable: Compare QuickJS parse/stringify, generic Rust conversion, and generated schema-aware codecs.
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
m25-002-a: compare quickjs parse stringify generic rust conversion and
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record (M25-002-A)

Status: **PASS**. Three-way codec strategy comparison implemented as a new
reproducible benchmark (`q-codec-bench`) with committed raw evidence, a
generated schema-aware projection prototype, and differential correctness
proofs. No production strategy, `ResponseStrategy`, RoutePlan metadata, or
compiler selection changed.

### Changed files

- `crates/q-bench-support/src/bin/codec_bench/main.rs` — benchmark binary:
  three candidates (`quickjs-json`, `generic-rust`, `generated-schema`),
  correctness-asserted timed cells, JSONL/summary/evidence output with
  sha256 hashes, `--emit-generated` regeneration mode.
- `crates/q-bench-support/src/bin/codec_bench/schemas.rs` — frozen Schema IR
  v2 corpus (small_user, nested_order, records100) + invalid-mutation
  fixtures (test-only).
- `crates/q-bench-support/src/bin/codec_bench/generator.rs` — Rust source
  generator emitting fused decode/validate projections from the corpus;
  `schema_supported` fail-closed guard (subset: plain strings, integer,
  number, boolean, array, object, optional/default, nullable,
  fallback-with-inner).
- `crates/q-bench-support/src/bin/codec_bench/generated.rs` — frozen generator
  output (locked by test).
- `crates/q-bench-support/Cargo.toml` — `q-codec-bench` bin + `sha2`.
- `benchmarks/raw/codec/{codec.jsonl,codec-summary.json,evidence.json}` —
  18,000 raw rows, 9 OK cells (2000/2000 correct each), artifact hashes.
- `docs/reports/m25-002-a-strategy-comparison.md` — strategy report with
  fairness limits and artifact hashes.
- `benchmarks/harness/run-all.ts` — codec benchmark joined the canonical
  `benchmark:all` suite; manifest gains `codecSummary`/`codecEvidence`
  references (canonical `benchmarks/manifest.json` itself NOT regenerated in
  this packet — refresh happens with B/C/D evidence per ADR-0012 discipline).

### Tests (exact names)

`generated_source_is_current`, `corpus_is_supported_by_generated_decoder`,
`generated_supports_matches_generator_guard`,
`unsupported_schemas_fail_closed_in_supports`,
`differential_decode_matches_generic_validator` (valid fixtures + 12 invalid
mutations, error-for-error parity with `q_schema_runtime::validate`),
`fallback_without_inner_stays_unavailable`.

### Evidence

| Command | Result |
| --- | --- |
| `cargo test -p q-bench-support` | 6 codec tests + 1 existing test passed |
| `./target/release/q-codec-bench --out-dir benchmarks/raw/codec --iters 2000` | 9/9 cells OK, 18,000 samples, run `m25-002-a-1787288528` |
| `cargo fmt --check` / `cargo clippy -p q-bench-support --all-targets -- -D warnings` | clean |

Headline (p50 μs, i5-13420H): generated projection beats generic validation
−28% (small_user), −8% (nested_order), −2% (records100); quickjs-json fastest
on records100 (+5% for generated) while doing no validation. No single
strategy wins across shapes — measured evidence only, selection is M25-002-D.

### Scope boundaries honored

- Direct byte scanner/decoder (no serde tree) is M25-003/M25-004 — not built.
- CPU/allocation capture is M25-002-C; strategy selection M25-002-D.
- Benchmark manifest, raw samples of existing suites, and release indexes
  untouched; no performance claim beyond the scope limits in the report.
