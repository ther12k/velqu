---
task_id: M28-001-V
parent_task: M28-001
milestone: M28
priority: P0
mode: VERIFY
status: PASS
context_card: context/milestones/M28.md
commit_required: true
---

# M28-001-V — Verify Accept fetch, TLS, redirect, and SSRF security ADR

## Atomic goal

Prove every acceptance criterion for parent task M28-001 without broadening scope.

## Parent intent

Freeze the public subset, trust boundaries, defaults, and non-goals.

## Dependencies

- `M28-001-A` — `tasks/05_m28_native_fetch/M28-001-A-define-url-schemes-redirect-policy-dns-rebinding-controls-proxy-behavior-tls-roo.md`
- `M28-001-B` — `tasks/05_m28_native_fetch/M28-001-B-specify-reverse-proxy-and-outbound-trust.md`
- `M28-001-C` — `tasks/05_m28_native_fetch/M28-001-C-define-unsupported-web-features.md`
- `M28-001-D` — `tasks/05_m28_native_fetch/M28-001-D-document-same-process-trusted-code-assumption.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M28.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`
- `context/components/schema-codecs.md`

### Source files

- `AGENTS.md`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-http/src/lib.rs`
- `crates/q-schema-runtime/src/lib.rs`
- `crates/q-schema-runtime/tests/fuzz_validator.rs`
- `crates/q-engine-quickjs/src/convert.rs`
- `packages/schema/src/index.ts`
- `conformance/schema/schema.conformance.test.ts`
- `Cargo.toml`
- `conformance/security/security.conformance.test.ts`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- Security defaults fail closed.
- Private/link-local/metadata behavior is explicit.
- Redirect revalidation is required.
- Direct TLS policy is documented.

## Targeted commands

```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p q-http
```
```bash
cargo test -p q-schema-runtime
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

- ADR.
- Threat model.
- Security test matrix.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m28-001-v: verify accept fetch tls redirect and ssrf security adr
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M28-001-V) — PASS

- Date: 2026-08-28
- Branch/PR: m28-001-v (squash-merged; see git log for final hash)
- Closes: #310

### Acceptance-criterion mapping (parent M28-001 guardrails)

1. **Security defaults fail closed** — ADR-0033 §1/§2/§7/§9 + `fetch_policy.rs`: `dangerous_schemes_fail_closed`, `private_ranges_are_denied_by_default`, `unspecified_multicast_and_reserved_are_denied`, `zero_or_over_ceiling_deadlines_fail_closed`, `body_limits_reject_zero_and_over_ceiling`, `default_policy_passes_full_validation`; ADR-0034: `outbound_trust_is_runtime_owned_not_application_owned`, `ambient_proxy_trust_is_never_enabled`; manifest-executed vectors: `fetch_policy_manifest_vectors_execute_against_compiled_policy` (24 vectors, both directions).
2. **Private/link-local/metadata behavior is explicit** — `loopback_linklocal_and_metadata_are_denied_with_named_classes` (incl. `169.254.169.254`, `fd00:ec2::235`), `ipv4_mapped_ipv6_cannot_evade_classification`, `explicit_loopback_mode_is_auditable_and_still_blocks_private`; DNS rebinding: `one_bad_resolved_record_fails_the_whole_fetch`, `empty_resolution_fails_closed`.
3. **Redirect revalidation is required** — `redirect_revalidation_rejects_scheme_and_downgrade_and_hops`, `manual_policy_follows_nothing`, `invalid_hop_limits_fail_at_construction` (ADR-0033 §4).
4. **Direct TLS policy is documented** — ADR-0033 §6 (webpki-roots, TLS 1.2+ minimum, mandatory hostname validation, no invalid-cert path) + ADR-0035 §3 (fetch policy is a network control, never isolation evidence); skips `NO_CLIENT_CERTS`/`NO_HTTP2_UPSTREAM` pinned in `wpt-manifest.json` fetch entry.

Trust-model coverage (ADR-0034/0035): `fetch_is_a_declared_capability_under_the_identity_system`, `forwarded_headers_are_never_trusted_identity`, `host_header_never_participates_in_policy_decisions`, `trusted_code_assumption_is_pinned`.

### Verification runs (this branch, worktree-fresh)
- `cargo test -p q-capabilities` → 132 unit + 8 integration passed
- `cargo test -p q-engine-quickjs` → 16+97 · `-p q-http` 4+6+1 · `-p q-schema-runtime` 58+5+4 · `-p velqu-runtime` 31 — all pass
- `bun test` → 215 pass / 0 fail (incl. fetch-manifest structure + frozen non-goals checks); `bun run typecheck` → clean
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `python3 scripts/generate-conformance-report.py --check` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**

### Evidence inventory
- ADRs: `docs/okf/decisions/0033-native-fetch-security-policy.md`, `0034-reverse-proxy-and-outbound-trust.md`, `0035-same-process-trusted-code-assumption.md` (all validate-okf accepted, indexed).
- Threat models: ADR-0033 §Threat model (12 rows), ADR-0034 §Threat model additions (6 rows), ADR-0035 §Threat model (6 rows).
- Security test matrix: 25 policy tests in `crates/q-capabilities/src/fetch_policy.rs` + manifest-execution test in `crates/q-capabilities/tests/wpt_wintertc_conformance.rs` + fetch-manifest TS checks in `conformance/web-api/web-api.conformance.test.ts`.

### Disclosures (standing)
- CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR. Local evidence above is complete.
