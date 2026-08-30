---
task_id: M3-007-D
parent_task: M3-007
milestone: M3
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M3.md
commit_required: true
---

# M3-007-D — Abort after shutdown deadline

## Atomic goal

Abort after shutdown deadline.

## Parent intent

Propagate cancellation and shutdown to the owning worker and native operations exactly once.

## Dependencies

- `M3-007-C` — `tasks/06_m3_multi_worker/M3-007-C-allow-bounded-in-flight-completion.md`

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
5. Implement exactly this deliverable: Abort after shutdown deadline.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- No orphan invocation/native task.
- Shutdown deadline is honored.
- Exit code/report reflects forced aborts.
- All slots/queues/pools quiesce.

## Targeted commands

```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p velqu-runtime
```

## Required evidence for this microtask

- Shutdown integration tests.
- Disconnect/cancel races.
- Resource invariant report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m3-007-d: abort after shutdown deadline
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M3-007-D) — PASS

- Date: 2026-08-31
- Branch/PR: m3-007-d (squash-merged; see git log for final hash)
- Closes: #411

### Changed files
- `crates/q-http/src/lib.rs`: the C-era budget-expiry path now FORCE-ABORTS
  through ownership instead of detaching. `connections.abort_all()` drops each
  connection future, which drops the in-flight handler future and runs its
  `CancelOnDrop` guard — the ownership binding settles and `Engine::cancel` is
  delivered exactly once; the worker's settlement owner aborts pending native
  ops. Aborted tasks are reaped WITHOUT counting as completed (this packet's
  test run caught the double-count: the report showed completed:1,aborted:1 for
  a single straggler). `ServeDrain.detached` → `ServeDrain.aborted`.
- `crates/q-runtime/src/lib.rs`: after serve returns, a DEFENSIVE ownership
  sweep settles any still-live binding (a leaked cancel path would be visible,
  never silent) and counts it as a forced abort; the shutdown report now carries
  `{"drain":{"refused":N,"completed":C,"aborted":A}}` — A is the forced-abort
  count, and invocations are pinned to `pending:0` deterministically.
- `crates/q-runtime/tests/runtime_conformance.rs`: straggler test upgraded from
  the C-era scheduling-race invariant to the deterministic D assertions.
- `benchmarks/manifest.json`: refreshed after rebuilding the release binary
  with verify's exact `--remap-path-prefix` flags (a refresh against an
  unremapped build recorded mismatching hashes — root-caused and fixed by
  building with the verify environment before refreshing).

### Tests changed
- `drain_waits_bounded_then_detaches_straggler_connection`: now asserts
  - `"drain":{"refused":0,"completed":0,"aborted":1}` (the straggler is
    force-aborted at the budget — awaited forever it is not),
  - `invocations: {pending:0, registered:1, settled:1}` — DETERMINISTIC (no
    scheduling race): the abort runs CancelOnDrop inside `serve()` before the
    report prints, closing the C/D boundary honestly,
  - `stats.cancelled_invocations == 1` — the engine recorded the forced
    cancellation,
  - elapsed ≥ the 5s budget (the bound was honored) and < 10s (bounded exit 0).
- The B/C-era drain tests follow the report-key rename (`detached` →
  `aborted`), values unchanged.

### Command results
- `cargo test -p q-http` → 4 + 6 — 0 failed
- `cargo test -p q-capabilities` → 237 + 7 + 1 + 4 + 9 — 0 failed
- `cargo test -p q-engine-quickjs` → 20 + 102 + 1 — 0 failed
- `cargo test -p velqu-runtime` → 55 unit + 6 + 5 + 2 + 35 conformance — 0 failed
- `./scripts/verify` → **ALL PASS** (fmt clean, clippy -D warnings clean, bun
  183 tests / 21 files)

### Guardrail mapping (parent M3-007 — complete)
- **No orphan invocation/native task** — abort-through-ownership settles every
  binding exactly once (report-pinned pending:0) and the worker aborts pending
  native ops (`native_tasks_aborted` in the report); the defensive sweep makes
  a silent orphan impossible.
- **Shutdown deadline is honored** — the full chain (graceful close → bounded
  wait → forced abort) is bounded by the ADR-0031 budget; the straggler test
  pins both the lower bound (budget honored) and the upper bound (exit).
- **Exit code/report reflects forced aborts** — `drain.aborted`,
  `stats.cancelled_invocations`, and `stats.native_tasks_aborted` carry the
  forced-abort record; exit stays 0 (a deadline-bounded shutdown that reports
  honestly is a successful shutdown).
- **All slots/queues/pools quiesce** — A (ownership) + B (admission gate) +
  C (graceful completion) + D (bounded abort) close the parent's four
  guardrails.

### Disclosures
- The straggler test now also surfaced and fixed a report double-count
  (aborted tasks were reaped into `completed`) — caught by the test, fixed in
  the same packet.
- Standing: CI fails with zero executed steps on every PR since ~#714
  (infrastructure-side); disclosed per PR.
