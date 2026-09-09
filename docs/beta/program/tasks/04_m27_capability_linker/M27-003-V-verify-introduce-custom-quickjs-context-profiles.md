---
task_id: M27-003-V
parent_task: M27-003
milestone: M27
priority: P1
mode: VERIFY
status: PASS
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

## Verification record — M27-003-V (PASS)

Parent: M27-003 "Introduce custom QuickJS context profiles".
Implementation packets merged prior: A (PR #854, #252), B
(PR #855, #253), C (PR #856, #254), D (PR #857, #255).

### Guardrail map

1. **Chosen profile has measurable startup/RSS benefit or feature
   is deferred.** Measured (5 fresh-process samples/profile, same
   binary+pack): full p50 3.929 ms / 7,144 kB; web 3.879 ms /
   7,012 kB — inside noise; minimal rejects the proof app outright
   (top-level RegExp → fail-closed at load). No demonstrated
   benefit ⇒ **selection DEFERRED to M27-011**; production serving
   stays Full. Report: docs/reports/m27-003-context-profiles-compat.md.

2. **No silent missing intrinsic.** Positive/negative probes pin
   every profile's kept/dropped sets (`web_profile_keeps…`,
   `minimal_profile_is_host_bridge_only`, regex-eval failure probe);
   unknown `--context-profile` names exit 2 pre-ready; degraded
   routes under lazy references return redacted internal problems,
   never silent wrong answers
   (`full_profile_retained_for_compatibility_testing`).

3. **Conformance passes for selected profile.** The only profile
   selected for production is the default Full: full gates pass —
   q-pack 98, q-engine-quickjs 102, q-capabilities 51, conformance
   31, bun 152/0, typecheck/fmt/clippy clean,
   `./scripts/verify` ALL PASS (exit 0) after the matched manifest
   refresh below.

4. **Profile identity enters runtime fingerprint/identity block.**
   Found missing during this verification and fixed here as a parent-
   necessary defect fix: the ready line now carries
   `"contextProfile":"<name>"` in its startup identity block (pinned
   by an assertion in the compat test). RuntimeFingerprint (the pack
   compatibility tuple) intentionally unchanged — profile is a run-
   time selection, not a pack-compatibility dimension; documented in
   the report.

### Changed files (defect fix)

- `crates/q-runtime/src/lib.rs` — ready-line contextProfile.
- `crates/q-runtime/tests/runtime_conformance.rs` — assertion.
- `docs/reports/m27-003-context-profiles-compat.md` (new) —
  context benchmark + compatibility report (required evidence).
- `benchmarks/manifest.json` — matched refresh (qRuntimeRelease
  hash changed because the binary now self-describes its profile;
  zero raw-data changes).
- Bookkeeping: STATUS.md, TASK_INDEX.md.

### Commands and results (fresh worktree on parent HEAD 33b391b)

All targeted commands pass; first verify failed benchmark validation
exactly as it should (binary changed), matched refresh applied, then
ALL PASS exit 0.
