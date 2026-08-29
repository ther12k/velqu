---
task_id: M28-007-C
parent_task: M28-007
milestone: M28
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M28.md
commit_required: true
---

# M28-007-C — Define credential/header stripping

## Atomic goal

Define credential/header stripping.

## Parent intent

Handle redirects and encoded responses safely and predictably.

## Dependencies

- `M28-007-B` — `tasks/05_m28_native_fetch/M28-007-B-reapply-ssrf-dns-policy-on-every-hop.md`

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
5. Implement exactly this deliverable: Define credential/header stripping.
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
m28-007-c: define credential header stripping
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M28-007-C) — PASS

- Date: 2026-08-29
- Branch/PR: m28-007-c (squash-merged; see git log for final hash)
- Closes: #344

### Changed files
- `crates/q-capabilities/src/fetch_policy.rs`: credential/header stripping policy (ADR-0033 §4, WHATWG fetch HTTP-redirect alignment) —
  - `CREDENTIAL_REDIRECT_HEADERS`: closed, lowercased set — `authorization`, `cookie`, `cookie2`, `proxy-authorization`.
  - `url_origin(url) -> Option<String>`: normalized origin (lowercased scheme/host, default ports elided: http 80 / https 443); `None` on malformed input.
  - `is_cross_origin_redirect(from, to)`: scheme/host/effective-port comparison; malformed URLs count as cross-origin (strip = fail-closed direction).
  - `is_credential_header(name)`: case-insensitive membership.
  - `headers_surviving_redirect(from, to, names)`: the executor's one-call hop filter — drops credential headers on cross-origin hops (and on malformed URLs), preserves everything else in input order with original casing, deduplicated case-insensitively.
- `crates/q-capabilities/src/lib.rs`: re-exports the five new symbols.

### Tests added (fetch_policy.rs, +4 → 162 lib tests)
- `cross_origin_hops_strip_credential_headers` (host change / scheme change / port change all drop Authorization, Cookie, Proxy-Authorization; non-credential headers survive)
- `same_origin_hops_keep_credentials` (path/query-only redirects keep everything; default-port elision: https://a:443 == https://a; differing port is cross-origin)
- `credential_header_detection_is_case_insensitive_and_closed` (set membership + non-membership edges like `authorizationx` and `www-authenticate`; the constant itself is pinned)
- `malformed_redirect_targets_fail_closed_to_stripping` (garbage URLs are cross-origin by definition, so credentials are stripped)

### Command results
- `cargo test -p q-capabilities` → **162 unit (was 158) + 4 backpressure + 8 WPT** — 0 failed
- `cargo test -p q-engine-quickjs` → 18+101 · `-p q-http` 4+6+1 · `-p q-bridge` 11 · `-p velqu-runtime` 8+5+31 — all pass
- `bun test` → 219 pass / 0 fail; `bun run typecheck` → clean (via ./scripts/verify)
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**
- Release binary hash unchanged (`ef142331…` matches manifest) — policy additions dead-code-eliminated until the executor wires in.

### Guardrail mapping
- **Sensitive headers never leak cross-origin** — the closed credential set is stripped on every origin change, with malformed-URL fail-closed semantics; the executor gets a single filter function to apply per hop.

### Disclosures
- The first insertion attempt silently no-opped (a heredoc `src.replace` whose anchor handling failed without erroring) — caught because the tests referencing the functions failed E0432; re-applied with the file-edit tool which fails loudly. Test casing expectations were then aligned with the preserved-casing contract (surviving names echo input casing). No production behavior changed beyond the new policy surface.
- Standing: CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR.
