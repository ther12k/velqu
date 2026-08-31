---
task_id: M4A-001-C
parent_task: M4A-001
milestone: M4A
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-001-C — Load new worker before switching traffic

## Atomic goal

Load new worker before switching traffic.

## Parent intent

Compile and reload the real QuickJS/QPack runtime with fast feedback and parity.

## Dependencies

- `M4A-001-B` — `tasks/07_m4a_developer_preview/M4A-001-B-build-incremental-temporary-qpack.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M4A.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/devex-beta.md`

### Source files

- `AGENTS.md`
- `packages/cli/src/index.ts`
- `packages/compiler/src/index.ts`
- `packages/treaty/src/index.ts`
- `packages/testing/src/index.ts`
- `crates/q-pack/src/lib.rs`
- `crates/q-pack/tests/fuzz_pack.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `docs/specs/pack-format-v1.md`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Load new worker before switching traffic.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- No Bun-only behavior mismatch by default.
- Failed reload keeps prior healthy app.
- Source maps point to TypeScript.
- Reload is bounded and observable.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
cargo test -p q-engine-quickjs
```
```bash
bun test
```
```bash
bun run typecheck
```

## Required evidence for this microtask

- Reload conformance.
- Failure recovery tests.
- Developer latency measurements.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m4a-001-c: load new worker before switching traffic
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M4A-001-C) — PASS

- Date: 2026-08-31
- Branch/PR: m4a-001-c (squash-merged; see git log for final hash)
- Closes: #434

### Changed files
- `packages/cli/src/dev-server.ts` (new): actual-runtime dev server and
  worker swap pipeline (`DevServer`, `WorkerInstance`, `ReloadResult`) —
  - `start()`: builds initial temporary QPack, spawns and verifies worker
    generation 1, starts public HTTP gateway.
  - `reload()`: builds updated temporary QPack, spawns candidate worker
    runtime on ephemeral port, verifies readiness before switching traffic.
  - Fail-safe retention: if build or candidate worker initialization fails,
    retains the prior healthy worker serving traffic without dropping requests
    (**"Failed reload keeps prior healthy app"**).
  - Atomic traffic switch: updates `activeWorker` pointer only after the new
    worker runtime is verified healthy and ready.
  - Proxy gateway: forwards public HTTP requests to the active worker port
    with standard forwarded headers and duplex streaming support.
- `packages/cli/src/index.ts`: exports `DevServer` and wires `velqu dev`
  CLI command to run `DevServer`.
- `packages/cli/src/dev-server.test.ts` (new): 4 integration tests.
- `benchmarks/manifest.json`: refreshed (standard remapped flow).

### Tests added (packages/cli/src/dev-server.test.ts, +4 tests)
- Starts dev server and proxies requests to QuickJS worker generation 1.
- Loads candidate worker and verifies readiness before switching traffic on reload.
- Retains prior healthy worker when reload compilation fails (failed reload keeps prior app).
- Drives proof fixture end-to-end through dev server gateway.

### Command results
- `cargo test -p q-pack` → 3 suites — 0 failed
- `cargo test -p q-engine-quickjs` → 20 + 102 + 1 — 0 failed
- `bun test` → **235 pass / 0 fail (30 files, +4 new tests)**
- `bun run typecheck` → clean
- `cargo fmt --check` clean; workspace clippy -D warnings → exit 0
- `./scripts/verify` → **ALL PASS**

### Guardrail mapping (parent M4A-001)
- **Failed reload keeps prior healthy app** — tested: syntax errors during
  reload retain generation 1 and traffic continues uninterrupted.
- **No Bun-only behavior mismatch** — dev server executes the real
  `velqu-runtime` binary with temporary QPacks.
- **Reload is bounded and observable** — reload latency measured (< 500ms).

### Disclosures
- Standing: CI fails with zero executed steps on every PR since ~#714
  (infrastructure-side); disclosed per PR.
