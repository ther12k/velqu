---
task_id: BETA-002-V
parent_task: BETA-002
milestone: BETA
priority: P1
mode: VERIFY
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-002-V — Verify Implement matched competitor candidates

## Atomic goal

Prove every acceptance criterion for parent task BETA-002 without broadening scope.

## Parent intent

Provide Raw Rust, Elysia 2, Hono/Bun, and Fastify/Node implementations of identical contracts.

## Dependencies

- `BETA-002-A` — `tasks/08_public_beta/BETA-002-A-match-sql-pool-jwt-timeouts-logging-responses-compression-and-deployment-limits.md`
- `BETA-002-B` — `tasks/08_public_beta/BETA-002-B-pin-versions.md`
- `BETA-002-C` — `tasks/08_public_beta/BETA-002-C-add-contract-response-verification.md`
- `BETA-002-D` — `tasks/08_public_beta/BETA-002-D-document-unavoidable-differences.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/BETA.md`
- `context/components/qpack-router.md`
- `context/components/devex-beta.md`

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
- `packages/treaty/src/index.ts`
- `packages/contract/src/index.ts`
- `packages/testing/src/index.ts`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- Candidates are semantically equivalent.
- No framework receives hidden advantages.
- All outputs pass contract fixtures.
- Version/hash metadata is captured.

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

- Candidate source.
- Parity tests.
- Fairness report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
beta-002-v: verify implement matched competitor candidates
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-002-V) — PASS (2026-09-03)

- Branch/PR: beta-002-v (squash-merged; see git log for final hash)
- Closes: #508

### Acceptance-criterion mapping (parent BETA-002)

1. **Candidates are semantically equivalent**
   - Source: `benchmarks/real-world/candidates/matched.ts` + `matched.cjs`
     (single shared contract: SQL, pool, auth, timeouts, response shapes).
   - Positive tests: `candidates/parity.test.ts` (7 tests pinning SQL text,
     pool bounds, JWT rejection matrix, limits, W1/W2/W3 store contracts);
     `contract-fixtures.ts` matrix (18 fixtures) driven live against every
     candidate by `verify-contract.ts` — PASS 4/4 candidates x 18/18
     fixtures on this branch (retained at
     `/tmp/beta-002-v-contract-verification.md` during the run).
   - Negative tests: `verify-contract.test.ts` (matrix coverage + comparison
     semantics); BETA-002-C's first live run demonstrated the failure mode
     (2 drifts caught, then fixed in candidate source).
2. **No framework receives hidden advantages**
   - Source: `MATCHED_CONFIG` pins logging off, compression off, keep-alive
     on, single worker, identical timeouts; all candidates import it.
   - Positive tests: `parity.test.ts` "enforces identical timeouts, logging,
     compression, and deployment limits"; `versions.test.ts` (9 tests) pins
     registry = package.json = frozen bun.lock, so no candidate can resolve
     a different framework version.
   - Documentation: `DIFFERENCES.md` declares the uncompensated differences
     and is sha256-pinned per run (`configHashes.differences`) and compared
     by `fairness.ts` — drift fails the audit (fairness.test.ts +2).
3. **All outputs pass contract fixtures**
   - Live evidence: `bun verify-contract.ts` PASS in this worktree
     (hono/elysia/bun-fetch on Bun, fastify on Node; controlled upstream).
   - Deterministic tests: `verify-contract.test.ts` 8/8.
4. **Version/hash metadata is captured**
   - Source: `versions.json`, `candidates/package.json` + `bun.lock`,
     summary `configHashes` (spec/workloads/schema/seed/versions/
     differences), `result-schema.ts` required-hex64 validation,
     `fairness.ts` cross-candidate hash comparison.

### Commands

- `bun test benchmarks/real-world benchmarks/real-world/candidates` -> 55 pass / 0 fail (8 files)
- `bun verify-contract.ts` -> PASS (4 candidates x 18 fixtures)
- `cargo test -p q-pack` -> 102 passed / 0 failed (3 suites)
- `cargo fmt --all --check` -> clean
- `cargo clippy --workspace --all-targets -- -D warnings` -> clean
- `bun test` -> 346 pass / 0 fail (57 files)
- `bun run typecheck` -> clean
- `./scripts/verify` -> ALL PASS (M0-M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)
  (isolated netns; standing environment note: unrelated user process on
  127.0.0.1:3000 trips the M4A-010-A scaffold live-Treaty skip predicate on
  the shared host. No test weakened.)

### Changed files

- `docs/codex-spark-beta/tasks/08_public_beta/BETA-002-V-verify-implement-matched-competitor-candidates.md`
  (verification closure only; no production or harness changes).

### Disclosures

- Verification-only packet; no runtime behavior changes.
- Environment note as above (host port-3000 collision documented in
  BETA-002-C record).
- Standing: CI `verify` workflows stall/fail with zero executed steps on PR
  creation across all branches (infrastructure-side, tracked since ~#714);
  local `./scripts/verify` is the real gate evidence.
