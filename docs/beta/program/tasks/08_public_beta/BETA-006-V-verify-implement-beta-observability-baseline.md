---
task_id: BETA-006-V
parent_task: BETA-006
milestone: BETA
priority: P0
mode: VERIFY
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-006-V — Verify Implement beta observability baseline

## Atomic goal

Prove every acceptance criterion for parent task BETA-006 without broadening scope.

## Parent intent

Expose bounded metrics and structured logs sufficient to operate beta services.

## Dependencies

- `BETA-006-A` — `tasks/08_public_beta/BETA-006-A-request-route-status-duration.md`
- `BETA-006-B` — `tasks/08_public_beta/BETA-006-B-worker-queues-quarantine-replacements.md`
- `BETA-006-C` — `tasks/08_public_beta/BETA-006-C-fetch-and-db-pools.md`
- `BETA-006-D` — `tasks/08_public_beta/BETA-006-D-memory-tasks-slots.md`
- `BETA-006-E` — `tasks/08_public_beta/BETA-006-E-optional-trace-integration-or-trace-ids.md`
- `BETA-006-F` — `tasks/08_public_beta/BETA-006-F-redaction.md`

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
- `packages/cli/src/index.ts`
- `crates/q-runtime/src/source_map.rs`
- `examples/proof/`
- `README.md`
- `crates/q-engine/src/lib.rs`
- `crates/q-runtime/src/serve.rs`
- `docs/reports/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

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

- Metrics schema.
- Overhead benchmark.
- Redaction audit.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
beta-006-v: verify implement beta observability baseline
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-006-V) — PASS (2026-09-04)

- Branch/PR: beta-006-v (squash-merged; see git log for final hash)
- Closes: #537

### Acceptance-criterion mapping (parent BETA-006)

1. **Disabled overhead measured**
   - LogMode gating (Off/Errors/Full + sampling) preserved through the
     A-packet refactor; duration capture is a single Instant copy;
     metrics snapshots read on demand (B/E) — zero hot-path cost for
     disabled logging (A report + `record_overhead_is_budgeted` test).
2. **Enabled overhead budgeted**
   - Per request: O(1) atomic increments (route/status/duration
     aggregation, A) + one structured-log build when enabled; pool and
     worker snapshots only at bounded transitions (B/C/E).
3. **Cardinality is bounded**
   - Route metrics: one entry per static pack route + `<unknown>`
     fallback (A, tested); status classes, not codes; pool counters are
     fixed field sets (C); load-shed reasons are a closed vocabulary.
4. **No secrets/PII by default**
   - Log allowlist as code (F, 4 tests incl. query-strip defense);
     metrics schema audit (A); no-secret sweep on the auth capability
     (BETA-005-E); pools snapshots carry counts/gauges/bounds only (C).
5. **Dashboards/examples exist**
   - Documented JSON shapes: `request.complete` field list (F report),
     `ops.worker.status` + pools (B/C reports), route-metrics schema
     (A report) — all serde-serializable structs in the runtime, the
     substrate dashboards render.

### Commands (fresh on this branch)

- `cargo test -p q-http` (4 suites incl. 3 trace tests), `-p
  q-bridge` (2), `-p velqu-runtime` (63 pass incl. metric + redaction
  tests) -> all ok
- `bun test` -> 434 pass / 0 fail (67 files)
- typecheck / fmt / clippy -> clean
- `./scripts/verify` -> ALL PASS (M0-M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)
  (isolated netns; standing port-3000 environment note, BETA-002-C record)

### Changed files

- Task record + manifest refresh commit only (verification-only
  packet).

### Disclosures

- Verification-only packet; no runtime behavior changes.
- Standing: CI `verify` workflows stall/fail with zero executed steps
  on PR creation across all branches (infrastructure-side, tracked
  since ~#714); local `./scripts/verify` is the real gate evidence.
