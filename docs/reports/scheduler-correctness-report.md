---
type: Evidence Report
title: M2.2.1 Scheduler Correctness Closure (r4.2.1)
status: complete
milestone: M2.2.1
---

# M2.2.1 scheduler correctness report (revisions 2, 3, 4, 4.1, and 4.2.1)

Closes all findings from the scheduler and terminalization reviews:
- **r2**: invocation-scoped job deadlines, async continuation ownership, fail-closed policy resolution, floating-operation cleanup, terminal slot cleanup, master-prompt scope alignment (ADR-0018).
- **r3**: physical native-task cancellation via `AbortHandle`, bounded ownerless watchdog drains, execution-phase guards against second-generation operations, and policy-ID → handler-key resolution.
- **r4 (terminal-state closure)**: bounded live-invocation drains, fail-closed poison quarantine, and the race-free `NativeTaskState`/`TaskLivenessGuard` accounting model.
- **r4.1 (terminalization unification)**: single quarantine path, queue-empty-or-quarantined drain contract, and runtime readiness exposure.
- **r4.2.1 (cleanup budget unification & encapsulation)**:
  1. **DrainReport with drain-local interrupt flag**: `drain_jobs_for` returns `DrainReport { outcome, interrupted }`. The `interrupted` flag is strictly drain-local; an interrupt during request B's cleanup never leaks to another request A, preventing false timeouts on interleaved completions.
  2. **Unified cleanup budget (`cleanup_budget(id)`)**: all invocation cleanup (`expire_timeouts`, `cancel_invocation`, `Step::Failed`, `abort_floating_ops`, and `finish_resolved` floating-op cleanup) strictly routes through `cleanup_budget(id)` with `SETTLEMENT_GRACE` (100 ms) and `ExecutionPhase::Cleanup`. The 5-second watchdog survives *only* for genuinely ownerless shutdown recovery.
  3. **Single-assignment settlement grace**: `grace_deadline` is assigned at most once per drain (`if job_interrupted && grace_deadline.is_none()`), preventing error jobs from renewing the 100 ms grace indefinitely.
  4. **Unconditional `pending_ops.swap(0)` on quarantine**: `quarantine_runtime` unconditionally swaps `pending_ops` to 0, ensuring `pending_ops == 0` on terminal state even if accounting drift occurs.
  5. **Encapsulated `EngineHealth` API**: `WorkerShared` is `pub(crate)`; `EngineHealth` exposes `is_ready()` and `is_quarantined()` with lock-free atomic reads (`Ordering::Acquire`), keeping mutable scheduler internals encapsulated.

## r4.2.1 findings & resolutions

| # | Finding | Resolution |
|---|---|---|
| 1 | Interrupted cleanup could leave global interrupt flag set, falsely timing out subsequent requests (P0) | `DrainReport` makes interrupt status drain-local; `interrupted.swap(false)` inside the drain guarantees zero leakage across requests. |
| 2 | Post-settlement floating-op cleanup and failed handlers used the 5s watchdog instead of cleanup budget (P0) | Helper `cleanup_budget(invocation_id)` with `SETTLEMENT_GRACE` (100ms) used for all invocation cleanup; 5s watchdog reserved exclusively for shutdown. |
| 3 | `grace_deadline` was renewable on every error iteration (P1) | Assigned once per drain: `if job_interrupted && grace_deadline.is_none() { grace_deadline = Some(now + grace); }`. |
| 4 | `pending_ops` unsigned subtraction could wrap on accounting drift (P1) | `swap(0, Ordering::SeqCst)` guarantees `pending_ops == 0` on quarantine; drift logged as boundary violation. |
| 5 | Public `health()` API exposed mutable `WorkerShared` struct (P1) | `WorkerShared` is `pub(crate)`; public API exposes `EngineHealth` with `is_ready()` and `is_quarantined()`. |

## r4.2.1 conformance tests (crates/q-engine-quickjs/tests/engine.rs)

