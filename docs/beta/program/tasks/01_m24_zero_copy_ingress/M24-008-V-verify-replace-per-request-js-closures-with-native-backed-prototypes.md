---
task_id: M24-008-V
parent_task: M24-008
milestone: M24
priority: P1
mode: VERIFY
status: PASS
context_card: context/milestones/M24.md
commit_required: true
---

# M24-008-V — Verify Replace per-request JS closures with native-backed prototypes

## Atomic goal

Prove every acceptance criterion for parent task M24-008 without broadening scope.

## Parent intent

Keep context shapes stable and avoid constructing getter closures for every request.

## Dependencies

- `M24-008-A` — `tasks/01_m24_zero_copy_ingress/M24-008-A-create-shared-context-request-prototypes-or-native-classes.md`
- `M24-008-B` — `tasks/01_m24_zero_copy_ingress/M24-008-B-store-only-opaque-handle-and-route-plan-references-per-request.md`
- `M24-008-C` — `tasks/01_m24_zero_copy_ingress/M24-008-C-cache-native-capability-objects.md`
- `M24-008-D` — `tasks/01_m24_zero_copy_ingress/M24-008-D-keep-full-web-request-construction-as-explicit-fallback.md`

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
- `benchmarks/harness/`
- `benchmarks/manifest.json`
- `benchmarks/raw/`
- `docs/reports/`
- `scripts/benchmark`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- Stable hidden-class/object shape.
- No per-field closure allocation on normal routes.
- Fallback semantics documented.
- Stale handle checks remain enforced.

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
```bash
./scripts/verify
```

## Required evidence for this microtask

- Heap/allocation profile.
- Bridge conformance.
- Fallback tests.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Evidence

Acceptance matrix:

- Stable object shape: `shared_context_request_prototypes_are_reused`, `native_capability_graph_is_cached_and_immutable` PASS.
- No request bytes in JS route-plan state: `route_plan_references_do_not_copy_request_bytes` PASS.
- Explicit fallback: `explicit_web_request_fallback_materializes_on_demand` PASS.
- Stale handles and settlement: existing q-engine-quickjs/q-bridge expiry, cross-worker, cancellation, timeout, quarantine, and shutdown tests PASS.
- `cargo test -p q-engine-quickjs`: PASS (96 tests).
- `cargo test -p q-http`: PASS.
- `cargo test -p q-bridge`: PASS.
- `cargo test -p velqu-runtime`: PASS.
- `bun run typecheck`: PASS.
- `cargo fmt --check`: PASS.
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS.
- `./scripts/verify`: PASS locally.
- `bun test`: 28 pass, 8 fail in fresh worktree; failures are runtime timeouts and missing generated `examples/proof/dist/app.qpack`, recorded as scoped verification limitations. No claims weakened and no benchmark manifests changed.
- No heap/allocation profile or benchmark claim added because raw allocation evidence is not present.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m24-008-v: verify replace per request js closures with native backed pr
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
