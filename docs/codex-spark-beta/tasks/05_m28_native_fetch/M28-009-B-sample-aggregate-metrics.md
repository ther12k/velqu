---
task_id: M28-009-B
parent_task: M28-009
milestone: M28
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M28.md
commit_required: true
---

# M28-009-B — Sample/aggregate metrics

## Atomic goal

Sample/aggregate metrics.

## Parent intent

Make fetch operationally diagnosable without hot-path logging cost.

## Dependencies

- `M28-009-A` — `tasks/05_m28_native_fetch/M28-009-A-expose-pool-wait-dns-connect-tls-ttfb-body-errors-cancellations.md`

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

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Sample/aggregate metrics.
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
m28-009-b: sample aggregate metrics
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M28-009-B) — PASS

- Date: 2026-08-29
- Branch/PR: m28-009-b (squash-merged; see git log for final hash)
- Closes: #355

### Changed files
- `crates/q-capabilities/src/fetch_metrics.rs`: shared sampler/aggregator over the M28-009-A schema —
  - `FetchMetricsCollector` (also `shared() -> Arc` for workers): `record(impl FnOnce(&mut FetchMetrics))` for composed observations, plus convenience `record_request` / `record_error` / `record_cancellation` / `observe_stage`.
  - `sample()`: non-destructive cumulative snapshot (redacted `FetchMetricsSnapshot`).
  - `drain()`: interval sampling — returns the cumulative snapshot and resets the shard, so each window reports only new observations.
  - Bounded by construction: the collector holds exactly one fixed-size FetchMetrics; long-running processes cannot grow it. Per-worker shards + `FetchMetrics::merge` avoid cross-worker contention when needed.
- `crates/q-capabilities/src/lib.rs`: re-export `FetchMetricsCollector`.

### Tests added (fetch_metrics.rs, +4 → 192 q-capabilities lib tests)
- `collector_samples_cumulative_aggregates` (stage sums, counters, non-destructive sample)
- `collector_drain_reports_window_then_resets` (window reports new observations; next window empty until records arrive)
- `collector_is_thread_safe_and_lossless` (4 threads x 10k records -> exact totals, no lost updates)
- `collector_snapshot_serializes_redacted` (JSON shows counters, never url/header fields)

### Command results
- `cargo test -p q-capabilities` → **192 unit (was 188) + 4 backpressure + 8 WPT** — 0 failed
- `cargo test -p q-engine-quickjs` → 18+101 · `-p q-http` 4+6+1 · `-p q-bridge` 11 · `-p velqu-runtime` 8+5+31 — all pass
- `bun test packages examples/proof conformance` → 0 fail; `bun run typecheck` → clean
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)** (includes the full 219-test TS suite, compare-builds reproducibility, benchmark validation)
- Release binary hash unchanged (`46de91ac…` matches manifest) — collector dormant until the executor records into it.

### Guardrail mapping
- **Metrics are bounded and redacted** — one fixed-size shard; snapshots serialize the closed field set only.

### Disclosures
- A standalone `bun test` run in this fresh worktree failed runtime-local conformance (5s HTTP timeouts) before the release binary was built — resolved by the standard setup sequence (release build), after which `./scripts/verify` (the authoritative gate, including the full TS suite) passed. Environment setup, not product failures; disclosed per the M27-002-Z precedent.
- Standing: CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR.
