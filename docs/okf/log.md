# Project Q Design Update Log

## 2026-08-17

- **Creation** — Created an OKF v0.2 design and product bundle for the proposed Rust + QuickJS cold-start-first TypeScript framework.
- **Review** — Reassessed the earlier architecture against current official Elysia 2, Eden Treaty, AWS LLRT, QuickJS, QuickJS-NG, rquickjs, Bun, hyper, Tokio, RFC 9457, and OKF sources.
- **Correction** — Reclassified native JSON decoding/validation and response serialization as measured hypotheses because cross-language materialization can erase native parsing gains.
- **Decision** — Kept Bun as the development toolchain while defining the production runtime as a separate Rust/QuickJS system.
- **Decision** — Selected QuickJS-NG only as the initial engine candidate behind an adapter; upstream QuickJS remains a benchmark alternative.
- **Decision** — Rejected simultaneous Rust and Zig implementation for the first version.
- **Decision** — Separated fast Treaty unit mode from native-runtime integration conformance.
- **Delivery** — Scoped the first agent handoff to M0–M2 and prohibited unsupported performance or compatibility claims.
- **Trust** — All design documents remain draft and unverified by a human reviewer.
- **Packaging** — Generated a machine-readable manifest, structural validation result, ZIP archive, and SHA-256 checksum for handoff.

## 2026-08-17 (implementation start)

- **Ingestion** — Bundle moved verbatim to `docs/okf/`; structural validation re-run (all PASS); implementation audit, open-decisions register, and live traceability created under `docs/`.
- **Owner instruction** — Working name for this implementation is "Velqu"; scope remains the authorized M0–M2 stop point.
- **Environment** — rustc 1.96.0, Bun 1.3.4, rquickjs =0.12.2 (vendoring quickjs-ng 0.15.1) verified to build and evaluate in this environment.
- **Freeze** — Benchmark fixture contract, application pack format v1, and public API sketch frozen under `benchmarks/fixtures/fixture-contract.json` and `docs/specs/`.

## 2026-08-17 (M0/M1 complete)

