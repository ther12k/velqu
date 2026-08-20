---
task_id: M24-007-Z
parent_task: M24-007
milestone: M24
priority: P0
mode: EVIDENCE
status: PASS
context_card: context/milestones/M24.md
commit_required: true
---

# M24-007-Z — Package evidence for Implement bounded read-once body admission

## Atomic goal

Create source-backed evidence and handoff for parent task M24-007; update status only if verification passed.

## Parent intent

Collect or stream request bodies only when declared and under route/global limits.

## Dependencies

- `M24-007-V` — `tasks/01_m24_zero_copy_ingress/M24-007-V-verify-implement-bounded-read-once-body-admission.md`

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

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

## Parent acceptance guardrails

- POST with no body contract does not collect body.
- DELETE/body routes work when declared.
- Oversize/slow bodies cancel cleanly.
- Client disconnect releases body work.

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

- Body-limit tests.
- Slowloris/partial-body tests.
- Cancellation metrics.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Evidence package

- Implementation commits: M24-007-A/B/C/D; verification commit: M24-007-V PR #682.
- RoutePlan `FieldNeeds.body` controls polling; DELETE/body and field-free route behavior verified.
- Bounded body transport uses `BytesMut`/`Bytes`; `Content-Length` over route limit rejects before polling, streaming overflow returns 413.
- One body representation mode per request generation; incompatible reads fail, settlement clears mode.
- Evidence tests: `routeplan_body_flag_controls_body_collection_independent_of_method`, `content_length_over_limit_rejects_before_body_poll`, `body_and_header_limits_reject_oversize`, `routing_precedes_body_materialization`, plus QuickJS body mode and disconnect coverage.
- Compiler fix included in verification: query schema IR uses `kind`, ensuring generated proof pack query IDs match QPack verification. Rebuilt proof pack and Bun runtime suite pass.
- `bun test`: PASS (35 pass, 0 fail).
- Rust targeted suites, typecheck, format, clippy, and OKF validation pass.
- Canonical benchmark manifests unchanged; no unsupported performance claims.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m24-007-z: package evidence for implement bounded read once body admiss
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