| Test | Proves |
|---|---|
| `cleanup_interrupt_does_not_timeout_unrelated_invocation` | Request B's interrupted cleanup does not falsely time out interleaved request A (A succeeds with 200) |
| `post_settlement_floating_cleanup_uses_cleanup_budget` | Post-settlement cleanup of floating timers uses 100ms grace, not 5s watchdog (next request serves in <2s) |
| `failed_handler_cleanup_uses_cleanup_budget` | Synchronous handler failure cleans up queued jobs under 100ms grace, not 5s watchdog |
| `promise_settlement_cleanup_uses_cleanup_budget` | Promise settlement floating-op cleanup runs under 100ms grace |
| `ordinary_async_timeout_does_not_quarantine_worker` | 5s timer with 50ms route deadline: Timeout, `queue_poisoned == false`, `pending_ops == 0`, subsequent sync + async requests succeed |
| `cancelled_async_request_cleanup_does_not_quarantine` | Cancelled async request gets fresh cleanup grace; worker remains healthy (`queue_poisoned == false`) |
| `pathological_timeout_cleanup_still_quarantines` | Timeout catch reaction chaining microtasks forever still quarantines after cleanup budget (not fail-open) |
| `quarantine_accounting_drift_resets_pending_ops_to_zero` | Quarantine unconditionally resets `pending_ops == 0` |

## Acceptance gates (r4.2.1)

| Gate | Result |
|---|---|
| Drain interruption is drain-local (0 leakage across requests) | **PASS** (`cleanup_interrupt_does_not_timeout_unrelated_invocation`) |
| All invocation cleanup uses fresh 100ms cleanup budget | **PASS** (floating cleanup, failed handlers, promise settlement) |
| Interrupted drain settlement grace is single-assignment | **PASS** |
| Quarantine unconditionally zeroes `pending_ops` | **PASS** (`quarantine_accounting_drift_resets_pending_ops_to_zero` — drift-injection unit test in `worker.rs`) |
| Lock-free `EngineHealth` API encapsulates `WorkerShared` | **PASS** |
| Ordinary async timeout produces clean 504 without quarantine | **PASS** (`ordinary_async_timeout_does_not_quarantine_worker`) |
| HEAD `/health/ready` body == 0 on 200 and 503 | **PASS** (`poisoned_runtime_marks_readiness_false`) |
| Response mapping (toJSON/getters) bounded by route deadline | **PASS** (r4.2.2: 7 response-budget tests) |
| 5s watchdog used only at shutdown | **PASS** — sole remaining call site is the shutdown drain; message-boundary leftovers use `cleanup_budget(0)` |
| Zero-microtask sync fast path preserved | **PASS** (`sync_fast_path_zero_plumbing_cost`: 1,000 sync requests $\to$ 0 drains) |

## r4.2.2 (response-mapping budget & accounting parity)

| # | Finding | Resolution |
|---|---|---|
| 1 | `value_to_outcome` executed user JS (toJSON/getters/proxy traps) with the interrupt deadline DISARMED — a spinning `toJSON()` could freeze the worker ignoring the route deadline (P0) | `InterruptDeadlineScope` RAII guard keeps the deadline armed through handler call, watch attachment, response conversion, and error extraction; disarmed only after the `Step` is built. `finish_resolved` arms the owner's deadline around Promise-result conversion (pre-check: expired deadline → deterministic Timeout). |
| 2 | Microtasks queued by getters/toJSON during conversion fell through to the ownerless watchdog (P0) | Remaining jobs after settlement drain under the owning invocation's budget (`settle_background` scoped); message-boundary leftovers use `cleanup_budget(0)`; the 5s watchdog is now shutdown-only. |
| 3 | `pending_ops.swap(0)` claimed but absent; `fetch_sub` could wrap on drift (P1) | Quarantine now uses `pending_ops.swap(0, SeqCst)`; drift recorded as boundary violation; drift-injection unit test proves zero gauge with no wrap. |

r4.2.2 tests (`engine.rs`): `sync/async_response_tojson_spin_obeys_route_deadline`,
`sync/async_response_getter_spin_obeys_route_deadline`,
`response_mapping_microtask_stays_with_owner`,
`response_mapping_timeout_leaves_worker_reusable`,
`problem_object_getter_is_bounded`; unit test
`quarantine_accounting_drift_resets_pending_ops_to_zero` (worker module).

## Verification (current)

`./scripts/verify` $\to$ **ALL PASS (M0–M2 + M2.2.1 verified)** — 113 Rust tests
(65 engine integration + 1 worker unit, 12 runtime, 35 unit/fuzz/router/schema), 35 TypeScript tests, clippy clean.

---

## Historical verification counts

