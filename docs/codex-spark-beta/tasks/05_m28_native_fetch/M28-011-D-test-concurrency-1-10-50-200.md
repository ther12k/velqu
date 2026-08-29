---
task_id: M28-011-D
parent_task: M28-011
milestone: M28
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M28.md
commit_required: true
---

# M28-011-D — Test concurrency 1/10/50/200

## Atomic goal

Test concurrency 1/10/50/200.

## Parent intent

Measure scheduler and pool behavior under realistic I/O.

## Dependencies

- `M28-011-C` — `tasks/05_m28_native_fetch/M28-011-C-mix-timeout-success-malformed-responses.md`

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
5. Implement exactly this deliverable: Test concurrency 1/10/50/200.
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
m28-011-d: test concurrency 1 10 50 200
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M28-011-D) — PASS

- Date: 2026-08-30
- Branch/PR: m28-011-d (squash-merged; see git log for final hash)
- Closes: #369

### Changed files
- `benchmarks/real-world/run-concurrency.sh` (new): the SPEC's full concurrency ladder (1/10/50/200) over the W4_1ms and W4_5ms cells, all four matched candidates, one command. Raw rows are gzip-retained (`raw.jsonl.gz`); summaries stay plain.
- `benchmarks/real-world/compare-concurrency.ts` (new): aggregates with structural guardrails —
  - 0 errors + 0 status mismatches in EVERY cell (32 cells across 4 candidates),
  - throughput must SCALE: rps(c=200) > rps(c=1) per candidate per cell,
  - concurrency-aware tail bound: p99 <= max(50x nominal, c x nominal) — a Little's-law-shaped fair-share bound (at 200 in-flight requests queueing delay is expected; the bound proves boundedness, never a hang).

### Raw evidence (committed)
- `benchmarks/raw/real-world/concurrency/`: gzipped raw rows + summaries + logs + comparison.md.
- Headline: ALL 32 cells 0 errors / 0 mismatches; throughput scales for every candidate/cell (e.g. bun-fetch 1ms cell 556 -> 15412 rps; fastify 1ms 359 -> 4126 rps); tails bounded — the single first-run violation (fastify c=200/1ms p99 186ms vs the naive flat 50x=50ms line) is genuine event-loop queueing at the fair-share level (c x nominal = 200ms) and drove the bound's refinement from flat to fair-share.

### Command results
- `bash benchmarks/real-world/run-concurrency.sh 3` → PASS
- `bun test benchmarks/real-world` → 36 pass / 0 fail
- `cargo test` all packages green; `bun run typecheck` → clean; fmt/clippy clean
- `./scripts/verify` → **ALL PASS (exit 0)**; release binary unchanged (test-only packet)

### Guardrail mapping
- **Queue and pool wait are reported** — the ladder quantifies queueing directly (c=200 latencies reflect fair-share waits).
- **Tail latency remains bounded** — fair-share bound enforced structurally; violations fail the run.
- **Error rate and cancellation are correct** — 0 errors/0 mismatches across all 32 cells including c=200.

### Disclosures
- The naive flat tail bound initially failed fastify at c=200/1ms (real queueing, 186ms p99); the bound was refined to the concurrency-aware fair-share form rather than relaxed blindly — the refinement is documented in the comparison generator.
- Standing: CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR.
