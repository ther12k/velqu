---
task_id: M24-004-V
parent_task: M24-004
milestone: M24
priority: P1
mode: VERIFY
status: TODO
context_card: context/milestones/M24.md
commit_required: true
---

# M24-004-V — Verify Capture path parameters as byte ranges

## Atomic goal

Prove every acceptance criterion for parent task M24-004 without broadening scope.

## Parent intent

Avoid allocating parameter strings until validation or JavaScript access requires them.

## Dependencies

- `M24-004-A` — `tasks/01_m24_zero_copy_ingress/M24-004-A-store-capture-start-end-ranges-against-the-uri-path.md`
- `M24-004-B` — `tasks/01_m24_zero_copy_ingress/M24-004-B-bind-route-specific-parameter-names-after-routeid-selection.md`
- `M24-004-C` — `tasks/01_m24_zero_copy_ingress/M24-004-C-validate-numeric-uuid-formats-directly-from-bytes-where-possible.md`
- `M24-004-D` — `tasks/01_m24_zero_copy_ingress/M24-004-D-materialize-js-strings-lazily.md`

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
- `crates/q-schema-runtime/src/lib.rs`
- `crates/q-schema-runtime/tests/fuzz_validator.rs`
- `crates/q-engine-quickjs/src/convert.rs`
- `packages/schema/src/index.ts`
- `conformance/schema/schema.conformance.test.ts`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- Parameterized routes preserve exact names and values.
- No owned parameter string on an unread path.
- Percent-decoding policy is explicit and tested.
- Invalid encodings fail consistently.

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

- Allocation test.
- Reference router parity.
- Encoding edge-case corpus.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m24-004-v: verify capture path parameters as byte ranges
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
