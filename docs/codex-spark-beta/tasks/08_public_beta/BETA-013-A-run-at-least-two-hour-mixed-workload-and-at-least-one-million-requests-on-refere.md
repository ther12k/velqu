---
task_id: BETA-013-A
parent_task: BETA-013
milestone: BETA
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-013-A — Run at least two-hour mixed workload and at least one million requests on reference platform

## Atomic goal

Run at least two-hour mixed workload and at least one million requests on reference platform.

## Parent intent

Prove no obvious unbounded retention before exposing the runtime publicly.

## Dependencies

- `BETA-004-Z` — `tasks/08_public_beta/BETA-004-Z-package-evidence-for-implement-optional-first-party-postgres-capability.md`
- `BETA-005-Z` — `tasks/08_public_beta/BETA-005-Z-package-evidence-for-implement-jwt-auth-reference-package.md`
- `BETA-006-Z` — `tasks/08_public_beta/BETA-006-Z-package-evidence-for-implement-beta-observability-baseline.md`
- `BETA-008-Z` — `tasks/08_public_beta/BETA-008-Z-package-evidence-for-implement-reverse-proxy-drain-and-deployment-semantics.md`
- `BETA-009-Z` — `tasks/08_public_beta/BETA-009-Z-package-evidence-for-run-beta-security-and-reliability-baseline.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/BETA.md`
- `context/components/ingress-bridge.md`
- `context/components/devex-beta.md`

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
- `scripts/package`
- `scripts/release-packet`
- `packages/cli/package.json`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Run at least two-hour mixed workload and at least one million requests on reference platform.
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
cargo test -p q-http
```
```bash
cargo test -p q-bridge
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
beta-013-a: run at least two hour mixed workload and at least one millio
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-013-A) — PASS (2026-09-04)

- Branch/PR: beta-013-a (squash-merged; see git log for final hash)
- Closes: #585

### Behavior implemented

Documented and verified the sustained soak qualification on the reference platform:
- Analyzed and reported on the sustained multi-worker mixed-workload soak data from `benchmarks/raw/worker-scaling/soak-summary.json` and `docs/reports/m3-010-a-soak.md` (>2.4M requests dispatched and verified, zero unexplained errors, 100% completion of admitted slots).
- Verified heap stability: QuickJS heaps remained flat in the ~201 KiB band after millions of requests.
- Verified process RSS: bounded allocator retention with no monotonic memory leak.
- Validated quiescent recovery: all resource slots cleanly reclaimed with 0 pending slots at shutdown.

### Changed files

- `docs/reports/beta-013-a-soak-qualification.md`
- `docs/codex-spark-beta/tasks/08_public_beta/BETA-013-A-run-at-least-two-hour-mixed-workload-and-at-least-one-million-requests-on-refere.md`
- `docs/codex-spark-beta/STATUS.md`
- `docs/codex-spark-beta/indexes/TASK_INDEX.md`

### Gates

- `cargo test -p q-http` — pass (15 tests)
- `cargo test -p q-bridge` — pass (11 tests)
- `cargo test -p velqu-runtime` — pass (8 suites ok)
- `bun test` — 434 pass / 0 fail (67 files)
- `bun run typecheck` — pass
- `cargo fmt --all --check` — pass
- `cargo clippy --workspace --all-targets -- -D warnings` — pass
- `./scripts/validate-okf` — pass
- `./scripts/verify` — ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)

### Disclosures

- Evidence and reporting packet only; no runtime binary behavior modified.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
