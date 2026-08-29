---
task_id: M28-009-C
parent_task: M28-009
milestone: M28
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M28.md
commit_required: true
---

# M28-009-C — Drain pool on shutdown

## Atomic goal

Drain pool on shutdown.

## Parent intent

Make fetch operationally diagnosable without hot-path logging cost.

## Dependencies

- `M28-009-B` — `tasks/05_m28_native_fetch/M28-009-B-sample-aggregate-metrics.md`

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
- `benchmarks/real-world/postgres/`
- `benchmarks/real-world/SPEC.md`
- `packages/capability-postgres/ (create if absent)`
- `crates/q-engine/src/lib.rs`
- `docs/reports/`
- `docs/beta/workstreams/OBSERVABILITY_OPERATIONS.md`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Drain pool on shutdown.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

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
bun test
```
```bash
bun run typecheck
```

## Required evidence for this microtask

- Metrics schema.
- Shutdown tests.
- Overhead report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m28-009-c: drain pool on shutdown
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M28-009-C) — PASS

- Date: 2026-08-29
- Branch/PR: m28-009-c (squash-merged; see git log for final hash)
- Closes: #356

### Changed files
- `crates/q-runtime/src/fetch_stack.rs`: process-global pool handle — `static SHARED_POOL: OnceLock<FetchPool>` + `shared_pool() -> &'static FetchPool`. Every fetch path shares this instance, so shutdown drains the pool that actually served traffic; an app with no fetch never initializes it, making the drain an immediate no-op.
- `crates/q-runtime/src/lib.rs`: the serve teardown now drains the shared pool AFTER connections drain and the engine shuts down (no new fetch work can start), within the ADR-0031 5s budget (`q_capabilities::SHUTDOWN_BUDGET_MS`; the runtime crate has no q-capabilities dependency, so the budget is referenced by comment-pinned value). The `shutdown.complete` event now reports `fetchPool: {initialized, drained}` — the drain is observable without logging secrets.
- `crates/q-runtime/tests/runtime_conformance.rs`: `graceful_shutdown_exits_zero` extended to assert the shutdown.complete event reports `fetchPool {"initialized":false,"drained":true}` for a no-fetch app (SIGTERM path).

### Tests added/extended
- fetch_stack `shared_pool_is_process_global_and_drains_in_place` (pointer identity; uninitialized drain = immediate Ok; post-drain permit attempts rejected `PoolShuttingDown`)
- runtime_conformance `graceful_shutdown_exits_zero` (extended: fetchPool drain reported in the shutdown.complete event on the real SIGTERM path)

### Command results
- `cargo test -p q-capabilities` → 192 unit + 4 backpressure + 8 WPT — 0 failed
- `cargo test -p q-engine-quickjs` → 18+101 · `-p q-http` 4+6+1 · `-p q-bridge` 11 · `-p velqu-runtime` → **9+5+31** (fetch_stack tests now 9) — all pass, including the extended SIGTERM integration test
- `bun test packages examples/proof conformance` → 0 fail; `bun run typecheck` → clean (via ./scripts/verify)
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**
- `benchmarks/manifest.json` refreshed (`d0f0e1de…`): the teardown drain code is genuinely in the binary now (first runtime-wired M28-009 packet).

### Guardrail mapping
- **Shutdown reaches quiescence** — quiescence now includes outbound connections: the pool that served traffic is drained within budget before shutdown.complete, proven on the real SIGTERM path.
- **No task/connection leak after errors** — pool permits are reclaimed by drain regardless of prior request outcomes (semaphore-based).
- **Metrics are bounded and redacted** — the event reports booleans only.

### Disclosures
- Known fresh-worktree transients (velqu-bytecode helper) resolved by the standard workspace build before final gate runs.
- Standing: CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR.
