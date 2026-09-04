# BETA-008-D — Graceful Drain and Termination

## Behavior

The runtime already has the complete bounded shutdown implementation, now
packaged as the BETA deployment evidence:

1. SIGTERM/SIGINT flips the lock-free `DrainGate` immediately and emits
   `drain.begin` with the live invocation count.
2. The accept loop stops admitting new connections. Established keep-alive
   connections are covered by Hyper's graceful watcher; native liveness can
   still answer while dynamic admissions check the drain gate.
3. Dynamic work arriving after the flip is refused with the frozen 503 overload
   problem and `Retry-After: 1`; in-flight work is allowed to complete.
4. The 5-second `SHUTDOWN_BUDGET_MS` bounds the wait. Stragglers are aborted
   through Tokio task ownership, which drops the cancellation guard and routes
   cancellation through the owning engine exactly once.
5. The runtime drains the shared outbound pool, tears down the engine, and
   emits `shutdown.complete` with drain refused/completed/aborted counts,
   invocation pending/registered/settled counts, load-shed counters, fetch-pool
   state, and engine/stage metrics. It then exits 0 deterministically.

## Evidence

- `drain_lets_in_flight_request_complete` — an 800 ms request receives its
  complete response after SIGTERM; `drain.begin` precedes
  `shutdown.complete`; one invocation settles exactly once.
- `drain_waits_bounded_then_detaches_straggler_connection` — a 20 s request
  is force-aborted at the 5 s budget, reports `aborted: 1`, cancellation and
  settlement exactly once, pending 0, and exits 0 within the bounded window.
- `graceful_drain_flips_gate_and_reports_before_exit` — drain gate visibility,
  event ordering, refusal count, and complete report.
- `scripts/proxy-smoke.sh` — release-runtime container-friendly smoke sends
  SIGTERM and requires prompt deterministic exit after health/route checks.

## Runbook update

`docs/beta/DEPLOYMENT-REVERSE-PROXY.md` now specifies readiness withdrawal
before SIGTERM, immediate gate/refusal behavior, in-flight completion, the
5-second force-abort boundary, and the no-pending-invocation exit contract.

## Gates

- `cargo test -p velqu-runtime` — pass (102 lib + 37 runtime conformance)
- `cargo fmt --all --check` / clippy `-D warnings` — pass
- `./scripts/validate-okf` — pass
- `./scripts/verify` — ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE)
- `bun test` — 434 pass / 0 fail (67 files)
- `bun run typecheck` — pass
- `scripts/proxy-smoke.sh` — `PROXY-SMOKE-OK`

## Disclosures

- The drain budget is 5 seconds by the frozen capability constant; operators
  must coordinate proxy admission and idempotency/retry policy at the edge.
- Forced abort is reported honestly and still exits 0; no durable job queue or
  post-process completion guarantee is claimed.
- Container image/example packaging remains BETA-008-E.
