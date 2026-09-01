---
task_id: M4A-005-B
parent_task: M4A-005
milestone: M4A
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-005-B — Tree-shakable client

## Atomic goal

Tree-shakable client.

## Parent intent

Support separate frontend repositories without importing server implementation.

## Dependencies

- `M4A-005-A` — `tasks/07_m4a_developer_preview/M4A-005-A-generate-d-ts-client-openapi-contract-lock.md`

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
5. Implement exactly this deliverable: Tree-shakable client.
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
m4a-005-b: tree shakable client
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M4A-005-B) — PASS

- Date: 2026-09-01
- Branch/PR: m4a-005-b (squash-merged; see git log for final hash)
- Closes: #457

### Changed files
- `packages/treaty/src/index.ts`: added `treatyRoutes`, a typed route-ID
  allowlist that filters the published contract before constructing the
  client; proxy namespaces absent from the allowlist are not materialized.
  Direct `treaty()` remains full-client compatible.
- `packages/treaty/src/tree-shaking.test.ts` (new): 4 tests covering runtime
  allowlist materialization, selected-route type preservation, client bundle
  isolation from `@velqu/core`/`@velqu/compiler`/q-runtime, and full-client
  compatibility.
- `benchmarks/manifest.json`: refreshed.

### Required evidence

- **Package content test**: published Treaty source contains no server/compiler
  imports; allowlist exposes only `health.live` and no `hello`/`users` namespace.
- **Type-scale report**: raw `scripts/typecheck-scale.ts` samples — 25 routes
  669.8/783.7/1079.4 ms; 100 routes 493.2/857.4/532.2 ms; 200 routes
  510.1/606.8/777.3 ms. Startup-dominated; no unsupported performance claim.
- **Reproducibility check**: inherited M4A-005-A two-output build manifest
  equality and artifact SHA-256 verification; current build remains
  release-backed and deterministic under `./scripts/verify`.

### Guardrail mapping (parent M4A-005)

- **Client package contains no server runtime**: isolation test scans the
  published Treaty source for server/compiler/runtime imports.
- **Types remain responsive at large route counts**: allowlist returns the
  same typed `TreatyClient<Api>` projection and scale benchmark remains green.
- **Version mismatch is diagnosable**: inherited published-manifest verifier
  reports format, app/hash, missing-file, byte, and SHA-256 drift.
- **Published artifact is deterministic**: inherited independent-output
  reproducibility test plus release-backed benchmark validation pass.

### Command results

- `cargo test -p q-pack` → 100 tests — 0 failed
- `bun test` → **299 pass / 0 fail (44 files)**
- `bun run typecheck` → clean
- `cargo fmt --check` clean; workspace clippy -D warnings → exit 0
- `./scripts/verify` → **ALL PASS**

### Disclosures

- Standing: CI fails with zero executed steps on every PR since ~#714
  (infrastructure-side); disclosed per PR.
