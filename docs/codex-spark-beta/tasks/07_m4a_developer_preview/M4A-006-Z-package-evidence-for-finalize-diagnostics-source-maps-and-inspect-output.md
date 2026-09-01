---
task_id: M4A-006-Z
parent_task: M4A-006
milestone: M4A
priority: P0
mode: EVIDENCE
status: PASS
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-006-Z — Package evidence for Finalize diagnostics, source maps, and inspect output

## Atomic goal

Create source-backed evidence and handoff for parent task M4A-006; update status only if verification passed.

## Parent intent

Make compile, startup, contract, capability, and runtime failures actionable.

## Dependencies

- `M4A-006-V` — `tasks/07_m4a_developer_preview/M4A-006-V-verify-finalize-diagnostics-source-maps-and-inspect-output.md`

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
- `crates/q-engine/src/lib.rs`
- `crates/q-runtime/src/serve.rs`
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

- No secrets in production diagnostics.
- Errors identify route/source/contract cause.
- Source maps are lazy on success path.
- Diagnostic catalog exists.

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

- Golden diagnostics.
- Redaction tests.
- Source-map tests.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m4a-006-z: package evidence for finalize diagnostics source maps and in
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M4A-006-Z) — PASS

- Date: 2026-09-01
- Branch/PR: m4a-006-z (squash-merged; see git log for final hash)
- Closes: #467
- Parent verification: M4A-006-V PASS (PR #1071, merged 5bdc2be); this
  packet packages the evidence and flips the M4A-006 ledger.

### Evidence package (parent M4A-006 — Finalize diagnostics, source maps, and inspect output)

- **Implementation commits (squash-merged):**
  - M4A-006-A structured diagnostic codes — #1067 → 4f85905
  - M4A-006-B source-map-aware stacks — #1068 → faf38e7
  - M4A-006-C redaction policy — #1069 → c632dd5
  - M4A-006-D inspect output fidelity — #1070 → 9b57f39
  - M4A-006-V verification closure — #1071 → 5bdc2be
- **Source-backed proofs:**
  - CLI `DiagnosticCode` catalog and source frames/hints.
  - Lazy, exact-pack-bound source-map sidecar symbolization.
  - Expanded sensitive-value redaction for authorization/cookie forms.
  - Inspect JSON route count/ID, codecs, bridges, stages, policies,
    capabilities, fallback reasons, and actual strategy distribution.
- **Tests:**
  - CLI diagnostics (6), source-map Rust conformance (3), inspect output (3),
    redaction assignment suite (4); all full suites green.
- **Raw type-scale samples** (3 reps): 25 routes
  719.8/677.1/647.0 ms; 100 routes 992.9/491.9/666.4 ms; 200 routes
  680.5/469.7/496.5 ms. Startup-dominated; no unsupported performance claim.

### Parent guardrail proofs

- No secrets in production diagnostics (redaction and structural fields).
- Errors identify route/source/contract cause (codes, frames, inspect IDs).
- Source maps lazy on success path (explicit tooling sidecar mapper).
- Diagnostic catalog exists (closed exported `DiagnosticCode` union).

### Gate results

- `cargo test -p velqu-runtime` → PASS
- `bun test` → **308 pass / 0 fail (48 files)**
- `bun run typecheck` → clean
- `cargo fmt --check` clean; workspace clippy -D warnings → exit 0
- `./scripts/verify` → **ALL PASS**

### Ledger

- `docs/beta/04_TASK_LEDGER.md`: M4A-006 TODO → **PASS**.
- STATUS.md and TASK_INDEX.md updated to PASS.

### Disclosures

- Evidence-only packet; no production behavior changes here.
- Standing: CI fails with zero executed steps on every PR since ~#714
  (infrastructure-side); disclosed per PR.
