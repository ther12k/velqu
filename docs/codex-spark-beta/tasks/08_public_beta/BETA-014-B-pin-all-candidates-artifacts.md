---
task_id: BETA-014-B
parent_task: BETA-014
milestone: BETA
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-014-B — Pin all candidates/artifacts

## Atomic goal

Pin all candidates/artifacts.

## Parent intent

Create an honest comparison for beta users.

## Dependencies

- `BETA-014-A` — `tasks/08_public_beta/BETA-014-A-include-cold-start-categories-warm-microbenchmarks-real-db-auth-i-o-cpu-jit-cros.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/BETA.md`
- `context/components/qpack-router.md`
- `context/components/devex-beta.md`
- `context/components/evidence.md`

### Source files

- `AGENTS.md`
- `docs/beta/`
- `scripts/verify`
- `package.json`
- `.github/workflows/verify.yml`
- `crates/q-pack/src/lib.rs`
- `crates/q-pack/tests/fuzz_pack.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `docs/specs/pack-format-v1.md`
- `benchmarks/harness/`
- `benchmarks/manifest.json`
- `benchmarks/raw/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Pin all candidates/artifacts.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Every number links to raw evidence.
- Fixture-specific wording.
- Velqu losses are included.
- No cloud cold-start claim from local process data.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
bun test
```
```bash
bun run typecheck
```

## Required evidence for this microtask

- Benchmark report.
- Raw archive.
- Methodology review.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
beta-014-b: pin all candidates artifacts
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-014-B) — PASS (2026-09-04)

- Branch/PR: beta-014-b (squash-merged; see git log for final hash)
- Closes: #592

### Behavior implemented

Verified that all candidate versions, drivers, images, and benchmark artifacts are strictly pinned:
- Competitor candidate pins in `benchmarks/real-world/versions.json` (`velqu: workspace:0.1.0`, `elysia: 2.0.0-beta.4`, `hono: 4.13.5`, `fastify: 5.12.1`) matching frozen `bun.lock` and `package.json` with zero floating ranges.
- Driver and runtime pins: `pg: 8.23.0`, `postgres: 3.4.9`, `bun: 1.4.0`, `nodeLtsMajor: 22`, `postgresImage: postgres:17.5-alpine3.22` (matching `compose.yaml`).
- Artifact digest pins: all QPack benchmark targets (`app-25.qpack` through `app-10000-bc.qpack`) pinned by SHA-256 and byte sizes in `benchmarks/manifest.json`.
- Tested via `benchmarks/real-world/versions.test.ts` (9 passed, 0 failed), `python3 scripts/validate-benchmark-evidence.py` (PASS), and `./scripts/validate-okf` (PASS).

### Changed files

- `docs/reports/beta-014-b-pin-candidates-artifacts.md`
- `docs/codex-spark-beta/tasks/08_public_beta/BETA-014-B-pin-all-candidates-artifacts.md`
- `docs/codex-spark-beta/STATUS.md`
- `docs/codex-spark-beta/indexes/TASK_INDEX.md`

### Gates

- `cargo test -p q-pack` — pass (100+2)
- `bun test` — 434 pass / 0 fail (67 files)
- `bun run typecheck` — pass
- `cargo fmt --all --check` — pass
- `cargo clippy --workspace --all-targets -- -D warnings` — pass
- `./scripts/validate-okf` — pass
- `./scripts/verify` — ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)

### Disclosures

- Candidate pinning verification only; no runtime binary behavior modified.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
