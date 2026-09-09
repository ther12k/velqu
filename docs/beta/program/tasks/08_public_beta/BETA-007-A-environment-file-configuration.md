---
task_id: BETA-007-A
parent_task: BETA-007
milestone: BETA
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-007-A — Environment/file configuration

## Atomic goal

Environment/file configuration.

## Parent intent

Provide typed configuration without top-level network I/O or accidental secret disclosure.

## Dependencies

- `M27-GATE` — `gates/M27-GATE.md`

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
- `crates/q-runtime/src/main.rs`
- `packages/core/src/index.ts`
- `examples/proof/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Environment/file configuration.
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
beta-007-a: environment file configuration
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-007-A) — PASS (2026-09-04)

- Branch/PR: beta-007-a (squash-merged; see git log for final hash)
- Closes: #539

### Behavior implemented

Typed, versioned, fail-closed environment/file configuration in the
new `crates/q-runtime/src/config.rs`: per-field layer stack
`CLI > env > file > default`; required `configVersion` (only 1
accepted); `deny_unknown_fields`; declared ranges (body 1..=64 MiB,
queue 1..=10 000, logSample 0..=1e9, port 1..=65535) that reject
startup — never clamp; typed `startup.rejected` (exit 2) before
engine/listener; closed log-mode set via
`serve::LogMode::parse_checked`; host shape validation (printable
ASCII, ≤253 bytes); legacy unversioned config files and silent
`PORT`/`--log` fallbacks deliberately retired.

### Changed files

- `crates/q-runtime/src/config.rs` (new module + 19 tests)
- `crates/q-runtime/src/lib.rs` (RunConfig Options; resolve wiring;
  structured config rejection)
- `crates/q-runtime/src/serve.rs` (`LogMode::parse_checked`/`as_str`)
- `crates/q-runtime/src/main.rs`, `crates/q-runtime/src/bin/velqu-standalone.rs`
  (CLI layers become optional)
- `crates/q-runtime/tests/runtime_conformance.rs` (fixture hardened to
  versioned schema)
- `examples/config/velqu.config.json` (new example, CI parse-tested)
- `docs/beta/CONFIGURATION.md` (new: defaults/bounds/env/precedence/
  examples/non-goals)
- `docs/reports/beta-007-a-environment-file-configuration.md` (new)

### Required evidence

- **Config tests**: 19 in `config::tests` (layering, fail-closed
  matrix, canonicalization, VELQU_CONFIG selection).
- **Redaction tests**: `redaction_unrelated_env_never_appears_in_errors`,
  `redaction_file_read_error_reports_path_not_contents`.
- **Examples**: `examples/config/velqu.config.json` +
  `docs/beta/CONFIGURATION.md` invocation examples.

### Guardrail proofs

1. **Invalid config fails before ready** — resolution runs before the
   tokio runtime/engine/listener; typed rejection JSON, exit 2.
2. **Secrets never appear in inspect/log/error** — config layer never
   reads credentials; redaction tests pin error rendering.
3. **Defaults are safe** — historical Limits posture preserved
   (127.0.0.1:3000, 1 MiB body, 256 queue, errors logging).
4. **Configuration is documented/versioned** — `configVersion`
   mandatory; CONFIGURATION.md is the canonical reference.

### Gate results (fresh on this branch)

- `cargo test -p velqu-runtime` -> 82 lib + 35 conformance + 16
  fetch/source-map, 0 failures; `-p q-http` 14; `-p q-bridge` 11
- fmt / clippy (`-D warnings`) / typecheck -> clean
- `bun test` -> 434 pass / 0 fail (67 files)
- `./scripts/verify` -> ALL PASS (M0-M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)
- `./scripts/validate-okf` -> PASS
  (verify run inside an isolated netns; standing port-3000 environment
  note, BETA-002-C record. No test weakened; the queue-limit conformance
  fixture now uses the versioned schema that this packet makes mandatory.)
