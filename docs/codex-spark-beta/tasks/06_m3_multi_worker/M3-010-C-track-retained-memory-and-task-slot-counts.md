---
task_id: M3-010-C
parent_task: M3-010
milestone: M3
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M3.md
commit_required: true
---

# M3-010-C — Track retained memory and task/slot counts

## Atomic goal

Track retained memory and task/slot counts.

## Parent intent

Prove sustained service stability and worker replacement.

## Dependencies

- `M3-010-B` — `tasks/06_m3_multi_worker/M3-010-B-inject-worker-poison-upstream-timeout-disconnect-and-shutdown.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M3.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`
- `context/components/multiworker.md`

### Source files

- `AGENTS.md`
- `crates/q-runtime/src/main.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-engine-quickjs/src/lib.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`
- `crates/q-http/src/lib.rs`
- `crates/q-bridge/src/lib.rs`
- `crates/q-runtime/tests/runtime_conformance.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Track retained memory and task/slot counts.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- No monotonic leak.
- Capacity recovers after replacement.
- No boundary violations.
- All errors are bounded and explained.

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
cargo test -p velqu-runtime
```

## Required evidence for this microtask

- Soak raw data.
- Chaos timeline.
- Leak analysis.
- [ ] Independent workers scale across cores with bounded queues.
- [ ] Serverless mode preserves one-worker cold-start behavior.
- [ ] Quarantine/replacement and readiness are reliable.
- [ ] Cancellation/shutdown remain exact.
- [ ] Scaling, memory, fairness, and soak evidence pass.
- 1/2/4 worker C1/C2/C3.
- Controlled I/O at c=10/50/200.
- Mixed slow/fast fairness.
- Poison/replacement soak.
- No shared mutable JavaScript heap.
- No distributed cluster coordinator.
- No hostile tenant isolation claim.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m3-010-c: track retained memory and task slot counts
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M3-010-C) — PASS

- Date: 2026-08-31
- Branch/PR: m3-010-c (squash-merged; see git log for final hash)
- Closes: #428

### Changed files
- `crates/q-bench-support/src/bin/soak.rs`: added explicit retained-memory
  and task/slot tracking to `q-soak` (format `velqu-soak-v2`) —
  - Initial per-worker heap captured immediately after load.
  - Full `InvocationOwnership` (M3-007-A) tracking for every soak
    request (`track` on pop, `settle` on outcome/error/disconnect).
  - Window samples record `ownershipPendingSlots` and `queueTotal`.
  - Shutdown collects final `EngineStats` from each worker
    (`native_tasks_started`, `native_tasks_completed`,
    `native_tasks_aborted`, `native_tasks_alive`, `pending_ops`,
    `cancelled_invocations`).
  - Summary includes `retainedMemory` block (initial vs final heap per
    worker, net heap delta, RSS growth per completed request) and
    `taskSlotCounts` block (ownership stats, peak live slots, final
    pending slots = 0, final alive native tasks = 0).
- `crates/q-engine-quickjs/src/worker.rs`: `WorkerMsg::Load` now updates
  `self.shared.heap_used` immediately upon bundle evaluation so post-load
  initial heap is accurately readable from engine stats.
- `benchmarks/raw/worker-scaling/soak.jsonl` + `soak-summary.json`:
  committed 15-minute soak evidence with memory and slot tracking.
- `docs/reports/m3-010-c-retained-memory-and-slots.md` (new):
  retained-memory analysis, task/slot accounting, and artifact hashes.
- `benchmarks/manifest.json`: refreshed (standard remapped flow).

### Committed evidence (exact values)
15 minutes (900.7 s, 30 windows), 2 workers, continuous chaos (14
rebuilds, 5 ‰ disconnects, 5 ‰ timeouts):
- **2 431 643 dispatched == 2 407 340 completed + 12 136 disconnects +
  12 167 timeouts — 100.0000 % accounted**; 0 unexplained errors.
- **Retained memory**: initial heaps `[201376, 201376]` B; final heaps
  `[206130, 202000]` B (net deltas **+4.7 KB / +0.6 KB** across 2.43 M
  requests and 14 engine replacements — flat); RSS drift 0.30 B/req
  (bounded allocator retention).
- **Task & slot quiescence**: `ownership.pendingAtShutdown == 0`
  (`registered == settled == 2431643`); `native_tasks_alive == 0`;
  `pending_ops == 0`; peak live queue slots 2 048 (bounded by capacity).

### Command results
- `cargo test -p q-engine-quickjs` → 20 + 102 + 1 — 0 failed
- `cargo test -p velqu-runtime` → 7 suites — 0 failed
- `bun test` → 219 pass / 0 fail; `bun run typecheck` → clean
- `cargo fmt --check` clean; workspace clippy -D warnings → exit 0
  (boxed `EngineStats` in `ConsumerMsg::Done`)
- `./scripts/verify` → **ALL PASS**

### Disclosures
- Standing: CI fails with zero executed steps on every PR since ~#714
  (infrastructure-side); disclosed per PR.
