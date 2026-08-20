---
task_id: M24-008-Z
parent_task: M24-008
milestone: M24
priority: P1
mode: EVIDENCE
status: PASS
context_card: context/milestones/M24.md
commit_required: true
---

# M24-008-Z — Package evidence for Replace per-request JS closures with native-backed prototypes

## Atomic goal

Create source-backed evidence and handoff for parent task M24-008; update status only if verification passed.

## Parent intent

Keep context shapes stable and avoid constructing getter closures for every request.

## Dependencies

- `M24-008-V` — `tasks/01_m24_zero_copy_ingress/M24-008-V-verify-replace-per-request-js-closures-with-native-backed-prototypes.md`

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
- `crates/q-runtime/src/main.rs`
- `crates/q-engine-quickjs/src/lib.rs`
- `benchmarks/harness/`
- `benchmarks/manifest.json`
- `benchmarks/raw/`
- `docs/reports/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

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

M24-008 implementation and verification commits are present through parent PR #688 merge `ed4674085d3f910c24ef651c27d60ff534bf6c46`.

Evidence matrix:

- Shared prototypes: `shared_context_request_prototypes_are_reused`.
- Opaque route-plan references and no request-byte copy: `route_plan_references_do_not_copy_request_bytes`.
- Cached native capability graph: `native_capability_graph_is_cached_and_immutable`.
- Explicit Web fallback: `explicit_web_request_fallback_materializes_on_demand`.
- Bridge stale-handle, settlement, cancellation, timeout, quarantine, and shutdown suites remain green.
- Rust suites, typecheck, format, clippy, and `./scripts/verify` passed locally.
- Bun conformance result retained honestly: 28 pass, 8 fail from runtime timeouts and missing generated proof pack in fresh worktree.
- No heap/allocation profile or benchmark manifest rewrite; unsupported performance claims omitted.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m24-008-z: package evidence for replace per request js closures with na
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
