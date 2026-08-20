---
task_id: M28-003-Z
parent_task: M28-003
milestone: M28
priority: P0
mode: EVIDENCE
status: TODO
context_card: context/milestones/M28.md
commit_required: true
---

# M28-003-Z — Package evidence for Implement connection pooling, DNS, and TLS

## Atomic goal

Create source-backed evidence and handoff for parent task M28-003; update status only if verification passed.

## Parent intent

Create a lazy, bounded outbound client shared safely by native services.

## Dependencies

- `M28-003-V` — `tasks/05_m28_native_fetch/M28-003-V-verify-implement-connection-pooling-dns-and-tls.md`

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
- `benchmarks/real-world/postgres/`
- `benchmarks/real-world/SPEC.md`
- `packages/capability-postgres/ (create if absent)`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

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

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m28-003-z: package evidence for implement connection pooling dns and tl
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
