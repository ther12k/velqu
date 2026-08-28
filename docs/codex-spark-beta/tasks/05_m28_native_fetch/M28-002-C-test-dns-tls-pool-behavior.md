---
task_id: M28-002-C
parent_task: M28-002
milestone: M28
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M28.md
commit_required: true
---

# M28-002-C — Test DNS/TLS/pool behavior

## Atomic goal

Test DNS/TLS/pool behavior.

## Parent intent

Choose a maintainable implementation based on cold start, size, correctness, streaming, and pooling.

## Dependencies

- `M28-002-B` — `tasks/05_m28_native_fetch/M28-002-B-measure-dependency-binary-startup-cost.md`

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
- `packages/cli/src/index.ts`
- `crates/q-runtime/src/source_map.rs`
- `examples/proof/`
- `README.md`
- `docs/beta/`
- `benchmarks/harness/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Test DNS/TLS/pool behavior.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Decision is evidence-backed.
- No framework benchmark alone determines choice.
- Selected stack supports cancellation/backpressure.
- Fallback strategy documented.

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
```bash
bun test
```
```bash
bun run typecheck
```

## Required evidence for this microtask

- Spike report.
- Raw measurements.
- Decision record.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m28-002-c: test dns tls pool behavior
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M28-002-C) — PASS

- Date: 2026-08-28
- Branch/PR: m28-002-c (squash-merged; see git log for final hash)
- Closes: #314

### Changed files
- `benchmarks/stack-spike/spike-hyper/tests/stack_behavior.rs` (new): 6 behavioral probes of the selected stack against local mock origins.
- `benchmarks/stack-spike/spike-hyper/Cargo.toml`: spike-only dev-deps (rcgen 0.13, tokio-rustls 0.26 ring-only, rustls 0.23 ring/no-defaults, rustls-pki-types 1) for the live TLS server; production graph untouched.
- `docs/reports/m28-002-c-dns-tls-pool-behavior.md` (new): results matrix + findings for M28-003/006/008.

### Probe results — 6/6 PASS
1. `pool_reuses_connection_for_sequential_same_origin_requests` — 2 sequential requests, exactly 1 TCP accept (keepalive reuse = the thing M28-003 must bound).
2. `pool_dials_a_separate_connection_per_origin` — distinct origins, distinct connections.
3. `dns_hostname_resolution_reaches_loopback_origin` — hostname requests resolve via system resolver (the path M28-008-A wraps with validate-after-resolve).
4. `dns_unresolvable_host_fails_typed_and_fast` — `.invalid` fails typed in < 10 s, no hang.
5. `tls_self_signed_certificate_is_rejected_fail_closed` — live rustls server with a self-signed cert is REJECTED by the webpki-roots-only client, fast; root validation mandatory, **no bypass knob** (ADR-0033 §6 verified end-to-end).
6. `streaming_body_supports_bounded_prefix_and_early_drop` — 1 MiB streamed body read as bounded frames, dropped mid-stream; server observes cancel (M28-006 semantics).

### Findings for M28-003+ (in the report)
- Pool reuse is real but unbounded by default — M28-003-B must set idle/per-host/total bounds explicitly.
- TLS rejection is structural (different builder = only path to a non-webpki CA; policy layer never constructs it).
- DNS is opaque inside the default connector — M28-008-A must replace the resolve step with a custom resolver so every address passes ADR-0033 §2 classification before dial (§3 connect-to-validated).
- Immediate-close EOF during TLS handshake is non-deterministic in the legacy connector (probe dropped; deterministic transport-error coverage lands with M28-003/M28-006 against the real pool).

### Command results
- `cargo test --test stack_behavior` (spike workspace) → 6 passed / 0 failed
- `cargo test -p q-engine-quickjs` 16+97 · `-p q-http` 4+6+1 · `-p q-capabilities` 132+8 · `-p velqu-runtime` 1+31 — all pass
- `bun test` → 215 pass / 0 fail; `bun run typecheck` → clean
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**

### Guardrail mapping
- **Decision is evidence-backed** — behavioral matrix + findings.
- **No framework benchmark alone determines choice** — behavior (pool/TLS/DNS/streaming) verified directly, not assumed.
- **Selected stack supports cancellation/backpressure** — proven by probe 6.
- **Fallback strategy documented** — unchanged (M28-002-A); no fallback trigger found.

### Disclosures (standing)
- CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR. Local evidence above is complete.
