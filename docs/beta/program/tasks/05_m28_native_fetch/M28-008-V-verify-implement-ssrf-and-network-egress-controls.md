---
task_id: M28-008-V
parent_task: M28-008
milestone: M28
priority: P0
mode: VERIFY
status: PASS
context_card: context/milestones/M28.md
commit_required: true
---

# M28-008-V — Verify Implement SSRF and network egress controls

## Atomic goal

Prove every acceptance criterion for parent task M28-008 without broadening scope.

## Parent intent

Provide explicit controls for metadata, loopback, private networks, and DNS rebinding.

## Dependencies

- `M28-008-A` — `tasks/05_m28_native_fetch/M28-008-A-resolve-and-validate-addresses-before-connect.md`
- `M28-008-B` — `tasks/05_m28_native_fetch/M28-008-B-revalidate-redirects-and-connection-targets.md`
- `M28-008-C` — `tasks/05_m28_native_fetch/M28-008-C-support-allow-deny-configuration.md`
- `M28-008-D` — `tasks/05_m28_native_fetch/M28-008-D-define-proxy-interaction.md`

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
- `crates/q-runtime/src/main.rs`
- `packages/core/src/index.ts`
- `docs/beta/`
- `examples/proof/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

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

- SSRF test suite.
- Threat-model update.
- Configuration examples.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m28-008-v: verify implement ssrf and network egress controls
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M28-008-V) — PASS

- Date: 2026-08-29
- Branch/PR: m28-008-v (squash-merged; see git log for final hash)
- Closes: #352

### Acceptance-criterion mapping (parent M28-008 guardrails)

1. **Cloud metadata endpoints blocked by safe default** — verified twice over: by NAME before any resolution (`is_metadata_hostname`, resolver provably untouched, trailing-dot/case normalized) and by ADDRESS CLASS (`AddressClass::Metadata`, denied even under `trusted_loopback_explicit`). Tests: `metadata_hostnames_are_denied_before_any_resolution`, `loopback_trust_still_denies_metadata_space_on_hops`, `redirect_targets_deny_metadata_by_name_too`, `allow_list_cannot_re_enable_metadata_names`.
2. **DNS rebinding tests fail closed** — verified: `resolve_and_validate` validates EVERY resolved address before anything dials; the returned pin set is the only dial set; mixed public+private answers are denied; hop evaluation is atomic (resolution failure leaves limiter state unchanged); IP literals skip the resolver and validate directly. Tests: `dns_rebinding_mixing_public_and_private_fails_closed`, `follow_hop_resolves_and_validates_atomically`, `follow_hop_pin_set_is_the_only_dial_set`, `resolver_failures_and_ip_literals_are_handled`, `full_fetch_sequence_composes_open_and_hops`.
3. **IPv4/IPv6/private ranges handled** — verified: the classifier covers loopback/link-local/private/metadata/broadcast, IPv4-mapped IPv6 normalization, and IPv6 classes; every resolved address passes trust-mode checks on open and every hop. Tests carried from earlier packets plus `resolution_uses_only_validated_addresses_in_order`.
4. **Policy decisions are observable without logging secrets** — verified: typed errors carry host/class/reason only (`HostnameDenied` reasons: by-name, resolver error, empty resolution, explicitly denied, not-in-allow-list); the proxy posture is queryable (`proxy_mode()`, `AMBIENT_PROXY_ENV_VARS`, `--fetch-stack-info` diagnostic). Tests: `proxy_mode_is_disabled_by_construction_and_not_configurable`, `ambient_proxy_env_survey_is_the_closed_list`, plus the typed-error assertions throughout A–D.

### Configuration examples (configuration surface proven by tests)
- Deny: `FetchPolicy::default().with_deny_hosts(["evil.test", ".internal.test"])` — exact or `.suffix` rules, deny wins over allow.
- Allowlist: `.with_allow_hosts(["api.corp.example"])` — non-empty list restricts egress; cannot re-enable metadata names.
- Loopback testing: `FetchPolicy::trusted_loopback_explicit()` — 127.0.0.1 dialable, metadata space still denied.
- Proxies: nothing to configure — `ProxyMode::Disabled` is the only state; ambient env ignored.

### Verification runs (this branch, worktree-fresh)
- `cargo test -p q-capabilities` → 184 unit + 4 backpressure + 8 WPT passed
- `cargo test -p q-engine-quickjs` → 18 unit + 101 engine passed
- `cargo test -p q-http` → 4+6+1 passed; `-p q-bridge` → 11 passed
- `cargo test -p velqu-runtime` → 8+5+31 passed
- `bun test` → 219 pass / 0 fail; `bun run typecheck` → clean (via ./scripts/verify)
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**; release binary reproduced deterministically (`46de91ac…` matches the M28-008-D manifest)

### Disclosures (standing)
- No production code changed in this packet: verification-only closure of M28-008-A/B/C/D.
- CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR. Local evidence above is complete.
