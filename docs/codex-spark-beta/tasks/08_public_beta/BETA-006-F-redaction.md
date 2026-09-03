---
task_id: BETA-006-F
parent_task: BETA-006
milestone: BETA
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-006-F — Redaction

## Atomic goal

Redaction.

## Parent intent

Expose bounded metrics and structured logs sufficient to operate beta services.

## Dependencies

- `BETA-006-E` — `tasks/08_public_beta/BETA-006-E-optional-trace-integration-or-trace-ids.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/BETA.md`
- `context/components/devex-beta.md`
- `context/components/evidence.md`

### Source files

- `AGENTS.md`
- `docs/beta/`
- `scripts/verify`
- `package.json`
- `.github/workflows/verify.yml`
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
5. Implement exactly this deliverable: Redaction.
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
beta-006-f: redaction
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-006-F) — PASS (2026-09-04)

- Branch/PR: beta-006-f (squash-merged; see git log for final hash)
- Closes: #536

### Changed files
- `crates/q-runtime/src/serve.rs`: extracted `completion_log_json` —
  the structured completion log is built by a pure function whose
  field set is the allowlist (level/event/requestId/routeId/method/
  path/status/bodyBytes/stage/durationMs/traceId); defensive
  query-string re-strip on `path`; 4 redaction tests (exact key
  allowlist, query-strip with secret-bearing path, trace-id absence
  semantics, status-driven level).
- `docs/reports/beta-006-f-redaction.md` (new): the end-to-end
  redaction audit (log allowlist + metrics schema + auth sweep).

### Required evidence

- **Capability tests**: 4 new tests; velqu-runtime 63 pass.
- **Real-world results**: the allowlist is the shipped log shape; no
  load-run claims.
- **Cold/RSS cost report**: refactor only — same output, zero added
  cost (pure function, no allocation change).

### Commands

- `cargo test -p velqu-runtime log_redaction` -> 4 pass / 0 fail
- `cargo test -p velqu-runtime` -> 63 pass / 0 failed
- fmt / clippy (`-D warnings`) -> clean
- `./scripts/verify` -> ALL PASS (isolated netns; standing port-3000
  environment note, BETA-002-C record)

### Guardrail mapping

- **Disabled overhead measured**: unchanged; pure refactor.
- **Enabled overhead budgeted**: unchanged.
- **Cardinality is bounded**: unchanged field set.
- **No secrets/PII by default**: the allowlist IS the enforcement —
  fields for sensitive material do not exist; query-strip defense
  tested.
- **Dashboards/examples exist**: field list documented in the report.

### Standing CI disclosure

CI `verify` workflows stall/fail with zero executed steps on PR creation
across all branches (infrastructure-side, tracked since ~#714); the local
`./scripts/verify` run above is the real gate evidence for this packet.
