---
type: Milestone Plan
title: M3 — Multi-Worker Service Runtime
status: draft
tags:
- milestone
- m3
- beta-roadmap

---

# M3 — Multi-Worker Service Runtime

## Objective

Scale across CPU cores with independent QuickJS runtimes, bounded queues, worker quarantine/replacement, and serverless/service/throughput profiles.

## Why this milestone exists

A public beta must serve normal concurrent APIs on multi-core hosts without hiding single-worker semantics or compromising cold-start mode.

## Entry criteria

- Required upstream dependencies in the task ledger are PASS.
- Working tree is clean and source/evidence baseline is identified.
- No unresolved upstream P0/P1 invalidates this milestone.

## Tasks

### M3-001 — Freeze independent-worker state semantics (P0)

**Dependencies:** M28-GATE

**Objective:** Define what JavaScript and native state is per worker versus shared.

**Implementation:**
- Accept ADR.
- Document module-level state replication.
- Forbid JSValue sharing.
- Define service/capability shared handles and thread safety.

**Acceptance:**
- Each runtime has one owner thread.
- Cross-worker mutable state is explicit.
- Initialization is deterministic.
- Developer docs describe per-worker globals.

**Required evidence:**
- ADR.
- Concurrency model tests plan.
- State examples.

### M3-002 — Implement bounded worker dispatcher (P0)

**Dependencies:** M3-001

**Objective:** Route matched requests to workers without unbounded queues or shared engine mutexes.

**Implementation:**
- Use bounded per-worker queues.
- Select worker using outstanding-load strategy.
- Define admission and overload response.
- Preserve RouteId/RoutePlan before dispatch.

**Acceptance:**
- Queue capacity is configurable and bounded.
- Overload fails quickly and observably.
- No head-of-line lock across workers.
- Per-worker queue latency is measured.

**Required evidence:**
- Dispatcher tests.
- Overload load test.
- Metrics.

### M3-003 — Implement serverless, service, and throughput profiles (P1)

**Dependencies:** M3-002

**Objective:** Make cold start versus immediate throughput an explicit deployment choice.

**Implementation:**
- Serverless starts one worker only.
- Service marks ready after worker 0 and adds workers adaptively.
- Throughput initializes configured workers before ready.
- Expose profile in inspect/config.

**Acceptance:**
- Serverless cold start remains within approved budget.
- Profiles have deterministic readiness.
- No hidden worker creation.
- Profile-specific RSS is reported.

**Required evidence:**
- Profile conformance.
- Cold/RSS report.
- Configuration docs.

### M3-004 — Implement deterministic worker initialization and artifact sharing (P0)

**Dependencies:** M3-002, M26-GATE

**Objective:** Load identical verified artifacts into independent runtimes efficiently.

**Implementation:**
- Share immutable mapped QPack bytes.
- Create separate QuickJS runtimes/functions/context state.
- Validate capability compatibility per worker.
- Bound startup parallelism.

**Acceptance:**
- Workers execute identical contracts.
- One worker failure does not corrupt others.
- No JS object crosses workers.
- Artifact memory sharing is measured.

**Required evidence:**
- Worker parity tests.
- Memory mapping report.
- Startup tests.

### M3-005 — Implement quarantine, replacement, and readiness aggregation (P0)

**Dependencies:** M3-002, M3-004

**Objective:** Replace poisoned workers without keeping the whole service permanently unhealthy.

**Implementation:**
- Remove quarantined worker from dispatch.
- Fail/settle its pending work.
- Initialize replacement under bounded policy.
- Aggregate readiness from usable capacity.

**Acceptance:**
- Poisoned worker receives no new requests.
- Replacement restores capacity.
- Repeated poison cannot create restart storm.
- Liveness/readiness semantics are correct.

**Required evidence:**
- Poison/replacement chaos tests.
- Readiness tests.
- Restart-rate metrics.

### M3-006 — Implement adaptive scale-up and scale-down (P1)

**Dependencies:** M3-003, M3-005

