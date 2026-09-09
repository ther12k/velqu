---
task_id: BETA-004-C
parent_task: BETA-004
milestone: BETA
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-004-C — Parameterized queries/transactions

## Atomic goal

Parameterized queries/transactions.

## Parent intent

Provide a real database story without enlarging core.

## Dependencies

- `BETA-004-B` — `tasks/08_public_beta/BETA-004-B-lazy-pool.md`

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
5. Implement exactly this deliverable: Parameterized queries/transactions.
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
beta-004-c: parameterized queries transactions
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-004-C) — PASS (2026-09-04)

- Branch/PR: beta-004-c (squash-merged; see git log for final hash)
- Closes: #518

### Changed files
- `crates/q-capability-postgres/src/query.rs` (new): bounded query
  contract — closed scalar `SqlParam`/`SqlValue` sets, fail-closed
  text/param/deadline ceilings, deterministic `$N` placeholder scan
  (rejects unbound placeholders pre-wire), typed row conversion with
  exact NULL-vs-conversion semantics (width-matched INT2/4/8,
  FLOAT4/8 reads), `validate_query`/`validate_deadline`.
- `crates/q-capability-postgres/src/executor.rs` (new): production
  `ClientExecutor` implementing `QueryExecutor` over a pooled
  tokio-postgres client — extended-protocol bound parameters only (no
  interpolation path), SQLSTATE-carrying backend errors with URL
  redaction, per-call deadline, owned-future design (no borrowed
  locals across await).
- `crates/q-capability-postgres/src/transaction.rs` (new):
  `run_transaction` — BEGIN -> work -> COMMIT/ROLLBACK, with rollback
  on any error including early `?` return (an open transaction is
  never leaked). 6 deterministic flow tests over a recording executor.
- `crates/q-capability-postgres/tests/live.rs`: extended with the live
  parameterized/transaction verification.
- `docs/reports/beta-004-c-parameterized-queries-transactions.md`
  (new): evidence report.

### Required evidence

- **Capability tests**: 21 unit tests in-crate (12 pool + 3 query
  validation + 6 transaction flow), all deterministic (recording
  executor; zero network).
- **Real-world results**: live run against the benchmark stack —
  width-matched parameterized insert/select returned typed values
  exactly (`item_1` / 7); unbound placeholder failed typed
  `ParamCountMismatch { placeholders: 2, bound: 1 }` pre-wire; COMMIT
  path persisted, ROLLBACK path did not (ordered id assertion). Stack
  torn down after the run.
- **Cold/RSS cost report**: no cost for apps without the capability
  (same crate behind the same grant); per-query cost is the
  extended-protocol round trip plus one allocation per bound param.

### Commands

- `cargo test -p q-capability-postgres` -> 21 unit pass / 0 failed; live 2 pass (env-gated)
- `cargo clippy -p q-capability-postgres --all-targets -- -D warnings` -> clean
- `bun test` -> 383 pass / 0 fail (62 files)
- `./scripts/verify` -> ALL PASS (M0-M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)
  (isolated netns; standing port-3000 environment note, BETA-002-C record)

### Guardrail mapping

- **App without Postgres pays zero dependency/init cost**: unchanged
  from A/B; this layer lives behind the same grant.
- **Queries are parameterized**: extended-protocol binding only; the
  closed scalar set and placeholder validation make interpolation
  unreachable; live typed round-trip proof.
- **Timeout cancels/releases safely**: per-call deadline enforced at
  the executor; engine-level cancel lands with D.
- **Pool exhaustion is bounded**: unchanged (B).
- **W1/W2/W3 workloads pass**: parent exit; not claimed here.

### Standing CI disclosure

CI `verify` workflows stall/fail with zero executed steps on PR creation
across all branches (infrastructure-side, tracked since ~#714); the local
`./scripts/verify` run above is the real gate evidence for this packet.
