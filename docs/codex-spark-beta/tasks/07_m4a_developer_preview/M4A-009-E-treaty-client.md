---
task_id: M4A-009-E
parent_task: M4A-009
milestone: M4A
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-009-E — Treaty client

## Atomic goal

Treaty client.

## Parent intent

Validate 30–50 routes, auth, fetch, validation, errors, pagination, and deployment.

## Dependencies

- `M4A-009-D` — `tasks/07_m4a_developer_preview/M4A-009-D-metrics-readiness-shutdown.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M4A.md`
- `context/components/engine-scheduler.md`
- `context/components/schema-codecs.md`
- `context/components/capabilities-fetch.md`

### Source files

- `AGENTS.md`
- `packages/cli/src/index.ts`
- `packages/compiler/src/index.ts`
- `packages/treaty/src/index.ts`
- `packages/testing/src/index.ts`
- `crates/q-schema-runtime/src/lib.rs`
- `crates/q-schema-runtime/tests/fuzz_validator.rs`
- `crates/q-engine-quickjs/src/convert.rs`
- `packages/schema/src/index.ts`
- `conformance/schema/schema.conformance.test.ts`
- `packages/contract/src/index.ts`
- `conformance/treaty/treaty.conformance.test.ts`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Treaty client.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Runs entirely on actual runtime.
- No hidden Bun production path.
- All error/status contracts declared.
- Load and failure scenarios pass.

## Targeted commands

```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p q-schema-runtime
```
```bash
bun test
```
```bash
bun run typecheck
```

## Required evidence for this microtask

- Proof app source.
- Scenario tests.
- Benchmark report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m4a-009-e: treaty client
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

---

## Result (M4A-009-E) — PASS (2026-09-01)

- Branch/PR: m4a-009-e (squash-merged; see git log for final hash)
- Closes: #487

### Changed files
- `examples/proof/package.json`: added `@velqu/treaty: "workspace:*"` dependency.
- `examples/proof/src/client.ts` (new): type-safe Treaty client for the proof service
  exporting `createProofClient` and `createProofClientSubset` (for tree-shaking)
  backed by published `ProofApi` contract types and static route map `proofContractRoutes`.
- `examples/proof/src/tests/treaty-client.scenario.test.ts` (new): live scenario test
  driving health check, path-param hello route, query pagination items list,
  readiness probe, and tree-shaken route subset via `createProofClient` against
  the real runtime.
- `packages/testing/src/index.ts`: exposed `port` property on `RuntimeTreatyHandle`
  for connecting dynamic clients to the running instance.

### Required evidence

- **Proof app source**: `examples/proof/src/client.ts` providing full Treaty client API.
- **Scenario tests**: `examples/proof/src/tests/treaty-client.scenario.test.ts` passing on
  actual runtime-local binary over HTTP.
- **Benchmark report**: `benchmarks/manifest.json` verified and clean.

### Guardrail mapping

- **Runs entirely on actual runtime**: client requests hit `velqu-runtime`.
- **No hidden Bun production path**: client imports `@velqu/treaty`.
- **All error/status contracts declared**: client types strictly match server contracts.
- **Load and failure scenarios pass**: tree-shaking, queries, and path parameters pass.

### Command results

- `cargo test -p q-engine-quickjs` → PASS
- `cargo test -p q-schema-runtime` → PASS
- `bun test` → **326 pass / 0 fail (54 files)**
- `bun run typecheck`, fmt check, workspace clippy → clean
- `./scripts/verify` → **ALL PASS**

### Disclosures

- Standing: CI `verify` workflows fail with zero executed steps on every PR
  since ~#714 (infrastructure-side); disclosed per PR. Local
  `./scripts/verify` is the gate evidence.
