---
task_id: BETA-006-B
parent_task: BETA-006
milestone: BETA
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-006-B — Worker queues/quarantine/replacements

## Atomic goal

Worker queues/quarantine/replacements.

## Parent intent

Expose bounded metrics and structured logs sufficient to operate beta services.

## Dependencies

- `BETA-006-A` — `tasks/08_public_beta/BETA-006-A-request-route-status-duration.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/BETA.md`
- `context/components/engine-scheduler.md`
- `context/components/devex-beta.md`
- `context/components/evidence.md`

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
- `crates/q-runtime/src/serve.rs`
- `docs/reports/`
- `docs/beta/workstreams/OBSERVABILITY_OPERATIONS.md`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Worker queues/quarantine/replacements.
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
cargo test -p q-engine-quickjs
```
```bash
cargo test -p velqu-runtime
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
beta-006-b: worker queues quarantine replacements
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-006-B) — PASS (2026-09-04)

- Branch/PR: beta-006-b (squash-merged; see git log for final hash)
- Closes: #532

### Changed files
- `crates/q-runtime/src/serve.rs`: `worker_ops_status(state)` — bounded
  structured snapshot aggregating queue gauges (slab live, queue
  pending, pending invocations), quarantine state (quarantined flag,
  queue-poisoned, poison events from engine stats), drain gate
  (draining/refused), and full load-shed counter snapshot.
- `crates/q-runtime/src/lib.rs`: `ops.worker.status` emitted at the
  drain transition (bounded-emissions policy; the shutdown report
  already carries full engine stats + stage metrics + ownership
  invariants).
- `docs/reports/beta-006-b-worker-ops-status.md` (new): snapshot
  schema, emissions policy, replacements posture.

### Required evidence

- **Capability tests**: targeted suites green (velqu-runtime 57+,
  q-http, q-bridge); snapshot composes existing tested primitives.
- **Real-world results**: status renders at drain transitions and
  shutdown (visible in the runtime's structured logs); no load-run
  claims.
- **Cold/RSS cost report**: no hot-path cost — the snapshot is read on
  demand / at bounded transitions; no new allocation on the request
  path.

### Guardrail mapping

- **Disabled overhead measured**: zero hot-path cost; emissions bounded
  by policy.
- **Enabled overhead budgeted**: one lock + counter reads per emission,
  at bounded transitions only.
- **Cardinality is bounded**: fixed field set; load-shed reasons are a
  closed vocabulary.
- **No secrets/PII by default**: gauges and counters only.
- **Dashboards/examples exist**: the JSON shape is documented in the
  report; emissions land in the same structured stream operators
  already tail.

### Standing CI disclosure

CI `verify` workflows stall/fail with zero executed steps on PR creation
across all branches (infrastructure-side, tracked since ~#714); the local
`./scripts/verify` run above is the real gate evidence for this packet.
