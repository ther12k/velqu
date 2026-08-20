---
task_id: M28-011-Z
parent_task: M28-011
milestone: M28
priority: P1
mode: EVIDENCE
status: TODO
context_card: context/milestones/M28.md
commit_required: true
---

# M28-011-Z — Package evidence for Run controlled upstream and fan-out benchmarks

## Atomic goal

Create source-backed evidence and handoff for parent task M28-011; update status only if verification passed.

## Parent intent

Measure scheduler and pool behavior under realistic I/O.

## Dependencies

- `M28-011-V` — `tasks/05_m28_native_fetch/M28-011-V-verify-run-controlled-upstream-and-fan-out-benchmarks.md`

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
- `crates/q-pack/src/lib.rs`
- `crates/q-pack/tests/fuzz_pack.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `docs/specs/pack-format-v1.md`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

## Parent acceptance guardrails

- Queue and pool wait are reported.
- Tail latency remains bounded.
- Error rate and cancellation are correct.
- Results compare matched Elysia/Hono/Fastify candidates.

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

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m28-011-z: package evidence for run controlled upstream and fan out ben
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
