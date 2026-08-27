---
task_id: M28-001-A
parent_task: M28-001
milestone: M28
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M28.md
commit_required: true
---

# M28-001-A — Define URL schemes, redirect policy, DNS rebinding controls, proxy behavior, TLS roots, timeout layers, compression, and body limits

## Atomic goal

Define URL schemes, redirect policy, DNS rebinding controls, proxy behavior, TLS roots, timeout layers, compression, and body limits.

## Parent intent

Freeze the public subset, trust boundaries, defaults, and non-goals.

## Dependencies

- `M27-GATE` — `gates/M27-GATE.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M28.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`

### Source files

- `AGENTS.md`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-http/src/lib.rs`
- `crates/q-bridge/src/lib.rs`
- `crates/q-runtime/tests/runtime_conformance.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-pack/src/lib.rs`
- `Cargo.toml`
- `crates/q-runtime/src/main.rs`
- `docs/beta/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Define URL schemes, redirect policy, DNS rebinding controls, proxy behavior, TLS roots, timeout layers, compression, and body limits.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Security defaults fail closed.
- Private/link-local/metadata behavior is explicit.
- Redirect revalidation is required.
- Direct TLS policy is documented.

## Targeted commands

```bash
cargo test -p q-pack
```
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

- ADR.
- Threat model.
- Security test matrix.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m28-001-a: define url schemes redirect policy dns rebinding controls pr
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M28-001-A) — PASS

- Date: 2026-08-28
- Branch/PR: m28-001-a (squash-merged; see git log for final hash)
- Closes: #306

### Changed files
- `docs/okf/decisions/0033-native-fetch-security-policy.md` (new): **ADR-0033** — freezes the outbound-fetch trust boundary: URL scheme closed allowlist (http/https only, fail closed), deny-by-default SSRF address classification (private/loopback/link-local/metadata/unspecified/multicast/reserved), DNS-rebinding controls (validate-after-resolve — every A/AAAA must pass — connect-to-validated-address), per-hop redirect revalidation with downgrade + hop-bound + cross-origin credential rules, no ambient proxy trust (environment proxies never read), verified TLS roots (webpki-roots, TLS 1.2+, mandatory hostname validation, no invalid-cert escape hatch), layered timeouts (30 s default total, `MAX_FETCH_DEADLINE_MS` = 300 000 matching ADR-0030), bounded compression (gzip opt-in, decompression capped by body limit), body limits (16 MiB request/response, enforced while streaming). Includes a 12-row threat-model matrix mapping each threat to its mitigation section and an explicit non-goals list (no WebSocket/SSE, no client certs, no HTTP/2 upstream, no SOCKS/env-proxy).
- `crates/q-capabilities/src/fetch_policy.rs` (new): the machine-checkable policy object `FetchPolicy` consuming later M28 packets — `AddressClass` classifier (IPv4-mapped normalization so mapped forms cannot evade), `TrustMode` (default deny; auditable `trusted_loopback_explicit` for local tests that still blocks private/metadata), `RedirectPolicy::{Manual, Follow}` with construction-time hop validation, `TimeoutPolicy` with fail-closed `validate_timeouts`, `CompressionPolicy`, body-limit validation, and typed `FetchPolicyError` naming every denial.
- `crates/q-capabilities/src/lib.rs`: `pub mod fetch_policy;` + re-exports.
- `docs/okf/decisions/index.md`: added ADR-0032 (previously missing from the index after M27-009-D) and ADR-0033 entries.

### Security test matrix (crates/q-capabilities/src/fetch_policy.rs, 20 tests)
- Schemes: `http_and_https_are_the_only_allowed_schemes`, `dangerous_schemes_fail_closed` (file/data/ftp/ws/wss/gopher/unix/empty)
- SSRF: `public_addresses_are_dialable_by_default`, `private_ranges_are_denied_by_default`, `loopback_linklocal_and_metadata_are_denied_with_named_classes`, `unspecified_multicast_and_reserved_are_denied`, `ipv4_mapped_ipv6_cannot_evade_classification`, `explicit_loopback_mode_is_auditable_and_still_blocks_private`
- Rebinding: `one_bad_resolved_record_fails_the_whole_fetch` (mixed public+private resolution rejected), `empty_resolution_fails_closed`
- Redirects: `redirect_revalidation_rejects_scheme_and_downgrade_and_hops`, `manual_policy_follows_nothing`, `invalid_hop_limits_fail_at_construction`
- Proxy: `ambient_proxy_trust_is_never_enabled`
- Timeouts: `default_timeouts_are_bounded_and_valid`, `zero_or_over_ceiling_deadlines_fail_closed`, `connect_and_tls_timeouts_cannot_exceed_total_budget`
- Compression/bodies: `compression_is_off_by_default_and_bounded_decompression_is_pinned`, `body_limits_reject_zero_and_over_ceiling`
- Full validation: `default_policy_passes_full_validation`

### Command results
- `cargo test -p q-capabilities` → 127 unit tests + 7 integration passed (was 107+7; +20 security matrix)
- `cargo test -p q-pack` → 96+2 passed
- `cargo test -p q-engine-quickjs` → 16+97 passed
- `cargo test -p q-http` → 4+6+1 passed
- `cargo test -p q-bridge` → 11 passed
- `cargo test -p velqu-runtime` → 31 passed
- `bun test` → 213 pass / 0 fail; `bun run typecheck` → clean
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/validate-okf` → exit 0 (ADR-0033 accepted)
- `./scripts/verify` → **ALL PASS (exit 0)**

### Guardrail mapping
- **Security defaults fail closed** — every default constructor validates; scheme/address/deadline/limit violations are typed errors before any I/O.
- **Private/link-local/metadata behavior is explicit** — named `AddressClass` variants, distinct metadata deny list, auditable loopback override that still blocks private/metadata.
- **Redirect revalidation is required** — `check_redirect_hop` revalidates scheme + downgrade + hop on every hop; addresses re-resolved and re-classified per hop per ADR §4.
- **Direct TLS policy is documented** — ADR §6: webpki-roots, TLS 1.2+ minimum, mandatory hostname validation, no invalid-cert path anywhere in the policy surface.

### Disclosures
- Fixed in this packet: first `./scripts/verify` run failed `cargo clippy` (`if_same_then_else` in `fetch_policy.rs`); the manual gate run had omitted `-- -D warnings` and masked it. Removed the dead branch (IPv4-mapped normalization made it unreachable); all tests unchanged and green. Gate commands now include the flag verbatim.
- Standing: CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR. Local evidence above is complete.
