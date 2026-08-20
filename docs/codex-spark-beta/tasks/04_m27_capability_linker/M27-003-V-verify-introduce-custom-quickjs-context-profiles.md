---
task_id: M27-003-V
parent_task: M27-003
milestone: M27
priority: P1
mode: VERIFY
status: TODO
context_card: context/milestones/M27.md
commit_required: true
---

# M27-003-V — Verify Introduce custom QuickJS context profiles

## Atomic goal

Prove every acceptance criterion for parent task M27-003 without broadening scope.

## Parent intent

Measure minimal/web/full contexts and select only meaningful reductions.

## Dependencies

- `M27-003-A` — `tasks/04_m27_capability_linker/M27-003-A-build-configurable-intrinsic-profiles.md`
- `M27-003-B` — `tasks/04_m27_capability_linker/M27-003-B-compile-application-requirements.md`
- `M27-003-C` — `tasks/04_m27_capability_linker/M27-003-C-report-missing-api-intrinsic-diagnostics.md`
- `M27-003-D` — `tasks/04_m27_capability_linker/M27-003-D-retain-full-profile-for-compatibility-testing.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M27.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/capabilities-fetch.md`

### Source files

- `AGENTS.md`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `packages/compiler/src/emit.ts`
- `crates/q-pack/src/lib.rs`
- `crates/q-pack/tests/fuzz_pack.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `docs/specs/pack-format-v1.md`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`
- `crates/q-runtime/src/main.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- Chosen profile has measurable startup/RSS benefit or feature is deferred.
- No silent missing intrinsic.
- Conformance passes for selected profile.
- Profile identity enters runtime fingerprint.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p q-capabilities
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

- Context benchmark.
- Test262 subset.
- Compatibility report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m27-003-v: verify introduce custom quickjs context profiles
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
