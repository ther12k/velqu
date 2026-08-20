---
task_id: BETA-016-V
parent_task: BETA-016
milestone: BETA
priority: P1
mode: VERIFY
status: TODO
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-016-V — Verify Run external clean-install and tutorial verification

## Atomic goal

Prove every acceptance criterion for parent task BETA-016 without broadening scope.

## Parent intent

Confirm a user outside the repository can complete the intended beta journey.

## Dependencies

- `BETA-016-A` — `tasks/08_public_beta/BETA-016-A-fresh-linux-vm-container.md`
- `BETA-016-B` — `tasks/08_public_beta/BETA-016-B-install-cli-runtime.md`
- `BETA-016-C` — `tasks/08_public_beta/BETA-016-C-scaffold-app.md`
- `BETA-016-D` — `tasks/08_public_beta/BETA-016-D-run-tests-dev-build.md`
- `BETA-016-E` — `tasks/08_public_beta/BETA-016-E-deploy-proof-service.md`
- `BETA-016-F` — `tasks/08_public_beta/BETA-016-F-use-treaty-client.md`

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
- `packages/cli/src/index.ts`
- `crates/q-runtime/src/source_map.rs`
- `examples/proof/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- No local unpublished dependency.
- Tutorial succeeds verbatim.
- Failures produce actionable diagnostics.
- Artifacts can be rolled back/uninstalled.

## Targeted commands

```bash
cargo test -p q-pack
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

- External transcript.
- Environment manifest.
- Issues and resolutions.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
beta-016-v: verify run external clean install and tutorial verification
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
