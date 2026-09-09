---
task_id: M4A-005-A
parent_task: M4A-005
milestone: M4A
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-005-A — Generate d.ts/client/OpenAPI/contract lock

## Atomic goal

Generate d.ts/client/OpenAPI/contract lock.

## Parent intent

Support separate frontend repositories without importing server implementation.

## Dependencies

- `M4A-004-Z` — `tasks/07_m4a_developer_preview/M4A-004-Z-package-evidence-for-complete-treaty-unit-local-runtime-local-and-remote-modes.md`

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
5. Implement exactly this deliverable: Generate d.ts/client/OpenAPI/contract lock.
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
m4a-005-a: generate d ts client openapi contract lock
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M4A-005-A) — PASS

- Date: 2026-09-01
- Branch/PR: m4a-005-a (squash-merged; see git log for final hash)
- Closes: #456

### Changed files
- `packages/compiler/src/index.ts`: build output now returns a concrete
  `publishedArtifacts` map and emits deterministic `published-manifest.json`
  covering `contract.json`, `contract.d.ts`, `contract.meta.json`,
  `openapi.json`, `contract.lock.json`, and the manifest itself; each record
  includes relative artifact path, byte count, and SHA-256.
- `packages/compiler/src/published.ts`: `verifyPublishedManifest` parses and
  validates format/app/hash fields and rehashes every published artifact,
  reporting missing files, byte drift, and digest drift diagnostically.
- `packages/compiler/src/published.test.ts`: package-content/hash validation,
  modified-artifact diagnostics, and deterministic two-output-directory
  reproducibility tests (3 tests).
- `benchmarks/manifest.json`: refreshed.

### Required evidence

- **Package content test**: published-manifest includes six client-facing
  artifacts; `verifyPublishedManifest` passes, and deliberate d.ts mutation
  reports both byte and SHA-256 mismatch.
- **Type-scale report**: existing `scripts/typecheck-scale.ts` evidence
  retained; no type surface weakened.
- **Reproducibility check**: two independent output directories produce an
  identical published manifest with identical artifact hashes.

### Guardrail mapping (parent M4A-005)

- **Client package contains no server runtime**: published manifest only
  enumerates contract/type/OpenAPI/lock metadata; no server imports or
  runtime code are emitted.
- **Types remain responsive at large route counts**: existing type-scale
  benchmark remains green; artifact metadata adds no client type complexity.
- **Version mismatch is diagnosable**: verifier reports unsupported manifest
  format, invalid hash, missing artifact, byte mismatch, and digest mismatch.
- **Published artifact is deterministic**: identical source/toolchain output
  yields equal manifest and artifact digests.

### Command results

- `cargo test -p q-pack` → 100 tests — 0 failed
- `bun test` → **295 pass / 0 fail (44 files)**
- `bun run typecheck` → clean
- `cargo fmt --check` clean; workspace clippy -D warnings → exit 0
- `./scripts/verify` → **ALL PASS**

### Disclosures

- Standing: CI fails with zero executed steps on every PR since ~#714
  (infrastructure-side); disclosed per PR.
