---
task_id: M3-001-C
parent_task: M3-001
milestone: M3
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M3.md
commit_required: true
---

# M3-001-C — Forbid JSValue sharing

## Atomic goal

Forbid JSValue sharing.

## Parent intent

Define what JavaScript and native state is per worker versus shared.

## Dependencies

- `M3-001-B` — `tasks/06_m3_multi_worker/M3-001-B-document-module-level-state-replication.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M3.md`
- `context/components/engine-scheduler.md`
- `context/components/multiworker.md`

### Source files

- `AGENTS.md`
- `crates/q-runtime/src/main.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-engine-quickjs/src/lib.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Forbid JSValue sharing.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Each runtime has one owner thread.
- Cross-worker mutable state is explicit.
- Initialization is deterministic.
- Developer docs describe per-worker globals.

## Targeted commands

```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p velqu-runtime
```

## Required evidence for this microtask

- ADR.
- Concurrency model tests plan.
- State examples.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m3-001-c: forbid jsvalue sharing
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M3-001-C) — PASS

- Date: 2026-08-30
- Branch/PR: m3-001-c (squash-merged; see git log for final hash)
- Closes: #374

### Changed files
- `crates/q-engine-quickjs/src/lib.rs`: type-level enforcement of ADR-0036 §5 (no JSValue crosses workers) —
  - Crate-level state-ownership docs with a **`compile_fail` doc test**: moving an `rquickjs::Value` into `std::thread::spawn` must not compile (the value holds `Rc` pointers into one runtime's heap — `!Send` by construction). This test runs in every `cargo test` and fails the build if a future rquickjs upgrade ever makes values cross-thread-movable.
  - `state_ownership_tests` module pinning the POSITIVE half of the contract: `WorkerMsg` (the only thing that crosses worker boundaries) is `Send + Sync` — plain data only, so no JS value can hide inside; `InvocationSpec`/`Outcome`/`EngineHealth`/`QuickJsEngine` are `Send` — the engine front door talks through channels, never runtime pointers.
- `benchmarks/manifest.json`: release binary hash refreshed (`333d563d…`).

### Tests added
- Crate doc `compile_fail` test (the prohibition itself, compiler-enforced).
- `worker_messages_are_plain_data_send_sync`
- `engine_boundary_types_are_send_sync`

### Command results
- `cargo test -p q-engine-quickjs` → **20 lib (was 18) + 101 engine + 1 doctest (compile_fail verified)** — 0 failed
- `cargo test -p velqu-runtime` → 12+5+44 — all pass
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**

### Guardrail mapping
- **Each runtime has one owner thread** — the type system makes cross-thread JS values a compile error, not a convention.

### Disclosures
- The release binary hash legitimately refreshed: crate-level doc lines land in the panic/metadata sections of the artifact.
- Standing: CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR.
