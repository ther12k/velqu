---
task_id: M24-010-Z
parent_task: M24-010
milestone: M24
priority: P0
mode: EVIDENCE
status: TODO
context_card: context/milestones/M24.md
commit_required: true
---

# M24-010-Z — Package evidence for Complete ingress bridge fuzzing and conformance

## Atomic goal

Create source-backed evidence and handoff for parent task M24-010; update status only if verification passed.

## Parent intent

Prove lazy materialization is semantically safe under malformed and adversarial inputs.

## Dependencies

- `M24-010-V` — `tasks/01_m24_zero_copy_ingress/M24-010-V-verify-complete-ingress-bridge-fuzzing-and-conformance.md`

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
- `crates/q-router/src/lib.rs`
- `crates/q-pack/src/lib.rs`
- `packages/compiler/src/emit.ts`
- `conformance/routing/routing.conformance.test.ts`
- `crates/q-pack/tests/fuzz_pack.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `docs/specs/pack-format-v1.md`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

## Parent acceptance guardrails

- No panic, leak, stale-handle access, or unbounded allocation.
- Queue-empty-or-quarantined remains true.
- All failing cases become regression fixtures.
- Fuzz corpus is committed or reproducibly generated.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
cargo test -p q-router
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

## Required evidence for this microtask

- Fuzz summaries.
- Regression corpus.
- Sanitizer-compatible test output.
- [ ] Request routing precedes decode/materialization.
- [ ] Unread fields are not materialized.
- [ ] Worker-local slab has no global mutex and is generation-safe.
- [ ] Bodies, queues, parsers, and disconnect handling are bounded.
- [ ] Measured fixed overhead improves without semantic regression.
- C0/C1 fixed-overhead comparison versus G0 baseline.
- C3 parameter allocation/latency.
- Header/query/body allocation matrix.
- Concurrency 1/10/50 with stage timings.
- No schema-specialized JSON codecs beyond compatibility hooks.
- No QPack binary format change.
- No multi-worker dispatch.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m24-010-z: package evidence for complete ingress bridge fuzzing and con
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
