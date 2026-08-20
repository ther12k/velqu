---
task_id: M24-006-V
parent_task: M24-006
milestone: M24
priority: P1
mode: VERIFY
status: TODO
context_card: context/milestones/M24.md
commit_required: true
---

# M24-006-V — Verify Implement lazy query and cookie decoding

## Atomic goal

Prove every acceptance criterion for parent task M24-006 without broadening scope.

## Parent intent

Parse query and cookies only when declared and only to the depth needed.

## Dependencies

- `M24-006-A` — `tasks/01_m24_zero_copy_ingress/M24-006-A-compile-query-cookie-field-ids.md`
- `M24-006-B` — `tasks/01_m24_zero_copy_ingress/M24-006-B-provide-repeated-key-policy.md`
- `M24-006-C` — `tasks/01_m24_zero_copy_ingress/M24-006-C-define-percent-decoding-and-invalid-byte-behavior.md`
- `M24-006-D` — `tasks/01_m24_zero_copy_ingress/M24-006-D-cache-decoded-fields-per-request-slot.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M24.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`
- `context/components/schema-codecs.md`

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
- `crates/q-schema-runtime/src/lib.rs`
- `crates/q-schema-runtime/tests/fuzz_validator.rs`
- `crates/q-engine-quickjs/src/convert.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

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
cargo test -p q-schema-runtime
```
```bash
cargo test -p velqu-runtime
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

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m24-006-v: verify implement lazy query and cookie decoding
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
