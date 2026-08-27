---
task_id: M27-005-Z
parent_task: M27-005
milestone: M27
priority: P1
mode: EVIDENCE
status: PASS
context_card: context/milestones/M27.md
commit_required: true
---

# M27-005-Z — Package evidence for Implement URL and URLSearchParams

## Atomic goal

Create source-backed evidence and handoff for parent task M27-005; update status only if verification passed.

## Parent intent

Provide interoperable URL behavior for backend libraries and fetch.

## Dependencies

- `M27-005-V` — `tasks/04_m27_capability_linker/M27-005-V-verify-implement-url-and-urlsearchparams.md`

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

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

## Parent acceptance guardrails

- Selected conformance threshold passes.
- No unbounded input behavior.
- URL behavior matches fetch usage.
- Binary/startup cost recorded.

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

- WPT report.
- Edge-case fixtures.
- Module cost report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m27-005-z: package evidence for implement url and urlsearchparams
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Evidence package

- Status: **PASS**. Parent verification M27-005-V merged in PR #870
  at commit `4c81073c709a9726126afcedb97ed1cd7991185a`; issue #268
  is closed. Based on clean parent HEAD `7629a10` (queue-regen).
- Parent acceptance matrix: `M27-005-V` maps all four guardrails
  (conformance vectors pass, explicit input and segment limits,
  WHATWG URL behavior, binary & startup overhead recorded).
- Source-backed implementation records:
  - `M27-005-A` (PR #866, #264 closed): WHATWG URL standard integration
    via `url = "2.5"` and `percent-encoding = "2.3"`, `ParsedUrl`,
    `ParsedSearchParams`, and regex-free JS prelude bindings.
  - `M27-005-B` (PR #867, #265 closed): WPT relative URL resolution,
    default port omission, IPv6 formatting, and WinterTC URLSearchParams
    conformance vectors + `docs/reports/m27-005-wpt-url-report.md`.
  - `M27-005-C` (PR #868, #266 closed): path percent-encode set, IDNA
    Punycode host normalization.
  - `M27-005-D` (PR #869, #267 closed): explicit bounded limits (`MAX_URL_LEN`,
    `MAX_SEARCH_PARAMS_LEN`, `MAX_SEARCH_PARAMS_COUNT`, `MAX_URL_PATH_SEGMENTS`).
  - `M27-005-V` (PR #870, #268 closed): verification closure + matched manifest refresh.
- Canonical evidence artifacts:
  - Tests: `q-capabilities` 70 passed (+12 URL/params/limits/WPT tests),
    `q-engine-quickjs` 107 passed, `bun test` 165 passed (+13 URL tests).
  - Report: `docs/reports/m27-005-wpt-url-report.md`.
  - Manifest: `benchmarks/manifest.json` matched refresh under verify remap environment.
- Exact verification (fresh on this branch): `cargo test` across all crates passes;
  `bun test` 165/0; typecheck, fmt --check, clippy `-D warnings` clean;
  `./scripts/verify` — ALL PASS (exit 0).
- Status bookkeeping: ledger marks M27-005 PASS; TASK_INDEX marks M27-005-Z PASS.
  Queues expose M27-006-A next.
- Remaining scope: M27-006+ (TextEncoder & TextDecoder), M27-GATE.
