---
task_id: M24-003-B
parent_task: M24-003
milestone: M24
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M24.md
commit_required: true
---

# M24-003-B — Use slot plus generation handles

## Atomic goal

Use slot plus generation handles.

## Parent intent

Eliminate the global request-store mutex and keep lazy request access worker-owned.

## Dependencies

- `M24-003-A` — `tasks/01_m24_zero_copy_ingress/M24-003-A-move-request-slots-into-each-quickjs-worker.md`

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
5. Implement exactly this deliverable: Use slot plus generation handles.
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
m24-003-b: use slot plus generation handles
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record

- Status: **PASS**
- Deliverable: typed slot-plus-generation capabilities. `q_bridge::RequestHandle` is a private-field capability `{worker_id, slot, generation}` minted only by the owning slab (`try_insert`) or reconstructed for this worker's numeric JS pair (`local_handle`); every slab draws a distinct `worker_id` from a monotonic clock. All store APIs are handle-typed (`insert`/`try_insert -> RequestHandle`, `settle(handle)`, `access(handle, ...)`), and worker identity is validated before any slot lookup — a handle minted by another worker's slab is denied without touching the destination slab. `WorkerInner` mints the handle at admission, overwrites whatever pair the spec carried, stores it in `PendingInvocation`, and uses it for every terminal settle; the JS ABI still exchanges only the numeric pair (call_runner and the native accessors reconstruct local handles). Requestless specs keep the `NO_REQUEST_SLOT` sentinel pair, whose settle/access remains a bounds-checked no-op.
- Changed files:
  - `crates/q-bridge/src/lib.rs` (RequestHandle type, worker-id clock, handle-typed store APIs, worker-identity pre-check, foreign-handle and stale-corpus tests)
  - `crates/q-engine-quickjs/src/worker.rs` (handle minted/overwritten in begin_invocation; PendingInvocation.handle; all terminal settles handle-typed; native accessors reconstruct local handles; InsertRequest/SettleRequest worker messages carry handles)
  - `crates/q-engine-quickjs/src/lib.rs` (insert_request/settle_request test helpers typed)
  - `crates/q-engine-quickjs/tests/engine.rs` (fixtures use typed handles; new forged-pair overwrite proof)
  - `docs/codex-spark-beta/tasks/01_m24_zero_copy_ingress/M24-003-B-use-slot-plus-generation-handles.md`, `docs/codex-spark-beta/STATUS.md`, `docs/codex-spark-beta/indexes/TASK_INDEX.md`
- Tests: new `typed_handle_from_foreign_worker_is_denied_before_slot_lookup` (q-bridge: worker A's minted handle denied by worker B's slab before slot lookup; wrong-worker settle touches neither slab; true owner still settles once) and `incoming_capability_pair_is_overwritten_by_worker_handle` (engine: spec carrying `slot=3, generation=u64::MAX` still serves lazy query fields because the worker-minted handle wins, and settles to 0 live slots). Existing proofs remain green: `settlement_expires_handle_and_reuse_is_isolated`, `stale_handle_corpus_never_reads_or_leaks`, `double_settle_is_idempotent`, `unread_request_costs_nothing`, `bounded_slab_rejects_growth`, `worker_local_slab_capacity_is_bounded`, `expired_handle_access_fails_deterministically`, `field_free_invocation_skips_request_store_slot`.
- Verification: `cargo test -p q-engine-quickjs` PASS (1 unit + 87 engine); `cargo test -p q-http` PASS (2 + 3); `cargo test -p q-bridge` PASS (7); `cargo test -p velqu-runtime` PASS (13 conformance); `cargo fmt --check` PASS; `cargo clippy --workspace --all-targets -- -D warnings` PASS.
- Acceptance criteria proven: no process-wide request-store mutex on any access path (unchanged from M24-003-A, all access now handle-typed); stale handles always fail (generation checks + corpus); terminal paths leave zero live slots (engine suite live_slots assertions); no JS value or request slot crosses worker ownership (handles are worker-stamped and checked; JS sees only the numeric pair).
- Remaining risk / deferred by design: a dedicated deterministic error variant and full cross-worker rejection corpus land in M24-003-D (a foreign handle currently maps to the existing deterministic `Expired` denial); single-settlement-owner consolidation is M24-003-C.
- Next dependency-ready task: M24-003-C (invalidate at settlement, timeout, cancellation, quarantine, and shutdown).
