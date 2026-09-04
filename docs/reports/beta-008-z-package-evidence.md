# BETA-008-Z — Package Evidence for Reverse-Proxy, Drain, and Deployment Semantics

## Parent closure

BETA-008 is **PASS**. Its implementation packets A–E and verification packet V
are merged through PR #1151. This evidence packet records the source-backed
acceptance mapping and a fresh gate run on the parent verification commit.

## Acceptance mapping

- **Spoofed forwarding headers are ignored unless trusted.** `q-http` carries
  the TCP peer from `TcpListener::accept`; its closed distrust list marks
  `X-Forwarded-*`, RFC 7239 `Forwarded`, and `Host` as ordinary data only.
  Black-box conformance proves forged metadata cannot authenticate or route.
- **Readiness drops before drain.** Native `/health/ready` is 200 only while
  the engine is healthy. SIGTERM flips `DrainGate` before `drain.begin`; the
  deployment runbook withdraws proxy admission/readiness before the signal.
- **In-flight requests honor deadline.** The 800 ms in-flight conformance
  completes during drain. The 20 s straggler is bounded by the 5 s budget,
  force-aborted through ownership, settled once, and reported with pending 0.
- **Container shutdown exits deterministically.** The multi-stage non-root
  Dockerfile/compose example sets SIGTERM and bounded grace. Both contract
  smokes require health/readiness, route, proxy posture, and prompt process
  exit.

## Evidence inventory

- `docs/reports/beta-008-a-trusted-proxy-configuration.md`
- `docs/reports/beta-008-b-forwarded-header-policy.md`
- `docs/reports/beta-008-c-liveness-readiness-startup-endpoints.md`
- `docs/reports/beta-008-d-graceful-drain-and-termination.md`
- `docs/reports/beta-008-e-container-example.md`
- `docs/reports/beta-008-v-verify-reverse-proxy-drain-deployment.md`
- `docs/beta/DEPLOYMENT-REVERSE-PROXY.md`
- `docs/beta/governance/TRUSTED_PROXY_RUNBOOK.md`
- `Dockerfile`, `docker-compose.beta.yml`, `.dockerignore`
- `scripts/proxy-smoke.sh`, `scripts/container-smoke.sh`

## Fresh gate results

- `cargo test -p velqu-runtime` — pass (102 lib + 37 runtime conformance)
- `cargo fmt --all --check` — pass
- `cargo clippy --workspace --all-targets -- -D warnings` — pass
- `./scripts/validate-okf` — pass
- `./scripts/verify` — ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE), isolated netns
- `bun test` — 434 pass / 0 fail (67 files)
- `bun run typecheck` — pass
- `scripts/proxy-smoke.sh` — `PROXY-SMOKE-OK`
- `scripts/container-smoke.sh` — `CONTAINER-SMOKE-OK`
- `docker compose -f docker-compose.beta.yml config` — pass

## Disclosures

- This is evidence/status packaging; no new runtime behavior is introduced.
- The Docker daemon image build is operator-environment work; repository
  definitions and the deterministic runtime-contract smoke are the evidence.
- Reverse-proxy-first is a deployment posture, not native TLS, an auth
  boundary, a durable queue, or a hostile-code sandbox.
- Standing CI disclosure: repository verify workflows have stalled/failed with
  zero executed steps at PR creation since roughly #714; local gates above are
  the acceptance evidence.
