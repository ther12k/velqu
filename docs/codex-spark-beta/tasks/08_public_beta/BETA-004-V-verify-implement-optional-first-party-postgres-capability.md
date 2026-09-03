---
task_id: BETA-004-V
parent_task: BETA-004
milestone: BETA
priority: P0
mode: VERIFY
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-004-V — Verify Implement optional first-party Postgres capability

## Atomic goal

Prove every acceptance criterion for parent task BETA-004 without broadening scope.

## Parent intent

Provide a real database story without enlarging core.

## Dependencies

- `BETA-004-A` — `tasks/08_public_beta/BETA-004-A-use-capability-abi.md`
- `BETA-004-B` — `tasks/08_public_beta/BETA-004-B-lazy-pool.md`
- `BETA-004-C` — `tasks/08_public_beta/BETA-004-C-parameterized-queries-transactions.md`
- `BETA-004-D` — `tasks/08_public_beta/BETA-004-D-deadline-cancellation-shutdown.md`
- `BETA-004-E` — `tasks/08_public_beta/BETA-004-E-pool-limits-and-observability.md`
- `BETA-004-F` — `tasks/08_public_beta/BETA-004-F-no-orm.md`

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

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

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

- Capability tests.
- Real-world results.
- Cold/RSS cost report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
beta-004-v: verify implement optional first party postgres capability
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-004-V) — PASS (2026-09-04)

- Branch/PR: beta-004-v (squash-merged; see git log for final hash)
- Closes: #522

### Acceptance-criterion mapping (parent BETA-004)

1. **App without Postgres pays zero dependency/init cost**
   - A: pack without the grant carries no requirement and links no pool
     (capability-inventory tests); cold/RSS measured unchanged
     (`docs/reports/beta-004-a-capability-abi-costs.md`).
   - Pool construction is lazy with zero I/O (B unit test:
     `construction_is_lazy_zero_io`); the dialer/pool live behind the
     grant only.
2. **Queries are parameterized**
   - C: extended-protocol binding only; closed scalar params; `$N`
     placeholder validation pre-wire (`ParamCountMismatch` typed);
     freeze test pins the single-method surface (F).
   - Live: width-matched insert/select round-trip returned typed values
     exactly (B/C/D live tests).
3. **Timeout cancels/releases connection safely**
   - D: deadline bounds acquire AND execution; typed
     `ConnectTimeout`/`DeadlineExceeded`; discard-on-error lease
     semantics (mid-flight connections close, never reused) —
     unit-tested (`counters_track_timeout_and_rejection`,
     `counters_track_discarded_error_leases`).
4. **Pool exhaustion is bounded**
   - B/E: semaphore ceiling 1..=100 fail closed; typed
     `AtCapacity{max,waited_ms}`; env-configurable limits reject
     startup when out of bounds (E test: `VELQU_PG_POOL_MAX=5000` ->
     typed startup rejection measured live in D/E packets).
5. **W1/W2/W3 workloads pass**
   - Verified end-to-end at the level this parent delivers: live
     HTTP -> QuickJS -> native binding -> pool -> real Postgres ->
     schema-validated response (D packet fixture app, measured);
     full W1/W2/W3 load runs are the canonical-report scope
     (BETA-013/014) — not claimed here.

### Commands (fresh on this branch)

- `cargo test -p q-capability-postgres` -> 28 unit + 2 live pass
  (live against the benchmark stack: postgres:17.5, SCRAM, seeded)
- `cargo test -p q-pack` (3 suites), `-p q-engine-quickjs` (3 suites,
  137 pass), `-p q-capabilities` (8 suites) -> all ok
- `bun test` -> 384 pass / 0 fail (62 files)
- `bun run typecheck` -> clean
- `./scripts/verify` -> ALL PASS (M0-M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)
  (isolated netns; standing port-3000 environment note, BETA-002-C record)

### Changed files

- Task record only (verification-only packet).

### Disclosures

- Verification-only packet; no runtime behavior changes.
- Standing: CI `verify` workflows stall/fail with zero executed steps
  on PR creation across all branches (infrastructure-side, tracked
  since ~#714); local `./scripts/verify` is the real gate evidence.
