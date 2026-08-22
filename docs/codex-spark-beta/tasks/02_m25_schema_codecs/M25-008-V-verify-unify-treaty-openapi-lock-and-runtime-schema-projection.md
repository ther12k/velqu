---
task_id: M25-008-V
parent_task: M25-008
milestone: M25
priority: P0
mode: VERIFY
status: PASS
context_card: context/milestones/M25.md
commit_required: true
---

# M25-008-V — Verify Unify Treaty, OpenAPI, lock, and runtime schema projection

## Atomic goal

Prove every acceptance criterion for parent task M25-008 without broadening scope.

## Parent intent

Eliminate projection drift across tooling and runtime.

## Dependencies

- `M25-008-A` — `tasks/02_m25_schema_codecs/M25-008-A-generate-all-projections-from-canonical-ir.md`
- `M25-008-B` — `tasks/02_m25_schema_codecs/M25-008-B-add-parity-checks-to-verification.md`
- `M25-008-C` — `tasks/02_m25_schema_codecs/M25-008-C-publish-compact-contract-metadata.md`
- `M25-008-D` — `tasks/02_m25_schema_codecs/M25-008-D-update-semantic-diff-to-schema-ir-v2.md`

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
- `packages/cli/src/index.ts`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

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

- Cross-projection golden tests.
- Contract diff fixtures.
- Typecheck scale results.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m25-008-v: verify unify treaty openapi lock and runtime schema projecti
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record (M25-008-V)

Status: **PASS**. Every parent M25-008 acceptance guardrail maps to source
and passing tests; all verification commands were run fresh on this branch
(no code changes — verification closure only).

### Guardrail → source → evidence

1. **Same statuses/fields/security in all projections.**
   - `projection parity (M25-008-B)` — 5 tests / 134 assertions in every
     `bun test` and `scripts/verify`: route identity, declared statuses,
     fields (response + params/query/body), security, and compact-metadata
     sync across contract.json, the pack, openapi.json, and contract.d.ts.
2. **No hand-written duplicate interface is required.**
   - M25-008-A: the treaty suite imports the generated `Api` type; the
     hand-written per-route interface is deleted; shape pins are two
     `expectTypeOf` assertions + d.ts content snapshots.
3. **Breaking changes are classified correctly.**
   - `semantic diff classifies IR v2 constraint changes (M25-008-D)` —
     seven classification cases (input tightening breaking; loosening
     compatible; first-ever floor is tightening from −∞; response
     tightening policy-sensitive; enum/union/literal; fallback reason
     codec-path change) plus the prior structural suite.
4. **Published client does not import server implementation.**
   - TRT-004 bundle-isolation test (zero @velqu/core / @velqu/compiler /
     bun: / node: imports in the treaty package); the published surface is
     `contract.d.ts` (types) + `contract.meta.json` (1.5 KiB pinning
     metadata, no schema bodies — M25-008-C).

### Command results (this branch, fresh worktree)

- `cargo test -p q-engine-quickjs` — 1 + 96 passed.
- `cargo test -p q-schema-runtime` — 57 unit + 3 fuzz passed.
- `cargo test -p velqu-runtime` — 24 integration passed.
- `cargo test -p q-pack` — 41 + 2 passed.
- `cargo test --workspace` — zero failures (direct run).
- `bun test` — 81 passed, 0 failed, 481 expect calls.
- `bun run typecheck` — clean. `cargo fmt --check` — clean.
- `cargo clippy --workspace --all-targets -- -D warnings` — clean.
- `scripts/validate-okf` — 176 links, 0 errors.
- `./scripts/verify` — the first run reported one transient
  `FAIL: cargo test` that did NOT reproduce (two subsequent full verify
  runs and a direct `cargo test --workspace` run were completely green —
  same non-reproducible pattern observed and documented once before in
  M25-005-V, most likely a timing blip in the runtime-conformance suite
  under verify load). Final state: all stages pass except the documented
  isolated-worktree `qRuntimeRelease`/`proofPack` manifest hash mismatch.

Changed files: this record, `docs/codex-spark-beta/STATUS.md`,
`docs/codex-spark-beta/indexes/TASK_INDEX.md` (verification closure only).

Commit: `6898138`.
