---
task_id: M4A-001-Z
parent_task: M4A-001
milestone: M4A
priority: P0
mode: EVIDENCE
status: PASS
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-001-Z — Package evidence for Implement actual-runtime `velqu dev` loop

## Atomic goal

Create source-backed evidence and handoff for parent task M4A-001; update status only if verification passed.

## Parent intent

Compile and reload the real QuickJS/QPack runtime with fast feedback and parity.

## Dependencies

- `M4A-001-V` — `tasks/07_m4a_developer_preview/M4A-001-V-verify-implement-actual-runtime-velqu-dev-loop.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M4A.md`
- `context/components/devex-beta.md`
- `context/components/evidence.md`

### Source files

- `AGENTS.md`
- `packages/cli/src/index.ts`
- `packages/compiler/src/index.ts`
- `packages/treaty/src/index.ts`
- `packages/testing/src/index.ts`
- `crates/q-runtime/src/source_map.rs`
- `examples/proof/`
- `README.md`
- `docs/beta/`
- `benchmarks/harness/`
- `benchmarks/manifest.json`
- `benchmarks/raw/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

## Parent acceptance guardrails

- No Bun-only behavior mismatch by default.
- Failed reload keeps prior healthy app.
- Source maps point to TypeScript.
- Reload is bounded and observable.

## Targeted commands

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

- Reload conformance.
- Failure recovery tests.
- Developer latency measurements.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m4a-001-z: package evidence for implement actual runtime velqu dev loop
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M4A-001-Z) — PASS

- Date: 2026-08-31
- Branch/PR: m4a-001-z (squash-merged; see git log for final hash)
- Closes: #437
- Parent verification: M4A-001-V PASS (PR #1041, merged 9b6ba1b) on the
  identical tree; this packet packages the evidence and flips the ledger.

### Evidence package (parent M4A-001 — actual-runtime `velqu dev` loop)
- **Implementation commits (squash-merged):**
  - M4A-001-A watch source and contracts — #1037 → b254854
  - M4A-001-B build incremental temporary QPack — #1038 → cd42ee1
  - M4A-001-C load new worker before switching — #1039 → c6376a1
  - M4A-001-D drain old worker & surface errors — #1040 → 4ddad04
  - M4A-001-V verification closure — #1041 → 9b6ba1b
- **Source implementations:**
  - `packages/compiler/src/watch.ts`: static dependency discovery without
    evaluating application code, directory-level file watching, debouncing,
    and directory polling fallback for inotify resilience.
  - `packages/compiler/src/incremental.ts`: fast-path temporary QPack
    compilation (`buildTemporaryPack`, `IncrementalPackBuilder`) with linked
    TypeScript source maps, contract change detection, and bounded temp disk
    cleanup.
  - `packages/cli/src/dev-server.ts`: `DevServer` worker swap & drain pipeline —
    candidate worker verified healthy before atomic traffic switch, fail-safe
    retention of prior worker on compile/startup failure, graceful old worker
    drain via SIGTERM with 5s timeout, and formatted compiler/runtime diagnostics.
  - `packages/cli/src/index.ts`: `velqu dev` and `velqu watch` CLI commands.
- **Key test coverage (30 test files, 237 tests):**
  - `packages/compiler/src/watch.test.ts` (7 tests): static discovery, file
    classification, change/rename/delete detection, debounce coalescing.
  - `packages/compiler/src/incremental.test.ts` (5 tests): fast compile,
    contract change detection, contract stability on handler edit, temp cleanup.
  - `packages/cli/src/dev-server.test.ts` (6 tests): proxying to real QuickJS
    worker, candidate verification before switch, prior worker retention on
    compile error, graceful drain of old worker, proof fixture proxying.
- **Gate results (worktree-fresh):** `./scripts/verify` **ALL PASS** (incl.
  q-pack 3 suites, q-engine-quickjs 20+102+1, velqu-runtime 7 suites, bun 237,
  fmt, workspace clippy -D warnings).

### Ledger
- `docs/beta/04_TASK_LEDGER.md`: M4A-001 TODO → **PASS** (all four
  guardrails proven; see the M4A-001-V mapping).

### Disclosures (standing)
- No runtime behavior changed in this packet: evidence-only closure.
- Standing: CI fails with zero executed steps on every PR since ~#714
  (infrastructure-side); disclosed per PR.
