---
task_id: M27-010-Z
parent_task: M27-010
milestone: M27
priority: P1
mode: EVIDENCE
status: PASS
context_card: context/milestones/M27.md
commit_required: true
---

# M27-010-Z — Package evidence for Establish Web API conformance program

## Atomic goal

Create source-backed evidence and handoff for parent task M27-010; update status only if verification passed.

## Parent intent

Separate standards compatibility from internal framework tests.

## Dependencies

- `M27-010-V` — `tasks/04_m27_capability_linker/M27-010-V-verify-establish-web-api-conformance-program.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M27.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/capabilities-fetch.md`

### Source files

- `AGENTS.md`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `packages/compiler/src/emit.ts`
- `crates/q-pack/src/lib.rs`
- `crates/q-pack/tests/fuzz_pack.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `docs/specs/pack-format-v1.md`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

## Parent acceptance guardrails

- No unsupported API is advertised.
- Pass/fail/skip counts are reproducible.
- Behavioral regressions block relevant gate.
- Reports link to exact runtime build.

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

- Conformance report.
- Pinned test manifest.
- CI output.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m27-010-z: package evidence for establish web api conformance program
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M27-010-Z) — PASS

- Date: 2026-08-27
- Branch/PR: m27-010-z (squash-merged; see git log for final hash)
- Closes: #299

### Parent closure — M27-010 Establish Web API conformance program

Parent intent: separate standards compatibility from internal framework tests. Status: **PASS**.

Packet commits (squash merges):
- M27-010-A — 28bb96b (#896, Closes #294): Pinned WPT / WinterTC subsets across all 4 capabilities (`conformance/web-api/wpt-manifest.json`, `conformance/web-api/web-api.conformance.test.ts`, `crates/q-capabilities/tests/wpt_wintertc_conformance.rs`, `docs/reports/m27-010-wpt-wintertc-conformance.md`)
- M27-010-B — a849725 (#897, Closes #295): Recorded 8 explicit skips with standard references, machine-readable reason codes, and deferred targets
- M27-010-C — de23c2b (#898, Closes #296): Automated regression reports generator (`scripts/generate-conformance-report.py`) with `--check` drift validation wired into `scripts/verify`
- M27-010-D — 9960f48 (#899, Closes #297): QuickJS engine tests asserting unadvertised Web APIs are strictly absent (`undefined`) and never stubbed (`crates/q-engine-quickjs/src/worker.rs`)
- M27-010-V — b4b435a (#900, Closes #298): Verification closure mapping all 4 acceptance guardrails

### Evidence ledger (required microtask evidence)
- **Conformance report**: `docs/reports/m27-010-wpt-wintertc-conformance.md` — generated programmatically with manifest SHA-256 and exact commit hash.
- **Pinned test manifest**: `conformance/web-api/wpt-manifest.json` — formal JSON declaration of 34 pinned test vectors (100% PASS) + 8 explicit skips.
- **CI output / Local gates**: All suites re-run green on this branch (q-capabilities 107+7, q-pack 96+2, q-engine-quickjs 15+97, velqu-runtime 31, bun test 213/213, ./scripts/verify ALL PASS).

### Command results (this branch)
- `cargo test -p q-capabilities` → 107 unit tests + 7 integration tests passed
- `cargo test -p q-engine-quickjs` → 15 unit tests + 97 worker tests passed
- `cargo test -p q-pack` → 96+2 passed
- `cargo test -p velqu-runtime` → 31 passed
- `bun test` → 213 pass / 0 fail (27 files)
- `bun run typecheck` → clean
- `cargo fmt --check` → clean
- `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `python3 scripts/generate-conformance-report.py --check` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**

### Ledger update
- `docs/beta/04_TASK_LEDGER.md`: M27-010 flipped TODO -> PASS.

### Disclosures (standing)
- CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR. Local evidence above is complete.
