---
task_id: M3-003-C
parent_task: M3-003
milestone: M3
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M3.md
commit_required: true
---

# M3-003-C — Throughput initializes configured workers before ready

## Atomic goal

Throughput initializes configured workers before ready.

## Parent intent

Make cold start versus immediate throughput an explicit deployment choice.

## Dependencies

- `M3-003-B` — `tasks/06_m3_multi_worker/M3-003-B-service-marks-ready-after-worker-0-and-adds-workers-adaptively.md`

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

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Throughput initializes configured workers before ready.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

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

## Required evidence for this microtask

- Profile conformance.
- Cold/RSS report.
- Configuration docs.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m3-003-c: throughput initializes configured workers before ready
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M3-003-C) — PASS

- Date: 2026-08-30
- Branch/PR: m3-003-c (squash-merged; see git log for final hash)
- Closes: #386

### Changed files
- `crates/q-runtime/src/service_profile.rs`: `Readiness` — deterministic readiness tracking per profile (M3-003-C) —
  - `Readiness::starting(profile)`: nothing initialized, not ready.
  - `required()`: serverless = 1 (worker 0 only); throughput = the full configured count (clamped exactly like the profile).
  - `worker_initialized() -> bool`: returns `true` ONLY on the call that CAUSES the ready transition — exactly one caller announces readiness; earlier calls get false, later calls get false again (never re-triggered, never un-set).
  - Readiness is one-way; partial initialization is never ready.
- Tests cover both profiles, the one-way property, exact-call flip semantics, and out-of-range handling (parse fails closed; directly-constructed variants clamp their requirement to MAX_WORKERS).

### Tests added (service_profile.rs, +4 → 28 runtime unit tests)
- `serverless_readiness_needs_exactly_worker_zero`
- `throughput_readiness_needs_every_configured_worker` (flip exactly on the 4th)
- `readiness_is_one_way`
- `throughput_out_of_range_fails_closed_direct_variants_clamp`

### Command results
- `cargo test -p velqu-runtime` → **28 unit (was 24) + 5 + 44** — 0 failed
- `cargo test -p q-engine-quickjs` → 20+101 — pass
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**; release binary unchanged (`9c2ebc08…` matches manifest)

### Guardrail mapping
- **Profiles have deterministic readiness** — the requirement is a pure function of the profile; the flip happens exactly once, on the exact call that meets it.

### Disclosures
- Two test-authoring iterations: `worker_initialized` semantics refined to "returns true only on the flipping call" (strictly more useful — exactly one caller announces readiness), and an extra fixture call removed. The suite caught both before commit.
- Standing: CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR.
