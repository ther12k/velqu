---
task_id: BETA-004-Z
parent_task: BETA-004
milestone: BETA
priority: P0
mode: EVIDENCE
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-004-Z — Package evidence for Implement optional first-party Postgres capability

## Atomic goal

Create source-backed evidence and handoff for parent task BETA-004; update status only if verification passed.

## Parent intent

Provide a real database story without enlarging core.

## Dependencies

- `BETA-004-V` — `tasks/08_public_beta/BETA-004-V-verify-implement-optional-first-party-postgres-capability.md`

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
- `benchmarks/harness/`
- `benchmarks/manifest.json`
- `benchmarks/raw/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

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
```bash
./scripts/validate-okf
```

## Required evidence for this microtask

- Capability tests.
- Real-world results.
- Cold/RSS cost report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
beta-004-z: package evidence for implement optional first party postgres
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-004-Z) — PASS (2026-09-04)

- Branch/PR: beta-004-z (squash-merged; see git log for final hash)
- Closes: #523
- Parent verification: BETA-004-V PASS (PR #1122); this packet packages
  the source-backed evidence across all child packets (A through F + V)
  and flips parent task BETA-004 to PASS in
  `docs/beta/04_TASK_LEDGER.md`.

### Evidence package

- **Implementation packets (squash-merged):**
  - BETA-004-A (PR #1116): capability ABI wiring — `runtime:postgres` v1
    ABI model (q-capabilities::postgres), `@velqu/capability-postgres`
    SDK (fail-closed, parameterized-only), compiler grant mapping,
    cold/RSS zero-cost report.
  - BETA-004-B (PR #1117): lazy bounded pool crate
    (`q-capability-postgres`) — zero-I/O construction, typed
    timeouts/capacity, idle hygiene, shutdown gate; 12 unit + 2 live
    tests.
  - BETA-004-C (PR #1118): parameterized query layer + transactions —
    extended-protocol binding, width-matched types, pre-wire
    placeholder validation, rollback-on-error (incl. early return).
  - BETA-004-D (PR #1119): engine boundary — `__velquPostgresQuery`
    native (phase guards, op registry, owner cancellation), discard-on-
    error leases, runtime startup linking with fail-closed requirement.
  - BETA-004-E (PR #1120): env-configurable pool limits (fail-closed)
    + `PoolCounters` observability + `postgres_ops_*` EngineStats.
  - BETA-004-F (PR #1121): no-ORM surface freeze + capability
    documentation (`docs/beta/POSTGRES-CAPABILITY.md`).
  - BETA-004-V (PR #1122): verification closure; fresh live re-runs
    reproduce.

### Required evidence

- **Capability tests**: crate total 28 unit + 2 live (env-gated;
  re-run fresh on this branch against the benchmark stack); 4 engine
  fail-closed/dialer tests; surface-freeze + env-config tests in the
  TS package; ABI lifecycle tests in q-capabilities.
- **Real-world results**: live HTTP -> QuickJS -> pool -> real Postgres
  round trip (D packet fixture); pool live tests green on this branch
  (SCRAM, seeded 1,000 users). Full W1/W2/W3 load runs are
  BETA-013/014 scope.
- **Cold/RSS cost report**: BETA-004-A report + lazy-construction unit
  proof (B); unchanged by C-F.

### Parent guardrail proofs

1. **App without Postgres pays zero dependency/init cost** — no grant,
   no requirement, no pool, unchanged cold/RSS (measured, A).
2. **Queries are parameterized** — extended-protocol binding only;
   interpolation structurally unreachable (C); freeze-tested (F).
3. **Timeout cancels/releases connection safely** — deadline bounds
   acquire+execution; discard-on-error leases (D).
4. **Pool exhaustion is bounded** — ceiling 1..=100 fail closed;
   env-configurable within bounds; typed `AtCapacity` (B/E).
5. **W1/W2/W3 workloads pass** — end-to-end fixture path verified live;
   full load runs are canonical-report scope (BETA-013/014), stated
   without overclaim.

### Gate results (fresh on this branch)

- `cargo test -p q-capability-postgres` -> 28 unit + 2 live pass
- `cargo test -p q-pack` (3), `-p q-engine-quickjs` (3, 137 pass),
  `-p q-capabilities` (8) -> all suites ok
- fmt / clippy (`-D warnings`) / typecheck -> clean
- `bun test` -> 384 pass / 0 fail (62 files)
- `./scripts/verify` -> ALL PASS (M0-M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)
- `./scripts/validate-okf` -> PASS
  (verify run inside an isolated netns; standing port-3000 environment
  note, BETA-002-C record. No test weakened.)

### Ledger

- `docs/beta/04_TASK_LEDGER.md`: BETA-004 flipped TODO -> **PASS**.
- STATUS.md and TASK_INDEX.md updated to PASS (BETA-004-Z row).
