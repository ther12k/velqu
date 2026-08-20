---
task_id: M24-009-Z
parent_task: M24-009
milestone: M24
priority: P1
mode: EVIDENCE
status: PASS
context_card: context/milestones/M24.md
commit_required: true
---

# M24-009-Z — Package evidence for Add ingress and bridge observability

## Atomic goal

Create source-backed evidence and handoff for parent task M24-009; update status only if verification passed.

## Parent intent

Measure the actual fixed overhead without per-request logging cost.

## Dependencies

- `M24-009-V` — `tasks/01_m24_zero_copy_ingress/M24-009-V-verify-add-ingress-and-bridge-observability.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M24.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`
- `context/components/schema-codecs.md`

### Source files

- `AGENTS.md`
- `crates/q-http/src/lib.rs`
- `crates/q-bridge/src/lib.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-runtime/tests/runtime_conformance.rs`
- `crates/q-schema-runtime/src/lib.rs`
- `crates/q-schema-runtime/tests/fuzz_validator.rs`
- `crates/q-engine-quickjs/src/convert.rs`
- `packages/schema/src/index.ts`
- `conformance/schema/schema.conformance.test.ts`
- `benchmarks/harness/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

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
cargo test -p q-schema-runtime
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

- Instrumentation overhead benchmark.
- Metrics schema.
- Redaction tests.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Evidence

M24-009-A through D implementation commits and M24-009-V verification are merged through PR #694 merge `4ef8b7be0ea1b44bfcf0fc988effa450dffe836b`.

Packaged evidence:

- `StageMetricsSnapshot` schema covers seven stage counters and slab/queue/body gauges.
- `--log-sample N` provides bounded successful-request sampling; errors remain visible.
- Completion logs redact headers and body contents.
- Raw matched overhead evidence: `benchmarks/raw/observability/metrics-overhead.json`.
- Full Rust package suites, format, clippy, and `./scripts/verify` pass.
- Canonical benchmark manifest remains unchanged; no unsupported performance claim added.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m24-009-z: package evidence for add ingress and bridge observability
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
