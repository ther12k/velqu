---
task_id: M26-010-Z
parent_task: M26-010
milestone: M26
priority: P1
mode: EVIDENCE
status: PASS
context_card: context/milestones/M26.md
commit_required: true
---

# M26-010-Z — Package evidence for Close route-count cold-start evidence

## Atomic goal

Create source-backed evidence and handoff for parent task M26-010; update status only if verification passed.

## Parent intent

Demonstrate flatter startup scaling and preserve small-app behavior.

## Dependencies

- `M26-010-V` — `tasks/03_m26_qpack_v2/M26-010-V-verify-close-route-count-cold-start-evidence.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M26.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/schema-codecs.md`

### Source files

- `AGENTS.md`
- `crates/q-pack/src/lib.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `crates/q-runtime/src/main.rs`
- `packages/compiler/src/emit.ts`
- `crates/q-pack/tests/fuzz_pack.rs`
- `docs/specs/pack-format-v1.md`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`
- `crates/q-schema-runtime/src/lib.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

## Parent acceptance guardrails

- No runtime router/schema compilation.
- No base64 decoding.
- 25-route budget is not sacrificed silently.
- 10,000-route scaling is documented honestly.

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
cargo test -p velqu-runtime
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

- Raw cold data.
- Generated report.
- Startup-stage trace.
- [x] QPack v2 is deterministic, fail-closed, and version/fingerprint safe.
- [x] Production startup maps verified runtime IR and raw bytecode without JSON/base64 reconstruction.
- [x] Legacy compatibility is isolated.
- [x] Shared and standalone artifacts pass conformance.
- [x] Cold-start route scaling evidence is canonical.
- 25/100/1,000/5,000/10,000 route cold start.
- Shared vs standalone RSS/startup.
- Pack parse/allocation stages.
- Source vs bytecode selection.
- No full capability ecosystem.
- No Node compatibility.
- No multi-worker yet.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m26-010-z: package evidence for close route count cold start evidence
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Evidence package

- Status: **PASS**. Parent verification M26-010-V merged in PR #839 at
  commit `a47ab0aa83e6b83266578b872c2872fb4debb76a`; issue #238 is
  closed. This package is based on clean parent HEAD (queue-regen
  commit `100472d`) before this commit.
- Parent acceptance matrix: `M26-010-V` maps all four guardrails to
  source and named tests (no runtime router/schema compilation —
  stage timings show router.build as plan application; no base64
  reconstruction — direct bytecode mapping after the single
  pack-load decode; 25-route budget measured at p50 6.143 ms, fastest
  velqu candidate; 10,000-route scaling reported with pack.load
  attribution 97%/94% and no slope-fix claim).
- Canonical evidence artifacts (all committed, hash-bound):
  - Raw: `benchmarks/raw/route-count/` — 2,000 JSONL rows (5 sizes ×
    4 candidates × 100 fresh processes), 0 failures, per-sample
    candidate/order LCG shuffle, 1,000 rows with startup-stage
    traces; `summary.json` format
    `velqu-route-count-v4-full-metrics` with p50/p95/p99,
    rssP50/rssP95, stageP50Ms, 10 pack sha256 hashes, runtime
    binary hash.
  - Report: `docs/reports/m26-010-a-route-count-ladder.md` —
    generated from the summary (ladder table, stage table, honesty
    disclosures, QPack v2 named as the authorized lever).
  - Cross-mode: `docs/reports/m26-009-b-standalone-mode.md` —
    shared vs standalone startup/RSS/sizes, n=10 per mode, raw
    samples retained.
- Source-backed implementation records:
  - `M26-010-A` (PR #835, #234 closed): route-count ladder harness
    at 25/100/1,000/5,000/10,000 routes, 4 candidates.
  - `M26-010-B` (PR #836, #235 closed): 100 fresh processes per
    cell for release evidence.
  - `M26-010-C` (PR #837, #236 closed): per-sample global shuffle of
    (candidate, n, sample) jobs with cell-completion gating.
  - `M26-010-D` (PR #838, #237 closed): p50/p95/p99, rssP50/rssP95,
    per-row ready-line stage timings, embedded binary+pack hashes.
  - `M26-010-V` (PR #839, #238 closed): verification closure; no
    defects found. This packet also corrects two issue refs in that
    record (A/B were cited as #233/#234; actual #234/#235).
- Exact verification (fresh on this branch): q-pack 96,
  q-engine-quickjs 98, q-schema-runtime 67, velqu-runtime 30 passed;
  `bun test` 125 pass / 0 fail; typecheck, `cargo fmt --check`,
  clippy `-D warnings` clean. `./scripts/verify` — ALL PASS
  (exit 0).
- Status bookkeeping: `docs/beta/04_TASK_LEDGER.md` marks M26-010
  PASS; TASK_INDEX marks M26-010-Z PASS. The generated Spark queues
  expose M26-GATE next.
- Remaining scope: `M26-GATE` (M2.6 exit gate) is the only M26
  packet left.
