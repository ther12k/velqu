---
task_id: BETA-009-A
parent_task: BETA-009
milestone: BETA
priority: P0
mode: IMPLEMENT
status: TODO
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-009-A — Run fuzz suites for pack/router/schema/bridge/HTTP

## Atomic goal

Run fuzz suites for pack/router/schema/bridge/HTTP.

## Parent intent

Establish a credible public-beta safety floor without claiming full GA hardening.

## Dependencies

- `M28-GATE` — `gates/M28-GATE.md`
- `M3-GATE` — `gates/M3-GATE.md`
- `BETA-004-Z` — `tasks/08_public_beta/BETA-004-Z-package-evidence-for-implement-optional-first-party-postgres-capability.md`
- `BETA-005-Z` — `tasks/08_public_beta/BETA-005-Z-package-evidence-for-implement-jwt-auth-reference-package.md`
- `BETA-007-Z` — `tasks/08_public_beta/BETA-007-Z-package-evidence-for-implement-configuration-and-secret-handling.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/BETA.md`
- `context/components/qpack-router.md`
- `context/components/schema-codecs.md`
- `context/components/devex-beta.md`

### Source files

- `AGENTS.md`
- `docs/beta/`
- `scripts/verify`
- `package.json`
- `.github/workflows/verify.yml`
- `crates/q-router/src/lib.rs`
- `crates/q-pack/src/lib.rs`
- `packages/compiler/src/emit.ts`
- `conformance/routing/routing.conformance.test.ts`
- `crates/q-pack/tests/fuzz_pack.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `docs/specs/pack-format-v1.md`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Run fuzz suites for pack/router/schema/bridge/HTTP.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- All beta trust boundaries documented.
- Critical/high blockers fixed or release blocked.
- Fuzz/chaos findings triaged.
- Same-process code clearly marked trusted.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
cargo test -p q-router
```
```bash
bun test
```
```bash
bun run typecheck
```

## Required evidence for this microtask

- Security report.
- Dependency scan.
- Chaos report.
- Known limitations.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
beta-009-a: run fuzz suites for pack router schema bridge http
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
