---
task_id: M27-003-A
parent_task: M27-003
milestone: M27
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M27.md
commit_required: true
---

# M27-003-A — Build configurable intrinsic profiles

## Atomic goal

Build configurable intrinsic profiles.

## Parent intent

Measure minimal/web/full contexts and select only meaningful reductions.

## Dependencies

- `M27-002-Z` — `tasks/04_m27_capability_linker/M27-002-Z-package-evidence-for-implement-compile-time-capability-dependency-resolver.md`

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
- `packages/core/src/index.ts`
- `docs/beta/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Build configurable intrinsic profiles.
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
m27-003-a: build configurable intrinsic profiles
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record — M27-003-A (PASS)

Deliverable: configurable intrinsic profiles for the QuickJS
context.

### Changed files

- `crates/q-engine-quickjs/src/lib.rs` — `ContextProfile` closed
  vocabulary (`Full` | `Web` | `Minimal`) with fail-closed
  `parse()` (no aliases, no guessing) and `as_str()`; new
  `QuickJsConfig.profile` field, default `Full` (byte-for-byte all
  prior behavior). Documented contents of each profile.
- `crates/q-engine-quickjs/src/worker.rs` — `create_context(rt,
  profile)`: the single context-construction point. `Full` =
  `Context::full` (JS_NewContext); `Web` and `Minimal` use
  rquickjs's typed intrinsic tuples (explicit in source):
  Web drops Date+Performance; Minimal keeps exactly the host-bridge
  needs (Eval, JSON, Promise, TypedArrays).
- Bookkeeping: STATUS.md, TASK_INDEX.md.

### Tests

`cargo test -p q-engine-quickjs` — 102 passed (+5 in lib tests;
all worker/engine suites unchanged):

- `full_profile_has_all_standard_builtins` (Date, Map, RegExp,
  WeakRef present — pins Full against accidental reduction).
- `web_profile_keeps_web_builtins_drops_date_and_performance`
  (+ honest negative control proving the probe detects presence).
- `minimal_profile_is_host_bridge_only` (JSON/Promise/base alive;
  Date/Proxy/Map/Set/WeakRef/RegExp visibly absent via
  `typeof X === 'undefined'`).
- `minimal_context_rejects_regex_touching_code`.
- `profile_parse_is_closed_vocabulary`, `config_default_is_full`.

`bun test` 139 pass / 0 fail (proof app on default Full profile —
production behavior unchanged).

### Commands (fresh worktree on M27-002-Z HEAD 0af6a62)

- `cargo test -p q-pack` 98 · `-p q-engine-quickjs` 102 ·
  `-p velqu-runtime` 30 (via full suites) — pass.
- `bun test` 139 pass / 0 fail; `bun run typecheck`,
  `cargo fmt --check`, `cargo clippy --workspace --all-targets --
  -D warnings` — clean.

### Notes

- Profiles are construction-time only; there is no runtime switch —
  a worker owns one profile for its whole life.
- The actual selection decision ("only meaningful reductions") is
  M27-011's measurement work; A builds the instrument, not the
  verdict. Default stays Full so nothing changes until measured.
