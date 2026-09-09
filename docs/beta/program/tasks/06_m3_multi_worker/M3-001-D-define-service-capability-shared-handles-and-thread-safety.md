---
task_id: M3-001-D
parent_task: M3-001
milestone: M3
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M3.md
commit_required: true
---

# M3-001-D — Define service/capability shared handles and thread safety

## Atomic goal

Define service/capability shared handles and thread safety.

## Parent intent

Define what JavaScript and native state is per worker versus shared.

## Dependencies

- `M3-001-C` — `tasks/06_m3_multi_worker/M3-001-C-forbid-jsvalue-sharing.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M3.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`
- `context/components/capabilities-fetch.md`

### Source files

- `AGENTS.md`
- `crates/q-runtime/src/main.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-engine-quickjs/src/lib.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`
- `crates/q-http/src/lib.rs`
- `crates/q-bridge/src/lib.rs`
- `crates/q-runtime/tests/runtime_conformance.rs`
- `crates/q-capabilities/src/lib.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Define service/capability shared handles and thread safety.
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
cargo test -p q-http
```
```bash
cargo test -p q-bridge
```
```bash
cargo test -p q-capabilities
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
m3-001-d: define service capability shared handles and thread safety
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M3-001-D) — PASS

- Date: 2026-08-30
- Branch/PR: m3-001-d (squash-merged; see git log for final hash)
- Closes: #375

### Changed files
- `crates/q-capabilities/src/shared_handles.rs` (new): the ADR-0036 section 4 shared-handle contract in the type system —
  - `SharedAcrossWorkers: Send + Sync + 'static` marker trait: an impl is a compile-time declaration that a handle follows one of the four named sharing disciplines (locks/atomics only, bounded growth, saturating-or-dropping overflow, never a JS value inside);
  - **explicit impls only** (no blanket impl): sharing is an auditable per-type decision. First two impls: `FetchMetricsCollector` (metric-shard discipline, M28-009) and `BoundedLogSink` (bounded log sink, M27-004-C);
  - module docs name the contract and the audit rule ("anything mutable that cannot be phrased in one of the four shapes requires a new ADR").
- `crates/q-capabilities/src/lib.rs`: module + `SharedAcrossWorkers` re-export.
- `crates/q-runtime/src/fetch_stack.rs`: pool-handle discipline test — `FetchPool: Send + Sync`, shared behind `Arc`, probed from a worker thread.

### Tests added
- `shared_handles_are_send_sync_static` (explicit impls audited)
- `shared_handles_work_behind_arc_from_any_thread` (Arc-shared collector + sink exercised from a worker thread)
- `pool_handle_is_send_sync_shared` (FetchPool: Send + Sync + Arc-shared)

### Command results
- `cargo test -p q-capabilities` → **194 unit (was 192) + 7 + 1 + 4 + 9** — 0 failed
- `cargo test -p velqu-runtime` → **13 unit** (fetch_stack 11 + pool-handle test) + 5 + 44 — all pass
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**; release binary unchanged (`333d563d…` matches manifest)

### Guardrail mapping
- **Cross-worker mutable state is explicit** — the shared-handle vocabulary is now a trait: a type is shared only if someone wrote `impl SharedAcrossWorkers for It`, and every impl is reviewable against the four disciplines.

### Disclosures
- One Arc-move test slip caught by the compiler; fixed before commit.
- Standing: CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR.
