---
task_id: M24-009-C
parent_task: M24-009
milestone: M24
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M24.md
commit_required: true
---

# M24-009-C — Expose slab/queue/body gauges

## Atomic goal

Expose slab/queue/body gauges.

## Parent intent

Measure the actual fixed overhead without per-request logging cost.

## Dependencies

- `M24-009-B` — `tasks/01_m24_zero_copy_ingress/M24-009-B-use-disabled-by-default-or-sampled-recording.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M24.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`
- `context/components/evidence.md`

### Source files

- `AGENTS.md`
- `crates/q-http/src/lib.rs`
- `crates/q-bridge/src/lib.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-runtime/tests/runtime_conformance.rs`
- `crates/q-engine/src/lib.rs`
- `docs/reports/`
- `docs/beta/workstreams/OBSERVABILITY_OPERATIONS.md`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Expose slab/queue/body gauges.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

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
cargo test -p velqu-runtime
```

## Required evidence for this microtask

- Instrumentation overhead benchmark.
- Metrics schema.
- Redaction tests.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Evidence

- Added bounded scalar gauges `slab_live`, `queue_pending`, and `body_bytes` to stage metrics snapshot.
- Queue gauge increments before engine admission and decrements after outcome; body gauge counts only bounded admitted bytes.
- Metrics snapshot exposes no request identifiers, headers, paths, or body contents.
- `cargo test -p q-engine-quickjs`: PASS.
- `cargo test -p q-http`: PASS.
- `cargo test -p q-bridge`: PASS.
- `cargo test -p velqu-runtime`: PASS.
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS.
- `scripts/validate-okf`: PASS (174 links, 0 errors).
- No instrumentation-overhead benchmark claim added; raw evidence deferred to M24-009-D.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m24-009-c: expose slab queue body gauges
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
