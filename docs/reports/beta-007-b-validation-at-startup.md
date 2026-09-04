# BETA-007-B — Validation at Startup

## What was built

The startup validation pass over the configuration environment,
completing BETA-007-A's typed surface with two properties:

1. **Closed `VELQU_*` environment namespace.** At startup the runtime
   enumerates the environment and rejects any `VELQU_*` name outside
   the documented allowlist (`config::KNOWN_ENV_VARS`, 19 names =
   the complete repo namespace: 7 runtime-configuration, 4 postgres
   capability, 1 build-time, 6 tooling-only). A typo'd knob
   (`VELQU_MAXQUEUE`) now fails before ready with a typed
   `startup.rejected` (`stage: "config.resolve"`) instead of being
   silently ignored. The variable's VALUE is deliberately never read
   or echoed (it may be a secret) — only the name appears in the
   error. The check is case-sensitive; non-`VELQU_` names are not the
   runtime's concern.
2. **Startup validation report.** After configuration validates, the
   `ready` line carries a `config` block: the resolved non-secret
   values plus per-field provenance (`cli` | `env` | `file` |
   `default`) via `config::startup_config_json` and the new
   `FieldSources` tracking in `config::resolve`. A mis-applied layer
   is now visible at startup. Block keys are a fixed, test-enforced
   allowlist; it never contains credentials or file contents.

`resolve` runs after the namespace check in the same fail-closed stage
— both rejections happen before the tokio runtime, engine, or listener
exist (exit 2).

Tooling-only names (`VELQU_RUNTIME`, `VELQU_PACK`, `VELQU_PG_LIVE_TEST`,
`VELQU_TEST_TRUST_KEYS`, `VELQU_ALLOC_PROFILE`, `VELQU_BENCH_DEBUG`)
are recognized but never consumed by the serving runtime — required
because the dev server and test harness legitimately set `VELQU_RUNTIME`
when spawning the runtime binary.

## Tests (5 new, in `config::tests`; 87 lib total)

- `unknown_velqu_env_name_rejected_value_never_echoed` — the canonical
  typo case; rendered rejection contains the variable NAME and never
  an `=`-value.
- `every_known_env_var_passes_the_namespace_check` — the documented
  allowlist validates as a whole.
- `namespace_check_ignores_non_velqu_names` — `PATH`/`PORT`/`HOME`
  pass; prefix is case-sensitive.
- `resolved_sources_report_the_winning_layer` — provenance tracks
  cli/env/file/default exactly across the layer stack.
- `startup_config_json_is_an_exact_field_allowlist` — 12 fixed keys,
  default provenance visible, no secret-shaped fields.

## Examples / docs

- `docs/beta/CONFIGURATION.md` gained "Closed environment namespace
  (BETA-007-B)" (allowlist table grouped runtime/postgres/build/
  tooling + rejection example) and "Startup validation report" (the
  ready-line `config` block, key allowlist, redaction statement).

## Gates (fresh on this branch)

- `cargo test -p q-engine-quickjs` -> 138 pass; `-p q-schema-runtime`
  -> 67 pass; `-p velqu-runtime` -> 87 lib (5 new) + 35
  runtime_conformance + 16 fetch/source-map, 0 failures; `-p q-http`
  14; `-p q-bridge` 11
- `cargo fmt --all --check` / `cargo clippy --workspace --all-targets
  -- -D warnings` / `bun run typecheck` -> clean
- `bun test` -> 434 pass / 0 fail (67 files)
- `./scripts/validate-okf` -> PASS
- `./scripts/verify` -> ALL PASS (M0–M2 + M2.2.1 + M2.3 +
  M23R2-GATE-CLOSE verified) — isolated netns; standing port-3000
  note (BETA-002-C). One manifest-refresh iteration after verify's
  release rebuild; committed manifest matches the final artifact. No
  test weakened.

## Disclosures

- Breaking (deliberate): unknown `VELQU_*` environment variables now
  reject startup. The allowlist is the documented closed namespace;
  new capability env vars must be added to `KNOWN_ENV_VARS` and
  CONFIGURATION.md together.
- Standing: CI `verify` workflows stall with zero executed steps on PR
  creation (infrastructure-side, tracked since ~#714); local verify is
  the real gate evidence.
