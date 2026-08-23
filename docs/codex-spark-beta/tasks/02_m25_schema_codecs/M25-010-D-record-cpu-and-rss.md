---
task_id: M25-010-D
parent_task: M25-010
milestone: M25
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M25.md
commit_required: true
---

# M25-010-D — Record CPU and RSS

## Atomic goal

Record CPU and RSS.

## Parent intent

Prove the selected strategies improve real payloads without inflating startup unacceptably.

## Dependencies

- `M25-010-C` — `tasks/02_m25_schema_codecs/M25-010-C-report-cold-start-delta-at-25-1-000-routes.md`

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
5. Implement exactly this deliverable: Record CPU and RSS.
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
m25-010-d: record cpu and rss
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record (M25-010-D)

Status: **PASS**.

- Raw evidence: `benchmarks/raw/codec-m25-010-d/` — codec.jsonl (2,000
  samples × 3 candidates × 10 cases), codec-summary.json (per-case
  `rssKbAfter`/`hwmKb`, process `maxRssKb`), codec.alloc.json (tracer
  profile), evidence.json (artifact sha256s). Allocator tracing
  captured this time (`allocatorStatus: captured`; tracer sha matches
  the M25-002-C record) — closes M25-010-A's disclosed gap.
- Bench change (`crates/q-bench-support/src/bin/codec_bench/main.rs`):
  added RSS recording (VmRSS/VmHWM per case + ru_maxrss process peak,
  with a note that ru_maxrss was observed inconsistent on some hosts —
  prefer hwmKb); PACKET field set to M25-010-D; COMMAND template and
  evidence paths now reflect the actual out-dir.
- Report: `docs/reports/m25-010-d-cpu-rss.md` — CPU p50 per cell,
  allocation deltas, RSS findings, decision matrix, host caveat.
- Key facts (honest): C2 CPU wins at array scale (records1000 −15%
  vs generic-rust) and is neutral-to-noise elsewhere; strategy choice
  moves per-case RSS by ≤ ~220 KB; bench process HWM 12.7 MB;
  host candidates share a parse-dominated allocation profile (+1.5%
  calls for the projection). Cold-start RSS cross-referenced from
  M25-010-C.

### Tests and evidence

- `cargo test -p q-engine-quickjs` — 1 + 96 passed; `cargo test -p
  q-schema-runtime` — 58 + 5 + 4 passed; workspace cargo tests green
  (incl. q-pack 41 + 2, velqu-runtime 24).
- `bun test` — 81 passed, 0 failed, 481 expect() calls.
- `bun run verify` — **ALL PASS** (fmt, clippy -D warnings,
  validate-okf, validate-production-plan, benchmark-report parity,
  release builds, typecheck, conformance suites, evidence validation).
- Bench invocation:
  `LD_PRELOAD=target/alloc-tracer.so VELQU_ALLOC_PROFILE=benchmarks/raw/codec-m25-010-d/codec.alloc.json target/release/q-codec-bench --out-dir benchmarks/raw/codec-m25-010-d --iters 2000`.
