---
task_id: M24-007-A
parent_task: M24-007
milestone: M24
priority: P0
mode: IMPLEMENT
status: TODO
context_card: context/milestones/M24.md
commit_required: true
---

# M24-007-A — Drive body behavior from RoutePlan, not HTTP method

## Atomic goal

Drive body behavior from RoutePlan, not HTTP method.

## Parent intent

Collect or stream request bodies only when declared and under route/global limits.

## Dependencies

- `M24-001-Z` — `tasks/01_m24_zero_copy_ingress/M24-001-Z-package-evidence-for-freeze-ingress-ownership-and-backpressure-design.md`
- `M24-003-Z` — `tasks/01_m24_zero_copy_ingress/M24-003-Z-package-evidence-for-implement-worker-local-generation-checked-request-slab.md`

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
5. Implement exactly this deliverable: Drive body behavior from RoutePlan, not HTTP method.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

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
cargo test -p q-capabilities
```
```bash
cargo test -p velqu-runtime
```

## Required evidence for this microtask

- Body-limit tests.
- Slowloris/partial-body tests.
- Cancellation metrics.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m24-007-a: drive body behavior from routeplan not http method
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
