# M3-GATE Review — Multi-Worker Service Runtime

Milestone exit decision for M3 (Multi-Worker Service Runtime).

## Milestone Decision: PASS

All 10 parent tasks (`M3-001` through `M3-010`) are complete, verified with source-backed evidence, and squash-merged to master.

### Parent Task Dependency Closure
1. **M3-001 (Freeze Independent Worker State Semantics)** — PRs #976–#977: ADR-0036 (multi-worker state ownership and concurrency model: one runtime per owner thread, per-worker JS state, 4 named shared-mutable disciplines, no JSValue crossing workers, compile_fail doc test enforcement).
2. **M3-002 (Implement Bounded Worker Dispatcher)** — PRs #978–#983: `BoundedWorkerQueue` (bounded per-worker FIFO queues, immediate typed `Full` rejection), `Dispatcher` (least-outstanding selection with round-robin cursor, typed `AllFull`), `admission_response` (503 overload with 1 s Retry-After).
3. **M3-003 (Implement Serverless, Service, and Throughput Profiles)** — PRs #984–#989: `ServiceProfile` (Serverless=1 always, Service{workers}, parse fail-closed), `AdaptiveWorkers` (tick cooldown+max), `--service-profile` CLI flag, ready-line profile reporting.
4. **M3-004 (Implement Deterministic Worker Initialization and Artifact Sharing)** — PRs #990–#995: `SharedPack` (`SharedAcrossWorkers` marker, thread-safe freeze/pack bytes sharing), `spawn_independent` with worker labels, deterministic initialization sequence.
5. **M3-005 (Implement Quarantine, Replacement, and Readiness Aggregation)** — PRs #996–#1001: `quarantine`/`replace`/`settle_quarantined` on Dispatcher (excluded from selection immediately, pending settled typed), `ReplacementPolicy` (fixed-window budget with countdown), `FleetReadiness` / `aggregate_readiness`.
6. **M3-006 (Implement Adaptive Scale-Up and Scale-Down)** — PRs #1006–#1011: `ScaleThresholds` + `HysteresisState` (dead band, cooldown, stability window, floor), `WorkerBounds` validation, `RetiringWorker` lossless drain/budget escalation, `ScaleGovernor` event cap.
7. **M3-007 (Implement Multi-Worker Cancellation and Graceful Shutdown)** — PRs #1012–#1017: `InvocationOwnership` (bounded registry, settle as exactly-once gate), `DrainGate` (lock-free serving/draining flag), `GracefulShutdown` in accept loop, abort-through-ownership at budget deadline (`drain {refused, completed, aborted}`, `invocations.pending: 0` deterministically).
8. **M3-008 (Add Fairness and Overload Controls)** — PRs #1018–#1023: `FairAdmission` (weighted shares + shared borrow pool + ceiling), `LongRunningPolicy` + `LongRunningBudget`, `LoadShedReason` + `LoadShedCounters` (closed 7-kind vocabulary, rendered in `loadShed` report), mixed-load / adversarial tests.
9. **M3-009 (Close Multi-Worker Scaling and Memory Evidence)** — PRs #1024–#1029: `q-worker-scaling` benchmark: 1/2/4 real parallel QuickJS runtimes behind Dispatcher; C1 CPU (1.97×/3.53×), C2 mixed (2.12×/3.92×), C3 controlled I/O (1.98×/3.93×); flat heaps (201 KB); physical core topology (8 physical / 12 logical i5-13420H).
10. **M3-010 (Run Multi-Worker Soak and Recovery)** — PRs #1030–#1035: `q-soak` benchmark: sustained 30-min soak (4.41 M requests, 100% verified, flat heaps, RSS −388 KiB); 15-min chaos soak (14 engine rebuilds ~4 ms, 0 errors, 100% accounted); `retainedMemory` + `taskSlotCounts` tracking; recovery integration tests.

### Architecture Decision Records (ADRs Accepted)
- **ADR-0036**: Multi-Worker State Ownership and Concurrency Model (one runtime per thread, per-worker JS state, 4 shared-mutable disciplines, no JSValue crossing workers).

### Evidence Reports
- `docs/reports/m3-009-a-worker-scaling.md` — 1/2/4 worker scaling measurement
- `docs/reports/m3-009-b-multiworker-metrics.md` — Consolidated throughput, percentiles, queue time, CPU, RSS, errors
- `docs/reports/m3-009-c-controlled-workloads.md` — C1 CPU / C2 mixed / C3 controlled I/O
- `docs/reports/m3-009-d-host-topology.md` — Host physical core topology key
- `docs/reports/m3-010-a-soak.md` — 30-minute sustained mixed-load soak (4.41 M requests)
- `docs/reports/m3-010-b-chaos.md` — 15-minute chaos soak (14 replacements, disconnects, timeouts)
- `docs/reports/m3-010-c-retained-memory-and-slots.md` — Retained memory analysis and task/slot quiescence
- `docs/reports/m3-010-d-recovery.md` — Multi-worker recovery verification

### Standards Conformance & Open Items
- **Open Decisions**:
  - `PACK_FORMAT_CURRENT` v1→v2 default flip remains owner-gated (carried from M26, tracked in REVIEW_INDEX openItems).
  - Numeric 2-worker scaling target remains an owner decision (tracked in REVIEW_INDEX openItems).
  - No unauthorized out-of-order features (WebSockets, SSE, general Node compatibility remain post-beta per ADR-0018 / AGENTS.md constraint 15).
- **Standing CI Disclosure**: CI in this repository fails with zero executed steps on PRs (infrastructure-side since ~#714); local verification passes 100% from the clean candidate commit.
