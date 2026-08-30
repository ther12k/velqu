---
task_id: M3-005-Z
parent_task: M3-005
milestone: M3
priority: P0
mode: EVIDENCE
status: PASS
context_card: context/milestones/M3.md
commit_required: true
---

# M3-005-Z — Package evidence for Implement quarantine, replacement, and readiness aggregation

## Atomic goal

Create source-backed evidence and handoff for parent task M3-005; update status only if verification passed.

## Parent intent

Replace poisoned workers without keeping the whole service permanently unhealthy.

## Dependencies

- `M3-005-V` — `tasks/06_m3_multi_worker/M3-005-V-verify-implement-quarantine-replacement-and-readiness-aggregation.md`

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
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`
- `docs/reports/`
- `docs/beta/workstreams/OBSERVABILITY_OPERATIONS.md`
- `conformance/security/security.conformance.test.ts`
- `crates/q-pack/tests/fuzz_pack.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

## Parent acceptance guardrails

- Poisoned worker receives no new requests.
- Replacement restores capacity.
- Repeated poison cannot create restart storm.
- Liveness/readiness semantics are correct.

## Targeted commands

```bash
cargo test -p q-pack
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

- Poison/replacement chaos tests.
- Readiness tests.
- Restart-rate metrics.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m3-005-z: package evidence for implement quarantine replacement and re
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M3-005-Z) — PASS

- Date: 2026-08-30
- Branch/PR: m3-005-z (squash-merged; see git log for final hash)
- Closes: #401

### Parent closure — M3-005 Implement quarantine, replacement, and readiness aggregation

Parent intent: replace poisoned workers without keeping the whole service permanently unhealthy. Status: **PASS**.

Packet commits (squash merges):
- M3-005-A — 8fa4443 (#1000, Closes #396): `Dispatcher::quarantine` — poisoned workers excluded from select() immediately; queue closes for the drain path; idempotent; quarantine_events restart-rate metric; replace() restores original capacity only
- M3-005-B — 2059f4d (#1001, Closes #397): `settle_quarantined` — pending jobs recovered FIFO for typed failure; never executed by the poisoned runtime; contract-guarded (healthy settle panics)
- M3-005-C — 17984df (#1002, Closes #398): `ReplacementPolicy` — fixed-window budget (budget/window rate limit) + cooldown between replacements; deterministic gates (budget, then cooldown); no cooldown before the first replacement
- M3-005-D — 79b54a1 (#1003, Closes #399): `aggregate_readiness` — fleet readiness from usable capacity; ready while >= 1 usable; observable degradation via the usable count
- M3-005-V — 008335e (#1004, Closes #400): verification closure mapping all 4 guardrails to the 15 new tests

### Required evidence
- **Poison/replacement chaos tests**: `repeated_poison_cycle_never_exceeds_initial_worker_count` (10 cycles, fleet size constant), `restart_storm_scenario_stays_bounded` (100 poison events → exactly 50 bounded replacements), `replacement_restores_capacity_and_keeps_restart_history`
- **Readiness tests**: `quarantine_lifecycle_reaches_degraded_then_ready_again` (4→2→1 usable, still ready, restored), `readiness_is_true_while_any_worker_is_usable`, `all_quarantined_means_no_selection`
- **Restart-rate metrics**: `quarantine_events()` (saturating; survives replacement), `ReplacementPolicy.replacements` (per-window budget accounting)

### Source/test map
- `crates/q-capabilities/src/dispatch.rs` (quarantine/settle/replace; 24 dispatch tests total)
- `crates/q-runtime/src/service_profile.rs` (ReplacementPolicy + aggregate_readiness; 39 profile tests total)
- Release binary `7c8b3f5b…` matches manifest (D's re-exports land in the runtime artifact)

### Command results (this branch)
- `cargo test -p q-capabilities` → 6 suites (221 unit incl. 24 dispatch); `-p velqu-runtime` → 39+5+44; `-p q-engine-quickjs` → 20+102+1; `-p q-http` → 4+6+1; `-p q-bridge` → 11 — all pass
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**

### Ledger update
- `docs/beta/04_TASK_LEDGER.md`: M3-005 flipped TODO -> PASS.

### Disclosures (standing)
- CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR. Local evidence above is complete.
