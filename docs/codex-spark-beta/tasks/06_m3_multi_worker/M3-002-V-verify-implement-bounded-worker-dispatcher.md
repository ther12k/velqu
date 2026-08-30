---
task_id: M3-002-V
parent_task: M3-002
milestone: M3
priority: P0
mode: VERIFY
status: PASS
context_card: context/milestones/M3.md
commit_required: true
---

# M3-002-V — Verify Implement bounded worker dispatcher

## Atomic goal

Prove every acceptance criterion for parent task M3-002 without broadening scope.

## Parent intent

Route matched requests to workers without unbounded queues or shared engine mutexes.

## Dependencies

- `M3-002-A` — `tasks/06_m3_multi_worker/M3-002-A-use-bounded-per-worker-queues.md`
- `M3-002-B` — `tasks/06_m3_multi_worker/M3-002-B-select-worker-using-outstanding-load-strategy.md`
- `M3-002-C` — `tasks/06_m3_multi_worker/M3-002-C-define-admission-and-overload-response.md`
- `M3-002-D` — `tasks/06_m3_multi_worker/M3-002-D-preserve-routeid-routeplan-before-dispatch.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M3.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`
- `context/components/multiworker.md`

### Source files

- `AGENTS.md`
- `crates/q-runtime/src/main.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-engine-quickjs/src/lib.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`
- `crates/q-http/src/lib.rs`
- `crates/q-bridge/src/lib.rs`
- `crates/q-runtime/tests/runtime_conformance.rs`
- `benchmarks/harness/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- Queue capacity is configurable and bounded.
- Overload fails quickly and observably.
- No head-of-line lock across workers.
- Per-worker queue latency is measured.

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

- Dispatcher tests.
- Overload load test.
- Metrics.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m3-002-v: verify implement bounded worker dispatcher
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M3-002-V) — PASS

- Date: 2026-08-30
- Branch/PR: m3-002-v (squash-merged; see git log for final hash)
- Closes: #382

### Acceptance-criterion mapping (parent M3-002 guardrails)

1. **Queue capacity is configurable and bounded** — verified: `BoundedWorkerQueue::with_capacity` clamps to [1, MAX_WORKER_QUEUE_CAPACITY=65536], default 256. Tests: `capacity_is_bounded_and_clamped` (M3-002-A).
2. **Overload fails quickly and observably** — verified: `try_push` rejects IMMEDIATELY with typed `QueueError::Full` (per-worker) / `AllFull` (global); saturating rejection counters; `admission_response` maps every variant to exactly one 503/overload/retry-1 verdict matching the runtime RFC 9457 registry. Tests: `overflow_fails_fast_with_typed_error_and_counts`, `overload_burst_is_rejected_fast_and_fully_counted` (10k vs 128), `admission_response_is_total_deterministic_and_redacted`, `admission_verdict_composes_with_dispatcher_overload` (A, C).
3. **No head-of-line lock across workers** — verified three ways: per-worker queues (a jammed full worker A while B flows: `no_head_of_line_lock_across_workers`); selection consults lengths only and skips full queues (`full_queues_are_skipped_until_only_choice`, `selection_targets_least_outstanding_load`); the dispatch job carries its resolved plan as Copy data so no post-dispatch matcher/route-table access exists (`route_identity_survives_the_dispatch_queue_boundary`, D).
4. **Per-worker queue latency is measured** — verified: per-item wait measured at pop; mean/max in the redacted `QueueStats`; aggregated per worker via `Dispatcher::stats`. Tests: `fifo_order_and_wait_measurement`, `aggregated_stats_cover_every_worker` (A, B).

### Verification runs (this branch, worktree-fresh)
- `cargo test -p q-capabilities` → 6 suites pass (210 unit incl. 16 dispatch tests)
- `cargo test -p velqu-runtime` → 17+5+44 (incl. 4 DispatchRoute boundary tests)
- `cargo test -p q-engine-quickjs` → 20+101; `-p q-http` 4+6+1; `-p q-bridge` 11 — all pass
- `bun test` → 0 fail; `bun run typecheck` → clean (via ./scripts/verify)
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**; release binary reproduced deterministically (`333d563d…`)

### Disclosures (standing)
- No production code changed in this packet: verification-only closure of M3-002-A/B/C/D.
- CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR. Local evidence above is complete.
