---
task_id: M24-006-A
parent_task: M24-006
milestone: M24
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M24.md
commit_required: true
---

# M24-006-A — Compile query/cookie field IDs

## Atomic goal

Compile query/cookie field IDs.

## Parent intent

Parse query and cookies only when declared and only to the depth needed.

## Dependencies

- `M24-003-Z` — `tasks/01_m24_zero_copy_ingress/M24-003-Z-package-evidence-for-implement-worker-local-generation-checked-request-slab.md`
- `M24-004-Z` — `tasks/01_m24_zero_copy_ingress/M24-004-Z-package-evidence-for-capture-path-parameters-as-byte-ranges.md`

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
5. Implement exactly this deliverable: Compile query/cookie field IDs.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- No query parse on routes without query.
- Repeated and missing values follow schema semantics.
- Cookie parsing is bounded.
- Access remains valid through owner-scoped microtasks.

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

- Query/cookie conformance.
- Fuzz parser tests.
- Microtask lifetime tests.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Evidence

- `RoutePlanDecl.query_name_ids` and `QPack.query_name_table` compile declared query object properties into sorted dense IDs.
- `RoutePlanDecl.cookie_name_ids` and `QPack.cookie_name_table` are explicit empty bounded surfaces until cookie authoring is introduced; non-empty cookie metadata fails closed.
- `QPack::verify` derives query IDs from schema properties and rejects table/ID tampering.
- `query_name_ids_are_canonical_and_cookie_table_is_bounded` proves sorted IDs, canonical table, and tamper rejection.
- `crates/q-runtime/tests/runtime_conformance.rs::finalize_numeric` derives query IDs for runtime fixtures.
- `cargo test -p q-pack`: PASS (37 tests).
- `cargo test -p q-engine-quickjs --test engine`: PASS.
- `cargo test -p q-http`: PASS.
- `cargo test -p q-bridge`: PASS.
- `cargo test -p velqu-runtime --test runtime_conformance`: PASS (13 tests).
- `cargo fmt --check`: PASS.
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS.
- `bun run typecheck`: PASS.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m24-006-a: compile query cookie field ids
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
