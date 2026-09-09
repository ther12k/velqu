---
task_id: BETA-007-C
parent_task: BETA-007
milestone: BETA
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-007-C — Secret value wrapper/redaction

## Atomic goal

Secret value wrapper/redaction.

## Parent intent

Provide typed configuration without top-level network I/O or accidental secret disclosure.

## Dependencies

- `BETA-007-B` — `tasks/08_public_beta/BETA-007-B-validation-at-startup.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/BETA.md`
- `context/components/devex-beta.md`
- `context/components/evidence.md`

### Source files

- `AGENTS.md`
- `docs/beta/`
- `scripts/verify`
- `package.json`
- `.github/workflows/verify.yml`
- `crates/q-engine/src/lib.rs`
- `crates/q-runtime/src/serve.rs`
- `docs/reports/`
- `docs/beta/workstreams/OBSERVABILITY_OPERATIONS.md`
- `crates/q-runtime/src/main.rs`
- `packages/core/src/index.ts`
- `examples/proof/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Secret value wrapper/redaction.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

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

## Required evidence for this microtask

- Config tests.
- Redaction tests.
- Examples.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
beta-007-c: secret value wrapper redaction
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-007-C) — PASS (2026-09-04)

- Branch/PR: beta-007-c (squash-merged; see git log for final hash)
- Closes: #541

### Behavior implemented

`config::SecretString` — typed wrapper for secret configuration
values, wired at the VELQU_DATABASE_URL environment boundary:
Debug/Display always render `[redacted]` (holders included), the only
read path is the explicit grep-auditable `expose()`, `from_env`
wraps at first touch, no Clone/PartialEq. `run()` now exposes the
database URL only to the pool constructor; behavior otherwise
unchanged (same fail-closed rejections).

### Changed files

- `crates/q-runtime/src/config.rs` (SecretString + 3 tests)
- `crates/q-runtime/src/lib.rs` (postgres URL wrapped at the env
  boundary; exposed only to `pool_from_url_and_env`)
- `docs/beta/CONFIGURATION.md` ("Secret value wrapper" section)
- `docs/reports/beta-007-c-secret-value-wrapper-redaction.md` (new)

### Required evidence

- **Config tests**: `secret_debug_and_display_render_redacted`,
  `secret_expose_is_the_only_read_path`,
  `secret_from_env_wraps_without_disclosure`.
- **Redaction tests**: the wrapper tests pin the no-disclosure
  property (including Debug of holder containers); existing redaction
  layers still green (config errors BETA-007-A, config block
  allowlist BETA-007-B, completion logs BETA-006-F, pool URL
  redaction BETA-004).
- **Examples**: CONFIGURATION.md "Secret value wrapper" section
  documents the wrapper→pool flow and the honest non-goal on memory
  zeroization.

### Guardrail proofs

1. **Invalid config fails before ready** — unchanged fail-closed
   postgres rejections; wrapper is behavior-preserving.
2. **Secrets never appear in inspect/log/error** — Debug/Display
   `[redacted]`; single explicit read path; layered redaction tests
   all green.
3. **Defaults are safe** — untouched.
4. **Configuration is documented/versioned** — wrapper documented in
   CONFIGURATION.md.

### Gate results (fresh on this branch)

- `cargo test -p velqu-runtime` -> 90 lib (3 new) + 35 conformance +
  16 fetch/source-map, 0 failures
- fmt / clippy (`-D warnings`) / typecheck -> clean
- `bun test` -> 434 pass / 0 fail (67 files)
- `./scripts/verify` -> ALL PASS (M0-M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)
- `./scripts/validate-okf` -> PASS
  (verify run inside an isolated netns; standing port-3000 environment
  note, BETA-002-C record. No test weakened.)
