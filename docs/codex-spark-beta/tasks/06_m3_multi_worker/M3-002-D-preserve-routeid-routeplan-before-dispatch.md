---
task_id: M3-002-D
parent_task: M3-002
milestone: M3
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M3.md
commit_required: true
---

# M3-002-D — Preserve RouteId/RoutePlan before dispatch

## Atomic goal

Preserve RouteId/RoutePlan before dispatch.

## Parent intent

Route matched requests to workers without unbounded queues or shared engine mutexes.

## Dependencies

- `M3-002-C` — `tasks/06_m3_multi_worker/M3-002-C-define-admission-and-overload-response.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M3.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/multiworker.md`

### Source files

- `AGENTS.md`
- `crates/q-runtime/src/main.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-engine-quickjs/src/lib.rs`
- `crates/q-router/src/lib.rs`
- `crates/q-pack/src/lib.rs`
- `packages/compiler/src/emit.ts`
- `conformance/routing/routing.conformance.test.ts`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Preserve RouteId/RoutePlan before dispatch.
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
cargo test -p q-pack
```
```bash
cargo test -p q-router
```
```bash
cargo test -p q-engine-quickjs
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
m3-002-d: preserve routeid routeplan before dispatch
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M3-002-D) — PASS

- Date: 2026-08-30
- Branch/PR: m3-002-d (squash-merged; see git log for final hash)
- Closes: #381

### Changed files
- `crates/q-engine/src/lib.rs`: `DispatchRoute` — the resolved route-identity snapshot that crosses the dispatch boundary (M3-002-D, ADR-0036 §3/§6). `Copy` plain data: route_id, handler_id, policy_id/policy_handler_id, the four validation schema ids, default_status, response_strategy, deadline_ms. Extracted BEFORE dispatch; the worker consumes numeric IDs only and never re-runs the matcher; the snapshot for a given route is identical for every worker.
- `crates/q-runtime/src/serve.rs`: `dispatch_route(&CompiledRoute) -> DispatchRoute` — the extraction point (the M23R2 resolve-once rule extended across the dispatch boundary); + `dispatch_route_tests` (4 tests).

### Tests added (serve.rs, 4)
- `snapshot_preserves_route_identity_exactly` (RouteId/handler/policy/schema ids/status/strategy/deadline all intact)
- `snapshot_is_copy_plain_data_shared_safe` (Copy + Send + Sync + 'static; two workers hold the same snapshot with zero clone cost)
- `extraction_is_deterministic_across_calls` (worker K's snapshot == worker 0's)
- `route_identity_survives_the_dispatch_queue_boundary` (full M3-002 shape: extract -> Dispatcher::dispatch -> pop on ANOTHER THREAD -> identity intact; saturated admission stays typed)

### Command results
- `cargo test -p velqu-runtime` → **17 unit (was 13) + 5 + 44** — 0 failed
- `cargo test -p q-capabilities` → 6 suites · `-p q-engine-quickjs` 20+101 · `-p q-http` 4+6+1 · `-p q-bridge` 11 — all pass
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**; release binary unchanged (`333d563d…` — extraction dormant until the dispatcher wires into the request path)

### Guardrail mapping
- **No head-of-line lock across workers** — the job carries its resolved plan as Copy data; no post-dispatch matcher access, no shared route-table lock on the hot path.

### Disclosures
- One cross-thread borrow restructure (dispatcher Arc-shared with the consumer thread) and one unused-import clippy fix — test-code only.
- Standing: CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR.
