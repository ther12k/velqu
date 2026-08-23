---
task_id: M26-001-Z
parent_task: M26-001
milestone: M26
priority: P0
mode: EVIDENCE
status: PASS
context_card: context/milestones/M26.md
commit_required: true
---

# M26-001-Z — Package evidence for Accept QPack v2 format and compatibility ADR

## Atomic goal

Create source-backed evidence and handoff for parent task M26-001; update status only if verification passed.

## Parent intent

Freeze the binary format goals, trust model, compatibility, and migration rules.

## Dependencies

- `M26-001-V` — `tasks/03_m26_qpack_v2/M26-001-V-verify-accept-qpack-v2-format-and-compatibility-adr.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M26.md`
- `context/components/qpack-router.md`
- `context/components/evidence.md`

### Source files

- `AGENTS.md`
- `crates/q-pack/src/lib.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `crates/q-runtime/src/main.rs`
- `packages/compiler/src/emit.ts`
- `crates/q-pack/tests/fuzz_pack.rs`
- `docs/specs/pack-format-v1.md`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

## Parent acceptance guardrails

- Unknown versions fail closed.
- Current mode has no legacy handler table.
- Compatibility policy is explicit.
- Untrusted arbitrary bytecode is forbidden.

## Targeted commands

```bash
cargo test -p q-pack
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

- ADR.
- Binary layout diagrams.
- Compatibility matrix.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m26-001-z: package evidence for accept qpack v2 format and compatibilit
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Evidence package

- Status: **PASS**. Parent verification M26-001-V merged in PR #779 at
  commit `17121ea1d2c949e0846e14ad56539f149e297477`; issue #184 is closed.
  The evidence package is based on clean parent HEAD `1e468fb` before
  this commit.
- Parent acceptance matrix: `M26-001-V` maps all four guardrails to
  source and named tests:
  1. Unknown versions fail closed: `detect_pack_format_mode` +
     `unknown_versions_fail_closed` +
     `mode_two_still_fails_closed_before_native_adapter`.
  2. Current mode has no legacy handler table:
     `numeric_pack_with_handler_table_is_rejected`,
     `numeric_pack_without_compiled_router_is_rejected`,
     `accepts_valid_numeric_pack`.
  3. Compatibility policy explicit: ADR-0024 (mode policy, named
     legacy-v1 boundary, compatibility matrix, migration rules),
     ADR-0025 (section directory), ADR-0026 (integrity ≠ authenticity),
     ADR-0027 (debug/source sidecar) + `docs/specs/pack-format-v2.md`.
  4. Untrusted arbitrary bytecode forbidden: compiler-owned,
     integrity-pinned bytecode (missing hash / hash mismatch /
     integrity-without-bytecode all reject; engine/ABI mismatches
     reject; runtime-local tampered-bytecode fails BEFORE ready).
- Source-backed implementation records:
  - `M26-001-A` (PR #775, #180 closed): numeric current mode + legacy v1
    adapter (ADR-0024, mode dispatch, fail-closed unknowns).
  - `M26-001-B` (PR #776, #181 closed): section directory, alignment,
    bounds, optional sections, versioning (ADR-0025, pack-format-v2
    spec, qpack2 module).
  - `M26-001-C` (PR #777, #182 closed): integrity separated from
    authenticity (ADR-0026 + pinning tests).
  - `M26-001-D` (PR #778, #183 closed): debug/source sidecar policy
    (ADR-0027 + runtime-independence pin).
- Exact verification (fresh on this branch): `cargo test -p q-pack`
  (48 + 2); `cargo test -p q-engine-quickjs` (1 + 96);
  `cargo test -p q-http` (4 + 6 + 1); `cargo test -p velqu-runtime`
  (24); `bun test` (81 passed, 0 failed, 481 expect calls);
  `bun run typecheck` clean; `cargo fmt --check` clean; `cargo clippy
  --workspace --all-targets -- -D warnings` clean; `scripts/validate-okf`
  clean; `./scripts/verify` ALL stages pass.
- Status bookkeeping: `docs/beta/04_TASK_LEDGER.md` marks M26-001 PASS;
  the beta checklist and task index mark this Z packet PASS. The
  generated Spark queues expose M26-002-A (#186) next.
- Remaining scope: `M26-002`+ remain TODO until implemented and
  evidenced.

Commit: `ef0d456`.
