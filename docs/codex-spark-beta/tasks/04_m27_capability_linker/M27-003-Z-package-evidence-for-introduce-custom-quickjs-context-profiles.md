---
task_id: M27-003-Z
parent_task: M27-003
milestone: M27
priority: P1
mode: EVIDENCE
status: PASS
context_card: context/milestones/M27.md
commit_required: true
---

# M27-003-Z — Package evidence for Introduce custom QuickJS context profiles

## Atomic goal

Create source-backed evidence and handoff for parent task M27-003; update status only if verification passed.

## Parent intent

Measure minimal/web/full contexts and select only meaningful reductions.

## Dependencies

- `M27-003-V` — `tasks/04_m27_capability_linker/M27-003-V-verify-introduce-custom-quickjs-context-profiles.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M27.md`
- `context/components/engine-scheduler.md`
- `context/components/capabilities-fetch.md`
- `context/components/evidence.md`

### Source files

- `AGENTS.md`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `packages/compiler/src/emit.ts`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`
- `crates/q-runtime/src/main.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-engine-quickjs/src/lib.rs`
- `benchmarks/harness/`
- `benchmarks/manifest.json`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

## Parent acceptance guardrails

- Chosen profile has measurable startup/RSS benefit or feature is deferred.
- No silent missing intrinsic.
- Conformance passes for selected profile.
- Profile identity enters runtime fingerprint.

## Targeted commands

```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p q-capabilities
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

- Context benchmark.
- Test262 subset.
- Compatibility report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m27-003-z: package evidence for introduce custom quickjs context profil
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Evidence package

- Status: **PASS**. Parent verification M27-003-V merged in PR #858
  at commit `8717dbc9b6796aeca1921f5500e50259c072e383`; issue #256
  is closed. Based on clean parent HEAD `1871148` (queue-regen).
- Parent acceptance matrix: `M27-003-V` maps all three guardrails —
  benefit-not-demonstrated ⇒ selection DEFERRED to M27-011 with
  production serving on Full; no silent missing intrinsic (typed
  probes both directions + fail-closed CLI); conformance passes for
  the selected Full profile; profile identity carried in the ready
  line (defect fixed during V).
- Source-backed implementation records:
  - `M27-003-A` (PR #854, #252): ContextProfile closed vocabulary,
    create_context single point, default Full byte-identical.
  - `M27-003-B` (PR #855, #253): compiler emits per-app intrinsic
    requirement as diagnostic data; regex-literal limitation
    documented.
  - `M27-003-C` (PR #856, #254): reduction impact diagnostics in
    manifest + `velqu inspect capabilities`.
  - `M27-003-D` (PR #857, #255): `--context-profile` runtime flag;
    full retained as explicit compatibility baseline; two distinct
    loud-failure modes pinned (top-level → fail at load; lazy →
    redacted internal problems per request).
  - `M27-003-V` (PR #858, #256): profile identity in ready line +
    matched manifest refresh + deferred verdict.
- Canonical evidence artifacts:
  - Tests: q-engine-quickjs 102 (+5 profile pins), conformance 31
    (+1 compat), TS capability/reduction suites inside bun's 152/0.
  - Report: `docs/reports/m27-003-context-profiles-compat.md`
    (context benchmark raw samples + compatibility findings).
- Exact verification (fresh on this branch): q-pack 98,
  q-engine-quickjs 102, q-capabilities 51, velqu-runtime 31;
  bun 152 pass / 0 fail; typecheck/fmt/clippy clean.
  Disclosed flake: one verify attempt hit a pre-existing port-race
  in an unrelated concurrency test
  (`fallback_paths_are_bounded_and_deadline_aware`, connection
  refused). Three consecutive clean suite runs + verify ALL PASS
  after. Not touched in this packet (unrelated finding).
- Bookkeeping: ledger marks M27-003 PASS; TASK_INDEX marks
  M27-003-Z PASS. Queues expose M27-004-A next.
