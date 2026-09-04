# BETA-008-A — Trusted Proxy Configuration

## Behavior implemented

The runtime now has an explicit, typed deployment boundary posture:

- `proxyMode: "reverse-proxy"` is the safe default. It requires a
  loopback bind (`127.0.0.1`, `::1`, `[::1]`, or `localhost`) and rejects
  public binds before the engine/listener is ready.
- `proxyMode: "direct"` is an explicit operator opt-in. It permits a
  non-loopback bind only after the operator accepts ownership of the direct
  boundary's TLS, access-control, forwarding-header, and exposure
  consequences. It does **not** add runtime TLS or make forwarded headers
  trusted.
- The setting is resolved by the typed configuration layer (`CLI > env >
  file > default`) through `--proxy-mode`, `VELQU_PROXY_MODE`, or the
  versioned file key `proxyMode`. Profile blocks cannot alter the deployment
  boundary.
- The ready line reports `config.proxyMode` and `config.proxyModeSource`.
- ADR-0034's trust rule remains explicit: `X-Forwarded-*`, RFC 7239
  `Forwarded`, and `Host` are ordinary untrusted data, never runtime identity,
  authentication, authorization, scheme, or routing input.

## Evidence

- `scripts/proxy-smoke.sh` is a container-friendly smoke that starts the
  release runtime privately, checks `/health/live` plus `/hello/smoke`, checks
  the ready-line `proxyMode` and loopback address, sends SIGTERM, and requires
  prompt deterministic process exit. Output:
  `PROXY-SMOKE-OK: loopback reverse-proxy posture, health/route, deterministic SIGTERM`.
- `docs/beta/governance/TRUSTED_PROXY_RUNBOOK.md` documents the proxy boundary,
  firewall/TLS/header checklist, rollout and shutdown sequence, failure
  diagnosis, and container smoke command.
- `docs/beta/DEPLOYMENT-REVERSE-PROXY.md` now identifies the typed proxy mode,
  loopback guard, direct opt-in, and forwarded-header distrust.
- `docs/beta/CONFIGURATION.md` documents the field, env variable, default, and
  ready-line reporting.

## Tests

New `crates/q-runtime/src/config.rs` tests:

- `proxy_mode_defaults_to_reverse_proxy_and_validates_bind`
- `proxy_mode_layers_and_is_case_insensitive`
- `invalid_proxy_mode_fails_closed`
- `startup_config_reports_proxy_mode_without_secrets`
- `unknown_proxy_mode_file_field_is_rejected_by_schema`

The full runtime library suite passes with the added proxy tests; q-http and
q-bridge suites remain green. No forwarded-header parsing was added (that is
BETA-008-B), and no readiness/drain implementation was pulled forward (those
are BETA-008-C/D).

## Gates

- `cargo test -p q-http` — pass
- `cargo test -p q-bridge` — pass
- `cargo test -p velqu-runtime` — pass
- `cargo fmt --all --check` — pass
- `cargo clippy --workspace --all-targets -- -D warnings` — pass
- `./scripts/validate-okf` — pass
- `./scripts/verify` — ALL PASS
- `bun test` — 434 pass / 0 fail
- `bun run typecheck` — pass
- `scripts/proxy-smoke.sh` — PROXY-SMOKE-OK

## Boundary disclosures

- Reverse-proxy mode is a bind safety guard and deployment contract, not a
  substitute for firewall policy or a hostile-code sandbox.
- Forwarded headers remain untrusted by design; the next packet owns their
  explicit policy surface.
- Direct mode is intentionally an owner/operator decision and carries no
  implicit TLS or authentication guarantee.
