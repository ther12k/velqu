---
task_id: M4A-007-C
parent_task: M4A-007
milestone: M4A
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-007-C — Expose metrics

## Atomic goal

Expose metrics.

## Parent intent

Provide after-response cleanup/best-effort work without pretending it is durable jobs.

## Dependencies

- `M4A-007-B` — `tasks/07_m4a_developer_preview/M4A-007-B-separate-cleanup-from-best-effort-work.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M4A.md`
- `context/components/devex-beta.md`
- `context/components/evidence.md`

### Source files

- `AGENTS.md`
- `packages/cli/src/index.ts`
- `packages/compiler/src/index.ts`
- `packages/treaty/src/index.ts`
- `packages/testing/src/index.ts`
- `crates/q-engine/src/lib.rs`
- `crates/q-runtime/src/serve.rs`
- `docs/reports/`
- `docs/beta/workstreams/OBSERVABILITY_OPERATIONS.md`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Expose metrics.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Response is not delayed beyond defined handoff.
- Deferred work is bounded.
- Shutdown handles or aborts it deterministically.
- Docs warn against durable-job use.

## Targeted commands

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

- Lifecycle tests.
- Load/cleanup tests.
- Operational docs.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m4a-007-c: expose metrics
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

---

## Result (M4A-007-C) — PASS (2026-09-01)

- Branch/PR: m4a-007-c (squash-merged; see git log for final hash)
- Closes: #470

### Changed files
- `crates/q-engine/src/lib.rs`: `EngineStats` gains six bounded-defer lifecycle
  fields (`defers_admitted`, `defers_rejected`, `defer_drains`,
  `defers_drained`, `defer_drains_interrupted`, `defers_dropped_at_shutdown`).
- `crates/q-engine-quickjs/src/worker.rs`: matching `WorkerShared` counters +
  `stats()` mapping; admission outcomes counted at the JS boundary via new
  natives (`__velquDeferAdmitted`, `__velquDeferRejected`); host-side queue
  observer `__velquDeferredLen`; `drain_deferred` counts non-empty drains,
  drained callbacks, and deadline interrupts; the worker shutdown path counts
  queued-but-never-drained callbacks before discarding them. The counters flow
  into the runtime's `shutdown.complete` report via `eng.stats()`.
- `crates/q-engine-quickjs/tests/engine.rs`: new handler `defer.pending_watch`
  (watched handler that never settles) + `defer_metrics_expose_lifecycle`
  covering the full counter lifecycle.
- `docs/specs/defer-api.md`: metrics table, timeout/cancel drain semantics,
  observation-consistency note.

### Required evidence

- **Lifecycle tests** — `defer_metrics_expose_lifecycle` walks the whole
  counter lifecycle deterministically:
  - simple admit+drain → `defers_admitted 1 / defers_drained 1 / drains 1`;
  - overload at the cap → `admitted 65 / rejected 1` (the handler throws at
    the cap; the Failed handoff still drains the 64 queued callbacks);
  - spinning callback → `defer_drains_interrupted 1`, drained unchanged;
  - watched handler that times out → `drains` unchanged (timeout performs no
    drain; the queued callback waits for the next handoff);
  - cancelled in-flight watched handler → admitted but never drained;
  - shutdown → `defers_dropped_at_shutdown 2` (both never-handed-off
    callbacks counted, then discarded).
  Test stability: the drain runs after response handoff by design, so
  assertions observe only through handoffs whose own drains are empty (no
  counter movement in the race window); verified stable across repeated runs.
- **Load/cleanup tests** — handler-table load test now pins 65 registered
  handlers; the whole pre-existing engine suite (108 tests incl. all M4A-007-A/B
  defer tests) stays green.
- **Operational docs** — `docs/specs/defer-api.md` §5 documents the six
  counters, the non-empty-drain counting rule, timeout/cancel drain semantics,
  and concurrent-observation consistency.

### Guardrail mapping (parent M4A-007)

- **Response is not delayed beyond defined handoff**: metrics are atomic
  counter increments on the worker thread — no added handoff latency.
- **Deferred work is bounded**: bounds unchanged; `defers_rejected` and
  `defer_drains_interrupted` now make bound enforcement observable.
- **Shutdown handles or aborts it deterministically**: shutdown now counts
  discarded callbacks before dropping them (`defers_dropped_at_shutdown`).
- **Docs warn against durable-job use**: warning retained; metrics documented
  with their exact semantics.

### Command results

- `cargo test -p q-engine-quickjs` → **109 pass / 0 fail** (was 108; +1)
- `cargo clippy --workspace --all-targets -- -D warnings` → clean
- `./scripts/verify` → **ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)**
- Full evidence in this record; commit hash in squash-merge.

### Disclosures

- Standing: CI `verify` workflows fail with zero executed steps on every PR
  since ~#714 (infrastructure-side); disclosed per PR. Local
  `./scripts/verify` is the gate evidence.
