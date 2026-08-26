---
task_id: M27-002-D
parent_task: M27-002
milestone: M27
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M27.md
commit_required: true
---

# M27-002-D — Remove unused modules

## Atomic goal

Remove unused modules.

## Parent intent

Resolve exactly which capabilities enter each application artifact.

## Dependencies

- `M27-002-C` — `tasks/04_m27_capability_linker/M27-002-C-emit-capability-inventory-hash-into-qpack.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M27.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/capabilities-fetch.md`

### Source files

- `AGENTS.md`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `packages/compiler/src/emit.ts`
- `crates/q-pack/src/lib.rs`
- `crates/q-pack/tests/fuzz_pack.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `docs/specs/pack-format-v1.md`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Remove unused modules.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Unrelated app pays zero linked capability cost.
- Dependency graph is deterministic.
- Missing capability fails at build or startup.
- `velqu inspect --capabilities` is accurate.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
cargo test -p q-engine-quickjs
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

- Resolver tests.
- Binary-size delta report.
- Cold-start delta report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m27-002-d: remove unused modules
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record — M27-002-D (PASS)

Deliverable: unused capability modules stay out of the artifact —
with two real defects found and fixed on the way.

### Defects found (this packet's substance)

1. **Silent grant loss**: `detectCapabilities` only recognized
   `ctx.native.*`; the destructured style (`async ({ native }) =>
   ...`) — the form the proof app itself uses — produced NO grants.
   Timer routes were silently ungranted: declared `[]`, inventory
   never linked. Fixed in extract.ts (detection covers `ctx.native`,
   destructured, and aliased `{ native: n }`; 4 regression tests
   including the pre-fix failing shape).
2. **Unknown grants silently dropped**: the old hardcoded
   `.filter((c) => c === "timer")` discarded unrecognized authority
   requests. Now a build error naming the grant and the known set.

### Changed files

- `packages/compiler/src/emit.ts` — `KNOWN_GRANTS`,
  `resolveLinkedModules(grants)` (builtin universe, exact-version,
  sorted output, fail-closed on unknown), validated grant pipeline,
  pruned `capabilityInventory` + hash emission.
- `packages/compiler/src/extract.ts` — destructured-native detection fix.
- `packages/cli/src/capability-inspect.ts` (new) +
  `packages/cli/src/index.ts` — `velqu inspect capabilities` now
  reports the pack's hash-verified linked inventory; recompute-hash
  mismatch and unsorted inventories fail loud instead of lying;
  pre-inventory packs reported honestly as "unknown".
- `packages/cli/src/capability-inventory.test.ts` (+7),
  `packages/cli/src/capability-detect.test.ts` (new, 4).
- `docs/reports/m27-002-d-prune-deltas.md` — matched size/cold-start
  evidence.
- Bookkeeping: STATUS.md, TASK_INDEX.md.

### Required evidence

- Resolver tests: q-capabilities 51 (A/B/C resolver + inventory
  suites pass unchanged); TS-side prune/inspect/detect tests 21 new
  total across the two test files (bun full suite 139 pass / 0 fail).
- Binary-size delta report: proof pack 24,534 → 24,590 B (**+56 B**
  for linking runtime:timers@1); zero-link apps unchanged
  structurally (`[]` → count-prefix canonical bytes).
- Cold-start delta report: n=10 matched fresh-process samples per
  side, same release binary; p50/p95 distributions overlap —
  **reported as statistically indistinguishable**, no capacity claim
  (raw samples retained in the report).

### Commands (fresh worktree → commits a160e35 / 074bebe)

- `cargo test -p q-pack` 98 · `-p q-engine-quickjs` 98 ·
  `-p q-capabilities` 51 — pass.
- `bun test` 139 pass / 0 fail; `bun run typecheck` clean.
- `cargo fmt --check`, `cargo clippy --workspace --all-targets --
  -D warnings` — clean.

### Notes

- Guardrail mapping: zero-cost for unrelated apps → empty-grant
  prune tests + before/after structural parity for `[]`;
  deterministic graph → A/B determinism suites unchanged; missing
  fails at build → C rules unchanged + unknown-grant build error;
  `velqu inspect --capabilities` accuracy → hash-verified inspect
  with loud-failure paths, tested.
