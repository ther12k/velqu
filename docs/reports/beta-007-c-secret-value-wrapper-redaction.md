# BETA-007-C — Secret Value Wrapper/Redaction

## What was built

`config::SecretString` — a typed wrapper for secret configuration
values, wired at the first touchpoint where the production runtime
handles a credential: the `VELQU_DATABASE_URL` environment boundary.

- **`Debug`/`Display` always render `[redacted]`** — an accidental
  log, error, or inspect of a holder (including containers that
  Debug-print their contents) cannot disclose the value.
- **`expose()` is the only read path** — explicit and
  grep-auditable; the database URL flows from the wrapper straight
  into the pool constructor and nowhere else.
- **`from_env(var, lookup)`** wraps at the env boundary through an
  injected lookup (testable without touching the process
  environment); the raw value never exists outside the wrapper.
- **No `Clone`, no `PartialEq`** — secret material is neither
  duplicated nor compared.
- **`run()` wiring** (`crates/q-runtime/src/lib.rs`): the postgres
  URL is read via `SecretString::from_env` and exposed only to
  `pool_from_url_and_env`. Behavior is unchanged (same fail-closed
  missing-var rejection, same pool-limit validation); the value is
  now typed as secret for its entire life inside the runtime.
- Memory zeroization on drop is deliberately not claimed — the
  guarantee is redaction across formatting/serialization paths, and
  the docs say so honestly.

## Tests (3 new, in `config::tests`; 90 lib total)

- `secret_debug_and_display_render_redacted` — `{}`/`{:?}` render
  `[redacted]`; a `Vec<&SecretString>` holder's Debug contains no
  secret material.
- `secret_expose_is_the_only_read_path` — `expose()` returns the
  wrapped value.
- `secret_from_env_wraps_without_disclosure` — present/absent env
  cases; Debug of the wrapper carries no material.

Redaction posture already enforced elsewhere still holds: config
errors never echo unrelated env values (BETA-007-A), the ready-line
config block is a fixed non-secret allowlist (BETA-007-B), completion
logs are field-allowlisted (BETA-006-F), and pool-side error
rendering redacts URL fragments (BETA-004).

## Examples / docs

- `docs/beta/CONFIGURATION.md` gained "Secret value wrapper
  (BETA-007-C)": redaction semantics, `expose()` audit story, no-
  clone/no-equality, wrapper→pool flow, honest non-goal on memory
  zeroization. The "What is NOT configurable here" section now points
  at the wrapper instead of a future packet.

## Gates (fresh on this branch)

- `cargo test -p velqu-runtime` -> 90 lib (3 new) + 35
  runtime_conformance + 16 fetch/source-map, 0 failures
- `cargo fmt --all --check` / clippy (`-D warnings`) clean
- `bun test` -> 434 pass / 0 fail (67 files); `bun run typecheck` clean
- `./scripts/validate-okf` -> PASS
- `./scripts/verify` -> ALL PASS (M0–M2 + M2.2.1 + M2.3 +
  M23R2-GATE-CLOSE verified) — isolated netns; standing port-3000
  note (BETA-002-C). One manifest-refresh iteration after verify's
  release rebuild; committed manifest matches the final artifact. No
  test weakened.

## Disclosures

- Behavior-preserving refactor of the postgres URL boundary; the only
  observable differences are the typed wrapper and its redaction
  guarantees.
- Flake observed once: in the first battery run, verify's filtered
  parallel `bun test` subset failed the M4A-009-D
  metrics-readiness-shutdown scenario at a ready-line assertion. The
  same test passes in isolation, in the full `bun test` (434/434,
  including this battery's re-run), and in verify's re-run — one-off
  parallel-spawn flake, not a regression from this packet. Recorded
  here rather than hidden.
- Standing: CI `verify` workflows stall with zero executed steps on PR
  creation (infrastructure-side, tracked since ~#714); local verify is
  the real gate evidence.
