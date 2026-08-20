---
task_id: M24-006-Z
parent_task: M24-006
milestone: M24
priority: P1
mode: EVIDENCE
status: TODO
context_card: context/milestones/M24.md
commit_required: true
---

# M24-006-Z — Package evidence for Implement lazy query and cookie decoding

## Atomic goal

Create source-backed evidence and handoff for parent task M24-006; update status only if verification passed.

## Parent intent

Parse query and cookies only when declared and only to the depth needed.

## Dependencies

- `M24-006-V` — `tasks/01_m24_zero_copy_ingress/M24-006-V-verify-implement-lazy-query-and-cookie-decoding.md`

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
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`
- `crates/q-runtime/tests/runtime_conformance.rs`
- `conformance/security/security.conformance.test.ts`
- `crates/q-pack/tests/fuzz_pack.rs`
- `crates/q-http/tests/fuzz_parsers.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

## Parent acceptance guardrails

- No query parse on routes without query.
- Repeated and missing values follow schema semantics.
- Cookie parsing is bounded.
- Access remains valid through owner-scoped microtasks.

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

- Query/cookie conformance.
- Fuzz parser tests.
- Microtask lifetime tests.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m24-006-z: package evidence for implement lazy query and cookie decodin
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
