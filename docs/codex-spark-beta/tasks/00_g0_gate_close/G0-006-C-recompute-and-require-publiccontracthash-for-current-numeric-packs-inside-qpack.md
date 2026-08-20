---
task_id: G0-006-C
parent_task: G0-006
milestone: G0
priority: P1
mode: VERIFY_OR_FIX
status: TODO
context_card: context/milestones/G0.md
commit_required: true
---

# G0-006-C — Recompute and require publicContractHash for current numeric packs inside QPack::verify

## Atomic goal

Recompute and require publicContractHash for current numeric packs inside QPack::verify.

## Parent intent

Define and verify a public contract hash containing only observable API semantics.

## Dependencies

- `G0-006-B` — `tasks/00_g0_gate_close/G0-006-B-exclude-function-names-ids-policy-implementation-handler-serializer-strategy-rou.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/G0.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/devex-beta.md`

### Source files

- `AGENTS.md`
- `crates/q-pack/src/lib.rs`
- `crates/q-router/src/lib.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `packages/compiler/src/emit.ts`
- `crates/q-pack/tests/fuzz_pack.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `docs/specs/pack-format-v1.md`
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
5. Implement exactly this deliverable: Recompute and require publicContractHash for current numeric packs inside QPack::verify.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Internal HandlerId reorder leaves public hash unchanged.
- Wire-visible schema/security/status changes change the public hash.
- Serializer implementation change leaves public hash unchanged.
- Treaty/OpenAPI/lock all identify the same public graph.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
cargo test -p q-router
```
```bash
cargo test -p q-engine-quickjs
```
```bash
bun test
```
```bash
bun run typecheck
```

## Required evidence for this microtask

- Hash stability tests.
- Contract projection parity tests.
- Semantic diff fixtures.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
g0-006-c: recompute and require publiccontracthash for current numeric
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
