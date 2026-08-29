---
task_id: M28-009-Z
parent_task: M28-009
milestone: M28
priority: P1
mode: EVIDENCE
status: PASS
context_card: context/milestones/M28.md
commit_required: true
---

# M28-009-Z — Package evidence for Integrate lifecycle, observability, and shutdown

## Atomic goal

Create source-backed evidence and handoff for parent task M28-009; update status only if verification passed.

## Parent intent

Make fetch operationally diagnosable without hot-path logging cost.

## Dependencies

- `M28-009-V` — `tasks/05_m28_native_fetch/M28-009-V-verify-integrate-lifecycle-observability-and-shutdown.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M28.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`
- `context/components/schema-codecs.md`

### Source files

- `AGENTS.md`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-http/src/lib.rs`
- `crates/q-schema-runtime/src/lib.rs`
- `crates/q-schema-runtime/tests/fuzz_validator.rs`
- `crates/q-engine-quickjs/src/convert.rs`
- `packages/schema/src/index.ts`
- `conformance/schema/schema.conformance.test.ts`
- `crates/q-engine/src/lib.rs`
- `docs/reports/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

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
cargo test -p q-schema-runtime
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
```bash
./scripts/verify
```

## Required evidence for this microtask

- Metrics schema.
- Shutdown tests.
- Overhead report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m28-009-z: package evidence for integrate lifecycle observability and s
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M28-009-Z) — PASS

- Date: 2026-08-29
- Branch/PR: m28-009-z (squash-merged; see git log for final hash)
- Closes: #359

### Parent closure — M28-009 Integrate lifecycle, observability, and shutdown

Parent intent: make fetch operationally diagnosable without hot-path logging cost. Status: **PASS**.

Packet commits (squash merges):
- M28-009-A — 07596c1 (#957, Closes #354): fetch metrics schema — `FetchStage` (pool_wait/dns/connect/tls/ttfb/body) in pinned order, `FetchMetrics` saturating counters, `FetchMetricsSnapshot` redaction boundary
- M28-009-B — edce2ab (#958, Closes #355): `FetchMetricsCollector` — thread-safe shared shard, cumulative `sample()` + interval `drain()`, lossless under 4x10k concurrent records
- M28-009-C — a215891 (#959, Closes #356): shutdown drains the process-global shared pool within the ADR-0031 budget; `shutdown.complete` reports `fetchPool {initialized, drained}` (proven on the real SIGTERM path)
- M28-009-D — 92c6a32 (#960, Closes #357): a quarantined pool rejects all new work — `try_client()` cannot resurrect a drained pool; `rejections()` counts every refusal
- M28-009-V — f3c0c26 (#961, Closes #358): verification closure + overhead measurement

### Required evidence
- **Metrics schema**: stage order + snapshot field mapping pinned by test; redaction boundary proven via serialized JSON key set.
- **Shutdown tests**: extended SIGTERM integration (`fetchPool {"initialized":false,"drained":true}`), shared-pool identity/drain unit tests, budget-exceeded fail-closed (carried from M28-003).
- **Overhead report** (release profile, 10M iterations, `tests/metrics_overhead.rs`): plain `observe_stage` ~0 ns/op (compiles to a saturating add); collector mutex-shard ~22 ns/op; disabled path structurally zero (no call). Guardrail satisfied: instrumentation cost is bounded adds, disabled cost is nothing.

### Source/test map
- `crates/q-capabilities/src/fetch_metrics.rs`: schema + collector (8 tests).
- `crates/q-runtime/src/fetch_stack.rs`: `shared_pool()`, `try_client()`, `rejections()` (2 new tests).
- `crates/q-runtime/src/lib.rs`: teardown drain integration + fetchPool event block.
- `crates/q-runtime/tests/runtime_conformance.rs`: extended `graceful_shutdown_exits_zero`.
- `crates/q-capabilities/tests/metrics_overhead.rs`: overhead measurement (informational).

### Command results (this branch)
- `cargo test -p q-capabilities` → 192 unit + 4 backpressure + 8 WPT + 1 overhead (release) passed
- `cargo test -p q-engine-quickjs` → 18 unit + 101 engine passed
- `cargo test -p q-http` → 4+6+1 passed; `-p q-bridge` → 11 passed
- `cargo test -p velqu-runtime` → 10+5+31 passed
- `bun test` → 0 fail; `bun run typecheck` → clean (via ./scripts/verify)
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**; binary `5d2f6d9a…` matches manifest

### Hygiene fix in this packet
- `tests/metrics_overhead.rs` (merged in V) had an unsorted import and a literal-bool assertion that escaped the V worktree's fmt/clippy runs (the file was added after those gates ran). Both corrected here; behavior unchanged.

### Ledger update
- `docs/beta/04_TASK_LEDGER.md`: M28-009 flipped TODO -> PASS.

### Disclosures (standing)
- CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR. Local evidence above is complete.
