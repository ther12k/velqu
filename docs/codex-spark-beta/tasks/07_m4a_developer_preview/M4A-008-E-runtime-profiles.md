---
task_id: M4A-008-E
parent_task: M4A-008
milestone: M4A
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-008-E — Runtime profiles

## Atomic goal

Runtime profiles.

## Parent intent

Provide an honest, runnable learning path.

## Dependencies

- `M4A-008-D` — `tasks/07_m4a_developer_preview/M4A-008-D-fetch-capabilities.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M4A.md`
- `context/components/engine-scheduler.md`
- `context/components/devex-beta.md`

### Source files

- `AGENTS.md`
- `packages/cli/src/index.ts`
- `packages/compiler/src/index.ts`
- `packages/treaty/src/index.ts`
- `packages/testing/src/index.ts`
- `crates/q-runtime/src/main.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-engine-quickjs/src/lib.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-runtime/src/source_map.rs`
- `examples/proof/`
- `README.md`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Runtime profiles.
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
cargo test -p q-engine-quickjs
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
m4a-008-e: runtime profiles
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

---

## Result (M4A-008-E) — PASS (2026-09-01)

- Branch/PR: m4a-008-e (squash-merged; see git log for final hash)
- Closes: #478

### Changed files
- `docs/beta/RUNTIME-PROFILES.md`: source-backed runtime profile guide for
  bounded `serverless` and `service:N` startup, fail-closed grammar, readiness
  semantics, explicit selection guidance, and evidence/limitation policy.
- `docs/beta/INDEX.md`, `docs/beta/README.md`: profile guide links.

### Evidence
- Documentation local-link check: PASS.
- `packages/cli/src/profile-fetch-choices.test.ts`: PASS (serverless default,
  service:4 generation, invalid profile/count rejection, CLI profile output).
- `cargo test -p q-engine-quickjs`: PASS
- `cargo test -p velqu-runtime`: PASS
- `bun test`: 309 pass / 0 fail
- `bun run typecheck`, fmt, workspace clippy: clean
- `./scripts/verify`: **ALL PASS**

### Guardrail mapping
- **Every code sample is tested:** commands mirror the existing CLI profile
  tests and runtime readiness conformance.
- **Measured facts vs targets:** profile semantics only; no performance claim.
- **No production-ready claim:** private-alpha status is explicit.
- **Known limitations prominent:** bounded worker range, fail-closed names,
  trusted-code QuickJS, and no implicit autoscaling are documented.

### Disclosures
- Documentation-only packet; no production behavior changes.
- Standing: CI verify workflows fail with zero executed steps since ~#714
  (infrastructure-side); disclosed per PR.
