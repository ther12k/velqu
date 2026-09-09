---
task_id: M28-010-D
parent_task: M28-010
milestone: M28
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M28.md
commit_required: true
---

# M28-010-D — Test proxy and cancellation

## Atomic goal

Test proxy and cancellation.

## Parent intent

Prove the beta subset across success and failure modes.

## Dependencies

- `M28-010-C` — `tasks/05_m28_native_fetch/M28-010-C-fuzz-headers-and-urls.md`

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
- `crates/q-runtime/src/main.rs`
- `docs/beta/`
- `examples/proof/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Test proxy and cancellation.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

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

## Required evidence for this microtask

- Conformance report.
- Fixture inventory.
- Fuzz report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m28-010-d: test proxy and cancellation
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M28-010-D) — PASS

- Date: 2026-08-29
- Branch/PR: m28-010-d (squash-merged; see git log for final hash)
- Closes: #363

### Changed files
- `crates/q-runtime/tests/fetch_proxy_cancellation_conformance.rs` (new, 2 tests) —
  - `ambient_proxy_env_vars_are_never_honored`: poisons `http_proxy`/`https_proxy`/`all_proxy` (+ empty `no_proxy`) at a never-speaking poison listener, then dials a local mock: the request must connect DIRECTLY (200 from the mock) and the poison listener must record ZERO connections. The declared posture is also asserted (`proxy_mode() == Disabled`).
  - `cancelling_mid_body_releases_pool_capacity_without_hang`: slow-body fixture (6 x 200 ms chunks); after the first frame the transfer is cancelled (body dropped, permit released) — the single-permit slot frees within a 2 s bounded poll, and the pool still serves a fresh request to a new target. No hang, no leak.
- `crates/q-runtime/tests/fetch_fixtures/mod.rs`: shared-fixture module annotated `#![allow(dead_code)]` (each test file uses the subset it needs).

### Command results
- `cargo test -p velqu-runtime` → **12 unit + 5 + 44 integration** (36 fixture-suite + 2 proxy/cancellation + existing) — all pass, 0 warnings
- `cargo test -p q-capabilities` → 192+7+1+4+9 — all pass
- `cargo test -p q-engine-quickjs` → 18+101 · `-p q-http` 4+6+1 · `-p q-bridge` 11 — all pass
- `bun test` → 0 fail; `bun run typecheck` → clean (via ./scripts/verify)
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**; release binary unchanged (test-only packet)

### Guardrail mapping
- **Documented subset passes** — proxy isolation and cancellation behavior match the documented postures (ProxyMode::Disabled; permit-based capacity).
- **No panic/hang/unbounded work** — cancellation path bounded (2 s polls); post-cancel service proven.
- **All failures map predictably** — a hypothetical proxy-honoring runtime would fail the zero-hits assertion; cancellation failures would surface as budget timeouts.
- **Skips are explicit** — proxy CONNECT remains the M28-010-A explicit skip (`NO_PROXY_BY_DESIGN`); this packet proves the isolation rather than skipping it.

### Disclosures
- `hyper::body` frame unwrapping is three levels deep (`timeout`/`Option`/`Result`); the compiler caught the depth. Test-only; no production code changed.
- Standing: CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR.
