---
task_id: M24-002-C
parent_task: M24-002
milestone: M24
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M24.md
commit_required: true
---

# M24-002-C — Read FieldNeeds from RoutePlan

## Atomic goal

Read FieldNeeds from RoutePlan.

## Parent intent

Avoid query/header/body work for routes that do not declare it.

## Dependencies

- `M24-002-B` — `tasks/01_m24_zero_copy_ingress/M24-002-B-match-routeid-using-method-path-before-creating-request-metadata.md`

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
5. Implement exactly this deliverable: Read FieldNeeds from RoutePlan.
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
m24-002-c: read fieldneeds from routeplan
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record

- Status: **PASS**
- Deliverable: the runtime reads `FieldNeeds` from the verified `RoutePlanDecl` (via `CompiledRoute::plan`) and gates every remaining materialization: query pairs parse only when `needs.query`, header pairs copy only when `needs.headers` (which QPack verification already proven covers declared header schemas AND policy/security headers), and content-type materializes only for body-bound routes. Undeclared fields now copy zero bytes even when the JS handler runs (spec §8.2 rule C3). Body admission remains gated on the route body binding (M24-002-B). `FieldNeeds` derives `Copy` (four bools).
- Changed files:
  - `crates/q-runtime/src/serve.rs` (FieldNeeds-gated query/header/content-type materialization)
  - `crates/q-pack/src/lib.rs` (FieldNeeds: Copy)
  - `docs/codex-spark-beta/tasks/01_m24_zero_copy_ingress/M24-002-C-read-fieldneeds-from-routeplan.md`
  - `docs/codex-spark-beta/STATUS.md`
  - `docs/codex-spark-beta/indexes/TASK_INDEX.md`
  - `docs/codex-spark-beta/indexes/EXECUTION_QUEUE.md`
  - `docs/codex-spark-beta/indexes/NEXT_25.md`
- Tests: existing suites prove both directions — the lifecycle/policy conformance test (security-driven `needs.headers`) still injects the auth header session, query-validated routes still enforce schemas, and field-free routes answer unchanged: runtime conformance 13/13, q-pack 34+2 (incl. `rejects_mismatched_field_needs`), engine 84/84, bridge 4/4, `bun test` 35/35.
- Verification: `cargo test -p q-engine-quickjs -p q-http -p q-bridge -p q-capabilities -p q-router -p q-pack -p velqu-runtime` all pass; `cargo fmt --check` clean; `cargo clippy --workspace --all-targets -- -D warnings` clean; `bun run typecheck` clean; `./scripts/validate-okf` PASS.
- Remaining risk / deferred by design: params still materialize as Strings inside the router (byte-range captures are M24-004); declared-header lazy access by compiled header ID is M24-005; field-free store-insert bypass is M24-002-D; counters/histograms are M24-009.
- Next dependency-ready task: M24-002-D (bypass request object creation for policy-free routes that need no request field).
