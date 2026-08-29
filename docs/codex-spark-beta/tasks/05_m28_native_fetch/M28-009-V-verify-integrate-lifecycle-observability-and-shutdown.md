---
task_id: M28-009-V
parent_task: M28-009
milestone: M28
priority: P1
mode: VERIFY
status: PASS
context_card: context/milestones/M28.md
commit_required: true
---

# M28-009-V — Verify Integrate lifecycle, observability, and shutdown

## Atomic goal

Prove every acceptance criterion for parent task M28-009 without broadening scope.

## Parent intent

Make fetch operationally diagnosable without hot-path logging cost.

## Dependencies

- `M28-009-A` — `tasks/05_m28_native_fetch/M28-009-A-expose-pool-wait-dns-connect-tls-ttfb-body-errors-cancellations.md`
- `M28-009-B` — `tasks/05_m28_native_fetch/M28-009-B-sample-aggregate-metrics.md`
- `M28-009-C` — `tasks/05_m28_native_fetch/M28-009-C-drain-pool-on-shutdown.md`
- `M28-009-D` — `tasks/05_m28_native_fetch/M28-009-D-quarantine-rejects-new-work.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M28.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`
- `context/components/capabilities-fetch.md`

### Source files

- `AGENTS.md`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-http/src/lib.rs`
- `Cargo.toml`
- `crates/q-engine/src/lib.rs`
- `docs/reports/`
- `docs/beta/workstreams/OBSERVABILITY_OPERATIONS.md`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- Metrics are bounded and redacted.
- Shutdown reaches quiescence.
- No task/connection leak after errors.
- Disabled instrumentation overhead is measured.

## Targeted commands

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

- Metrics schema.
- Shutdown tests.
- Overhead report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m28-009-v: verify integrate lifecycle observability and shutdown
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M28-009-V) — PASS

- Date: 2026-08-29
- Branch/PR: m28-009-v (squash-merged; see git log for final hash)
- Closes: #358

### Acceptance-criterion mapping (parent M28-009 guardrails)

1. **Metrics are bounded and redacted** — verified: fixed `[u64; 6]` stage array + saturating `u32` counters; serialized snapshot field set provably closed (9 keys, no URL/header/host fields possible); collector holds exactly one fixed-size struct. Tests: `stage_and_counter_observations_saturate_without_panicking`, `snapshot_field_set_is_the_redaction_boundary`, `collector_snapshot_serializes_redacted`, `collector_is_thread_safe_and_lossless`.
2. **Shutdown reaches quiescence** — verified: the runtime teardown drains the shared outbound pool within the ADR-0031 budget AFTER connections drain and the engine shuts down; the drain is reported in `shutdown.complete`. Tests: `graceful_shutdown_exits_zero` (extended, real SIGTERM path: `fetchPool {"initialized":false,"drained":true}`), `shared_pool_is_process_global_and_drains_in_place`, plus `drain_shutdown` budget-exceeded fail-closed (carried from M28-003).
3. **No task/connection leak after errors** — verified: a drained/quarantined pool rejects all new work at every layer and cannot resurrect its client; engine-level quarantine rejects new invocations immediately and resets pending ops to zero. Tests: `quarantined_pool_rejects_new_work_and_counts_refusals`, `quarantine rejects subsequent dynamic JS requests immediately`, `quarantine_accounting_drift_resets_pending_ops_to_zero`.
4. **Disabled instrumentation overhead is measured** — measured this packet (release build, `tests/metrics_overhead.rs`, informational no-assertion test): plain `observe_stage` ~0 ns/op (compiles to a saturating add); collector (mutex shard) ~22 ns/op; the disabled path is structurally zero (no call exists). Design bounds the enabled cost to a few integer adds per stage.

### Required evidence
- **Metrics schema**: `FetchStage` order + snapshot field mapping pinned; redaction boundary proven via serialized JSON.
- **Shutdown tests**: extended SIGTERM integration (fetchPool drain reported) + shared-pool unit tests.
- **Overhead report**: measured numbers above (10M iterations each, release profile, this machine).

### Verification runs (this branch, worktree-fresh)
- `cargo test -p q-capabilities` → 192 unit + 4 backpressure + 8 WPT passed (+1 overhead measurement, release)
- `cargo test -p q-engine-quickjs` → 18 unit + 101 engine passed
- `cargo test -p q-http` → 4+6+1 passed; `-p q-bridge` → 11 passed
- `cargo test -p velqu-runtime` → 10+5+31 passed
- `bun test` → 0 fail; `bun run typecheck` → clean (via ./scripts/verify)
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**; release binary reproduced deterministically (`5d2f6d9a…` matches the M28-009-D manifest)

### Disclosures (standing)
- The only code change in this packet is the informational overhead-measurement test (no assertions, no production impact).
- CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR. Local evidence above is complete.
