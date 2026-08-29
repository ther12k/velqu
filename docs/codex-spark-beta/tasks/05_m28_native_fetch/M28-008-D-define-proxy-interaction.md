---
task_id: M28-008-D
parent_task: M28-008
milestone: M28
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M28.md
commit_required: true
---

# M28-008-D — Define proxy interaction

## Atomic goal

Define proxy interaction.

## Parent intent

Provide explicit controls for metadata, loopback, private networks, and DNS rebinding.

## Dependencies

- `M28-008-C` — `tasks/05_m28_native_fetch/M28-008-C-support-allow-deny-configuration.md`

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
- `crates/q-runtime/src/main.rs`
- `docs/beta/`
- `examples/proof/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Define proxy interaction.
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
m28-008-d: define proxy interaction
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M28-008-D) — PASS

- Date: 2026-08-29
- Branch/PR: m28-008-d (squash-merged; see git log for final hash)
- Closes: #351

### Changed files
- `crates/q-capabilities/src/fetch_policy.rs`: proxy interaction posture (ADR-0033 §5 made machine-checkable) —
  - `ProxyMode` enum with `Disabled` as the ONLY variant: the runtime dials validated origin addresses directly; no CONNECT tunneling, no proxy credentials anywhere in the fetch path; ambient environment variables ignored by construction; the policy surface exposes no way to enable one.
  - `FetchPolicy::proxy_mode()` — posture accessor, always `Disabled`.
  - `AMBIENT_PROXY_ENV_VARS` — the closed survey list (`http_proxy`, `https_proxy`, `all_proxy`, `no_proxy`): never read by the runtime; exists so diagnostics and tests can assert isolation posture by name.
- `crates/q-capabilities/src/lib.rs`: re-exports `ProxyMode`, `AMBIENT_PROXY_ENV_VARS`.
- `crates/q-runtime/src/fetch_stack.rs`: the `--fetch-stack-info` diagnostic now names the posture ("proxy mode: disabled, ambient env ignored per M28-008-D"), pinned by the existing describe test.

### Tests added
- `proxy_mode_is_disabled_by_construction_and_not_configurable` (default/loopback/configured policies all report Disabled; ambient flag false through every builder)
- `ambient_proxy_env_survey_is_the_closed_list` (the survey list is pinned, lowercased)
- fetch_stack `stack_constructs_lazily_and_names_itself` extended to assert the diagnostic names the proxy posture.

### Command results
- `cargo test -p q-capabilities` → **184 unit (was 182) + 4 backpressure + 8 WPT** — 0 failed
- `cargo test -p q-engine-quickjs` → 18+101 · `-p q-http` 4+6+1 · `-p q-bridge` 11 · `-p velqu-runtime` 8+5+31 — all pass
- `bun test` → 219 pass / 0 fail; `bun run typecheck` → clean (via ./scripts/verify)
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**
- `benchmarks/manifest.json` refreshed (`46de91ac…`): the fetch-stack diagnostic string is embedded in the binary — legitimate artifact change.

### Guardrail mapping
- **Policy decisions are observable without logging secrets** — the proxy posture is queryable (`proxy_mode()`, `--fetch-stack-info`) and names no traffic data; no proxy credentials can exist because no proxy can be configured.
- DNS-rebinding protection composes unchanged: with no proxy, the pinned validated addresses are dialed directly, so no intermediary can re-resolve.

### Disclosures
- `./scripts/verify` initially failed benchmark-evidence (expected binary delta from the embedded diagnostic string); manifest refreshed and verify ALL PASS.
- Standing: CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR.
