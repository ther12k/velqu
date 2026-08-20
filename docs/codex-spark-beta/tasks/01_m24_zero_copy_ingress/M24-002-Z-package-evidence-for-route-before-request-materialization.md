---
task_id: M24-002-Z
parent_task: M24-002
milestone: M24
priority: P0
mode: EVIDENCE
status: PASS
context_card: context/milestones/M24.md
commit_required: true
---

# M24-002-Z — Package evidence for Route before request materialization

## Atomic goal

Create source-backed evidence and handoff for parent task M24-002; update status only if verification passed.

## Parent intent

Avoid query/header/body work for routes that do not declare it.

## Dependencies

- `M24-002-V` — `tasks/01_m24_zero_copy_ingress/M24-002-V-verify-route-before-request-materialization.md`

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

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

## Parent acceptance guardrails

- C0/C1 perform no query parse, header clone, cookie parse, or body collect.
- 404/405 does not materialize request bodies.
- Malformed oversized inputs fail within coarse ingress budgets.
- Routing behavior remains contract-equivalent.

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

- Source-backed evidence package for the M24-002 route-first and bounded-admission guardrails.
- Negative body/header/queue tests, requestless bridge counter proof, and routing/policy conformance.
- Exact merged implementation and verification commits, changed files, command results, and raw log paths.
- Explicit evidence boundary: aggregate ingress counters, stage histograms, and instrumentation-overhead benchmarks remain M24-009 deliverables and are not claimed by this M24-002 package.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m24-002-z: package evidence for route before request materialization
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record

- Status: **PASS**
- Parent verification: M24-002-V merged by PR #644 as commit `94689c7`; issue #70 closed.
- Implementation commits: M24-002-A `e6dfa26`, M24-002-B `c4409d8`, M24-002-C `57934c0`, and M24-002-D `1eaa3f7`.
- Evidence package: native `Method`/`Uri`/`HeaderMap`/body ownership in `crates/q-http/src/lib.rs`; native route-first matching, FieldNeeds gates, route-bound bounded body admission, and requestless predicate in `crates/q-runtime/src/serve.rs`; generation-checked bridge counters in `crates/q-bridge/src/lib.rs`; requestless QuickJS sentinel handling in `crates/q-engine-quickjs/src/prelude.rs`, `worker.rs`, and `tests/engine.rs`.
- Exact tests: `routing_precedes_body_materialization`, `body_and_header_limits_reject_oversize`, `queue_limit_returns_503_when_saturated`, and `full_runtime_conformance` in `crates/q-runtime/tests/runtime_conformance.rs`; `field_free_invocation_skips_request_store_slot` in `crates/q-engine-quickjs/tests/engine.rs`; `header_materialization_lowercases_names_and_keeps_values` in `crates/q-http/src/lib.rs`; q-pack field-needs verification; q-router property/conformance; policy/security and liveness conformance.
- Exact verification: targeted Rust matrix passed (q-engine-quickjs 85, q-http 2 + 3 parser tests, q-bridge 4, velqu-runtime 13 runtime-conformance tests); the isolated `graceful_shutdown_exits_zero` retry also passed; `cargo fmt --check` PASS; `cargo clippy --workspace --all-targets -- -D warnings` PASS; `bun run typecheck` PASS; proof build PASS; `bun test` PASS (35/35, 141 assertions); `./scripts/validate-okf` PASS (174 links, 0 errors). Raw logs: `/tmp/m24z-rust.log`, `/tmp/m24z-fmt.log`, `/tmp/m24z-clippy.log`, `/tmp/m24z-type.log`, `/tmp/m24z-bun2.log`, `/tmp/m24z-okf.log`. The first parallel Bun attempt ran before proof artifacts existed and is not used as acceptance evidence; after `bun install --frozen-lockfile` and the proof build, the complete Bun suite passed.
- Repository verification note: `./scripts/verify` was run and all Rust, TypeScript, proof-build, and Bun stages completed, but `validate-benchmark-evidence` reported the canonical `qRuntimeRelease` hash mismatch for the temporary worktree build. No benchmark manifest or performance claim was changed; this is not M24-002 evidence.
- Acceptance criteria: C0/C1 avoid premature query/header/cookie/body materialization; 404/405 answer before an incomplete body finishes; URI/header/body limits are bounded; queue-full returns 503 with `Retry-After: 1`; field-free requestless routes allocate no request-store slot; routing/policy behavior remains contract-equivalent.
- Evidence boundary: bridge counters prove lazy/requestless materialization only. Aggregate ingress counters, stage histograms, and instrumentation-overhead benchmarks remain M24-009 deliverables. M24-001-V/Z and M24-GATE remain TODO.
- Changed files in this evidence commit: this packet, `docs/codex-spark-beta/STATUS.md`, `docs/codex-spark-beta/indexes/TASK_INDEX.md`, `docs/codex-spark-beta/indexes/EXECUTION_QUEUE.md`, and `docs/codex-spark-beta/indexes/NEXT_25.md`.
- Next dependency-ready task: none within the authorized order; M24-003 remains blocked by M24-001-Z.
