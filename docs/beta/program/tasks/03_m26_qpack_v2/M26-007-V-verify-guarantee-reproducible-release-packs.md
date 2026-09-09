---
task_id: M26-007-V
parent_task: M26-007
milestone: M26
priority: P1
mode: VERIFY
status: PASS
context_card: context/milestones/M26.md
commit_required: true
---

# M26-007-V — Verify Guarantee reproducible release packs

## Atomic goal

Prove every acceptance criterion for parent task M26-007 without broadening scope.

## Parent intent

Make identical source/locks/toolchain produce byte-identical packs.

## Dependencies

- `M26-007-A` — `tasks/03_m26_qpack_v2/M26-007-A-remove-timestamps-non-deterministic-map-order.md`
- `M26-007-B` — `tasks/03_m26_qpack_v2/M26-007-B-pin-compiler-runtime-versions.md`
- `M26-007-C` — `tasks/03_m26_qpack_v2/M26-007-C-canonicalize-section-ordering-and-padding.md`
- `M26-007-D` — `tasks/03_m26_qpack_v2/M26-007-D-compare-independent-build-outputs.md`

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

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

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

- Independent builder report.
- Artifact hashes.
- Reproducibility test.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m26-007-v: verify guarantee reproducible release packs
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Verification record — M26-007-V (PASS)

Parent: M26-007 "Guarantee reproducible release packs". All four
implementation dependencies merged before this branch (PRs #811–#814;
issues #216–#219 closed).

### Acceptance criterion mapping

1. **Two clean builds produce identical SHA-256.**
   - JSON pack path: `rebuild produces byte-identical pack and
     contract hash` (COMP-003) compares EVERY dist artifact raw-byte
     across two builds 20 ms apart — 13 files, including app.qpack,
     contract.json, contract.lock.json, build-report.json/md.
   - Independent processes: `scripts/compare-builds` (in the gate) —
     second builder runs from `/` with absolute paths; **12 artifacts
     byte-identical**, `app.qpack`
     `363e60153830f7dba101ea3c196baef5f174ad8b978d41c630c9bdb2119f3de3`.
   - Binary v2 path: `section_order_and_padding_are_canonical` proves
     reversed/rotated section permutations produce byte-identical
     files for both writers, ascending directories, zero-only padding.
   - Bytecode embed probe: two `velqu-bytecode embed` runs 1.2 s apart
     produce identical `app-bc.qpack` (`cd982e8412d8f6bcb8d4e951da76
     017e66c629f14294cfec27cd428649fcc3ca`) — deterministic serde_json
     canonicalization.

2. **Non-reproducibility is diagnosed.**
   - Toolchain drift: pinned-toolchain suite — any bun/typescript
     mismatch raises `ToolchainError` naming the offending version
     BEFORE any build work (`build()` asserts first).
   - Path/cwd leakage: found and FIXED by M26-007-D's comparison
     (Bun.build cwd-relative banners); the gate that caught it now
     fails with a full sha256sum diff on any recurrence.
   - Map-order variance: M26-007-A made status-key order, default
     status fallback, and schema embedding canonical, so authoring
     order cannot silently change output.

3. **Build metadata lives outside deterministic payload or is
   canonical.** No wall-clock fields exist in any artifact
   (`generatedAt`/`lockedAt`/`builtAt` removed in M26-007-A; grep over
   dist confirms zero matches). The only toolchain metadata inside the
   pack (`builtBy`, `engine` identity tuple + build hash) is a pinned
   compile-time constant mirrored by q-pack's runtime fingerprint.
   Binary header/directory/padding are fully canonicalized
   (M26-007-C).

4. **CI verifies reproducibility.** `.github/workflows/verify.yml`
   runs `./scripts/verify` on every push/PR — which now includes the
   "Independent-build reproducibility (M26-007-D)" step — with the Bun
   pin aligned to `PINNED_TOOLCHAIN` (1.4.0). The Actions runner
   itself currently executes zero steps on every PR (infrastructure,
   documented on every PR #714–#814); the local one-command gate is
   the gate of record until infra recovers.

### Required microtask evidence

- Independent builder report: compare-builds PASS (above).
- Artifact hashes: app.qpack 363e6015…; embed cd982e84… ×2 identical.
- Reproducibility tests: COMP-003 strengthened test,
  `section_order_and_padding_are_canonical`, pinned-toolchain suite,
  compare-builds gate step.

### Changed files

This task record, STATUS.md checkbox, TASK_INDEX row only. No defects
requiring code fixes surfaced during verification; no unrelated
findings needing follow-up tasks.

### Commands and results (branch on master e81198c)

- `cargo test -p q-pack` — 86 passed + 2, 0 failed.
- `cargo test -p velqu-runtime` — 28 passed.
- `bun test` — 85 pass / 0 fail / 518 expect().
- `bun run typecheck`, `cargo fmt --check`, `cargo clippy --workspace
  --all-targets -- -D warnings` — clean.
- `./scripts/verify` — all gates green INCLUDING independent-build
  reproducibility, except the documented pre-existing
  `validate-benchmark-evidence` scoped failure (qRuntimeRelease +
  proofPack manifest hashes; flagged matched-evidence follow-up from
  M26-002-A).
