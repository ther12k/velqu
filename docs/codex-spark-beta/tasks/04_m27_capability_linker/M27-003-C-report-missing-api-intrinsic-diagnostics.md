---
task_id: M27-003-C
parent_task: M27-003
milestone: M27
priority: P1
mode: IMPLEMENT
status: TODO
context_card: context/milestones/M27.md
commit_required: true
---

# M27-003-C — Report missing API/intrinsic diagnostics

## Atomic goal

Report missing API/intrinsic diagnostics.

## Parent intent

Measure minimal/web/full contexts and select only meaningful reductions.

## Dependencies

- `M27-003-B` — `tasks/04_m27_capability_linker/M27-003-B-compile-application-requirements.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M27.md`
- `context/components/engine-scheduler.md`
- `context/components/capabilities-fetch.md`
- `context/components/devex-beta.md`

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
- `packages/cli/src/index.ts`
- `crates/q-runtime/src/source_map.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Report missing API/intrinsic diagnostics.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

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

## Required evidence for this microtask

- Context benchmark.
- Test262 subset.
- Compatibility report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m27-003-c: report missing api intrinsic diagnostics
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
