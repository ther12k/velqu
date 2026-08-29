---
task_id: M28-011-C
parent_task: M28-011
milestone: M28
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M28.md
commit_required: true
---

# M28-011-C — Mix timeout/success/malformed responses

## Atomic goal

Mix timeout/success/malformed responses.

## Parent intent

Measure scheduler and pool behavior under realistic I/O.

## Dependencies

- `M28-011-B` — `tasks/05_m28_native_fetch/M28-011-B-run-one-two-and-four-parallel-calls.md`

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
5. Implement exactly this deliverable: Mix timeout/success/malformed responses.
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
m28-011-c: mix timeout success malformed responses
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M28-011-C) — PASS

- Date: 2026-08-29
- Branch/PR: m28-011-c (squash-merged; see git log for final hash)
- Closes: #368

### Changed files
- `benchmarks/real-world/upstream.ts`: deterministic `/bad` fixture endpoint (HTTP 200 + garbage body, content-type json) — malformed-response source for the mixed matrix. Additive; existing endpoints untouched.
- `benchmarks/real-world/candidates/*`: every candidate gains `GET /api/bench/mixed?mode=success|timeout|malformed` mapping deterministic upstream outcomes to typed statuses — success: 200 relay; timeout: upstream 500ms vs a 100ms client deadline (`AbortSignal.timeout`) -> **504**; malformed: 200 + garbage -> JSON parse failure -> **502**. `shared.ts`/`shared.cjs` gain `validateMode` (closed mode set).
- `benchmarks/real-world/workloads.json`: MIX_SUCCESS (200) / MIX_TIMEOUT (504) / MIX_MALFORMED (502) cells.
- `benchmarks/real-world/workloads.test.ts`: the status guard now permits the documented failure-mode statuses (502/504) for `/api/bench/mixed` paths only; PATHS guard extended.
- `benchmarks/real-world/run-mixed.sh` (new): one-command mixed matrix.
- `benchmarks/real-world/compare-mixed.ts` (new): aggregates with guardrails — every cell 0 errors + 0 mismatches (each mode maps to its EXACT typed status) and bounded handling overhead (timeout/malformed p50 <= 2x success p50 + 250ms).

### Raw evidence (committed)
- `benchmarks/raw/real-world/mixed/`: per-candidate summaries + logs + comparison.md.
- Headline: ALL 24 cells 0 errors / 0 mismatches — every candidate maps success->200, timeout->504, malformed->502 exactly. Timeout cells sit at ~101ms p50 (the deadline), malformed at ~144-4132us p50 (parse-fail is fast); overhead guardrail green.

### Command results
- `bash benchmarks/real-world/run-mixed.sh 3 1,10` → PASS
- `bun test benchmarks/real-world` → 36 pass / 0 fail
- `cargo test` all packages green; `bun run typecheck` → clean; fmt/clippy clean
- `./scripts/verify` → **ALL PASS (exit 0)**; release binary unchanged

### Guardrail mapping
- **Error rate and cancellation are correct** — the mixed matrix IS the error-path rate proof: deterministic typed statuses at load, zero unexpected errors.

### Disclosures
- The first matrix run correctly FAILED the comparison: hono returned 500 for every mixed mode (module-scope handler lost the route context `c`) and bun-fetch returned 500 on timeout (missing try/catch around the abort rejection) while elysia/fastify passed. Both bugs fixed and the probe now shows exact 200/504/502 for all four. This is the comparison harness doing its job — failure handling bugs surface as data.
- Standing: CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR.
