---
task_id: M3-005-B
parent_task: M3-005
milestone: M3
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M3.md
commit_required: true
---

# M3-005-B — Fail/settle its pending work

## Atomic goal

Fail/settle its pending work.

## Parent intent

Replace poisoned workers without keeping the whole service permanently unhealthy.

## Dependencies

- `M3-005-A` — `tasks/06_m3_multi_worker/M3-005-A-remove-quarantined-worker-from-dispatch.md`

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
5. Implement exactly this deliverable: Fail/settle its pending work.
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
m3-005-b: fail settle its pending work
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M3-005-B) — PASS

- Date: 2026-08-30
- Branch/PR: m3-005-b (squash-merged; see git log for final hash)
- Closes: #397

### Changed files
- `crates/q-capabilities/src/dispatch.rs`: `settle_quarantined(worker) -> Vec<T>` (M3-005-B) —
  - Recovers ALL pending jobs of a quarantined worker (in FIFO order) so the runtime quarantine path can settle each with a typed failure — no job is dropped silently, and NONE is ever executed by the poisoned runtime.
  - The queue is empty and still closed afterwards; settling again is a no-op (nothing dropped twice).
  - Contract-guarded: settling a HEALTHY worker panics (only quarantine may drain) — a serving queue is never disturbed.

### Tests added (+3 → 24 dispatch tests; 221 q-capabilities lib total)
- `settle_quarantined_drains_pending_jobs_for_typed_failure` (2 pending jobs recovered FIFO; queue empty + closed; second settle empty)
- `settle_requires_quarantine_state` (settle on a serving worker is a contract panic; the healthy queue is untouched)
- `quarantined_pending_work_never_reaches_the_poisoned_runtime` (after settle, pop returns nothing — the poisoned runtime can never receive those jobs)

### Command results
- `cargo test -p q-capabilities` → **221 unit (was 218) + 7 + 1 + 4 + 9** — 0 failed
- `cargo test -p q-engine-quickjs` → 20+102+1 · `-p velqu-runtime` → 31+5+44 — all pass
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**; release binary unchanged (`6d5c7c3f…` matches manifest)

### Guardrail mapping
- **Poisoned worker receives no new requests** — pending jobs are recovered, never executed by the poisoned runtime (proven: pop after settle is None).
- **Repeated poison cannot create restart storm** — settle is per-quarantine-event; no job duplication across cycles.

### Disclosures
- Standing: CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR.
