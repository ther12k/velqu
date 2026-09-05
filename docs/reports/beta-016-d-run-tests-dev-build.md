# BETA-016-D — Run tests/dev/build (external user)

## Overview

Acts as the external user in the BETA-016-A/B/C environment and
exercises the documented development loop end to end on the scaffolded
app: `bun run test`, `bun run build` (twice — verifying the
deterministic-bytes claim), `bun run check`, the `velqu dev` reload
loop probed over HTTP, and the production `velqu-runtime` serving the
built pack.

**This packet also fixes three real external-journey defects the
verification surfaced** — the tutorial's in-scaffold commands did not
work outside the monorepo before this packet.

## Deliverables

Product fixes:

- `packages/cli/src/scaffold.ts` — scaffold scripts now invoke the CLI
  through its workspace-linked path with an explicit project:
  `bun node_modules/@velqu/cli/src/index.ts {dev,build,check} --project .`
  (a bare `velqu` binary does not exist externally; the CLI's bare
  `--project` default assumes the monorepo's `examples/proof`).
- `packages/cli/src/dev-server.ts` — `findRuntimeBinary` additionally
  looks in the install tree the CLI ships in
  (`packages/cli/src → ../../../target/…`, identical layout in-repo and
  externally); the not-found error now names the build command and the
  `VELQU_RUNTIME` override.
- `packages/cli/src/profile-fetch-choices.test.ts` — script assertions
  updated to the fixed template contract (same properties, new values).
- `docs/beta/QUICKSTART.md` — documented link step now includes `cli`
  (required for the in-scaffold scripts to work).

Verification tooling:

- `scripts/beta-external/run-tests-dev-build.sh` — fail-closed external
  journey: documented link step → test → build ×2 (sha256 equality) →
  check → dev server on :8084 probed → production runtime on :8081
  probed.
- `scripts/beta-external/scaffold-app.sh` — link loop aligned with the
  documented step (adds `cli`).

## External transcript (key facts)

- Same environment: user `beta`, image `sha256:9076de16f6ec…a2f5570`,
  install tree `~/velqu` with the two fixed CLI files from this branch
  synced in (the packet under test).
- `bun run test` — scaffold suite passes.
- `bun run build` ×2 — `velqu build [serverless]: 3 routes` with the
  full artifact set (app.qpack 12317B, route/schema/capability
  manifests, contract + lock, OpenAPI, build report);
  `app.qpack sha256=cb00bc37555786120429da0e6f769d7f1e66802515d1f17f9a92edbf82952118`
  identical across builds (determinism claim holds externally).
- `velqu dev … --port 8084` →
  `{"status":"ok"}` on `/health/live`, `{"message":"Hello, dev!"}` on
  `/greetings/dev`.
- `velqu-runtime --pack dist/app.qpack --port 8081` →
  `{"status":"ok"}`, `{"message":"Hello, world!"}`.
- `DEVBUILD-OK`; artifacts confined to the app dir (`dist/`),
  uninstall `rm -rf ~/hello-velqu`.

## Issues and resolutions

Each defect below was surfaced by the fail-closed script with a
step-numbered, cause-naming error (the "actionable diagnostics"
guardrail working as intended):

1. **`bun run build` → `velqu: command not found`.** The scaffold
   scripts invoked a bare `velqu` binary that no installation path
   provides externally (`@velqu/cli` declares `bin`, but external users
   never run an installer inside the scaffold — deps resolve via
   documented links). Resolution: template scripts call the CLI via
   `bun node_modules/@velqu/cli/src/index.ts`.
2. **CLI default project leaked a monorepo assumption**
   (`--project` defaults to `examples/proof`; externally:
   "project path not found: examples/proof"). Resolution: scaffold
   scripts pass `--project .`.
3. **Dev server could not find the runtime binary** — it searched only
   the scaffold's own `target/`, while the external runtime lives in
   the install tree. Resolution: install-tree-relative candidates in
   `findRuntimeBinary` (correct in both environments) + actionable
   error naming `cargo build`/`VELQU_RUNTIME`.
4. **QUICKSTART link step omitted `cli`**, so the fixed scripts' target
   would not resolve. Resolution: documentation updated; `scaffold-app.sh`
   aligned.
5. Verification-script hygiene: `wait` after killing the dev/runtime
   servers returned their termination status; now tolerated explicitly.

## Parent guardrails advanced

- **Tutorial succeeds verbatim** — after the fixes, every QUICKSTART
  command works word-for-word outside the monorepo.
- **No local unpublished dependency** — resolution uses only the
  documented links into the installed tree.
- **Failures produce actionable diagnostics** — demonstrated in
  practice; the fixes also improve the CLI's own not-found error.
- **Rollback/uninstall** — artifacts confined to the app directory.

## Gates (this worktree)

- `cargo test -p velqu-runtime` — 37+3 pass, 0 fail
- `bun test` — 434 pass / 0 fail (67 files, in `unshare -rn` netns)
- `bun run typecheck` — pass
- `cargo fmt --all --check` — pass
- Scaffold-affected tests (`profile-fetch-choices`, `clean-install`,
  `exit-codes`) — 16 pass / 0 fail, twice consecutively in netns (one
  earlier flaky fail was the host port-3000 collision class; see
  disclosures)

## Disclosures

- The container's install tree was built from archive `cfe3604` with
  this branch's `scaffold.ts` and `dev-server.ts` synced in — the
  packet under test is this branch; the release-packet regeneration at
  the next packaging point will carry the fixes natively.
- Host-network `bun test` runs collide with a non-Velqu dev server on
  `127.0.0.1:3000` (scaffold Treaty tests self-skip only when the port
  is free); netns runs are the acceptance basis.
- Standing CI disclosure applies (zero-step verify workflows since
  ~#714); local gates are the acceptance basis.
