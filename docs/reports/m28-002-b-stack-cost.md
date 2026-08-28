# M28-002-B — Dependency / Binary / Startup Cost of the Selected Fetch Stack

Cost measurement for the stack selected in M28-002-A (hyper 1 +
hyper-util client-legacy + hyper-rustls webpki-roots), linked into the
production `velqu-runtime` binary. Companion to
`docs/reports/m28-002-a-stack-comparison.md`.

## Method

- **before**: release binary at the base commit (no outbound stack),
  remap-flag reproducible build.
- **after**: same tree + the selected dependencies linked through a new
  dormant-but-reachable `fetch_stack` module (`--fetch-stack-info`
  constructs the client once; nothing dials; M28-003 wires the bounded
  pool).
- Identical proof pack, identical host, n=10 fresh-process spawns per
  side, ready-line `startupMs`, nearest-rank percentiles. Raw samples
  retained in `benchmarks/raw/stack-spike/fetch-stack-cost.json`.

## Results

| Metric | before | after | Delta |
| --- | --- | --- | --- |
| Binary size | 5,552,936 B | 6,468,376 B | **+915,440 B (+16.5%)** |
| Cold-start p50 | 4.061 ms | 4.512 ms | +0.451 ms |
| Cold-start p95 | 4.704 ms | 5.214 ms | +0.509 ms |
| Cold-start p99 | 4.732 ms | 5.223 ms | +0.491 ms |

Startup delta is ~0.45 ms: the dormant module constructs nothing at
boot — the delta is the one-time cost of a slightly larger binary (page
in) plus noise at n=10. Both sides stay far under the 10 ms cold-start
budget (M27-011 baseline context: p50 ≈ 4.16 ms).

## Dependency-graph cost

- hyper 1 and hyper-util 0.1 were already production dependencies for
  ingress; the outbound use **extends their feature sets in place**
  (`client`, `client-legacy`) — no duplicate copies.
- New crates: hyper-rustls 0.27, http-body-util 0.1, plus their transitive
  TLS stack (rustls, ring, webpki-roots) — the same TLS core the M28-002-A
  spike measured in isolation.
- Crate-count delta visible in `Cargo.lock` diff; the standalone spike
  (43 crates) predicted the direction; the linked-in cost above is the
  production-accurate number.

## Budget verdicts

| Guardrail | Budget | Measured | Verdict |
| --- | --- | --- | --- |
| Cold-start p50 | < 10 ms | 4.512 ms | PASS |
| Cold-start p95 | < 10 ms | 5.214 ms | PASS |
| Binary growth (capability add) | ≤ +1 MiB class (M27-011 added +120 KB for 4 capabilities; fetch is a full HTTP/TLS stack) | +915,440 B | PASS — inside the +1 MiB envelope, consistent with the standalone spike ratio |

The cost is linked now but **dormant**: no dialing exists until M28-003,
and every future dial is gated by the ADR-0033 policy object. If the
+0.9 MiB were ever judged unacceptable, the documented fallback (M28-002-A)
would be re-evaluated — but the budget passes, so no split/defer decision
is triggered.

## Reproducing

```bash
export RUSTFLAGS="--remap-path-prefix=$(pwd)=/velqu-src"
export CFLAGS="-ffile-prefix-map=$(pwd)=/velqu-src -fdebug-prefix-map=$(pwd)=/velqu-src"
cargo build --release -p velqu-runtime
./target/release/velqu-runtime --fetch-stack-info   # constructs the stack, prints identity
# before/after numbers: git stash the stack commit and re-measure per the protocol above
```
