# BETA-007-D — Profile-Specific Settings

## What was built

Named profile blocks in the versioned config file, selected at
startup, overlaying the file's base fields as an explicit layer in the
stack: `CLI > env > active profile > file > default`.

- **Schema** (`configVersion: 1` extended, back/forward-compatible):
  optional `profiles` map (name → block) and optional
  `activeProfile`. A block accepts the same optional fields as the
  file (`host`, `port`, `maxBodyBytes`, `maxQueue`, `log`,
  `logSample`); `deny_unknown_fields` applies inside blocks, so
  nesting profiles inside a profile is structurally rejected.
- **Selection**: `VELQU_PROFILE` (new env, wins) else the file's
  `activeProfile`. No selection → no profile layer; plain file
  behavior unchanged.
- **Fail-closed rules**: selecting an undeclared profile rejects
  startup with a typed error naming the declared set (or "declares no
  profiles"); profile names are a closed shape (1..=32 of `a-z`,
  `0-9`, `-`) and every declared name is validated even when
  inactive; block values pass the same bounds/closed sets as
  everywhere else (out-of-range rejects, never clamps).
- **Provenance**: new `FieldSource::Profile`; the ready-line `config`
  block gains `activeProfile` and `profile` sources on the fields the
  profile set — the applied profile is visible at startup.
- **Namespace**: `VELQU_PROFILE` added to `KNOWN_ENV_VARS`
  (runtime-configuration group) and documented.

## Tests (6 new, in `config::tests`; 96 lib total)

- `profile_overrides_file_but_not_env` — profile beats base file
  fields, env beats profile; provenance tracked per field.
- `velqu_profile_env_selects_and_beats_file_selection`
- `unknown_active_profile_fails_closed` — undeclared selection from
  file and from env; active profile with no profiles map.
- `profile_names_are_validated` — bad declared name; bad env-selected
  name.
- `profile_blocks_reject_unknown_fields_and_nesting`
- `active_profile_reported_in_startup_config` — `activeProfile` key
  null when no profile; `profile` provenance visible.

The fixed key allowlist test now includes `activeProfile`.

## Examples / docs

- `docs/beta/CONFIGURATION.md`: layer stack updated, `VELQU_PROFILE`
  env row, namespace table row, and a new "Profile-specific settings
  (BETA-007-D)" section with a production/development example and the
  fail-closed rules.

## Gates (fresh on this branch)

- `cargo test -p q-engine-quickjs` 138 / `-p q-schema-runtime` 67 /
  `-p velqu-runtime` 96 lib (6 new) + 35 runtime_conformance + 16
  fetch/source-map / `-p q-http` 14 / `-p q-bridge` 11 — 0 failures
- fmt / clippy (`-D warnings`) / typecheck -> clean
- `bun test` -> 434 pass / 0 fail (67 files)
- `./scripts/validate-okf` -> PASS
- `./scripts/verify` -> ALL PASS (M0–M2 + M2.2.1 + M2.3 +
  M23R2-GATE-CLOSE verified) — isolated netns; standing port-3000
  note (BETA-002-C). One manifest-refresh iteration after verify's
  release rebuild; committed manifest matches the final artifact. No
  test weakened.

## Disclosures

- Config-file schema extension is additive: existing version-1 files
  without profiles behave exactly as before.
- Standing: CI `verify` workflows stall with zero executed steps on PR
  creation (infrastructure-side, tracked since ~#714); local verify is
  the real gate evidence.
