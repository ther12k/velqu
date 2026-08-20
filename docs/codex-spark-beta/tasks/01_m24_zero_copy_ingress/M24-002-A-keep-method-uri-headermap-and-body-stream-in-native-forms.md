---
task_id: M24-002-A
parent_task: M24-002
milestone: M24
priority: P0
mode: IMPLEMENT
status: TODO
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

## Verification blocker record

- Task ID: `M24-002-A`
- Blocking fact: The current `q-http` API invokes a `Fn(RequestContext)` handler after q-http has already cloned headers, parsed query, and collected method-selected bodies. Deferring body polling requires route-aware admission before constructing `RequestContext`; the runtime currently owns routing in `crates/q-runtime/src/serve.rs`, so a local q-http type change either breaks the handler boundary or preserves the forbidden eager path.
- Exact source locations: `crates/q-http/src/lib.rs:205-337` (permit, cloned headers, eager query/body materialization); `crates/q-runtime/src/serve.rs:121-337` (routing and body/schema decisions after `RequestContext` exists); `crates/q-runtime/src/main.rs:234-235` (handler passed to q-http).
- Exact command/result: `cargo check -p q-http -p velqu-runtime` PASS after restoring baseline. The attempted native-head-only refactor failed because `velqu-runtime` still requires `Fn(RequestContext)` and route-aware body admission is not available at the q-http boundary.
- Dependency or owner required: Introduce the route-aware admission boundary first, then move body ownership/read-once behavior into M24-002/M24-007 without retaining an eager compatibility collect. Keep this packet TODO until admission counters, negative body/header budget tests, and perf stage timings exist.
- Safe work completed before stopping: inspected q-http/runtime integration and confirmed the smallest local API change cannot satisfy C0/C1 guardrails without coordinated route-boundary work; no source behavior changed.
- Files changed but not committed: this packet record only.
- Suggested next action: implement the route-aware native admission seam in the next authorized M24 implementation packet; do not mark M24-002-A, M24-001-V, M24-001-Z, or M24-GATE PASS.
