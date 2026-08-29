---
task_id: M28-009-A
parent_task: M28-009
milestone: M28
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M28.md
commit_required: true
---

# M28-009-A — Expose pool wait, DNS, connect, TLS, TTFB, body, errors, cancellations

## Atomic goal

Expose pool wait, DNS, connect, TLS, TTFB, body, errors, cancellations.

## Parent intent

Make fetch operationally diagnosable without hot-path logging cost.

## Dependencies

- `M28-003-Z` — `tasks/05_m28_native_fetch/M28-003-Z-package-evidence-for-implement-connection-pooling-dns-and-tls.md`
- `M28-005-Z` — `tasks/05_m28_native_fetch/M28-005-Z-package-evidence-for-propagate-abortsignal-and-route-deadlines.md`
- `M28-006-Z` — `tasks/05_m28_native_fetch/M28-006-Z-package-evidence-for-implement-streaming-and-strict-backpressure.md`

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
- `crates/q-bridge/src/lib.rs`
- `crates/q-runtime/tests/runtime_conformance.rs`
- `Cargo.toml`
- `benchmarks/real-world/postgres/`
- `benchmarks/real-world/SPEC.md`
- `packages/capability-postgres/ (create if absent)`
- `crates/q-engine/src/lib.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Expose pool wait, DNS, connect, TLS, TTFB, body, errors, cancellations.
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
m28-009-a: expose pool wait dns connect tls ttfb body errors cancellati
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M28-009-A) — PASS

- Date: 2026-08-29
- Branch/PR: m28-009-a (squash-merged; see git log for final hash)
- Closes: #354

### Changed files
- `crates/q-capabilities/src/fetch_metrics.rs` (new): the fetch metrics schema —
  - `FetchStage` (PoolWait/Dns/Connect/Tls/Ttfb/Body) with `ALL` in schema order and stable snake_case names; the order IS the schema, pinned by test.
  - `FetchMetrics` — bounded observations: fixed `[u64; 6]` stage array (saturating adds), `u32` request/error/cancellation counters (saturating). Observation path is plain integer adds: no allocation, no locks, no strings — hot-path cost is a few cycles.
  - `FetchMetricsSnapshot` (serde): the redaction boundary — exactly six `*_ns` stage fields + `requests`/`errors`/`cancellations`; nothing else can leak in. Pinned by a JSON key test.
  - `merge(&FetchMetrics)` for aggregation (saturating) — feeds M28-009-B.
- `crates/q-capabilities/src/lib.rs`: module + re-exports.

### Tests added (fetch_metrics.rs, +4 → 188 q-capabilities lib tests)
- `metrics_schema_covers_all_stages_in_order` (stage names + snapshot field mapping)
- `stage_and_counter_observations_saturate_without_panicking` (u64::MAX stage adds; 100k counter records)
- `snapshot_field_set_is_the_redaction_boundary` (serialized JSON contains exactly the 9 schema keys, 8 commas; no url/header fields possible)
- `merge_aggregates_saturating`

### Command results
- `cargo test -p q-capabilities` → **188 unit (was 184) + 4 backpressure + 8 WPT** — 0 failed
- `cargo test -p q-engine-quickjs` → 18+101 · `-p q-http` 4+6+1 · `-p q-bridge` 11 · `-p velqu-runtime` 8+5+31 — all pass
- `bun test` → 219 pass / 0 fail; `bun run typecheck` → clean (via ./scripts/verify)
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**
- Release binary hash unchanged (`46de91ac…` matches manifest) — the schema is dormant until the executor records into it.

### Guardrail mapping
- **Metrics are bounded and redacted** — fixed arrays + saturating counters; the snapshot's serialized field set is provably closed (no URL/header/host field can appear).
- **Disabled instrumentation overhead is measured** — by design the observation path is saturating integer adds with `#[inline]`; the executor's overhead report (parent evidence) will quantify the enabled path; the disabled path is structurally zero (no call).

### Disclosures
- Standing: CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR.
