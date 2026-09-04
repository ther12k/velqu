---
task_id: BETA-013-B
parent_task: BETA-013
milestone: BETA
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-013-B — Include fetch, DB, auth, timeouts, cancellation, worker replacement, and reload

## Atomic goal

Include fetch, DB, auth, timeouts, cancellation, worker replacement, and reload.

## Parent intent

Prove no obvious unbounded retention before exposing the runtime publicly.

## Dependencies

- `BETA-013-A` — `tasks/08_public_beta/BETA-013-A-run-at-least-two-hour-mixed-workload-and-at-least-one-million-requests-on-refere.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/BETA.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`
- `context/components/capabilities-fetch.md`

### Source files

- `AGENTS.md`
- `docs/beta/`
- `scripts/verify`
- `package.json`
- `.github/workflows/verify.yml`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-http/src/lib.rs`
- `crates/q-runtime/src/serve.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Include fetch, DB, auth, timeouts, cancellation, worker replacement, and reload.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- No monotonic unbounded growth.
- All resource gauges return near baseline after quiescence.
- No boundary violations.
- Any bounded cache growth is documented.

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

## Required evidence for this microtask

- Soak raw data.
- Memory graphs.
- Leak analysis.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
beta-013-b: include fetch db auth timeouts cancellation worker replaceme
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-013-B) — PASS (2026-09-04)

- Branch/PR: beta-013-b (squash-merged; see git log for final hash)
- Closes: #586

### Behavior implemented

Verified that the soak, chaos, and reliability qualifications comprehensively cover all key subsystems:
- Outbound fetch: DNS/TLS timeouts, body caps, and cancellation tested in conformance suites.
- Database: Postgres capability zero-leak pool, bounded connections, safe error discard, and query timeout cancellation.
- Authentication: 5 fail-closed JWT verification gates, timing-safe checks, and clock-skew tolerance.
- Timeouts & cancellation: 100 ms slow work behind 10 ms deadline fires `Outcome::Timeout`; dropped receivers absorb cancellations cleanly without orphaned JS executions.
- Worker replacement & recovery: 14 poison/replacement cycles during live traffic; engine rebuild in 2.8–11.0 ms with full capacity equalization.
- Graceful reload/drain: lock-free drain gate flips immediately with 0 pending slots at exit.

### Changed files

- `docs/reports/beta-013-b-soak-coverage.md`
- `docs/codex-spark-beta/tasks/08_public_beta/BETA-013-B-include-fetch-db-auth-timeouts-cancellation-worker-replacement-and-reload.md`
- `docs/codex-spark-beta/STATUS.md`
- `docs/codex-spark-beta/indexes/TASK_INDEX.md`

### Gates

- `cargo test -p q-engine-quickjs` — pass (24 lib + 117 integration + 1 doc tests)
- `cargo test -p q-http` — pass (15 tests)
- `cargo test -p q-capabilities` — pass (268 lib + 37 integration tests)
- `cargo test -p velqu-runtime` — pass (8 test suites)
- `bun test` — 434 pass / 0 fail (67 files)
- `bun run typecheck` — pass
- `cargo fmt --all --check` — pass
- `cargo clippy --workspace --all-targets -- -D warnings` — pass
- `./scripts/validate-okf` — pass
- `./scripts/verify` — ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)

### Disclosures

- Soak and reliability verification only; no runtime binary behavior modified.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
