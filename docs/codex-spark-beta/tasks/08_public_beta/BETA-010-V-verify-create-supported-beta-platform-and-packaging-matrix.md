---
task_id: BETA-010-V
parent_task: BETA-010
milestone: BETA
priority: P1
mode: VERIFY
status: TODO
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-010-V — Verify Create supported beta platform and packaging matrix

## Atomic goal

Prove every acceptance criterion for parent task BETA-010 without broadening scope.

## Parent intent

Ship installable binaries/packages for an explicit narrow platform promise.

## Dependencies

- `BETA-010-A` — `tasks/08_public_beta/BETA-010-A-linux-x86-64-glibc-mandatory-working-assumption.md`
- `BETA-010-B` — `tasks/08_public_beta/BETA-010-B-linux-arm64-glibc-when-ci-is-available.md`
- `BETA-010-C` — `tasks/08_public_beta/BETA-010-C-npm-packages-under-beta-tag.md`
- `BETA-010-D` — `tasks/08_public_beta/BETA-010-D-runtime-binary-qpack-tools.md`
- `BETA-010-E` — `tasks/08_public_beta/BETA-010-E-clean-install-tests.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/BETA.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
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
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- Published platform list is exact.
- Unsupported platforms fail with guidance.
- Packages contain no accidental source/compiler artifacts.
- Install works in clean environment.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
cargo test -p q-engine-quickjs
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

- Platform CI.
- Package inventory.
- Install transcript.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
beta-010-v: verify create supported beta platform and packaging matrix
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
