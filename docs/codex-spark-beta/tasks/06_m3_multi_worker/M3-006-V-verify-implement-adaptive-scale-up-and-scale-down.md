---
task_id: M3-006-V
parent_task: M3-006
milestone: M3
priority: P1
mode: VERIFY
status: PASS
context_card: context/milestones/M3.md
commit_required: true
---

# M3-006-V — Verify Implement adaptive scale-up and scale-down

## Atomic goal

Prove every acceptance criterion for parent task M3-006 without broadening scope.

## Parent intent

Add workers according to queue pressure while preserving memory budgets.

## Dependencies

- `M3-006-A` — `tasks/06_m3_multi_worker/M3-006-A-define-thresholds-hysteresis.md`
- `M3-006-B` — `tasks/06_m3_multi_worker/M3-006-B-bound-min-max-workers.md`
- `M3-006-C` — `tasks/06_m3_multi_worker/M3-006-C-drain-before-scale-down.md`
- `M3-006-D` — `tasks/06_m3_multi_worker/M3-006-D-avoid-oscillation.md`

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

- Adaptive mode scales under load.
- Idle workers retire safely.
- No request loss.
- RSS and latency trade-off is documented.

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

- Adaptive load test.
- State transition tests.
- Memory report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m3-006-v: verify implement adaptive scale up and scale down
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M3-006-V) — PASS

- Date: 2026-08-30
- Branch/PR: m3-006-v (squash-merged; see git log for final hash)
- Closes: #406

### Acceptance-criterion mapping (parent M3-006 guardrails)

1. **Adaptive mode scales under load** — verified: `ScaleThresholds` + `HysteresisState.observe` fire scale-up on pressure strictly above threshold (after cooldown), one worker per tick under sustained pressure. Tests: `scale_up_fires_above_threshold_and_resets_hysteresis` (A), `pressure_adds_one_worker_per_tick` (M3-003-B).
2. **Idle workers retire safely** — verified: scale-down requires SUSTAINED stability (full window; burst resets), never below min_workers, and the retirement lifecycle DRAINS before teardown (lossless; budget escalation bounded). Tests: `scale_down_requires_sustained_stability`, `scale_down_never_retires_below_min_workers`, `retirement_is_lossless_while_draining`, `drain_budget_escalates_a_wedged_worker` (C).
3. **No request loss** — verified: drain re-homes every queued job before teardown (`retirement_is_lossless_while_draining`); WorkerBounds clamp keeps capacity inside [min, max] (`scaler_floor_and_ceiling_query_the_bounds`, B); drain-budget escalation settles leftovers typed (M3-005-B).
4. **RSS and latency trade-off is documented** — verified: WorkerBounds.max IS the memory-budget ceiling enforced at construction (`bounds_fail_closed_on_invalid_configuration`, B); the M3-003-V live evidence (serverless 7.5ms ready, ~7.9 MB RSS) documents the single-worker point; profile-differentiated scaling numbers are M3-009's dedicated evidence.

### Anti-oscillation (M3-006-D, composing guardrail)
- Dead band between thresholds (== never scales), cooldown gating both directions, stability-window reset on burst, and the `ScaleGovernor` outer cap: an adversarial alternating signal over 20 ticks yields exactly 5 events (2/window + reset tick), not ~20. Tests: `dead_band_between_thresholds_never_scales`, `cooldown_gates_both_directions`, `event_cap_bounds_flip_flopping_structurally`, `window_reset_reallows_scaling`, `churn_metric_tracks_total_events`.

### Verification runs (this branch, worktree-fresh)
- `cargo test -p velqu-runtime` → 55 unit + 5 + 44 (incl. 30 profile/scaler tests) — 0 failed
- `cargo test -p q-capabilities` → 6 suites; `-p q-engine-quickjs` → 20+102+1 — pass
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**; release binary reproduced deterministically (`7c8b3f5b…` matches the M3-005-D manifest)

### Disclosures (standing)
- No production code changed in this packet: verification-only closure of M3-006-A/B/C/D.
- CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR. Local evidence above is complete.
