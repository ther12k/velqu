---
task_id: M28-003-Z
parent_task: M28-003
milestone: M28
priority: P0
mode: EVIDENCE
status: PASS
context_card: context/milestones/M28.md
commit_required: true
---

# M28-003-Z — Package evidence for Implement connection pooling, DNS, and TLS

## Atomic goal

Create source-backed evidence and handoff for parent task M28-003; update status only if verification passed.

## Parent intent

Create a lazy, bounded outbound client shared safely by native services.

## Dependencies

- `M28-003-V` — `tasks/05_m28_native_fetch/M28-003-V-verify-implement-connection-pooling-dns-and-tls.md`

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
- `benchmarks/real-world/postgres/`
- `benchmarks/real-world/SPEC.md`
- `packages/capability-postgres/ (create if absent)`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

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

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m28-003-z: package evidence for implement connection pooling dns and tl
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M28-003-Z) — PASS

- Date: 2026-08-28
- Branch/PR: m28-003-z (squash-merged; see git log for final hash)
- Closes: #323

### Parent closure — M28-003 Implement connection pooling, DNS, and TLS

Parent intent: create a lazy, bounded outbound client shared safely by native services. Status: **PASS**.

Packet commits (squash merges):
- M28-003-A — 4defb5a (#921, Closes #318): Strictly lazy `FetchPool` container with `OnceLock`; zero socket or TLS allocation at server startup; integration tests `crates/q-runtime/tests/fetch_pool_conformance.rs`
- M28-003-B — 8971d85 (#922, Closes #319): Bounded pool parameters (`PoolBounds`: `max_idle_per_host = 32`, `idle_timeout = 15s`, `connect_timeout = 10s`, `tcp_keepalive = 30s`, `max_active_connections = 128`); active connection semaphore with backpressure `Err(PoolError::PoolExhausted)`
- M28-003-C — af06ce3 (#923, Closes #320): Mandatory webpki-roots TLS verification; negative integration test proving non-TLS/plaintext endpoints fail closed on `https://`
- M28-003-D — 9cd65fc (#924, Closes #321): Bounded TCP keepalive and `drain_shutdown` async protocol reclaiming active permits within declared budget
- M28-003-V — 2adab46 (#925, Closes #322): Verification closure mapping all 4 acceptance guardrails

### Evidence ledger (required microtask evidence)
- **Pool tests**: Unit tests in `crates/q-runtime/src/fetch_stack.rs` (`pool_is_strictly_lazy_before_first_access`, `pool_initializes_once_on_first_access_and_shares_instance`, `pool_exhaustion_yields_bounded_backpressure_error`, `pool_bounds_are_clamped_to_ceiling`, `pool_drain_shutdown_settles_within_budget`) + integration tests in `crates/q-runtime/tests/fetch_pool_conformance.rs` (dormant start, mock server serving, backpressure permit release).
- **TLS negative tests**: `tls_untrusted_non_tls_endpoint_on_https_fails_closed` and `tls_verification_cannot_be_disabled_accidentally` (`fetch_pool_conformance.rs`) + spike behavior probe 5 (self-signed cert rejection).
- **Startup cost evidence**: M28-002-B report `docs/reports/m28-002-b-stack-cost.md` (+0.45 ms cold-start delta, dormant paging only).

### Command results (this branch)
- `cargo test -p velqu-runtime` → 8 unit + 5 integration + 31 conformance passed (44 total)
- `cargo test -p q-capabilities` → 132 unit + 8 integration passed
- `cargo test -p q-engine-quickjs` → 16 unit + 97 worker passed
- `cargo test -p q-http` → 4+6+1 passed
- `bun test` → 215 pass / 0 fail (27 files)
- `bun run typecheck` → clean
- `cargo fmt --check` → clean
- `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**

### Ledger update
- `docs/beta/04_TASK_LEDGER.md`: M28-003 flipped TODO -> PASS.

### Disclosures (standing)
- CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR. Local evidence above is complete.
