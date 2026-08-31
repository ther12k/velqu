---
task_id: M3-009-D
parent_task: M3-009
milestone: M3
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M3.md
commit_required: true
---

# M3-009-D — Record physical core topology

## Atomic goal

Record physical core topology.

## Parent intent

Demonstrate real scaling without hiding queue latency or per-worker RSS.

## Dependencies

- `M3-009-C` — `tasks/06_m3_multi_worker/M3-009-C-run-c1-c2-c3-and-controlled-i-o.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M3.md`
- `context/components/engine-scheduler.md`
- `context/components/multiworker.md`
- `context/components/evidence.md`

### Source files

- `AGENTS.md`
- `crates/q-runtime/src/main.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-engine-quickjs/src/lib.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`
- `benchmarks/harness/`
- `benchmarks/manifest.json`
- `benchmarks/raw/`
- `docs/reports/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Record physical core topology.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- 2 workers achieve approved scaling target or limitation is documented.
- 4-worker memory is budgeted.
- Serverless profile remains unchanged.
- No p99 collapse under saturation.

## Targeted commands

```bash
cargo test -p q-engine-quickjs
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

- Raw scaling data.
- Generated report.
- Artifact/environment hashes.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m3-009-d: record physical core topology
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M3-009-D) — PASS

- Date: 2026-08-31
- Branch/PR: m3-009-d (squash-merged; see git log for final hash)
- Closes: #423

### Changed files
- `scripts/capture-host-topology.py` (new): deterministic host topology
  capture from /proc/cpuinfo + sysfs — logical/physical/socket counts,
  SMT factor, models, clock range, NUMA nodes, and a SHA-256 of the raw
  cpuinfo bytes (evidence binds to the exact host description);
  unreadable fields recorded as null, never fabricated.
- `crates/q-bench-support/src/bin/worker_scaling.rs`: `physicalTopology`
  block embedded in every worker-scaling summary (parsed from
  /proc/cpuinfo the same way); replaced the misleading top-level
  `physicalCores` key that actually held the LOGICAL count.
- `benchmarks/raw/worker-scaling/`: host-topology.json + regenerated
  summary/JSONL (71 100 samples) from the final source.
- `docs/reports/m3-009-d-host-topology.md` (new): the topology record
  and what it explains in the A/B/C numbers.

### Captured topology (exact)
i5-13420H: 12 logical / **8 physical** cores / 1 socket, siblings-per-core
**1.5** (hybrid 4 P ×2-way SMT + 4 E ×1), 1 NUMA node, 12 MB cache,
2 399.7–3 802.7 MHz.

### What the topology explains (report carries the full reading)
- W=4 ≈ the physical-core budget → measured 3.53×–3.93× consistent with
  slightly under 4× (producers + Tokio share the cores).
- W=2 ratios >2× (2.09×–2.22×) are topology effects (dedicated P-cores
  vs a shared core at W=1) — legitimate to report BECAUSE the topology
  is recorded.
- Heterogeneous cores + frequency scaling explain the repetition
  spread; interleaved repetitions average placement luck.

### Evidence regenerated from the final source
C1 811/1 690/2 987 ops/s (2.09×/3.69×), C2 3 507/7 780/13 580
(2.22×/3.87×), C3 442/874/1 736 (1.98×/3.93×) — 0 errors across all
21 600 measured requests.

### Command results
- `cargo test -p q-engine-quickjs` → 20 + 102 + 1 — 0 failed
- `cargo test -p velqu-runtime` → 7 suites — 0 failed
- `bun test` → 219/219; `bun run typecheck` → clean
- `cargo fmt --check` clean; workspace clippy -D warnings → exit 0
- `./scripts/verify` → **ALL PASS**

### Disclosures
- Standing: CI fails with zero executed steps on every PR since ~#714
  (infrastructure-side); disclosed per PR.
