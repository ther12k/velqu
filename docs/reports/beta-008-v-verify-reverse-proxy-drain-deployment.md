# BETA-008-V — Verify Reverse Proxy, Drain, and Deployment Semantics

## Acceptance mapping

| Parent guardrail | Source and evidence |
| --- | --- |
| Spoofed forwarding headers are ignored unless trusted | `q-http::tests::forwarded_headers_are_data_not_identity`; `q-runtime::tests::forwarded_headers_are_ordinary_data_and_never_identity`; ADR-0034 distrust list; Host never routes. |
| Readiness drops before drain | Native `/health/ready` contract; SIGTERM drain gate emits `drain.begin` before `shutdown.complete`; runtime runbook requires proxy removal/readiness withdrawal before signal. |
| In-flight requests honor deadline | `drain_lets_in_flight_request_complete` completes 800 ms request; `drain_waits_bounded_then_detaches_straggler_connection` force-aborts 20 s work at the frozen 5 s budget, settles ownership once, pending 0. |
| Container shutdown exits deterministically | `scripts/container-smoke.sh` and `scripts/proxy-smoke.sh` both require prompt SIGTERM exit; Dockerfile/compose stop signal and grace period are explicit. |

## Evidence inventory

- BETA-008-A typed `proxyMode` configuration and loopback guard.
- BETA-008-B closed forwarded-header distrust list and TCP-peer-only identity.
- BETA-008-C native liveness/readiness and structured startup identity.
- BETA-008-D drain gate, bounded in-flight completion, force-abort ownership,
  fetch-pool drain, deterministic `shutdown.complete`.
- BETA-008-E Dockerfile, compose example, non-root runtime, and container
  contract smoke.
- `docs/beta/DEPLOYMENT-REVERSE-PROXY.md` and
  `docs/beta/governance/TRUSTED_PROXY_RUNBOOK.md` operational runbook.

## Verification commands

- `cargo test -p q-engine-quickjs` — pass (24 lib + 117 worker + doc support)
- `cargo test -p q-http` — pass (8 lib + 6 fuzz + 1 regression)
- `cargo test -p q-bridge` — pass (11 lib)
- `cargo test -p velqu-runtime` — pass (102 lib + 37 conformance)
- `cargo fmt --all --check` — pass
- `cargo clippy --workspace --all-targets -- -D warnings` — pass
- `./scripts/validate-okf` — pass
- `./scripts/verify` — ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE), isolated netns
- `bun test` — 434 pass / 0 fail (67 files)
- `bun run typecheck` — pass
- `scripts/proxy-smoke.sh` — `PROXY-SMOKE-OK`
- `scripts/container-smoke.sh` — `CONTAINER-SMOKE-OK`
- `docker compose -f docker-compose.beta.yml config` — pass

## Boundary disclosures

- Verification confirms the beta deployment contract; it does not claim
  native runtime TLS, trusted forwarded identity, a durable queue, or a
  hostile-code sandbox.
- Docker daemon/image build is separately operator-environment work; the
  repository provides syntax-validated definitions and a deterministic runtime
  contract smoke.
- Standing CI disclosure: repository verify workflows have stalled/failed with
  zero executed steps at PR creation since roughly #714; local gates above are
  the acceptance evidence.
