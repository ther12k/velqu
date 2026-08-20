---
task_id: M27-003-Z
parent_task: M27-003
milestone: M27
priority: P1
mode: EVIDENCE
status: TODO
context_card: context/milestones/M27.md
commit_required: true
---

# M27-003-Z — Package evidence for Introduce custom QuickJS context profiles

## Atomic goal

Create source-backed evidence and handoff for parent task M27-003; update status only if verification passed.

## Parent intent

Measure minimal/web/full contexts and select only meaningful reductions.

## Dependencies

- `M27-003-V` — `tasks/04_m27_capability_linker/M27-003-V-verify-introduce-custom-quickjs-context-profiles.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M27.md`
- `context/components/engine-scheduler.md`
- `context/components/capabilities-fetch.md`
- `context/components/evidence.md`

### Source files

- `AGENTS.md`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `packages/compiler/src/emit.ts`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`
- `crates/q-runtime/src/main.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-engine-quickjs/src/lib.rs`
- `benchmarks/harness/`
- `benchmarks/manifest.json`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

## Parent acceptance guardrails

- Chosen profile has measurable startup/RSS benefit or feature is deferred.
- No silent missing intrinsic.
- Conformance passes for selected profile.
- Profile identity enters runtime fingerprint.

## Targeted commands

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

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m27-003-z: package evidence for introduce custom quickjs context profil
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
