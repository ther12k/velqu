# BETA-007-A — Environment/File Configuration

## What was built

Typed, versioned, fail-closed configuration for the production hosts
(`velqu-runtime`, `velqu-standalone`) in the new
`crates/q-runtime/src/config.rs`, replacing the previous ad-hoc
handling (untyped `--config` JSON with silent unknown-field acceptance
and no bounds; silent fallbacks for `PORT` and `--log`).

- **Layer stack, first hit wins:** `CLI flag > environment > config
  file > built-in default`. Documented in `docs/beta/CONFIGURATION.md`.
- **Versioned file schema:** `configVersion` is required; only version
  1 is accepted. Unversioned legacy files and unknown fields are
  rejected outright (`deny_unknown_fields`) — a typo can never
  silently disable a limit.
- **Fields:** `host`, `port`, `maxBodyBytes`, `maxQueue`, `log`,
  `logSample`. Env: `VELQU_HOST`, `VELQU_PORT` (wins over legacy
  `PORT`), `VELQU_LOG`, `VELQU_LOG_SAMPLE`, `VELQU_MAX_BODY_BYTES`,
  `VELQU_MAX_QUEUE`, `VELQU_CONFIG`.
- **Fail-closed, never clamped:** every numeric field has a declared
  range (body 1..=64 MiB, queue 1..=10 000, logSample 0..=1e9, port
  1..=65535). Out-of-range, wrong-type, and unknown-enum values reject
  startup with a typed `startup.rejected` line (`stage:
  "config.resolve"`, exit 2) BEFORE the engine or listener exist.
- **No more silent fallbacks:** an unparsable `PORT` (previously
  silently ignored) and an unknown `--log`/`VELQU_LOG` mode (previously
  silent `errors`) are now typed rejections. `serve::LogMode::parse_checked`
  expresses the closed set; `parse_mode` stays for already-validated
  canonical strings.
- **Host shape validation:** 1..=253 bytes printable ASCII (no
  whitespace/control characters), so the resolved host can never
  smuggle structure into the startup line.
- **RunConfig** CLI fields became `Option` (`host`, `log`,
  `log_sample`) so both deployment binaries share one resolution path.

## Secrets stay out of the surface

The configuration layer never reads capability credentials (e.g.
`VELQU_DATABASE_URL`); they remain environment-only. Rendered
`ConfigError`s name the field, source layer, and expected shape —
never file contents, never values of environment variables outside the
typed surface (both properties pinned by tests).

## Tests (19 new, in `config::tests`; plus fixture hardening)

- Layering: `defaults_when_nothing_is_configured`,
  `file_overrides_defaults`, `env_overrides_file`, `cli_overrides_env`,
  `legacy_port_env_still_works_and_velqu_port_wins`,
  `velqu_config_env_selects_file_cli_wins`
- Fail-closed: `unknown_file_field_fails_closed`,
  `missing_or_unsupported_config_version_fails_closed`,
  `file_type_errors_fail_closed`,
  `out_of_bounds_values_rejected_never_clamped`,
  `invalid_env_values_fail_closed`, `port_zero_rejected_everywhere`,
  `invalid_host_rejected`, `unknown_log_mode_fails_closed_everywhere`,
  `velqu_config_env_missing_file_fails_closed`
- Redaction: `redaction_unrelated_env_never_appears_in_errors`
  (a `VELQU_DATABASE_URL` secret in the environment never appears in a
  configuration rejection),
  `redaction_file_read_error_reports_path_not_contents`
- Canonicalization + example: `log_mode_canonicalized_case_insensitively`,
  `example_config_file_parses` (parses
  `examples/config/velqu.config.json` and asserts the documented
  values)
- Conformance fixture `queue_limit_returns_503_when_saturated` now
  writes the versioned schema (`configVersion: 1`) — the old
  unversioned fixture is exactly what BETA-007-A rejects, so the test
  proves the new fail-closed posture while keeping its 503 assertion.

## Examples

- `examples/config/velqu.config.json` — full versioned config file
  (parse-tested in CI).
- `docs/beta/CONFIGURATION.md` — defaults table, bounds, env table,
  precedence, fail-closed semantics, CLI/env/file/mixed invocation
  examples, rejected-startup example, explicit non-goals (secrets →
  BETA-007-C; behavioral profiles stay CLI-only → BETA-007-D).

## Gates (fresh on this branch)

- `cargo test -p velqu-runtime` -> 82 lib (19 new config tests) + 16
  fetch/source-map + 35 runtime_conformance, 0 failures; `-p q-http`
  14; `-p q-bridge` 11
- `cargo fmt --all --check` / `cargo clippy --workspace --all-targets
  -- -D warnings` / `bun run typecheck` -> clean
- `bun test` -> 434 pass / 0 fail (67 files)
- `./scripts/validate-okf` -> PASS
- `./scripts/verify` -> ALL PASS (M0–M2 + M2.2.1 + M2.3 +
  M23R2-GATE-CLOSE verified) — run inside an isolated netns; standing
  port-3000 environment note (BETA-002-C record). One manifest-refresh
  iteration was needed after verify's release rebuild re-captured the
  runtime hash; the committed manifest matches the final release
  artifact. No test weakened.

## Disclosures

- Breaking (deliberate, guardrail-driven): unversioned `--config`
  files are rejected with a typed error; migrate by adding
  `"configVersion": 1`. Documented in CONFIGURATION.md.
- Standing: CI `verify` workflows stall/fail with zero executed steps
  on PR creation (infrastructure-side, tracked since ~#714); local
  `./scripts/verify` is the real gate evidence.