**Objective:** Add workers according to queue pressure while preserving memory budgets.

**Implementation:**
- Define thresholds/hysteresis.
- Bound min/max workers.
- Drain before scale-down.
- Avoid oscillation.

**Acceptance:**
- Adaptive mode scales under load.
- Idle workers retire safely.
- No request loss.
- RSS and latency trade-off is documented.

**Required evidence:**
- Adaptive load test.
- State transition tests.
- Memory report.

### M3-007 — Implement multi-worker cancellation and graceful shutdown (P0)

**Dependencies:** M3-002, M3-004

**Objective:** Propagate cancellation and shutdown to the owning worker and native operations exactly once.

**Implementation:**
- Track invocation-to-worker ownership.
- Stop admission on drain.
- Allow bounded in-flight completion.
- Abort after shutdown deadline.

**Acceptance:**
- No orphan invocation/native task.
- Shutdown deadline is honored.
- Exit code/report reflects forced aborts.
- All slots/queues/pools quiesce.

**Required evidence:**
- Shutdown integration tests.
- Disconnect/cancel races.
- Resource invariant report.

### M3-008 — Add fairness and overload controls (P1)

**Dependencies:** M3-002, M3-006

**Objective:** Prevent one route/tenant/slow workload from monopolizing workers.

**Implementation:**
- Add route/global queue limits or weighted admission.
- Define long-running JS policy.
- Expose load-shed reasons.
- Test mixed workloads.

**Acceptance:**
- Small requests make progress under slow workload.
- Overload does not cause unbounded memory.
- Limits are configurable.
- No starvation in approved scenarios.

**Required evidence:**
- Mixed-load benchmarks.
- Fairness metrics.
- Adversarial tests.

### M3-009 — Close multi-worker scaling and memory evidence (P1)

**Dependencies:** M3-003, M3-006, M3-008

**Objective:** Demonstrate real scaling without hiding queue latency or per-worker RSS.

**Implementation:**
- Measure 1/2/4 workers.
- Report throughput, p50/p95/p99, queue time, CPU, RSS, errors.
- Run C1/C2/C3 and controlled I/O.
- Record physical core topology.

**Acceptance:**
- 2 workers achieve approved scaling target or limitation is documented.
- 4-worker memory is budgeted.
- Serverless profile remains unchanged.
- No p99 collapse under saturation.

**Required evidence:**
- Raw scaling data.
- Generated report.
- Artifact/environment hashes.

### M3-010 — Run multi-worker soak and recovery (P0)

**Dependencies:** M3-005, M3-007, M3-009

**Objective:** Prove sustained service stability and worker replacement.

**Implementation:**
- Run multi-hour mixed load.
- Inject worker poison, upstream timeout, disconnect, and shutdown.
- Track retained memory and task/slot counts.
- Verify recovery.

**Acceptance:**
- No monotonic leak.
- Capacity recovers after replacement.
- No boundary violations.
- All errors are bounded and explained.

**Required evidence:**
- Soak raw data.
- Chaos timeline.
- Leak analysis.

## M3-GATE — Exit gate

- [ ] Independent workers scale across cores with bounded queues.
- [ ] Serverless mode preserves one-worker cold-start behavior.
- [ ] Quarantine/replacement and readiness are reliable.
- [ ] Cancellation/shutdown remain exact.
- [ ] Scaling, memory, fairness, and soak evidence pass.

## Required benchmark/evidence set

- 1/2/4 worker C1/C2/C3.
- Controlled I/O at c=10/50/200.
- Mixed slow/fast fairness.
- Poison/replacement soak.

## Explicit exclusions

- No shared mutable JavaScript heap.
- No distributed cluster coordinator.
- No hostile tenant isolation claim.

## Checkpoint deliverables

```text
clean source ZIP
Git bundle or patch history
SOURCE-COMMIT record
SHA-256 manifest
milestone report
review index
evidence index
captured test/typecheck/clippy output
raw benchmark/fuzz/soak evidence where required
known limitations and P2 backlog
```
