---
task_id: M3-002-A
parent_task: M3-002
milestone: M3
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M3.md
commit_required: true
---

# M3-002-A — Use bounded per-worker queues

## Atomic goal

Use bounded per-worker queues.

## Parent intent

Route matched requests to workers without unbounded queues or shared engine mutexes.

## Dependencies

- `M3-001-Z` — `tasks/06_m3_multi_worker/M3-001-Z-package-evidence-for-freeze-independent-worker-state-semantics.md`

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

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Use bounded per-worker queues.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

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

## Required evidence for this microtask

- Dispatcher tests.
- Overload load test.
- Metrics.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m3-002-a: use bounded per worker queues
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M3-002-A) — PASS

- Date: 2026-08-30
- Branch/PR: m3-002-a (squash-merged; see git log for final hash)
- Closes: #378

### Changed files
- `crates/q-capabilities/src/dispatch.rs` (new): `BoundedWorkerQueue<T>` — the per-worker dispatch queue primitive (ADR-0036 §4 queue discipline) —
  - bounded by construction: capacity configurable per queue, clamped to [1, MAX_WORKER_QUEUE_CAPACITY=65536]; DEFAULT=256.
  - `try_push` — IMMEDIATE typed rejection `QueueError::Full { worker, len, capacity }` at capacity: overload fails fast (never blocks a producer) and observably (rejections counted).
  - `pop` (blocking, wakes on push/close) and `pop_timeout` for the owning worker thread; FIFO order; per-item queue-wait measured at pop.
  - `close()` idempotent: consumer drains remaining items then receives None promptly.
  - `stats() -> QueueStats`: redacted snapshot (len/capacity/pushed/popped/rejected/mean_wait/max_wait — counters saturate; no job payloads).
  - `T: Send` bound keeps JS values out by construction; `impl SharedAcrossWorkers` (explicit marker: auditable sharing decision).
- `crates/q-capabilities/src/lib.rs`: module + re-exports.

### Tests added (dispatch.rs, +8 → 202 q-capabilities lib tests)
- `capacity_is_bounded_and_clamped` (config range: floor 1, default 256, ceiling clamp)
- `overflow_fails_fast_with_typed_error_and_counts` (typed Full error; <5ms rejection; counters)
- `fifo_order_and_wait_measurement` (order + measured wait >= 10ms after sleep)
- `no_head_of_line_lock_across_workers` (worker A jammed+full; worker B flows freely)
- `closed_queue_drains_then_returns_none` (drain then prompt None)
- `blocking_pop_wakes_on_push_and_close` (condvar wake both directions)
- `stats_are_redacted_and_complete`
- `overload_burst_is_rejected_fast_and_fully_counted` (**overload load test**: 10k pushes vs capacity 128 — exactly 128 accepted, 9872 rejected immediately, all counted, <2s total)

### Command results
- `cargo test -p q-capabilities` → **202 unit (was 194) + 7 + 1 + 4 + 9** — 0 failed
- `cargo test -p q-engine-quickjs` → 20+101 · `-p q-http` 4+6+1 · `-p q-bridge` 11 · `-p velqu-runtime` 13+5+44 — all pass
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**; release binary unchanged (`333d563d…` matches manifest — primitive dormant until M3-002-B/C wire the dispatcher)

### Guardrail mapping
- **Queue capacity is configurable and bounded** — clamped config, hard ceiling.
- **Overload fails quickly and observably** — typed immediate rejection + saturating rejection counters + burst load test.
- **No head-of-line lock across workers** — per-worker queues; proven by the jammed-A/flowing-B test.
- **Per-worker queue latency is measured** — per-item wait at pop; mean/max in stats.

### Disclosures
- One shell-escaping slip wrote a spurious closing quote into a lifetime (`'static'`), producing confusing char-literal errors — hexdump diagnosis, one-character fix, caught before any commit.
- Standing: CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR.
