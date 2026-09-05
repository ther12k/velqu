# BETA-016-C — Scaffold app (external user)

## Overview

Acts as the external user inside the BETA-016-A/B environment: scaffolds
a starter app with the installed CLI using the quickstart command
verbatim (`create hello-velqu --name hello-velqu`,
`docs/beta/QUICKSTART.md`), links the workspace packages exactly as the
documentation instructs, and structurally verifies the scaffold with
`velqu check` (3 routes, clean).

## Deliverable

- `scripts/beta-external/scaffold-app.sh` — fail-closed external
  scaffold verification: clean slate → `create` (quickstart command) →
  documented `@velqu/{core,schema,treaty}` links → structure checks
  (package.json, src/app.ts, health route present) → `velqu check`
  must report 3 routes; prints the uninstall path.

## External transcript (key facts)

- Same environment as BETA-016-B: user `beta` in
  `velqu-beta-external:0.1.0-beta.1`
  (digest `sha256:9076de16f6ec…a2f5570`), installed tree `~/velqu`
  from commit `cfe3604`.
- `create hello-velqu --name hello-velqu` produced: `package.json`,
  `tsconfig.json`, `README.md`, `src/app.ts`,
  `src/modules/health/routes.ts`, `src/modules/greetings/routes.ts`
  (serverless profile, fetch disabled).
- Links: `node_modules/@velqu/{core,schema,treaty}` → install tree.
- `velqu check --project ~/hello-velqu` →
  `velqu check: 3 routes in /home/beta/hello-velqu — clean`.
- Result: `SCAFFOLD-OK`; uninstall `rm -rf ~/hello-velqu`.

## Environment manifest

Unchanged from BETA-016-B (image digest `sha256:9076de16f6ec…a2f5570`,
probe `MANIFEST-OK`, `tooling_homes_writable=yes`,
`fresh=no-velqu-material` at image level; the scaffold adds only
`~/hello-velqu`).

## Issues and resolutions

1. **CLI diagnostics wart (recorded, not fixed — out of scope):**
   `create --help` does not print help; it silently scaffolds a default
   project (`my-velqu-app`) into the current directory. The journey
   itself is unaffected — the quickstart command works verbatim — but
   `--help` on a subcommand alias should print help rather than act.
   Recorded for the parent BETA-016 rollup; a fix would belong to a
   CLI-focused packet.
2. **Probe pollution incident:** the `--help` probe above created stray
   scaffold files directly in `$HOME`. Resolution: removed the stray
   files, then re-ran the verification from a pristine home; the
   transcript above is from the pristine run.
3. No product code path failed; scaffold and check behaved as
   documented on the first clean attempt.

## Parent guardrails advanced

- **Tutorial succeeds verbatim** — the QUICKSTART `create` command and
  the documented link step were used unchanged.
- **No local unpublished dependency** — links resolve into the
  installed tree (from the release archive), not a checkout.
- **Failures produce actionable diagnostics** — the script fails with
  a step number and cause on any deviation.
- **Rollback/uninstall** — single app directory; uninstall printed.

## Gates (this worktree)

- `cargo test -p velqu-runtime` — 37+3 pass, 0 fail
- `bun test` — 434 pass / 0 fail (67 files, in `unshare -rn` netns)
- `bun run typecheck` — pass

## Disclosures

- Verification only; no Velqu code paths changed.
- Standing CI disclosure applies (zero-step verify workflows since
  ~#714); local gates are the acceptance basis.
