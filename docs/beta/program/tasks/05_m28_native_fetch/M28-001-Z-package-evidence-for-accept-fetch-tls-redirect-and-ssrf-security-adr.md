---
task_id: M28-001-Z
parent_task: M28-001
milestone: M28
priority: P0
mode: EVIDENCE
status: PASS
context_card: context/milestones/M28.md
commit_required: true
---

# M28-001-Z — Package evidence for Accept fetch, TLS, redirect, and SSRF security ADR

## Atomic goal

Create source-backed evidence and handoff for parent task M28-001; update status only if verification passed.

## Parent intent

Freeze the public subset, trust boundaries, defaults, and non-goals.

## Dependencies

- `M28-001-V` — `tasks/05_m28_native_fetch/M28-001-V-verify-accept-fetch-tls-redirect-and-ssrf-security-adr.md`

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

- ADR.
- Threat model.
- Security test matrix.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m28-001-z: package evidence for accept fetch tls redirect and ssrf secu
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M28-001-Z) — PASS

- Date: 2026-08-28
- Branch/PR: m28-001-z (squash-merged; see git log for final hash)
- Closes: #311

### Parent closure — M28-001 Accept fetch, TLS, redirect, and SSRF security ADR

Parent intent: freeze the public subset, trust boundaries, defaults, and non-goals. Status: **PASS**.

Packet commits (squash merges):
- M28-001-A — b56f83b (#909, Closes #306): ADR-0033 + `q_capabilities::fetch_policy` (schemes, SSRF classification, rebinding controls, redirects, proxy, TLS, timeouts, compression, bodies; 20-test security matrix; also added the missing ADR-0032 index entry)
- M28-001-B — df5583f (#910, Closes #307): ADR-0034 (fetch = declared capability `runtime:fetch@1`, runtime-owned outbound trust, forwarded headers never identity, loopback bind default, Host never routes; 4 tests)
- M28-001-C — c295772 (#911, Closes #308): manifest v1.1.0 — fetch capability with 24 executable policy vectors + 12 explicit skips (`NO_WEBSOCKET_BETA` … `NO_BR_ZSTD`); Rust manifest-execution test; TS manifest checks; report regenerated (58 vectors + 20 skips)
- M28-001-D — ba698d2 (#912, Closes #309): ADR-0035 same-process trusted-code assumption; `TRUSTED_CODE_ASSUMPTION` pinned + test; crate-root trust-model rustdoc
- M28-001-V — e8d9888 (#913, Closes #310): verification closure mapping all four guardrails to source + tests

### Evidence ledger (required microtask evidence)
- **ADR**: `docs/okf/decisions/0033-native-fetch-security-policy.md`, `0034-reverse-proxy-and-outbound-trust.md`, `0035-same-process-trusted-code-assumption.md` — all accepted, indexed, validate-okf clean.
- **Threat model**: ADR-0033 (12 rows), ADR-0034 (6 rows), ADR-0035 (6 rows).
- **Security test matrix**: 25 policy unit tests + manifest-execution integration test (`fetch_policy_manifest_vectors_execute_against_compiled_policy`) + TS manifest structure/non-goal checks; pinned in `conformance/web-api/wpt-manifest.json` v1.1.0.

### Command results (this branch)
- `cargo test -p q-capabilities` → 132 unit + 8 integration passed
- `cargo test -p q-engine-quickjs` 16+97 · `-p q-http` 4+6+1 · `-p q-schema-runtime` 58+5+4 · `-p q-pack` 96+2 — all pass
- `cargo test -p velqu-runtime` → 31 passed (see disclosure)
- `bun test` → 215 pass / 0 fail; `bun run typecheck` → clean
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)** (includes full workspace tests + conformance-report drift check)

### Ledger update
- `docs/beta/04_TASK_LEDGER.md`: M28-001 flipped TODO -> PASS.

### Disclosures
- Transient test failure: the first targeted `cargo test -p velqu-runtime` run on this branch reported 1 failure (30/31); four consecutive re-runs passed 31/31 and the full `./scripts/verify` workspace run (same suite) passed twice. Same port-race pattern previously observed and documented in M27-002-Z; no test or fixture was modified.
- Standing: CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR. Local evidence above is complete.
