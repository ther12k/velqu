---
task_id: M4A-005-D
parent_task: M4A-005
milestone: M4A
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-005-D — Package verification

## Atomic goal

Package verification.

## Parent intent

Support separate frontend repositories without importing server implementation.

## Dependencies

- `M4A-005-C` — `tasks/07_m4a_developer_preview/M4A-005-C-version-and-public-contract-hash.md`

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
5. Implement exactly this deliverable: Package verification.
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
m4a-005-d: package verification
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M4A-005-D) — PASS

- Date: 2026-09-01
- Branch/PR: m4a-005-d (squash-merged; see git log for final hash)
- Closes: #459

### Changed files
- `packages/compiler/src/published.ts`: added `verifyPublishedPackage`, a
  package-level verifier that composes artifact/hash checks with expected
  appId, public contract hash, and format-version pins; diagnostics identify
  the mismatched pin and actual value.
- `packages/compiler/src/package-verification.test.ts` (new): complete-package
  acceptance with matching pins and explicit app/hash mismatch diagnostics
  (2 tests).
- `benchmarks/manifest.json`: refreshed.

### Required evidence

- **Package content test**: complete published package verifies with matching
  app/hash/version pins; result exposes non-empty artifact metadata.
- **Type-scale report**: raw `scripts/typecheck-scale.ts` samples — 25 routes
  635.6/620.5/749.4 ms; 100 routes 493.1/499.6/498.9 ms; 200 routes
  627.2/1041.7/792.5 ms. Startup-dominated; no unsupported performance claim.
- **Reproducibility check**: inherited M4A-005-A two-output manifest equality
  and SHA-256 artifact verification; release-backed `./scripts/verify` remains
  green.

### Guardrail mapping (parent M4A-005)

- **Client package contains no server runtime**: package verifier reads only
  published metadata and contract artifacts.
- **Types remain responsive at large route counts**: verifier adds no client
  route/type surface; type-scale evidence remains green.
- **Version mismatch is diagnosable**: `verifyPublishedPackage` produces
  explicit appId, contractHash, and formatVersion mismatch messages.
- **Published artifact is deterministic**: package verification relies on
  manifest byte/hash records and inherited reproducibility tests.

### Command results

- `cargo test -p q-pack` → 100 tests — 0 failed
- `bun test` → **303 pass / 0 fail (46 files)**
- `bun run typecheck` → clean
- `cargo fmt --check` clean; workspace clippy -D warnings → exit 0
- `./scripts/verify` → **ALL PASS**

### Disclosures

- Standing: CI fails with zero executed steps on every PR since ~#714
  (infrastructure-side); disclosed per PR.
