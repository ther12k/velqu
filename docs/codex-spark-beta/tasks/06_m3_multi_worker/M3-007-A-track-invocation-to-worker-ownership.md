---
task_id: M3-007-A
parent_task: M3-007
milestone: M3
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M3.md
commit_required: true
---

# M3-007-A — Track invocation-to-worker ownership

## Atomic goal

Track invocation-to-worker ownership.

## Parent intent

Propagate cancellation and shutdown to the owning worker and native operations exactly once.

## Dependencies

- `M3-002-Z` — `tasks/06_m3_multi_worker/M3-002-Z-package-evidence-for-implement-bounded-worker-dispatcher.md`
- `M3-004-Z` — `tasks/06_m3_multi_worker/M3-004-Z-package-evidence-for-implement-deterministic-worker-initialization-and-artifact.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M3.md`
- `context/components/engine-scheduler.md`
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

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Track invocation-to-worker ownership.
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
cargo test -p q-engine-quickjs
```
```bash
cargo test -p velqu-runtime
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
m3-007-a: track invocation to worker ownership
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M3-007-A) — PASS

- Date: 2026-08-31
- Branch/PR: m3-007-a (squash-merged; see git log for final hash)
- Closes: #408

### Changed files
- `crates/q-capabilities/src/invocation.rs` (new): `InvocationOwnership` — the bounded
  invocation-to-worker ownership registry (M3-007-A, ADR-0036 §4 lifecycle/infrastructure
  discipline; `SharedAcrossWorkers`).
  - `with_workers(workers, capacity)` / `new(workers)` — capacity clamped
    `1..=MAX_INVOCATION_TRACKING_CAPACITY` (65_536, matching the dispatcher queue ceiling;
    default 4_096, sized above every admission bound). Bounded by construction: at most
    `capacity` live bindings, ever.
  - `track(id, worker)` — typed rejections (`TrackError::AtCapacity`, `AlreadyTracked`,
    `UnknownWorker`), counted; never blocks, never grows silently. One admission, one
    registration.
  - `owner_of(id)` — the cancel-route lookup: a cancellation for an invocation is
    delivered to exactly the worker that recorded it.
  - `settle(id) -> Option<worker>` — the terminal transition and the exactly-once gate:
    `Some(owner)` exactly once; every later settlement/cancel observes `None`.
  - `pending` / `pending_of_worker` / `invocations_of_worker` (ascending ids) /
    `snapshot` — deterministic enumeration for the drain/abort paths (M3-007-B..D).
  - `stats()` — redacted counters (`registered - settled == pending` invariant).
- `crates/q-runtime/src/serve.rs`: the pipeline now tracks ownership end-to-end —
  - admission binds `(invocation_id, worker 0)` BEFORE the engine invoke; both tracking
    rejections fail closed with typed problems (AtCapacity → 503 overload + retry-after;
    duplicate/unknown-worker → 500 internal with `contract.violation.ownership` log) and
    unwind the stage metrics.
  - the terminal transition settles FIRST (settle → disarm ordering): a drop between the
    two statements cannot re-cancel a delivered outcome; `debug_assert` pins the
    always-settles-here invariant.
  - `CancelOnDrop` is now ownership-routed: settling IS the gate — `Engine::cancel` is
    delivered exactly once, only when the drop is the terminal transition. A disconnect
    race with a delivered outcome can no longer double-cancel.
- `crates/q-runtime/src/lib.rs`: `ServeState` constructs the registry (single-worker
  topology = 1 worker today; the multi-worker runtime passes its fleet size); the
  `shutdown.complete` event now carries the ownership invariant:
  `"invocations":{"pending":N,"registered":N,"settled":N}` — a graceful drain must
  report `pending == 0` (no orphan invocation).
- `crates/q-runtime/Cargo.toml`: `q-capabilities` promoted dev-dependency → full
  dependency (the registry is host infrastructure the runtime consumes, ADR-0036 §4;
  no dependency cycle — q-capabilities does not depend on q-runtime).
- `crates/q-capabilities/src/lib.rs`: module + re-exports.
- `benchmarks/manifest.json`: qRuntimeRelease hash refreshed (release binary changed;
  standard `refresh-benchmark-manifest.py` flow, verify re-run after).

### Tests added
- `crates/q-capabilities/src/invocation.rs` (10 unit tests, lib 221 → 231):
  capacity clamp/bounds; track/settle round-trip records the owner; unknown-worker typed
  + counted; duplicate track names the existing owner and never inflates `registered`;
  bounded tracking rejects closed and frees slots on settle; per-worker enumeration
  deterministic (ascending ids) + whole-fleet snapshot; stats balance
  (registered − settled == pending) and redaction (no invocation ids in Debug);
  settle-as-exactly-once-gate under a two-thread race (exactly one `Some`);
  4-thread × 250-admission + concurrent settlement stress with the balance invariant
  pinned after the race and every survivor settling exactly once; full
  admit→settle cycle leaves an empty live set (no orphan).
- `crates/q-runtime/tests/runtime_conformance.rs`: `graceful_shutdown_exits_zero`
  extended — one real JS request (`GET /js-text`) before SIGTERM, then the
  `shutdown.complete` report must carry
  `"invocations":{"pending":0,"registered":1,"settled":1}` (no orphan invocation,
  exactly one lifecycle). Duplicate fetchPool assertion removed (same test).

### Command results
- `cargo test -p q-capabilities` → **231 lib (was 221) + 7 fuzz + 1 + 4 + 9
  WPT-manifest** — 0 failed
- `cargo test -p q-engine-quickjs` → 20 + 102 + 1 — 0 failed
- `cargo test -p velqu-runtime` → 55 unit + 6 + 5 + 2 + 32 conformance (was 31) — 0 failed
- `cargo fmt --check` → clean; `cargo clippy -p q-capabilities -p velqu-runtime
  --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS** (after the manifest refresh for the new release hash)

### Guardrail mapping (parent M3-007)
- **No orphan invocation** — every admission binds; every terminal transition settles
  exactly once; the shutdown report asserts `pending == 0` on a graceful drain and the
  registry audit (`snapshot`) enumerates any hypothetical live set.
- Cancellation routing is now ownership-driven (`owner_of`/settle-gate) — the foundation
  for B (stop admission on drain), C (bounded in-flight completion), D (abort after the
  shutdown deadline).

### Disclosures
- The registry rejects (rather than panics) on contract violations: `track` races are
  host bugs, but a shared cross-thread structure must return typed errors, not unwind
  arbitrary worker threads (contrast: `settle_quarantined` asserts — single-caller
  quarantine path).
- Standing: CI fails with zero executed steps on every PR since ~#714
  (infrastructure-side); disclosed per PR.
