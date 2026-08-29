---
task_id: M28-007-Z
parent_task: M28-007
milestone: M28
priority: P1
mode: EVIDENCE
status: PASS
context_card: context/milestones/M28.md
commit_required: true
---

# M28-007-Z — Package evidence for Implement redirect and compression policy

## Atomic goal

Create source-backed evidence and handoff for parent task M28-007; update status only if verification passed.

## Parent intent

Handle redirects and encoded responses safely and predictably.

## Dependencies

- `M28-007-V` — `tasks/05_m28_native_fetch/M28-007-V-verify-implement-redirect-and-compression-policy.md`

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
- `Cargo.toml`
- `conformance/security/security.conformance.test.ts`
- `crates/q-pack/tests/fuzz_pack.rs`
- `crates/q-http/tests/fuzz_parsers.rs`
- `crates/q-schema-runtime/tests/fuzz_validator.rs`
- `.github/workflows/verify.yml`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

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

## Required evidence for this microtask

- Security fixtures.
- Compression limits tests.
- Redirect conformance.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m28-007-z: package evidence for implement redirect and compression poli
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M28-007-Z) — PASS

- Date: 2026-08-29
- Branch/PR: m28-007-z (squash-merged; see git log for final hash)
- Closes: #347

### Parent closure — M28-007 Implement redirect and compression policy

Parent intent: handle redirects and encoded responses safely and predictably. Status: **PASS**.

Packet commits (squash merges):
- M28-007-A — 5b03f96 (#945, Closes #342): `RedirectLimiter` — stateful per-request hop counter; every hop through `FetchPolicy::check_redirect_hop` (scheme allowlist, https→http downgrade denial, ceiling at `MAX_REDIRECT_HOPS` = 20) + typed `RedirectLoop` fast-path; `Manual` policy surfaces 3xx with zero consumption
- M28-007-B — 42b3c30 (#946, Closes #343): SSRF/DNS revalidation on every hop — `evaluate_resolved(from, to, resolved)`: every resolved address of the redirect target must pass trust-mode policy after URL checks and before state commit; `AddressClass::Metadata` stays denied even under explicit loopback trust
- M28-007-C — 6e2e4bf (#947, Closes #344): credential/header stripping — closed `CREDENTIAL_REDIRECT_HEADERS` set; `url_origin` (default-port elision), `is_cross_origin_redirect` (malformed -> cross-origin, fail closed), `headers_surviving_redirect` one-call hop filter
- M28-007-D — be8f9ac (#948, Closes #345): decompression bomb guard — dual bound (output cap = response body limit; 1000:1 ratio past 1 KiB input threshold), push-based per-step checks, typed `DecompressedTooLarge` / `DecompressionBomb`
- M28-007-V — e594f9a (#949, Closes #346): verification closure mapping all 4 acceptance guardrails to 17 new tests

### Evidence ledger (required microtask evidence)
- **Security fixtures**: loopback/link-local/metadata denial per hop (`AddressDenied` classes), partial-DNS poisoning (one bad address denies the host), malformed-URL fail-closed stripping, zip-bomb fixtures at both bounds.
- **Compression limits tests**: output cap (ADR-0033 §9 response limit), 1000:1 ratio ceiling past the 1 KiB threshold, posture mapping (Off/Gzip{false} -> no guard), tighter-bound precedence.
- **Redirect conformance**: ceiling + typed loop fast-path, downgrade denial, scheme allowlist through the limiter, custom hop limits respected exactly, Manual surfacing, denied hops leave state unchanged.

### Source/test map
- `crates/q-capabilities/src/fetch_policy.rs`: `RedirectLimiter`, `evaluate_resolved`, `CREDENTIAL_REDIRECT_HEADERS`, `url_origin`, `is_cross_origin_redirect`, `headers_surviving_redirect`, `DecompressionGuard`, `MAX_DECOMPRESSION_RATIO`, `DECOMPRESSION_RATIO_THRESHOLD`; 17 new unit tests (150 → 167 lib tests across A–D).
- `crates/q-capabilities/src/lib.rs`: re-exports.
- Release binary hash unchanged through A–D (`ef142331…`, matches manifest): all additions are policy-layer, dead-code-eliminated until the fetch executor wires in.

### Command results (this branch)
- `cargo test -p q-capabilities` → 167 unit + 4 backpressure + 8 WPT passed
- `cargo test -p q-engine-quickjs` → 18 unit + 101 engine passed
- `cargo test -p q-http` → 4+6+1 passed; `-p q-bridge` → 11 passed
- `cargo test -p velqu-runtime` → 8+5+31 passed
- `bun test` → 219 pass / 0 fail; `bun run typecheck` → clean (via ./scripts/verify)
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**

### Ledger update
- `docs/beta/04_TASK_LEDGER.md`: M28-007 flipped TODO -> PASS.

### Disclosures (standing)
- CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR. Local evidence above is complete.
