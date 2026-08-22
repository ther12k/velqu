---
task_id: M25-008-D
parent_task: M25-008
milestone: M25
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M25.md
commit_required: true
---

# M25-008-D — Update semantic diff to Schema IR v2

## Atomic goal

Update semantic diff to Schema IR v2.

## Parent intent

Eliminate projection drift across tooling and runtime.

## Dependencies

- `M25-008-C` — `tasks/02_m25_schema_codecs/M25-008-C-publish-compact-contract-metadata.md`

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
5. Implement exactly this deliverable: Update semantic diff to Schema IR v2.
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
m25-008-d: update semantic diff to schema ir v2
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record (M25-008-D)

Status: **PASS**. The semantic diff covers Schema IR v2 constraint nodes
with classified change kinds (guardrail: breaking changes classified
correctly):

- `packages/compiler/src/emit.ts` `diffSchemaTypes` extended:
  - **Bounds** (`minLength`/`maxLength`, `minimum`/`maximum`,
    `minItems`/`maxItems`): tightening (floor appears/rises, ceiling
    appears/falls) is BREAKING on inputs (previously-accepted values now
    reject) and POLICY-SENSITIVE on responses; loosening is COMPATIBLE.
    Note a first-ever floor is tightening from −∞ — tested explicitly.
  - **`pattern`/`format`**: added/changed is breaking on input,
    policy-sensitive on response; removed is compatible.
  - **enum**: value removal breaks inputs, widens responses (compatible).
  - **literal**: value change is breaking.
  - **union**: member removal (canonical member comparison) breaks
    inputs, widens responses.
  - **nullable/optional**: recursion into the inner shape.
  - **fallback**: reason change is policy-sensitive (codec path change,
    M25-007-A visibility); inner shape still diffs.
- Existing structural diff behavior unchanged (route add/remove, path/
  method changes, status changes, security changes, object property
  changes) — the prior suite stays green.

### Tests and evidence

- `semantic diff classifies IR v2 constraint changes (M25-008-D)` —
  seven classification cases: input maxLength tightening (breaking),
  input minimum loosening 0 → −5 (compatible), pattern addition
  (breaking), enum removal (breaking), array minItems addition
  (breaking), response bounds tightening (policy-sensitive, NOT
  breaking), fallback reason change (policy-sensitive).
- Prior `semantic diff detects schema structural changes accurately`
  suite — unchanged and green.
- `bun test` — 81 passed, 0 failed, 481 expect calls.
- `cargo test -p q-engine-quickjs` — 1 + 96; `cargo test -p q-schema-runtime`
  — 57 + 3; `cargo test -p velqu-runtime` — 24; `cargo test -p q-pack` —
  41 + 2 — all passed.
- `bun run typecheck` — clean. `cargo fmt --check` — clean.
- `cargo clippy --workspace --all-targets -- -D warnings` — clean.
- `scripts/validate-okf` — 176 links, 0 errors.
- `./scripts/verify` — all stages pass except the documented
  isolated-worktree `qRuntimeRelease`/`proofPack` manifest hash mismatch
  (known, pre-existing on every packet branch).

Commit: `b31d832`.
