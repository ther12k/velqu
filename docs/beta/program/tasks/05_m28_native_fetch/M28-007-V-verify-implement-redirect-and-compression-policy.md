---
task_id: M28-007-V
parent_task: M28-007
milestone: M28
priority: P1
mode: VERIFY
status: PASS
context_card: context/milestones/M28.md
commit_required: true
---

# M28-007-V — Verify Implement redirect and compression policy

## Atomic goal

Prove every acceptance criterion for parent task M28-007 without broadening scope.

## Parent intent

Handle redirects and encoded responses safely and predictably.

## Dependencies

- `M28-007-A` — `tasks/05_m28_native_fetch/M28-007-A-limit-redirect-count.md`
- `M28-007-B` — `tasks/05_m28_native_fetch/M28-007-B-reapply-ssrf-dns-policy-on-every-hop.md`
- `M28-007-C` — `tasks/05_m28_native_fetch/M28-007-C-define-credential-header-stripping.md`
- `M28-007-D` — `tasks/05_m28_native_fetch/M28-007-D-bound-decompression-ratio-and-output.md`

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

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- Redirect loops fail boundedly.
- Sensitive headers never leak cross-origin.
- Zip-bomb style expansion is limited.
- Observed URL/status follows documented semantics.

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

- Security fixtures.
- Compression limits tests.
- Redirect conformance.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m28-007-v: verify implement redirect and compression policy
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M28-007-V) — PASS

- Date: 2026-08-29
- Branch/PR: m28-007-v (squash-merged; see git log for final hash)
- Closes: #346

### Acceptance-criterion mapping (parent M28-007 guardrails)

1. **Redirect loops fail boundedly** — verified: hop ceiling (`TooManyRedirects` at the policy `max_hops`, default `MAX_REDIRECT_HOPS` = 20) as the hard backstop plus typed `RedirectLoop` fast-path over the bounded visited set. Tests: `redirect_limiter_follows_up_to_ceiling_then_fails_typed`, `redirect_loop_fails_fast_and_typed`, `custom_hop_limit_is_respected_exactly`, `empty_resolution_and_ceiling_precede_address_checks` (ceiling fires before DNS work).
2. **Sensitive headers never leak cross-origin** — verified: closed `CREDENTIAL_REDIRECT_HEADERS` set stripped on every origin change (scheme/host/effective-port), same-origin keeps them, malformed URLs fail closed to stripping. Tests: `cross_origin_hops_strip_credential_headers`, `same_origin_hops_keep_credentials`, `credential_header_detection_is_case_insensitive_and_closed`, `malformed_redirect_targets_fail_closed_to_stripping`.
3. **Zip-bomb style expansion is limited** — verified: dual bound — output cap at the response body limit and 1000:1 ratio ceiling past the 1 KiB input threshold, per-step push accounting where a failed step accepts no bytes. Tests: `decompression_output_is_capped_typed`, `zip_bomb_ratio_is_bounded_typed`, `small_payloads_are_not_ratio_limited_below_threshold`, `bomb_fixture_output_cap_fires_before_ratio_when_tighter`, `guard_from_policy_matches_compression_posture`.
4. **Observed URL/status follows documented semantics** — verified: every hop (URL checks, SSRF/DNS revalidation) goes through the ONE policy path (`FetchPolicy::check_redirect_hop` + `check_resolved`); `Manual` policy surfaces the 3xx with zero hop consumption; denied hops leave limiter state unchanged. Tests: `manual_policy_surfaces_3xx_without_following`, `ssrf_policy_is_reapplied_on_every_hop`, `redirect_target_resolving_partial_loopback_is_denied`, `loopback_trust_still_denies_metadata_space_on_hops`, `scheme_and_downgrade_denials_flow_through_limiter`.

### Verification runs (this branch, worktree-fresh)
- `cargo test -p q-capabilities` → 167 unit + 4 backpressure + 8 WPT passed
- `cargo test -p q-engine-quickjs` → 18 unit + 101 engine passed
- `cargo test -p q-http` → 4+6+1 passed; `-p q-bridge` → 11 passed
- `cargo test -p velqu-runtime` → 8+5+31 passed
- `bun test` → 219 pass / 0 fail; `bun run typecheck` → clean (via ./scripts/verify)
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**; release binary reproduced deterministically (`ef142331…` matches the manifest refreshed in M28-006-D — M28-007 policy additions are dead-code-eliminated until the executor wires in)

### Disclosures (standing)
- No production code changed in this packet: verification-only closure of M28-007-A/B/C/D.
- CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR. Local evidence above is complete.
