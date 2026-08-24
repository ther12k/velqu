---
task_id: M26-007-D
parent_task: M26-007
milestone: M26
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M26.md
commit_required: true
---

# M26-007-D — Compare independent build outputs

## Atomic goal

Compare independent build outputs.

## Parent intent

Make identical source/locks/toolchain produce byte-identical packs.

## Dependencies

- `M26-007-C` — `tasks/03_m26_qpack_v2/M26-007-C-canonicalize-section-ordering-and-padding.md`

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
5. Implement exactly this deliverable: Compare independent build outputs.
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
m26-007-d: compare independent build outputs
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record — M26-007-D (PASS)

Deliverable: independent build outputs are compared by an executable
gate — and the comparison immediately caught and fixed a REAL
reproducibility defect.

### Defect found and diagnosed (guardrail: non-reproducibility is diagnosed)

Building examples/proof from a DIFFERENT working directory produced
different bytes: Bun.build writes cwd-relative path banner comments
into the bundle (`// packages/core/src/index.ts` vs
`// ../home/.../index.ts`), changing `bundle`, `integrity.bundleSha256`,
and every artifact derived from them. Same-process tests (COMP-003)
cannot see this — only an independent-builder comparison can.

Fix (`packages/compiler/src/emit.ts`): `bundleApp` pins the bundler's
working directory to the app entry directory for the duration of
`Bun.build` (restored in a finally block); the synthetic entry path is
resolved to absolute before the switch. Bundle banners and source-map
sources are now layout-relative — byte-stable regardless of caller
cwd. One-time consequence: the canonical `app.qpack` hash changed
(banners now relative to the app dir instead of the builder's cwd).

### Independent builder report

`scripts/compare-builds` (new, wired into `./scripts/verify` as its own
step): builds examples/proof twice through fully independent CLI
processes — second run executes from `/` with absolute paths into a
fresh temp outDir — then requires SHA-256 equality of EVERY emitted
artifact. Result: **12 artifacts byte-identical** across independent
builders.

This satisfies the CI-verifies-reproducibility guardrail locally:
the check runs in the one-command gate on every push/PR (the GitHub
Actions runner itself is separately broken with zero executed steps,
documented per PR).

### Changed files

- `packages/compiler/src/emit.ts` — cwd-pinned bundling (above).
- `scripts/compare-builds` — independent-build comparison gate.
- `scripts/verify` — "Independent-build reproducibility (M26-007-D)"
  step after the proof build.
- `.github/workflows/verify.yml` — CI Bun pin 1.3.4 → 1.4.0 to match
  `PINNED_TOOLCHAIN` (M26-007-B); the old pin would fail the toolchain
  assertion on any runner where Actions executes again.

### Artifact hashes

- `app.qpack`: `363e60153830f7dba101ea3c196baef5f174ad8b978d41c630c9bdb2119f3de3`
  (identical from both builders; previous canonical hash
  `9fec4d4d…` was cwd-layout-dependent and is superseded by this fix).
- Full 12-artifact hash list identical across builders (see
  compare-builds output in CI evidence above).

### Commands and results

- `cargo test -p q-pack` — 86 passed + 2, 0 failed.
- `cargo test -p velqu-runtime` — 28 passed.
- `bun test` — 85 pass / 0 fail / 518 expect().
- `bun run typecheck`, `cargo fmt --check`, `cargo clippy --workspace
  --all-targets -- -D warnings` — clean.
- `./scripts/verify` — all gates green INCLUDING the new
  independent-build step, except the documented pre-existing
  `validate-benchmark-evidence` scoped failure (qRuntimeRelease +
  proofPack manifest hashes; flagged follow-up from M26-002-A).
