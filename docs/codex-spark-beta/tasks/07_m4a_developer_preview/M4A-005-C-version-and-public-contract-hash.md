---
task_id: M4A-005-C
parent_task: M4A-005
milestone: M4A
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-005-C — Version and public contract hash

## Atomic goal

Version and public contract hash.

## Parent intent

Support separate frontend repositories without importing server implementation.

## Dependencies

- `M4A-005-B` — `tasks/07_m4a_developer_preview/M4A-005-B-tree-shakable-client.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M4A.md`
- `context/components/qpack-router.md`
- `context/components/devex-beta.md`

### Source files

- `AGENTS.md`
- `packages/cli/src/index.ts`
- `packages/compiler/src/index.ts`
- `packages/treaty/src/index.ts`
- `packages/testing/src/index.ts`
- `crates/q-pack/src/lib.rs`
- `crates/q-pack/tests/fuzz_pack.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `docs/specs/pack-format-v1.md`
- `packages/contract/src/index.ts`
- `conformance/treaty/treaty.conformance.test.ts`
- `packages/compiler/src/emit.ts`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Version and public contract hash.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Client package contains no server runtime.
- Types remain responsive at large route counts.
- Version mismatch is diagnosable.
- Published artifact is deterministic.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
bun test
```
```bash
bun run typecheck
```

## Required evidence for this microtask

- Package content test.
- Type-scale report.
- Reproducibility check.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m4a-005-c: version and public contract hash
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M4A-005-C) — PASS

- Date: 2026-09-01
- Branch/PR: m4a-005-c (squash-merged; see git log for final hash)
- Closes: #458

### Changed files
- `packages/compiler/src/published.ts`: published-manifest verification now
  cross-checks the manifest public `contractHash` against `contract.json` and
  diagnoses hash drift in addition to format, missing-file, byte, and digest
  mismatches.
- `packages/compiler/src/public-contract.test.ts` (new): stable 128-bit public
  hash/version pinning across contract/meta/manifest/d.ts and explicit
  version/hash drift diagnostics (2 tests).
- `benchmarks/manifest.json`: refreshed.

### Required evidence

- **Package content test**: contract/meta/manifest all carry formatVersion 1,
  the same 32-hex public contract hash, and generated d.ts exports that hash.
- **Type-scale report**: inherited M4A-005-A `typecheck-scale.ts` raw samples;
  no public type surface changed.
- **Reproducibility check**: inherited two-output published manifest equality
  and artifact hash verification; current release-backed `./scripts/verify`
  remains green.

### Guardrail mapping (parent M4A-005)

- **Client package contains no server runtime**: only public metadata/hash
  validation is added; no runtime import is published.
- **Types remain responsive at large route counts**: no route/type projection
  changes; inherited scale evidence remains valid.
- **Version mismatch is diagnosable**: verifier now explicitly reports
  unsupported manifest format and manifest-vs-contract hash drift.
- **Published artifact is deterministic**: public hash/version values are
  asserted across generated outputs and source/toolchain reproducibility is
  retained.

### Command results

- `cargo test -p q-pack` → 100 tests — 0 failed
- `bun test` → **290 pass / 0 fail (41 files)**
- `bun run typecheck` → clean
- `cargo fmt --check` clean; workspace clippy -D warnings → exit 0
- `./scripts/verify` → **ALL PASS**

### Disclosures

- Standing: CI fails with zero executed steps on every PR since ~#714
  (infrastructure-side); disclosed per PR.
