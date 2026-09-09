---
task_id: BETA-007-Z
parent_task: BETA-007
milestone: BETA
priority: P0
mode: EVIDENCE
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-007-Z — Package evidence for Implement configuration and secret handling

## Atomic goal

Create source-backed evidence and handoff for parent task BETA-007; update status only if verification passed.

## Parent intent

Provide typed configuration without top-level network I/O or accidental secret disclosure.

## Dependencies

- `BETA-007-V` — `tasks/08_public_beta/BETA-007-V-verify-implement-configuration-and-secret-handling.md`

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
- `packages/cli/src/index.ts`
- `crates/q-runtime/src/source_map.rs`
- `examples/proof/`
- `README.md`
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
```bash
./scripts/validate-okf
```

## Required evidence for this microtask

- Config tests.
- Redaction tests.
- Examples.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
beta-007-z: package evidence for implement configuration and secret hand
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-007-Z) — PASS (2026-09-04)

- Branch/PR: beta-007-z (squash-merged; see git log for final hash)
- Closes: #545
- Parent verification: BETA-007-V PASS (PR #1144); this packet packages
  the source-backed evidence across all child packets (A through E + V)
  and flips parent task BETA-007 to PASS in
  `docs/beta/04_TASK_LEDGER.md`.

### Evidence package

- **Implementation packets (squash-merged):**
  - BETA-007-A (PR #1139): typed environment/file configuration —
    per-field layer stack (CLI > env > file > default), mandatory
    `configVersion: 1`, declared ranges that reject (never clamp),
    fail-closed startup rejection before engine/listener, silent
    `PORT`/`--log` fallbacks retired.
  - BETA-007-B (PR #1140): validation at startup — closed `VELQU_*`
    environment namespace (19 documented names; unknown names reject
    with values never read/echoed) and the ready-line `config` block
    with per-field provenance.
  - BETA-007-C (PR #1141): `SecretString` wrapper — Debug/Display
    `[redacted]`, explicit `expose()` as the only read path, database
    URL wrapped at the env boundary.
  - BETA-007-D (PR #1142): profile-specific settings — named profile
    blocks overlaying the file layer (`CLI > env > profile > file >
    default`), `VELQU_PROFILE` selection, fail-closed name/declaration
    validation, `activeProfile` + `profile` provenance in the ready
    line.
  - BETA-007-E (PR #1143): no dynamic code execution — the `eval`
    global (direct and indirect), the `Function` global, and the
    function/async/generator prototype constructor routes fail with a
    typed `TypeError`; lockdown installs before any application code
    in every profile.
  - BETA-007-V (PR #1144): verification closure; fresh full-gate run
    reproduces.

### Required evidence

- **Config tests**: 32 config tests in `velqu-runtime` (layering,
  fail-closed matrix, namespace closure, profiles, provenance,
  startup report) + 4 lockdown tests in the engine; raw counts in the
  V record.
- **Redaction tests**: value-never-echoed (B), `[redacted]` wrapper
  (C), config-error path/contents discipline (A), fixed non-secret
  config-block allowlist (B), layered BETA-006-F/BETA-004 redaction
  still green.
- **Examples**: `examples/config/velqu.config.json` (CI parse-tested);
  `docs/beta/CONFIGURATION.md` (layer stack, bounds, env table, closed
  namespace allowlist, profiles, secret wrapper, startup report);
  `docs/beta/LIMITS-AND-NON-GOALS.md` (dynamic-code guarantee).

### Parent guardrail proofs

1. **Invalid config fails before ready** — one fail-closed
   `config.resolve` stage before tokio/engine/listener; typed
   rejection JSON, exit 2.
2. **Secrets never appear in inspect/log/error** — wrapper `[redacted]`
   rendering, single grep-auditable `expose()`, namespace check never
   reads unknown values, fixed non-secret config block.
3. **Defaults are safe** — historical posture preserved; no profile =
   no profile layer; lockdown unconditional, no opt-out.
4. **Configuration is documented/versioned** — mandatory
   `configVersion: 1`; CONFIGURATION.md is the canonical reference;
   additions require code + docs together (namespace, profiles).

### Gate results (fresh on this branch)

- `cargo test -p q-engine-quickjs` 24 lib + 117 worker + 1 /
  `-p q-schema-runtime` 67 / `-p velqu-runtime` 96 lib + 35
  conformance + 16 fetch/source-map / `-p q-http` 14 / `-p q-bridge`
  11 — 0 failures
- fmt / clippy (`-D warnings`) / typecheck -> clean
- `bun test` -> 434 pass / 0 fail (67 files)
- `./scripts/verify` -> ALL PASS (M0-M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)
- `./scripts/validate-okf` -> PASS
  (verify run inside an isolated netns; standing port-3000 environment
  note, BETA-002-C record. No test weakened.)

### Ledger

- `docs/beta/04_TASK_LEDGER.md`: BETA-007 flipped TODO -> **PASS**.
- STATUS.md and TASK_INDEX.md updated to PASS (BETA-007-Z row).
