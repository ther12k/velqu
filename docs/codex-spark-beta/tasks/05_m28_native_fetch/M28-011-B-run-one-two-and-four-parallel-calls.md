---
task_id: M28-011-B
parent_task: M28-011
milestone: M28
priority: P1
mode: IMPLEMENT
status: TODO
context_card: context/milestones/M28.md
commit_required: true
---

# M28-011-B — Run one, two, and four parallel calls

## Atomic goal

Run one, two, and four parallel calls.

## Parent intent

Measure scheduler and pool behavior under realistic I/O.

## Dependencies

- `M28-011-A` — `tasks/05_m28_native_fetch/M28-011-A-run-1-5-10-25ms-upstream-latency.md`

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

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Run one, two, and four parallel calls.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

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

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m28-011-b: run one two and four parallel calls
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
