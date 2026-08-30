---
task_id: M3-007-C
parent_task: M3-007
milestone: M3
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M3.md
commit_required: true
---

# M3-007-C — Allow bounded in-flight completion

## Atomic goal

Allow bounded in-flight completion.

## Parent intent

Propagate cancellation and shutdown to the owning worker and native operations exactly once.

## Dependencies

- `M3-007-B` — `tasks/06_m3_multi_worker/M3-007-B-stop-admission-on-drain.md`

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
5. Implement exactly this deliverable: Allow bounded in-flight completion.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- No orphan invocation/native task.
- Shutdown deadline is honored.
- Exit code/report reflects forced aborts.
- All slots/queues/pools quiesce.

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

- Shutdown integration tests.
- Disconnect/cancel races.
- Resource invariant report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m3-007-c: allow bounded in flight completion
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M3-007-C) — PASS

- Date: 2026-08-31
- Branch/PR: m3-007-c (squash-merged; see git log for final hash)
- Closes: #410

### Changed files
- `crates/q-http/src/lib.rs`: `serve()` now produces honest drain evidence.
  - Connection tasks are tracked in a `tokio::task::JoinSet`; every connection is
    wrapped in hyper-util's `GracefulShutdown` watcher (`server-graceful` feature
    added to the workspace hyper-util). On the shutdown trigger, IDLE keep-alive
    connections close immediately and mid-header connections close at once (no
    dispatched request to protect) — only DISPATCHED requests hold the drain.
  - After the accept loop stops, in-flight work is ALLOWED to complete:
    `graceful.shutdown()` + a bounded join. The wait is bounded by a new explicit
    `drain_budget: Duration` parameter (call site passes the ADR-0031 5s budget,
    `SHUTDOWN_BUDGET_MS`).
  - New `ServeDrain { completed, detached }` return: connections that reached a
    terminal state within the budget, and ACTIVE stragglers detached at expiry
    (the dropped watcher already told each connection to finish its current work
    and stop; tasks are detached so the process exits on schedule).
- `crates/q-runtime/src/lib.rs`: passes the budget; the `shutdown.complete` drain
  block now reports `{"refused":N,"completed":C,"detached":D}`.
- `crates/q-runtime/tests/runtime_conformance.rs`: fixture gains route
  `async.slow` (GET /async-slow, shared timer handler, query schema
  `sch:async-slow.query` with ms ≤ 60_000, deadline 30_000) so a dispatched
  request can deterministically outlive the 5s drain budget.
- `Cargo.toml`: hyper-util `server-graceful` feature. `benchmarks/manifest.json`:
  qRuntimeRelease hash refreshed (standard flow).

### Design note (why GracefulShutdown)
The first cut (join ALL connection tasks) hung every Bun conformance test at its
5s timeout: normal HTTP clients hold idle keep-alive connections open, and the
naive join made every shutdown burn the whole budget. hyper's graceful watcher
is the correct semantics — idle connections close at once, dispatched requests
finish, and only a genuinely stuck ACTIVE request can reach the budget.

### Tests added / changed
- `drain_lets_in_flight_request_complete` (new): GET /async?ms=800, SIGTERM
  120ms in — the response still arrives with the full `waited:800` (work was
  not cut short), exit 0 within 5s, drain reports
  `{refused:0,completed:1,detached:0}`, ownership settles exactly once.
- `drain_waits_bounded_then_detaches_straggler_connection` (rewritten): a
  DISPATCHED 20s timer invocation (30s route deadline) is in flight at SIGTERM;
  the drain waits the full budget (elapsed ≥ 5s pinned), detaches the straggler
  (`detached:1`), discloses its lifecycle exactly (registered == 1 and
  pending + settled == 1 — the engine's shutdown-cancel plus the detached
  pipeline's late settle race the report print, so both outcomes are pinned as
  one invariant), and exits 0 within 10s.
- `graceful_drain_flips_gate_and_reports_before_exit`: extended to the full
  drain outcome keys (`refused:0,completed:1,detached:0`).
- Mid-header stragglers are covered by the semantics disclosure above: hyper's
  graceful close terminates them instantly (no dispatched request), which the
  first (failed) version of the straggler test proved empirically.

### Honest C/D boundary
At budget expiry the straggler's connection task is DETACHED, not aborted: the
engine's own shutdown-cancel handles the JS side, but the report-time state of
its ownership binding is scheduling-dependent (the invariant above), and no
forced-abort counter exists yet. M3-007-D replaces the detach with explicit
abort-through-ownership and pins the report to `pending:0` + a forced-abort
count.

### Command results
- `cargo test -p q-http` → 4 + 6 — 0 failed
- `cargo test -p q-capabilities` → 237 + 7 + 1 + 4 + 9 — 0 failed
- `cargo test -p q-engine-quickjs` → 20 + 102 + 1 — 0 failed
- `cargo test -p velqu-runtime` → 55 unit + 6 + 5 + 2 + **35 conformance
  (was 33)** — 0 failed
- `./scripts/verify` → **ALL PASS** (includes bun 183 tests / 21 files after the
  GracefulShutdown fix; fmt clean; clippy -D warnings clean)

### Guardrail mapping (parent M3-007)
- **Shutdown deadline is honored** — the drain wait is bounded by the ADR-0031
  budget and the bound is pinned by test (elapsed ≥ budget and < budget+margin).
- **In-flight completion** — a dispatched request mid-flight at the signal
  completes with its full result; no work is cut short.
- **All slots/queues/pools quiesce** — idle keep-alives close at once; detached
  stragglers are counted and disclosed.

### Disclosures
- The drain gate's 503 (M3-007-B) is now reachable only in the flip race window
  by design: hyper's graceful close means a keep-alive request sent after the
  signal observes connection-close (the standard client signal), not a 503. The
  B-era keep-alive-503 test premise was dropped for that reason; the refusal
  path remains unit-proven (6 q-capabilities tests) and the flip is
  report-proven.
- Standing: CI fails with zero executed steps on every PR since ~#714
  (infrastructure-side); disclosed per PR.
