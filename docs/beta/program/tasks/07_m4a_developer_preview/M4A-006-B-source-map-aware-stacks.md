---
task_id: M4A-006-B
parent_task: M4A-006
milestone: M4A
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-006-B — Source-map-aware stacks

## Atomic goal

Source-map-aware stacks.

## Parent intent

Make compile, startup, contract, capability, and runtime failures actionable.

## Dependencies

- `M4A-006-A` — `tasks/07_m4a_developer_preview/M4A-006-A-structured-diagnostic-codes.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M4A.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/capabilities-fetch.md`

### Source files

- `AGENTS.md`
- `packages/cli/src/index.ts`
- `packages/compiler/src/index.ts`
- `packages/treaty/src/index.ts`
- `packages/testing/src/index.ts`
- `packages/contract/src/index.ts`
- `conformance/treaty/treaty.conformance.test.ts`
- `packages/compiler/src/emit.ts`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-pack/src/lib.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Source-map-aware stacks.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- No secrets in production diagnostics.
- Errors identify route/source/contract cause.
- Source maps are lazy on success path.
- Diagnostic catalog exists.

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

- Golden diagnostics.
- Redaction tests.
- Source-map tests.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m4a-006-b: source map aware stacks
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M4A-006-B) — PASS

- Date: 2026-09-01
- Branch/PR: m4a-006-b (squash-merged; see git log for final hash)
- Closes: #463

### Changed files
- `crates/q-runtime/src/source_map.rs`: added an explicit advisory
  `mapper_for_sidecar` symbolization path. It loads sidecar data only on
  request, verifies format and exact pack SHA-256 binding, parses the map,
  and fails closed for malformed/mismatched sidecars; default runtime mapper
  remains identity when no map is present.
- `crates/q-runtime/tests/source_map_conformance.rs` (new): valid bound
  sidecar lookup, mismatch isolation/fail-closed, and invalid embedded-map
  identity fallback (3 tests).
- `benchmarks/manifest.json`: refreshed.

### Required evidence

- **Golden diagnostics**: existing CLI code-frame/diagnostic-code tests stay
  green alongside real source-map lookup.
- **Redaction tests**: sidecar errors expose only binding/format diagnostics;
  runtime serving never consults the advisory sidecar.
- **Source-map tests**: three Rust conformance tests prove lazy sidecar load,
  exact pack binding, invalid-map fallback, and no effect on default mapper.

### Guardrail mapping (parent M4A-006)

- **No secrets in production diagnostics**: sidecar tooling is advisory and
  does not add source contents to serving errors.
- **Errors identify route/source/contract cause**: bound maps return original
  source locations via the existing `SourceMapper` interface.
- **Source maps are lazy on success path**: `mapper_for_sidecar` is separate
  from startup `mapper_for`; only tooling requesting symbolization reads it.
- **Diagnostic catalog exists**: M4A-006-A closed code catalog remains
  unchanged and all diagnostic tests pass.

### Command results

- `cargo test -p q-pack` → PASS
- `cargo test -p q-engine-quickjs` → PASS
- `cargo test -p q-capabilities` → PASS
- `cargo test -p velqu-runtime` → PASS, including 3 source-map tests
- `bun test` → **305 pass / 0 fail (47 files)**
- `bun run typecheck` → clean
- `cargo fmt --check` clean; workspace clippy -D warnings → exit 0
- `./scripts/verify` → **ALL PASS**

### Disclosures

- Standing: CI fails with zero executed steps on every PR since ~#714
  (infrastructure-side); disclosed per PR.