- **M0 PASS** — frozen fixture contract + canonical checker: all four candidates 27/27; type spike + scale runs recorded (budget miss at 100 routes: fixed tsc floor).
- **M1 PASS** — Rust host + single quickjs-ng worker: 45 tests green; route-before-JS, lazy bridge counters, cancellation matrix, limits, tamper rejection, redaction, source maps all demonstrated on the actual binary.
- **Measurement** — velqu C3 cold p50 2.9ms / p95 4.4ms vs matched Elysia 2 AOT p50 132.6ms / p95 152.0ms; idle RSS 6.2 MiB; 0 failures in 1680 cold samples.
- **Correction (evidence-driven)** — bridge benchmark: native (Rust serde) JSON inputs beat engine JSON.parse by 11–42% on this host (counter to the review's expectation) → adopted as compiler default; ADR-0015.
- **Negative result preserved** — 1,000-route cold start p50 15.7ms is +409% over 25 routes (budget ≤20%): FAIL recorded honestly; absolute value still ~10× faster than matched Elysia candidate.
- **Treaty deviation** — route-id navigation chosen over Eden-exact single-segment form (ambiguity on `/users`); open decision ID-011.

## 2026-08-18 (M2 complete — authorized stop point reached)

- **M2 PASS** — static compiler (AST extraction, zero app dry-run; trap/collision/import/dynamic diagnostics all source-located), deterministic pack + 9 artifacts (qpack, manifests, contract.json/.d.ts, openapi.json 3.1, lock, build-report), `q` CLI (build/inspect/contract diff), proof app (9 frozen-contract routes incl. native liveness detection), Treaty runtime-local + unit-local modes, 21 TS conformance tests + 45 Rust tests green via `scripts/verify`.
- **Evidence** — warm suite 16 cells 0 errors (velqu C2 116k req/s p50 85μs); cold suite re-run 0 failures; type-scale negative checks caught at 100/500/1000.
- **Handoff** — 14 evidence reports + live traceability (44/44 PASS) + final archive with SHA-256. Stop point: M2; no M3/alpha work started.

## 2026-08-18 (post-M2 enhancement pass)

- **Correctness** — drain-time interrupt arming (runaway `.then()` continuations now interruptible; worker survives); result envelopes restricted to tagged `{__ok}`/`{__problem}` (business objects with `status`/`value` fields are bodies); Timeout mapping requires a confirmed interrupt flag (genuine errors near deadlines are no longer masked).
- **Contract depth** — response schemas now emitted end-to-end (pack registry, typed `contract.d.ts` success bodies, OpenAPI response content) and enforced at runtime with controlled 500 + `contract.violation.response` diagnostics.
- **Lock workflow fixed** — `contract.lock.json` is written once and preserved across builds (`--update-lock` to refresh); `q contract diff` now detects real drift (test proves removal → breaking).
- **Hardening** — property-fuzz suites for pack parser/mutation (tamper >200/256), query/percent-decode, and schema validator (40k cases each, no panics); CI workflow (x86_64 + aarch64) running `scripts/verify`.

## 2026-08-18 (naming decided — ADR-0016)

- **Decision (owner)** — Brand **Velqu**, descriptive **VelquJS**, CLI `velqu`, packages `@velqu/*`, runtime binary `velqu-runtime`; ranking Velqu > VelquJS > VelquTS. OD-001/OD-002 closed; repository/license/governance remain open.
- **Migration** — workspace packages, imports, tsconfig paths, generated contract.d.ts, CLI, and the runtime binary renamed. Internal Rust crate names (`q-*`) intentionally unchanged (internal units); historical bundle/reports keep original wording (provenance preserved; ADR-0016 is the mapping).

## 2026-08-18 (post-M2 scope authorized + M2.2.1 scheduler closure r2)

- **ADR-0018 accepted** — post-M2 roadmap authorized in order: M2.2.1 scheduler correctness, M2.3 numeric RoutePlan, M2.4 zero-copy ingress/worker-local slab, M2.5 schema JSON codecs, M2.6 binary QPack v2, M2.7 capability linker/WinterTC, M2.8 native fetch, M3 multi-worker, M4 alpha. Master prompt §16 superseded; AGENTS.md constraint 15 updated; each milestone an independent review checkpoint.
- **M2.2.1-r2** — closed the conditional-pass findings: invocation-scoped `JobBudget` job draining (checkpoint uses `spec.deadline`, never min-pending/watchdog), `PendingOp{invocation_id,deadline}` + `InvocationScope` RAII restoring continuation ownership, fail-closed policy resolution (engine + `QPack::verify` policy-handler checks), `abort_floating_ops` at settlement, missing-handler slot cleanup, `pending_ops`/`scheduler_boundary_violations` stats, 11 new scheduler conformance tests + 2 pack tests. Evidence: `docs/reports/scheduler-correctness-report.md`. Zero-drain sync fast path preserved (1,000 sync invocations → 0 drains).

## 2026-08-19 (M2.2.1-r3 native-operation cleanup)

- **Physical cancellation** — `PendingOp` carries the Tokio `AbortHandle`; every host-initiated rejection aborts the real task; shutdown aborts all leftovers. Task accounting split (`native_tasks_alive/completed/aborted`); 2,000-request accumulation test proves the op cap bounds real tasks again.
- **Bounded ownerless watchdog** — one absolute deadline + 10k job cap + kill round. Engine limitation found and documented: quickjs-ng never polls the interrupt handler in tiny promise jobs, so an unquiescable microtask chain cannot be deadline-killed; contained via queue poison marking (`queue_poisoned`/`poison_events`) — later drains skip the queue, sync serving continues, event logged. Queued for the engine-upgrade lane.
- **ExecutionPhase guard** — `Idle/Invocation/Cleanup/Shutdown` thread-local with RAII; native ops refused outside live invocations; cleanup/cancel/settlement reactions cannot spawn second-generation ownerless ops (tested both via catch-reactions).
- **Policy resolution fixed** — runtime resolves `route.policy → PolicyEntry.handler` (was passing the policy ID; valid packs with differing keys failed closed at request time). Conformance fixture now uses `auth.session` → handler `auth.session.check`; all policy tests exercise the resolution.
- Evidence: `docs/reports/scheduler-correctness-report.md` (r3 section). verify: ALL PASS — 81 Rust + 35 TS tests.

## 2026-08-19 (M2.2.1-r4 scheduler terminal-state closure)

- **Bounded live-invocation drains** — `drain_jobs_for` bounds both wall-clock time between microtasks (`Instant::now() >= budget.deadline`) and job count (`MAX_INVOCATION_JOBS = 100_000`); live handlers with unquiescable microtask chains cannot hang the worker.
- **Fail-closed poison quarantine** — unquiescable microtask chains trigger immediate quarantine: pending invocations fail closed immediately (`Outcome::EngineFailure`), live native tasks abort, and subsequent dynamic JS requests fail closed (<1ms).
- **Race-free native task state machine** — `NativeTaskState` (`Running=0`, `Completed=1`, `AbortRequested=2`) with `TaskLivenessGuard` dropped on Tokio task destruction. Increments before `tokio.spawn` prevent zero-delay underflow; atomic CAS transitions prevent double-counting.
- **Evidence** — 10 new tests in `engine.rs` (zero-delay alive counter, completion vs abort CAS races, physical drop guard, live sync/async tiny chain bounds, immediate poison rejection, pending invocation poison-fail). `scripts/verify` passes cleanly (87 Rust + 35 TS tests).

## 2026-08-19 (M2.2.1-r4.1 terminalization unification)

- **One quarantine path** — `quarantine_runtime(reason)` is the only writer of `queue_poisoned`; the cleanup path (`kill_remaining_jobs`) routes through it, so cleanup-triggered quarantine also fails pending invocations immediately, aborts native ops, and corrects `pending_ops` with checked accounting.
- **Queue-empty-or-quarantined drain contract** — every drain returns Quiesced or RuntimeQuarantined; quiescence is checked before budget enforcement (a finite final job finishing after the deadline or at the exact job cap no longer quarantines); interrupted/throwing jobs obey the same budget; after an engine interrupt, terminal settlement jobs drain under a bounded 100ms grace inside the same invocation scope (leftover jobs never escape to another owner). Job cap configurable via `QuickJsConfig::max_invocation_jobs`.
- **Readiness exposure** — built-in `/health/ready` flips to 503 (`engine quarantined`) while `/health/live` stays 200; dynamic JS routes fail closed at the HTTP boundary (503 + retry-after). Runtime conformance test added.
- **Evidence honesty fixes** — real completion/abort race tests (300×~1ms floating timers + deterministic abort side); timing claims reworded to bounded thresholds; task-count invariant documented as holding after quiescence (eventually consistent snapshot); single current verification block with historical counts labeled.
- verify: ALL PASS — 97 Rust (50 engine, 12 runtime) + 35 TS tests.

## 2026-08-19 (M2.2.1-r4.2 cleanup budget separation)

- **Cleanup budget separated from route deadline** — `expire_timeouts`, `cancel_invocation`, and settlement cleanup run under a fresh `JobBudget { deadline: Instant::now() + SETTLEMENT_GRACE }` rather than the expired `p.spec.deadline`; ordinary 504 timeouts of async requests no longer mis-trigger quarantine and the worker remains healthy.
- **Single-assignment settlement grace** — `grace_deadline` is assigned once per drain; `pending_ops` reset unconditionally uses `swap(0, Ordering::SeqCst)` on quarantine.
- **Lock-free readiness check** — `ServeState` per-request check uses `health.queue_poisoned.load(Ordering::Acquire)` (0 engine mutex acquisitions); HEAD `/health/ready` 503 sets `head_only = true` (0 body emitted).
- **Evidence** — 3 new tests in `engine.rs` (`ordinary_async_timeout_does_not_quarantine_worker`, `cancelled_async_request_cleanup_does_not_quarantine`, `pathological_timeout_cleanup_still_quarantines`) + HEAD assertions in `runtime_conformance.rs`.
- verify: ALL PASS — 100 Rust (53 engine, 12 runtime) + 35 TS tests.

## 2026-08-19 (M2.2.1-r4.2.1 cleanup budget unification & encapsulation)

- **Drain-local interrupt status** — `drain_jobs_for` returns `DrainReport { outcome, interrupted }`; `interrupted.swap(false)` inside the drain guarantees zero leakage across requests (request B's interrupted cleanup never misclassifies interleaved request A as a timeout).
- **All invocation cleanup uses cleanup_budget** — `cleanup_budget(invocation_id)` with `SETTLEMENT_GRACE` (100ms) used for `cancel_invocation`, `expire_timeouts`, `Step::Failed`, `abort_floating_ops`, and `finish_resolved` floating-op cleanup. The 5s watchdog is reserved exclusively for shutdown.
- **Single-assignment settlement grace** — `grace_deadline` assigned at most once per drain; `pending_ops` reset unconditionally uses `swap(0, Ordering::SeqCst)`.
- **Encapsulated EngineHealth** — `WorkerShared` is `pub(crate)`; `EngineHealth` exposes `is_ready()` and `is_quarantined()` with lock-free atomic reads (`Ordering::Acquire`).
- **Evidence** — 5 new tests in `engine.rs` (`cleanup_interrupt_does_not_timeout_unrelated_invocation`, `post_settlement_floating_cleanup_uses_cleanup_budget`, `failed_handler_cleanup_uses_cleanup_budget`, `promise_settlement_cleanup_uses_cleanup_budget`, `quarantine_accounting_drift_resets_pending_ops_to_zero`).
- verify: ALL PASS — 105 Rust (58 engine, 12 runtime) + 35 TS tests.

## 2026-08-19 (M2.2.1-r4.2.2 response-mapping budget & accounting parity)

- **Response mapping under the route deadline (P0)** — `InterruptDeadlineScope` RAII guard keeps the interrupt deadline armed through ALL synchronous JS work: handler call, watch attachment, response conversion (`value_to_outcome` may run user JS via toJSON/getters/proxy traps), and error extraction. `finish_resolved` pre-checks the owner's deadline (expired → deterministic Timeout) and arms it around Promise-result conversion. 7 new tests prove sync/async toJSON/getter spins are deadline-killed, mapping microtasks stay with their owner, the worker stays reusable, and problem-object getters are bounded.
- **Watchdog is shutdown-only (P0)** — getter/toJSON reaction jobs drain under the owning invocation's budget; message-boundary ownerless leftovers use `cleanup_budget(0)` (100 ms); the 5 s watchdog's sole remaining call site is the shutdown drain.
- **pending_ops.swap(0) actually landed (P1)** — quarantine uses `pending_ops.swap(0, SeqCst)`; drift recorded as boundary violation; drift-injection unit test in the worker module proves a zero terminal gauge with no unsigned wrap.
- **Evidence parity** — report now has exactly one "Verification (current)" section (r4.1 count relabeled historical); claims match the packaged source.
- verify: ALL PASS — 113 Rust (65 engine + 1 worker unit, 12 runtime, 35 other) + 35 TS tests.

## 2026-08-19 (M2.3-r2 Execution Graph Closure, Automaton Router & Verified Evidence)

- **Decoupled EngineLoadPlan (P0)** — Introduced `EngineLoadPlan::{Numeric{count}, Legacy{expected_handlers}}`. Numeric mode loads and checks exact function vector length directly without legacy `handlerTable` or BTreeMap allocations.
- **Exact Bidirectional RoutePlan Equivalence (P0)** — `QPack::verify()` enforces strict bidirectional status set equality (`declared_statuses == planned_statuses`), `default_status ∈ declared_statuses`, status ranges `100..=599` (no 0, no duplicates), exact `deadline_ms`, `response_strategy`, `FieldNeeds`, and exact `SchemaId` mapping.
- **Execution Graph Integrity** — `routes_canonical_json` and `routes_canonical_sha256` now include `functions: &[FunctionDecl]`, cryptographically binding the function manifest to the deployment pack integrity hash.
- **In-Memory Terminal Router Automaton (M2.3)** — Replaced candidate-scanning router in `q-router` with a Trie/Automaton of `RouterNode`s carrying `Terminal` method masks (`u16`) and `route_by_method: [Option<usize>; 7]`. Matches in a single traversal and derives 405 `Allow` headers in $O(1)$ from method masks without repeat full-route scans.
- **Numeric Pipeline Completeness** — Active `RouteId`, `PolicyId`, `SchemaId` wired into `InvocationSpec` and `CompiledRoute`. `schemaManifest` emitted by compiler with dense numeric `SchemaId`s.
- **Terminal Cleanup Hardening** — `finish_timeout` clears settlement entries before and after floating-op rejection cleanup. `quarantine_runtime` wholesale clears the settlement table. Added test `interrupted_watched_chain_retention_is_zero`.
- **Master Benchmark Suite Regenerated** — Full benchmark run completed (`bun run benchmark:all`): bridge, cold-start (1,680 samples), route-count scaling (40 samples, 25 vs 1,000 routes), warm load (10s fixed duration, c=1, 10, 50 across all candidates), and TypeScript scale.
- verify: ALL PASS (M0–M2 + M2.2.1 + M2.3 verified) — 137 Rust tests (79 engine, 12 runtime, 46 other) + 35 TS tests.


