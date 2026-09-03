---
task_id: BETA-004-B
parent_task: BETA-004
milestone: BETA
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-004-B — Lazy pool

## Atomic goal

Lazy pool.

## Parent intent

Provide a real database story without enlarging core.

## Dependencies

- `BETA-004-A` — `tasks/08_public_beta/BETA-004-A-use-capability-abi.md`

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
5. Implement exactly this deliverable: Lazy pool.
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
beta-004-b: lazy pool
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-004-B) — PASS (2026-09-04)

- Branch/PR: beta-004-b (squash-merged; see git log for final hash)
- Closes: #517

### Changed files
- `crates/q-capability-postgres/` (new crate — core unenlarged):
  - `src/lib.rs`: `LazyPool<F: Connector>` — lazy-by-construction pool
    (zero I/O at construction, unit-tested with a counting connector),
    semaphore-bounded capacity (config validated 1..=100, fail closed),
    typed `PoolError` set (`AtCapacity{max,waited_ms}`, `ConnectTimeout`,
    `ConnectRejected`, `ShuttingDown`, `InvalidConfig`,
    `MissingDatabaseUrl`), idle reuse with stale/dead discard on
    acquire, `begin_shutdown()` gate (release-under-shutdown closes,
    not parks), `PoolStats` snapshot, generic core over a `Connector`
    trait with a mock-tested deterministic core and a production
    `TokioConnector` (tokio-postgres, no TLS; connect-error strings
    redacted of any URL fragment).
  - `tests/live.rs`: env-gated live verification
    (`VELQU_PG_LIVE_TEST=1`) against the real benchmark stack.
  - 12 deterministic unit tests (no network) + 1 live test.
- `Cargo.toml` / `Cargo.lock`: workspace member + pinned
  `tokio-postgres = "0.7"` workspace dependency.
- `docs/reports/beta-004-b-lazy-pool.md` (new): lazy/bounded/typed
  evidence + live real-world results + cold/RSS cost report.

### Required evidence

- **Capability tests**: 12 deterministic unit tests (laziness, reuse,
  ceiling, typed timeouts/rejections, stale/dead discard, shutdown
  gate) + 9 indirectly through the workspace suite; clippy `-D
  warnings` clean (caught and fixed a mutex-held-across-await defect
  during review).
- **Real-world results**: live run against the benchmark stack
  (postgres:17.5, SCRAM, seeded 1,000 users): `SELECT count(*) FROM
  users` -> 1,000; idle reuse (created_total=1); 5th acquire under a
  4-connection ceiling -> typed `AtCapacity{max:4}`; unroutable connect
  -> typed timeout/rejection; post-shutdown acquire -> `ShuttingDown`.
  Stack torn down after the run.
- **Cold/RSS cost report**: pool construction is free (no parse, no
  DNS, no socket, no backend session until first acquire); an app that
  never grants postgres never constructs a pool (BETA-004-A wiring);
  baseline cold/RSS unchanged (BETA-004-A report).

### Commands

- `cargo test -p q-capability-postgres` -> 12 pass / 0 failed (unit) + 1 pass (live, env-gated)
- `cargo clippy -p q-capability-postgres --all-targets -- -D warnings` -> clean
- `bun test` -> 383 pass / 0 fail (62 files)
- `./scripts/verify` -> ALL PASS (M0-M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)
  (isolated netns; standing port-3000 environment note, BETA-002-C record)

### Guardrail mapping

- **App without Postgres pays zero dependency/init cost**: pool exists
  only behind the capability; construction performs zero I/O (tested).
- **Queries are parameterized**: pool carries connections only; the
  query surface (C) binds positional params via tokio-postgres
  extended protocol.
- **Timeout cancels/releases safely**: acquire/connect bounded typed;
  release returns the connection to idle (or closes under shutdown) —
  engine-level cancel semantics land with D on top of this gate.
- **Pool exhaustion is bounded**: semaphore ceiling 1..=100 fail
  closed; exhaustion is a typed `AtCapacity` error, never an unbounded
  wait.
- **W1/W2/W3 workloads pass**: parent exit; not claimed here.

### Standing CI disclosure

CI `verify` workflows stall/fail with zero executed steps on PR creation
across all branches (infrastructure-side, tracked since ~#714); the local
`./scripts/verify` run above is the real gate evidence for this packet.
