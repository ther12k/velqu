---
task_id: M28-007-B
parent_task: M28-007
milestone: M28
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M28.md
commit_required: true
---

# M28-007-B — Reapply SSRF/DNS policy on every hop

## Atomic goal

Reapply SSRF/DNS policy on every hop.

## Parent intent

Handle redirects and encoded responses safely and predictably.

## Dependencies

- `M28-007-A` — `tasks/05_m28_native_fetch/M28-007-A-limit-redirect-count.md`

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
5. Implement exactly this deliverable: Reapply SSRF/DNS policy on every hop.
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
m28-007-b: reapply ssrf dns policy on every hop
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M28-007-B) — PASS

- Date: 2026-08-29
- Branch/PR: m28-007-b (squash-merged; see git log for final hash)
- Closes: #343

### Changed files
- `crates/q-capabilities/src/fetch_policy.rs`: SSRF/DNS revalidation on every redirect hop —
  - `RedirectLimiter::evaluate_resolved(from_url, to_url, resolved: &[IpAddr])`: hop evaluation **with SSRF/DNS revalidation** — after the URL-level checks (scheme allowlist, downgrade denial, hop ceiling, loop detection) and before any state commit, the redirect target's host is validated through `FetchPolicy::check_resolved`: EVERY resolved address must pass the trust-mode policy. A public origin can never lure a hop into loopback, link-local, private, or cloud-metadata space. A denied hop leaves limiter state unchanged (checks precede the commit).
  - Internal refactor: `check_hop_urls` (URL-level checks, no mutation) + `commit_hop` (only after every check passed); `evaluate` keeps its exact prior behavior.
  - `url_host()` helper: host extraction with userinfo/port handling (IPv6 bracket literals kept intact); malformed input → empty → fail closed via the resolved-address check.
- `crates/q-capabilities/src/lib.rs`: no new exports needed (method on the already-exported `RedirectLimiter`).

### Tests added (fetch_policy.rs, +4 → 158 lib tests)
- `ssrf_policy_is_reapplied_on_every_hop` (public→public follows; hop to a loopback-resolving target denied `AddressDenied{Loopback}` with state unchanged; subsequent public hop follows)
- `redirect_target_resolving_partial_loopback_is_denied` (DNS round-robin with one private address poisons the host — every address must pass)
- `loopback_trust_still_denies_metadata_space_on_hops` (explicit loopback testing mode dials 127.0.0.1 but still denies 169.254.169.254 — its own `AddressClass::Metadata`)
- `empty_resolution_and_ceiling_precede_address_checks` (empty DNS → typed `HostnameDenied`; over-ceiling hop → `TooManyRedirects` before address checks matter)

### Command results
- `cargo test -p q-capabilities` → **158 unit (was 154) + 4 backpressure + 8 WPT** — 0 failed
- `cargo test -p q-engine-quickjs` → 18+101 · `-p q-http` 4+6+1 · `-p q-bridge` 11 · `-p velqu-runtime` 8+5+31 — all pass
- `bun test` → 219 pass / 0 fail; `bun run typecheck` → clean (via ./scripts/verify)
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**
- Release binary hash unchanged (`ef142331…` matches manifest) — policy additions dead-code-eliminated until the executor wires in.

### Guardrail mapping
- **Observed URL/status follows documented semantics** — SSRF revalidation rides the same `RedirectLimiter` hop path (one policy path); the executor's future DNS hook has a single, typed entry point.
- **Redirect loops fail boundedly** — revalidation composes with (never replaces) the ceiling/loop checks from M28-007-A.

### Disclosures
- First test pass caught two wrong assumptions, fixed before commit: an https→http metadata target trips the downgrade rule before the address check (test retargeted to an https loopback URL; the ordering is separately pinned), and 169.254.169.254 has its own `AddressClass::Metadata` (assertion corrected). A temporary debug test file was used to print the actual variants and removed.
- Standing: CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR.
