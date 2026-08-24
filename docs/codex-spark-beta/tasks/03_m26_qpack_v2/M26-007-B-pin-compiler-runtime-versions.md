---
task_id: M26-007-B
parent_task: M26-007
milestone: M26
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M26.md
commit_required: true
---

# M26-007-B — Pin compiler/runtime versions

## Atomic goal

Pin compiler/runtime versions.

## Parent intent

Make identical source/locks/toolchain produce byte-identical packs.

## Dependencies

- `M26-007-A` — `tasks/03_m26_qpack_v2/M26-007-A-remove-timestamps-non-deterministic-map-order.md`

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
5. Implement exactly this deliverable: Pin compiler/runtime versions.
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
m26-007-b: pin compiler runtime versions
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record — M26-007-B (PASS)

Deliverable: the compiler/runtime toolchain that produces pack bytes is
pinned and ENFORCED — any other bun/typescript version fails the build
by name before any build work, so non-reproducibility is diagnosed at
build time instead of surfacing as artifact drift.

### Changed files

- `packages/compiler/src/toolchain.ts` (new) — `PINNED_TOOLCHAIN`
  (compiler 0.1.0, typescript 5.9.3, bun 1.4.0): single source of
  truth; `assertPinnedToolchain({bun, typescript})` throws
  `ToolchainError` naming every mismatch; `ToolchainError` exported.
- `packages/compiler/src/index.ts` — `build()` enforces the pin FIRST
  (before extraction/bundling); `builtBy.compiler` now comes from
  `PINNED_TOOLCHAIN.compiler` instead of a duplicated literal.
- `package.json` — exact dependency pins (`typescript 5.9.3`,
  `@types/bun 1.3.4`, caret ranges removed) + `engines.bun: "1.4.0"`.
- `bun.lock` — refreshed: the old `^1.3.4` range had drifted to
  `@types/bun` 1.3.14 in the lockfile; exact pins resolve to 1.3.4
  (type-only package; no runtime/pack effect).
- `conformance/compiler/compiler.test.ts` — new "pinned toolchain"
  suite: running toolchain satisfies pins; mismatched bun / typescript
  / both each raise `ToolchainError` containing "toolchain mismatch"
  and the offending `bun <version>` / `typescript <version>` text.

### Rust side (already pinned, verified unchanged)

`rquickjs =0.12.2` (exact, workspace-pinned per AGENTS.md constraint)
with quickjs-ng 0.15.1; the pack's engine identity tuple +
`runtimeBuildHash()` are compile-time constants mirrored by
q-pack's runtime fingerprint (M26-002). No change needed.

### Artifact hashes

`app.qpack` sha256 unchanged from M26-007-A evidence:
`9fec4d4dfe08a9641977795756da2162c09468932cf9207e0b74a2290d39d4a7`
(pins record the versions actually in use — byte-identical output).

### Commands and results

- `cargo test -p q-pack` — 85+2 passed.
- `cargo test -p velqu-runtime` — 28 passed.
- `bun test` — 85 pass / 0 fail / 518 expect().
- `bun run typecheck`, `cargo fmt --check`, `cargo clippy --workspace
  --all-targets -- -D warnings` — clean.
- `./scripts/verify` — all gates green except the documented
  pre-existing `validate-benchmark-evidence` scoped failure
  (qRuntimeRelease + proofPack manifest hashes; flagged follow-up from
  M26-002-A).

Independent-builder comparison remains M26-007-D.
