---
task_id: M28-008-B
parent_task: M28-008
milestone: M28
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M28.md
commit_required: true
---

# M28-008-B — Revalidate redirects and connection targets

## Atomic goal

Revalidate redirects and connection targets.

## Parent intent

Provide explicit controls for metadata, loopback, private networks, and DNS rebinding.

## Dependencies

- `M28-008-A` — `tasks/05_m28_native_fetch/M28-008-A-resolve-and-validate-addresses-before-connect.md`

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

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Revalidate redirects and connection targets.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Cloud metadata endpoints blocked by safe default.
- DNS rebinding tests fail closed.
- IPv4/IPv6/private ranges handled.
- Policy decisions are observable without logging secrets.

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

- SSRF test suite.
- Threat-model update.
- Configuration examples.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m28-008-b: revalidate redirects and connection targets
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M28-008-B) — PASS

- Date: 2026-08-29
- Branch/PR: m28-008-b (squash-merged; see git log for final hash)
- Closes: #349

### Changed files
- `crates/q-capabilities/src/fetch_policy.rs`: revalidation composition on the redirect limiter —
  - `RedirectLimiter::follow_hop(from_url, to_url, resolve) -> Result<FollowedHop, FetchPolicyError>`: the executor's ONE-call revalidation gate, atomic by construction — URL-level checks (scheme allowlist, downgrade denial, hop ceiling, loop detection) run first; the hop target is resolved and EVERY address validated through `resolve_and_validate` (including metadata-by-name denial for redirect targets); only after all checks pass is hop state committed. A failed attempt leaves limiter state exactly as before.
  - `FollowedHop { outcome, pinned }`: on `Follow`, `pinned` is the validated dial-ready address set (the connector dials these and never re-resolves); empty for `Surface`.
  - `RedirectLimiter::seed_target(url)`: records the initial request target as visited (no hop consumed) so a chain that leads back to the origin URL fails via the typed `RedirectLoop` path.
- `crates/q-capabilities/src/lib.rs`: re-exports `FollowedHop`.

### Tests added (fetch_policy.rs, +5 → 176 lib tests)
- `follow_hop_resolves_and_validates_atomically` (public hop follows with pin set; DNS failure after URL checks leaves `hops()` unchanged; a valid hop still follows afterwards)
- `redirect_targets_deny_metadata_by_name_too` (redirect to `metadata.google.internal` denied by name with the resolver provably untouched)
- `follow_hop_pin_set_is_the_only_dial_set` (mixed public+private answer denied; clean answer pins exactly the validated address)
- `manual_gate_surfaces_without_any_resolution` (Surface, empty pins, resolver never called)
- `full_fetch_sequence_composes_open_and_hops` (executor shape: `resolve_and_validate` opens the origin target, `seed_target` records it, same-origin hop follows, redirect back to the origin URL is a typed loop denial with state unchanged)

### Command results
- `cargo test -p q-capabilities` → **176 unit (was 171) + 4 backpressure + 8 WPT** — 0 failed
- `cargo test -p q-engine-quickjs` → 18+101 · `-p q-http` 4+6+1 · `-p q-bridge` 11 · `-p velqu-runtime` 8+5+31 — all pass
- `bun test` → 219 pass / 0 fail; `bun run typecheck` → clean (via ./scripts/verify)
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**
- Release binary hash unchanged (`ef142331…` matches manifest) — policy additions dead-code-eliminated until the executor wires in.

### Guardrail mapping
- **DNS rebinding tests fail closed** — resolution is atomic with hop checks: no committed-but-undialable states, no unvalidated dials; pins are the only dial set.
- **Policy decisions observable without logging secrets** — typed errors carry host/class/reason only (no header values, no credentials).

### Disclosures
- The composition test caught a real semantic gap before commit: the original request target was not in the limiter's visited set, so a chain redirecting back to the origin URL escaped loop detection. Fixed with `seed_target` (no hop consumed); the test now pins that semantics.
- Standing: CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR.
