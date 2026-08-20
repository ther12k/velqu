---
task_id: G0-008-Z
parent_task: G0-008
milestone: G0
priority: P1
mode: EVIDENCE
status: TODO
context_card: context/milestones/G0.md
commit_required: true
---

# G0-008-Z — Package evidence for Close canonical performance evidence

## Atomic goal

Create source-backed evidence and handoff for parent task G0-008; update status only if verification passed.

## Parent intent

Close the frozen G0 benchmark evidence requirements instead of relying on a single clean pass.

## Dependencies

- `G0-008-V` — `tasks/00_g0_gate_close/G0-008-V-verify-close-canonical-performance-evidence.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/G0.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/evidence.md`

### Source files

- `AGENTS.md`
- `crates/q-pack/src/lib.rs`
- `crates/q-router/src/lib.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `packages/compiler/src/emit.ts`
- `crates/q-pack/tests/fuzz_pack.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `docs/specs/pack-format-v1.md`
- `benchmarks/harness/`
- `benchmarks/manifest.json`
- `benchmarks/raw/`
- `docs/reports/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

## Parent acceptance guardrails

- Markdown reports are generated from current raw data.
- Verifier fails on stale reports.
- No public claim uses a single spot check.
- Any regression is documented rather than hidden.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
cargo test -p q-router
```
```bash
cargo test -p q-engine-quickjs
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
```bash
./scripts/validate-production-plan
```
```bash
./scripts/validate-okf
```

## Required evidence for this microtask

- Raw benchmark directory.
- Generated report.
- Environment and artifact manifest.
- Ablation results for relevant changes.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
g0-008-z: package evidence for close canonical performance evidence
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
