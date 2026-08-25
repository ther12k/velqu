---
task_id: BETA-001-B
parent_task: BETA-001
milestone: BETA
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-001-B — Pin candidate versions

## Atomic goal

Pin candidate versions.

## Parent intent

Turn the current SPEC/schema/workloads scaffold into deterministic infrastructure.

## Dependencies

- `BETA-001-A` — `tasks/08_public_beta/BETA-001-A-add-postgres-compose-seed-reset-controlled-upstream-result-schema-load-generator.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/BETA.md`
- `context/components/engine-scheduler.md`
- `context/components/schema-codecs.md`
- `context/components/devex-beta.md`

### Source files

- `AGENTS.md`
- `docs/beta/`
- `scripts/verify`
- `package.json`
- `.github/workflows/verify.yml`
- `crates/q-schema-runtime/src/lib.rs`
- `crates/q-schema-runtime/tests/fuzz_validator.rs`
- `crates/q-engine-quickjs/src/convert.rs`
- `packages/schema/src/index.ts`
- `conformance/schema/schema.conformance.test.ts`
- `packages/cli/src/index.ts`
- `crates/q-runtime/src/source_map.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Pin candidate versions.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- One command prepares/runs/reports.
- Dataset resets deterministically.
- Candidate failure is retained.
- Protocol records environment and hashes.

## Targeted commands

```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p q-schema-runtime
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

- Harness source.
- Smoke results.
- Fairness audit.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
beta-001-b: pin candidate versions
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
## Completion record

- Status: **PASS**
- Deliverable: candidate/toolchain versions pinned in `benchmarks/real-world/versions.json` (`velqu-realworld-versions-v1`): Postgres `17.5-alpine3.22` (matches compose.yaml), Bun `1.4.0` (matches CI pin), Node LTS line 22 (exact patch recorded per run), candidates velqu (workspace/commit-pinned), elysia `2.0.0-beta.4` (matches baseline package.json), hono `4.13.4`, fastify `5.12.1`, drivers pg `8.23.0` / postgres `3.4.9`. Registry versions resolved from npm on 2026-08-25 (exact, no ranges).
- Wiring: `load.ts` hashes `versions.json` into every summary (`configHashes.versions`) and records `environment.nodeVersion`; `result-schema.ts` requires both (validator + tests updated); `run.sh prepare` echoes the pin manifest; smoke evidence regenerated with the new fields (env now carries `nodeVersion: v24.11.0`, `commit: 31ad65d`).
- Changed files: `benchmarks/real-world/{versions.json,versions.test.ts,load.ts,result-schema.ts,result-schema.test.ts,run.sh}` + regenerated `benchmarks/raw/real-world/smoke/{summary.json,report.md}`.
- Tests and exact results: `bun test benchmarks/real-world` 22/22 (7 new version-pin tests incl. cross-file agreement with compose.yaml, CI workflow, and elysia baseline); full `bun test` 111/111; `bun run typecheck` PASS; `cargo test -p q-engine-quickjs` PASS (98); `cargo test -p q-schema-runtime` PASS (67); `cargo test -p velqu-runtime` PASS (28/28 after building the debug `velqu-bytecode` tool the bytecode tests spawn — precondition, not a code change).
- Smoke results: `./run.sh smoke` PASS — summary validated with versions hash; report regenerated.
- Fairness audit: version parity is the fairness prerequisite this packet owns; full fairness checks remain BETA-001-C.
- Remaining risk / deferred by design: candidate implementations (BETA-002) consume these pins; Node patch drift within LTS line is recorded per run rather than pinned.
- Next dependency-ready task: BETA-001-C (Define fairness checks) — #498.
- Working tree clean: yes after commit.
