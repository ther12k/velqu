# BETA-016-E — Deploy proof service (external user)

## Overview

Acts as the external user in the BETA-016-A/B/C/D environment and
deploys the proof service end to end in the documented shape
(`docs/beta/INSTALL.md` + `docker-compose.beta.yml`): the proof app is
built from the installed tree, the production runtime serves the pack
in shared mode bound to loopback (reverse-proxy-first), an edge proxy
publishes a loopback-only port, the service is verified through the
edge, and the whole deployment is rolled back cleanly.

## Deliverable

- `scripts/beta-external/deploy-proof-service.sh` — fail-closed deploy
  with four subcommands mirroring a real deployment split:
  - `app` (unprivileged user): build pack, run
    `velqu-runtime --pack … --proxy-mode reverse-proxy` in the
    background, wait for readiness, assert the ready log reports
    `"proxyMode":"reverse-proxy"`.
  - `edge` (operator/root): install nginx, write the loopback
    `:8080 → 127.0.0.1:3000` site, validate config, start.
  - `verify` (any user): probe `/health/live`, `/hello/beta`,
    `/health/ready` through the edge with exact-body assertions.
  - `rollback` (operator/root): remove edge site, SIGTERM the service
    and require exit, remove `dist/` artifacts + pidfile.

## External transcript (key facts)

- Same environment: user `beta`, image `sha256:9076de16f6ec…a2f5570`,
  install tree `~/velqu` (with the BETA-016-D CLI fixes).
- `app`: proof pack built from `~/velqu/examples/proof`; runtime up on
  `127.0.0.1:3000`, loopback only, reverse-proxy mode asserted from the
  startup log — `APP-OK pid=4181`.
- `edge`: nginx installed and configured as operator tooling,
  `nginx -t` clean — `EDGE-OK 127.0.0.1:8080 → 127.0.0.1:3000`.
- `verify` (through the edge): `/health/live` → `{"status":"ok"}`;
  `/hello/beta` → `{"message":"Hello beta"}` (INSTALL.md bodies
  verbatim); `/health/ready` → ready — `VERIFY-OK`.
- `rollback`: edge site removed, service stopped gracefully via
  SIGTERM (exit enforced), `dist/` + pidfile removed, pack absence
  asserted — `ROLLBACK-OK`.
- Post-rollback `verify` fails closed (`edge /health/live failed`) —
  the rollback is real, not cosmetic.

## Environment manifest

Unchanged from BETA-016-B (image digest `sha256:9076de16f6ec…a2f5570`,
probe `MANIFEST-OK`). The deploy adds only run-time state (nginx
package, removed at rollback) and build artifacts (removed at
rollback).

## Issues and resolutions

1. **Orchestration error (transcript, first attempt):** the `edge`
   subcommand was invoked without root, refused with a clear message
   ("run 'edge' as root (operator provisioning)"), and the subsequent
   verify failed closed. Resolution: re-ran with operator privileges;
   the refusal messages behaved exactly as designed — recorded as
   evidence, not suppressed.
2. No Velqu product defects: the proof pack built and served correctly
   in the documented posture on the first clean attempt.

## Parent guardrails advanced

- **Tutorial succeeds verbatim** — INSTALL.md's endpoints and posture
  (loopback bind, reverse-proxy-first, `{"message":"Hello beta"}`)
  reproduced exactly.
- **Failures produce actionable diagnostics** — subcommand privilege
  requirements and step-numbered failures; post-rollback failure
  confirms probes are not vacuous.
- **Artifacts can be rolled back/uninstalled** — demonstrated:
  graceful stop enforced, artifacts and edge removed, absence asserted.

## Gates (this worktree)

- `bun test` — 434 pass / 0 fail (67 files, in `unshare -rn` netns)
- `bun run typecheck` — pass

## Disclosures

- The nginx edge is platform-operator tooling (root inside the
  container), mirroring `docker-compose.beta.yml`'s operator-managed
  edge; the Velqu service itself runs as the unprivileged user.
- The public docker-image deploy alternative (building the repository
  `Dockerfile`) remains Owner-gated publishing territory; this packet
  verifies the binary+pack shared-mode deployment that INSTALL.md
  documents as the primary path.
- Standing CI disclosure applies (zero-step verify workflows since
  ~#714); local gates are the acceptance basis.
