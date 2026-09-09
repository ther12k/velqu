---
task_id: M4A-008-A
parent_task: M4A-008
milestone: M4A
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-008-A — Quickstart

## Atomic goal

Quickstart.

## Parent intent

Provide an honest, runnable learning path.

## Dependencies

- `M4A-002-Z` — `tasks/07_m4a_developer_preview/M4A-002-Z-package-evidence-for-complete-cli-command-surface.md`
- `M4A-004-Z` — `tasks/07_m4a_developer_preview/M4A-004-Z-package-evidence-for-complete-treaty-unit-local-runtime-local-and-remote-modes.md`
- `M4A-006-Z` — `tasks/07_m4a_developer_preview/M4A-006-Z-package-evidence-for-finalize-diagnostics-source-maps-and-inspect-output.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M4A.md`
- `context/components/devex-beta.md`

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

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Quickstart.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Every code sample is tested.
- Docs distinguish measured facts from targets.
- No production-ready claim.
- Known limitations are prominent.

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

## Required evidence for this microtask

- Docs test output.
- Link check.
- Example CI.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m4a-008-a: quickstart
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

---

## Result (M4A-008-A) — PASS (2026-09-01)

- Branch/PR: m4a-008-a (squash-merged; see git log for final hash)
- Closes: #474

### Changed files
- `docs/beta/QUICKSTART.md`: runnable private-alpha path covering pinned
  prerequisites, frozen install, CLI init, serverless/service:N profiles,
  develop/check/test, QPack build/inspect/runtime, next steps, and prominent
  limitations (not production-ready, trusted-code QuickJS, non-durable defer,
  evidence-bound performance claims).
- `docs/beta/INDEX.md`, `docs/beta/README.md`: quickstart links added to the
  beta documentation entry points.

### Evidence
- Documentation content/link validator: PASS (all referenced local links
  exist; commands, profile grammar, limitations, and private-alpha notice
  asserted).
- `packages/cli/src/scaffold.test.ts`: **5 pass / 0 fail** (generated
  starter structure, no credentials, workspace disclosure, compile/extraction,
  CLI init receipt).
- `bun test`: **308 pass / 0 fail (48 files)**
- `bun run typecheck`: clean
- `cargo test -p velqu-runtime`: PASS
- `cargo fmt --check`, workspace clippy `-D warnings`: clean
- `./scripts/verify`: **ALL PASS**

### Guardrail mapping
- **Every code sample is tested:** starter commands and generated project path
  are covered by scaffold tests and proof build in verify.
- **Measured facts vs targets:** quickstart makes no performance claims and
  points to retained raw p50/p95/p99 evidence.
- **No production-ready claim:** explicitly labeled private alpha and beta
  finish line.
- **Known limitations prominent:** package availability, trusted-code
  QuickJS, non-durable defer, and supported scope are called out.

### Disclosures
- Documentation-only packet; no production runtime behavior changes.
- Standing: CI verify workflows fail with zero executed steps since ~#714
  (infrastructure-side); disclosed per PR.
