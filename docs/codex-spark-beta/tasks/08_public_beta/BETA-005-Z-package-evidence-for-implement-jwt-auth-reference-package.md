---
task_id: BETA-005-Z
parent_task: BETA-005
milestone: BETA
priority: P0
mode: EVIDENCE
status: TODO
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-005-Z — Package evidence for Implement JWT/auth reference package

## Atomic goal

Create source-backed evidence and handoff for parent task BETA-005; update status only if verification passed.

## Parent intent

Provide a secure documented policy example and typed errors.

## Dependencies

- `BETA-005-V` — `tasks/08_public_beta/BETA-005-V-verify-implement-jwt-auth-reference-package.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/BETA.md`
- `context/components/qpack-router.md`
- `context/components/devex-beta.md`
- `context/components/evidence.md`

### Source files

- `AGENTS.md`
- `docs/beta/`
- `scripts/verify`
- `package.json`
- `.github/workflows/verify.yml`
- `crates/q-pack/src/lib.rs`
- `crates/q-pack/tests/fuzz_pack.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `docs/specs/pack-format-v1.md`
- `packages/auth-jwt/ (create if absent)`
- `packages/core/src/index.ts`
- `packages/treaty/src/index.ts`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

## Parent acceptance guardrails

- Invalid tokens fail closed.
- Algorithm confusion is impossible.
- Auth policy error appears in Treaty contract.
- Performance/caching is documented.

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
```bash
./scripts/validate-okf
```

## Required evidence for this microtask

- Security tests.
- Reference docs.
- W1/W2/W3 integration.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
beta-005-z: package evidence for implement jwt auth reference package
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
