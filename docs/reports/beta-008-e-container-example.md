# BETA-008-E — Container Example

## Deliverables

- `Dockerfile`: multi-stage Bun/Rust build. The final Debian slim image
  contains only the release `velqu-runtime` and verified proof `app.qpack`,
  runs as UID/GID 10001 non-root, uses the reverse-proxy-first loopback
  posture, exposes port 3000, and has SIGTERM/stop-grace configuration in the
  compose example.
- `docker-compose.beta.yml`: local beta service example with loopback-only
  host publishing (`127.0.0.1:3000`), explicit direct mode only where the
  compose boundary owns exposure, readiness healthcheck, SIGTERM, and a
  bounded 10-second stop grace period.
- `.dockerignore`: excludes build outputs, dependencies, and logs from build
  context.
- `scripts/container-smoke.sh`: deterministic container-contract smoke that
  mirrors the final image's non-root/private runtime command when a Docker
  daemon is unavailable; checks readiness, real route, proxy posture, and
  SIGTERM exit. Output: `CONTAINER-SMOKE-OK`.

The existing `scripts/proxy-smoke.sh` remains the release runtime smoke for
loopback reverse-proxy behavior. The Dockerfile does not copy source maps,
private keys, credentials, or development tooling into the final image.

## Runbook

`docs/beta/governance/TRUSTED_PROXY_RUNBOOK.md` and
`docs/beta/DEPLOYMENT-REVERSE-PROXY.md` now cross-reference the container
surface: build the proof pack, build the image, publish only through a trusted
edge, wait for readiness, remove the old upstream before SIGTERM, and honor
the bounded drain report.

## Evidence

- `scripts/container-smoke.sh` → `CONTAINER-SMOKE-OK`.
- Dockerfile is syntactically inspectable and uses pinned Bun `1.4.0`,
  non-root UID 10001, loopback/private defaults, and release-only runtime.
- Compose healthcheck uses `/health/ready`; stop signal is SIGTERM.

## Gates

- `cargo test -p velqu-runtime` — pass (102 lib + 37 conformance)
- `cargo fmt --all --check` / clippy `-D warnings` — pass
- `./scripts/validate-okf` — pass
- `./scripts/verify` — ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE)
- `bun test` — 434 pass / 0 fail (67 files)
- `bun run typecheck` — pass
- `scripts/container-smoke.sh` — `CONTAINER-SMOKE-OK`

## Disclosures

- Docker daemon/image build was not used as the acceptance gate in this
  environment; the deterministic runtime-contract smoke is the raw evidence.
  Operators should run `docker build` and `docker compose -f
  docker-compose.beta.yml up` in their deployment environment.
- The final image remains a reverse-proxy backend, not a public TLS endpoint;
  certificates and access control belong at the trusted edge.
- Container shutdown delegates to the BETA-008-D runtime drain budget; it is
  not a durable work queue.
