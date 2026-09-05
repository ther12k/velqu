# BETA-016-F — Use Treaty client (external user)

## Overview

Acts as the external user in the beta environment and drives the live
runtime through the scaffold's type-safe Treaty client: the dev server
is started on `127.0.0.1:3000` (the client's documented base URL), the
scaffold's client example makes typed calls (health, create greeting,
fetch by route param), and the scaffold test suite runs asserting the
Treaty contract tests execute against the live runtime rather than
self-skipping.

**This packet also corrects a compromised evidence claim from
BETA-016-E** (discovered by this verification's pre-flight guard) and
adds leak-proof process hygiene to the external verification tooling.

## Deliverable

- `scripts/beta-external/use-treaty-client.sh` — fail-closed treaty
  journey: pre-flight port-free check → dev server start → **identity
  precheck** (a greetings route must answer, proving the listener is
  the scaffold and not a foreign service) → `bun run client` with
  exact-output assertions → `bun test` with an explicit no-skip
  assertion → leak-proof teardown with port-release assertion.
- `scripts/beta-external/deploy-proof-service.sh` — rollback corrected
  (see E correction below).

## External transcript (key facts; two consecutive clean runs)

- Same environment: user `beta`, image `sha256:9076de16f6ec…a2f5570`,
  scaffold `~/hello-velqu` with the BETA-016-D fixes.
- Pre-flight: port 3000 free; precheck `GET /greetings/precheck` →
  `{"message":"Hello, precheck!"}` (scaffold identity confirmed).
- `bun run client` → `Health OK: ok`, `Created greeting: {…}`,
  `Message: Greetings from Treaty!` — typed calls through the live
  runtime, matching the scaffold template's documented flow.
- `bun test` → 5 pass / 0 fail with **no** "skipping: dev server not
  reachable" warning — the Treaty contract tests ran against the live
  runtime.
- Teardown: port 3000 released; second run passes identically
  (repeatability, no leak).

## Issues and resolutions

1. **BETA-016-E rollback claim was partially vacuous — corrected.**
   E's rollback ran as root, so `$HOME` resolved to `/root`: the
   pidfile/artifact paths silently pointed at nothing, the service stop
   was skipped ("no pidfile" branch), and `ROLLBACK-OK` was reported
   while the proof runtime was still listening on 3000. This packet's
   pre-flight guard caught it (foreign 404s on greeting probes).
   Resolution: `deploy-proof-service.sh` rollback now resolves paths
   via the owning user's passwd entry, **fails closed** on a missing
   pidfile, asserts the edge port closed, and asserts the upstream port
   released. E's lifecycle was re-run end to end with the corrected
   script: `APP-OK → EDGE-OK → VERIFY-OK → ROLLBACK-OK` (service exit
   proven via kernel state) `→ post-rollback verify fails closed`.
2. **Zombie processes defeat `kill -0` liveness checks.** The container
   init (`sleep infinity`) never reaps children, so the gracefully
   exited runtime (kernel state `Z`, ~200 ms after SIGTERM — the
   RUN-008 contract holds) still satisfied `kill -0`. Resolution: the
   rollback reads `/proc/<pid>/stat` state instead of trusting
   `kill -0`, and the behavioral port-release assertion is the primary
   guarantee. Product behavior itself is correct.
3. **`pkill` is absent in the beta image (no procps)** — the earlier
   teardown's `pkill … || true` was a silent no-op, leaving dev-server
   process trees leaked across runs. Resolution: teardown now scans
   `/proc` cmdlines directly (own-user processes only, pure bash) and
   kills the full `bun run → dev CLI → spawned runtime` tree; the
   port-release assertion proves the teardown.

## Parent guardrails advanced

- **Tutorial succeeds verbatim** — the scaffold's client example and
  contract tests work exactly as written against the live dev server.
- **Failures produce actionable diagnostics** — demonstrated twice:
  the pre-flight guard named the leftover-service cause, and the
  corrected rollback refuses to guess when the pidfile is missing.
- **Artifacts can be rolled back/uninstalled** — teardown proven by
  port-release assertion and an immediately repeated clean run.
- **No local unpublished dependency** — the treaty stack resolves
  through the documented workspace links into the installed tree.

## Gates (this worktree)

- `cargo test -p velqu-runtime` — 37+3 pass, 0 fail
- `bun test` — 434 pass / 0 fail (67 files, in `unshare -rn` netns)
- `bun run typecheck` — pass

## Disclosures

- The E report's original rollback paragraph overclaimed service-stop
  verification; this packet's Issues section documents the correction
  and the re-run evidence supersedes it.
- Dev-server process hygiene (orphaned children across `bun run`
  nesting) is tooling-level; no Velqu product change is carried here.
- Standing CI disclosure applies (zero-step verify workflows since
  ~#714); local gates are the acceptance basis.
