---
task_id: M26-010-D
parent_task: M26-010
milestone: M26
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M26.md
commit_required: true
---

# M26-010-D — Record p50/p95/p99, RSS, stage timings, and hashes

## Atomic goal

Record p50/p95/p99, RSS, stage timings, and hashes.

## Parent intent

Demonstrate flatter startup scaling and preserve small-app behavior.

## Dependencies

- `M26-010-C` — `tasks/03_m26_qpack_v2/M26-010-C-randomize-source-bytecode-competitor-order.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M26.md`
- `context/components/qpack-router.md`
- `context/components/evidence.md`

### Source files

- `AGENTS.md`
- `crates/q-pack/src/lib.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `crates/q-runtime/src/main.rs`
- `packages/compiler/src/emit.ts`
- `crates/q-pack/tests/fuzz_pack.rs`
- `docs/specs/pack-format-v1.md`
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
5. Implement exactly this deliverable: Record p50/p95/p99, RSS, stage timings, and hashes.
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
m26-010-d: record p50 p95 p99 rss stage timings and hashes
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record — M26-010-D (PASS)

Deliverable: the route-count evidence now records the full metric set
per cell — p50/p95/p99, RSS p50/p95, per-stage startup timings, and
self-identifying hashes.

Harness changes (`benchmarks/harness/route-count.ts`):

- `sample()` pipes velqu stdout and captures the ready line's `stages`
  (null for bun-based competitors); each raw JSONL row carries its own
  stage timings.
- Cell results add `p99Ms`, `rssP95Kb`, and `stageP50Ms` (per-stage
  p50 across the cell's ready lines).
- Summary format `velqu-route-count-v4-full-metrics` embeds sha256 of
  the measured `velqu-runtime` binary and of all ten ladder packs —
  raw evidence self-identifies what produced it (manifest remains the
  canonical release record).

Canonical run `m26-010-d` (4 × 5 × 100 = 2,000 spawns, zero
failures). Stage attribution (velqu p50, pack.load / total): 3.541 /
6.497 ms at 25 routes → 892.354 / 947.533 ms at 10,000 (source) —
pack.load reaches ~94% of startup, quantified per sample rather than
inferred from the single 10k profile. p99 within ~5% of p95 (no
long-tail pathology); RSS p95 ≈ p50.

Changed files: harness, canonical raw + summary, regenerated report,
D section in the ladder report, matched manifest refresh.

Commands: q-pack 94+2; velqu-runtime 30; bun 125 pass / 0 fail;
typecheck, fmt, clippy -D warnings clean; `./scripts/verify` — ALL
PASS (exit 0).
