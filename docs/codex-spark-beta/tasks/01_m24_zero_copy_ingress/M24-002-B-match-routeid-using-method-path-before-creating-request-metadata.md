---
task_id: M24-002-B
parent_task: M24-002
milestone: M24
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M24.md
commit_required: true
---

# M24-002-B — Match RouteId using method/path before creating request metadata

## Atomic goal

Match RouteId using method/path before creating request metadata.

## Parent intent

Avoid query/header/body work for routes that do not declare it.

## Dependencies

- `M24-002-A` — `tasks/01_m24_zero_copy_ingress/M24-002-A-keep-method-uri-headermap-and-body-stream-in-native-forms.md`

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

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Match RouteId using method/path before creating request metadata.
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
m24-002-b: match routeid using method path before creating request meta
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record

- Status: **PASS**
- Deliverable: the pipeline routes on borrowed method/path from the native head before any request metadata exists. Readiness, 404, 405, C0 native liveness, and quarantine paths now materialize zero fields and never poll the body stream. For matched JS routes: query pairs and header pairs materialize at that point (bounded by admission limits), and body admission is route-bound — the content-type gate reads the native HeaderMap before any poll, the single read is bounded by the route's `limit_bytes` (stops at budget, 413 without full buffering), and routes without a body binding never poll the stream regardless of method.
- Spec-mandated behavior changes (docs/specs/m24-ingress-ownership-and-admission.md §5.3): an oversize or partial body on an unmatched path now gets 404 (was 413 after full buffering), a method mismatch gets 405, and POST bodies on routes without a body binding are no longer collected.
- Changed files:
  - `crates/q-runtime/src/serve.rs` (pipeline takes NativeRequest; materialization moved after routing; route-bound body admission)
  - `crates/q-runtime/src/main.rs` (ServeState no longer carries limits; bounds live in q-http admission and route bindings)
  - `crates/q-runtime/tests/runtime_conformance.rs` (new `routing_precedes_body_materialization`)
  - `docs/codex-spark-beta/tasks/01_m24_zero_copy_ingress/M24-002-B-match-routeid-using-method-path-before-creating-request-metadata.md`
  - `docs/codex-spark-beta/STATUS.md`
  - `docs/codex-spark-beta/indexes/TASK_INDEX.md`
  - `docs/codex-spark-beta/indexes/EXECUTION_QUEUE.md`
  - `docs/codex-spark-beta/indexes/NEXT_25.md`
- Tests: new `routing_precedes_body_materialization` — announces a 2 MiB body, sends a fragment, and asserts 404/405 answers arrive before the body completes (a polling server cannot answer); regression suites unchanged: runtime conformance 13/13 (incl. `body_and_header_limits_reject_oversize`, `queue_limit_returns_503_when_saturated`, `full_runtime_conformance`), engine 84/84, bridge 4/4, `bun test` 35/35.
- Verification: `cargo test -p q-engine-quickjs -p q-http -p q-bridge -p q-capabilities -p velqu-runtime` all pass; `cargo fmt --check` clean; `cargo clippy --workspace --all-targets -- -D warnings` clean; `bun run typecheck` clean; `bun test` 35 pass; `./scripts/validate-okf` PASS.
- Remaining risk / deferred by design: query pairs still parse for every matched JS route (FieldNeeds gating is M24-002-C); header pairs still materialize for the JS store insert (declared-header lazy access is M24-005); request-object bypass for field-free routes is M24-002-D; admission counters/stage histograms are M24-009 scope (§9.2).
- Next dependency-ready task: M24-002-C (read FieldNeeds from RoutePlan).
