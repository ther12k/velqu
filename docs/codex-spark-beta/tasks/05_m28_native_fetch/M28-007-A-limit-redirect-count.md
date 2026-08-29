---
task_id: M28-007-A
parent_task: M28-007
milestone: M28
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M28.md
commit_required: true
---

# M28-007-A — Limit redirect count

## Atomic goal

Limit redirect count.

## Parent intent

Handle redirects and encoded responses safely and predictably.

## Dependencies

- `M28-003-Z` — `tasks/05_m28_native_fetch/M28-003-Z-package-evidence-for-implement-connection-pooling-dns-and-tls.md`
- `M28-004-Z` — `tasks/05_m28_native_fetch/M28-004-Z-package-evidence-for-implement-request-response-and-headers-subset.md`

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

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Limit redirect count.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Redirect loops fail boundedly.
- Sensitive headers never leak cross-origin.
- Zip-bomb style expansion is limited.
- Observed URL/status follows documented semantics.

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

- Security fixtures.
- Compression limits tests.
- Redirect conformance.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m28-007-a: limit redirect count
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M28-007-A) — PASS

- Date: 2026-08-29
- Branch/PR: m28-007-a (squash-merged; see git log for final hash)
- Closes: #342

### Changed files
- `crates/q-capabilities/src/fetch_policy.rs`: stateful per-request redirect limiter —
  - `RedirectLimiter::new(FetchPolicy)` / `hops()` / `limit()`: drives the fetch follow loop against the frozen policy.
  - `evaluate(from_url, to_url) -> Result<RedirectOutcome, FetchPolicyError>`: every 3xx hop passes through the ONE policy path — `FetchPolicy::check_redirect_hop` (scheme allowlist, https→http downgrade denial, hop ceiling vs the policy's `max_hops`, default `MAX_REDIRECT_HOPS` = 20) — plus loop detection over the visited set: returning to an already-visited URL fails typed `FetchPolicyError::RedirectLoop` (new variant) immediately instead of burning the ceiling. `RedirectPolicy::Manual` yields `RedirectOutcome::Surface` (3xx to caller) with zero hop consumption. Denied/failed hops leave the limiter state unchanged.
  - Memory bounded by construction: at most `max_hops` visited URLs (URL strings arrive via Location headers, already bounded by header limits).
  - `RedirectOutcome::{Follow, Surface}` enum; `url_scheme()` helper (no scheme delimiter → empty → fail closed via the allowlist).
- `crates/q-capabilities/src/lib.rs`: re-exports `RedirectLimiter`, `RedirectOutcome`.

### Tests added (fetch_policy.rs, +5 → 154 lib tests)
- `redirect_limiter_follows_up_to_ceiling_then_fails_typed` (hops 1..=20 follow with distinct targets; hop 21 → `TooManyRedirects{20}`)
- `manual_policy_surfaces_3xx_without_following` (Surface, hops stay 0)
- `redirect_loop_fails_fast_and_typed` (a→b→a loop fires `RedirectLoop` at the revisit, before the ceiling; failed hop not counted)
- `scheme_and_downgrade_denials_flow_through_limiter` (DowngradeRedirect / SchemeNotAllowed / scheme-less URL fail closed; limiter unchanged)
- `custom_hop_limit_is_respected_exactly` (Follow{3}: hops 1–3 follow, 4 → `TooManyRedirects{3}`)

### Command results
- `cargo test -p q-capabilities` → **154 unit (was 149) + 4 backpressure + 8 WPT** — 0 failed
- `cargo test -p q-engine-quickjs` → 18+101 · `-p q-http` 4+6+1 · `-p q-bridge` 11 · `-p velqu-runtime` 8+5+31 — all pass
- `bun test` → 219 pass / 0 fail; `bun run typecheck` → clean (via ./scripts/verify)
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**
- Release binary hash unchanged (`ef142331…` matches manifest): pure policy additions are dead-code-eliminated until the fetch executor wires in.

### Guardrail mapping
- **Redirect loops fail boundedly** — hop ceiling (`TooManyRedirects` at ≤ 20) as the hard backstop + typed `RedirectLoop` fast-path; both bounded in hops and memory.
- **Observed URL/status follows documented semantics** — per-hop evaluation goes through the same `check_redirect_hop` used everywhere; no second policy path.
- Executor wiring consumes this limiter when dialing lands (M28-003 stack dormant by design).

### Disclosures
- One placement slip: the limiter block was first inserted inside `impl FetchPolicy` and one helper returned a reference to a temporary — both caught by compile before commit, fixed by relocation/signature. No test weakened.
- Two `field_reassign_with_default` clippy lints in new tests fixed with struct-update syntax.
- Standing: CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR.
