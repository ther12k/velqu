---
task_id: M25-008-C
parent_task: M25-008
milestone: M25
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M25.md
commit_required: true
---

# M25-008-C — Publish compact contract metadata

## Atomic goal

Publish compact contract metadata.

## Parent intent

Eliminate projection drift across tooling and runtime.

## Dependencies

- `M25-008-B` — `tasks/02_m25_schema_codecs/M25-008-B-add-parity-checks-to-verification.md`

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
- `scripts/package`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Publish compact contract metadata.
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
m25-008-c: publish compact contract metadata
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record (M25-008-C)

Status: **PASS**. Compact contract metadata is published with every build:

- `packages/compiler/src/index.ts`: the build now emits
  `contract.meta.json` alongside the other artifacts — the minimal facts
  a client repository needs to pin and verify contract sync: `formatVersion`,
  `appId`, `contractHash` (same hash as contract.json/lock), `generatedAt`,
  and per route `{ method, path, statuses, secured }`. No schema bodies,
  no IRs (proof app: 1,459 bytes). Clients import `contract.d.ts` for
  types and carry this file for runtime pinning; neither imports server
  implementation (TRT-004 suite).
- Parity coverage: `projection parity (M25-008-B)` gained a fifth test —
  "compact contract metadata stays in sync and compact" — asserting hash
  and appId agreement with contract.json, route set + statuses + security
  agreement, the absence of schema bodies, and a 4 KiB compactness bound.

### Tests and evidence

- `projection parity > compact contract metadata stays in sync and
  compact (M25-008-C)` — passed.
- `bun test` — 80 passed, 0 failed, 474 expect calls.
- `cargo test -p q-engine-quickjs` — 1 + 96; `cargo test -p q-schema-runtime`
  — 57 + 3; `cargo test -p velqu-runtime` — 24; `cargo test -p q-pack` —
  41 + 2 — all passed.
- `bun run typecheck` — clean. `cargo fmt --check` — clean.
- `cargo clippy --workspace --all-targets -- -D warnings` — clean.
- `scripts/validate-okf` — 176 links, 0 errors.
- `./scripts/verify` — all stages pass except the documented
  isolated-worktree `qRuntimeRelease`/`proofPack` manifest hash mismatch
  (known, pre-existing on every packet branch).

Commit: `c54f1e9`.
