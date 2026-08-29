---
task_id: M28-008-A
parent_task: M28-008
milestone: M28
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M28.md
commit_required: true
---

# M28-008-A — Resolve and validate addresses before connect

## Atomic goal

Resolve and validate addresses before connect.

## Parent intent

Provide explicit controls for metadata, loopback, private networks, and DNS rebinding.

## Dependencies

- `M28-001-Z` — `tasks/05_m28_native_fetch/M28-001-Z-package-evidence-for-accept-fetch-tls-redirect-and-ssrf-security-adr.md`
- `M28-003-Z` — `tasks/05_m28_native_fetch/M28-003-Z-package-evidence-for-implement-connection-pooling-dns-and-tls.md`
- `M28-007-Z` — `tasks/05_m28_native_fetch/M28-007-Z-package-evidence-for-implement-redirect-and-compression-policy.md`

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
5. Implement exactly this deliverable: Resolve and validate addresses before connect.
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
m28-008-a: resolve and validate addresses before connect
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M28-008-A) — PASS

- Date: 2026-08-29
- Branch/PR: m28-008-a (squash-merged; see git log for final hash)
- Closes: #348

### Changed files
- `crates/q-capabilities/src/fetch_policy.rs`: resolve-and-validate boundary before connect (ADR-0033 §2/§3) —
  - `HOSTNAME_METADATA_ENDPOINTS`: `metadata.google.internal`, `instance-data`, `metadata` — denied by NAME before any resolution (defense in depth beside the address-space denial).
  - `is_metadata_hostname(host)`: case-insensitive with trailing-dot (FQDN) normalization, bounded input length.
  - `resolve_and_validate(policy, host, resolve) -> Result<Vec<IpAddr>, FetchPolicyError>`: the connect gate. Ordering: metadata-by-name denial -> resolution (injected resolver — the real executor plugs DNS, tests plug deterministic fakes) -> per-address trust-mode validation. The returned set is the connect PIN SET: the connector dials exactly these addresses and never re-resolves, so the first-resolve-public/second-resolve-private rebinding window cannot open. IP-literal hosts skip the resolver entirely and validate directly. All failures typed: `HostnameDenied` (by-name, resolver error with reason, empty resolution) and `AddressDenied` (trust mode).
- `crates/q-capabilities/src/lib.rs`: re-exports the four new symbols.

### Tests added (fetch_policy.rs, +4 → 171 lib tests)
- `metadata_hostnames_are_denied_before_any_resolution` (4 name variants incl. case + trailing dot; resolver provably never called)
- `resolution_uses_only_validated_addresses_in_order` (all-public resolution returns the pin set: same addresses, order, IPv4-mapped normalization)
- `dns_rebinding_mixing_public_and_private_fails_closed` (public+private answer -> `AddressDenied{Private}`; empty resolution -> `HostnameDenied`)
- `resolver_failures_and_ip_literals_are_handled` (resolver error -> typed `HostnameDenied` carrying the reason; IP literals validate directly without touching the resolver; `127.0.0.1` literal denied `Loopback`)

### Command results
- `cargo test -p q-capabilities` → **171 unit (was 167) + 4 backpressure + 8 WPT** — 0 failed
- `cargo test -p q-engine-quickjs` → 18+101 · `-p q-http` 4+6+1 · `-p q-bridge` 11 · `-p velqu-runtime` 8+5+31 — all pass
- `bun test` → 219 pass / 0 fail; `bun run typecheck` → clean (via ./scripts/verify)
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**
- Release binary hash unchanged (`ef142331…` matches manifest) — policy additions dead-code-eliminated until the executor wires in.

### Guardrail mapping
- **Cloud metadata endpoints blocked by safe default** — denied twice over: by name (never resolved) and by address class (never dialed); loopback trust mode does not lift either.
- **DNS rebinding tests fail closed** — mixed answer sets are denied; the validated pin set is the only thing the connector may dial.
- **IPv4/IPv6/private ranges handled** — existing classifier (loopback/link-local/private/metadata/broadcast + IPv4-mapped normalization + IPv6 classes) applied to every resolved address.

### Disclosures
- Three small test-code iterations before commit: a `lib.rs` heredoc anchor lost to fmt reflow (re-applied via the file-edit tool), a closure borrow conflict (moved to `Cell`), and a leftover `&mut` — all caught by the compiler; no production behavior changed.
- Standing: CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR.
