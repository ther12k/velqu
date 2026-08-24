---
task_id: M26-007-Z
parent_task: M26-007
milestone: M26
priority: P1
mode: EVIDENCE
status: PASS
context_card: context/milestones/M26.md
commit_required: true
---

# M26-007-Z — Package evidence for Guarantee reproducible release packs

## Atomic goal

Create source-backed evidence and handoff for parent task M26-007; update status only if verification passed.

## Parent intent

Make identical source/locks/toolchain produce byte-identical packs.

## Dependencies

- `M26-007-V` — `tasks/03_m26_qpack_v2/M26-007-V-verify-guarantee-reproducible-release-packs.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M26.md`
- `context/components/qpack-router.md`
- `context/components/devex-beta.md`
- `context/components/evidence.md`

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

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

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

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m26-007-z: package evidence for guarantee reproducible release packs
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Evidence package

- Status: **PASS**. Parent verification M26-007-V merged in PR #815 at
  commit `951b3fa`; issue #220 is closed. The evidence package is based
  on clean parent HEAD `f807305` before this commit.
- Parent acceptance matrix: `M26-007-V` maps all four guardrails to
  source and named tests (raw-byte artifact equality across clean and
  independent builds; ToolchainError + sha-diff diagnosis paths;
  canonical-only build metadata with zero wall-clock fields; the
  one-command gate carrying the independent-build step on every
  push/PR).
- Source-backed implementation records:
  - `M26-007-A` (PR #811, #216 closed): timestamps removed from every
    artifact; status-key order / default-status fallback / schema
    embedding canonicalized; COMP-003 test compares ALL dist artifacts
    raw-byte.
  - `M26-007-B` (PR #812, #217 closed): `PINNED_TOOLCHAIN` enforced
    before any build work; exact dependency pins; lockfile drift
    corrected (bun-types 1.3.14 → 1.3.4); Rust side verified
    already-pinned.
  - `M26-007-C` (PR #813, #218 closed): binary v2 writers sort by
    section id, zero-only padding pinned;
    `section_order_and_padding_are_canonical` proves permutation
    invariance for both writers.
  - `M26-007-D` (PR #814, #219 closed): found+fixed real cwd-leakage
    defect (Bun.build banners); `scripts/compare-builds` gate — 12
    artifacts byte-identical across independent builders; CI Bun pin
    aligned 1.4.0.
  - `M26-007-V` (PR #815, #220 closed): verification closure; embed
    determinism probe (`cd982e84…` ×2); no defects requiring fixes.
- Artifact hashes: `app.qpack`
  `363e60153830f7dba101ea3c196baef5f174ad8b978d41c630c9bdb2119f3de3`;
  embedded bytecode pack `cd982e8412d8f6bcb8d4e951da76017e66c629f1429
  4cfec27cd428649fcc3ca`.
- Exact verification (fresh on this branch): q-pack 86+2, velqu-runtime
  28 passed; bun 85 pass / 0 fail / 518 expect(); typecheck, fmt
  --check, clippy `-D warnings` clean. `./scripts/verify` completes
  every stage INCLUDING "Independent-build reproducibility" except the
  documented pre-existing benchmark-manifest mismatch (qRuntimeRelease
  + proofPack; flagged matched-evidence follow-up from M26-002-A).
- Status bookkeeping: `docs/beta/04_TASK_LEDGER.md` marks M26-007
  PASS; TASK_INDEX marks M26-007-Z PASS (V row was marked by its own
  packet). STATUS.md marks the Z checkbox. The generated Spark queues
  expose M26-008-A next.
- Remaining scope: `M26-008`+ remain TODO until implemented and
  evidenced.
