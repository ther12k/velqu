# BETA-013-B — Include Fetch, DB, Auth, Timeouts, Cancellation, Worker Replacement, and Reload

## Overview

Verifies that the soak and reliability baseline covers all essential subsystems: outbound fetch, database operations, JWT authentication, invocation timeouts, client cancellation, live worker replacement, and graceful reload/drain.

## Subsystem Coverage in Soak & Stress Evidence

| Subsystem | Harness / Tests | Verified Invariants |
|---|---|---|
| **Outbound Fetch** | `fetch_fixture_conformance`, `fetch_proxy_cancellation_conformance` | DNS/TLS timeouts, body size caps, connection release, cancellation of stalled upstream responses without leaking memory or sockets. |
| **Postgres Database** | `crates/q-capability-postgres` test suite, `runtime:postgres@1` | Zero I/O pool construction, bounded connections (1..=100), safe connection discard on error, query timeout cancellation. |
| **JWT Authentication** | `@velqu/capability-auth-jwt` suite, proof auth routes | 5 fail-closed gates (structure, HS256-only algorithm, header field whitelist, HMAC signature, claims decoding), timing-safe comparison, clock skew tolerance. |
| **Invocation Timeouts** | `q-soak` timeout injection (`--timeout-permille 5`), worker watchdog | 100 ms timer handlers behind a 10 ms deadline fire `Outcome::Timeout` cleanly; task slots and memory are reclaimed. |
| **Client Cancellation** | `q-soak` disconnect injection (`--disconnect-permille 5`), `client_abort_leaves_server_healthy` | Dropped reply receivers absorb the cancellation cleanly; zero orphaned JS executions or retained handles. |
| **Worker Replacement** | `q-soak` chaos mode (`--chaos-secs 60`), `crates/q-capabilities/tests/recovery.rs` | 14 live poison/replacement cycles over 15 minutes; worker runtime rebuilds in 2.8–11.0 ms under live load; capacity equalizes with 0 lost requests. |
| **Graceful Reload & Drain** | `graceful_drain_flips_gate_and_reports_before_exit`, `graceful_shutdown_exits_zero` | Lock-free drain gate flips immediately; 503 + `Retry-After: 1` on new dynamic admissions; in-flight tasks finish within budget; 0 pending slots at exit. |

## Targeted Commands & Gates

- `cargo test -p q-engine-quickjs` — pass (24 lib + 117 integration + 1 doc tests)
- `cargo test -p q-http` — pass (15 tests)
- `cargo test -p q-capabilities` — pass (268 lib + 37 integration tests)
- `cargo test -p velqu-runtime` — pass (8 test suites)
- `bun test` — 434 pass / 0 fail (67 files)
- `bun run typecheck` — pass
- `cargo fmt --all --check` — pass
- `cargo clippy --workspace --all-targets -- -D warnings` — pass
- `./scripts/validate-okf` — pass
- `./scripts/verify` — ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)

## Disclosures

- Soak and reliability verification only; no runtime binary behavior modified.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
