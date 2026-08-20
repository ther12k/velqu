---
task_id: M24-002-D
parent_task: M24-002
milestone: M24
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M24.md
commit_required: true
---

# M24-002-D — Bypass request-object creation for policy-free routes that need no request fields

## Atomic goal

Bypass request-object creation for policy-free routes that need no request fields.

## Parent intent

Avoid query/header/body work for routes that do not declare it.

## Dependencies

- `M24-002-C` — `tasks/01_m24_zero_copy_ingress/M24-002-C-read-fieldneeds-from-routeplan.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M24.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`

### Source files

- `AGENTS.md`
- `crates/q-http/src/lib.rs`
- `crates/q-bridge/src/lib.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-runtime/tests/runtime_conformance.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Bypass request-object creation for policy-free routes that need no request fields.
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
m24-002-d: bypass request object creation for policy free routes that n
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record

- Status: **PASS**
- Deliverable: policy-free routes whose verified RoutePlan `FieldNeeds` are all false now bypass `RequestStore::insert` and the JavaScript request-field surface entirely. The runtime publishes `q_engine::NO_REQUEST_SLOT`; the QuickJS prelude maps it to `slot === -1` and defines no `params`, `query`, `headers`, `json`, `text`, or `bytes` request accessors. All native bridge accessors reject the sentinel fail-closed. Policy-bearing routes always retain a store slot even when their own field needs are empty.
- Changed files:
  - `crates/q-engine/src/lib.rs` (`NO_REQUEST_SLOT`)
  - `crates/q-runtime/src/serve.rs` (requestless predicate and conditional store insertion)
  - `crates/q-engine-quickjs/src/prelude.rs` (requestless context shape)
  - `crates/q-engine-quickjs/src/worker.rs` (sentinel conversion and native fail-closed guards)
  - `crates/q-engine-quickjs/tests/engine.rs` (requestless handler and counter proof)
  - `docs/codex-spark-beta/tasks/01_m24_zero_copy_ingress/M24-002-D-bypass-request-object-creation-for-policy-free-routes-that-need-no-request-field.md`
  - `docs/codex-spark-beta/STATUS.md`
  - `docs/codex-spark-beta/indexes/TASK_INDEX.md`
  - `docs/codex-spark-beta/indexes/EXECUTION_QUEUE.md`
  - `docs/codex-spark-beta/indexes/NEXT_25.md`
- Tests: `field_free_invocation_skips_request_store_slot` asserts the requestless handler sees no request properties and `BridgeCounters` report `live_slots=0`, `host_calls=0`, `materialized_fields=0`, `materialized_bytes=0`; runtime conformance 13/13 covers negative 404/405/413/431 and queue-limit paths; engine 85/85, bridge 4/4, q-http 2+3, Bun 35/35.
- Verification: `cargo test -p q-engine-quickjs -p q-http -p q-bridge -p velqu-runtime` PASS; `cargo fmt --check` clean; `cargo clippy --workspace --all-targets -- -D warnings` clean; `bun run typecheck` clean; `./scripts/validate-okf` PASS (174 links).
- Evidence boundary: bridge counters and negative body/header/route tests are source-backed in this packet. Full ingress admission counters and dedicated ingress stage histograms remain M24-009 deliverables; existing startup `stages`/`durationMs` logs are not claimed as an ingress performance benchmark.
- Next dependency-ready task: M24-002-V (verify route before request materialization).
