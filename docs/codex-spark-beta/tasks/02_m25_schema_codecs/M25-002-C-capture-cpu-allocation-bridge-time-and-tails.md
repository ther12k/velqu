---
task_id: M25-002-C
parent_task: M25-002
milestone: M25
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M25.md
commit_required: true
---

# M25-002-C — Capture CPU, allocation, bridge time, and tails

## Atomic goal

Capture CPU, allocation, bridge time, and tails.

## Parent intent

Measure QuickJS and native strategies across realistic payload shapes.

## Dependencies

- `M25-002-B` — `tasks/02_m25_schema_codecs/M25-002-B-use-256b-1kb-16kb-64kb-nested-objects-arrays-100-1-000-optional-null-and-problem.md`

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
5. Implement exactly this deliverable: Capture CPU, allocation, bridge time, and tails.
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
m25-002-c: capture cpu allocation bridge time and tails
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record (M25-002-C)

Status: **PASS**. Per-sample CPU (getrusage), allocation-event/requested-byte
deltas (LD_PRELOAD tracer snapshots), native bridge access timing
(feature-gated), stage splits (codec/engine/total), and metric-specific
p50/p95/p99 tails were added to the frozen ten-case benchmark. No production
codec, strategy, or fallback behavior changed; default workspace builds carry
no instrumentation code.

### Changed files

- `crates/q-bridge/Cargo.toml`, `crates/q-bridge/src/lib.rs` —
  `bench-instrumentation` feature: `access_time_ns` counter + snapshot field,
  `access`/`cached_query` wrappers timed around the unchanged inner logic,
  two feature-gated timing tests (default build unchanged: 11 tests).
- `crates/q-engine-quickjs/Cargo.toml` — feature propagation.
- `crates/q-bench-support/Cargo.toml` — feature propagation + `libc`.
- `crates/q-bench-support/src/bin/codec_bench/main.rs` — `Sample` capture
  (total/codec/engine wall, CPU user/system, bridge counters and access time,
  allocator deltas via `dlsym`), v2 raw rows (24 fields, zero null allocator
  values), per-metric summary stats, M25-002-C run id/metadata, current-exe
  evidence, process-profile reference.
- `scripts/alloc-tracer.c` — exported `velqu_alloc_snapshot` ABI (six packed
  u64s) alongside the unchanged exit profile.
- `scripts/validate-benchmark-evidence.py` — strict codec-c validation:
  2,000 rows per cell, all rows correct and fully populated, summary metric
  completeness, evidence hashes, tracer exit profile, process time capture.
- `benchmarks/raw/codec-c/{codec.jsonl,codec-summary.json,evidence.json,
  codec.alloc.json,codec.process.time.txt}` — run `m25-002-c-1787293512`,
  60,000 rows, 30/30 cells OK.
- `docs/reports/m25-002-c-cpu-allocation-bridge-tails.md` — full metric
  tables, instrumentation semantics/limits, process totals, observations,
  artifact hashes.
- `docs/codex-spark-beta/STATUS.md`,
  `docs/codex-spark-beta/indexes/TASK_INDEX.md` — M25-002-C PASS.

### Tests and evidence

- `cargo test -p q-bridge` — 11 passed (default, no timing code).
- `cargo test -p q-bridge --features bench-instrumentation` — 13 passed
  (adds `access_and_cached_query_accumulate_timing`,
  `denied_access_is_still_timed`).
- `cargo test -p q-bench-support` — 6 codec + 1 existing test passed.
- Canonical instrumented run — 30 cells × 2,000 samples, all correct,
  allocator captured, bridge timing enabled; validated by
  `scripts/validate-benchmark-evidence.py` (codec-c checks clean).
- Remaining targeted commands (`q-engine-quickjs`, `q-http`, `q-bridge`,
  `q-schema-runtime`, `velqu-runtime`, `bun test`, `typecheck`) run at commit
  time; results in the PR body.

M25-002-B evidence under `benchmarks/raw/codec/` and its report are
deliberately untouched; the canonical root `benchmarks/manifest.json` was not
regenerated.

Commit: `dd8c041`.
