---
task_id: M24-002-V
parent_task: M24-002
milestone: M24
priority: P0
mode: VERIFY
status: TODO
context_card: context/milestones/M24.md
commit_required: true
---

# M24-002-V — Verify Route before request materialization

## Atomic goal

Prove every acceptance criterion for parent task M24-002 without broadening scope.

## Parent intent

Avoid query/header/body work for routes that do not declare it.

## Dependencies

- `M24-002-A` — `tasks/01_m24_zero_copy_ingress/M24-002-A-keep-method-uri-headermap-and-body-stream-in-native-forms.md`
- `M24-002-B` — `tasks/01_m24_zero_copy_ingress/M24-002-B-match-routeid-using-method-path-before-creating-request-metadata.md`
- `M24-002-C` — `tasks/01_m24_zero_copy_ingress/M24-002-C-read-fieldneeds-from-routeplan.md`
- `M24-002-D` — `tasks/01_m24_zero_copy_ingress/M24-002-D-bypass-request-object-creation-for-policy-free-routes-that-need-no-request-field.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M24.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`

### Source files

- `AGENTS.md`
- `crates/q-http/src/lib.rs`
- `crates/q-bridge/src/lib.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-router/src/lib.rs`
- `crates/q-pack/src/lib.rs`
- `packages/compiler/src/emit.ts`
- `conformance/routing/routing.conformance.test.ts`
- `crates/q-runtime/tests/runtime_conformance.rs`
- `packages/treaty/src/index.ts`
- `packages/contract/src/index.ts`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- C0/C1 perform no query parse, header clone, cookie parse, or body collect.
- 404/405 does not materialize request bodies.
- Malformed oversized inputs fail within coarse ingress budgets.
- Routing behavior remains contract-equivalent.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
cargo test -p q-router
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

- Admission counters.
- Negative body/header budget tests.
- Perf stage timings.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m24-002-v: verify route before request materialization
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Verification blocker record

- Task ID: `M24-002-V`
- Status: `BLOCKED` (packet status remains `TODO`)
- Blocking fact: The A–D implementation and regression behavior are green, but the packet's required admission-counter and perf-stage-timing evidence is not present. `q-bridge::BridgeCounters` proves request-field materialization laziness, while the M24 ingress admission counters and stage histograms are explicitly owned by M24-009; existing `request.complete` `stage`/`durationMs` logs are per-request logs, not the required raw stage-timing evidence.
- Exact source locations: `crates/q-http/src/lib.rs:153-175, 257-332` (bounded semaphore admission and native URI/header checks without admission counters); `crates/q-runtime/src/serve.rs:108-146` (completion log stage/duration fields); `crates/q-bridge/src/lib.rs:45-74` (bridge materialization counters); `docs/specs/m24-ingress-ownership-and-admission.md:374-434` (required metric vocabulary and ownership boundary).
- Negative/positive tests proven: `routing_precedes_body_materialization`, `body_and_header_limits_reject_oversize`, `queue_limit_returns_503_when_saturated` in `crates/q-runtime/tests/runtime_conformance.rs`; `field_free_invocation_skips_request_store_slot` in `crates/q-engine-quickjs/tests/engine.rs`; `header_materialization_lowercases_names_and_keeps_values` in `crates/q-http/src/lib.rs`.
- Exact command results: targeted Rust suites passed (q-pack, q-router, q-engine-quickjs: 85 tests, q-http, q-bridge, velqu-runtime); `cargo fmt --check` PASS; `cargo clippy --workspace --all-targets -- -D warnings` PASS; `bun test` PASS (35/35); `bun run typecheck` PASS; `./scripts/validate-okf` PASS (`links_checked: 174`, `errors: []`). Raw logs: `/tmp/m24v2-rust.log`, `/tmp/m24v2-clippy.log`, `/tmp/m24v2-bun.log`, `/tmp/m24v2-type.log`, `/tmp/m24v2-okf.log`.
- Dependency or owner required: Complete M24-009 admission/stage observability (or provide equivalent committed raw counter/timing evidence), then rerun M24-002-V. Keep `M24-002-V`, `M24-002-Z`, `M24-001-V`, `M24-001-Z`, and `M24-GATE` TODO until their required evidence exists.
- Safe work completed before stopping: audited A–D source paths, mapped the parent guardrails to tests, ran the full acceptance matrix, and verified that no unsupported performance claim is being made.
- Files changed but not committed: this packet record only.
- Suggested next action: proceed to M24-003 implementation only through the dependency workflow, while leaving M24-002-V/Z TODO; return to M24-002-V after M24-009 evidence is available.
