---
task_id: M25-010-Z
parent_task: M25-010
milestone: M25
priority: P1
mode: EVIDENCE
status: TODO
context_card: context/milestones/M25.md
commit_required: true
---

# M25-010-Z — Package evidence for Close codec performance and cold-start evidence

## Atomic goal

Create source-backed evidence and handoff for parent task M25-010; update status only if verification passed.

## Parent intent

Prove the selected strategies improve real payloads without inflating startup unacceptably.

## Dependencies

- `M25-010-V` — `tasks/02_m25_schema_codecs/M25-010-V-verify-close-codec-performance-and-cold-start-evidence.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M25.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`

### Source files

- `AGENTS.md`
- `crates/q-schema-runtime/src/lib.rs`
- `crates/q-engine-quickjs/src/convert.rs`
- `packages/schema/src/index.ts`
- `packages/compiler/src/emit.ts`
- `crates/q-pack/src/lib.rs`
- `crates/q-pack/tests/fuzz_pack.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `docs/specs/pack-format-v1.md`
- `crates/q-http/src/lib.rs`
- `crates/q-bridge/src/lib.rs`
- `crates/q-runtime/src/serve.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

## Parent acceptance guardrails

- C2 materially improves or limitation is documented.
- No unapproved cold-start regression.
- Reports match raw data.
- Route-specific strategy is inspectable.

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

## Required evidence for this microtask

- Raw performance suite.
- Generated report.
- Decision matrix.
- [ ] Canonical Schema IR drives runtime, Treaty, OpenAPI, lock, and diff.
- [ ] Generated decoders/encoders are semantically equivalent and bounded.
- [ ] Fallbacks are explicit and measured.
- [ ] Response errors/problems are exact and redacted correctly.
- [ ] Performance evidence supports route-level strategy selection.
- C2 small JSON.
- 1KB/16KB/64KB dynamic payloads.
- Arrays 100/1,000.
- Request decode and response encode stage timings.
- No binary QPack encoding yet.
- No capability API expansion.
- No ORM.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m25-010-z: package evidence for close codec performance and cold start
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
