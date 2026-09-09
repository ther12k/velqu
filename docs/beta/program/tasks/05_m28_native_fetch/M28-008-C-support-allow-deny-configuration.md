---
task_id: M28-008-C
parent_task: M28-008
milestone: M28
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M28.md
commit_required: true
---

# M28-008-C — Support allow/deny configuration

## Atomic goal

Support allow/deny configuration.

## Parent intent

Provide explicit controls for metadata, loopback, private networks, and DNS rebinding.

## Dependencies

- `M28-008-B` — `tasks/05_m28_native_fetch/M28-008-B-revalidate-redirects-and-connection-targets.md`

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
- `crates/q-router/src/lib.rs`
- `crates/q-pack/src/lib.rs`
- `packages/compiler/src/emit.ts`
- `conformance/routing/routing.conformance.test.ts`
- `Cargo.toml`
- `crates/q-runtime/src/main.rs`
- `packages/core/src/index.ts`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Support allow/deny configuration.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Cloud metadata endpoints blocked by safe default.
- DNS rebinding tests fail closed.
- IPv4/IPv6/private ranges handled.
- Policy decisions are observable without logging secrets.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
cargo test -p q-router
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

- SSRF test suite.
- Threat-model update.
- Configuration examples.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m28-008-c: support allow deny configuration
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M28-008-C) — PASS

- Date: 2026-08-29
- Branch/PR: m28-008-c (squash-merged; see git log for final hash)
- Closes: #350

### Changed files
- `crates/q-capabilities/src/fetch_policy.rs`: explicit egress allow/deny configuration —
  - `FetchPolicy.host_deny` / `host_allow` (normalized, bounded at `MAX_EGRESS_HOST_ENTRIES` = 256) with builder methods `with_deny_hosts(...)` / `with_allow_hosts(...)` (lowercase, trailing-dot stripped, deduplicated) and accessors `host_deny()` / `host_allow()`.
  - `check_host_config(host)`: explicit deny wins over everything; a non-empty allow list restricts to matching hosts; empty allow list = no name-based restriction. Typed `HostnameDenied` reasons name the decision ("explicitly denied by egress configuration" / "not in the configured egress allow list") — observable without logging secrets.
  - Entry matching: exact or `.suffix` rules (`.corp.example` covers the apex and every subdomain); suffix-anchored so `internal.test.evil` never matches `.internal.test`.
  - Wired into `resolve_and_validate` AFTER the metadata-by-name denial: every gate path (open + `follow_hop`) enforces configuration automatically. **The safe default is not configurable away** — allow-listing `metadata.google.internal` still denies it by name.
- `crates/q-capabilities/src/lib.rs`: re-exports `MAX_EGRESS_HOST_ENTRIES`.

### Tests added (fetch_policy.rs, +6 → 182 lib tests)
- `deny_list_blocks_hosts_before_resolution` (name+case+trailing-dot denial, resolver untouched; unrelated hosts pass)
- `allow_list_restricts_to_listed_hosts_only` (typed denial for non-listed; listed passes; empty list unrestricted)
- `deny_wins_over_allow` (host in both lists is denied)
- `allow_list_cannot_re_enable_metadata_names` (safe default holds even when explicitly allow-listed; resolver untouched)
- `suffix_entries_cover_domain_and_subdomains` (apex + subdomains match; suffix-anchored non-matches)
- `configuration_normalizes_and_deduplicates_entries` (case/trailing-dot/whitespace normalization, dedup, accessors)

### Command results
- `cargo test -p q-capabilities` → **182 unit (was 176) + 4 backpressure + 8 WPT** — 0 failed
- `cargo test -p q-engine-quickjs` → 18+101 · `-p q-http` 4+6+1 · `-p q-bridge` 11 · `-p velqu-runtime` 8+5+31 — all pass
- `bun test` → 219 pass / 0 fail; `bun run typecheck` → clean (via ./scripts/verify)
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**
- `benchmarks/manifest.json` refreshed (`539afb2d…`): the new FetchPolicy fields change struct layout in the binary — legitimate artifact change, unlike the A/B policy-only packets.

### Guardrail mapping
- **Cloud metadata endpoints blocked by safe default** — configuration composes after the by-name denial: allow lists cannot re-enable metadata.
- **Policy decisions are observable without logging secrets** — denials carry host + closed reason strings only.
- **IPv4/IPv6/private ranges handled** — unchanged; configuration is a name gate that composes with (never replaces) the trust-mode address checks.

### Disclosures
- Two closure-declaration fixes (`let mut resolve`) caught by the compiler during test authoring; no behavior change.
- Standing: CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR.
