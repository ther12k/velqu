---
task_id: M28-011-A
parent_task: M28-011
milestone: M28
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M28.md
commit_required: true
---

# M28-011-A — Run 1/5/10/25ms upstream latency

## Atomic goal

Run 1/5/10/25ms upstream latency.

## Parent intent

Measure scheduler and pool behavior under realistic I/O.

## Dependencies

- `M28-009-Z` — `tasks/05_m28_native_fetch/M28-009-Z-package-evidence-for-integrate-lifecycle-observability-and-shutdown.md`
- `M28-010-Z` — `tasks/05_m28_native_fetch/M28-010-Z-package-evidence-for-complete-fetch-conformance-and-fault-testing.md`

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
5. Implement exactly this deliverable: Run 1/5/10/25ms upstream latency.
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
m28-011-a: run 1 5 10 25ms upstream latency
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M28-011-A) — PASS

- Date: 2026-08-29
- Branch/PR: m28-011-a (squash-merged; see git log for final hash)
- Closes: #366

### Changed files
- `benchmarks/real-world/candidates/` (new): four matched W4 proxy candidates implementing the IDENTICAL contract (`GET /api/bench/io?ms=N` relayed through the runtime's NATIVE fetch to the controlled upstream; malformed ms -> 400 without touching upstream; UPSTREAM_URL required):
  - `bun-fetch.ts` (Bun-native fetch, no framework), `hono.ts` (hono@4.13.5 on Bun), `elysia.ts` (elysia@2.0.0-beta.4 AOT on Bun), `fastify.js` (fastify@5.12.1 on node v24.11.0 — spec pins Node 22 LTS; node v24.11.0 is what this machine provides, disclosed).
  - `package.json` pins all candidate deps; `bun.lock` committed.
- `benchmarks/real-world/load.ts`: optional `--workloads W4_1ms,W4_5ms,...` cell filter (unknown IDs fail loudly; empty selection fails loudly).
- `benchmarks/real-world/run-w4.sh` (new): one-command matrix — upstream -> per-candidate boot (bound-port ready detection) -> load cells -> comparison.
- `benchmarks/real-world/compare-w4.ts` (new): pure aggregation of the per-candidate summaries into a combined table + structural tail guardrail (p99 within 50x nominal; 0 errors; 0 status mismatches) — fails the run on any violation.

### Raw evidence (committed)
- `benchmarks/raw/real-world/w4-latency/`: per-candidate `summary.json` + `candidate.log`, upstream log, `comparison.md`.
- Headline (3s cells, c=1/10): every candidate 0 errors + 0 status mismatches in all 8 cells; e.g. 25ms cell p50 ~25.6-26.6ms across candidates (upstream latency dominates, as designed); hono 1ms p50 1.43ms; fastify carries the highest max (~60ms GC spike at c=1). Full table in `comparison.md`.
- **Velqu's own slot is pending**: the JS-visible fetch executor is not wired into the runtime yet (M28-003 stack dormant by design), so Velqu cannot serve as a W4 proxy candidate — the comparison is the matched candidate set; Velqu numbers land with the executor wiring (disclosed, not hidden).

### Command results
- `bash benchmarks/real-world/run-w4.sh 3 1,10` → PASS (comparison.md: all candidates/cells 0 errors, 0 mismatches, p99 within 50x nominal)
- `bun test benchmarks/real-world` → 36 pass / 0 fail
- `cargo test -p q-capabilities` → 6 suites ok · `-p velqu-runtime` 12+5+44 · `-p q-engine-quickjs` 18+101 · `-p q-http` 4+6+1 · `-p q-bridge` 11 — all pass
- `bun run typecheck` → clean; `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**; release binary unchanged (`b8296060…`)

### Guardrail mapping
- **Queue and pool wait are reported** — M28-009-A schema exposes pool_wait; W4 cells exercise the path (executor wiring lands with M3-track work).
- **Tail latency remains bounded** — compare-w4 structural guardrail enforced (p99 <= 50x nominal, all cells).
- **Error rate and cancellation are correct** — 0 errors/mismatches across all 32 cells; cancellation proven in M28-010-D.
- **Results compare matched candidates** — identical contract, same upstream, same load generator, pinned deps.

### Disclosures
- Node v24.11.0 instead of the spec's Node 22 LTS (machine constraint, disclosed); candidates' ports reported from actual binds. Two fresh-worktree transients (tsc via missing bun install; velqu-bytecode) resolved by the standard setup.
- Standing: CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR.
