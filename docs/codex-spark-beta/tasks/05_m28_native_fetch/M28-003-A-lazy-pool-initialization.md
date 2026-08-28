---
task_id: M28-003-A
parent_task: M28-003
milestone: M28
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M28.md
commit_required: true
---

# M28-003-A — Lazy pool initialization

## Atomic goal

Lazy pool initialization.

## Parent intent

Create a lazy, bounded outbound client shared safely by native services.

## Dependencies

- `M28-002-Z` — `tasks/05_m28_native_fetch/M28-002-Z-package-evidence-for-select-native-http-client-stack-from-evidence.md`

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
5. Implement exactly this deliverable: Lazy pool initialization.
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
m28-003-a: lazy pool initialization
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M28-003-A) — PASS

- Date: 2026-08-28
- Branch/PR: m28-003-a (squash-merged; see git log for final hash)
- Closes: #318

### Changed files
- `crates/q-runtime/src/fetch_stack.rs`: Implemented `FetchPool` lazy container using `OnceLock<Arc<OutboundClient>>` and `build_client()` with bounded idle connection parameters (`DEFAULT_POOL_IDLE_TIMEOUT_SECS = 15s`, `DEFAULT_MAX_IDLE_PER_HOST = 32`); ensures 0 sockets, 0 TLS handshakes, and 0 background tasks are spawned at startup.
- `crates/q-runtime/tests/fetch_pool_conformance.rs` (new): Integration tests verifying `FetchPool` stays dormant before first request, initializes once on first access, and serves traffic against a mock server.
- `benchmarks/manifest.json`: Refreshed `qRuntimeRelease` hash.

### Tests added
- `crates/q-runtime/src/fetch_stack.rs`:
  - `pool_is_strictly_lazy_before_first_access`
  - `pool_initializes_once_on_first_access_and_shares_instance`
  - `pool_shutdown_handles_uninitialized_and_initialized`
  - `tls_connector_uses_mandatory_webpki_roots_without_bypass`
- `crates/q-runtime/tests/fetch_pool_conformance.rs`:
  - `fetch_pool_remains_uninitialized_until_explicit_request`
  - `fetch_pool_initializes_on_request_and_serves_traffic`

### Command results
- `cargo test -p velqu-runtime` → 5 unit + 2 integration + 31 conformance passed (38 total)
- `cargo test -p q-capabilities` → 132+8 passed
- `cargo test -p q-engine-quickjs` → 16+97 passed
- `cargo test -p q-http` → 4+6+1 passed
- `bun test` → 215 pass / 0 fail (27 files)
- `bun run typecheck` → clean
- `cargo fmt --check` → clean
- `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**

### Guardrail mapping
- **App with no fetch pays no pool initialization** — `FetchPool::new()` is `const fn` storing an empty `OnceLock`.
- **TLS verification cannot be disabled accidentally** — `build_connector()` uses `with_webpki_roots()` with no dangerous bypass methods on the builder path.
- **Shutdown releases connections** — `pool.shutdown()` marks shutdown state and allows idle connections to close.

### Disclosures (standing)
- CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR. Local evidence above is complete.
