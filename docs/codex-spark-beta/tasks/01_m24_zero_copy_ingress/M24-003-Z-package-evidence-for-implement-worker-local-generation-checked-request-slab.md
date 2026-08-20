---
task_id: M24-003-Z
parent_task: M24-003
milestone: M24
priority: P0
mode: EVIDENCE
status: PASS
context_card: context/milestones/M24.md
commit_required: true
---

# M24-003-Z — Package evidence for Implement worker-local generation-checked request slab

## Atomic goal

Create source-backed evidence and handoff for parent task M24-003; update status only if verification passed.

## Parent intent

Eliminate the global request-store mutex and keep lazy request access worker-owned.

## Dependencies

- `M24-003-V` — `tasks/01_m24_zero_copy_ingress/M24-003-V-verify-implement-worker-local-generation-checked-request-slab.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M24.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`
- `context/components/evidence.md`

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
- `docs/reports/`
- `docs/beta/workstreams/OBSERVABILITY_OPERATIONS.md`
- `conformance/security/security.conformance.test.ts`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

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
```bash
bun test
```
```bash
bun run typecheck
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

- Race tests.
- Slot accounting metrics.
- Fuzzed stale-handle operations.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m24-003-z: package evidence for implement worker local generation check
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record

- Status: **PASS**
- Deliverable: source-backed evidence package for parent task M24-003 (worker-local generation-checked request slab). Implementation commits: `b5db596` (M24-003-A, PR #647 — worker-local bounded slab, request moves in `InvocationSpec.request`, typed `RequestCapacity` → 503 overload + `Retry-After: 1`, slab capacity wired to the HTTP `max_queue` admission bound), `9506998` (M24-003-B, PR #648 — typed private-field `RequestHandle {worker_id, slot, generation}` minted only by the owning slab, worker-identity check before slot lookup, `PendingInvocation` carries the handle), `11c2ecc` (M24-003-C, PR #649 — single settlement owner `settle_request`, worker-owned `settle_all` sweeps at quarantine and shutdown, runtime `CancelOnDrop` disconnect/outer-timeout cancellation guard), `6bb547c` (M24-003-D, PR #650 — dedicated `BridgeError::ForeignWorker` denial, 2048-triple LCG fuzz corpus, T11 cross-worker inertness across two live engines). Verification commit: `f1c46f2` (M24-003-V, PR #651 — guardrail-to-source/test mapping, ADR-0021 T1–T12 coverage).
- Exact changed files (implementation scope): `crates/q-engine/src/lib.rs`, `crates/q-bridge/src/lib.rs`, `crates/q-bridge/Cargo.toml`, `crates/q-engine-quickjs/src/lib.rs`, `crates/q-engine-quickjs/src/worker.rs`, `crates/q-engine-quickjs/tests/engine.rs`, `crates/q-runtime/src/main.rs`, `crates/q-runtime/src/serve.rs`, `crates/q-runtime/src/problems.rs`, `crates/q-bench-support/src/bin/bridge_bench.rs`, `crates/q-bench-support/tests/timer_repro.rs`, plus packet/status/index documents.
- Evidence index (key tests): q-bridge 9 — `bounded_slab_rejects_growth`, `settle_all_is_bounded_and_idempotent_with_handle_settles`, `access_materializes_and_counts`, `settlement_expires_handle_and_reuse_is_isolated`, `typed_handle_from_foreign_worker_is_denied_before_slot_lookup`, `fuzzed_handle_triples_fail_closed_without_side_effects`, `stale_handle_corpus_never_reads_or_leaks`, `unread_request_costs_nothing`, `double_settle_is_idempotent`; engine 90 — `worker_local_slab_capacity_is_bounded`, `incoming_capability_pair_is_overwritten_by_worker_handle`, `quarantine_settles_slots_without_pending_entries` (T7), `shutdown_settles_all_remaining_slots` (T8), `cross_worker_handle_is_inert_on_foreign_worker` (T11), `field_free_invocation_skips_request_store_slot`, `expired_handle_access_fails_deterministically`, `deadline_timeout_interrupts_and_replies` (T5), `completion_wins_abort_race_without_double_count` / `abort_actually_wins_completion_race` (T4), `cancellation_before_completion`, `shutdown_aborts_all_native_tasks`; runtime conformance 13 — `routing_precedes_body_materialization`, `queue_limit_returns_503_when_saturated`, `body_and_header_limits_reject_oversize`, `client_abort_leaves_server_healthy`, `graceful_shutdown_exits_zero`, `poisoned_runtime_marks_readiness_false`, `full_runtime_conformance`.
- Exact command results (this worktree): `cargo test -p q-engine-quickjs` PASS (1 unit + 90 engine); `cargo test -p q-http` PASS (2 unit + 3 parser fuzz); `cargo test -p q-bridge` PASS (9); `cargo test -p velqu-runtime` PASS (13 conformance); `cargo fmt --check` PASS; `cargo clippy --workspace --all-targets -- -D warnings` PASS; `bun run typecheck` PASS; `bun test` stage PASS within `./scripts/verify` (V recorded 35 pass / 0 fail across 9 files); `./scripts/validate-okf` PASS (174 links, 0 errors). Raw logs: `/tmp/m24-003-z-rust.log`, `/tmp/m24-003-z-type.log`, `/tmp/m24-003-z-verify.log`, and the V-run logs `/tmp/m24-003-v-*.log`.
- Scoped verification limitation (identical to M24-002-V/M24-003-V, honestly recorded, canonical manifests untouched): `./scripts/verify` exits 1 on the single stage `validate-benchmark-evidence` — fresh-worktree stage ordering reports missing artifacts on the first pass; after the artifacts are built, the worktree release binary hash differs from the canonical `qRuntimeRelease` manifest hash. No benchmark manifest or performance claim changed; `benchmark reports are current` passed; all other stages (OKF, production plan, fmt, clippy, workspace tests, release builds, raw-rust baseline, bun install/typecheck, proof build, bun test) passed.
- Evidence boundary: `BridgeCounters` snapshots are used only for laziness/slot-accounting proof. Aggregate ingress counters, stage histograms, and instrumentation-overhead benchmarks remain M24-009 deliverables and are not claimed here. M3 multi-worker instantiation exercises the same ownership rules over N workers.
- Next dependency-ready tasks: M24-004-A (store capture start/end ranges) and M24-005-A (compile header-name IDs into RoutePlan) unblock on this packet.

