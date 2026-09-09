---
task_id: BETA-001-D
parent_task: BETA-001
milestone: BETA
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-001-D — Keep raw samples

## Atomic goal

Keep raw samples.

## Parent intent

Turn the current SPEC/schema/workloads scaffold into deterministic infrastructure.

## Dependencies

- `BETA-001-C` — `tasks/08_public_beta/BETA-001-C-define-fairness-checks.md`

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
5. Implement exactly this deliverable: Keep raw samples.
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
beta-001-d: keep raw samples
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
## Completion record

- Status: **PASS**
- Deliverable: raw-sample retention implemented in `benchmarks/real-world/retain.ts`: every run's complete per-request raw JSONL is retained as a deterministic gzip archive (`raw.jsonl.gz`, gzip mtime pinned to 0 — byte-reproducible) committed beside `summary.json`, with a `RETENTION.md` manifest recording row count, both sizes, and both sha256 hashes plus a one-line verification command. Policy: uncompressed rows and logs stay local-only; archives are evidence.
- Wiring: `run.sh smoke` now runs retention after load generation; `.gitignore` keeps ignoring `raw.jsonl`/logs while archives ship.
- Retention evidence (live): committed smoke run now includes `benchmarks/raw/real-world/smoke/raw.jsonl.gz` — 323,505 rows, 35,906,767 raw bytes -> 1,794,557 gz bytes, raw sha256 `7aaecb20f951680106e1ef1f41e72935f2b74012c332ef214fdae0185f71628c`, gz sha256 `ca2ded2124d9522b45e0c68b8506d662da662348270a0a157c76c949e9a8e7ec`, manifest `RETENTION.md` alongside.
- Changed files: `benchmarks/real-world/{retain.ts,retain.test.ts,run.sh,README.md}` + `.gitignore` + regenerated `benchmarks/raw/real-world/smoke/` evidence (summary/report + new archive + manifest).
- Tests and exact results: `bun test benchmarks/real-world` 36/36 (5 new retention tests: deterministic gzip bytes, lossless round-trip + row counting, verifyArchive accept/reject, manifest/hash consistency on disk, full-rebuild reproducibility); full `bun test` 125/125; `bun run typecheck` PASS; `cargo test -p q-engine-quickjs` PASS (98); `cargo test -p q-schema-runtime` PASS (67); `cargo test -p velqu-runtime` PASS (28/28).
- Remaining risk / deferred by design: very large candidate runs (full 10s x 4-concurrency matrix) may warrant per-candidate size budgets before BETA-003 evidence collection.
- Next dependency-ready task: BETA-001-V (Verify Make the real-world benchmark harness executable) — #500, once BETA-001-A..D are merged.
- Working tree clean: yes after commit.
