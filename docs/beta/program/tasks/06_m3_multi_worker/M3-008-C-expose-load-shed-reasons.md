---
task_id: M3-008-C
parent_task: M3-008
milestone: M3
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M3.md
commit_required: true
---

# M3-008-C — Expose load-shed reasons

## Atomic goal

Expose load-shed reasons.

## Parent intent

Prevent one route/tenant/slow workload from monopolizing workers.

## Dependencies

- `M3-008-B` — `tasks/06_m3_multi_worker/M3-008-B-define-long-running-js-policy.md`

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
5. Implement exactly this deliverable: Expose load-shed reasons.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Small requests make progress under slow workload.
- Overload does not cause unbounded memory.
- Limits are configurable.
- No starvation in approved scenarios.

## Targeted commands

```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p velqu-runtime
```

## Required evidence for this microtask

- Mixed-load benchmarks.
- Fairness metrics.
- Adversarial tests.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m3-008-c: expose load shed reasons
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M3-008-C) — PASS

- Date: 2026-08-31
- Branch/PR: m3-008-c (squash-merged; see git log for final hash)
- Closes: #416

### Changed files
- `crates/q-capabilities/src/load_shed.rs` (new): the closed load-shed
  vocabulary (M3-008-C).
  - `LoadShedReason` — 7 kinds covering every genuine capacity refusal:
    `worker_queue_full`, `all_workers_full` (M3-002), `global_admission_full`,
    `class_ceiling` (M3-008-A), `long_running_slots` (M3-008-B), `draining`
    (M3-007-B), `tracking_full` (M3-007-A).
  - `kind()` — stable metric/log labels; `client_detail()` — redacted
    (bounds only, never worker/class/caller); `problem_kind()` — the frozen
    `overload` verdict for every capacity refusal (M3-002-C / M3-007-B
    precedent); `retry_after_secs()` — the shared 1 s posture.
  - Conversions: `From<QueueError>`, `From<LongSlotsExhausted>`; 
    `FairnessReject::load_shed_reason()` and `TrackError::load_shed_reason()`
    return `Option` — contract violations (UnknownClass, AlreadyTracked,
    UnknownWorker) are host bugs, NOT load-shed events, and can never be
    laundered into "server is busy".
  - `LoadShedCounters` — fixed `[AtomicU64; 7]` (closed set, never growable;
    ADR-0036 §4 metrics discipline), saturating, kind-sorted snapshot.
- `crates/q-runtime/src/serve.rs`: `ServeState.load_shed`; both live refusal
  sites record their reason (drain gate → `draining`; ownership capacity →
  `tracking_full`).
- `crates/q-runtime/src/lib.rs`: constructs the counters; `shutdown.complete`
  renders `"loadShed": {<kind>: count for all 7 kinds}`.
- `crates/q-capabilities/src/lib.rs`: module + re-exports.
- `benchmarks/manifest.json`: refreshed (standard remapped-build flow).

### Tests added
- q-capabilities (+4, lib 256 → 260): stable/deterministic/distinct kind
  labels with full snapshot coverage; redacted client detail + frozen verdict
  across all seven kinds; conversions cover every component rejection while
  contract violations map to None; counters record/saturate/snapshot
  deterministically (BTreeMap kind order).
- runtime conformance: `graceful_drain_flips_gate_and_reports_before_exit`
  now parses the report and asserts the `loadShed` object carries all seven
  kinds with `draining: 0` in the no-refusal scenario.

### Command results
- `cargo test -p q-capabilities` → **260 lib (was 256) + 7 fuzz + 1 + 4 + 9
  WPT-manifest** — 0 failed
- `cargo test -p velqu-runtime` → 7 suites — 0 failed (35 conformance)
- `cargo fmt --check` → clean; `cargo clippy -p q-capabilities
  -p velqu-runtime --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS**

### Disclosures
- The two new runtime record sites (drain, tracking capacity) are the only
  externally-reachable refusals in the single-engine runtime today; the other
  five kinds become reachable as the dispatcher/fairness/long-running wiring
  lands (M3-008-D exercises mixed workloads; the multi-worker integration
  completes it). The vocabulary and counters are complete now by design.
- Standing: CI fails with zero executed steps on every PR since ~#714
  (infrastructure-side); disclosed per PR.
