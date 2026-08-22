---
task_id: M25-007-Z
parent_task: M25-007
milestone: M25
priority: P1
mode: EVIDENCE
status: PASS
context_card: context/milestones/M25.md
commit_required: true
---

# M25-007-Z — Package evidence for Implement explicit generic and Web fallback paths

## Atomic goal

Create source-backed evidence and handoff for parent task M25-007; update status only if verification passed.

## Parent intent

Support advanced cases without hiding performance or semantic costs.

## Dependencies

- `M25-007-V` — `tasks/02_m25_schema_codecs/M25-007-V-verify-implement-explicit-generic-and-web-fallback-paths.md`

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
- `packages/cli/src/index.ts`
- `crates/q-runtime/src/source_map.rs`
- `examples/proof/`
- `README.md`
- `docs/beta/`
- `benchmarks/harness/`
- `benchmarks/manifest.json`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

## Parent acceptance guardrails

- Fallback never activates silently.
- Raw Response bypass behavior is documented.
- No contract claim is generated when adapter lacks required projection.
- Fallback routes pass conformance.

## Targeted commands

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

- Inspect snapshots.
- Fallback integration tests.
- Performance delta report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m25-007-z: package evidence for implement explicit generic and web fall
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Evidence package

- Status: **PASS**. Parent verification M25-007-V merged in PR #754 at
  commit `3290f5a820b527523cb67d965b2adc4f61c8eb78`; issue #160 is closed.
  The evidence package is based on clean parent HEAD `f944160` before this
  commit.
- Parent acceptance matrix: `M25-007-V` maps all four guardrails to source
  and named tests:
  1. Fallback never activates silently: pack-load rejection rules
     (`rejects_silent_fallback_and_invalid_reasons`), rogue raw-envelope
     500s, explicit descriptors for developer-forced js responses.
  2. Raw Response bypass documented: unsupported-transformations.md §5.
  3. No contract claim without required projection: raw-response/schema
     exclusivity at verify; real codec choices in the route manifest +
     CLI inspect snapshot.
  4. Fallback routes pass conformance:
     `fallback_paths_are_bounded_and_deadline_aware` (deadline kills on
     all three fallback paths, engine stays healthy, oversize 413) and
     the fallback parity tests.
- Source-backed implementation records:
  - `M25-007-A` (PR #750, #156 closed): fallback reasons tagged in the
    RoutePlan with closed-vocabulary verification.
  - `M25-007-B` (PR #751, #157 closed): raw Response and full Request
    escape hatches (capability-gated, documented, fail-closed).
  - `M25-007-C` (PR #752, #158 closed): bounded and deadline-aware
    fallback evidence across every path.
  - `M25-007-D` (PR #753, #159 closed): bridge crossings and codec choice
    exposed in `velqu inspect` (plus the hardcoded-native manifest bug
    fix).
- Inspect snapshots (required parent evidence): the M25-007-D compiler
  conformance test's live CLI snapshot plus manifest assertions.
- Performance delta report: no new measurement is made in M25-007; the
  fallback cost model remains the M25-002-D estimates surfaced in
  `build-report.json` and `velqu inspect fallbacks`. No performance claim
  is asserted; the benchmark manifest is preserved unchanged.
- Exact verification (fresh on this branch): `cargo test -p q-pack`
  (41 + 2 pass); `cargo test -p q-schema-runtime` (57 unit + 3 fuzz);
  `cargo test -p velqu-runtime` (24 pass); `cargo test -p q-engine-quickjs`
  (1 + 96 pass); `bun test` (75 passed, 0 failed, 340 expect calls);
  `bun run typecheck` clean; `cargo fmt --check` clean; `cargo clippy
  --workspace --all-targets -- -D warnings` clean; `scripts/validate-okf`
  (176 links, 0 errors).
- Full `./scripts/verify` completed all Rust, typecheck, proof-build, and
  TypeScript stages. Its final benchmark check reports only the known
  isolated-worktree hash mismatches for `qRuntimeRelease` and `proofPack`
  against `benchmarks/manifest.json`. The canonical root manifest and
  historical raw benchmarks were preserved.
- Status bookkeeping: `docs/beta/04_TASK_LEDGER.md` marks M25-007 PASS;
  the beta checklist and task index mark this Z packet PASS. The generated
  Spark queues now expose M25-008-A (#162) as the next dependency-ready
  packet.
- Remaining scope: `M25-008`–`M25-010` and `M25-GATE` remain TODO until
  implemented and evidenced.

Commit: `52f2907`.
