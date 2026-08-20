---
task_id: M28-003-V
parent_task: M28-003
milestone: M28
priority: P0
mode: VERIFY
status: TODO
context_card: context/milestones/M28.md
commit_required: true
---

# M28-003-V — Verify Implement connection pooling, DNS, and TLS

## Atomic goal

Prove every acceptance criterion for parent task M28-003 without broadening scope.

## Parent intent

Create a lazy, bounded outbound client shared safely by native services.

## Dependencies

- `M28-003-A` — `tasks/05_m28_native_fetch/M28-003-A-lazy-pool-initialization.md`
- `M28-003-B` — `tasks/05_m28_native_fetch/M28-003-B-bound-idle-active-connections-and-dns-cache.md`
- `M28-003-C` — `tasks/05_m28_native_fetch/M28-003-C-use-verified-tls-roots-and-hostname-validation.md`
- `M28-003-D` — `tasks/05_m28_native_fetch/M28-003-D-define-keepalive-and-shutdown.md`

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
- `packages/cli/src/index.ts`
- `crates/q-runtime/src/source_map.rs`
- `examples/proof/`
- `README.md`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

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

- Pool tests.
- TLS negative tests.
- Startup cost evidence.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m28-003-v: verify implement connection pooling dns and tls
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
