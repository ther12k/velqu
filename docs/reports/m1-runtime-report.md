---
type: Evidence Report
title: M1 Runtime Feasibility Report
status: complete
milestone: M1
---

# M1 — Rust/QuickJS runtime feasibility (2026-08-17)

## Architecture verdict: PASS

All M1 gate items pass; one performance BUDGET (route-count scaling) is
exceeded — recorded honestly below and in the budgets section; it is not an
M1 gate item (M1 gate = runtime/bridge feasibility, which passed).

## Gate items

| M1 gate requirement | Evidence | Status |
|---|---|---|
| actual Rust binary serves one QuickJS worker | `target/release/velqu-runtime`; startup log `handlers:10, engine: quickjs-ng/0.15.1`; binary conformance tests | PASS |
| handler references cached | engine load → `Persistent<Function>` cache; `load_verifies_handler_table_and_caches`; startup log eval 0.10ms | PASS |
| native route before JS demonstrated | `x-velqu-stage: native` on C0; 404/405 without engine; logs show `stage:native` for C0 | PASS |
| text, small JSON, params, JSON input, async, cancel, throw paths pass | `crates/velqu-runtime/tests/runtime_conformance.rs` — 8 tests; engine tests — 12 | PASS |
| opaque request handle expiry/ownership tests | q-bridge 4 tests (expiry, wrong-owner, reuse isolation, unread=0) + engine expired-handle test | PASS |
| body/header/queue/heap/stack/time limits | 413/431/503 responses tested; heap 32MiB + stack 512KiB set on the engine; deadline interrupt kills runaway loops (test) | PASS |
| application pack tamper/version mismatch fails before ready | `tampered_pack_fails_before_before_ready` (exit≠0, integrity error); engine mismatch unit tests | PASS |
| bridge A/B strategy report exists | `docs/reports/bridge-report.md` + raw JSONL | PASS |
| process cold start + idle RSS raw data | `benchmarks/raw/cold-start/*` (1680 samples, 0 failures), RSS p50 6.2 MiB | PASS |
| source maps usable | `source_mapped_exception_identifies_original_location` (TS file+line mapped into diagnostics) | PASS |
| go/conditional-go/stop decision | **PASS** (below) | PASS |

## Runtime architecture as built

```
velqu-runtime (bin)
 ├─ q-pack        load+verify (versions, sha256 of bundle + canonical routes)
 ├─ q-router      zero-parse route table (pack pathSegments), 404/405+Allow, HEAD→GET
 ├─ q-http        hyper 1.11 + TokioTimer; header/body/URI limits; queue semaphore (503)
 ├─ q-bridge      request store: (slot,generation) handles; settle invalidates; counters
 ├─ q-engine      engine trait (boundary seam, ADR-0006)
 └─ q-engine-quickjs  ONE worker thread owning Runtime+Context
      ├─ prelude: __velquRegister / lazy ctx getters / __velquRun / watch table
      ├─ natives: __velquReqRaw(JSON str), body text/len/fill, timer callbacks
      ├─ timer capability: JS-side op table + native op registry (bounded 1024)
      ├─ deadline interrupt (shared Instant cell) + promise-watch settlement
      └─ exception → message+stack → sourcemap → redacted 500
```

Startup stages are timed and logged (`pack.load, router.build, engine.spawn,
bundle.load, listen`) — zero route/schema/OpenAPI/TS compilation at startup
(G-004): the router consumes pre-compiled `pathSegments`, validation uses
pre-compiled schema IR, the bundle is a plain JS string.

## Cancellation and async safety (RUN-006)

Worker message loop handles Invoke/Cancel/TimerFired/Shutdown plus deadline
timeouts. Cancellation settles the request slot (generation bump), rejects the
invocation's pending ops, and drops the reply. Late completions are dropped
and counted (`late_completions_dropped`). Tests cover: timer resolves; cancel
before completion; completion before cancel; late completion after settlement
(dropped); timeout interrupt; handler-catches-abort (via op rejection);
shutdown with pending work; bounded registry (op cap).

## Runtime invariants (measured, not assumed)

- route compilation at runtime: 0 (segments pre-compiled in pack)
- schema compilation at runtime: 0 (IR validated directly)
- initial QuickJS workers: 1 (single OS thread)
- service initialization for unrelated cold route: 0 (users service lazy;
  C0/C1/C2 cold starts identical to within noise)
- compiler bytes in runtime: 0 (runtime consumes artifacts only)

## Known limitations (honest)

1. 1,000-route startup scaling budget EXCEEDED: p50 3.08ms (25 routes) →
   15.7ms (1000 routes), +409% vs the ≤20% budget. Cause profile: pack JSON
   parse (871KB) + bundle eval (1000 registrations) + route table load.
   Absolute number remains ~10× faster than the matched Elysia candidate
   (161ms). Remediation candidates for M2+: pack chunking/binary format,
   bytecode (ADR-0014), incremental handler resolution. NOT fixed silently.
2. QuickJS bytecode load spike: UNEXECUTED in M1 (optional per gate); pack
   carries source; bytecode remains a measured hypothesis.
3. `unsafe` usage: exactly one reviewed block (`__velquFillBytes` copy into a
   caller-owned Uint8Array) — see FFI ownership review.
4. Same-process QuickJS runs TRUSTED code only (SEC-002) — resource limits
   are robustness controls, not a sandbox.

## Commands

```bash
cargo test --workspace              # 45 tests green
cargo build --release -p velqu-runtime  # 4.6MB stripped binary
./target/release/velqu-runtime --pack examples/proof/dist/app.qpack --port 3000
bun benchmarks/harness/check-server.ts 3000 --candidate velqu   # 31/31
```
