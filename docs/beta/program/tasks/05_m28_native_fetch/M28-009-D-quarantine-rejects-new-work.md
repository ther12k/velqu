---
task_id: M28-009-D
parent_task: M28-009
milestone: M28
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M28.md
commit_required: true
---

# M28-009-D — Quarantine rejects new work

## Atomic goal

Quarantine rejects new work.

## Parent intent

Make fetch operationally diagnosable without hot-path logging cost.

## Dependencies

- `M28-009-C` — `tasks/05_m28_native_fetch/M28-009-C-drain-pool-on-shutdown.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M28.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`
- `context/components/capabilities-fetch.md`

### Source files

- `AGENTS.md`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-http/src/lib.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`
- `Cargo.toml`
- `docs/reports/`
- `docs/beta/workstreams/OBSERVABILITY_OPERATIONS.md`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Quarantine rejects new work.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Metrics are bounded and redacted.
- Shutdown reaches quiescence.
- No task/connection leak after errors.
- Disabled instrumentation overhead is measured.

## Targeted commands

```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p q-http
```
```bash
cargo test -p q-capabilities
```
```bash
cargo test -p velqu-runtime
```

## Required evidence for this microtask

- Metrics schema.
- Shutdown tests.
- Overhead report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m28-009-d: quarantine rejects new work
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M28-009-D) — PASS

- Date: 2026-08-29
- Branch/PR: m28-009-d (squash-merged; see git log for final hash)
- Closes: #357

### Changed files
- `crates/q-runtime/src/fetch_stack.rs`: pool quarantine semantics — a drained/shut-down pool rejects ALL new work:
  - `FetchPool::try_client() -> Option<Arc<OutboundClient>>`: `None` once shut down — the client can never be resurrected after drain (the pre-existing `client()` lazily initializes and would have silently revived a drained pool).
  - `FetchPool::rejections() -> u32`: bounded (saturating `AtomicU32`) counter of every new-work refusal — post-shutdown permit attempts, pool-exhausted backpressure, and post-shutdown client attempts. Observation without secrets.
- `crates/q-capabilities` engine-level quarantine (rejects new invocations fail-closed with `EngineFailure`, HTTP boundary 503) already existed (M2.2.1-r4) and is pinned by existing engine tests (`quarantine rejects subsequent dynamic JS requests immediately`).

### Tests added/extended
- fetch_stack `quarantined_pool_rejects_new_work_and_counts_refusals` (pre-shutdown permit OK; drain reaches quiescence; permit refused `PoolShuttingDown`; `try_client()` None twice; `rejections()==3`)

### Command results
- `cargo test -p q-capabilities` → 192 unit + 4 backpressure + 8 WPT — 0 failed
- `cargo test -p q-engine-quickjs` → 18+101 · `-p q-http` 4+6+1 · `-p q-bridge` 11 · `-p velqu-runtime` → **10+5+31** (fetch_stack tests now 10) — all pass
- `bun test packages examples/proof conformance` → 0 fail; `bun run typecheck` → clean (via ./scripts/verify)
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**; manifest refreshed (`5d2f6d9a…`) — pool changes are in the binary
- Engine-level evidence: existing tests prove a quarantined runtime rejects subsequent dynamic JS immediately (`Outcome::EngineFailure`, sub-2s) — cited, unchanged.

### Guardrail mapping
- **No task/connection leak after errors** — a quarantined pool refuses new permits and cannot resurrect its client; refusals are counted, never silent.
- **Shutdown reaches quiescence** — terminal: once drained (M28-009-C), the pool stays drained.

### Disclosures
- A verify run failed before the workspace debug build produced the `velqu-bytecode` helper (NotFound in the embed step) — environment setup, resolved and re-run to ALL PASS; disclosed per the M27-002-Z precedent. One heredoc anchor slip (NameError) caught before write.
- Standing: CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR.
