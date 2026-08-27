---
task_id: M28-001-B
parent_task: M28-001
milestone: M28
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M28.md
commit_required: true
---

# M28-001-B — Specify reverse-proxy and outbound trust

## Atomic goal

Specify reverse-proxy and outbound trust.

## Parent intent

Freeze the public subset, trust boundaries, defaults, and non-goals.

## Dependencies

- `M28-001-A` — `tasks/05_m28_native_fetch/M28-001-A-define-url-schemes-redirect-policy-dns-rebinding-controls-proxy-behavior-tls-roo.md`

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
- `crates/q-runtime/src/main.rs`
- `docs/beta/`
- `examples/proof/`
- `conformance/security/security.conformance.test.ts`
- `crates/q-pack/tests/fuzz_pack.rs`
- `crates/q-http/tests/fuzz_parsers.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Specify reverse-proxy and outbound trust.
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
m28-001-b: specify reverse proxy and outbound trust
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M28-001-B) — PASS

- Date: 2026-08-28
- Branch/PR: m28-001-b (squash-merged; see git log for final hash)
- Closes: #307

### Changed files
- `docs/okf/decisions/0034-reverse-proxy-and-outbound-trust.md` (new): **ADR-0034** — freezes the two remaining M2.8 trust boundaries. Outbound: `fetch` is a declared capability `runtime:fetch@1` under the ADR-0029 identity system (per-route compiler grants, pruned inventory, absent-API fail closed — no ambient global), and outbound trust is runtime-owned (single `FetchPolicy::default()`, zero JS-facing widening surface in beta). Ingress: `X-Forwarded-For/Proto/Host/Port/All` and RFC 7239 `Forwarded` are never identity/authn/authz input (peer = the connection); TLS termination is an edge-proxy deployment concern; `--addr` defaults to loopback `127.0.0.1`; `Host` never selects a route. Includes a 6-row threat-model addition (spoofed forwarded headers, scheme confusion, direct exposure, host-header routing confusion, undeclared socket access, handler self-widening) and non-goals (no signed proxy identity, no runtime TLS termination in beta, no per-route policy overrides).
- `crates/q-capabilities/src/fetch_policy.rs`: module doc extended; added `FETCH_CAPABILITY_ID`/`FETCH_CAPABILITY_VERSION` (validated against `CapabilityId::parse`), `UNTRUSTED_FORWARD_HEADERS` closed list, and `is_untrusted_forward_header()` (case-insensitive).
- `crates/q-capabilities/src/lib.rs`: re-exports for the trust-model surface.
- `docs/okf/decisions/index.md`: ADR-0034 entry.

### Security test matrix (4 new tests, q-capabilities 131 total)
- `fetch_is_a_declared_capability_under_the_identity_system` — id parses under the closed `runtime:` namespace, version pinned.
- `forwarded_headers_are_never_trusted_identity` — all 6 forwarded forms flagged (case variants); ordinary headers not flagged.
- `outbound_trust_is_runtime_owned_not_application_owned` — default trust mode, no escape hatch, ambient proxy always off.
- `host_header_never_participates_in_policy_decisions` — host values flow only through the address policy; metadata IP rejected regardless of hostname.

### Command results
- `cargo test -p q-capabilities` → 131 unit + 7 integration passed (was 127+7)
- `cargo test -p q-pack` → 96+2; `-p q-engine-quickjs` → 16+97; `-p q-http` → 4+6+1; `-p velqu-runtime` → 31 — all pass
- `bun test` → 213 pass / 0 fail; `bun run typecheck` → clean
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/validate-okf` → exit 0 (ADR-0034 accepted)
- `./scripts/verify` → **ALL PASS (exit 0)**

### Guardrail mapping
- **Security defaults fail closed** — loopback bind default; forwarded headers distrusted by default; no ambient fetch global.
- **Private/link-local/metadata behavior is explicit** — unchanged from ADR-0033 §2; this packet adds no trust widening.
- **Redirect revalidation is required** — unchanged (ADR-0033 §4).
- **Direct TLS policy is documented** — ADR-0034 §4: runtime serves plain HTTP on the listener; TLS is edge-proxy responsibility; outbound TLS per ADR-0033 §6.

### Disclosures (standing)
- CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR. Local evidence above is complete.
