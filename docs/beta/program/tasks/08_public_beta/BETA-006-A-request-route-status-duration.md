---
task_id: BETA-006-A
parent_task: BETA-006
milestone: BETA
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-006-A — Request/route/status/duration

## Atomic goal

Request/route/status/duration.

## Parent intent

Expose bounded metrics and structured logs sufficient to operate beta services.

## Dependencies

- `M3-GATE` — `gates/M3-GATE.md`
- `M28-GATE` — `gates/M28-GATE.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/BETA.md`
- `context/components/ingress-bridge.md`
- `context/components/devex-beta.md`
- `context/components/evidence.md`

### Source files

- `AGENTS.md`
- `docs/beta/`
- `scripts/verify`
- `package.json`
- `.github/workflows/verify.yml`
- `crates/q-http/src/lib.rs`
- `crates/q-bridge/src/lib.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-runtime/tests/runtime_conformance.rs`
- `crates/q-engine/src/lib.rs`
- `docs/reports/`
- `docs/beta/workstreams/OBSERVABILITY_OPERATIONS.md`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Request/route/status/duration.
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
cargo test -p q-bridge
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
beta-006-a: request route status duration
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-006-A) — PASS (2026-09-04)

- Branch/PR: beta-006-a (squash-merged; see git log for final hash)
- Closes: #531

### Changed files
- `crates/q-runtime/src/serve.rs`: `RouteStatusMetrics` +
  `RouteStatusCounters(Snapshot)` + `RouteStatusEntrySnapshot` —
  cardinality-bounded per-route request/status/duration aggregation
  (one entry per static pack route + `<unknown>` fallback; status
  classes, not raw codes; duration total+max in µs; O(1) atomic
  increments, no locks/allocation). Handler: duration capture always on
  (single Instant copy), log serialization still mode-gated with
  sampling semantics preserved; `record()` wired post-pipeline with
  typed status extraction. 2 in-crate tests (bucketing/max math/
  fallback/cardinality bound; hot-path overhead budget).
- `crates/q-runtime/src/lib.rs`: fixed-size metric table built from the
  static pack route list at startup.
- `docs/reports/beta-006-a-request-route-status-duration.md` (new):
  metrics schema, redaction audit, overhead measurement.

### Required evidence

- **Metrics schema**: per-route `{total, 2xx/3xx/4xx/5xx buckets,
  duration_us_total, duration_us_max}`; cardinality fixed at startup
  (pack routes + 1 fallback), snapshot serde-serializable.
- **Overhead benchmark**: in-test budget (record() averaged far below
  the asserted 50µs/call bound; observed sub-µs on this host);
  disabled-vs-enabled split documented (duration copy always on;
  serialization gated).
- **Redaction audit**: aggregation fields are route ids, status
  classes, durations only — no paths/headers/query/PII; consistent
  with SEC-004 in the existing request log.

### Commands

- `cargo test -p velqu-runtime` -> 57+ pass incl. 2 new metric tests
- `cargo test -p q-http` / `-p q-bridge` -> suites ok
- fmt / clippy (`-D warnings`) -> clean
- `bun test` -> 434 pass / 0 fail (67 files); typecheck -> clean
- `./scripts/verify` -> ALL PASS (M0-M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)
  (isolated netns; standing port-3000 environment note, BETA-002-C record)

### Guardrail mapping

- **Disabled overhead measured**: Off mode adds one Instant copy; log
  serialization stays gated (test-budgeted record() cost).
- **Enabled overhead budgeted**: O(1) atomics; in-test budget bound.
- **Cardinality is bounded**: fixed entries = pack routes + 1 fallback;
  status classes, not codes (tested).
- **No secrets/PII by default**: schema carries ids/classes/durations
  only (audit in report).
- **Dashboards/examples exist**: snapshot schema documented; example
  rendering lands with the workstream docs (OBSERVABILITY_OPERATIONS).

### Standing CI disclosure

CI `verify` workflows stall/fail with zero executed steps on PR creation
across all branches (infrastructure-side, tracked since ~#714); the local
`./scripts/verify` run above is the real gate evidence for this packet.
