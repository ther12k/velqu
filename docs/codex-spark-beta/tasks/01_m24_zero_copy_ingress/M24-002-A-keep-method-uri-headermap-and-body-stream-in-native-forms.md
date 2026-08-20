---
task_id: M24-002-A
parent_task: M24-002
milestone: M24
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M24.md
commit_required: true
---

# M24-002-A — Keep Method, Uri, HeaderMap, and body stream in native forms

## Atomic goal

Keep Method, Uri, HeaderMap, and body stream in native forms.

## Parent intent

Avoid query/header/body work for routes that do not declare it.

## Dependencies

- `M24-001-Z` — `tasks/01_m24_zero_copy_ingress/M24-001-Z-package-evidence-for-freeze-ingress-ownership-and-backpressure-design.md`

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

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Keep Method, Uri, HeaderMap, and body stream in native forms.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

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
cargo test -p q-capabilities
```
```bash
cargo test -p velqu-runtime
```

## Required evidence for this microtask

- Admission counters.
- Negative body/header budget tests.
- Perf stage timings.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m24-002-a: keep method uri headermap and body stream in native forms
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record

- Status: **PASS**
- Deliverable: native ingress seam. `q-http` now enforces URI/header admission limits directly on the native head and hands the handler a `NativeRequest` (hyper `Method`, `Uri`, `HeaderMap`, `Incoming` body, request ID, started). No query parse, header clone, or body poll happens in `q-http` anymore; materialization is relocated into the runtime handler and is behavior-equivalent. Body reads go through `collect_body_bounded`, which stops at the byte budget instead of buffering the whole stream before the 413 check.
- Changed files:
  - `crates/q-http/src/lib.rs` (NativeRequest, native-only admit, collect_body_bounded, materialize_headers, RequestContext removed)
  - `crates/q-runtime/src/serve.rs` (private pipeline-local RequestContext built inside the handler; ServeState.limits)
  - `crates/q-runtime/src/main.rs` (passes limits into ServeState)
  - `docs/codex-spark-beta/tasks/01_m24_zero_copy_ingress/M24-002-A-keep-method-uri-headermap-and-body-stream-in-native-forms.md`
  - `docs/codex-spark-beta/STATUS.md`
  - `docs/codex-spark-beta/indexes/TASK_INDEX.md`
  - `docs/codex-spark-beta/indexes/EXECUTION_QUEUE.md`
  - `docs/codex-spark-beta/indexes/NEXT_25.md`
- Tests: new `header_materialization_lowercases_names_and_keeps_values` (q-http); behavior equivalence proven by unchanged suites — `body_and_header_limits_reject_oversize`, `queue_limit_returns_503_when_saturated`, `full_runtime_conformance` and the rest of runtime conformance (12/12), engine 84/84, bridge 4/4, `bun test` 35/35.
- Verification: `cargo test -p q-engine-quickjs -p q-http -p q-bridge -p q-capabilities -p velqu-runtime` all pass; `cargo fmt --check` clean; `cargo clippy --workspace --all-targets -- -D warnings` clean; `bun run typecheck` clean; `bun test` 35 pass; `./scripts/validate-okf` PASS.
- Remaining risk / deferred by design: routing-before-materialization (404/405/C0 zero materialization) lands in M24-002-B; declaration-driven FieldNeeds laziness in M24-002-C; request-object bypass in M24-002-D; admission counters and stage histograms are M24-009 scope per spec §9.2 (existing completion logs already carry per-request stage + durationMs). Parent guardrail "C0/C1 perform no query parse/header clone/body collect" is therefore not yet fully satisfied — it is delivered across B/C/D, not by this packet alone.
- Next dependency-ready task: M24-002-B (match RouteId using method/path before creating request metadata).
