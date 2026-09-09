---
task_id: M4A-004-Z
parent_task: M4A-004
milestone: M4A
priority: P0
mode: EVIDENCE
status: PASS
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-004-Z — Package evidence for Complete Treaty unit-local, runtime-local, and remote modes

## Atomic goal

Create source-backed evidence and handoff for parent task M4A-004; update status only if verification passed.

## Parent intent

Deliver Eden-quality type-safe clients and distinct test fidelity levels.

## Dependencies

- `M4A-004-V` — `tasks/07_m4a_developer_preview/M4A-004-V-verify-complete-treaty-unit-local-runtime-local-and-remote-modes.md`

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
- `packages/contract/src/index.ts`
- `conformance/treaty/treaty.conformance.test.ts`
- `packages/compiler/src/emit.ts`
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

- No public `any`.
- 2xx data and non-2xx errors narrow correctly.
- Undeclared status is a contract error.
- All modes share the same contract.

## Targeted commands

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

- Negative type tests.
- Mode parity tests.
- Typecheck scale benchmark.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m4a-004-z: package evidence for complete treaty unit local runtime loca
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M4A-004-Z) — PASS

- Date: 2026-09-01
- Branch/PR: m4a-004-z (squash-merged; see git log for final hash)
- Closes: #455
- Parent verification: M4A-004-V PASS (PR #1059, merged 646e069); this
  packet packages the evidence and flips the M4A-004 ledger.

### Evidence package (parent M4A-004 — Complete Treaty modes)
- **Implementation commits (squash-merged):**
  - M4A-004-A direct dispatcher — #1055 → e03df88
  - M4A-004-B runtime-local Rust/QuickJS process — #1056 → 46f416a
  - M4A-004-C remote fetch client — #1057 → d039fde
  - M4A-004-D exact typing — #1058 → f9c9d6c
  - M4A-004-V verification closure — #1059 → 646e069
- **Contract/type surface:**
  - `packages/treaty/src/index.ts`: common `TreatyClient<Api>` with exact
    method/body/query/header/path parameter typing, status-split results,
    portable fetch, direct dispatch transport.
  - `packages/contract/src/index.ts`: typed header generic on
    `RouteContract`, preserving legacy six-argument contracts.
  - `packages/testing/src/index.ts`: explicitly labeled `unitTreatyDirect`,
    `unitTreaty`, `remoteTreaty`, and `runtimeTreaty`; runtime adapter loads
    published `contract.json`, captures ready identity, and boundedly drains
    SIGTERM → SIGKILL.
- **Tests/evidence:**
  - `unit-direct.test.ts` (6): direct/loopback parity, typed problems,
    undeclared status, method mismatch.
  - `runtime-local.test.ts` (3): generated contract, actual Rust+QuickJS,
    ready identity, typed routes, bounded drain, service:2.
  - `remote.test.ts` (4): HTTP success/errors, abort/network, direct/remote
    parity.
  - `exact-typing.test.ts` (2): exact query/header forwarding + 400 problem.
  - `types-negative.test-d.ts`: compile-time rejection of unsupported
    methods, wrong/missing body/query/headers/params, impossible 200 error;
    exact status/problem narrowing.
  - `typecheck-scale.ts`: raw 3-repetition measurements on this run:
    25 routes 690.6/614.1/658.4 ms; 100 routes 659.0/724.7/1285.5 ms;
    200 routes 920.3/763.6/746.6 ms. Startup-dominated; no unsupported
    performance claim.
- **Parent guardrail proofs:**
  - No public `any`; exact request and response types.
  - 2xx data vs declared non-2xx typed errors plus status-0 network/abort.
  - Undeclared status fails loud as a contract error in direct mode.
  - All modes share the same generated contract and client result machinery.

### Gate results (fresh worktree)

- `bun test` → **292 pass / 0 fail (42 files)**
- `bun run typecheck` → clean
- `cargo fmt --check` clean; workspace clippy -D warnings → exit 0
- `./scripts/verify` (parent V evidence) → **ALL PASS**

### Ledger

- `docs/beta/04_TASK_LEDGER.md`: M4A-004 TODO → **PASS**.
- STATUS.md and TASK_INDEX.md updated to PASS.

### Disclosures

- Evidence-only packet; no production runtime behavior changed here.
- Standing: CI fails with zero executed steps on every PR since ~#714
  (infrastructure-side); disclosed per PR.
