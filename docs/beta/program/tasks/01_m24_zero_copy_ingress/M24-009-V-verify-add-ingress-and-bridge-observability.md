---
task_id: M24-009-V
parent_task: M24-009
milestone: M24
priority: P1
mode: VERIFY
status: PASS
context_card: context/milestones/M24.md
commit_required: true
---

# M24-009-V — Verify Add ingress and bridge observability

## Atomic goal

Prove every acceptance criterion for parent task M24-009 without broadening scope.

## Parent intent

Measure the actual fixed overhead without per-request logging cost.

## Dependencies

- `M24-009-A` — `tasks/01_m24_zero_copy_ingress/M24-009-A-add-counters-histograms-for-route-queue-decode-bridge-js-encode-and-write-stages.md`
- `M24-009-B` — `tasks/01_m24_zero_copy_ingress/M24-009-B-use-disabled-by-default-or-sampled-recording.md`
- `M24-009-C` — `tasks/01_m24_zero_copy_ingress/M24-009-C-expose-slab-queue-body-gauges.md`
- `M24-009-D` — `tasks/01_m24_zero_copy_ingress/M24-009-D-measure-instrumentation-overhead.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M24.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`
- `context/components/capabilities-fetch.md`

### Source files

- `AGENTS.md`
- `crates/q-http/src/lib.rs`
- `crates/q-bridge/src/lib.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-runtime/tests/runtime_conformance.rs`
- `crates/q-capabilities/src/lib.rs`
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

- Logging off path adds no formatting or timing work beyond approved counters.
- Metrics are bounded and reset/testable.
- Stage timings identify regressions.
- No sensitive request data is emitted.

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
cargo fmt --check
```
```bash
cargo clippy --workspace --all-targets -- -D warnings
```
```bash
./scripts/verify
```

## Required evidence for this microtask

- Instrumentation overhead benchmark.
- Metrics schema.
- Redaction tests.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Evidence

Acceptance matrix:

- Stage schema: `StageMetricsSnapshot` exposes route, queue, decode, bridge, JS, encode, write counters plus slab_live, queue_pending, body_bytes gauges.
- Sampling: `--log-sample N` records successful requests every Nth request; errors remain unsampled; zero preserves default behavior.
- Redaction: completion logs include request ID, route ID, method, path, status, body size, and stage only; no headers or body contents.
- Boundedness: all metrics are scalar atomics; queue gauge decrements after outcome; body gauge counts admitted bytes only.
- Raw overhead: `benchmarks/raw/observability/metrics-overhead.json`, 100,000 matched samples with mean/p50/p95/p99 for disabled and atomic paths.
- `cargo test -p q-engine-quickjs`: PASS.
- `cargo test -p q-http`: PASS.
- `cargo test -p q-bridge`: PASS.
- `cargo test -p q-capabilities`: PASS.
- `cargo test -p velqu-runtime`: PASS.
- `cargo fmt --check`: PASS.
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS.
- `./scripts/verify`: PASS.
- No benchmark manifest rewritten and no unsupported performance claim added.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m24-009-v: verify add ingress and bridge observability
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
