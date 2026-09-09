---
task_id: M4A-007-B
parent_task: M4A-007
milestone: M4A
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-007-B — Separate cleanup from best-effort work

## Atomic goal

Separate cleanup from best-effort work.

## Parent intent

Provide after-response cleanup/best-effort work without pretending it is durable jobs.

## Dependencies

- `M4A-007-A` — `tasks/07_m4a_developer_preview/M4A-007-A-define-deferred-owner-queue-deadline-cancellation-shutdown.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M4A.md`
- `context/components/devex-beta.md`

### Source files

- `AGENTS.md`
- `packages/cli/src/index.ts`
- `packages/compiler/src/index.ts`
- `packages/treaty/src/index.ts`
- `packages/testing/src/index.ts`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Separate cleanup from best-effort work.
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
m4a-007-b: separate cleanup from best effort work
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

---

## Result (M4A-007-B) — PASS (2026-09-01)

- Branch/PR: m4a-007-b (squash-merged; see git log for final hash)
- Closes: #469

### Changed files
- `crates/q-engine-quickjs/src/worker.rs`: new `ExecutionPhase::DeferredDrain`
  variant; the best-effort deferred drain (`drain_deferred`, including its
  follow-up job drain) now runs in that dedicated phase instead of settlement
  `Cleanup`; the native-op phase guard rejects op starts during the drain with
  a distinct message (`native operations are unavailable while deferred work
  drains`); new `__velquCanAdmitDefer` native predicate exposes the execution
  phase to the prelude; `drain_deferred` no longer keys off the prelude handle
  cache (embedded-prelude packs now drain; a missing queue global is simply
  nothing to drain).
- `crates/q-engine-quickjs/src/prelude.rs`: `__velquDefer` enforces admission
  ownership via `__velquCanAdmitDefer` — only the Invocation phase may enqueue
  (`defer queue unavailable outside the invocation owner`).
- `crates/q-engine-quickjs/tests/engine.rs`: four new handlers
  (`defer.reenqueue_in_drain`, `defer.nativeop_in_drain`,
  `defer.from_reaction`, `defer.check_spied`) and two tests:
  `defer_admission_requires_invocation_owner`,
  `deferred_drain_is_op_free_and_cannot_reenqueue`.
- `docs/specs/defer-api.md`: ownership/dedicated-phase/op-free-drain bounds
  documented; bounds table extended.
- `benchmarks/manifest.json`: refreshed.

### Required evidence

- **Lifecycle tests**:
  - `defer_admission_requires_invocation_owner` — a rejection reaction of an
    aborted floating timer (settlement Cleanup) cannot admit deferred work;
    nothing is enqueued and nothing runs.
  - `deferred_drain_is_op_free_and_cannot_reenqueue` — inside the drain, both
    re-enqueue (owner rule) and native op start (op-free drain) are rejected
    with their distinct phase messages; neither callback effect occurs.
  - Existing A tests stay green (`defer_runs_after_response_handoff`,
    `defer_queue_is_bounded`, `defer_drain_deadline_bounds_spinning_callback`,
    `shutdown_aborts_queued_deferred_work`) — the response is still handed off
    before any drain, and the drain is still deadline-bounded.
- **Load/cleanup tests** — handler-table load test now pins 64 registered
  handlers (was 60); settlement cleanup still runs on `ExecutionPhase::Cleanup`
  with `SETTLEMENT_GRACE`/`MAX_CLEANUP_JOBS`, untouched by the drain phase.
- **Operational docs** — `docs/specs/defer-api.md` updated (dedicated
  DeferredDrain phase, admission rule now enforced, op-free drain, promise-
  executor rejection nuance, embedded-prelude drain parity).

### Guardrail mapping (parent M4A-007)

- **Response is not delayed beyond defined handoff**: unchanged handoff-first
  ordering on all settlement paths; A tests re-verified green.
- **Deferred work is bounded**: cap + deadline unchanged; the separation adds
  no unbounded path (drain re-enqueue is now rejected outright).
- **Shutdown handles or aborts it deterministically**: shutdown semantics
  unchanged; embedded-prelude packs now participate in the same bounded drain.
- **Docs warn against durable-job use**: `docs/specs/defer-api.md` warning
  retained; ownership/phase semantics documented to match the enforced code.

### Command results

- `cargo test -p q-engine-quickjs` → **108 pass / 0 fail** (was 106; +2)
- `cargo test -p velqu-runtime --test runtime_conformance` → 35 pass / 0 fail
- `bun test` + `bun run typecheck` → clean (via verify)
- `cargo fmt --all --check` clean; workspace clippy -D warnings → exit 0
- `./scripts/verify` → **ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)**

### Disclosures

- Standing: CI `verify` workflows fail with zero executed steps on every PR
  since ~#714 (infrastructure-side); disclosed per PR. Local
  `./scripts/verify` is the gate evidence.
