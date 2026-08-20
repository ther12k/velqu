---
task_id: M24-003-A
parent_task: M24-003
milestone: M24
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M24.md
commit_required: true
---

# M24-003-A — Move request slots into each QuickJS worker

## Atomic goal

Move request slots into each QuickJS worker.

## Parent intent

Eliminate the global request-store mutex and keep lazy request access worker-owned.

## Dependencies

- `M24-001-Z` — `tasks/01_m24_zero_copy_ingress/M24-001-Z-package-evidence-for-freeze-ingress-ownership-and-backpressure-design.md`
- `M24-002-Z` — `tasks/01_m24_zero_copy_ingress/M24-002-Z-package-evidence-for-route-before-request-materialization.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M24.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`

### Source files

- `AGENTS.md`
- `crates/q-http/src/lib.rs`
- `crates/q-bridge/src/lib.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`
- `crates/q-runtime/tests/runtime_conformance.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Move request slots into each QuickJS worker.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- No process-wide request-store mutex on normal access.
- Stale handles always fail.
- No request slot leaks after terminal paths.
- No JS value or request slot crosses worker ownership.

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

- Race tests.
- Slot accounting metrics.
- Fuzzed stale-handle operations.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m24-003-a: move request slots into each quickjs worker
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record

- Status: **PASS**
- Deliverable: worker-local bounded request slab. `q-bridge::RequestStore` is no longer a process-wide `Mutex<Vec<Slot>>`; it is a single-threaded, capacity-bounded slab (`RefCell` state, worker-local generation clock) owned by each QuickJS worker via `Rc<RequestStore>`. Request data now moves into the worker inside `InvocationSpec.request: Option<RequestMeta>` (`q_engine::RequestMeta`), the worker is the only allocator of slots/generations, and the host no longer holds or inserts request bytes (`ServeState.store` and the `QuickJsEngine::spawn` store argument are gone). Slab capacity comes from `QuickJsConfig::request_slot_capacity`, set by the runtime to the HTTP `max_queue` admission bound; exhaustion is the typed `Outcome::RequestCapacity`, mapped to the spec §9.1 `overload` problem (503 + `Retry-After: 1`; registry entry added, matching the URN q-http already emits). Only read-only atomic counters (`bridge_snapshot`) cross back to the engine handle.
- Changed files:
  - `crates/q-engine/src/lib.rs` (RequestMeta moved to the engine boundary; InvocationSpec.request; Outcome::RequestCapacity)
  - `crates/q-bridge/src/lib.rs` (worker-local bounded slab, try_insert + BridgeError::Capacity, Arc<BridgeCounters> sharing, stale-handle corpus test)
  - `crates/q-bridge/Cargo.toml` (q-engine dependency for the shared RequestMeta type)
  - `crates/q-engine-quickjs/src/lib.rs` (spawn without store; request_slot_capacity; bridge_snapshot; test-only insert_request/settle_request worker messages)
  - `crates/q-engine-quickjs/src/worker.rs` (Rc<RequestStore> owned by WorkerInner; slot allocation in begin_invocation; InsertRequest/SettleRequest messages; RequestCapacity outcome)
  - `crates/q-engine-quickjs/tests/engine.rs` (fixtures moved to worker admission; new capacity test; live-slot assertions via bridge_snapshot)
  - `crates/q-runtime/src/main.rs` (slab capacity wired to limits.max_queue; no store)
  - `crates/q-runtime/src/serve.rs` (RequestMeta moves via spec; RequestCapacity → 503 overload + Retry-After)
  - `crates/q-runtime/src/problems.rs` (overload registry entry per spec §9.1)
  - `crates/q-bench-support/src/bin/bridge_bench.rs`, `crates/q-bench-support/tests/timer_repro.rs` (worker admission migration)
  - `docs/codex-spark-beta/tasks/01_m24_zero_copy_ingress/M24-003-A-move-request-slots-into-each-quickjs-worker.md`, `docs/codex-spark-beta/STATUS.md`, `docs/codex-spark-beta/indexes/TASK_INDEX.md`
- Tests: new `bounded_slab_rejects_growth` and `stale_handle_corpus_never_reads_or_leaks` (q-bridge: arbitrary stale slot/generation pairs never materialize bytes or leak slots) and `worker_local_slab_capacity_is_bounded` (q-engine-quickjs: capacity-1 worker rejects a second in-flight request with `Outcome::RequestCapacity`, settles to 0 live slots after cancel). Behavior preserved by the unchanged suites, now asserting against the worker-local slab: engine 86/86 including `completion_wins_abort_race_without_double_count`, `abort_actually_wins_completion_race`, `expired_handle_access_fails_deterministically`, `shutdown_aborts_all_native_tasks`, `field_free_invocation_skips_request_store_slot`; q-bridge 6/6; q-http 2 unit + 3 parser fuzz; velqu-runtime 13/13 runtime-conformance including `queue_limit_returns_503_when_saturated`, `client_abort_leaves_server_healthy`, `graceful_shutdown_exits_zero`; q-bench-support `timer_promise_in_bench_context`.
- Verification: `cargo test -p q-engine-quickjs` PASS (1 unit + 86 engine); `cargo test -p q-http` PASS (2 + 3); `cargo test -p q-bridge` PASS (6); `cargo test -p velqu-runtime` PASS (13 conformance); `cargo fmt --check` PASS; `cargo clippy --workspace --all-targets -- -D warnings` PASS. Raw logs: `/tmp/m24-003-a-q-engine-quickjs.log`, `/tmp/m24-003-a-q-http.log`, `/tmp/m24-003-a-q-bridge.log`, `/tmp/m24-003-a-velqu-runtime.log`, `/tmp/m24-003-a-fmt.log`, `/tmp/m24-003-a-clippy.log`.
- Acceptance criteria proven: no process-wide request-store mutex exists on any access path (the only RequestStore is worker-thread-confined); stale handles fail deterministically (corpus + reuse-isolation tests); terminal paths leave zero live slots (engine suite asserts live_slots == 0 after completion, timeout, cancel, poison, shutdown); no JS value or request slot crosses worker ownership (metadata moves by value into the single worker; only scalar counters return).
- Dependency note: the declared dependency M24-001-Z remains TODO by instruction; this packet lands under the approved dependency-ordered plan (same precedent as M24-002-A..D), with M24-001-V/Z re-verification scheduled after M24-003 evidence exists.
- Remaining risk / deferred by design: raw `slot`/`generation` fields remain on InvocationSpec until the typed handle lands in M24-003-B; single-settlement-owner consolidation is M24-003-C; cross-worker handle rejection with a dedicated error is M24-003-D. Aggregate ingress metrics remain M24-009 scope.
- Next dependency-ready task: M24-003-B (use slot plus generation handles).
