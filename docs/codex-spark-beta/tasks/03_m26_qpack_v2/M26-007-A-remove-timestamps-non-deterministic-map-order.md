---
task_id: M26-007-A
parent_task: M26-007
milestone: M26
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M26.md
commit_required: true
---

# M26-007-A — Remove timestamps/non-deterministic map order

## Atomic goal

Remove timestamps/non-deterministic map order.

## Parent intent

Make identical source/locks/toolchain produce byte-identical packs.

## Dependencies

- `M26-003-Z` — `tasks/03_m26_qpack_v2/M26-003-Z-package-evidence-for-encode-compiled-router-routeplans-schemas-policies-and-func.md`
- `M26-004-Z` — `tasks/03_m26_qpack_v2/M26-004-Z-package-evidence-for-embed-raw-quickjs-bytecode-without-base64.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M26.md`
- `context/components/qpack-router.md`
- `context/components/devex-beta.md`

### Source files

- `AGENTS.md`
- `crates/q-pack/src/lib.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `crates/q-runtime/src/main.rs`
- `packages/compiler/src/emit.ts`
- `crates/q-pack/tests/fuzz_pack.rs`
- `docs/specs/pack-format-v1.md`
- `scripts/package`
- `scripts/release-packet`
- `packages/cli/package.json`
- `package.json`
- `.github/workflows/verify.yml`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Remove timestamps/non-deterministic map order.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Two clean builds produce identical SHA-256.
- Non-reproducibility is diagnosed.
- Build metadata lives outside deterministic payload or is canonical.
- CI verifies reproducibility.

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

## Required evidence for this microtask

- Independent builder report.
- Artifact hashes.
- Reproducibility test.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m26-007-a: remove timestamps non deterministic map order
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record — M26-007-A (PASS)

Deliverable: no wall-clock timestamps and no source-literal-dependent
map order anywhere in compiler output — identical
source/locks/toolchain produce byte-identical build outputs.

### Changed files

- `packages/compiler/src/index.ts` — removed `generatedAt` from
  contract.json + contract.meta.json, `lockedAt` from
  contract.lock.json, `builtAt` from build-report.json and the
  "Built:" line from build-report.md. Nothing consumed these fields
  (verified by repo-wide grep incl. Treaty/core/conformance/benchmarks;
  benchmark-harness `generatedAt` fields are per-run raw evidence,
  untouched).
- `packages/compiler/src/emit.ts` — canonical map order in the pack:
  (1) per-route `responses` serialized with numerically-sorted status
  keys; (2) `defaultStatus` fallback (no 200 declared) is now the
  LOWEST sorted status, never "first literal key" — authoring order
  cannot change compiled output; (3) embedded `schemas` object written
  in sorted key order, aligning it with the schema-manifest IDs that
  already derived from sorted keys.
- `conformance/compiler/compiler.test.ts` — strengthened the COMP-003
  determinism test: two clean builds must now produce byte-identical
  files for EVERY dist artifact (raw bytes compared, file-name sets
  equal), not just pack-internal hashes.

### Reproducibility test

`rebuild produces byte-identical pack and contract hash`
(conformance/compiler/compiler.test.ts) now compares all 13 artifacts
byte-for-byte across builds separated by a 20 ms sleep.

### Artifact hashes (two clean CLI builds, ~1.5 s apart)

Both builds byte-identical (`sha256sum` diff empty), including:

- `app.qpack` — `9fec4d4dfe08a9641977795756da2162c09468932cf9207e0b74a2290d39d4a7`
- `contract.json` — `952d0db1f72bc17f702f2deaef450d8785e7aeea8e2d03b31934daa537662a57`
- `contract.lock.json` — `93da0653105f41a99d721c77e6ddaed4a37274dbc79d1402ec98b2c046c631fe`
- `build-report.json` — `bbe80268963e60df10620f65fe04d4857e55f57df48b5fe4711d94e811cd50cf`

Independent-builder comparison (different machine/container) is
M26-007-D; this packet proves same-toolchain determinism.

### Commands and results

- `cargo test -p q-pack` — 85+2 passed.
- `cargo test -p velqu-runtime` — 28 passed.
- `bun test` — 83 pass / 0 fail / 508 expect().
- `bun run typecheck`, `cargo fmt --check`, `cargo clippy --workspace
  --all-targets -- -D warnings` — clean.
- `./scripts/verify` — all gates green except the documented
  pre-existing `validate-benchmark-evidence` scoped failure
  (qRuntimeRelease + proofPack manifest hashes; flagged follow-up from
  M26-002-A).

Guardrails advanced: identical source/locks/toolchain → byte-identical
outputs (same toolchain); non-reproducibility sources removed rather
than diagnosed; build metadata eliminated from deterministic payloads
(nothing was canonicalizable wall-clock data); CI reproducibility
verification remains for M26-007-D/V.
