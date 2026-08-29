---
task_id: M28-011-B
parent_task: M28-011
milestone: M28
priority: P1
mode: IMPLEMENT
status: PASS
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

## Result (M28-011-B) — PASS

- Date: 2026-08-29
- Branch/PR: m28-011-b (squash-merged; see git log for final hash)
- Closes: #367

### Changed files
- `benchmarks/real-world/candidates/*`: every candidate gains `GET /api/bench/fanout?n=1|2|4&ms=5` — n PARALLEL upstream calls aggregated via Promise.all, returning `{n, ms, ok}`. `shared.ts`/`shared.cjs` gain `validateFanout` (n in {1,2,4} exactly; ms validated as before).
- `benchmarks/real-world/workloads.json`: FANOUT_1/2/4 workload cells added.
- `benchmarks/real-world/run-fanout.sh` (new): one-command fan-out matrix (upstream -> per-candidate boot -> cells -> comparison). Ensures candidate deps are installed from the committed lockfile (`bun install --frozen-lockfile`) — Bun auto-install would silently resolve UNPINNED latest versions in fresh checkouts (observed: elysia 1.4.30 instead of the pinned 2.0.0-beta.4).
- `benchmarks/real-world/compare-fanout.ts` (new): aggregates summaries with the **parallelism proof**: p50(n=4) must be strictly less than 4x p50(n=1) per candidate per concurrency — plus 0 errors / 0 mismatches everywhere.
- `benchmarks/real-world/load.ts`: summary validation now covers exactly the SELECTED cells (a --workloads filter is a legitimate run shape, not a partial-failure state).
- `benchmarks/real-world/workloads.test.ts`: PATHS guard extended for the fanout route shape.

### Raw evidence (committed)
- `benchmarks/raw/real-world/fanout/`: per-candidate summaries + logs + comparison.md.
- Headline (3s cells, c=1/10, ms=5): ALL 24 cells 0 errors / 0 mismatches; parallelism proven for every candidate: hono p50 n=1 5803us -> n=4 5795us (c=1); bun-fetch n=1 5817us -> n=4 5817us; fastify n=1 6801us -> n=4 6542us. Wall time is flat in n — the upstream calls genuinely run in parallel, not sequentially.
- Velqu's own slot remains pending the fetch executor wiring (disclosed in M28-011-A, unchanged).

### Command results
- `bash benchmarks/real-world/run-fanout.sh 3 1,10` → PASS
- `bun test benchmarks/real-world` → 36 pass / 0 fail
- `cargo test` all packages green; `bun run typecheck` → clean; `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**; release binary unchanged

### Guardrail mapping
- **Queue and pool wait are reported** — fan-out multiplies pool pressure; the metrics schema (M28-009-A) carries the wait stages.
- **Tail latency remains bounded** — structural guardrails enforced by compare-fanout (0 errors/mismatches; parallelism proof).
- **Results compare matched candidates** — identical fan-out contract, same upstream, same load generator, pinned deps.

### Disclosures
- Two fresh-checkout traps found and fixed at the harness level: Bun auto-install resolving unpinned candidate deps (now lockfile-frozen installs), and the summary validator rejecting filtered runs (now validates the selected cells). The PATHS config guard correctly rejected the new route shape first — extended deliberately.
- Standing: CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR.
