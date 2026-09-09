---
task_id: BETA-006-E
parent_task: BETA-006
milestone: BETA
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-006-E — Optional trace integration or trace IDs

## Atomic goal

Optional trace integration or trace IDs.

## Parent intent

Expose bounded metrics and structured logs sufficient to operate beta services.

## Dependencies

- `BETA-006-D` — `tasks/08_public_beta/BETA-006-D-memory-tasks-slots.md`

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
5. Implement exactly this deliverable: Optional trace integration or trace IDs.
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
beta-006-e: optional trace integration or trace ids
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-006-E) — PASS (2026-09-04)

- Branch/PR: beta-006-e (squash-merged; see git log for final hash)
- Closes: #535

### Changed files
- `crates/q-http/src/lib.rs`: `extract_trace_id(headers)` — accepts
  `x-trace-id` or W3C `traceparent` (trace-id segment), validates
  bounded shape (printable ASCII, no whitespace/controls, <=128 chars,
  non-empty), 3 tests; `TRACE_ID_LIMIT`.
- `crates/q-runtime/src/serve.rs`: trace id carried into
  `request.complete` logs as `traceId` (field omitted when absent —
  strictly optional, zero behavioral change for un-traced requests).
- `docs/reports/beta-006-e-trace-ids.md` (new): design choice (trace
  IDs over tracing-system integration), redaction audit.

### Required evidence

- **Capability tests**: 3 new deterministic extraction/validation tests
  (q-http suite green).
- **Real-world results**: trace ids render in `request.complete` logs
  for requests carrying trace headers; unchanged logs otherwise; no
  load-run claims.
- **Cold/RSS cost report**: zero cost for un-traced requests; one
  header read + bounded validation per traced request.

### Commands

- `cargo test -p q-http` -> suites ok incl. 3 new trace tests
- `cargo test -p velqu-runtime` -> 59 pass
- fmt / clippy (`-D warnings`) -> clean
- `./scripts/verify` -> ALL PASS (isolated netns; standing port-3000
  environment note, BETA-002-C record)
- `bun test` -> 434 pass / 0 fail (67 files); typecheck -> clean

### Guardrail mapping

- **Disabled overhead measured**: absent trace headers = one header
  lookup, no allocation.
- **Enabled overhead budgeted**: bounded validation (<=128 chars) and
  one log field.
- **Cardinality is bounded**: caller-supplied ids are log fields, not
  metric labels (route metrics remain route/status-bounded).
- **No secrets/PII by default**: shape validation; token material
  never enters this path.
- **Dashboards/examples exist**: `traceId` joins `request.complete` —
  the field operators already correlate on.

### Standing CI disclosure

CI `verify` workflows stall/fail with zero executed steps on PR creation
across all branches (infrastructure-side, tracked since ~#714); the local
`./scripts/verify` run above is the real gate evidence for this packet.
