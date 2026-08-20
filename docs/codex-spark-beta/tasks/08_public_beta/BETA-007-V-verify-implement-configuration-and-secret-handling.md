---
task_id: BETA-007-V
parent_task: BETA-007
milestone: BETA
priority: P0
mode: VERIFY
status: TODO
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-007-V — Verify Implement configuration and secret handling

## Atomic goal

Prove every acceptance criterion for parent task BETA-007 without broadening scope.

## Parent intent

Provide typed configuration without top-level network I/O or accidental secret disclosure.

## Dependencies

- `BETA-007-A` — `tasks/08_public_beta/BETA-007-A-environment-file-configuration.md`
- `BETA-007-B` — `tasks/08_public_beta/BETA-007-B-validation-at-startup.md`
- `BETA-007-C` — `tasks/08_public_beta/BETA-007-C-secret-value-wrapper-redaction.md`
- `BETA-007-D` — `tasks/08_public_beta/BETA-007-D-profile-specific-settings.md`
- `BETA-007-E` — `tasks/08_public_beta/BETA-007-E-no-dynamic-code-execution.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/BETA.md`
- `context/components/devex-beta.md`

### Source files

- `AGENTS.md`
- `docs/beta/`
- `scripts/verify`
- `package.json`
- `.github/workflows/verify.yml`
- `packages/cli/src/index.ts`
- `crates/q-runtime/src/source_map.rs`
- `examples/proof/`
- `README.md`
- `crates/q-runtime/src/main.rs`
- `packages/core/src/index.ts`
- `scripts/package`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- Invalid config fails before ready.
- Secrets never appear in inspect/log/error.
- Defaults are safe.
- Configuration is documented/versioned.

## Targeted commands

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

- Config tests.
- Redaction tests.
- Examples.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
beta-007-v: verify implement configuration and secret handling
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
