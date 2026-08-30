---
task_id: M3-007-Z
parent_task: M3-007
milestone: M3
priority: P0
mode: EVIDENCE
status: PASS
context_card: context/milestones/M3.md
commit_required: true
---

# M3-007-Z — Package evidence for Implement multi-worker cancellation and graceful shutdown

## Atomic goal

Create source-backed evidence and handoff for parent task M3-007; update status only if verification passed.

## Parent intent

Propagate cancellation and shutdown to the owning worker and native operations exactly once.

## Dependencies

- `M3-007-V` — `tasks/06_m3_multi_worker/M3-007-V-verify-implement-multi-worker-cancellation-and-graceful-shutdown.md`

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

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

## Parent acceptance guardrails

- No orphan invocation/native task.
- Shutdown deadline is honored.
- Exit code/report reflects forced aborts.
- All slots/queues/pools quiesce.

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

- Shutdown integration tests.
- Disconnect/cancel races.
- Resource invariant report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m3-007-z: package evidence for implement multi worker cancellation and
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M3-007-Z) — PASS

- Date: 2026-08-31
- Branch/PR: m3-007-z (squash-merged; see git log for final hash)
- Closes: #413
- Parent verification: M3-007-V PASS (PR #1016, merged 9c3b937) on the
  identical tree; this packet packages the evidence and flips the ledger.

### Evidence package (parent M3-007 — multi-worker cancellation & graceful shutdown)
- **Implementation commits (squash-merged):**
  - M3-007-A ownership registry — #1012 → 43f0a7e
  - M3-007-B drain gate — #1013 → da99de6
  - M3-007-C bounded in-flight completion — #1014 → 2c8d89b
  - M3-007-D abort after shutdown deadline — #1015 → 3d49ba9
  - M3-007-V verification closure — #1016 → 9c3b937
- **Source paths:** `crates/q-capabilities/src/invocation.rs` (InvocationOwnership),
  `crates/q-capabilities/src/drain.rs` (DrainGate), `crates/q-runtime/src/serve.rs`
  (ownership-tracked admission/terminal transition, CancelOnDrop settle-gate,
  drain gate check), `crates/q-runtime/src/lib.rs` (flip task, sweep,
  shutdown.complete invocations+drain blocks), `crates/q-http/src/lib.rs`
  (JoinSet + GracefulShutdown watcher, bounded drain wait, abort-through-ownership,
  ServeDrain).
- **Key tests:** `settle_is_the_exactly_once_gate_for_cancel_routing`,
  `concurrent_admission_and_settlement_stays_consistent`,
  `no_orphan_audit_over_a_full_admit_settle_cycle`,
  `drain_state_is_visible_across_threads_immediately` (A);
  `begin_flips_exactly_once_and_refuses_admission`,
  `concurrent_begins_have_exactly_one_winner` (B);
  `drain_lets_in_flight_request_complete` (C);
  `drain_waits_bounded_then_detaches_straggler_connection` (D);
  `graceful_shutdown_exits_zero`, `graceful_drain_flips_gate_and_reports_before_exit`
  (integration).
- **Resource invariant report (from the shutdown.complete evidence):**
  `invocations {pending:0, registered, settled}` on every graceful drain —
  no orphan invocation; `drain {refused, completed, aborted}` — admission
  refusals, in-flight completions, and forced aborts each counted;
  `stats.cancelled_invocations` / `native_tasks_aborted` — the engine's
  forced-abort record. The straggler test pins all of these deterministically
  with the budget bounds (elapsed ≥ 5s budget, < 10s exit).
- **Disconnect/cancel races:** settle-as-gate CancelOnDrop (settle-before-disarm
  ordering), two-thread settle race, M2.2.1 engine cancellation suite.
- **Full gate results (this branch, worktree-fresh):** q-engine-quickjs
  20+102+1; velqu-runtime 55+6+5+2+35; fmt clean; workspace clippy -D warnings
  exit 0; `./scripts/verify` **ALL PASS** (bun 183 scoped tests + release hash
  matching the manifest).

### Ledger
- `docs/beta/04_TASK_LEDGER.md`: M3-007 TODO → **PASS** (all four guardrails
  proven; see M3-007-V mapping).

### Disclosures (standing)
- No runtime behavior changed in this packet: evidence-only closure.
- CI fails with zero executed steps on every PR since ~#714
  (infrastructure-side); disclosed per PR.
