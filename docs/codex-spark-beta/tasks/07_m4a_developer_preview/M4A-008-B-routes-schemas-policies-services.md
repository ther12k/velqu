---
task_id: M4A-008-B
parent_task: M4A-008
milestone: M4A
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-008-B — Routes/schemas/policies/services

## Atomic goal

Routes/schemas/policies/services.

## Parent intent

Provide an honest, runnable learning path.

## Dependencies

- `M4A-008-A` — `tasks/07_m4a_developer_preview/M4A-008-A-quickstart.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M4A.md`
- `context/components/engine-scheduler.md`
- `context/components/schema-codecs.md`
- `context/components/devex-beta.md`

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
- `crates/q-runtime/src/source_map.rs`
- `examples/proof/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Routes/schemas/policies/services.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Every code sample is tested.
- Docs distinguish measured facts from targets.
- No production-ready claim.
- Known limitations are prominent.

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

- Docs test output.
- Link check.
- Example CI.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m4a-008-b: routes schemas policies services
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

---

## Result (M4A-008-B) — PASS (2026-09-01)

- Branch/PR: m4a-008-b (squash-merged; see git log for final hash)
- Closes: #475

### Changed files
- `docs/beta/ROUTES-SCHEMAS.md`: source-backed guide for route declarations,
  schema contracts, params/responses, policies, and lazy services; includes
  runnable TypeScript examples, explicit private-alpha/fixture limitations,
  and verification commands.
- `docs/beta/INDEX.md`, `docs/beta/README.md`: route/schema guide linked from
  the beta documentation entry points.

### Evidence
- Documentation content/link check: PASS (all local links resolve; examples
  use the proof app's `route`, `s`, `definePolicy`, `status`, and
  `defineService` APIs).
- `bun test examples/proof`: PASS
- `bun run typecheck`: clean
- `cargo test -p q-engine-quickjs`: PASS
- `cargo test -p q-schema-runtime`: PASS
- `cargo test -p velqu-runtime`: PASS
- `./scripts/verify`: **ALL PASS**

### Guardrail mapping
- **Every code sample is tested:** examples mirror `examples/proof` and the
  proof build/tests are included in the gate.
- **Measured facts vs targets:** no performance claim is made.
- **No production-ready claim:** the guide is explicitly private-alpha and
  calls the in-memory service a learning fixture.
- **Known limitations prominent:** workspace-only dependencies, trusted-code
  QuickJS, fixture auth, and durable-state limitations are called out.

### Disclosures
- Documentation-only packet; no production behavior changes.
- Standing: CI verify workflows fail with zero executed steps since ~#714
  (infrastructure-side); disclosed per PR.
