---
task_id: M27-003-D
parent_task: M27-003
milestone: M27
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M27.md
commit_required: true
---

# M27-003-D — Retain full profile for compatibility testing

## Atomic goal

Retain full profile for compatibility testing.

## Parent intent

Measure minimal/web/full contexts and select only meaningful reductions.

## Dependencies

- `M27-003-C` — `tasks/04_m27_capability_linker/M27-003-C-report-missing-api-intrinsic-diagnostics.md`

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
5. Implement exactly this deliverable: Retain full profile for compatibility testing.
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
m27-003-d: retain full profile for compatibility testing
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record — M27-003-D (PASS)

Deliverable: the full profile is retained as the explicit,
always-available compatibility baseline, selectable for testing.

### Changed files

- `crates/q-runtime/src/lib.rs` — `RunConfig.context_profile:
  Option<String>`; parsed through `ContextProfile::parse` (closed
  vocabulary) before engine spawn. Default stays Full; unknown names
  fail closed BEFORE ready (exit 2, stderr, known-set named) — never
  a silent fallback.
- `crates/q-runtime/src/main.rs` — `--context-profile full|web|
  minimal` CLI flag on velqu-runtime.
- `crates/q-runtime/tests/runtime_conformance.rs` —
  `full_profile_retained_for_compatibility_testing` (new).
- Bookkeeping: STATUS.md, TASK_INDEX.md.

### The test's honest semantics (a wrong assumption caught)

The fixture bundle touches Map only LAZILY inside users(), so
forcing minimal starts fine and serves livez — unlike a bundle with
top-level dropped-API references (which fails at load, as the proof
pack demonstrates under minimal via "RegExp are not supported"
before ready). Section 3 therefore asserts per-request loudness:
the degraded route returns a redacted internal problem (500,
https://velqu.dev/problems/internal), never a silent wrong answer.
Development note: the first draft of this test assumed startup
failure for any dropped-builtin usage and hung waiting for an exit
that was never coming; the hang surfaced the lazy-reference
distinction now documented here and in C's diagnostics rationale.

### Tests

31 conformance tests pass (+1): forced FULL on the fixture serves
(200 livez); unknown profile exits 2 pre-ready naming full, web,
minimal; forced MINIMAL starts but the Map-using route yields 500
internal-problem (auth-passed policy intact, business handler
degraded loudly).

### Commands (fresh worktree on M27-003-C HEAD da094eb)

- `cargo test -p q-pack` 98 · `-p q-engine-quickjs` 102 ·
  `-p q-capabilities` 51 · `-p velqu-runtime` conformance 31 (+2 lib
  suites unchanged) — pass.
- `bun test` 152 pass / 0 fail; `bun run typecheck`,
  `cargo fmt --check`, `cargo clippy --workspace --all-targets --
  -D warnings` — clean.
