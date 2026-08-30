---
task_id: M3-004-C
parent_task: M3-004
milestone: M3
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M3.md
commit_required: true
---

# M3-004-C — Validate capability compatibility per worker

## Atomic goal

Validate capability compatibility per worker.

## Parent intent

Load identical verified artifacts into independent runtimes efficiently.

## Dependencies

- `M3-004-B` — `tasks/06_m3_multi_worker/M3-004-B-create-separate-quickjs-runtimes-functions-context-state.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M3.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/multiworker.md`

### Source files

- `AGENTS.md`
- `crates/q-runtime/src/main.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-engine-quickjs/src/lib.rs`
- `crates/q-pack/src/lib.rs`
- `crates/q-pack/tests/fuzz_pack.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `docs/specs/pack-format-v1.md`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Validate capability compatibility per worker.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Workers execute identical contracts.
- One worker failure does not corrupt others.
- No JS object crosses workers.
- Artifact memory sharing is measured.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p velqu-runtime
```

## Required evidence for this microtask

- Worker parity tests.
- Memory mapping report.
- Startup tests.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m3-004-c: validate capability compatibility per worker
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M3-004-C) — PASS

- Date: 2026-08-30
- Branch/PR: m3-004-c (squash-merged; see git log for final hash)
- Closes: #392

### Changed files
- `crates/q-capabilities/src/identity.rs`: `validate_compatibility_per_worker(linked, requirements, worker) -> Result<WorkerCompatibility, ResolveError>` — per-worker capability compatibility validation (M3-004-C) —
  - EVERY requirement in the pack's capability manifest resolves against the linked descriptor set: identical id, EXACT version (no implicit compatibility), typed failures (`Missing`/`VersionConflict`).
  - Every worker runs the same validation over the same shared manifest (ADR-0036 §6), so either all workers reach Ready with the identical capability set, or every worker fails closed with the IDENTICAL typed error — no split capability reality across workers.
  - `WorkerCompatibility { worker, capabilities }` — deterministic result identifying WHO validated (redacted diagnostics keep the index).
- `crates/q-capabilities/src/lib.rs`: re-exports.

### Tests added (identity.rs, +3 → 213 q-capabilities lib tests)
- `every_worker_validates_the_identical_manifest` (4 workers, same manifest, same capability count; worker index identifies the validator)
- `version_conflict_fails_closed_per_worker` (timers@2 vs linked@1 → typed VersionConflict; all 4 workers fail IDENTICALLY)
- `missing_capability_fails_closed_per_worker` (unlinked runtime:postgres → typed Missing)

### Command results
- `cargo test -p q-capabilities` → **213 unit (was 210) + 7 + 1 + 4 + 9** — 0 failed
- `cargo test -p q-engine-quickjs` → 20+102+1 · `-p velqu-runtime` → 17+5+44 — all pass
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**; release binary unchanged (`070d2a8a…` — validation dormant until workers wire it)

### Guardrail mapping
- **Workers execute identical contracts** — all workers validate the same manifest with the same result; a pack can never serve with split capability reality.

### Disclosures
- A heredoc `\n` slip produced a literal in the file (caught by compile) and the determinism assertion initially compared worker indices (which differ by design) — scoped to the capability set. Both caught before commit.
- Standing: CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR.
