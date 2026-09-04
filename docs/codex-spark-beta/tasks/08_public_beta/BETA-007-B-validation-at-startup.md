---
task_id: BETA-007-B
parent_task: BETA-007
milestone: BETA
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-007-B — Validation at startup

## Atomic goal

Validation at startup.

## Parent intent

Provide typed configuration without top-level network I/O or accidental secret disclosure.

## Dependencies

- `BETA-007-A` — `tasks/08_public_beta/BETA-007-A-environment-file-configuration.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/BETA.md`
- `context/components/engine-scheduler.md`
- `context/components/schema-codecs.md`
- `context/components/devex-beta.md`

### Source files

- `AGENTS.md`
- `docs/beta/`
- `scripts/verify`
- `package.json`
- `.github/workflows/verify.yml`
- `crates/q-schema-runtime/src/lib.rs`
- `crates/q-schema-runtime/tests/fuzz_validator.rs`
- `crates/q-engine-quickjs/src/convert.rs`
- `packages/schema/src/index.ts`
- `conformance/schema/schema.conformance.test.ts`
- `crates/q-runtime/src/main.rs`
- `packages/core/src/index.ts`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Validation at startup.
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
cargo test -p q-schema-runtime
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
beta-007-b: validation at startup
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-007-B) — PASS (2026-09-04)

- Branch/PR: beta-007-b (squash-merged; see git log for final hash)
- Closes: #540

### Behavior implemented

Startup validation pass completing BETA-007-A's typed surface:
(1) the `VELQU_*` environment namespace is closed — unknown names
reject startup with a typed `startup.rejected` before the engine or
listener exist; values of unknown names are never read or echoed;
(2) the ready line carries a `config` block with the resolved
non-secret values and per-field provenance (`FieldSources` tracked in
`config::resolve`), so the validated configuration is visible at
startup with a fixed, test-enforced key allowlist.

### Changed files

- `crates/q-runtime/src/config.rs` (KNOWN_ENV_VARS allowlist, FieldSource/
  FieldSources, UnknownEnvVar error, validate_env_namespace,
  startup_config_json, source tracking in resolve, 5 new tests)
- `crates/q-runtime/src/lib.rs` (namespace check wired into the
  config.resolve stage; `config` block in the ready line)
- `docs/beta/CONFIGURATION.md` (namespace allowlist + startup report
  sections)
- `docs/reports/beta-007-b-validation-at-startup.md` (new)

### Required evidence

- **Config tests**: `unknown_velqu_env_name_rejected_value_never_echoed`,
  `every_known_env_var_passes_the_namespace_check`,
  `namespace_check_ignores_non_velqu_names`,
  `resolved_sources_report_the_winning_layer`,
  `startup_config_json_is_an_exact_field_allowlist`.
- **Redaction tests**: value-never-echoed property pinned in the
  unknown-var test; config-block allowlist test proves no
  secret-shaped fields; A's redaction tests still green.
- **Examples**: CONFIGURATION.md rejection example + ready-line
  `config` block example.

### Guardrail proofs

1. **Invalid config fails before ready** — namespace check + resolve
   run in one fail-closed stage before tokio/engine/listener (exit 2).
2. **Secrets never appear in inspect/log/error** — unknown-var errors
   name the variable only; the config block is a fixed non-secret
   allowlist (test-enforced).
3. **Defaults are safe** — unchanged defaults; provenance makes
   "which layer won" visible.
4. **Configuration is documented/versioned** — allowlist documented
   in CONFIGURATION.md; additions require code+docs together.

### Gate results (fresh on this branch)

- `cargo test -p q-engine-quickjs` 138 / `-p q-schema-runtime` 67 /
  `-p velqu-runtime` 87 lib + 35 conformance + 16 fetch/source-map /
  `-p q-http` 14 / `-p q-bridge` 11 — 0 failures
- fmt / clippy (`-D warnings`) / typecheck -> clean
- `bun test` -> 434 pass / 0 fail (67 files)
- `./scripts/verify` -> ALL PASS (M0-M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)
- `./scripts/validate-okf` -> PASS
  (verify run inside an isolated netns; standing port-3000 environment
  note, BETA-002-C record. No test weakened.)
