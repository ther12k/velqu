---
task_id: M25-008-A
parent_task: M25-008
milestone: M25
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M25.md
commit_required: true
---

# M25-008-A — Generate all projections from canonical IR

## Atomic goal

Generate all projections from canonical IR.

## Parent intent

Eliminate projection drift across tooling and runtime.

## Dependencies

- `M25-001-Z` — `tasks/02_m25_schema_codecs/M25-001-Z-package-evidence-for-define-canonical-schema-ir-v2.md`
- `M25-003-Z` — `tasks/02_m25_schema_codecs/M25-003-Z-package-evidence-for-generate-params-query-header-decoders.md`
- `M25-004-Z` — `tasks/02_m25_schema_codecs/M25-004-Z-package-evidence-for-generate-json-body-decoders.md`
- `M25-005-Z` — `tasks/02_m25_schema_codecs/M25-005-Z-package-evidence-for-generate-status-specific-response-encoders.md`
- `M25-006-Z` — `tasks/02_m25_schema_codecs/M25-006-Z-package-evidence-for-generate-rfc-9457-problem-encoders.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M25.md`
- `context/components/engine-scheduler.md`
- `context/components/schema-codecs.md`
- `context/components/devex-beta.md`

### Source files

- `AGENTS.md`
- `crates/q-schema-runtime/src/lib.rs`
- `crates/q-engine-quickjs/src/convert.rs`
- `packages/schema/src/index.ts`
- `packages/compiler/src/emit.ts`
- `crates/q-schema-runtime/tests/fuzz_validator.rs`
- `conformance/schema/schema.conformance.test.ts`
- `packages/treaty/src/index.ts`
- `packages/contract/src/index.ts`
- `packages/testing/src/index.ts`
- `conformance/treaty/treaty.conformance.test.ts`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Generate all projections from canonical IR.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Same statuses/fields/security in all projections.
- No hand-written duplicate interface is required.
- Breaking changes are classified correctly.
- Published client does not import server implementation.

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

- Cross-projection golden tests.
- Contract diff fixtures.
- Typecheck scale results.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m25-008-a: generate all projections from canonical ir
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record (M25-008-A)

Status: **PASS**. All projections generate from the canonical Schema IR
and the hand-written duplicate client interface is gone:

- **Dead projection removed** (`packages/compiler/src/emit.ts` +
  `index.ts`): `contractFor` — an unused function whose body carried
  hand-written problem placeholders (`{ title: "string" }`) — is deleted
  along with its import. The live projections are all IR-driven:
  `contract.json` (raw IR JSON), `contract.d.ts` (`tsTypeOfIr` +
  `problemTypeOf`), `openapi.json` (`irToSchema` + `problemSchemaFor`),
  the pack's decoder/encoder programs, and the runtime validation.
- **No hand-written duplicate interface** (guardrail):
  `conformance/treaty/treaty.conformance.test.ts` now IMPORTS the
  generated contract type (`Api` from `examples/proof/dist/contract`) as
  `ProofPublishedApi` — the entire hand-written route-by-route interface
  is deleted. A consumer needs only the generated file. Shape facts stay
  pinned by two type-level `expectTypeOf` assertions (hello.get success
  shape; users.get success + exact 401 unauthorized envelope) plus the
  compiler conformance suite's d.ts content snapshots.

### Tests and evidence

- Treaty conformance — 3 passed with the generated-type import (source
  parity: the suite still drives the live runtime through the compiled
  pack using ONLY the generated type).
- `bun test` — 75 passed, 0 failed, 340 expect calls.
- `cargo test -p q-engine-quickjs` — 1 + 96; `cargo test -p q-schema-runtime`
  — 57 + 3; `cargo test -p velqu-runtime` — 24; `cargo test -p q-pack` —
  41 + 2 — all passed.
- `bun run typecheck` — clean (the generated d.ts typechecks as imported).
- `cargo fmt --check` — clean. `cargo clippy --workspace --all-targets --
  -D warnings` — clean.
- `scripts/validate-okf` — 176 links, 0 errors.
- `./scripts/verify` — all stages pass except the documented
  isolated-worktree `qRuntimeRelease`/`proofPack` manifest hash mismatch
  (known, pre-existing on every packet branch).

Parity checks across projections land in M25-008-B; compact contract
metadata in M25-008-C; the Schema IR v2 semantic diff in M25-008-D.

Commit: `7f093d7`.
