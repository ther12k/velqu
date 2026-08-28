---
task_id: M28-005-V
parent_task: M28-005
milestone: M28
priority: P0
mode: VERIFY
status: PASS
context_card: context/milestones/M28.md
commit_required: true
---

# M28-005-V — Verify Propagate AbortSignal and route deadlines

## Atomic goal

Prove every acceptance criterion for parent task M28-005 without broadening scope.

## Parent intent

Ensure request cancellation physically stops outbound work and keeps ownership correct.

## Dependencies

- `M28-005-A` — `tasks/05_m28_native_fetch/M28-005-A-combine-explicit-abort-route-deadline-disconnect-shutdown-and-quarantine.md`
- `M28-005-B` — `tasks/05_m28_native_fetch/M28-005-B-use-one-terminal-state-for-each-operation.md`
- `M28-005-C` — `tasks/05_m28_native_fetch/M28-005-C-cancel-dns-connect-body-streaming.md`
- `M28-005-D` — `tasks/05_m28_native_fetch/M28-005-D-map-failures-deterministically.md`

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
- `crates/q-bridge/src/lib.rs`
- `crates/q-runtime/tests/runtime_conformance.rs`
- `crates/q-pack/src/lib.rs`
- `benchmarks/harness/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

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
cargo test -p q-bridge
```
```bash
cargo test -p q-capabilities
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

## Required evidence for this microtask

- Race tests.
- Task accounting.
- Timeout/cancel conformance.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m28-005-v: verify propagate abortsignal and route deadlines
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M28-005-V) — PASS

- Date: 2026-08-28
- Branch/PR: m28-005-v (squash-merged; see git log for final hash)
- Closes: #334

### Acceptance-criterion mapping (parent M28-005 guardrails)

1. **No outbound task survives terminal invocation without defer ownership** — verified: `combined_route_deadline_abort_signal_and_shutdown_lifecycle` (`engine.rs`), `each_operation_reaches_exactly_one_terminal_state`, and `mid_flight_cancel_physically_stops_native_task_and_cleanup_cannot_escalate` all prove `native_tasks_alive == 0` after every cancellation, timeout, and shutdown stage; cleanup reactions cannot spawn second-generation ops (`cleanup_reaction_cannot_start_native_operation`, `cancel_reaction_cannot_spawn_second_generation_op`).
2. **Timeout counted once** — verified: `timeouts == 1` after route-deadline expiry in the combined lifecycle test; abort lands via single CAS in `abort_op_task` (`completion_wins_abort_race_without_double_count`, `abort_actually_wins_completion_race`).
3. **Cancellation latency is bounded** — verified: `floating_timer_aborts_underlying_tokio_task` (no late completions), `post_settlement_floating_cleanup_uses_cleanup_budget` (100ms grace, not the 5s watchdog), `ordinary_async_timeout_does_not_quarantine_worker` (prompt timeout, no quarantine).
4. **Worker remains reusable** — verified: `sync_runaway_microtask_leaves_worker_reusable`, `deadline_timeout_interrupts_and_replies`, `response_mapping_timeout_leaves_worker_reusable`, and the tail of every M28-005 A–D test (workers serve `js.text` after cancellations/timeouts).

Failure-mapping guardrail (D packet): `terminal_failures_map_deterministically` — every terminal outcome maps identically across repeats (EngineFailure/ContractViolation/Timeout/Problem).

### Verification runs (this branch, worktree-fresh)
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

### Disclosures (standing)
- CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR. Local evidence above is complete.
