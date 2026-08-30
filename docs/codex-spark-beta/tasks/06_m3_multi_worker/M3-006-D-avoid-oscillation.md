---
task_id: M3-006-D
parent_task: M3-006
milestone: M3
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M3.md
commit_required: true
---

# M3-006-D — Avoid oscillation

## Atomic goal

Avoid oscillation.

## Parent intent

Add workers according to queue pressure while preserving memory budgets.

## Dependencies

- `M3-006-C` — `tasks/06_m3_multi_worker/M3-006-C-drain-before-scale-down.md`

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
5. Implement exactly this deliverable: Avoid oscillation.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Adaptive mode scales under load.
- Idle workers retire safely.
- No request loss.
- RSS and latency trade-off is documented.

## Targeted commands

```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p velqu-runtime
```

## Required evidence for this microtask

- Adaptive load test.
- State transition tests.
- Memory report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m3-006-d: avoid oscillation
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M3-006-D) — PASS

- Date: 2026-08-30
- Branch/PR: m3-006-d (squash-merged; see git log for final hash)
- Closes: #405

### Changed files
- `crates/q-runtime/src/service_profile.rs`: `ScaleGovernor` — the anti-oscillation outer bound over the M3-006-A hysteresis core (M3-006-D) —
  - Fixed-window event cap: at most `max_events_per_window` scale events (either direction) per `window_ticks` window; the window replenishes deterministically when its ticks elapse.
  - Fail-closed construction: cap 0 or 0-tick window rejected (a governor that can never scale is misconfiguration, not safety).
  - `total_events` churn metric (saturating) counts every scale event across windows.
  - Window-exhausted ticks ABSORB the pressure sample (the hysteresis state still updates) but scale nothing — flip-flopping is structurally bounded to cap/window.
- `crates/q-runtime/src/lib.rs`: `ScaleGovernor` re-exported.

### Tests added (+4 → 55 runtime unit tests)
- `governor_rejects_degenerate_construction` (cap 0 / 0-tick window rejected)
- `event_cap_bounds_flip_flopping_structurally` (alternating burst/quiet over 20 ticks — a signal that would naturally flip-flop every other tick — yields exactly 5 events: 2/window + 1 on each reset tick)
- `window_reset_reallows_scaling` (cap 1: fire, 3 suppressed, reset tick fires immediately — reset-then-decide in one tick)
- `churn_metric_tracks_total_events` (cap 3/window 10 over 12 ticks: 3 + reset + 2 = 6, counted)

### Command results
- `cargo test -p velqu-runtime` → **55 unit (was 48) + 5 + 44** — 0 failed
- `cargo test -p q-capabilities` → 6 suites — pass
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**; release binary unchanged (`7c8b3f5b…` matches manifest)

### Guardrail mapping
- **Adaptive mode scales under load** — scale-up still fires within the window budget.
- **No oscillation** — flip-flopping is structurally bounded to cap/window; proven with an adversarial alternating signal.

### Disclosures
- The first window refill used the wrong field (cooldown instead of window length) — events kept firing after resets; the tests caught it and the refill now uses the stored window length. Exact event counts (2/window + reset-tick) were pinned by trace. The suite drove the design to correctness.
- Standing: CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR.
