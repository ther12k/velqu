---
task_id: M4A-009-A
parent_task: M4A-009
milestone: M4A
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-009-A — Feature modules

## Atomic goal

Feature modules.

## Parent intent

Validate 30–50 routes, auth, fetch, validation, errors, pagination, and deployment.

## Dependencies

- `M4A-004-Z` — `tasks/07_m4a_developer_preview/M4A-004-Z-package-evidence-for-complete-treaty-unit-local-runtime-local-and-remote-modes.md`
- `M4A-007-Z` — `tasks/07_m4a_developer_preview/M4A-007-Z-package-evidence-for-implement-bounded-defer-and-lifecycle-hooks.md`
- `M28-GATE` — `gates/M28-GATE.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M4A.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`
- `context/components/schema-codecs.md`

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
- `crates/q-capabilities/src/lib.rs`
- `crates/q-http/src/lib.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Feature modules.
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
m4a-009-a: feature modules
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

---

## Result (M4A-009-A) — PASS (2026-09-01)

- Branch/PR: m4a-009-a (squash-merged; see git log for final hash)
- Closes: #483

### Changed files
- `examples/proof/src/modules/items/service.ts` (new): lazy `defineService`
  item store with deterministic 12-item seed, cursor pagination, and
  create/get/update/remove. In-memory learning fixture, not durable state.
- `examples/proof/src/modules/items/routes.ts` (new): five routes covering
  `GET`+pagination (`items.list`), `POST`+201 (`items.create`),
  `GET`+declared 404 problem (`items.get`), `PATCH`+404 (`items.update`), and
  `DELETE`+404 (`items.delete`); every param/query/body bound is validated
  (patterns, min/max lengths, integer ranges, array bounds).
- `examples/proof/src/app.ts`: items module registered (proof now exposes 14
  application routes).
- `examples/proof/src/modules/items/service.test.ts` (new): pagination cursors,
  page-size clamping, and CRUD lifecycle unit tests (3 tests).
- `conformance/treaty/treaty.conformance.test.ts`: runtime-local scenario
  extended with end-to-end pagination (cursor continuation), 422 validation
  rejection, create/read/patch/delete, and the exact declared 404 not-found
  problem envelope — all on the actual Rust/QuickJS runtime binary.
- Pinned inventory tests updated to the new canonical facts:
  `inspect-output.test.ts` (routeCount 14), `compiler.test.ts` (dense query
  table `["cursor","limit","ms"]` with per-route dense IDs),
  `projection-parity.test.ts` (problem-tagged statuses carry the registry
  envelope, not an authored schema), `package-verification.test.ts` (current
  proof contract hash).
- `benchmarks/manifest.json`: refreshed executable hashes.

### Required evidence

- **Proof app source**: the items module and registration above; proof route
  count now 14 with fully declared error/status contracts (no undeclared
  statuses; 404s use the typed not-found problem).
- **Scenario tests**:
  - `items service > seeds a deterministic corpus and paginates with cursors`
  - `items service > clamps page size to the store and returns an empty last
    page safely`
  - `items service > creates, reads, updates, and deletes items`
  - `Treaty runtime-local mode > drives compiled proof pack end-to-end`
    (now including pagination continuation, 422 validation, full CRUD, and
    declared-404 scenarios over actual HTTP on the actual runtime).
- **Benchmark report**: `benchmarks/manifest.json` refreshed (release runtime
  rebuilt with the reproducibility remap); no new performance claim is made —
  this packet adds routes, not measured numbers.

### Guardrail mapping
- **Runs entirely on actual runtime**: the new scenarios run through
  `runtimeTreaty` against `target/release/velqu-runtime` serving the compiled
  QPack.
- **No hidden Bun production path**: no Bun-specific imports added; the pack
  build remains the only production artifact path.
- **All error/status contracts declared**: every items route declares its
  success statuses; 404s are typed not-found problems; validation failures
  are the standard declared 422 path.
- **Load and failure scenarios pass**: pagination edge cases (empty last
  page, oversized pages, out-of-range cursors), missing-item 404s, and 422
  rejections are covered; failure-path cleanup suites stay green.

### Command results

- `cargo test -p q-engine-quickjs` → PASS (113 tests)
- `cargo test -p q-http` → PASS
- `cargo test -p q-schema-runtime` → PASS
- `cargo test -p q-capabilities` → PASS
- `cargo test -p velqu-runtime` → PASS
- `bun test` → **312 pass / 0 fail (49 files)**
- `bun run typecheck` → clean
- `cargo fmt --check`, workspace clippy `-D warnings` → clean
- `./scripts/verify` → **ALL PASS**

### Disclosures
- Standing: CI `verify` workflows fail with zero executed steps on every PR
  since ~#714 (infrastructure-side); disclosed per PR. Local
  `./scripts/verify` is the gate evidence.
