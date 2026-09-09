---
task_id: M24-003-V
parent_task: M24-003
milestone: M24
priority: P0
mode: VERIFY
status: PASS
context_card: context/milestones/M24.md
commit_required: true
---

# M24-003-V — Verify Implement worker-local generation-checked request slab

## Atomic goal

Prove every acceptance criterion for parent task M24-003 without broadening scope.

## Parent intent

Eliminate the global request-store mutex and keep lazy request access worker-owned.

## Dependencies

- `M24-003-A` — `tasks/01_m24_zero_copy_ingress/M24-003-A-move-request-slots-into-each-quickjs-worker.md`
- `M24-003-B` — `tasks/01_m24_zero_copy_ingress/M24-003-B-use-slot-plus-generation-handles.md`
- `M24-003-C` — `tasks/01_m24_zero_copy_ingress/M24-003-C-invalidate-at-settlement-timeout-cancellation-quarantine-and-shutdown.md`
- `M24-003-D` — `tasks/01_m24_zero_copy_ingress/M24-003-D-reject-stale-or-cross-worker-handles-deterministically.md`

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

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

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

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m24-003-v: verify implement worker local generation checked request sla
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record

- Status: **PASS**
- Deliverable: verification of the M24-003 A–D worker-local generation-checked request slab against the four parent guardrails. Implementation commits verified: `b5db596` (M24-003-A, PR #647), `9506998` (M24-003-B, PR #648), `11c2ecc` (M24-003-C, PR #649), `6bb547c` (M24-003-D, PR #650).
- Guardrail mapping (criterion → source → positive/negative tests):
  - **No process-wide request-store mutex on normal access.** `q_bridge::RequestStore` is single-threaded `RefCell` state (structurally non-`Sync`), owned by the QuickJS worker as `Rc<RequestStore>` (`crates/q-engine-quickjs/src/worker.rs`); `ServeState.store` and the `QuickJsEngine::spawn` store argument are gone (`crates/q-runtime/src/serve.rs`, `crates/q-engine-quickjs/src/lib.rs`). Proven by every suite executing through the worker-owned slab (engine 90/90, bridge 9/9, runtime conformance 13/13) — cross-task sharing would not compile.
  - **Stale handles always fail.** Negative tests: `settlement_expires_handle_and_reuse_is_isolated`, `stale_handle_corpus_never_reads_or_leaks`, `fuzzed_handle_triples_fail_closed_without_side_effects` (2048 LCG triples), `expired_handle_access_fails_deterministically` (engine, JS-side), `typed_handle_from_foreign_worker_is_denied_before_slot_lookup` (dedicated `ForeignWorker` denial with a decoy live slot at the same index; zero materialization on denial).
  - **No request slot leaks after terminal paths.** ADR-0021 T4/T5/T7/T8: `completion_wins_abort_race_without_double_count`, `abort_actually_wins_completion_race`, `deadline_timeout_interrupts_and_replies`, `quarantine_settles_slots_without_pending_entries`, `shutdown_settles_all_remaining_slots`, `cancellation_before_completion`, plus `settle_all_is_bounded_and_idempotent_with_handle_settles` and live_slots==0 assertions across the engine suite; runtime-level `client_abort_leaves_server_healthy`, `graceful_shutdown_exits_zero`, `queue_limit_returns_503_when_saturated`.
  - **No JS value or request slot crosses worker ownership.** `RequestMeta` moves by value inside `InvocationSpec.request`; only scalar `CountersSnapshot` atomics cross the seam; handles are worker-stamped and JS carries only the numeric pair. Tests: `cross_worker_handle_is_inert_on_foreign_worker` (T11, two live engines), `incoming_capability_pair_is_overwritten_by_worker_handle`, `worker_local_slab_capacity_is_bounded` (typed `RequestCapacity` → 503 overload + `Retry-After`).
- Required evidence: race tests (completion/abort race pair + floating-race suite), slot accounting metrics (`BridgeCounters` live_slots/host_calls/materialized_fields assertions), fuzzed stale-handle operations (bounded LCG corpus).
- Exact command results: `cargo test -p q-engine-quickjs` PASS (1 unit + 90 engine); `cargo test -p q-http` PASS (2 unit + 3 parser fuzz); `cargo test -p q-bridge` PASS (9); `cargo test -p velqu-runtime` PASS (13 conformance); `cargo fmt --check` PASS; `cargo clippy --workspace --all-targets -- -D warnings` PASS; `bun test packages examples/proof conformance` PASS (35 pass / 0 fail, 9 files); `./scripts/validate-okf` PASS (174 links, 0 errors). Raw logs: `/tmp/m24-003-v-rust.log`, `/tmp/m24-003-v-verify.log`, `/tmp/m24-003-v-bun.log`.
- Scoped verification limitation (reported honestly, no manifest rewritten): `./scripts/verify` exits 1 with a single failing stage, `validate-benchmark-evidence`. In this fresh worktree the validator runs before the release/proof builds that produce its artifacts (stage ordering), reporting missing artifacts on the first pass; after the artifacts exist it reports `hash mismatch for qRuntimeRelease` because the worktree-built release binary differs from the hash recorded in the canonical benchmark manifest (same limitation recorded by M24-002-V). The canonical manifest and every performance claim are unchanged; `benchmark reports are current` passed; all other verify stages (OKF, production plan, fmt, clippy, workspace tests, release builds, raw-rust baseline, bun install/typecheck, proof build, bun test) completed successfully.
- Remaining risk / deferred by design: M3 instantiates N workers over the same rules; aggregate ingress metrics remain M24-009 scope.
- Next dependency-ready task: M24-003-Z (package evidence for Implement worker-local generation-checked request slab).

