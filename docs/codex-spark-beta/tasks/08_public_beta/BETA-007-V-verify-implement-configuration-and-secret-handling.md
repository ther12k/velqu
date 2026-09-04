---
task_id: BETA-007-V
parent_task: BETA-007
milestone: BETA
priority: P0
mode: VERIFY
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-007-V — Verify Implement configuration and secret handling

## Atomic goal

Prove every acceptance criterion for parent task BETA-007 without broadening scope.

## Parent intent

Provide typed configuration without top-level network I/O or accidental secret disclosure.

## Dependencies

- `BETA-007-A` — `tasks/08_public_beta/BETA-007-A-environment-file-configuration.md`
- `BETA-007-B` — `tasks/08_public_beta/BETA-007-B-validation-at-startup.md`
- `BETA-007-C` — `tasks/08_public_beta/BETA-007-C-secret-value-wrapper-redaction.md`
- `BETA-007-D` — `tasks/08_public_beta/BETA-007-D-profile-specific-settings.md`
- `BETA-007-E` — `tasks/08_public_beta/BETA-007-E-no-dynamic-code-execution.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/BETA.md`
- `context/components/devex-beta.md`

### Source files

- `AGENTS.md`
- `docs/beta/`
- `scripts/verify`
- `package.json`
- `.github/workflows/verify.yml`
- `packages/cli/src/index.ts`
- `crates/q-runtime/src/source_map.rs`
- `examples/proof/`
- `README.md`
- `crates/q-runtime/src/main.rs`
- `packages/core/src/index.ts`
- `scripts/package`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- Invalid config fails before ready.
- Secrets never appear in inspect/log/error.
- Defaults are safe.
- Configuration is documented/versioned.

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

- Config tests.
- Redaction tests.
- Examples.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
beta-007-v: verify implement configuration and secret handling
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-007-V) — PASS (2026-09-04)

- Branch/PR: beta-007-v (squash-merged; see git log for final hash)
- Closes: #544

### Acceptance-criterion mapping (parent BETA-007)

1. **Invalid config fails before ready**
   - Config resolution runs in a fail-closed stage before the tokio
     runtime/engine/listener; typed `startup.rejected` (`stage:
     "config.resolve"`, exit 2). Unknown file fields, missing or
     unsupported `configVersion`, out-of-range values (never clamped),
     invalid env values, unknown `VELQU_*` names, and undeclared
     profiles all reject (A/B/D test matrices, re-run fresh here).
2. **Secrets never appear in inspect/log/error**
   - Redaction layers, all green on this run: config errors never echo
     file contents or unrelated env values (A); the ready-line config
     block is a fixed non-secret allowlist with provenance (B);
     `SecretString` renders `[redacted]` for Debug/Display with the
     database URL exposed only to the pool constructor (C); the closed
     env-namespace check never reads unknown values (B); completion
     logs stay field-allowlisted (BETA-006-F).
3. **Defaults are safe**
   - Historical posture preserved: 127.0.0.1:3000, 1 MiB body, 256
     queue, errors logging, logSample off; no profile selected means
     no profile layer; dynamic-code lockdown is unconditional (no
     opt-out).
4. **Configuration is documented/versioned**
   - `configVersion` mandatory (only 1 accepted); `docs/beta/
     CONFIGURATION.md` documents layer stack, bounds, env table,
     closed namespace allowlist, profiles, secret wrapper, and the
     startup validation report; `docs/beta/LIMITS-AND-NON-GOALS.md`
     documents the no-dynamic-code-execution guarantee.

### Commands (fresh on this branch)

- `cargo test -p q-engine-quickjs` 24 lib (incl. 4 lockdown tests) +
  117 worker + 1; `-p q-schema-runtime` 67; `-p velqu-runtime` 96 lib
  (incl. 32 config tests) + 35 runtime_conformance + 16
  fetch/source-map; `-p q-http` 14; `-p q-bridge` 11 -> 0 failures
- `bun test` -> 434 pass / 0 fail (67 files)
- fmt / clippy (`-D warnings`) / `bun run typecheck` -> clean
- `./scripts/verify` -> ALL PASS (M0-M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)
  (isolated netns; standing port-3000 environment note, BETA-002-C
  record)
- `./scripts/validate-okf` -> PASS

### Changed files

- Task record + manifest refresh commit only (verification-only
  packet; no runtime behavior changes).

### Disclosures

- Verification-only packet; no runtime behavior changes.
- Standing: CI `verify` workflows stall/fail with zero executed steps
  on PR creation across all branches (infrastructure-side, tracked
  since ~#714); local `./scripts/verify` is the real gate evidence.
- One manifest-refresh iteration after verify's release rebuild; the
  committed manifest matches the final release artifact.
