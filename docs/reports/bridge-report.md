---
type: Evidence Report
title: Bridge Strategy Report (JSON A/B)
status: complete
milestone: M1
---

# Bridge strategy report — engine JSON (A) vs native JSON (B)

Raw data: `benchmarks/raw/bridge/bridge.jsonl` (per-sample) +
`benchmarks/raw/bridge/bridge-summary.json`. Command:
`./target/release/q-bridge-bench --out-dir benchmarks/raw/bridge --iters 2000`
(warmup 200; correctness asserted per case — all 16 cases OK).

Environment: same host as cold-start (i5-13420H, Linux, debug-noise-free
release build, single engine worker, in-process — no network).

## Results (μs, lower is better)

| Case | A (engine JSON) p50 | B (native JSON) p50 | B vs A |
|---|---:|---:|---:|
| input small object | 29.0 | 19.2 | **−34%** |
| input nested object | 30.6 | 27.3 | −11% |
| input array of 100 records | 313.3 | 181.9 | **−42%** |
| output int | 22.0 | — | baseline |
| output string | 18.4 | — | text path |
| output small object | 22.6 (A) | 22.1 (B) | ≈equal |
| output nested object | — | 23.4 | — |
| output array of 100 | 198.6 (A) | 153.4 (B) | **−23%** |
| typed problem | 19.0 | — | value path |
| pre-serialized bytes | 22.3 | — | passthrough |
| promise completion (1ms timer) | 2311.8 | — | timer-bound |
| 5 scalar accesses | 26.6 | — | lazy getters |

## Decision (ID-006)

**Native strategy B is adopted as the compiler default for JSON body inputs**:
it is faster on every input shape measured (11–42%), because serde_json parse +
one-pass object construction beats shipping raw bytes into the engine and
JSON.parse-ing them there. For RESPONSES the strategies are within noise on
small objects (22.1 vs 22.6μs) and B wins on large arrays (−23%), so the
default response strategy is also native, with engine stringify (A) retained
per-route as an escape hatch.

This is a measured reversal of the design-session worry ("native JSON is not
automatically faster once conversion is counted") for the INPUT direction on
this host/engine pair: rquickjs object construction is cheap enough that
parsing in Rust wins. Scope-limited claim: exact shapes, quickjs-ng 0.15.1,
rquickjs 0.12.2, this CPU.

## Bridge gates (performance-budgets.md)

| Gate | Result |
|---|---|
| lazy unread request fields materialize 0 fields/bytes | PASS (q-bridge `unread_request_costs_nothing`: host_calls=0, bytes=0; engine `lazy_ctx_touches_nothing`) |
| expired/wrong-owner handles accepted | 0 (tests) PASS |
| native JSON adopted only if end-to-end faster | PASS — measured faster (above) |
| response strategy parity | PASS — both strategies produce identical bodies (correctness asserted) |

## Notes

- Promise completion cost is dominated by the actual 1ms timer; the bridge
  overhead portion is ~20–30μs.
- The per-case `us` value is a full invoke→outcome round trip through the
  worker channel (includes scheduling); pure FFI call cost is below
  measurement noise for scalar access.
- Strategy is recorded per-route in the pack (`validationStrategy`,
  `responses.*.strategy`) so build reports show exactly what runs (SCHEMA-005).
