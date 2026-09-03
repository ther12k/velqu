---
task_id: BETA-002-C
parent_task: BETA-002
milestone: BETA
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-002-C — Add contract-response verification

## Atomic goal

Add contract-response verification.

## Parent intent

Provide Raw Rust, Elysia 2, Hono/Bun, and Fastify/Node implementations of identical contracts.

## Dependencies

- `BETA-002-B` — `tasks/08_public_beta/BETA-002-B-pin-versions.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/BETA.md`
- `context/components/devex-beta.md`

### Source files

- `AGENTS.md`
- `docs/beta/`
- `scripts/verify`
- `package.json`
- `.github/workflows/verify.yml`
- `packages/treaty/src/index.ts`
- `packages/contract/src/index.ts`
- `packages/testing/src/index.ts`
- `conformance/treaty/treaty.conformance.test.ts`
- `packages/compiler/src/emit.ts`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Add contract-response verification.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Candidates are semantically equivalent.
- No framework receives hidden advantages.
- All outputs pass contract fixtures.
- Version/hash metadata is captured.

## Targeted commands

```bash
bun test
```
```bash
bun run typecheck
```

## Required evidence for this microtask

- Candidate source.
- Parity tests.
- Fairness report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
beta-002-c: add contract response verification
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-002-C) — PASS (2026-09-03)

- Branch/PR: beta-002-c (squash-merged; see git log for final hash)
- Closes: #506

### Changed files
- `benchmarks/real-world/contract-fixtures.ts` (new): the 18-fixture contract
  matrix — W1 lookup (200 + 404 + 401 missing/malformed bearer), W2 order
  (201 first-order receipt + 400 empty items + 400 unknown product + 409
  insufficient stock + 401), W3 exact paginated body, W4 io/mixed/fanout
  (200/400/504 timeout/502 malformed/400 invalid n), and unknown-route 404.
  Expected bodies are computed from the shared `matched.ts` reference store,
  not hand-pinned; wall-clock `actualMs` uses a `{ __typeof__ }` sentinel
  resolved by `matchesExpected` (deep equality, key-order-insensitive).
- `benchmarks/real-world/verify-contract.ts` (new): boots every candidate
  server (`hono`, `elysia`, `bun-fetch` via Bun; `fastify` via Node) against
  the controlled upstream (spawned by the verifier or passed via
  `--upstream-url`), parses each `candidate.ready` line for the actual port,
  drives the full fixture matrix per candidate with the matched 5s request
  deadline, writes the per-fixture PASS/FAIL report, exits 1 on any mismatch.
  Fails fast with instructions when candidate deps or node are missing.
- `benchmarks/real-world/verify-contract.test.ts` (new): 8 deterministic
  tests — matrix completeness/coverage (all statuses 201/200/404/401/400/409/
  504/502 present), expected bodies derived from the shared store, W2 first-
  order receipt identity (`ord_1`), auth-header placement per fixture, and
  sentinel comparison semantics.
- `benchmarks/real-world/run.sh`: new `contracts` phase (upstream + verifier,
  report retained at `../raw/real-world/contracts/contract-verification.md`);
  `all` now runs prepare -> smoke -> contracts.
- `benchmarks/real-world/README.md` + `SPEC.md`: document the
  contract-response verification gate before timing.
- `.gitignore`: track `contracts/contract-verification.md` evidence, ignore
  the bulky `upstream.log` (same policy as the smoke phase).

### Candidate drift found and fixed
The first live run failed exactly as designed (2 mismatches across
4 candidates x 18 fixtures):
- `elysia.ts`: unknown routes returned Elysia's default RFC-9457 body
  `{"type":"not-found","title":"Not Found","status":404}` instead of the
  contract shape — fixed with an explicit `.all("/*")` handler returning
  `{error:"not found"}` (404).
- `bun-fetch.ts`: unknown routes returned `{error:"not found", path}` with an
  extra `path` field — fixed to `{error:"not found"}`.
After the fixes, the second live run passes 4/4 candidates x 18 fixtures.

### Required evidence

- **Candidate source**: all four candidates boot and serve the full W1..W4
  surface (verifier output retains each candidate's port and per-fixture
  results).
- **Parity tests**: `verify-contract.test.ts` 8/8 pass; live
  `bun verify-contract.ts` PASS (contract-verification.md retained).
- **Fairness report**: `run.sh contracts` is now a hard pre-timing gate;
  `./run.sh contracts` end-to-end PASS on this worktree.

### Commands

- `bun test benchmarks/real-world/verify-contract.test.ts` -> 8 pass / 0 fail
- `bun verify-contract.ts` (standalone, run 1) -> FAIL (2 drifts, see above)
- `bun verify-contract.ts` (run 2) -> PASS 4 candidates x 18 fixtures
- `./run.sh contracts` -> PASS
- `bun test` -> 344 pass / 0 fail (57 files)
- `bun run typecheck` -> clean
- `./scripts/verify` -> ALL PASS (M0-M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)

### Environment note (honest disclosure)

On this host, `./scripts/verify` currently fails in its clean-install step
solely because an unrelated process (the user's HIEstate Vite dev server,
pid 15837, started mid-session) occupies 127.0.0.1:3000, which the M4A-010-A
scaffold template's Treaty live test assumes is "a velqu dev server or
absent". Run inside an isolated network namespace (own loopback), where the
scaffold test takes its designed skip path, `./scripts/verify` is ALL PASS.
No test was weakened; the foreign responder satisfies neither the app
contract nor the skip predicate. Standing CI disclosure also applies (below).

### Guardrail mapping

- **Candidates are semantically equivalent**: proven per-fixture per-candidate
  by the verifier; the two drifts found are fixed in candidate source.
- **No framework receives hidden advantages**: identical contract answers on
  identical fixtures, including error paths, before any timing is allowed.
- **All outputs pass contract fixtures**: 18/18 fixtures x 4 candidates PASS.
- **Version/hash metadata is captured**: versions.json pins locked by
  BETA-002-B tests; verifier reports record command, ports, and results.

### Standing CI disclosure

CI `verify` workflows stall/fail with zero executed steps on PR creation
across all branches (infrastructure-side, tracked since ~#714); the local
`./scripts/verify` run above is the real gate evidence for this packet.
