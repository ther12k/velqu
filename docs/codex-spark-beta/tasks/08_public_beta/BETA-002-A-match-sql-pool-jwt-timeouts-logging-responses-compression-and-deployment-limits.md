---
task_id: BETA-002-A
parent_task: BETA-002
milestone: BETA
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-002-A — Match SQL, pool, JWT, timeouts, logging, responses, compression, and deployment limits

## Atomic goal

Match SQL, pool, JWT, timeouts, logging, responses, compression, and deployment limits.

## Parent intent

Provide Raw Rust, Elysia 2, Hono/Bun, and Fastify/Node implementations of identical contracts.

## Dependencies

- `BETA-001-Z` — `tasks/08_public_beta/BETA-001-Z-package-evidence-for-make-the-real-world-benchmark-harness-executable.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/BETA.md`
- `context/components/ingress-bridge.md`
- `context/components/capabilities-fetch.md`
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
- `crates/q-capabilities/src/lib.rs`
- `crates/q-http/src/lib.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Match SQL, pool, JWT, timeouts, logging, responses, compression, and deployment limits.
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
cargo test -p q-http
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
beta-002-a: match sql pool jwt timeouts logging responses compression an
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

---

## Result (BETA-002-A) — PASS (2026-09-01)

- Branch/PR: beta-002-a (squash-merged; see git log for final hash)
- Closes: #504

### Changed files
- `benchmarks/real-world/candidates/matched.ts` (new): canonical matched contract
  (`MATCHED_CONFIG`) and `DeterministicStore` reference implementation used by all
  TypeScript candidates — identical SQL statements (W1 lookup, W2 stock check / order
  insert / order-item insert / stock decrement, W3 paginated join+aggregation), pool
  bounds (20 conns, 5s connect, 30s idle), JWT HS256 + benchmark token and typed 401
  rejection, 5s request / 100ms upstream timeouts, `off` logging, compression disabled,
  loopback HTTP/1.1 keep-alive single-worker deployment.
- `benchmarks/real-world/candidates/matched.cjs` (new): CJS twin of `matched.ts` for the
  Fastify/Node candidate — same constants, same store logic, same auth rules.
- `benchmarks/real-world/candidates/bun-fetch.ts`: extended from W4-only to full W1..W4
  contract (W1 authenticated user lookup, W2 transactional order with stock check +
  decrement, W3 paginated aggregation) using the shared `matched.ts` store.
- `benchmarks/real-world/candidates/hono.ts`: extended to W1..W4 on Hono routes with the
  same auth gate, same store, same response shapes.
- `benchmarks/real-world/candidates/elysia.ts`: extended to W1..W4 with Elysia `set.status`
  semantics and the same shared contract.
- `benchmarks/real-world/candidates/fastify.js`: extended to W1..W4 with Fastify reply
  codes and the same shared `matched.cjs` contract.
- `benchmarks/real-world/candidates/parity.test.ts` (new): 7 parity tests pinning
  identical SQL text, pool bounds, JWT rejection matrix (missing/malformed/valid),
  timeout/logging/compression/deployment limits, and W1/W2/W3 deterministic-store
  response contracts shared by every candidate.

### Required evidence

- **Candidate source**: all four Bun/Node candidates (`bun-fetch.ts`, `hono.ts`,
  `elysia.ts`, `fastify.js`) implement identical W1..W4 routes on the shared
  `matched.ts`/`matched.cjs` contract; `baselines/raw-rust` remains the transport
  lower bound per `baselines/README.md` fairness notes.
- **Parity tests**: `benchmarks/real-world/candidates/parity.test.ts` — 7/7 pass.
- **Fairness report wiring**: contract hashes (spec/workloads/schema/seed/versions)
  flow through `load.ts` into `fairness.ts` unchanged; candidates now emit identical
  response shapes so the 0%-mismatch requirement is reachable.

### Guardrail mapping

- **Candidates are semantically equivalent**: single shared contract module enforces
  one SQL set, one auth rule, one pool shape, one timeout set across all candidates.
- **No framework receives hidden advantages**: identical compression-off, logging-off,
  keep-alive-on, single-worker posture pinned in `MATCHED_CONFIG` and asserted by test.
- **All outputs pass contract fixtures**: W1/W2/W3 store responses (including 401/404/
  400/409 error paths) asserted by `parity.test.ts`.
- **Version/hash metadata is captured**: existing `versions.json` + `load.ts` hash
  pipeline unchanged and still enforced by `result-schema.ts`.

### Command results

- `cargo test -p q-http` → PASS
- `cargo test -p q-capabilities` → PASS (261+6+7+1+3+4+9)
- `bun test` → **334 pass / 0 fail (56 files)**
- `bun run typecheck`, fmt check, workspace clippy → clean
- `./scripts/verify` → **ALL PASS**

### Disclosures

- Standing: CI `verify` workflows fail with zero executed steps on every PR
  since ~#714 (infrastructure-side); disclosed per PR. Local
  `./scripts/verify` is the gate evidence.
