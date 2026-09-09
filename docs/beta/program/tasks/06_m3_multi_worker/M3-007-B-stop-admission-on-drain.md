---
task_id: M3-007-B
parent_task: M3-007
milestone: M3
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M3.md
commit_required: true
---

# M3-007-B — Stop admission on drain

## Atomic goal

Stop admission on drain.

## Parent intent

Propagate cancellation and shutdown to the owning worker and native operations exactly once.

## Dependencies

- `M3-007-A` — `tasks/06_m3_multi_worker/M3-007-A-track-invocation-to-worker-ownership.md`

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
- `docs/beta/`
- `examples/proof/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Stop admission on drain.
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
m3-007-b: stop admission on drain
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M3-007-B) — PASS

- Date: 2026-08-31
- Branch/PR: m3-007-b (squash-merged; see git log for final hash)
- Closes: #409

### Changed files
- `crates/q-capabilities/src/drain.rs` (new): `DrainGate` — the serving/draining
  lifecycle flag (M3-007-B, ADR-0036 §4 lifecycle-atomics discipline;
  `SharedAcrossWorkers`).
  - `begin() -> bool` — Serving -> Draining via an AcqRel swap: exactly ONE caller
    wins (the runtime's `drain.begin` logger); every later call is an idempotent
    no-op.
  - `is_draining()` — lock-free Acquire load (hot path, same posture as the
    engine-health quarantine check).
  - `check_admission()` — `Ok` while serving; once draining, the refusal is
    typed (`AdmissionDrained`, redacted Display) and counted with a checked
    saturating increment (never wraps).
  - `refused()` — the shutdown-report counter: clients honestly told to retry.
- `crates/q-runtime/src/serve.rs`: `ServeState.drain_gate`; the pipeline checks
  the gate AFTER native liveness (health probes keep answering so load
  balancers observe the instance going away) and BEFORE the quarantine gate —
  a draining instance refuses every request that would enter JS with the
  FROZEN `overload` problem (503, `retry-after: 1`, detail "server is
  draining"; the problem registry is frozen, so no new URN — the stage tag
  `draining` distinguishes it internally). Refused requests never touch the
  engine or the ownership registry.
- `crates/q-runtime/src/lib.rs`: constructs the gate; a signal task clones the
  shutdown watch and flips the gate the INSTANT the signal fires, logging
  `{"event":"drain.begin","pending":N}`; `shutdown.complete` now carries
  `"drain":{"refused":N}` after the invocations invariant block.
- `benchmarks/manifest.json`: qRuntimeRelease hash refreshed (release binary
  changed; standard flow, verify re-run after).

### Tests added
- `crates/q-capabilities/src/drain.rs` (6 unit tests): serving-by-default with
  uncounted admissions; begin flips exactly once with typed+counted refusals;
  refused counter saturates at u64::MAX (this test caught a wrapping
  `fetch_add` — fixed to a checked saturating `fetch_update`); 8 concurrent
  `begin()` threads produce exactly one winner; drain state crosses threads
  immediately (AcqRel/Acquire); redacted Debug (no request data).
- `crates/q-runtime/tests/runtime_conformance.rs`:
  `graceful_drain_flips_gate_and_reports_before_exit` — one JS request, then
  SIGTERM; asserts the `drain.begin` event (`"pending":0`), the
  `"drain":{"refused":0}` key in `shutdown.complete`, the ownership invariant
  intact (`registered:1, settled:1, pending:0`), and exit 0.

### Scope note (honest)
The end-to-end drain refusal of a request arriving on an ESTABLISHED
keep-alive connection during the drain window is NOT pinned by this packet's
integration test: today `host.serve` returns as soon as the accept loop stops
and the process exits within milliseconds, so that window cannot be driven
deterministically from outside. Wiring the refusal is proven by unit tests +
the pipeline gate placement; the end-to-end assertion lands with M3-007-C
(bounded in-flight completion keeps the process alive across the window).

### Command results
- `cargo test -p q-capabilities` → **237 lib (was 231; +6 drain tests) + 7 fuzz
  + 1 + 4 + 9 WPT-manifest** — 0 failed
- `cargo test -p q-engine-quickjs` → 20 + 102 + 1 — 0 failed
- `cargo test -p velqu-runtime` → 55 unit + 6 + 5 + 2 + **33 conformance
  (was 32)** — 0 failed
- `cargo fmt --check` → clean; `cargo clippy -p q-capabilities -p velqu-runtime
  --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS** (after the manifest refresh for the new
  release hash)

### Guardrail mapping (parent M3-007)
- **All slots/queues/pools quiesce** — the drain boundary is defined per
  request by a lock-free gate; the refusal is typed, counted, and reported.
- Admission stops at the flip instant; in-flight completion bounding is
  M3-007-C, deadline abort M3-007-D (ADR-0036 obligations table).

### Disclosures
- The drain refusal reuses the frozen `overload` problem URN (registry is
  frozen by the pack-format spec) with drain-specific detail; internal stage
  tag `draining` keeps them distinguishable in logs/metrics.
- Standing: CI fails with zero executed steps on every PR since ~#714
  (infrastructure-side); disclosed per PR.
