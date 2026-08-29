---
task_id: M28-010-Z
parent_task: M28-010
milestone: M28
priority: P0
mode: EVIDENCE
status: PASS
context_card: context/milestones/M28.md
commit_required: true
---

# M28-010-Z — Package evidence for Complete fetch conformance and fault testing

## Atomic goal

Create source-backed evidence and handoff for parent task M28-010; update status only if verification passed.

## Parent intent

Prove the beta subset across success and failure modes.

## Dependencies

- `M28-010-V` — `tasks/05_m28_native_fetch/M28-010-V-verify-complete-fetch-conformance-and-fault-testing.md`

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

- Documented subset passes.
- No panic/hang/unbounded work.
- All failures map predictably.
- Skips are explicit.

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

- Conformance report.
- Fixture inventory.
- Fuzz report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m28-010-z: package evidence for complete fetch conformance and fault te
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M28-010-Z) — PASS

- Date: 2026-08-29
- Branch/PR: m28-010-z (squash-merged; see git log for final hash)
- Closes: #365

### Parent closure — M28-010 Complete fetch conformance and fault testing

Parent intent: prove the beta subset across success and failure modes. Status: **PASS**.

Packet commits (squash merges):
- M28-010-A — 71a149f (#963, Closes #360): WPT manifest v1.2.0 — 3 new executable policy subsets (redirect 7, egress 9, decompression 5 = 21 vectors; total 79), 3 new explicit skips, regenerated report, manifest-driven Rust executor asserting exact typed variants
- M28-010-B — 9d1eca0 (#964, Closes #361): deterministic fixture library (`fetch_fixtures`): canned/exactly-once DNS resolvers, real 302 redirect chains, slow-body dribble server, untrusted TLS endpoints; 6 executor-shape tests, 0.5s wall, hermetic
- M28-010-C — 4b5552a (#965, Closes #362): property fuzzing — 7 properties x 512 iterations asserting security invariants beyond no-panic (credential stripping, dialable pins, hop ceiling, decompression bounds, scheme exactness); fixed a keep-alive parsing race in the M28-010-B mock server found by the full-suite run
- M28-010-D — 2c12c92 (#966, Closes #363): proxy isolation (poisoned ambient env vars, zero hits on the poison listener, direct dial succeeds) + mid-body cancellation (capacity freed within 2s, post-cancel service proven)
- M28-010-V — 3d4be3b (#967, Closes #364): verification closure mapping all 4 acceptance guardrails to the evidence

### Required evidence
- **Conformance report**: `docs/reports/m27-010-wpt-wintertc-conformance.md` at manifest v1.2.0 — 79 pinned vectors (100% PASS) + 23 explicit skips (vocabulary-enforced).
- **Fixture inventory**: `crates/q-runtime/tests/fetch_fixtures/mod.rs` + 8 executor-shape tests in 2 conformance files (redirect chains, slow bodies, untrusted TLS, proxy isolation, cancellation).
- **Fuzz report**: `crates/q-capabilities/tests/fuzz_fetch_inputs.rs` — 7 properties x 512 deterministic iterations (3,584 executions); invariant assertions beyond no-panic; one real defect found and fixed (keep-alive race).

### Source/test map
- `conformance/web-api/wpt-manifest.json` (v1.2.0) + `web-api.conformance.test.ts` (TS executor + closed skip vocabulary)
- `crates/q-capabilities/tests/wpt_wintertc_conformance.rs` (Rust manifest executors: 27 total vectors across both)
- `crates/q-capabilities/tests/fuzz_fetch_inputs.rs` (7 properties)
- `crates/q-runtime/tests/fetch_fixtures/mod.rs`, `fetch_fixture_conformance.rs`, `fetch_proxy_cancellation_conformance.rs` (8 tests)
- `crates/q-capabilities/src/fetch_policy.rs`: `with_redirect_policy` builder (A)
- Binary `b8296060…` matches manifest (B's workspace tokio io-util feature; C/D test-only)

### Command results (this branch)
- `cargo test -p q-capabilities` → 192 unit + 7 fuzz + 1 + 4 backpressure + 9 WPT-manifest passed
- `cargo test -p velqu-runtime` → 12 unit + 5 + 44 integration passed
- `cargo test -p q-engine-quickjs` → 18+101; `-p q-http` → 4+6+1; `-p q-bridge` → 11 passed
- `bun test` → 0 fail; `bun run typecheck` → clean (via ./scripts/verify)
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**

### Ledger update
- `docs/beta/04_TASK_LEDGER.md`: M28-010 flipped TODO -> PASS.

### Disclosures (standing)
- CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR. Local evidence above is complete.
