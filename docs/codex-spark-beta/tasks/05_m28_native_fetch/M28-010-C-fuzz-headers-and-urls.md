---
task_id: M28-010-C
parent_task: M28-010
milestone: M28
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M28.md
commit_required: true
---

# M28-010-C — Fuzz headers and URLs

## Atomic goal

Fuzz headers and URLs.

## Parent intent

Prove the beta subset across success and failure modes.

## Dependencies

- `M28-010-B` — `tasks/05_m28_native_fetch/M28-010-B-create-deterministic-dns-tls-redirect-slow-body-fixtures.md`

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
- `conformance/security/security.conformance.test.ts`
- `crates/q-pack/tests/fuzz_pack.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Fuzz headers and URLs.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

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

## Required evidence for this microtask

- Conformance report.
- Fixture inventory.
- Fuzz report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m28-010-c: fuzz headers and urls
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M28-010-C) — PASS

- Date: 2026-08-29
- Branch/PR: m28-010-c (squash-merged; see git log for final hash)
- Closes: #362

### Changed files
- `crates/q-capabilities/tests/fuzz_fetch_inputs.rs` (new): deterministic property-based fuzzing over fetch inputs (xorshift PRNG, no external deps; 7 tests x 512 iterations = 3,584 fuzz executions per run) —
  - `fuzzed_urls_never_panic_and_malformed_never_pass_scheme_gate` (parse Ok/Err only; scheme gate exactness)
  - `fuzzed_header_names_never_leak_credentials_cross_origin` (survival == non-credential for ARBITRARY names; same-origin keeps everything; port-443 equivalence)
  - `credential_set_is_exhaustively_stripped_regardless_of_fuzzed_origins` (each closed-set header stripped on every cross-origin origin-pair)
  - `fuzzed_hosts_never_panic_the_egress_gate_and_ok_implies_dialable` (name gate + full gate; every pinned address provably dialable; config bounded at MAX_EGRESS_HOST_ENTRIES)
  - `fuzzed_redirect_sequences_stay_bounded_and_typed` (fuzzed walks never exceed the ceiling; TooManyRedirects only at exactly the ceiling)
  - `fuzzed_decompression_sequences_never_exceed_the_bounds` (accepted output never exceeds cap or the ratio ceiling past threshold, per-step)
  - `fuzzed_helper_sizes_fail_closed_monotonically` (boundary exactness per helper)
- `crates/q-runtime/tests/fetch_fixtures/mod.rs` (from M28-010-B): mock redirect server keep-alive parsing fixed — the request accumulation buffer is now reset per request (the stale-buffer race produced a rare ConnectionReset in the redirect-chain test); restructured to clippy's never_loop-compliant single-connection accept (the fixture serves one keep-alive connection by design).

### Command results
- `cargo test -p q-capabilities` → **192 unit + 7 fuzz + 1 helper-sizes + 4 backpressure + 9 WPT-manifest** — 0 failed
- `cargo test -p velqu-runtime` → 11+5+36 — all pass; fixture suite 6/6 across 6 consecutive runs (flake fixed)
- `cargo test -p q-engine-quickjs` → 18+101 · `-p q-http` 4+6+1 · `-p q-bridge` 11 — all pass
- `bun test` → 0 fail; `bun run typecheck` → clean (via ./scripts/verify)
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**; release binary unchanged (`b8296060…` matches manifest)

### Guardrail mapping
- **No panic/hang/unbounded work** — fuzz properties assert boundedness structurally: hops ceiling, decompression caps, dialable-only pins, scheme exactness — over thousands of adversarial inputs.
- **All failures map predictably** — every fuzzed failure path is a typed variant (the property matchers enumerate the closed error sets).

### Disclosures
- The M28-010-B mock server had a keep-alive parsing race (stale accumulation buffer) that produced a rare ConnectionReset; found by verify's full-suite run in THIS packet, root-caused, fixed, and validated 6x stable. Clippy's never_loop then drove the single-connection-accept restructure (clippy was right: the loop never iterated).
- Standing: CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR.
