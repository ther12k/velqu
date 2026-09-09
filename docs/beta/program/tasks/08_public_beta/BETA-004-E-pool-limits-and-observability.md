---
task_id: BETA-004-E
parent_task: BETA-004
milestone: BETA
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-004-E — Pool limits and observability

## Atomic goal

Pool limits and observability.

## Parent intent

Provide a real database story without enlarging core.

## Dependencies

- `BETA-004-D` — `tasks/08_public_beta/BETA-004-D-deadline-cancellation-shutdown.md`

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
5. Implement exactly this deliverable: Pool limits and observability.
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
beta-004-e: pool limits and observability
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-004-E) — PASS (2026-09-04)

- Branch/PR: beta-004-e (squash-merged; see git log for final hash)
- Closes: #520

### Changed files
- `crates/q-capability-postgres/src/lib.rs`: `PoolCounters` (10
  monotonic counters) + `PoolCountersSnapshot` + `LazyPool::counters()`;
  instrumentation across acquire/reuse/create/stale-dead discard/
  error-discard/at-capacity/connect-timeout/connect-rejected/shutdown
  refusal paths. 6 new tests (counter tracking incl. env-config).
- `crates/q-capability-postgres/src/dialer.rs`: env-configurable limits
  — `VELQU_PG_POOL_MAX` / `VELQU_PG_POOL_CONNECT_TIMEOUT_MS` /
  `VELQU_PG_POOL_IDLE_TIMEOUT_MS` via `pool_config_from_lookup` +
  `pool_from_url_and_env`; invalid values are startup rejections, never
  clamps. 3 env-config tests.
- `crates/q-runtime/src/lib.rs`: startup wiring uses the env-configured
  limits; invalid limits reject startup with a typed readiness error.
- `crates/q-engine/src/lib.rs` + `crates/q-engine-quickjs/src/worker.rs`:
  `postgres_ops_started` / `postgres_ops_completed` on `EngineStats`.
- `docs/reports/beta-004-e-pool-limits-observability.md` (new).

### Required evidence

- **Capability tests**: crate total 28 unit + 2 live pass (6 new
  counter/env tests).
- **Real-world results**: invalid `VELQU_PG_POOL_MAX=5000` -> typed
  startup rejection; valid `VELQU_PG_POOL_MAX=3` -> fixture app served
  live queries under the limit; stack torn down after the run.
- **Cold/RSS cost report**: counters are plain atomics; snapshots never
  block; zero cost for apps without the grant (unchanged from A-D).

### Commands

- `cargo test -p q-capability-postgres` -> 28 unit + 2 live pass
- fmt / clippy (`-D warnings`) / typecheck -> clean
- `bun test` -> 383 pass / 0 fail (62 files)
- `./scripts/verify` -> ALL PASS (M0-M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)
  (isolated netns; standing port-3000 environment note, BETA-002-C record)

### Guardrail mapping

- **App without Postgres pays zero dependency/init cost**: unchanged.
- **Queries are parameterized**: unchanged from C.
- **Timeout cancels/releases safely**: unchanged from D; counters
  observe without absorbing failures.
- **Pool exhaustion is bounded**: ceiling configurable within
  fail-closed bounds; exhaustion stays a typed error, now counted.
- **W1/W2/W3 workloads pass**: parent exit; not claimed here.

### Standing CI disclosure

CI `verify` workflows stall/fail with zero executed steps on PR creation
across all branches (infrastructure-side, tracked since ~#714); the local
`./scripts/verify` run above is the real gate evidence for this packet.