- r4.2.1: 105 Rust tests (58 engine, 12 runtime) — superseded by r4.2.2.
- r4.2: 100 Rust tests (53 engine, 12 runtime) — superseded by r4.2.1.
- r4.1: 97 Rust tests (50 engine, 12 runtime) — superseded by r4.2.
- r4: 87 Rust tests (41 engine, 11 runtime) — superseded by r4.1.
- r3: 81 Rust tests (35 engine, 11 runtime) — superseded by r4.

## r4 findings & resolutions

| # | Finding | Resolution |
|---|---|---|
| 1 | Invocation microtask drains (`drain_jobs_for`) lacked host-clock & job-count bounds (P0) | `drain_jobs_for` checks `Instant::now() >= budget.deadline` and the per-drain job cap on every iteration; unquiesced work triggers the unified quarantine (r4.1). |
| 2 | Poisoned runtime continued serving dynamic JS routes (P0) | When quarantined, `begin_invocation` fails closed immediately for all subsequent dynamic requests; all active pending requests are aborted and settled immediately. |
| 3 | `native_tasks_alive` increment after spawn created zero-delay underflow race; abort raced with completion causing double counts (P0/P1) | `NativeTaskState` atomic state machine + `TaskLivenessGuard` dropped on task future destruction; increment before `tokio.spawn`; atomic CAS transition guarantees exactly one winning state (Completed or Aborted). |

## r4 conformance tests (crates/q-engine-quickjs/tests/engine.rs)

| Test | Proves |
|---|---|
| `zero_delay_timer_does_not_wrap_alive_counter` | 50 zero-delay timers: `alive == 0` (never underflows to `u64::MAX`), `started == completed == 50`, `aborted == 0` |
| `sync_tiny_self_rescheduling_chain_is_bounded` | Live synchronous handler running an infinite microtask chain times out within the bounded test threshold without hanging the worker |
| `async_tiny_self_rescheduling_chain_is_bounded` | Live async Promise handler running an infinite microtask chain fails closed / times out within the bounded test threshold without hanging |
| `poisoned_worker_rejects_new_dynamic_requests_immediately` | Quarantined worker rejects new sync and promise routes within the bounded test threshold with `EngineFailure` |
| `all_pending_invocations_fail_when_worker_is_poisoned` | 5-second pending async request fails closed within the bounded threshold when an unquiescable chain poisons the queue |

## Acceptance gates (r4 + r4.1)

