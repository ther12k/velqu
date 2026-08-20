---
task_id: M28-008-V
parent_task: M28-008
milestone: M28
priority: P0
mode: VERIFY
status: TODO
context_card: context/milestones/M28.md
commit_required: true
---

# M28-008-V — Verify Implement SSRF and network egress controls

## Atomic goal

Prove every acceptance criterion for parent task M28-008 without broadening scope.

## Parent intent

Provide explicit controls for metadata, loopback, private networks, and DNS rebinding.

## Dependencies

- `M28-008-A` — `tasks/05_m28_native_fetch/M28-008-A-resolve-and-validate-addresses-before-connect.md`
- `M28-008-B` — `tasks/05_m28_native_fetch/M28-008-B-revalidate-redirects-and-connection-targets.md`
- `M28-008-C` — `tasks/05_m28_native_fetch/M28-008-C-support-allow-deny-configuration.md`
- `M28-008-D` — `tasks/05_m28_native_fetch/M28-008-D-define-proxy-interaction.md`

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
- `crates/q-bridge/src/lib.rs`
- `crates/q-runtime/tests/runtime_conformance.rs`
- `Cargo.toml`
- `crates/q-runtime/src/main.rs`
- `packages/core/src/index.ts`
- `docs/beta/`
- `examples/proof/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

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

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m28-008-v: verify implement ssrf and network egress controls
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
