# BETA-009-A — Fuzz Suites for Pack, Router, Schema, Bridge, and HTTP

## Scope

This packet runs the existing deterministic property/fuzz and minimized-corpus
suites across the beta trust boundaries. No parser or runtime behavior was
changed.

## Suites and results

- `cargo test -p q-pack`: pack random-byte and single-byte mutation corpus;
  all tests pass. Random malformed packs fail closed without panics; integrity
  mutations are rejected.
- `cargo test -p q-router`: route/path grammar, precedence, method, and
  malformed input corpus; all tests pass.
- `cargo test -p q-schema-runtime`: validator fuzz and codec minimized
  corpus; all tests pass. Arbitrary JSON classification is deterministic and
  total for every IR source/kind.
- `cargo test -p q-bridge`: stale/foreign handle fuzz corpus and bounded slab
  lifecycle property tests; all tests pass. No stale access or unbounded slot
  growth.
- `cargo test -p q-http`: query/percent decoder fuzz, bounded header/body
  corpus, and minimized ingress regression corpus; all tests pass.
- `cargo test -p q-capabilities`: fetch policy, forwarding distrust, SSRF,
  timeout, fairness, and capability lifecycle suites; all tests pass.
- `cargo test -p velqu-runtime`: full runtime/conformance suite; all tests
  pass after rebuilding the required bytecode helper and proof artifact.
- `bun test`: 434 pass / 0 fail across 67 files.
- `bun run typecheck`: pass.

The committed historical security report records the deterministic iteration
counts: q-pack 448 random + 256 mutations; HTTP 40,000; schema validator
40,000. `cargo-fuzz`/ASan/TSan remain a disclosed GA-track hardening item,
not claimed by this beta packet.

## Required companion evidence

- Security boundary report: `docs/reports/security-review.md`.
- Dependency maintenance/security report: `docs/reports/m28-002-d-maintenance-security.md`.
- Chaos report: `docs/reports/m3-010-b-chaos.md`.
- Known limitations: `docs/beta/LIMITS-AND-NON-GOALS.md` and
  `docs/beta/governance/RISK_REGISTER.md`.

## Gates

- All targeted Rust package suites listed above — pass.
- `cargo fmt --all --check` — pass.
- `cargo clippy --workspace --all-targets -- -D warnings` — pass.
- `./scripts/validate-okf` — pass.
- `./scripts/verify` — ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE).

## Findings and triage

No new crash, panic, stale-handle, integrity, parser-totality, or unbounded
allocation finding was produced. The first attempts that ran without build
artifacts failed only because the runtime bytecode helper/proof `dist` was
absent; rebuilding `q-bytecode-tool` and `examples/proof` resolved that
harness precondition without changing assertions.
