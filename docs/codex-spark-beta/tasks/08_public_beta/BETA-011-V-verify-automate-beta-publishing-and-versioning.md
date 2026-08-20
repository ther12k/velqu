---
task_id: BETA-011-V
parent_task: BETA-011
milestone: BETA
priority: P1
mode: VERIFY
status: TODO
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-011-V — Verify Automate beta publishing and versioning

## Atomic goal

Prove every acceptance criterion for parent task BETA-011 without broadening scope.

## Parent intent

Produce repeatable pre-release packages without implying API stability.

## Dependencies

- `BETA-011-A` — `tasks/08_public_beta/BETA-011-A-use-semver-prerelease.md`
- `BETA-011-B` — `tasks/08_public_beta/BETA-011-B-publish-next-beta-tag.md`
- `BETA-011-C` — `tasks/08_public_beta/BETA-011-C-generate-changelog-and-migration-notes.md`
- `BETA-011-D` — `tasks/08_public_beta/BETA-011-D-create-github-style-release-packet.md`
- `BETA-011-E` — `tasks/08_public_beta/BETA-011-E-support-yanking-rollback.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/BETA.md`
- `context/components/qpack-router.md`
- `context/components/devex-beta.md`

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
- `scripts/package`
- `scripts/release-packet`
- `packages/cli/package.json`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- Version is consistent across packages/binary/QPack.
- Re-running release does not mutate existing version.
- Rollback procedure is tested.
- Breaking beta changes require notes.

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

- Dry-run publish.
- Release workflow logs.
- Rollback rehearsal.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
beta-011-v: verify automate beta publishing and versioning
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
