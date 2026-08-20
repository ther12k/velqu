---
task_id: M4A-005-Z
parent_task: M4A-005
milestone: M4A
priority: P1
mode: EVIDENCE
status: TODO
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-005-Z — Package evidence for Publish compact contract and SDK artifacts

## Atomic goal

Create source-backed evidence and handoff for parent task M4A-005; update status only if verification passed.

## Parent intent

Support separate frontend repositories without importing server implementation.

## Dependencies

- `M4A-005-V` — `tasks/07_m4a_developer_preview/M4A-005-V-verify-publish-compact-contract-and-sdk-artifacts.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M4A.md`
- `context/components/qpack-router.md`
- `context/components/devex-beta.md`
- `context/components/evidence.md`

### Source files

- `AGENTS.md`
- `packages/cli/src/index.ts`
- `packages/compiler/src/index.ts`
- `packages/treaty/src/index.ts`
- `packages/testing/src/index.ts`
- `crates/q-pack/src/lib.rs`
- `crates/q-pack/tests/fuzz_pack.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `docs/specs/pack-format-v1.md`
- `packages/contract/src/index.ts`
- `conformance/treaty/treaty.conformance.test.ts`
- `packages/compiler/src/emit.ts`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

## Parent acceptance guardrails

- Client package contains no server runtime.
- Types remain responsive at large route counts.
- Version mismatch is diagnosable.
- Published artifact is deterministic.

## Targeted commands

```bash
cargo test -p q-pack
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

- Package content test.
- Type-scale report.
- Reproducibility check.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m4a-005-z: package evidence for publish compact contract and sdk artifa
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
