---
task_id: M3-010-V
parent_task: M3-010
milestone: M3
priority: P0
mode: VERIFY
status: PASS
context_card: context/milestones/M3.md
commit_required: true
---

# M3-010-V — Verify Run multi-worker soak and recovery

## Atomic goal

Prove every acceptance criterion for parent task M3-010 without broadening scope.

## Parent intent

Prove sustained service stability and worker replacement.

## Dependencies

- `M3-010-A` — `tasks/06_m3_multi_worker/M3-010-A-run-multi-hour-mixed-load.md`
- `M3-010-B` — `tasks/06_m3_multi_worker/M3-010-B-inject-worker-poison-upstream-timeout-disconnect-and-shutdown.md`
- `M3-010-C` — `tasks/06_m3_multi_worker/M3-010-C-track-retained-memory-and-task-slot-counts.md`
- `M3-010-D` — `tasks/06_m3_multi_worker/M3-010-D-verify-recovery.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M3.md`
- `context/components/engine-scheduler.md`
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

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

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
cargo test -p velqu-runtime
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

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m3-010-v: verify run multi worker soak and recovery
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M3-010-V) — PASS

- Date: 2026-08-31
- Branch/PR: m3-010-v (squash-merged; see git log for final hash)
- Closes: #430

### Acceptance-criterion mapping (parent M3-010 guardrails)

1. **No monotonic leak** — verified:
   - Sustained 30-minute soak (4.41 M requests): final per-worker heaps
     203 853 / 201 427 B (flat in the ~201 KB band); process RSS ended
     −388 KiB below start (M3-010-A report).
   - 15-minute continuous chaos soak (2.43 M requests, 14 engine
     replacements): initial heaps `[201376, 201376]` B vs final
     `[206130, 202000]` B (net delta **+4.7 KB / +0.6 KB** across
     2.43 M requests and 14 engine rebuilds); RSS drift 0.30 B/req
     (bounded allocator retention; M3-010-C report).
2. **Capacity recovers after replacement** — verified:
   - 14/14 soak replacements initialized in 2.8–11.0 ms (median ~4 ms)
     and resumed full ~2.4k ops/s throughput within the same window (A/B/C).
   - Dedicated recovery test suite (`crates/q-capabilities/tests/recovery.rs`):
     `capacity_recovers_to_full_parallelism_after_worker_replacement`,
     `no_leaked_invocations_or_slots_across_repeated_poison_and_recovery`
     (50 rapid poison/settle/replace cycles with zero leaks, `pending == 0`),
     `least_outstanding_converges_loads_after_drain_and_rebuild` (D).
3. **All errors are bounded and explained** — verified:
   - 30-minute soak: 4 407 585 dispatched == 4 407 585 completed (100.0 %),
     0 errors (A).
   - 15-minute chaos soak: 2 431 643 dispatched == 2 407 340 completed +
     12 136 disconnects + 12 167 timeouts (100.0 % exact accounting),
     0 unexplained errors (C).
4. **No boundary violations** — verified: verify's scheduler boundary
   assertions pass; engine stats audit shows 0 boundary violations.

### Evidence chain (all committed, raw + generated + hashed)
- **A** #1030 (45ea7f2): `q-soak` harness + 30-minute soak (4.41 M requests,
  100% verified, 0 errors, flat heaps, RSS −388 KiB).
- **B** #1031 (859e763): chaos injection (worker poison every 60 s,
  disconnects, timeouts); 14 engine rebuilds ~4 ms; 100% accounting.
- **C** #1032 (886542e): retained memory tracking (initial vs final heaps,
  delta +4.7 KB / +0.6 KB) + task/slot accounting (`InvocationOwnership`
  quiescence: `pendingAtShutdown: 0`, `native_tasks_alive: 0`, `pending_ops: 0`).
- **D** #1033 (4624f8d): recovery integration test suite (3 tests) +
  recovery analysis report.

### Verification runs (this branch, worktree-fresh)
- `cargo test -p q-capabilities` → 260 lib + 6 workload + 3 recovery + 7 fuzz + 1 + 4 + 9 WPT — 0 failed
- `cargo test -p q-engine-quickjs` → 20 + 102 + 1 — 0 failed
- `cargo test -p velqu-runtime` → 7 suites — 0 failed
- `bun test` → 219/219; `bun run typecheck` → clean
- `cargo fmt --check` clean; workspace clippy -D warnings → exit 0
- `./scripts/verify` → **ALL PASS**

### Disclosures (standing)
- No production code changed in this packet: verification-only closure of
  M3-010-A/B/C/D.
- CI fails with zero executed steps on every PR since ~#714
  (infrastructure-side); disclosed per PR. Local evidence above is complete.
