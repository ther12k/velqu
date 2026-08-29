---
task_id: M28-010-V
parent_task: M28-010
milestone: M28
priority: P0
mode: VERIFY
status: PASS
context_card: context/milestones/M28.md
commit_required: true
---

# M28-010-V — Verify Complete fetch conformance and fault testing

## Atomic goal

Prove every acceptance criterion for parent task M28-010 without broadening scope.

## Parent intent

Prove the beta subset across success and failure modes.

## Dependencies

- `M28-010-A` — `tasks/05_m28_native_fetch/M28-010-A-run-selected-wpt-cases.md`
- `M28-010-B` — `tasks/05_m28_native_fetch/M28-010-B-create-deterministic-dns-tls-redirect-slow-body-fixtures.md`
- `M28-010-C` — `tasks/05_m28_native_fetch/M28-010-C-fuzz-headers-and-urls.md`
- `M28-010-D` — `tasks/05_m28_native_fetch/M28-010-D-test-proxy-and-cancellation.md`

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

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- Documented subset passes.
- No panic/hang/unbounded work.
- All failures map predictably.
- Skips are explicit.

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
cargo fmt --check
```
```bash
cargo clippy --workspace --all-targets -- -D warnings
```
```bash
./scripts/verify
```

## Required evidence for this microtask

- Conformance report.
- Fixture inventory.
- Fuzz report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m28-010-v: verify complete fetch conformance and fault testing
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M28-010-V) — PASS

- Date: 2026-08-29
- Branch/PR: m28-010-v (squash-merged; see git log for final hash)
- Closes: #364

### Acceptance-criterion mapping (parent M28-010 guardrails)

1. **Documented subset passes** — verified: 79/79 pinned manifest vectors green across the TS and Rust executors (v1.2.0: URL 15, text 9, abort 4, crypto 6, fetch 45 across 4 subsets); deterministic fixture suite green. Tests: `fetch_policy_manifest_vectors_execute_against_compiled_policy`, `fetch_m28_policy_manifest_vectors_execute_against_compiled_policy`, `fetch_fixture_conformance` (6), `fetch_proxy_cancellation_conformance` (2).
2. **No panic/hang/unbounded work** — verified: 3,584 fuzz executions per run assert no-panic AND security properties (bounded hops, decompression caps, dialable-only pins, scheme exactness); slow-body cut by explicit budget; handshake failures bounded; cancellation frees capacity within 2 s. Tests: all 7 `fuzz_fetch_inputs` tests, `slow_body_transfer_is_bounded_by_explicit_budget`, `untrusted_tls_endpoints_fail_closed_deterministically`, `cancelling_mid_body_releases_pool_capacity_without_hang`.
3. **All failures map predictably** — verified: every manifest deny vector names its exact typed variant; fuzz properties enumerate the closed error sets; rebinding -> `AddressDenied`, empty DNS -> `HostnameDenied`, budget overrun -> timeout. Tests: `redirect_chain_follows_bounded_and_records_hops`, `dns_rebinding_table_with_private_answer_fails_closed`, plus the executor matchers.
4. **Skips are explicit** — verified: 23 machine-checked skips (v1.2.0 adds `NETWORK_EGRESS_IN_TEST` x2, `NO_PROXY_BY_DESIGN`), vocabulary-enforced in TS and rendered in the regenerated report. Proxy isolation is proven behaviorally (`ambient_proxy_env_vars_are_never_honored`) rather than skipped.

### Required evidence
- **Conformance report**: `docs/reports/m27-010-wpt-wintertc-conformance.md` regenerated at v1.2.0 — 79 pinned vectors (100% PASS) + 23 explicit skips.
- **Fixture inventory**: `crates/q-runtime/tests/fetch_fixtures/mod.rs` — deterministic resolver, RebindingResolver (exactly-once resolution), redirect-chain server, slow-body server, immediate-close server; used by 8 executor-shape tests.
- **Fuzz report**: `crates/q-capabilities/tests/fuzz_fetch_inputs.rs` — 7 properties x 512 iterations (3,584 executions), deterministic xorshift seeds, security invariants beyond no-panic.

### Verification runs (this branch, worktree-fresh)
- `cargo test -p q-capabilities` → 192 unit + 7 fuzz + 1 helper + 4 backpressure + 9 WPT-manifest passed
- `cargo test -p velqu-runtime` → 12 unit + 5 + 44 integration passed (fixtures + proxy/cancellation + SIGTERM)
- `cargo test -p q-engine-quickjs` → 18+101; `-p q-http` → 4+6+1; `-p q-bridge` → 11 passed
- `bun test` → 0 fail; `bun run typecheck` → clean (via ./scripts/verify)
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**; release binary reproduced deterministically (`b8296060…` matches the M28-010-B manifest)

### Disclosures (standing)
- No production code changed in this packet: verification-only closure of M28-010-A/B/C/D.
- CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR. Local evidence above is complete.
