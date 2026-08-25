---
task_id: M26-010-C
parent_task: M26-010
milestone: M26
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M26.md
commit_required: true
---

# M26-010-C — Randomize source/bytecode/competitor order

## Atomic goal

Randomize source/bytecode/competitor order.

## Parent intent

Demonstrate flatter startup scaling and preserve small-app behavior.

## Dependencies

- `M26-010-B` — `tasks/03_m26_qpack_v2/M26-010-B-at-least-100-fresh-processes-for-release-evidence.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M26.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/capabilities-fetch.md`

### Source files

- `AGENTS.md`
- `crates/q-pack/src/lib.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `crates/q-runtime/src/main.rs`
- `packages/compiler/src/emit.ts`
- `crates/q-pack/tests/fuzz_pack.rs`
- `docs/specs/pack-format-v1.md`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/src/worker.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Randomize source/bytecode/competitor order.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

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
cargo test -p q-capabilities
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

- Raw cold data.
- Generated report.
- Startup-stage trace.
- [ ] QPack v2 is deterministic, fail-closed, and version/fingerprint safe.
- [ ] Production startup maps verified runtime IR and raw bytecode without JSON/base64 reconstruction.
- [ ] Legacy compatibility is isolated.
- [ ] Shared and standalone artifacts pass conformance.
- [ ] Cold-start route scaling evidence is canonical.
- 25/100/1,000/5,000/10,000 route cold start.
- Shared vs standalone RSS/startup.
- Pack parse/allocation stages.
- Source vs bytecode selection.
- No full capability ecosystem.
- No Node compatibility.
- No multi-worker yet.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m26-010-c: randomize source bytecode competitor order
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record — M26-010-C (PASS)

Deliverable: order randomization strengthened from cell-level to
SAMPLE-level — source, bytecode, and competitor samples interleave
throughout the run.

Harness change (`benchmarks/harness/route-count.ts`):

- Every (candidate, size, sample-index) triple is one job; ALL jobs
  shuffle together (seeded LCG, seed recorded). Previously cells were
  shuffled but each cell's samples ran consecutively (100 in a row) —
  thermal drift/cache state biased later cells.
- Summary format bumped to `velqu-route-count-v3-sample-shuffled` with
  `sampleOrderRandomized: true` (cell-order flag retained).
- Cell stats computed exactly once, when a cell's last sample lands
  (a partial-duplication bug in the first shuffle implementation was
  caught by the 20-cell assertion before any evidence was kept).

Canonical run `m26-010-c` (4 × 5 × 100 = 2,000 spawns, zero
failures, every sample retained): max consecutive same-cell samples
**3** (was 100 by construction). p50s consistent with A/B runs —
velqu source 25-route 6.440 ms, 10k 948.156 ms; bytecode 5.952 /
926.173 ms; raw-bun ~7.8-8.5; elysia2 109.6 → 251.5 — medians did
not move, so prior ordering was not distorting results; the
randomization now PROVES that rather than assuming it.

Changed files: harness, new canonical raw + summary, regenerated
data-driven report, C section in the ladder report, matched manifest
refresh.

Commands: q-pack 94+2; velqu-runtime 30; bun 125 pass / 0 fail;
typecheck, fmt, clippy -D warnings clean; `./scripts/verify` — ALL
PASS (exit 0).
