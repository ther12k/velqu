---
task_id: M3-005-A
parent_task: M3-005
milestone: M3
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M3.md
commit_required: true
---

# M3-005-A — Remove quarantined worker from dispatch

## Atomic goal

Remove quarantined worker from dispatch.

## Parent intent

Replace poisoned workers without keeping the whole service permanently unhealthy.

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
5. Implement exactly this deliverable: Remove quarantined worker from dispatch.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Poisoned worker receives no new requests.
- Replacement restores capacity.
- Repeated poison cannot create restart storm.
- Liveness/readiness semantics are correct.

## Targeted commands

```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p velqu-runtime
```

## Required evidence for this microtask

- Poison/replacement chaos tests.
- Readiness tests.
- Restart-rate metrics.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m3-005-a: remove quarantined worker from dispatch
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M3-005-A) — PASS

- Date: 2026-08-30
- Branch/PR: m3-005-a (squash-merged; see git log for final hash)
- Closes: #396

### Changed files
- `crates/q-capabilities/src/dispatch.rs`: quarantine semantics on `Dispatcher` (M3-005-A) —
  - `quarantine(worker)`: the worker stops receiving new dispatches at once (the next `select()` skips it), its queue CLOSES (the drain/settle path is M3-005-B), and the quarantine-event counter increments. Idempotent — re-quarantining an already-quarantined worker does not double-count.
  - `is_quarantined(worker)` + `quarantine_events()` (saturating restart-rate metric).
  - `replace(worker)`: fresh bounded queue for the slot, back to Serving — quarantine history survives replacement (restart-rate survives restarts).
  - `select()` skips quarantined workers before the load scan: a poisoned worker receives NO new requests.
- This is the dispatcher half of M3-005; engine-side poison detection already exists (M2.2.1 quarantine) and is consumed by these methods.

### Tests added (+5 → 21 dispatch tests; 218 q-capabilities lib total)
- `quarantined_worker_receives_no_new_requests` (12 selections, quarantined worker never chosen; queue empty)
- `quarantine_closes_queue_and_is_idempotent` (closed for drain; no double-count)
- `all_quarantined_means_no_selection` (select None; dispatch typed error)
- `replacement_restores_capacity_and_keeps_restart_history` (fresh queue, Serving again, history survives, re-poison counts)
- `repeated_poison_cycle_never_exceeds_initial_worker_count` (**restart-storm guardrail**: 10 poison→replace cycles — fleet size never grows; dispatch still healthy)

### Command results
- `cargo test -p q-capabilities` → **218 unit (was 213) + 7 + 1 + 4 + 9** — 0 failed
- `cargo test -p q-engine-quickjs` → 20+102+1 · `-p velqu-runtime` → 31+5+44 — all pass
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0 (len_zero lint on a new test assertion fixed)
- `./scripts/verify` → **ALL PASS (exit 0)**; release binary unchanged (`6d5c7c3f…` matches manifest)

### Guardrail mapping
- **Poisoned worker receives no new requests** — select() skips quarantined workers; queue closes for the drain path.
- **Repeated poison cannot create restart storm** — replace() restores the ORIGINAL size only; quarantine_events is the restart-rate metric.
- **Liveness/readiness semantics are correct** — healthy workers keep flowing during a quarantine; all-quarantined means no selection (typed error, not a hang).

### Disclosures
- One clippy iteration (len_zero in a test assertion). No production behavior beyond the quarantine semantics.
- Standing: CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR.
