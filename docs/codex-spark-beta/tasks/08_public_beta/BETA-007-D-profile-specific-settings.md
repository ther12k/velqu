---
task_id: BETA-007-D
parent_task: BETA-007
milestone: BETA
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-007-D — Profile-specific settings

## Atomic goal

Profile-specific settings.

## Parent intent

Provide typed configuration without top-level network I/O or accidental secret disclosure.

## Dependencies

- `BETA-007-C` — `tasks/08_public_beta/BETA-007-C-secret-value-wrapper-redaction.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/BETA.md`
- `context/components/engine-scheduler.md`
- `context/components/devex-beta.md`

### Source files

- `AGENTS.md`
- `docs/beta/`
- `scripts/verify`
- `package.json`
- `.github/workflows/verify.yml`
- `crates/q-runtime/src/main.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-engine-quickjs/src/lib.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `packages/core/src/index.ts`
- `examples/proof/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Profile-specific settings.
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
beta-007-d: profile specific settings
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-007-D) — PASS (2026-09-04)

- Branch/PR: beta-007-d (squash-merged; see git log for final hash)
- Closes: #542

### Behavior implemented

Named profile blocks in the versioned config file as an explicit layer
(`CLI > env > active profile > file > default`): optional `profiles`
map + `activeProfile`, selected by `VELQU_PROFILE` (new env, wins) or
the file. Fail-closed throughout: undeclared selection rejects startup
naming the declared set; closed name shape (1..=32 of a-z 0-9 '-'),
all declared names validated; unknown fields inside blocks (including
nesting) reject; profile values pass the same bounds/closed sets. The
ready-line config block reports `activeProfile` and `profile`
provenance.

### Changed files

- `crates/q-runtime/src/config.rs` (ProfileBlock, FieldSource::Profile,
  UnknownProfile/InvalidProfileName errors, overlay layer in resolve,
  VELQU_PROFILE in the namespace, 6 new tests)
- `docs/beta/CONFIGURATION.md` (layer stack, env/namespace rows, new
  profile section)
- `docs/reports/beta-007-d-profile-specific-settings.md` (new)

### Required evidence

- **Config tests**: `profile_overrides_file_but_not_env`,
  `velqu_profile_env_selects_and_beats_file_selection`,
  `unknown_active_profile_fails_closed`, `profile_names_are_validated`,
  `profile_blocks_reject_unknown_fields_and_nesting`,
  `active_profile_reported_in_startup_config`.
- **Redaction tests**: unchanged set still green (profile layer adds
  no string echoing; error labels name profile + declared names only).
- **Examples**: production/development profile example in
  CONFIGURATION.md.

### Guardrail proofs

1. **Invalid config fails before ready** — profile selection and
   validation run inside the same fail-closed config.resolve stage.
2. **Secrets never appear in inspect/log/error** — profile errors name
   profiles, never values; config block stays a fixed allowlist.
3. **Defaults are safe** — no profile selected means no profile layer;
   existing files behave identically.
4. **Configuration is documented/versioned** — same `configVersion: 1`
   file, additive schema, documented in CONFIGURATION.md.

### Gate results (fresh on this branch)

- `cargo test -p q-engine-quickjs` 138 / `-p q-schema-runtime` 67 /
  `-p velqu-runtime` 96 lib + 35 conformance + 16 fetch/source-map /
  `-p q-http` 14 / `-p q-bridge` 11 — 0 failures
- fmt / clippy (`-D warnings`) / typecheck -> clean
- `bun test` -> 434 pass / 0 fail (67 files)
- `./scripts/verify` -> ALL PASS (M0-M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)
- `./scripts/validate-okf` -> PASS
  (verify run inside an isolated netns; standing port-3000 environment
  note, BETA-002-C record. No test weakened.)
