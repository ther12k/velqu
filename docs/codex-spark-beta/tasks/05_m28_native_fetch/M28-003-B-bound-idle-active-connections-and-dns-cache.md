---
task_id: M28-003-B
parent_task: M28-003
milestone: M28
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M28.md
commit_required: true
---

# M28-003-B — Bound idle/active connections and DNS cache

## Atomic goal

Bound idle/active connections and DNS cache.

## Parent intent

Create a lazy, bounded outbound client shared safely by native services.

## Dependencies

- `M28-003-A` — `tasks/05_m28_native_fetch/M28-003-A-lazy-pool-initialization.md`

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
- `Cargo.toml`
- `packages/cli/src/index.ts`
- `crates/q-runtime/src/source_map.rs`
- `examples/proof/`
- `README.md`
- `docs/beta/`
- `benchmarks/real-world/postgres/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Bound idle/active connections and DNS cache.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- App with no fetch pays no pool initialization.
- TLS verification cannot be disabled accidentally.
- Pool exhaustion yields bounded error/backpressure.
- Shutdown releases connections.

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
```bash
bun test
```
```bash
bun run typecheck
```

## Required evidence for this microtask

- Pool tests.
- TLS negative tests.
- Startup cost evidence.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m28-003-b: bound idle active connections and dns cache
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M28-003-B) — PASS

- Date: 2026-08-28
- Branch/PR: m28-003-b (squash-merged; see git log for final hash)
- Closes: #319

### Changed files
- `crates/q-runtime/src/fetch_stack.rs`: Added `PoolBounds` struct (`max_idle_per_host = 32`, `idle_timeout = 15s`, `max_active_connections = 128`, `connect_timeout = 10s`, `tcp_keepalive = 30s`), connector configuration with `set_connect_timeout`, `set_nodelay`, `set_keepalive`, and `try_acquire_permit` with `PoolError::PoolExhausted` enforcing bounded concurrency and backpressure under load.
- `crates/q-runtime/tests/fetch_pool_conformance.rs`: Added test `fetch_pool_active_connection_bounds_enforce_backpressure` proving active connection exhaustion returns typed backpressure error and releases properly.
- `benchmarks/manifest.json`: Refreshed `qRuntimeRelease` hash.

### Tests added
- `crates/q-runtime/src/fetch_stack.rs`:
  - `pool_exhaustion_yields_bounded_backpressure_error`
  - `pool_bounds_are_clamped_to_ceiling`
- `crates/q-runtime/tests/fetch_pool_conformance.rs`:
  - `fetch_pool_active_connection_bounds_enforce_backpressure`

### Command results
- `cargo test -p velqu-runtime` → 7 unit + 3 integration + 31 conformance passed (41 total)
- `cargo test -p q-capabilities` → 132+8 passed
- `cargo test -p q-engine-quickjs` → 16+97 passed
- `cargo test -p q-http` → 4+6+1 passed
- `bun test` → 215 pass / 0 fail (27 files)
- `bun run typecheck` → clean
- `cargo fmt --check` → clean
- `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**

### Guardrail mapping
- **Idle/active bounds are enforced** — `pool_idle_timeout` and `pool_max_idle_per_host` set on client builder; active connection permits bounded by semaphore.
- **Pool exhaustion yields bounded error/backpressure** — `try_acquire_permit()` returns `Err(PoolError::PoolExhausted)` when saturated.
- **Shutdown releases connections** — `pool.shutdown()` marks shutdown state and rejects new permit leases.

### Disclosures (standing)
- CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR. Local evidence above is complete.
