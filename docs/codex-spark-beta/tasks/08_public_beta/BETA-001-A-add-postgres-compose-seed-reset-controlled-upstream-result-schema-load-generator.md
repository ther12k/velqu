---
task_id: BETA-001-A
parent_task: BETA-001
milestone: BETA
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-001-A — Add Postgres compose, seed/reset, controlled upstream, result schema, load generator, and report generator

## Atomic goal

Add Postgres compose, seed/reset, controlled upstream, result schema, load generator, and report generator.

## Parent intent

Turn the current SPEC/schema/workloads scaffold into deterministic infrastructure.

## Dependencies

- `G0-GATE` — `gates/G0-GATE.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/BETA.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`
- `context/components/schema-codecs.md`

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
- `crates/q-capabilities/src/lib.rs`
- `crates/q-http/src/lib.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Add Postgres compose, seed/reset, controlled upstream, result schema, load generator, and report generator.
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
cargo test -p q-http
```
```bash
cargo test -p q-schema-runtime
```
```bash
cargo test -p q-capabilities
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
beta-001-a: add postgres compose seed reset controlled upstream result s
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
## Completion record

- Status: **PASS**
- Deliverable: the `benchmarks/real-world/` scaffold is now deterministic, one-command infrastructure:
  - `compose.yaml` — pinned `postgres:17.5-alpine3.22`, benchmark-only creds, `pg_isready` healthcheck, tmpfs data (down discards state).
  - `postgres/reset.sql` + `postgres/seed.sql` + `reset.sh` — deterministic reset/seed (generate_series + modular arithmetic; no random functions). Live verified: users=1000, products=500, reviews=10000, electronics=100, orders=0 on every invocation.
  - `upstream.ts` — controlled upstream (W4): `GET /io?ms=N` (validated 0–1000ms) + `/health`; live-verified p50 ≈ 1.37/5.55/10.6/25.7ms for the 1/5/10/25ms cells.
  - `result-schema.ts` — `velqu-realworld-summary-v1` types + `validateRealWorldSummary` (cells complete, p50≤p95≤p99≤max, error counts ≤ totals, environment + sha256 config hashes required).
  - `load.ts` — fixed-duration load generator over `workloads.json` at concurrency 1/10/50/200; per-request raw JSONL rows retain errors/status mismatches (candidate failure recorded, never dropped); summary records environment (bun/os/arch/commit) + config hashes.
  - `report.ts` — deterministic report generator (per-workload tables, retained-failures section, protocol footer).
  - `run.sh` — one command: `prepare` (compose up --wait + reset) → `smoke` (upstream + 2s load-gen + result-schema validation + report). README documents usage/determinism/CI scope.
- Changed files: all under `benchmarks/real-world/` (compose.yaml, postgres/{reset,seed}.sql, reset.sh, upstream.ts, result-schema.ts, load.ts, report.ts, run.sh, README.md, *.test.ts) plus `.gitignore` (bulky smoke `raw.jsonl`/logs stay local; `benchmarks/raw/real-world/smoke/{summary.json,report.md}` retained as evidence). No runtime-crate, docs/reports, or benchmarks/manifest.json changes.
- Smoke results (live, this host): `./run.sh` end-to-end PASS — compose healthy, deterministic counts, W4 latencies timer-accurate, W1–W3 vs bare upstream correctly retained as status mismatches (failure-retention path exercised), `result-schema: PASS`. Evidence: `benchmarks/raw/real-world/smoke/summary.json`, `benchmarks/raw/real-world/smoke/report.md`.
- Fairness audit: none claimed here — candidate version pinning (BETA-001-B), fairness checks (C), and raw-sample retention policy (D) remain their own packets; smoke drives the upstream only.
- Tests and exact results: `cargo test -p q-engine-quickjs` PASS (98); `cargo test -p q-http` PASS (11); `cargo test -p q-schema-runtime` PASS (67); `cargo test -p q-capabilities` PASS (crate has no tests; clean build/exit 0); `bun test` PASS (104/104 incl. 15 new: result-schema 6, workloads 5, report 4); `bun run typecheck` PASS.
- Remaining risk / deferred by design: real candidates (Velqu/BETA-002) not yet driven; docker-dependent phases are operator-run (CI covers unit-tested pieces only); smoke `raw.jsonl` (44MB) intentionally untracked.
- Next dependency-ready task: BETA-001-B (Pin candidate versions) — #497.
- Working tree clean: yes after commit.
