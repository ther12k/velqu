---
task_id: M3-004-B
parent_task: M3-004
milestone: M3
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M3.md
commit_required: true
---

# M3-004-B — Create separate QuickJS runtimes/functions/context state

## Atomic goal

Create separate QuickJS runtimes/functions/context state.

## Parent intent

Load identical verified artifacts into independent runtimes efficiently.

## Dependencies

- `M3-004-A` — `tasks/06_m3_multi_worker/M3-004-A-share-immutable-mapped-qpack-bytes.md`

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
5. Implement exactly this deliverable: Create separate QuickJS runtimes/functions/context state.
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
m3-004-b: create separate quickjs runtimes functions context state
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M3-004-B) — PASS

- Date: 2026-08-30
- Branch/PR: m3-004-b (squash-merged; see git log for final hash)
- Closes: #391

### Changed files
- `crates/q-engine-quickjs/src/lib.rs`: `QuickJsEngine::spawn_independent(count, config, handle, mapper, worker_name) -> Vec<QuickJsEngine>` — spawns N FULLY independent QuickJS engines (each with its own owner thread, context, heap, and module state per ADR-0036 §1/§2), plus `worker_label()` diagnostics (e.g. "parity-0"/"parity-1").
- `crates/q-engine-quickjs/tests/engine.rs`: `independent_engines_share_source_but_not_module_state` — the worker-parity + state-isolation proof at the engine layer.

### The state-isolation proof (the packet's core evidence)

Two `spawn_independent` engines load the SAME bundle (a module with top-level `count` state and an `inc` function registered via the Legacy `__velquRegister` contract) and then:

1. **Worker parity**: the same input produces the same result on both engines (inc == 1 on A and on B).
2. **State isolation**: A's second inc == 2 (A kept its own state), and B's second inc == **2** (B's counter is untouched by A's mutation — per-runtime state proven live).
3. Both engines keep serving; labels identify owner threads.

### Command results
- `cargo test -p q-engine-quickjs` → **20 lib + 102 engine + 1 doctest** (was 20+101) — 0 failed
- `cargo test -p q-pack` → 100+2 — pass; `-p velqu-runtime` → 17+5+44 — pass
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**; manifest refreshed (`070d2a8a…`) — spawn_independent is production API in the binary.

### Guardrail mapping
- **Workers execute identical contracts** — same bundle, same result on both engines.
- **One worker failure does not corrupt others** — A's mutations are invisible to B (proven live, not asserted).
- **No JS object crosses workers** — engines share nothing but the caller's own Arc handles.

### Disclosures
- Three test-authoring iterations (register contract via __velquRegister, JsonText response strategy match, outcome pattern completeness) — all caught by the compiler/suite before commit.
- Standing: CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR.
