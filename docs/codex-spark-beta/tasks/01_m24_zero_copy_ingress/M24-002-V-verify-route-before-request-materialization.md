---
task_id: M24-002-V
parent_task: M24-002
milestone: M24
priority: P0
mode: VERIFY
status: PASS
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

- Native route-first and bounded-admission source/tests proving the parent guardrails.
- Negative body/header/queue tests and the requestless bridge counter proof.
- Routing and policy/security conformance proving behavior remains contract-equivalent.
- Explicit evidence boundary: aggregate ingress counters, stage histograms, and instrumentation-overhead benchmarks are M24-009 deliverables and are not prerequisites for this M24-002 verification packet. Existing per-request stage/duration logs are not claimed as M24-009 benchmark evidence.

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

## Completion record

- Status: **PASS**
- Deliverable: verification of the M24-002 A–D route-before-materialization implementation against the parent guardrails. Native routing occurs before query/header/body materialization; body reads are route-bound and bounded; field-free policy-free routes bypass request-store allocation.
- Changed files: `docs/codex-spark-beta/tasks/01_m24_zero_copy_ingress/M24-002-V-verify-route-before-request-materialization.md`, `docs/codex-spark-beta/STATUS.md`, `docs/codex-spark-beta/indexes/TASK_INDEX.md`, `docs/codex-spark-beta/indexes/EXECUTION_QUEUE.md`, and `docs/codex-spark-beta/indexes/NEXT_25.md`.
- Source evidence: `crates/q-http/src/lib.rs` native admission and `header_materialization_lowercases_names_and_keeps_values`; `crates/q-runtime/src/serve.rs` native method/path routing, FieldNeeds gates, route-bound body admission, and bounded limits; `crates/q-bridge/src/lib.rs` generation-checked bridge counters; `crates/q-engine-quickjs/tests/engine.rs` `field_free_invocation_skips_request_store_slot`; `crates/q-runtime/tests/runtime_conformance.rs` `routing_precedes_body_materialization`, `body_and_header_limits_reject_oversize`, `queue_limit_returns_503_when_saturated`, routing, policy, and liveness conformance.
- Exact command results: targeted Rust suites passed — q-pack 34 + 2 fuzz tests, q-router 12, q-engine-quickjs 85, q-http, q-bridge 4, and velqu-runtime 13 runtime-conformance tests; `cargo fmt --check` PASS; `cargo clippy --workspace --all-targets -- -D warnings` PASS; `bun test` PASS (35/35, 141 assertions); `bun run typecheck` PASS; `./scripts/validate-okf` PASS (`links_checked: 174`, `errors: []`). Raw logs: `/tmp/m24v3-rust.log`, `/tmp/m24v3-fmt.log`, `/tmp/m24v3-clippy.log`, `/tmp/m24v3-bun.log`, `/tmp/m24v3-type.log`, `/tmp/m24v3-okf.log`. The full `./scripts/verify` matrix was also run; its only failure was the pre-existing canonical benchmark-manifest `qRuntimeRelease` hash mismatch for the temporary worktree build (`validate-benchmark-evidence`), while its Rust, TypeScript, proof-build, and Bun stages completed successfully. No benchmark manifest or performance claim was changed.
- Acceptance criteria proven: C0/C1 route and native liveness paths avoid query parsing, header cloning, cookie parsing, and body polling; 404/405 answer before an incomplete body finishes; URI/header/body limits remain bounded; queue-full returns 503 with `Retry-After: 1`; requestless execution creates no request-store slot or bridge materialization; routing and policy/security behavior remain contract-equivalent.
- Evidence boundary: `BridgeCounters` are used only for lazy/requestless materialization proof. Aggregate ingress counters, stage histograms, and instrumentation-overhead benchmarks remain M24-009 deliverables and are not claimed by M24-002-V. Existing per-request `stage`/`durationMs` logs are not used as M24-009 benchmark evidence.
- Remaining risk: M24-001-V/Z remain TODO by instruction; worker-local request slab and later M24 packets remain incomplete; M24-GATE remains TODO.
- Next dependency-ready task: M24-002-Z (package evidence for Route before request materialization).
