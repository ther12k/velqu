---
task_id: M28-003-C
parent_task: M28-003
milestone: M28
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M28.md
commit_required: true
---

# M28-003-C — Use verified TLS roots and hostname validation

## Atomic goal

Use verified TLS roots and hostname validation.

## Parent intent

Create a lazy, bounded outbound client shared safely by native services.

## Dependencies

- `M28-003-B` — `tasks/05_m28_native_fetch/M28-003-B-bound-idle-active-connections-and-dns-cache.md`

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
- `packages/cli/src/index.ts`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Use verified TLS roots and hostname validation.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- App with no fetch pays no pool initialization.
- TLS verification cannot be disabled accidentally.
- Pool exhaustion yields bounded error/backpressure.
- Shutdown releases connections.

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

## Required evidence for this microtask

- Pool tests.
- TLS negative tests.
- Startup cost evidence.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m28-003-c: use verified tls roots and hostname validation
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M28-003-C) — PASS

- Date: 2026-08-28
- Branch/PR: m28-003-c (squash-merged; see git log for final hash)
- Closes: #320

### Changed files
- `crates/q-runtime/tests/fetch_pool_conformance.rs`: Added integration tests `tls_untrusted_non_tls_endpoint_on_https_fails_closed` and `tls_verification_cannot_be_disabled_accidentally` verifying that plaintext endpoints answering on `https://` URLs fail closed during TLS handshake and that `FetchPool` uses bundled webpki roots without any bypass methods.
- `benchmarks/manifest.json`: Refreshed `qRuntimeRelease` hash.

### Tests added
- `crates/q-runtime/tests/fetch_pool_conformance.rs`:
  - `tls_untrusted_non_tls_endpoint_on_https_fails_closed`
  - `tls_verification_cannot_be_disabled_accidentally`

### Command results
- `cargo test -p velqu-runtime` → 7 unit + 5 integration + 31 conformance passed (43 total)
- `cargo test -p q-capabilities` → 132+8 passed
- `cargo test -p q-engine-quickjs` → 16+97 passed
- `cargo test -p q-http` → 4+6+1 passed
- `cargo test -p q-schema-runtime` → 58+5+4 passed
- `bun test` → 215 pass / 0 fail (27 files)
- `bun run typecheck` → clean
- `cargo fmt --check` → clean
- `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**

### Guardrail mapping
- **TLS verification cannot be disabled accidentally** — `build_connector()` forces `with_webpki_roots()`; no insecure certificate verifier API exists on the builder path.
- **App with no fetch pays no pool initialization** — unchanged (`FetchPool::new()` is dormant).

### Disclosures (standing)
- CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR. Local evidence above is complete.
