---
task_id: M24-008-C
parent_task: M24-008
milestone: M24
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M24.md
commit_required: true
---

# M24-008-C — Cache native capability objects

## Atomic goal

Cache native capability objects.

## Parent intent

Keep context shapes stable and avoid constructing getter closures for every request.

## Dependencies

- `M24-008-B` — `tasks/01_m24_zero_copy_ingress/M24-008-B-store-only-opaque-handle-and-route-plan-references-per-request.md`

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
- `crates/q-runtime/tests/runtime_conformance.rs`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-pack/src/lib.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Cache native capability objects.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Stable hidden-class/object shape.
- No per-field closure allocation on normal routes.
- Fallback semantics documented.
- Stale handle checks remain enforced.

## Targeted commands

```bash
cargo test -p q-pack
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
cargo test -p q-capabilities
```
```bash
cargo test -p velqu-runtime
```

## Required evidence for this microtask

- Heap/allocation profile.
- Bridge conformance.
- Fallback tests.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Evidence

- `__velquNativeCapabilities` creates one frozen worker-level capability graph; each request context assigns same graph to `ctx.native`.
- `native_capability_graph_is_cached_and_immutable` invokes two requests and proves shared identity plus frozen capability objects.
- Native timer authorization and operation ownership remain enforced by existing native bridge checks and settlement tests.
- `cargo test -p q-engine-quickjs`: PASS (95 tests).
- `cargo test -p q-pack`: PASS.
- `cargo test -p q-http`: PASS.
- `cargo test -p q-bridge`: PASS.
- `cargo test -p q-capabilities`: PASS.
- `cargo test -p velqu-runtime`: PASS.
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS.
- `scripts/validate-okf`: PASS (174 links, 0 errors).
- No unsupported heap/allocation or benchmark claim added.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m24-008-c: cache native capability objects
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
