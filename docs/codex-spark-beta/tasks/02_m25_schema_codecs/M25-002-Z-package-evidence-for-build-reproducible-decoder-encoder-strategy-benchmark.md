---
task_id: M25-002-Z
parent_task: M25-002
milestone: M25
priority: P1
mode: EVIDENCE
status: PASS
context_card: context/milestones/M25.md
commit_required: true
---

# M25-002-Z — Package evidence for Build reproducible decoder/encoder strategy benchmark

## Atomic goal

Create source-backed evidence and handoff for parent task M25-002; update status only if verification passed.

## Parent intent

Measure QuickJS and native strategies across realistic payload shapes.

## Dependencies

- `M25-002-V` — `tasks/02_m25_schema_codecs/M25-002-V-verify-build-reproducible-decoder-encoder-strategy-benchmark.md`

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

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

## Parent acceptance guardrails

- Raw and generated results are committed.
- No single strategy is forced globally.
- Compiler decision rules are deterministic.
- Fallback cost is visible in inspect output.

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
```bash
cargo fmt --check
```
```bash
cargo clippy --workspace --all-targets -- -D warnings
```
```bash
./scripts/verify
```

## Required evidence for this microtask

- Benchmark raw data.
- Strategy decision report.
- Artifact hashes.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m25-002-z: package evidence for build reproducible decoder encoder stra
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Evidence package

- Status: **PASS**. Parent verification M25-002-V merged in PR #724 at commit
  `60caa3176cbe5360e7da9a17fb5f4ca2f32efcb1`; issue #130 is closed. The evidence
  package is based on clean parent HEAD `5afe415` before this commit.
- Parent acceptance matrix: `M25-002-V` maps all four guardrails to source and
  named tests:
  1. Raw and generated results committed: `benchmarks/raw/codec/`,
     `benchmarks/raw/codec-c/`, `crates/q-bench-support/src/bin/codec_bench/generated.rs`.
  2. No single strategy forced globally: `packages/compiler/src/strategy.ts`,
     `docs/reports/m25-002-d-strategy-selection.md`.
  3. Compiler decision rules deterministic: `packages/compiler/src/strategy.ts`,
     `conformance/compiler/compiler.test.ts`.
  4. Fallback cost visible in inspect output: `packages/cli/src/index.ts`,
     `build-report.json`, `build-report.md`.
- Source-backed implementation records:
  - `M25-002-A` (PR #720, #126 closed): initial 3-candidate strategy benchmark
    and generated projection prototype (`docs/reports/m25-002-a-strategy-comparison.md`).
  - `M25-002-B` (PR #721, #127 closed): payload matrix expansion across 10 shapes,
    256B/1KB/16KB/64KB, arrays 100/1,000, opt/null, problems (`docs/reports/m25-002-b-payload-matrix.md`).
  - `M25-002-C` (PR #722, #128 closed): per-sample CPU (rusage), allocator deltas
    (LD_PRELOAD tracer snapshot ABI), bridge timing, tails (`docs/reports/m25-002-c-cpu-allocation-bridge-tails.md`).
  - `M25-002-D` (PR #723, #129 closed): compiler strategy selection rules, fallback
    cost surfacing, inspect integration (`docs/reports/m25-002-d-strategy-selection.md`).
- Exact verification: `cargo test -p q-pack` (pass); `cargo test -p q-engine-quickjs`
  (1 unit + 96 integration pass); `cargo test -p q-schema-runtime` (28 lib + 2 fuzz pass);
  `cargo test -p q-bridge` (11 default pass, 13 feature pass); `bun test` (69 passed,
  0 failed, 296 expect calls); `bun run typecheck` clean; `cargo fmt --check` clean;
  `cargo clippy --workspace --all-targets -- -D warnings` clean; `scripts/validate-okf`
  (176 links, 0 errors); `scripts/validate-benchmark-evidence.py` (codec-c checks clean).
- Full `./scripts/verify` completed all Rust, typecheck, proof-build, and TypeScript
  stages. Its final benchmark check reports only the known isolated-worktree hash
  mismatches for `qRuntimeRelease` and `proofPack` against `benchmarks/manifest.json`.
  The canonical root manifest and historical raw benchmarks were preserved.
- Status bookkeeping: `docs/beta/04_TASK_LEDGER.md` marks M25-002 PASS; the
  beta checklist and task index mark this Z packet PASS. The generated Spark
  queues now expose M25-003-A (#132) as the next dependency-ready packet.
- Remaining scope: `M25-GATE` remains TODO and future M25 packets (M25-003+)
  remain TODO until implemented and evidenced.

Commit: `a3ff6d4`.