| Gate | Result |
|---|---|
| Any unquiesceable queue: exactly one quarantine path | **PASS** — `quarantine_runtime` is the only writer of `queue_poisoned` |
| Any drain return: queue empty OR runtime quarantined | **PASS** — quiescence checked before budget enforcement on every iteration |
| After quarantine: pending invocations == 0, pending ops == 0, live request slots == 0 | **PASS** (checked `pending_ops` accounting incl. the current invocation's ops) |
| Readiness == false after quarantine | **PASS** — `/health/ready` 503, liveness 200, dynamic routes 503 |
| `native_tasks_started == completed + aborted + alive` | **PASS** — holds exactly after the native-task set quiesces (independent relaxed atomics give an eventually consistent live snapshot) |
| `native_tasks_alive` never wraps / underflows | **PASS** |
| Live invocation microtask drain is bounded | **PASS** |
| `scheduler_boundary_violations == 0` | **PASS** |
| Zero-microtask sync fast path preserved | **PASS** (`sync_fast_path_zero_plumbing_cost`: 1,000 sync requests $\to$ 0 drains) |
| ownerless native operations | PASS — 0 (phase guard refuses Idle/Cleanup/Shutdown starts) |
| late completion after successful cancellation | PASS — aborted tasks send nothing |

## Verification (historical r4.1)

`./scripts/verify` → **ALL PASS** — 97 Rust tests
(50 engine, 12 runtime incl. readiness), 35 TypeScript tests, clippy clean.

---

## historical verification counts (r4 and earlier)

- r4: 87 Rust tests (41 engine, 11 runtime) — superseded by r4.1.
- r3: 81 Rust tests (35 engine, 11 runtime) — superseded by r4.

---

## r2 record (previous revision)

## Findings addressed

| # | Finding (severity) | Resolution |
|---|---|---|
| 1 | Microtask checkpoint used min-pending/watchdog deadline instead of `spec.deadline` (P0) | `drain_jobs_for(JobBudget)` owns deadline arming/cleanup; checkpoint drains under the invocation's own `spec.deadline`. Generic watchdog only for ownerless work (shutdown, floating-op unwinding). |
| 2 | Continuations inherited `CURRENT_INVOCATION` of the most recently started request (P0) | `PendingOp` now stores `{invocation_id, deadline}` captured at op start; `complete_timer` returns the owning `JobBudget`; continuations drain inside `InvocationScope::enter(owner)` RAII guard. |
| 3 | Missing policy handler failed OPEN (P0 security) | Engine: `policy_key` set but handler absent → settle slot + `EngineFailure` reply, business handler never invoked. Pack: `QPack::verify` rejects any `PolicyEntry.handler` missing from `handler_table` and key/id mismatches. |
| 4 | Floating native ops survived synchronous settlement (P1) | `abort_floating_ops(spec.id)` after the microtask checkpoint on Immediate and Failed paths; `finish_resolved` aborts stragglers of settled promise invocations; rejections unwind under the bounded watchdog. |
| 5 | Missing-handler path leaked the request slot (P1) | Terminal failure path settles `store.settle(slot, generation)` before replying. |
| 6 | `MASTER_AGENT_PROMPT.md` stop point conflicted with ADR-0018 (P1) | Section 16 superseded: ADR-0018 authorizes ordered M2.2.1–M4; each milestone an independent checkpoint; out-of-order work prohibited. `scripts/verify` now reports `M0–M2 + M2.2.1`. |

## New measurement points

`EngineStats` gained `pending_ops` (live native ops) and
`scheduler_boundary_violations` (message-boundary invariant: `CURRENT_INVOCATION
== 0`, `CURRENT_DEADLINE == None`, `sync_deadline` unarmed — checked after
every worker message; violations counted, logged, and state restored).

## Conformance tests (crates/q-engine-quickjs/tests/engine.rs)

| Test | Proves |
|---|---|
| `sync_runaway_microtask_respects_route_deadline` | 100ms route deadline kills a runaway sync microtask well before the 5s watchdog |
| `sync_runaway_microtask_leaves_worker_reusable` | worker serves sync + microtask requests after a deadline kill; 0 boundary violations |
| `sync_checkpoint_does_not_borrow_other_pending_request_deadline` | a pending 5s async request cannot inflate a 100ms sync request's checkpoint budget |
| `sync_checkpoint_does_not_interrupt_from_other_request_deadline` | a pending 80ms async request cannot kill a 5s sync request's 300ms microtask |
| `async_continuation_preserves_invocation_owner` | cancelling request B does not reject request A's nested timer; A completes `{total:460}` |
| `nested_native_op_is_owned_by_original_invocation` | chained ops: started=2, completed=2, pending=0, dropped=0 despite interleaved request |
| `floating_native_op_is_cancelled_at_sync_settlement` | unawaited 60s timer cancelled at settlement; pending_ops=0; worker healthy |
| `missing_policy_handler_fails_closed` | EngineFailure mentioning the policy and `fail closed`; slot settled |
| `missing_policy_handler_never_calls_business_handler` | guarded marker never executed |
| `missing_handler_settles_request_slot` | slot does not leak on unknown handler |
| `deadline_and_current_invocation_clear_at_message_boundary` | mixed workload (sync, checkpoint, timer, timeout, cancel): 0 boundary violations |

Pack-level (crates/q-pack/src/lib.rs):
`rejects_policy_with_unknown_handler` and
`accepts_policy_with_resolvable_handler`.

## Zero-drain fast path preserved

`sync_fast_path_zero_plumbing_cost` still asserts 1,000 synchronous
microtask-free invocations produce 0 job drains, 0 promise watches, 0
settlement scans. Post-change spot check (release build, c=10, 4s):
C1 61,186 req/s · C2 117,405 req/s · C3 68,180 req/s — no regression
(gate was ≤3%); improvement is within run-to-run noise of a
single-candidate harness.

## Verification

`./scripts/verify` → **ALL PASS (M0–M2 + M2.2.1 verified)** — 74 Rust tests
(29 engine incl. 11 new scheduler tests), 35 TypeScript tests, clippy clean,
OKF manifest hashes regenerated for the superseded master-prompt section.
