---
task_id: M25-010-A
parent_task: M25-010
milestone: M25
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M25.md
commit_required: true
---

# M25-010-A — Run C2 plus medium/large JSON workloads

## Atomic goal

Run C2 plus medium/large JSON workloads.

## Parent intent

Prove the selected strategies improve real payloads without inflating startup unacceptably.

## Dependencies

- `M25-002-Z` — `tasks/02_m25_schema_codecs/M25-002-Z-package-evidence-for-build-reproducible-decoder-encoder-strategy-benchmark.md`
- `M25-009-Z` — `tasks/02_m25_schema_codecs/M25-009-Z-package-evidence-for-add-codec-fuzzing-and-differential-tests.md`

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
- `crates/q-schema-runtime/tests/fuzz_validator.rs`
- `conformance/schema/schema.conformance.test.ts`
- `benchmarks/harness/`
- `benchmarks/manifest.json`
- `benchmarks/raw/`
- `docs/reports/`
- `scripts/benchmark`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Run C2 plus medium/large JSON workloads.
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
m25-010-a: run c2 plus medium large json workloads
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record (M25-010-A)

Status: **PASS**. C2 plus medium/large JSON workloads ran fresh on the
current tree (post-M25-003..009 codecs), with raw samples retained:

- Raw evidence: `benchmarks/raw/codec-m25-010-a/` — codec.jsonl (2,000
  samples per candidate per case after 200 warmup), codec-summary.json,
  evidence.json (sha256 of the freshly built binary + generated module;
  the harness-generated `command` field carries the stale codec-c
  template string — disclosed in the report, generated files not
  hand-edited).
- Report: `docs/reports/m25-010-a-codec-workloads.md` — environment,
  actual invocation, the full 10-case matrix (small JSON, nested order,
  arrays 100/1,000, padded 256B/1KB/16KB/64KB, optional/null, problem
  shape), C2 codec-stage timings, findings, decision matrix.
- Findings (honest): C2 materially improves records1000 (+16.6% total
  p50 vs generic) and pad_256 (+25.4%); within ±3% on the other eight
  (both host candidates share the serde_json parse + QuickJS boundary —
  documented limitation). Native vs engine: 3.2x faster at 16KB and 6.2x
  at 64KB; quickjs-json wins records100 by 7% (the one parity shape).
  Decision matrix supports the M25-002-D native default + measured
  fallback; strategy remains inspectable (M25-007-D).
- Scope lines honored: no binary QPack encoding, no capability API
  expansion, no ORM. Root benchmark manifest untouched (raw evidence
  added under a new versioned directory; historical evidence preserved).

### Tests and evidence

- `cargo test -p q-schema-runtime` — 58 + 4 fuzz + 5 standards passed.
- `cargo test -p q-engine-quickjs` — 1 + 96 passed.
- `bun test` — 81 passed, 0 failed, 481 expect calls.
- `bun run typecheck` — clean. `cargo fmt --check` — clean.
- `cargo clippy --workspace --all-targets -- -D warnings` — clean.
- `scripts/validate-okf` — 176 links, 0 errors.

Commit: `ce74aff`.
