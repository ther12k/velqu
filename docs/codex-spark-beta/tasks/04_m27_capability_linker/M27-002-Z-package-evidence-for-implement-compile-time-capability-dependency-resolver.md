---
task_id: M27-002-Z
parent_task: M27-002
milestone: M27
priority: P0
mode: EVIDENCE
status: PASS
context_card: context/milestones/M27.md
commit_required: true
---

# M27-002-Z — Package evidence for Implement compile-time capability dependency resolver

## Atomic goal

Create source-backed evidence and handoff for parent task M27-002; update status only if verification passed.

## Parent intent

Resolve exactly which capabilities enter each application artifact.

## Dependencies

- `M27-002-V` — `tasks/04_m27_capability_linker/M27-002-V-verify-implement-compile-time-capability-dependency-resolver.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M27.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/capabilities-fetch.md`

### Source files

- `AGENTS.md`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `packages/compiler/src/emit.ts`
- `crates/q-pack/src/lib.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

## Parent acceptance guardrails

- Unrelated app pays zero linked capability cost.
- Dependency graph is deterministic.
- Missing capability fails at build or startup.
- `velqu inspect --capabilities` is accurate.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p q-capabilities
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

- Resolver tests.
- Binary-size delta report.
- Cold-start delta report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m27-002-z: package evidence for implement compile time capability depen
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Evidence package

- Status: **PASS**. Parent verification M27-002-V merged in PR #852
  at commit `511e813dbf88d0beba9b66b2e480da2454ea9e85`; issue #250
  is closed. Based on clean parent HEAD `17d4491` (queue-regen)
  plus this packet's bookkeeping.
- Parent acceptance matrix: `M27-002-V` maps all four guardrails to
  source and named tests (zero-cost empty-link structural parity +
  measured +56 B when linking; deterministic id-sorted closure with
  cross-language pinned vectors; typed Missing/VersionConflict/Cycle
  and unknown-grant failures at build time with runtime-side
  hash-bound verification; hash-recomputing accurate inspect).
- Source-backed implementation records:
  - `M27-002-A` (PR #848, #246 closed): dependency DAG builder —
    transitive closure, id-sorted deterministic output,
    visited-once termination anchor for B.
  - `M27-002-B` (PR #849, #247 closed): cycle rejection — DFS path
    stack, `ResolveError::Cycle` naming the full traversal path;
    A's termination test flipped as designed.
  - `M27-002-C` (PR #850, #248 closed): inventory section in qpack —
    canonical encoding hash-bound across Rust/TS/Python
    implementations; cross-language vectors caught a real TS
    encoding-order bug pre-merge.
  - `M27-002-D` (PR #851, #249 closed): pruning + two real defect
    fixes (destructured-native grant detection silently losing
    timer grants; unknown grants silently dropped). Measured size/
    cold-start deltas with retained raw samples.
  - `M27-002-V` (PR #852, #250 closed): verification closure;
    matched manifest refresh after C/D legitimately changed the
    compiled proof pack.
- Canonical evidence artifacts:
  - Tests: q-capabilities 51 (resolver 13 + inventory 8 + M27-001's
    30); TS capability suites 21+2 vectors inside bun's 139/0.
  - Reports: `docs/reports/m27-002-d-prune-deltas.md`.
  - Benchmarks: `benchmarks/manifest.json` refreshed under verify's
    remap environment at V (proofPack 5329b73…, byte-identical
    independent rebuilds via compare-builds).
- Exact verification (fresh on this branch): q-pack 98,
  q-engine-quickjs 98, q-capabilities 51, velqu-runtime 30 passed;
  `bun test` 139 pass / 0 fail; typecheck, fmt --check, clippy
  `-D warnings` clean. `./scripts/verify`: first attempt FAILED
  validate-benchmark-evidence (documented environmental race: the
  prior plain `-p velqu-runtime` debug-env build landed a
  non-remapped binary before verify's remapped rebuild); two
  consecutive clean reruns from identical state — ALL PASS exit 0.
  No test weakened, no evidence altered.
- Status bookkeeping: ledger marks M27-002 PASS; TASK_INDEX marks
  M27-002-Z PASS. Queues expose M27-003-A next.
- Remaining scope: M27-003+ (QuickJS context profiles), M27-GATE.
