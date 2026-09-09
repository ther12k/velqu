---
task_id: BETA-004-A
parent_task: BETA-004
milestone: BETA
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-004-A — Use capability ABI

## Atomic goal

Use capability ABI.

## Parent intent

Provide a real database story without enlarging core.

## Dependencies

- `M27-GATE` — `gates/M27-GATE.md`
- `BETA-001-Z` — `tasks/08_public_beta/BETA-001-Z-package-evidence-for-make-the-real-world-benchmark-harness-executable.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/BETA.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/capabilities-fetch.md`

### Source files

- `AGENTS.md`
- `docs/beta/`
- `scripts/verify`
- `package.json`
- `.github/workflows/verify.yml`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-pack/src/lib.rs`
- `benchmarks/real-world/postgres/`
- `benchmarks/real-world/SPEC.md`
- `packages/capability-postgres/ (create if absent)`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Use capability ABI.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- App without Postgres pays zero dependency/init cost.
- Queries are parameterized.
- Timeout cancels/releases connection safely.
- Pool exhaustion is bounded.
- W1/W2/W3 workloads pass.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p q-capabilities
```
```bash
bun test
```
```bash
bun run typecheck
```

## Required evidence for this microtask

- Capability tests.
- Real-world results.
- Cold/RSS cost report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
beta-004-a: use capability abi
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-004-A) — PASS (2026-09-03)

- Branch/PR: beta-004-a (squash-merged; see git log for final hash)
- Closes: #516

### Changed files
- `crates/q-capabilities/src/postgres.rs` (new): the `runtime:postgres` v1
  ABI model — identity (`postgres_capability_id`/`postgres_requirement`),
  lazy lifecycle via `CapabilityLifecycle` (ops only in `Ready`; terminal
  phases terminal), and the bounded query-op surface
  (`PostgresCapability::start_query`: owner-tagged, cancellable-only,
  deadline ceiling `MAX_POSTGRES_OP_DEADLINE_MS` = 120s, stricter than the
  ABI-wide 300s). No wire protocol, no pool, no I/O — ABI contract only.
- `crates/q-capabilities/src/lib.rs`: module export.
- `packages/capability-postgres/` (new, `@velqu/capability-postgres`):
  `src/index.ts` — identity constants mirroring the Rust model,
  parameterized-only `sql(text, params, deadlineMs)` (RangeError above the
  ceiling before any native call), typed `PostgresCapabilityUnavailable`
  fail-closed error when the host binding is absent; importing constructs
  nothing.
- `packages/capability-postgres/src/index.test.ts` (new): 5 tests —
  identity pinning, zero-construction posture, fail-closed (incl.
  non-function binding), parameterized-only shape, binding-pass-through.
- `packages/capability-postgres/src/pack-wiring.test.ts` (new): 2
  end-to-end CLI-build tests — a `native.postgres` route declares the
  grant in `capability-manifest.json`; a plain route declares nothing
  (zero-cost default).
- `packages/compiler/src/emit.ts`: `postgres` in `KNOWN_GRANTS` +
  `GRANT_MODULES` -> exact `runtime:postgres` v1 requirement in the pack.
- `packages/core/src/index.ts`: type-only `PostgresCapability` on the
  handler `native` context (erased at emit — zero pack bytes).
- `packages/cli/src/capability-inventory.test.ts`: grant universe pinned to
  `["timer", "postgres"]` + postgres requirement-mapping tests (2 new).
- `bun.lock`: new workspace member entry.
- `benchmarks/raw/cold-start/beta004a-velqu.jsonl`: fresh cold/RSS evidence.
- `docs/reports/beta-004-a-capability-abi-costs.md` (new): cold/RSS cost
  report.

### Required evidence

- **Capability tests**: 7 Rust ABI tests (lazy-until-Ready, ops-outside-
  Ready, deadline bounds incl. const-ceiling check, cancellable-only query
  ops through the closed op states, drain/quiesce/terminal); 7 TS tests
  (identity pinning, zero-construction, fail-closed, parameterized-only);
  2 CLI end-to-end wiring tests; 2 grant-mapping tests.
- **Real-world results**: W1/W2/W3 are the parent exit criteria
  (BETA-004-B..E with the live pool); this packet adds no benchmark
  claims — stated in the report.
- **Cold/RSS cost report**:
  `docs/reports/beta-004-a-capability-abi-costs.md` — proof app (no
  postgres grant) cold start p50 11.697ms (ready 11.338ms + first
  response 0.358ms), RSS after ready p50 9,676 kB, 0 failures; pack
  capability inventory unchanged (timer only). Zero dependency/init/RSS
  cost for Postgres-free apps.

### Commands

- `cargo test -p q-capabilities` -> 284 pass / 0 failed (incl. 7 new)
- `cargo test -p q-pack` -> all suites ok
- `cargo test -p q-engine-quickjs` -> all suites ok
- `bun test packages/capability-postgres packages/cli/src/capability-inventory.test.ts` -> 18 pass / 0 fail
- `bun test` -> 383 pass / 0 fail (62 files)
- `bun run typecheck` -> clean; `cargo fmt --all --check` -> clean;
  clippy `-D warnings` -> clean
- `./scripts/verify` -> ALL PASS (M0-M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)
  (isolated netns; standing port-3000 environment note, BETA-002-C record)

### Guardrail mapping

- **App without Postgres pays zero dependency/init cost**: grant absent ->
  no requirement, no module link, no RSS/init (measured + tested).
- **Queries are parameterized**: SDK surface is parameterized-only
  (positional params; no concatenation API); wire behavior pinned in C.
- **Timeout cancels/releases safely**: every query op is cancellable with
  a bounded deadline (ABI-tested); release semantics land with the pool (D).
- **Pool exhaustion is bounded**: pool is B/E; the ABI op model carries
  owner/deadline state they will reuse.
- **W1/W2/W3 workloads pass**: parent exit; not claimed here.

### Standing CI disclosure

CI `verify` workflows stall/fail with zero executed steps on PR creation
across all branches (infrastructure-side, tracked since ~#714); the local
`./scripts/verify` run above is the real gate evidence for this packet.
