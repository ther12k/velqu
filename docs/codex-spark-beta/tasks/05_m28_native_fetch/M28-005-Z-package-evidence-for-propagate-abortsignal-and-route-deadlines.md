---
task_id: M28-005-Z
parent_task: M28-005
milestone: M28
priority: P0
mode: EVIDENCE
status: PASS
context_card: context/milestones/M28.md
commit_required: true
---

# M28-005-Z — Package evidence for Propagate AbortSignal and route deadlines

## Atomic goal

Create source-backed evidence and handoff for parent task M28-005; update status only if verification passed.

## Parent intent

Ensure request cancellation physically stops outbound work and keeps ownership correct.

## Dependencies

- `M28-005-V` — `tasks/05_m28_native_fetch/M28-005-V-verify-propagate-abortsignal-and-route-deadlines.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M28.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`

### Source files

- `AGENTS.md`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-http/src/lib.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`
- `crates/q-pack/src/lib.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

## Parent acceptance guardrails

- No outbound task survives terminal invocation without defer ownership.
- Timeout counted once.
- Cancellation latency is bounded.
- Worker remains reusable.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p q-http
```
```bash
cargo test -p q-capabilities
```
```bash
cargo test -p velqu-runtime
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

- Race tests.
- Task accounting.
- Timeout/cancel conformance.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m28-005-z: package evidence for propagate abortsignal and route deadlin
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M28-005-Z) — PASS

- Date: 2026-08-28
- Branch/PR: m28-005-z (squash-merged; see git log for final hash)
- Closes: #335

### Parent closure — M28-005 Propagate AbortSignal and route deadlines

Parent intent: ensure request cancellation physically stops outbound work and keeps ownership correct. Status: **PASS**.

Packet commits (squash merges):
- M28-005-A — c18e96e (#933, Closes #330): Combined cancellation lifecycle test (`combined_route_deadline_abort_signal_and_shutdown_lifecycle`) covering explicit cancel, route-deadline expiry, worker reuse, and shutdown with pending ops
- M28-005-B — f948a7e (#934, Closes #331): Terminal-state invariant test (`each_operation_reaches_exactly_one_terminal_state`) — 8 mixed terminations (3 cancels + 2 timeouts + 3 completions), `started == completed + aborted + alive` after every phase, zero double-counting
- M28-005-C — b40012d (#935, Closes #332): Mid-flight cancellation test (`mid_flight_cancel_physically_stops_native_task_and_cleanup_cannot_escalate`) — abort reaches the sleeping Tokio task; cleanup cannot escalate to quarantine
- M28-005-D — 19e09ef (#936, Closes #333): Deterministic failure-mapping test (`terminal_failures_map_deterministically`) — EngineFailure/ContractViolation/Timeout/Problem identical across repeats
- M28-005-V — efeac29 (#937, Closes #334): Verification closure mapping all 4 acceptance guardrails + failure-mapping

### Evidence ledger (required microtask evidence)
- **Race tests**: completion-vs-abort CAS races (`completion_wins_abort_race_without_double_count`, `abort_actually_wins_completion_race`, `floating.race` matrix); cleanup-path races (`cleanup_poison_fails_all_pending_immediately`, `cleanup_interrupt_does_not_timeout_unrelated_invocation`).
- **Task accounting**: `native_tasks_started == completed + aborted + alive` verified after every scenario; zero underflow (`zero_delay_timer_does_not_wrap_alive_counter`); no accumulation (`repeated_floating_timers_do_not_accumulate_tasks` over 2000 requests).
- **Timeout/cancel conformance**: runtime mapping Timeout→`timeout`/504, RequestCapacity→`overload`/503, EngineFailure/ContractViolation→`internal`/500 redacted; cancellation latency bounded by SETTLEMENT_GRACE (100ms, not the 5s watchdog).

### Command results (this branch)
- `cargo test -p q-engine-quickjs` → 17 unit + 101 engine passed
- `cargo test -p velqu-runtime` → 8 unit + 5 integration + 31 conformance passed
- `cargo test -p q-capabilities` → 132+8 passed
- `cargo test -p q-http` → 4+6+1 passed
- `cargo test -p q-bridge` → 11 passed
- `cargo test -p q-pack` → 96+2 passed
- `bun test` → 219 pass / 0 fail (27 files)
- `bun run typecheck` → clean
- `cargo fmt --check` → clean
- `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**

### Ledger update
- `docs/beta/04_TASK_LEDGER.md`: M28-005 flipped TODO -> PASS.

### Disclosures (standing)
- CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR. Local evidence above is complete.
