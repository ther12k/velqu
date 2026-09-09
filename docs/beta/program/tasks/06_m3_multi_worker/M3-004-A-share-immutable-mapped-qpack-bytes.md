---
task_id: M3-004-A
parent_task: M3-004
milestone: M3
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M3.md
commit_required: true
---

# M3-004-A — Share immutable mapped QPack bytes

## Atomic goal

Share immutable mapped QPack bytes.

## Parent intent

Load identical verified artifacts into independent runtimes efficiently.

## Dependencies

- `M3-002-Z` — `tasks/06_m3_multi_worker/M3-002-Z-package-evidence-for-implement-bounded-worker-dispatcher.md`
- `M26-GATE` — `gates/M26-GATE.md`

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
5. Implement exactly this deliverable: Share immutable mapped QPack bytes.
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
m3-004-a: share immutable mapped qpack bytes
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M3-004-A) — PASS

- Date: 2026-08-30
- Branch/PR: m3-004-a (squash-merged; see git log for final hash)
- Closes: #390

### Changed files
- `crates/q-pack/src/lib.rs`: `SharedPack` — the shared verified pack artifact (M3-004-A, ADR-0036 §3) —
  - `SharedPack::freeze(pack, bytes)`: verify once, freeze forever. `pack: Arc<QPack>` + `bytes: Arc<[u8]>` + the SHA-256 identity pin computed at freeze time.
  - `pack()` / `bytes()` / `sha256()` — Arc-shared reads with clone cost = refcount bump; every worker gets the SAME artifact.
  - Explicit `impl q_capabilities::SharedAcrossWorkers for SharedPack` — the auditable sharing decision (shared-immutable discipline).
- `mod shared_pack_tests` (4 tests) — worker parity, freeze immutability, cross-thread sharing.

### Tests added (4)
- `freeze_is_idempotent_per_bytes_and_shareable` (identity pin matches frozen bytes; `Arc::ptr_eq` proves workers share ONE verified artifact)
- `worker_parities_hold_for_every_shared_clone` (4 "workers": identical routes/ABI/contract-version from the shared clone)
- `shared_bytes_are_frozen_against_mutation` (a worker mutating its own clone cannot touch the frozen artifact — one worker failure does not corrupt others)
- `cross_thread_sharing_never_blocks_and_never_shares_js` (8 threads, barrier-synchronized concurrent reads; identical identity hashes; reads never block; nothing shared is a JS value)

### Command results
- `cargo test -p q-pack` → **100 unit + 2 fuzz** (was 96+2) — 0 failed
- `cargo test -p q-engine-quickjs` → 20+101 · `-p velqu-runtime` → 17+5+44 — all pass
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**
- `benchmarks/manifest.json` refreshed (`d24b1a7c…`) — SharedPack lands in the runtime artifact.

### Guardrail mapping
- **Workers execute identical contracts** — `worker_parities_hold_for_every_shared_clone`.
- **One worker failure does not corrupt others** — `shared_bytes_are_frozen_against_mutation`.
- **No JS object crosses workers** — the shared artifact is plain bytes (`T: Send` discipline; the compile_fail rule from M3-001-C still guards).
- **Artifact memory sharing is measured** — Arc refcount semantics proven by `Arc::ptr_eq` + concurrent-read test; M3-004-D owns the measured report.

### Disclosures
- Three clippy/compile iterations on the new test module (module placement outside cfg(test), unused imports). No production behavior changed beyond the new frozen-artifact type.
- Standing: CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR.
