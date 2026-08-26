# M27-002-D — Capability Pruning: Size and Cold-Start Deltas

Packet: M27-002-D (remove unused modules). Measured on the proof
app; both sides are reproducible commits with the same release
runtime binary.

## What changed

Before this packet the compiler had two defects that made pruning
vacuous:

1. Route capability detection only recognized `ctx.native.*`
   property access — the destructured handler style
   (`async ({ native }) => ... native.timer.delay(...)`) produced
   **no grant**, so nothing was ever declared.
2. The pack emitted a constant empty inventory regardless of what
   routes used.

Now: detection understands `ctx.native`, destructured `native`, and
aliased destructuring (`{ native: n }`); unknown grants fail the
build (fail closed); the linked inventory is the pruned resolver
output — only capabilities some route actually declares enter the
artifact.

## Matched evidence

- **before** = commit `a160e35` (M27-002-C head), same app source,
  fresh build in a detached worktree.
- **after** = commit `074bebe` (this packet), fresh build.
- Same `target/release/velqu-runtime` binary for both cold-start
  runs (runtime code unchanged by this packet); 10 fresh-process
  samples per side, ready-line `startupMs`, nearest-rank percentiles.

| metric                      | before | after | delta |
| --------------------------- | ------ | ----- | ----- |
| app.qpack bytes             | 24,534 | 24,590 | +56 |
| inventory entries           | 0 (`[]`) | 1 (`runtime:timers@1`) | +1 |
| declared grants             | `[]` (bug: missed) | `["timer"]` | +1 |
| cold-start p50 ms (n=10)    | 6.653 | 3.828 | noise |
| cold-start p95 ms (n=10)    | 7.787 | 9.566 | noise |

Raw startupMs:

- before: 3.260845 3.282337 3.75581 5.8504689999999995 6.6527840000000005
  6.900331 7.264054 7.394245 7.693651 7.787317
- after: 2.823026 3.180028 3.343183 3.550657 3.827505 4.623282000000001
  6.954753999999999 7.296261 8.703401 9.566455

## Honest reading

- **Size**: the linked `runtime:timers@1` entry costs exactly **+56
  bytes** of pack (inventory entries + hash binding). An application
  that declares no capabilities keeps an empty inventory: its
  canonical form is the 4-byte count prefix, hash-bound, which the
  before-side already carried — so the zero-link cost is
  structurally unchanged by this packet (pinned by
  `resolveLinkedModules([])` → `[]` and C's empty-vector test).
- **Cold start**: p50 differs (6.65 → 3.83 ms) but the distributions
  overlap heavily (min/max 3.26–7.79 vs 2.82–9.57, p95 crosses);
  these are same-host laptop numbers at n=10 with no isolation.
  The honest statement is **statistically indistinguishable at this
  sample size** — no startup claim is made for or against the
  inventory section. Any real capacity claim would need the matched
  harness protocol (≥100 fresh processes per cell) and belongs to a
  benchmark packet, not this one.
- Before-side `declared: []` was not "zero usage", it was
  **undetected usage**: the proof app's timer routes were silently
  ungranted pre-fix. That is the actual defect this packet closes;
  the security posture improves because grants can no longer be
  silently dropped, and unknown grant names now fail the build.
