# BETA-008-C — Liveness, Readiness, and Startup Endpoints

## Behavior

The runtime's health surface is now explicitly documented and black-box tested:

- `/health/live` is a native liveness response (`{"status":"ok"}`) with
  `x-velqu-stage: native`; it does not enter JavaScript and remains a process /
  listener check.
- `/health/ready` is a native readiness response (`{"ready":true}`) while the
  engine is healthy, and returns 503 with the stable `engine quarantined`
  problem after quarantine. GET and HEAD are both supported; HEAD has no body.
- Startup emits one structured JSON `ready` identity line containing mode,
  bound address, engine/runtime identity, routes/handlers, context/service
  profiles, non-secret resolved configuration/provenance, contract hash,
  startup timing, stage timings, and bundle evaluation timing.
- Health endpoints are safe to put behind the reverse proxy: proxy health
  checks do not depend on forwarded headers or application handler execution.

## Tests

- Added `health_endpoints_are_native_and_startup_identity_is_structured` to
  `crates/q-runtime/tests/runtime_conformance.rs`: ready-line JSON fields,
  forged Host on native liveness/readiness, exact liveness bytes/stage, GET +
  HEAD behavior, and readiness JSON/stage.
- Existing `poisoned_runtime_marks_readiness_false` continues to pin liveness
  200/readiness 503 after engine quarantine and HEAD no-body behavior.

## Documentation

- `docs/beta/DEPLOYMENT-REVERSE-PROXY.md` now specifies endpoint methods,
  native handling, status/body contracts, and rollout usage.
- `docs/reports/beta-008-c-liveness-readiness-startup-endpoints.md` packages
  the evidence for this packet.

## Gates

- `cargo test -p velqu-runtime` — pass (102 lib tests; runtime conformance
  includes 37 tests)
- `cargo fmt --all --check` — pass
- `cargo clippy --workspace --all-targets -- -D warnings` — pass
- `./scripts/validate-okf` — pass
- `./scripts/verify` — ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE)
- `bun test` — 434 pass / 0 fail (67 files)
- `bun run typecheck` — pass

## Boundary disclosures

- Liveness/readiness are runtime-native health signals, not authentication
  endpoints and not a substitute for proxy/firewall policy.
- The ready line is a fixed, non-secret startup identity surface; secrets are
  excluded by the existing configuration/redaction contract.
- Drain admission, in-flight deadlines, and container shutdown remain in the
  following BETA-008-D/E packets.
