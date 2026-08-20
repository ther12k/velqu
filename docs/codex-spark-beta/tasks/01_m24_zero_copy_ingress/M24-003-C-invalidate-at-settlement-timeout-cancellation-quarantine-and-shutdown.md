---
task_id: M24-003-C
parent_task: M24-003
milestone: M24
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M24.md
commit_required: true
---

# M24-003-C — Invalidate at settlement, timeout, cancellation, quarantine, and shutdown

## Atomic goal

Invalidate at settlement, timeout, cancellation, quarantine, and shutdown.

## Parent intent

Eliminate the global request-store mutex and keep lazy request access worker-owned.

## Dependencies

- `M24-003-B` — `tasks/01_m24_zero_copy_ingress/M24-003-B-use-slot-plus-generation-handles.md`

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
5. Implement exactly this deliverable: Invalidate at settlement, timeout, cancellation, quarantine, and shutdown.
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
m24-003-c: invalidate at settlement timeout cancellation quarantine and
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record

- Status: **PASS**
- Deliverable: single-owner terminal settlement across every path. `WorkerInner::settle_request` is now the only routine that invalidates a per-handle capability (completion, failure, timeout, cancellation, quarantine of a pending entry — all existing call sites route through it; the store's settle is called nowhere else in the worker). `q_bridge::RequestStore::settle_all` adds a worker-owned, capacity-bounded terminal sweep invoked at two paths: `quarantine_runtime` (after failing pending entries — catches Active slots no pending entry tracks, e.g. the invocation executing when the poison fired) and the worker-loop shutdown epilogue (after draining pending and aborting native tasks). Both sweeps are idempotent against per-handle settles via the generation check, so double settlement cannot double-decrement `live_slots`. On the host side, `serve.rs` gains a `CancelOnDrop` guard: if the pipeline future is dropped before the engine replies (client disconnect aborts the response future, or the outer deadline fires), `Engine::cancel` is delivered so the worker settles the slot and aborts its native operations exactly once; normal completion disarms the guard. This replaces the previous comment-only disconnect intent.
- Changed files:
  - `crates/q-bridge/src/lib.rs` (settle_all bounded terminal sweep + idempotence test)
  - `crates/q-engine-quickjs/src/worker.rs` (settle_request single owner; quarantine_runtime and shutdown epilogue call settle_all; all terminal sites routed)
  - `crates/q-runtime/src/serve.rs` (CancelOnDrop disconnect/outer-timeout cancellation guard, disarmed on completion)
  - `crates/q-engine-quickjs/tests/engine.rs` (ADR-0021 T7 quarantine-sweep and T8 shutdown-sweep proofs)
  - `docs/codex-spark-beta/tasks/01_m24_zero_copy_ingress/M24-003-C-invalidate-at-settlement-timeout-cancellation-quarantine-and-shutdown.md`, `docs/codex-spark-beta/STATUS.md`, `docs/codex-spark-beta/indexes/TASK_INDEX.md`
- Tests: new `settle_all_is_bounded_and_idempotent_with_handle_settles` (q-bridge: sweep settles remaining Active slots exactly once; repeated sweep and late per-handle settles are no-ops; settled slots reuse with fresh generations), `quarantine_settles_slots_without_pending_entries` (T7: an orphan admitted slot with no pending entry settles when the runtime quarantines — zero live slots), `shutdown_settles_all_remaining_slots` (T8: after worker join, zero live slots). Existing terminal-path proofs stay green: `deadline_timeout_interrupts_and_replies` (T5, pending_ops and live_slots zero), `cancellation_before_completion`, `completion_wins_abort_race_without_double_count`, `abort_actually_wins_completion_race` (T4), `cleanup_poison_aborts_all_native_ops_and_zeroes_pending_ops`, `shutdown_aborts_all_native_tasks`, `client_abort_leaves_server_healthy`, `graceful_shutdown_exits_zero`.
- Verification: `cargo test -p q-engine-quickjs` PASS (1 unit + 89 engine); `cargo test -p q-http` PASS (2 + 3); `cargo test -p q-bridge` PASS (8); `cargo test -p velqu-runtime` PASS (13 conformance); `cargo fmt --check` PASS; `cargo clippy --workspace --all-targets -- -D warnings` PASS. Raw logs: `/tmp/m24-003-c-engine.log`, `/tmp/m24-003-c-http.log`, `/tmp/m24-003-c-bridge.log`, `/tmp/m24-003-c-runtime.log`, `/tmp/m24-003-c-clippy.log`.
- Acceptance criteria proven: no process-wide request-store mutex on any access path; stale handles always fail; no request slot leaks after terminal paths (per-handle settles + bounded sweeps at quarantine and shutdown, asserted by T7/T8 and the existing live_slots==0 assertions); no JS value or request slot crosses worker ownership.
- Evidence boundary: the black-box runtime conformance suite cannot read bridge counters of the spawned binary process; disconnect settlement is proven at the engine boundary (cancel settles slots, `cancellation_before_completion`) plus the guard wiring in `serve.rs`, while `client_abort_leaves_server_healthy` proves the server stays functional after a mid-flight disconnect.
- Remaining risk / deferred by design: dedicated foreign-worker error variant and cross-worker corpus land in M24-003-D.
- Next dependency-ready task: M24-003-D (reject stale or cross-worker handles deterministically).
