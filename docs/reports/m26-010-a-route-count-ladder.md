# M26-010-A — Route-count cold-start ladder (25 / 100 / 1,000 / 5,000 / 10,000)

Command (canonical evidence run):

```bash
ROUTE_COUNT_RUN_ID=m26-010-a ROUTE_COUNT_SEED=20260826 \
  bun benchmarks/harness/route-count.ts --samples=40
```

4 candidates × 5 sizes × 40 fresh processes per cell = 800 spawns,
seeded randomized candidate/size order, **zero failures**. Raw JSONL:
`benchmarks/raw/route-count/route-count-1787681619011.jsonl` (every
sample retained); summary:
`benchmarks/raw/route-count/summary.json`. The generated
`docs/reports/cold-start-report.md` route-count section is derived
from this summary by `scripts/generate-benchmark-reports.py`
(data-driven since this packet — no hand-edited numbers).

## Fixture refresh (disclosed)

The scale packs (`benchmarks/raw/packs/app-N.qpack`) predated the
M26-002-A runtime fingerprint and would now FAIL `verify()` (missing
`rquickjs`/`buildHash` engine fields); they had only been hash-checked
since, never re-verified. `build-proof-pack.ts` now emits the full
fingerprint (mirroring the compiler's `runtimeBuildHash`), and ALL
five pack pairs (source + bytecode-embedded) were regenerated and
re-verified (`velqu-runtime --fingerprint` → compatible). New pack
hashes are tracked in `benchmarks/manifest.json`.

## Results (p50 cold start, ms; VmRSS p50 after ready)

| routes | velqu source | velqu bytecode | raw-bun | elysia2 |
|---:|---:|---:|---:|---:|
| 25 | 6.116 | 5.710 | 7.035 | 109.113 |
| 100 | 11.032 | 11.820 | 7.149 | 111.974 |
| 1,000 | 82.389 | 83.529 | 7.594 | 127.289 |
| 5,000 | 433.289 | 428.076 | 6.643 | 184.396 |
| 10,000 | 947.671 | 926.186 | 8.028 | 249.895 |

RSS at 10,000 routes: velqu ≈ 305 MB, raw-bun ≈ 20 MB, elysia2 ≈ 97 MB.

## Honest reading

- **Small-app budget is preserved, not sacrificed**: at 25 routes
  velqu cold-starts in ~6 ms — the fastest candidate on this host.
- **Scaling is super-linear and dominated by pack.load**: 400× the
  routes costs ~155× the startup. The committed 10,000-route startup
  trace (`benchmarks/raw/profiles/startup-10000.json`) attributes
  ~342 ms of ~434 ms to `pack.load` — JSON text parsing of the v1
  pack, not routing (serialized router load: 6.8 ms) or JS
  (bundle.load: 85 ms). Zero startup compilation is performed (G-004);
  the cost is parsing an already-verified artifact format.
- **The authorized lever is QPack v2 binary sections** (this
  milestone): dense tables + raw bytecode sections exist behind the
  reader (M26-003..M26-005); the native v2 production load path is
  the remaining step. No claim is made here that v2 fixes the slope —
  that requires its own matched evidence.
- raw-bun's flat profile reflects a trivial router over N in-process
  handlers with no per-route contract; it is a floor reference, not a
  like-for-like comparison. Same-host, release builds, loopback
  HTTP/1.1; not universal claims.

## Guardrails

- No runtime router/schema compilation: startup stages remain
  pack.load → router.build (deserialize only) → engine.spawn →
  bundle.load (bytecode or source) → listen.
- No base64 decoding at startup on the bytecode path (single-decode
  cache, M26-004-B).
- 25-route budget documented above (not silently sacrificed).
- 10,000-route scaling documented honestly (super-linear; cause
  attributed from the committed stage trace).

Remaining M26-010 obligations: ≥100 fresh processes per cell (B),
candidate-order randomization is already seeded+recorded (C will pin
its disclosure), p50/p95/p99 + RSS + stage timings + hashes as the
canonical closing record (D).

## M26-010-B: ≥100 fresh processes per cell (release evidence)

Re-run at `--samples=100` (`ROUTE_COUNT_RUN_ID=m26-010-b`): 4 × 5 ×
100 = **2,000 fresh processes, zero failures**, every sample retained
(`route-count-1787682637700.jsonl`). p50s are stable against the
40-process A run (e.g. velqu source 10k: 949.101 vs 947.671 ms;
25-route: 6.073 vs 6.116 ms) — cross-run consistency within noise on
this host. The generated `docs/reports/cold-start-report.md` and the
summary now carry this canonical run.
