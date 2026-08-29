---
task_id: M28-011-Z
parent_task: M28-011
milestone: M28
priority: P1
mode: EVIDENCE
status: PASS
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

## Result (M28-011-Z) — PASS

- Date: 2026-08-30
- Branch/PR: m28-011-z (squash-merged; see git log for final hash)
- Closes: #371

### Parent closure — M28-011 Run controlled upstream and fan-out benchmarks

Parent intent: measure scheduler and pool behavior under realistic I/O. Status: **PASS**.

Packet commits (squash merges):
- M28-011-A — 1b56d46 (#969, Closes #366): W4 controlled-latency matrix (1/5/10/25ms) — four matched proxy candidates (bun-fetch/hono/elysia2/fastify, pinned), `--workloads` cell filter, comparison with structural tail guardrail; 32/32 cells 0 errors/0 mismatches
- M28-011-B — 07517d9 (#970, Closes #367): fan-out 1/2/4 — parallelism proven per candidate (p50(n=4) < 4x p50(n=1) everywhere; wall time flat in n); lockfile-frozen candidate installs (auto-install resolved UNPINNED elysia)
- M28-011-C — 688d7d9 (#971, Closes #368): mixed-outcome matrix — success->200, deadline timeout->504, malformed body->502, exact typed statuses in all 24 cells; the comparison caught two real candidate bugs (hono context scoping, bun-fetch missing abort catch) which were fixed
- M28-011-D — 6630b63 (#972, Closes #369): concurrency ladder 1/10/50/200 — 32/32 cells clean, throughput scales everywhere, concurrency-aware fair-share tail bound
- M28-011-V — 07bf502 (#973, Closes #370): verification closure; disclosed a reproducibility finding: fastify (Node fetch) c=200/1ms tail does not reproduce within the fair-share bound across days (186ms committed vs 0.9-1.5s re-measured) — Node-client limitation at extreme fan-in, not a velqu path

### Required evidence
- **Raw results**: `benchmarks/raw/real-world/{w4-latency,fanout,mixed,concurrency}/` — per-candidate raw JSONL (+gz for the ladder), summaries, logs, comparison reports — all committed.
- **Generated report**: `comparison.md` per matrix with enforced structural guardrails (0 errors/0 mismatches; parallelism proof; fair-share tail bounds).
- **Candidate hashes**: `candidates/bun.lock` + `package.json` pin hono@4.13.5, elysia@2.0.0-beta.4, fastify@5.12.1; runner installs `--frozen-lockfile`.
- **Overhead**: M28-009-V measured the instrumentation path (~0 ns plain / ~22 ns collector / 0 disabled).

### Source/test map
- `benchmarks/real-world/candidates/` (4 matched apps + shared contract), `run-w4.sh`, `run-fanout.sh`, `run-mixed.sh`, `run-concurrency.sh`, `compare-w4.ts`, `compare-fanout.ts`, `compare-mixed.ts`, `compare-concurrency.ts`, `load.ts` (--workloads filter + selected-cells validation), `upstream.ts` (/bad malformed fixture), `workloads.json` (+6 cells), `workloads.test.ts` (guards extended)
- `crates/q-runtime/tests/runtime_conformance.rs`: `graceful_shutdown_exits_zero` (fetchPool drain assertion; log collection made race-free with a bounded poll this packet)

### Command results (this branch)
- `cargo test -p q-capabilities` → 6 suites; `-p velqu-runtime` → 12+5+44; `-p q-engine-quickjs` → 18+101; `-p q-http` → 4+6+1; `-p q-bridge` → 11 — all pass
- `bun test benchmarks/real-world` → 36 pass / 0 fail; `bun run typecheck` → clean
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**

### Ledger update
- `docs/beta/04_TASK_LEDGER.md`: M28-011 flipped TODO -> PASS.

### Disclosures
- This packet fixes a test-harness race found during evidence runs: the extended SIGTERM test drained the log pipe once, before the reader thread had forwarded the final lines — replaced with a bounded poll (3/3 stable). Test-collection fix; no production change.
- Standing: CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR.
