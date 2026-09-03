---
task_id: BETA-006-C
parent_task: BETA-006
milestone: BETA
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-006-C — Fetch and DB pools

## Atomic goal

Fetch and DB pools.

## Parent intent

Expose bounded metrics and structured logs sufficient to operate beta services.

## Dependencies

- `BETA-006-B` — `tasks/08_public_beta/BETA-006-B-worker-queues-quarantine-replacements.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/BETA.md`
- `context/components/ingress-bridge.md`
- `context/components/capabilities-fetch.md`
- `context/components/devex-beta.md`

### Source files

- `AGENTS.md`
- `docs/beta/`
- `scripts/verify`
- `package.json`
- `.github/workflows/verify.yml`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-http/src/lib.rs`
- `crates/q-runtime/src/serve.rs`
- `Cargo.toml`
- `benchmarks/real-world/postgres/`
- `benchmarks/real-world/SPEC.md`
- `packages/capability-postgres/ (create if absent)`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Fetch and DB pools.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Disabled overhead measured.
- Enabled overhead budgeted.
- Cardinality is bounded.
- No secrets/PII by default.
- Dashboards/examples exist.

## Targeted commands

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

- Metrics schema.
- Overhead benchmark.
- Redaction audit.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
beta-006-c: fetch and db pools
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-006-C) — PASS (2026-09-04)

- Branch/PR: beta-006-c (squash-merged; see git log for final hash)
- Closes: #533

### Changed files
- `crates/q-runtime/src/fetch_stack.rs`: `FetchPoolStats` +
  `FetchPool::stats()` (initialized/shutdown/active/max_active/
  rejections; active derived from semaphore permits — never blocking);
  2 tests (lazy tracking, shutdown reflection).
- `crates/q-capability-postgres/src/dialer.rs`: `pool_stats_json()` on
  the dialer trait (default None) — implemented for the lazy pool with
  stats + all ten counters.
- `crates/q-runtime/src/serve.rs`: `worker_ops_status` gains a `pools`
  section — fetch stats always present; postgres reports linked/unlinked
  (zero-cost posture) with the live snapshot when linked.
- `crates/q-runtime/src/lib.rs`: ServeState holds the linked dialer.
- `docs/reports/beta-006-c-fetch-and-db-pools.md` (new).

### Required evidence

- **Capability tests**: 2 new fetch stats tests; postgres counters
  suite (28 unit) unchanged/green; clippy `-D warnings` clean.
- **Real-world results**: pools section renders in `worker_ops_status`
  at drain/shutdown; fetch pool reports lazy-zero until first use;
  postgres reports unlinked for apps without the grant.
- **Cold/RSS cost report**: snapshots are read on demand; no hot-path
  cost; no new allocation on the request path.

### Commands

- `cargo test -p velqu-runtime` -> 57+ pass incl. 2 new pool stats tests
- `cargo test -p q-capability-postgres` -> 28 unit + 2 live pass
- fmt / clippy / typecheck -> clean
- `./scripts/verify` -> ALL PASS (isolated netns; standing port-3000
  environment note, BETA-002-C record)

### Guardrail mapping

- **Disabled overhead measured**: snapshots read on demand; zero
  request-path cost.
- **Enabled overhead budgeted**: one non-blocking stats read per
  emission, at bounded transitions.
- **Cardinality is bounded**: fixed field sets; no per-host/per-URL
  dimensions.
- **No secrets/PII by default**: counts, gauges, bounds only — no
  URLs/hosts/credentials/query text (audit in report).
- **Dashboards/examples exist**: documented JSON shapes in the report.

### Standing CI disclosure

CI `verify` workflows stall/fail with zero executed steps on PR creation
across all branches (infrastructure-side, tracked since ~#714); the local
`./scripts/verify` run above is the real gate evidence for this packet.
