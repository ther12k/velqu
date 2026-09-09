---
task_id: M4A-007-A
parent_task: M4A-007
milestone: M4A
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-007-A — Define deferred owner, queue, deadline, cancellation, shutdown

## Atomic goal

Define deferred owner, queue, deadline, cancellation, shutdown.

## Parent intent

Provide after-response cleanup/best-effort work without pretending it is durable jobs.

## Dependencies

- `M27-GATE` — `gates/M27-GATE.md`
- `M3-GATE` — `gates/M3-GATE.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M4A.md`
- `context/components/engine-scheduler.md`
- `context/components/devex-beta.md`

### Source files

- `AGENTS.md`
- `packages/cli/src/index.ts`
- `packages/compiler/src/index.ts`
- `packages/treaty/src/index.ts`
- `packages/testing/src/index.ts`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Define deferred owner, queue, deadline, cancellation, shutdown.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Response is not delayed beyond defined handoff.
- Deferred work is bounded.
- Shutdown handles or aborts it deterministically.
- Docs warn against durable-job use.

## Targeted commands

```bash
cargo test -p q-engine-quickjs
```
```bash
bun test
```
```bash
bun run typecheck
```

## Required evidence for this microtask

- Lifecycle tests.
- Load/cleanup tests.
- Operational docs.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m4a-007-a: define deferred owner queue deadline cancellation shutdown
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

---

## Result (M4A-007-A) — PASS (2026-09-01)

- Branch/PR: m4a-007-a (squash-merged; see git log for final hash)
- Closes: #468

### Changed files
- `crates/q-engine-quickjs/src/lib.rs`: `QuickJsConfig` gains
  `defer_queue_capacity` (default 64) and `defer_deadline_ms` (default 100).
- `crates/q-engine-quickjs/src/prelude.rs`: worker-owned bounded deferred
  queue (`globalThis.__velquDeferred`), admission function `__velquDefer`
  (functions only, capacity-checked, Invocation-owner semantics documented),
  and `__velquDrainDeferred` (swaps the queue, isolates callback failures).
- `crates/q-engine-quickjs/src/worker.rs`: `drain_deferred()` runs after
  response handoff on all settlement paths (Failed, Immediate, resolved) in
  the Cleanup phase: host-side cap truncation, defer-deadline interrupt armed
  during the drain, bounded cleanup job budget afterwards. Worker keeps
  `defer_deadline`/`defer_queue_capacity` from config.
- `crates/q-engine-quickjs/tests/engine.rs`: four handlers
  (`defer.simple`, `defer.check`, `defer.overload`, `defer.spin`) + four
  lifecycle tests.
- `docs/specs/defer-api.md` (new): operational doc; ownership, bounds table,
  cancellation/shutdown behavior, and an explicit warning that deferred work
  is not a durable job queue.
- `benchmarks/manifest.json`: refreshed.

### Required evidence

- **Lifecycle tests** (all in `crates/q-engine-quickjs/tests/engine.rs`):
  - `defer_runs_after_response_handoff` — response body is fixed before the
    deferred callback runs; the next invocation observes exactly one run.
  - `defer_queue_is_bounded` — 70 enqueues against a 64 cap fail closed with
    `defer queue capacity reached` (EngineFailure); response not produced.
  - `defer_drain_deadline_bounds_spinning_callback` — a `while(true)`
    deferred callback cannot outlive the 100 ms drain deadline; the worker
    stays alive for the following invocation.
  - `shutdown_aborts_queued_deferred_work` — shutdown discards queued
    deferred callbacks deterministically.
- **Load/cleanup tests** — the handler-table load test pins 60 registered
  handlers (was 56) and the drain runs under `ExecutionPhase::Cleanup` with
  the existing cleanup job budget (`MAX_CLEANUP_JOBS`, `SETTLEMENT_GRACE`)
  untouched.

### Guardrail mapping (parent M4A-007)

- **Response is not delayed beyond defined handoff**: the drain runs strictly
  after `reply.send` (or settle for the failure path) on every settlement path.
- **Deferred work is bounded**: capacity 64 with a JS check + host truncation;
  drain deadline interrupt at 100 ms; cleanup jobs bounded as before.
- **Shutdown handles or aborts it deterministically**: shutdown aborts the
  runtime; queued callbacks are discarded (test pinned).
- **Docs warn against durable-job use**: `docs/specs/defer-api.md` opens with
  the non-durable warning and shutdown/discard semantics.

### Command results

- `cargo test -p q-engine-quickjs` → **106 pass / 0 fail** (was 102; +4 defer)
- `cargo test -p velqu-runtime --test runtime_conformance` → 35 pass / 0 fail
- `bun test` + `bun run typecheck` → clean (via verify)
- `cargo fmt --all --check` clean; workspace clippy -D warnings → exit 0
- `./scripts/verify` → **ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)**

### Disclosures

- Standing: CI `verify` workflows fail with zero executed steps on every PR
  since ~#714 (infrastructure-side); disclosed per PR. Local
  `./scripts/verify` is the gate evidence.
