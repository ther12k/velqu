---
task_id: M27-003-B
parent_task: M27-003
milestone: M27
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M27.md
commit_required: true
---

# M27-003-B — Compile application requirements

## Atomic goal

Compile application requirements.

## Parent intent

Measure minimal/web/full contexts and select only meaningful reductions.

## Dependencies

- `M27-003-A` — `tasks/04_m27_capability_linker/M27-003-A-build-configurable-intrinsic-profiles.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M27.md`
- `context/components/engine-scheduler.md`
- `context/components/capabilities-fetch.md`

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

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Compile application requirements.
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
m27-003-b: compile application requirements
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record — M27-003-B (PASS)

Deliverable: the compiler now compiles each application's intrinsic
requirement as deterministic diagnostic data.

### Changed files

- `packages/compiler/src/intrinsic-requirements.ts` (new) —
  `compileIntrinsicRequirement(bundle)`: word-boundary scan of the
  bundled output for builtins dropped by each profile boundary.
  Fail-safe direction: any hit forces a stronger profile
  (`full` > `web` > `minimal`). Documented limitation: regex
  *literals* are lexically invisible; misses fail loudly at runtime,
  never silently, and serving keeps its configured default until
  measured selection lands.
- `packages/compiler/src/index.ts` — requirement computed at build
  and emitted into `capability-manifest.json` +
  `build-report.json` as `intrinsicRequirement { requirement, used }`.
- `packages/cli/src/intrinsic-requirements.test.ts` (new, 7 tests).
- Bookkeeping: STATUS.md, TASK_INDEX.md.

### Design boundaries (deliberate)

- Diagnostic data only in B: the pack schema and the runtime's
  serving profile are untouched; full-profile compatibility is
  explicitly retained (M27-003-D's deliverable), and actual
  reduction selection requires measurement (M27-011 / parent intent).
- End-to-end on the proof app: requirement computes to `"web"`
  (bundle touches Map via bundled helpers); emitted in both artifact
  JSONs deterministically.

### Tests

7 TS tests: minimal-required clean bundle; each of Map/Set/Proxy/
WeakRef/RegExp constructor usage forcing web; Date/performance
forcing full; full-wins precedence; word-boundary non-over-matching
(`UpdateFoo` ≠ `Date`); documented regex-literal limitation;
determinism.

### Commands (fresh worktree on M27-003-A HEAD 65fefcf)

- `cargo test -p q-pack` 98 · `-p q-engine-quickjs` 102 ·
  `-p q-capabilities` 51 · `-p velqu-runtime` 30 — pass.
- `bun test` 146 pass / 0 fail; `bun run typecheck`,
  `cargo fmt --check`, `cargo clippy --workspace --all-targets --
  -D warnings` — clean.

### Notes

- Parent intent ("measure minimal/web/full") progresses A
  (instrument) → B (per-app requirement data) → C (diagnostics of
  what a reduction would break) → D (full-profile retention) with
  the measured selection verdict at M27-011.
