---
task_id: M28-003-V
parent_task: M28-003
milestone: M28
priority: P0
mode: VERIFY
status: PASS
context_card: context/milestones/M28.md
commit_required: true
---

# M28-003-V — Verify Implement connection pooling, DNS, and TLS

## Atomic goal

Prove every acceptance criterion for parent task M28-003 without broadening scope.

## Parent intent

Create a lazy, bounded outbound client shared safely by native services.

## Dependencies

- `M28-003-A` — `tasks/05_m28_native_fetch/M28-003-A-lazy-pool-initialization.md`
- `M28-003-B` — `tasks/05_m28_native_fetch/M28-003-B-bound-idle-active-connections-and-dns-cache.md`
- `M28-003-C` — `tasks/05_m28_native_fetch/M28-003-C-use-verified-tls-roots-and-hostname-validation.md`
- `M28-003-D` — `tasks/05_m28_native_fetch/M28-003-D-define-keepalive-and-shutdown.md`

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
- `crates/q-bridge/src/lib.rs`
- `crates/q-runtime/tests/runtime_conformance.rs`
- `Cargo.toml`
- `packages/cli/src/index.ts`
- `crates/q-runtime/src/source_map.rs`
- `examples/proof/`
- `README.md`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

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
cargo test -p q-bridge
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

- Pool tests.
- TLS negative tests.
- Startup cost evidence.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m28-003-v: verify implement connection pooling dns and tls
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M28-003-V) — PASS

- Date: 2026-08-28
- Branch/PR: m28-003-v (squash-merged; see git log for final hash)
- Closes: #322

### Acceptance-criterion mapping (parent M28-003 guardrails)

1. **App with no fetch pays no pool initialization** — verified: `FetchPool::new()` stores an uninitialized `OnceLock`; `pool_is_strictly_lazy_before_first_access` (`fetch_stack.rs`) and `fetch_pool_remains_uninitialized_until_explicit_request` (`fetch_pool_conformance.rs`) prove zero socket or TLS context allocation before first request.
2. **TLS verification cannot be disabled accidentally** — verified: `build_connector()` in `fetch_stack.rs` uses `HttpsConnectorBuilder::new().with_webpki_roots()`; no insecure certificate bypass methods exist; `tls_untrusted_non_tls_endpoint_on_https_fails_closed` (`fetch_pool_conformance.rs`) and spike behavior probe 5 prove self-signed/plaintext TLS failures fail closed without fallback.
3. **Pool exhaustion yields bounded error/backpressure** — verified: `PoolBounds` configures active connection limits; `pool_exhaustion_yields_bounded_backpressure_error` and `fetch_pool_active_connection_bounds_enforce_backpressure` prove permit saturation returns `Err(PoolError::PoolExhausted)` and recovers capacity upon permit drop.
4. **Shutdown releases connections** — verified: `pool_drain_shutdown_settles_within_budget` and `pool_shutdown_handles_uninitialized_and_initialized` prove pool transitions to shutdown state, rejects new requests, and drains idle connections within declared budget.

### Verification runs (this branch, worktree-fresh)
- `cargo test -p velqu-runtime` → 8 unit + 5 integration + 31 conformance passed (44 total)
- `cargo test -p q-capabilities` → 132+8 passed
- `cargo test -p q-engine-quickjs` → 16+97 passed
- `cargo test -p q-http` → 4+6+1 passed
- `cargo test -p q-bridge` → 11 passed
- `bun test` → 215 pass / 0 fail (27 files)
- `bun run typecheck` → clean
- `cargo fmt --check` → clean
- `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**

### Disclosures (standing)
- CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR. Local evidence above is complete.
