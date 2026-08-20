---
task_id: M28-011-V
parent_task: M28-011
milestone: M28
priority: P1
mode: VERIFY
status: TODO
context_card: context/milestones/M28.md
commit_required: true
---

# M28-011-V — Verify Run controlled upstream and fan-out benchmarks

## Atomic goal

Prove every acceptance criterion for parent task M28-011 without broadening scope.

## Parent intent

Measure scheduler and pool behavior under realistic I/O.

## Dependencies

- `M28-011-A` — `tasks/05_m28_native_fetch/M28-011-A-run-1-5-10-25ms-upstream-latency.md`
- `M28-011-B` — `tasks/05_m28_native_fetch/M28-011-B-run-one-two-and-four-parallel-calls.md`
- `M28-011-C` — `tasks/05_m28_native_fetch/M28-011-C-mix-timeout-success-malformed-responses.md`
- `M28-011-D` — `tasks/05_m28_native_fetch/M28-011-D-test-concurrency-1-10-50-200.md`

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
- `benchmarks/harness/`
- `benchmarks/manifest.json`
- `benchmarks/raw/`
- `docs/reports/`
- `scripts/benchmark`
- `benchmarks/real-world/postgres/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- Queue and pool wait are reported.
- Tail latency remains bounded.
- Error rate and cancellation are correct.
- Results compare matched Elysia/Hono/Fastify candidates.

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

- Raw real-world results.
- Generated report.
- Candidate hashes.
- [ ] Fetch is useful, Web-compatible within a documented subset, and lazy when unused.
- [ ] DNS/TLS/redirect/SSRF defaults fail closed.
- [ ] Deadlines and AbortSignal physically cancel work.
- [ ] Streaming is bounded and backpressured.
- [ ] Conformance and realistic I/O evidence pass.
- Controlled upstream 1/5/10/25ms.
- Fan-out 1/2/4.
- Large streaming bodies.
- Pool saturation and cancellation.
- No node:http/node:https.
- No arbitrary raw sockets.
- No WebSocket.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m28-011-v: verify run controlled upstream and fan out benchmarks
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
