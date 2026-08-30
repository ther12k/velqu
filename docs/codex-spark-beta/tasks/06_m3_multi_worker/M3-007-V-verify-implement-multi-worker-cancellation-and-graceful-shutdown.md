---
task_id: M3-007-V
parent_task: M3-007
milestone: M3
priority: P0
mode: VERIFY
status: PASS
context_card: context/milestones/M3.md
commit_required: true
---

# M3-007-V — Verify Implement multi-worker cancellation and graceful shutdown

## Atomic goal

Prove every acceptance criterion for parent task M3-007 without broadening scope.

## Parent intent

Propagate cancellation and shutdown to the owning worker and native operations exactly once.

## Dependencies

- `M3-007-A` — `tasks/06_m3_multi_worker/M3-007-A-track-invocation-to-worker-ownership.md`
- `M3-007-B` — `tasks/06_m3_multi_worker/M3-007-B-stop-admission-on-drain.md`
- `M3-007-C` — `tasks/06_m3_multi_worker/M3-007-C-allow-bounded-in-flight-completion.md`
- `M3-007-D` — `tasks/06_m3_multi_worker/M3-007-D-abort-after-shutdown-deadline.md`

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
- `benchmarks/real-world/postgres/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

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

- Shutdown integration tests.
- Disconnect/cancel races.
- Resource invariant report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m3-007-v: verify implement multi worker cancellation and graceful shut
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M3-007-V) — PASS

- Date: 2026-08-31
- Branch/PR: m3-007-v (squash-merged; see git log for final hash)
- Closes: #412

### Acceptance-criterion mapping (parent M3-007 guardrails)

1. **No orphan invocation/native task** — verified.
   - Ownership exactly-once: `settle_is_the_exactly_once_gate_for_cancel_routing`
     (two-thread settle race yields exactly one winner),
     `concurrent_admission_and_settlement_stays_consistent` (4×250 admissions +
     concurrent settlement, balance invariant pinned after the race),
     `no_orphan_audit_over_a_full_admit_settle_cycle` (A, q-capabilities).
   - Runtime pin: `shutdown.complete` carries `invocations.pending:0` on a
     graceful drain (`graceful_shutdown_exits_zero`).
   - Forced-abort path: abort-through-ownership runs CancelOnDrop inside
     `serve()`; `drain_waits_bounded_then_detaches_straggler_connection` pins
     `pending:0, registered:1, settled:1` DETERMINISTICALLY plus
     `stats.cancelled_invocations == 1` and `native_tasks_aborted == 1` (D);
     the defensive sweep in run() makes a silent orphan impossible.
2. **Shutdown deadline is honored** — verified: the full chain (graceful close
   of idle connections → bounded wait for dispatched work → forced abort) is
   bounded by the ADR-0031 5s budget. `drain_waits_bounded...` pins BOTH bounds:
   elapsed ≥ 5s (budget actually honored, not skipped) and < 10s (exit within
   the window). `drain_lets_in_flight_request_complete` pins the prompt path
   (< 5s when nothing straggles).
3. **Exit code/report reflects forced aborts** — verified: the report carries
   `drain.aborted` (forced-abort count), `stats.cancelled_invocations`, and
   `stats.native_tasks_aborted`; exit stays 0 — a deadline-bounded shutdown
   that reports honestly is a successful shutdown (asserted exit 0 in all
   drain tests, including the aborted-straggler one).
4. **All slots/queues/pools quiesce** — verified:
   - admission: the drain gate refuses dynamic admission from the flip instant
     (`graceful_drain_flips_gate_and_reports_before_exit` + 6 DrainGate unit
     tests, B);
   - queues: dispatcher close/quarantine/quiesce semantics from M3-002/M3-005
     (regression-covered: `close_all_shuts_every_queue_down`,
     `settle_quarantined_drains_pending_jobs_for_typed_failure`);
   - connections: idle keep-alives close at once, dispatched requests complete
     (`drain_lets_in_flight_request_complete`: full `waited:800` response AFTER
     the signal, prompt exit);
   - outbound pool: `fetchPool {initialized, drained:true}` in every
     shutdown.complete (M28-009-C, regression-covered).

### Disconnect/cancel race evidence
- `CancelOnDrop` is ownership-routed with settle-as-gate: a disconnect racing a
  delivered outcome can never double-cancel (serve.rs settle-before-disarm
  ordering; `debug_assert` pins the always-settles-here invariant).
- Registry-level: the two-thread settle race test above; engine-level cancel
  races remain covered by the M2.2.1 suite (`cargo test -p q-engine-quickjs`
  20+102+1, incl. cancellation and late-completion tests).

### Verification runs (this branch, worktree-fresh)
- `cargo test -p q-engine-quickjs` → 20 + 102 + 1 — 0 failed
- `cargo test -p q-http` → 4 + 6 — 0 failed
- `cargo test -p q-bridge` → 11 — 0 failed
- `cargo test -p velqu-runtime` → 55 unit + 6 + 5 + 2 + 35 conformance — 0 failed
- `cargo test -p q-capabilities` → 237 lib + 7 fuzz + 1 + 4 + 9 WPT-manifest — 0 failed
- `bun test` → **219 pass, 0 fail** (27 files; after the standard fresh-worktree
  release+proof build — the initial 9 failures were missing artifacts, not
  regressions)
- `bun run typecheck` → clean
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets --
  -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS** (release binary hash matches the manifest
  pinned by M3-007-D)

### Disclosures (standing)
- No production code changed in this packet: verification-only closure of
  M3-007-A/B/C/D.
- CI fails with zero executed steps on every PR since ~#714
  (infrastructure-side); disclosed per PR. Local evidence above is complete.
