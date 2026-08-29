---
task_id: M28-008-Z
parent_task: M28-008
milestone: M28
priority: P0
mode: EVIDENCE
status: PASS
context_card: context/milestones/M28.md
commit_required: true
---

# M28-008-Z — Package evidence for Implement SSRF and network egress controls

## Atomic goal

Create source-backed evidence and handoff for parent task M28-008; update status only if verification passed.

## Parent intent

Provide explicit controls for metadata, loopback, private networks, and DNS rebinding.

## Dependencies

- `M28-008-V` — `tasks/05_m28_native_fetch/M28-008-V-verify-implement-ssrf-and-network-egress-controls.md`

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
- `crates/q-runtime/src/main.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

## Parent acceptance guardrails

- Cloud metadata endpoints blocked by safe default.
- DNS rebinding tests fail closed.
- IPv4/IPv6/private ranges handled.
- Policy decisions are observable without logging secrets.

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

- SSRF test suite.
- Threat-model update.
- Configuration examples.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m28-008-z: package evidence for implement ssrf and network egress contr
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M28-008-Z) — PASS

- Date: 2026-08-29
- Branch/PR: m28-008-z (squash-merged; see git log for final hash)
- Closes: #353

### Parent closure — M28-008 Implement SSRF and network egress controls

Parent intent: provide explicit controls for metadata, loopback, private networks, and DNS rebinding. Status: **PASS**.

Packet commits (squash merges):
- M28-008-A — a1e03bb (#951, Closes #348): resolve-and-validate connect gate — `resolve_and_validate(policy, host, resolve)` with metadata-hostname denial before resolution, per-address trust-mode validation, IP-literal direct validation, and the validated PIN SET as the only dial set
- M28-008-B — 13a72ba (#952, Closes #349): atomic redirect revalidation — `RedirectLimiter::follow_hop` composes URL checks + resolve + validate + commit in one call; `FollowedHop.pinned` is the dial set; `seed_target` makes redirect-back-to-origin a typed loop
- M28-008-C — 0a5b532 (#953, Closes #350): egress allow/deny configuration — `with_deny_hosts`/`with_allow_hosts` (bounded 256, normalized), deny wins over allow, allow lists cannot re-enable metadata names, `.suffix` rules cover apex + subdomains
- M28-008-D — ffa1729 (#954, Closes #351): proxy interaction defined — `ProxyMode::Disabled` as the only state, `AMBIENT_PROXY_ENV_VARS` closed survey, `--fetch-stack-info` diagnostic names the posture
- M28-008-V — 8a015dd (#955, Closes #352): verification closure mapping all 4 acceptance guardrails + configuration examples

### Evidence ledger (required microtask evidence)
- **SSRF test suite**: 19 new policy tests across A–D (150 -> 184 lib tests) — metadata by name + by address class, mixed-answer rebinding denial, atomic hop semantics, deny/allow composition, suffix anchoring, IP literals, proxy posture.
- **Threat-model update**: ADR-0033 §2/§3 (SSRF/rebinding) and §5 (no ambient proxy) now have machine-checkable enforcement surfaces; `TRUSTED_CODE_ASSUMPTION` unchanged; the proxy-free dial path makes the pin-set guarantee structural.
- **Configuration examples**: deny/allowlist/loopback-testing/proxy postures documented in the M28-008-V record, each backed by an executing test.

### Source/test map
- `crates/q-capabilities/src/fetch_policy.rs`: `resolve_and_validate`, `is_metadata_hostname`, `HOSTNAME_METADATA_ENDPOINTS`, `RedirectLimiter::follow_hop`/`seed_target`, `FollowedHop`, `MAX_EGRESS_HOST_ENTRIES`, `check_host_config`, `ProxyMode`, `AMBIENT_PROXY_ENV_VARS`.
- `crates/q-capabilities/src/lib.rs`: re-exports.
- `crates/q-runtime/src/fetch_stack.rs`: diagnostic posture string (pinned by test).
- Binary: manifest hash `46de91ac…` (struct-layout change from C's policy fields + D's embedded diagnostic; deterministic).

### Command results (this branch)
- `cargo test -p q-capabilities` → 184 unit + 4 backpressure + 8 WPT passed
- `cargo test -p q-engine-quickjs` → 18 unit + 101 engine passed
- `cargo test -p q-http` → 4+6+1 passed; `-p q-bridge` → 11 passed
- `cargo test -p velqu-runtime` → 8+5+31 passed
- `bun test` → 219 pass / 0 fail; `bun run typecheck` → clean (via ./scripts/verify)
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**

### Ledger update
- `docs/beta/04_TASK_LEDGER.md`: M28-008 flipped TODO -> PASS.

### Disclosures (standing)
- CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR. Local evidence above is complete.
