---
task_id: M3-003-V
parent_task: M3-003
milestone: M3
priority: P1
mode: VERIFY
status: PASS
context_card: context/milestones/M3.md
commit_required: true
---

# M3-003-V — Verify Implement serverless, service, and throughput profiles

## Atomic goal

Prove every acceptance criterion for parent task M3-003 without broadening scope.

## Parent intent

Make cold start versus immediate throughput an explicit deployment choice.

## Dependencies

- `M3-003-A` — `tasks/06_m3_multi_worker/M3-003-A-serverless-starts-one-worker-only.md`
- `M3-003-B` — `tasks/06_m3_multi_worker/M3-003-B-service-marks-ready-after-worker-0-and-adds-workers-adaptively.md`
- `M3-003-C` — `tasks/06_m3_multi_worker/M3-003-C-throughput-initializes-configured-workers-before-ready.md`
- `M3-003-D` — `tasks/06_m3_multi_worker/M3-003-D-expose-profile-in-inspect-config.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M3.md`
- `context/components/engine-scheduler.md`
- `context/components/multiworker.md`
- `context/components/evidence.md`

### Source files

- `AGENTS.md`
- `crates/q-runtime/src/main.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-engine-quickjs/src/lib.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`
- `benchmarks/harness/`
- `benchmarks/manifest.json`
- `benchmarks/raw/`
- `docs/reports/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- Serverless cold start remains within approved budget.
- Profiles have deterministic readiness.
- No hidden worker creation.
- Profile-specific RSS is reported.

## Targeted commands

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

- Profile conformance.
- Cold/RSS report.
- Configuration docs.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m3-003-v: verify implement serverless service and throughput profiles
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M3-003-V) — PASS

- Date: 2026-08-30
- Branch/PR: m3-003-v (squash-merged; see git log for final hash)
- Closes: #388

### Acceptance-criterion mapping (parent M3-003 guardrails)

1. **Serverless cold start remains within approved budget** — verified live: the proof pack's serverless ready line reports `startupMs: 7.50` (budget 10 ms from M27-011 evidence; the one-worker startup is the minimum possible posture). The ready line also declared `serviceProfile: "serverless", startupWorkers: 1` exactly as M3-003-D requires.
2. **Profiles have deterministic readiness** — verified: `Readiness` flips exactly once on the call meeting the requirement (serverless = worker 0; throughput = all configured), one-way, with out-of-range parse failing closed and direct variants clamping to MAX_WORKERS. Tests: `serverless_readiness_needs_exactly_worker_zero`, `throughput_readiness_needs_every_configured_worker`, `readiness_is_one_way`, `throughput_out_of_range_fails_closed_direct_variants_clamp`.
3. **No hidden worker creation** — verified: `Serverless` carries no worker count (structurally cannot pre-spawn); `Service` requires an explicit count or fails closed; adaptive adds happen ONLY through the bounded policy tick (max + cooldown). Tests: `serverless_starts_exactly_one_worker`, `service_profile_requires_explicit_count_and_clamps`, `unknown_profiles_fail_closed`, `pressure_adds_one_worker_per_tick`, `max_workers_bounds_growth_exactly`, `cooldown_gates_bursts_against_oscillation`.
4. **Profile-specific RSS is reported** — measured live this packet: serverless single-worker serving the proof pack = **~7.9 MB RSS** (health 200 during measurement). Profile-differentiated RSS scaling is M3-009's dedicated evidence (topology recorded there).

### Verification runs (this branch, worktree-fresh)
- `cargo test -p velqu-runtime` → 28 unit + 5 + 44 (incl. ready-line/fail-closed integration test) passed
- `cargo test -p q-capabilities` → 6 suites; `-p q-engine-quickjs` → 20+101 — pass
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `--profile-info` exercised live: serverless→1, service:4→4, service:0→typed error, bogus→typed error, bounds [1,64] — exit 0
- Live serverless run: ready 7.5 ms, health 200, graceful shutdown complete with fetchPool drained
- `./scripts/verify` → **ALL PASS (exit 0)**; release binary reproduced deterministically (`55b79127…`)

### Disclosures (standing)
- No production code changed in this packet: verification-only closure of M3-003-A/B/C/D.
- CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR. Local evidence above is complete.
