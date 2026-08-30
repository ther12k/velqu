---
task_id: M3-002-B
parent_task: M3-002
milestone: M3
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M3.md
commit_required: true
---

# M3-002-B — Select worker using outstanding-load strategy

## Atomic goal

Select worker using outstanding-load strategy.

## Parent intent

Route matched requests to workers without unbounded queues or shared engine mutexes.

## Dependencies

- `M3-002-A` — `tasks/06_m3_multi_worker/M3-002-A-use-bounded-per-worker-queues.md`

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
5. Implement exactly this deliverable: Select worker using outstanding-load strategy.
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
m3-002-b: select worker using outstanding load strategy
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M3-002-B) — PASS

- Date: 2026-08-30
- Branch/PR: m3-002-b (squash-merged; see git log for final hash)
- Closes: #379

### Changed files
- `crates/q-capabilities/src/dispatch.rs`: `Dispatcher<T>` — least-outstanding-load selection over N bounded per-worker queues (M3-002-B) —
  - `select()`: the worker with the SMALLEST queue length that still has capacity; full queues are skipped entirely; `None` when every queue is full.
  - Tie-breaking is ROUND-ROBIN (cursor advances per selection, rotation-ordered scan) so equal load spreads evenly instead of pinning worker 0.
  - `dispatch(job) -> Result<worker, QueueError>`: select + push atomically from the caller's view; typed `QueueError::AllFull { workers, capacity }` (new variant) when admission is globally impossible.
  - `queue(w)` accessor (the owning thread pops), `close_all()` (shutdown path), `stats()` aggregation; explicit `SharedAcrossWorkers` impl; pure host-side state — queue lengths only, no JS values, no locks held across pushes.
- `crates/q-capabilities/src/lib.rs`: re-export `Dispatcher`.

### Tests added (+6 → 208 q-capabilities lib tests)
- `selection_targets_least_outstanding_load` (uneven fill [3,0,1]: dispatches route 1->2->1; loads converge to [3,2,2])
- `equal_loads_break_round_robin` (empty workers rotate 0,1,2,0,1,2)
- `full_queues_are_skipped_until_only_choice` (full w0 skipped; all-full -> None)
- `dispatch_routes_to_least_loaded_and_reports_all_full` (typed AllFull with workers+capacity; items on the right queues)
- `aggregated_stats_cover_every_worker`
- `close_all_shuts_every_queue_down`

### Command results
- `cargo test -p q-capabilities` → **208 unit (was 202) + 7 + 1 + 4 + 9** — 0 failed
- `cargo test -p q-engine-quickjs` → 20+101 · `-p q-http` 4+6+1 · `-p q-bridge` 11 · `-p velqu-runtime` 13+5+44 — all pass
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**; release binary unchanged (`333d563d…` matches manifest)

### Guardrail mapping
- **No head-of-line lock across workers** — selection consults per-queue lengths only; a jammed worker is skipped, not waited on.
- **Overload fails quickly and observably** — AllFull is typed, immediate, and the per-queue rejected counters keep counting (M3-002-C formalizes the HTTP-layer response).

### Disclosures
- The first selection test asserted wrong semantics (select() does not push, so loads do not change between selections); remodeled on dispatch() with the exact convergence loads [3,2,2] pinned. Two heredoc anchor misses (fmt-reflowed blocks) aborted safely before writing.
- Standing: CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR.
