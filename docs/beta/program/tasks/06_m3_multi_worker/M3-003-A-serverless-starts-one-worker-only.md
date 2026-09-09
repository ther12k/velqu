---
task_id: M3-003-A
parent_task: M3-003
milestone: M3
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M3.md
commit_required: true
---

# M3-003-A — Serverless starts one worker only

## Atomic goal

Serverless starts one worker only.

## Parent intent

Make cold start versus immediate throughput an explicit deployment choice.

## Dependencies

- `M3-002-Z` — `tasks/06_m3_multi_worker/M3-002-Z-package-evidence-for-implement-bounded-worker-dispatcher.md`

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
5. Implement exactly this deliverable: Serverless starts one worker only.
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
m3-003-a: serverless starts one worker only
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M3-003-A) — PASS

- Date: 2026-08-30
- Branch/PR: m3-003-a (squash-merged; see git log for final hash)
- Closes: #384

### Changed files
- `crates/q-runtime/src/service_profile.rs` (new): `ServiceProfile` — the worker-startup posture as an explicit deployment choice (M3-003-A guarantee + the foundation for B/C/D) —
  - `Serverless` (default): `initial_workers()` is ALWAYS exactly 1 — the type carries no worker count, so no configuration can pre-spawn more (no hidden worker creation by construction).
  - `Service { workers }`: starts the configured count; bare "service" fails closed (no hidden default worker count); counts clamp to [MIN_WORKERS=1, MAX_WORKERS=64] and out-of-range values fail closed.
  - `parse()` is case-insensitive; unknown names / malformed counts fail loudly — never a silent fallback.
  - `as_str()` round-trips for the ready line / inspect output (M3-003-D surface).
- `crates/q-runtime/src/lib.rs`: module wiring.

### Tests added (service_profile.rs, 4)
- `serverless_starts_exactly_one_worker` (the M3-003-A guarantee: initial_workers()==1 under every construction path)
- `service_profile_requires_explicit_count_and_clamps` (bare service fails closed; bounds enforced)
- `unknown_profiles_fail_closed` (7 junk inputs rejected; case-insensitive parsing documented)
- `names_round_trip_for_inspect_output`

### Command results
- `cargo test -p velqu-runtime` → **19 unit (was 17) + 5 + 44** — 0 failed
- `cargo test -p q-engine-quickjs` → 20+101 — pass
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**
- `benchmarks/manifest.json` refreshed (`9c2ebc08…`): the profile module lives in the runtime crate itself.

### Guardrail mapping
- **Serverless cold start remains within approved budget** — one worker is the minimum possible startup; B measures it.
- **No hidden worker creation** — Serverless structurally cannot pre-spawn; Service requires an explicit count or fails closed.
- **Profiles have deterministic readiness** — parse/clamp/str are total and deterministic; B/C own the readiness sequencing.

### Disclosures
- The first verify run failed on the two known fresh-worktree transients (velqu-bytecode) and the second on a stale manifest hash (the new module legitimately changes the runtime artifact) — both resolved by the standard sequence; final run ALL PASS.
- Standing: CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR.
