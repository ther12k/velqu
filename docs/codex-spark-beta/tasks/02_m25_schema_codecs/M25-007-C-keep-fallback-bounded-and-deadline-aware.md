---
task_id: M25-007-C
parent_task: M25-007
milestone: M25
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M25.md
commit_required: true
---

# M25-007-C — Keep fallback bounded and deadline-aware

## Atomic goal

Keep fallback bounded and deadline-aware.

## Parent intent

Support advanced cases without hiding performance or semantic costs.

## Dependencies

- `M25-007-B` — `tasks/02_m25_schema_codecs/M25-007-B-support-raw-response-full-request-escape-hatches.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M25.md`
- `context/components/engine-scheduler.md`
- `context/components/schema-codecs.md`
- `context/components/evidence.md`

### Source files

- `AGENTS.md`
- `crates/q-schema-runtime/src/lib.rs`
- `crates/q-engine-quickjs/src/convert.rs`
- `packages/schema/src/index.ts`
- `packages/compiler/src/emit.ts`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`
- `benchmarks/harness/`
- `benchmarks/manifest.json`
- `benchmarks/raw/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Keep fallback bounded and deadline-aware.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Fallback never activates silently.
- Raw Response bypass behavior is documented.
- No contract claim is generated when adapter lacks required projection.
- Fallback routes pass conformance.

## Targeted commands

```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p q-schema-runtime
```
```bash
bun test
```
```bash
bun run typecheck
```

## Required evidence for this microtask

- Inspect snapshots.
- Fallback integration tests.
- Performance delta report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m25-007-c: keep fallback bounded and deadline aware
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record (M25-007-C)

Status: **PASS**. Every fallback path is bounded and deadline-aware, with
live evidence:

- Bounds already in force (unchanged, now evidenced for the fallback
  paths): body admission by `limit_bytes` (413 before the engine),
  serde_json recursion cap, `MAX_VALIDATE_DEPTH` typed bound, transport
  header admission limits, JS heap limits, and the M25-004-D anchored
  request deadline that bounds admission + read + decode + handler.
- New evidence `runtime_conformance::fallback_paths_are_bounded_and_
  deadline_aware` — one pack, a busy (`while (true) {}`) handler served on
  three fallback routes plus a healthy probe:
  1. js-validation fallback route (raw-body crossing): busy handler
     settles 504 at the 200ms route deadline (< 2s wall clock).
  2. raw-response escape route: the deadline applies BEFORE any raw
     mapping — 504 at the deadline.
  3. full-request escape route: identical deadline ownership — 504.
  4. Interrupt deadline kills are NOT quarantine: the engine keeps
     serving (probe 200 immediately after the three kills).
  5. Fallback admission bound: a 70,000-byte body on the js-fallback
     route rejects 413 before the engine is ever entered (the busy
     handler never runs).

### Tests and evidence

- `runtime_conformance::fallback_paths_are_bounded_and_deadline_aware`
  (live HTTP; see the five points above).
- Existing bound evidence stays green: deeply-nested 422 on the
  js-fallback route (M25-004-C), body/header 413/431 admission tests,
  engine poison/quarantine tests, M25-004-D stalled-body deadline test.
- `cargo test -p q-engine-quickjs` — 1 + 96 passed.
- `cargo test -p q-schema-runtime` — 57 unit + 3 fuzz passed.
- `cargo test -p velqu-runtime` — 24 integration passed.
- `cargo test -p q-pack` — 41 + 2 passed.
- `bun test` — 74 passed, 0 failed, 319 expect calls.
- `bun run typecheck` — clean. `cargo fmt --check` — clean.
- `cargo clippy --workspace --all-targets -- -D warnings` — clean.
- `scripts/validate-okf` — 176 links, 0 errors.
- `./scripts/verify` — all stages pass except the documented
  isolated-worktree `qRuntimeRelease`/`proofPack` manifest hash mismatch
  (known, pre-existing on every packet branch).

Commit: `3787e75`.
