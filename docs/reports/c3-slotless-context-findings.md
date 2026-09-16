# C3 slotless prevalidated context findings

**Status:** diagnostic measurement only; no acceptance threshold or canonical
benchmark claim. Two matched runs are retained: a short blocked probe and a
longer interleaved paired run. The interleaved run is the evidence of record
for magnitude; the stage accumulators are the evidence of record for
mechanism.

## Scope

This packet compares the pre-C3 request-store lifecycle with the C3 slotless
prevalidated-input path on the same proof QPack and the same
`GET /hello/Rafi` dynamic JavaScript handler. The handler remains
JavaScript-bound and the path parameter is still natively validated before
invocation.

Invariants preserved (each covered by a test):

- own-property presence, not value, determines whether a field is
  prevalidated; `Some(null)` is an own property holding JS `null`
  (`slotless_validated_null_is_own_property_null`);
- an absent prevalidated field has no own property and no lazy fallback on a
  slotless invocation;
- a declared field without a prevalidated value retains a request slot;
- policy and `full-request` declarations always retain a slot
  (`full_request_stays_request_backed_with_prevalidated_params`);
- JavaScript body-validation fallback remains request-backed;
- no native/static response fold is involved (`json_to_js` maps JSON null to
  JS null, never undefined).

## Protocol (interleaved run of record)

- host: local Linux host (noisy; see variance note), release binaries, one
  runtime worker; the 72h soak r3 was running throughout, as it was for both
  candidates equally;
- engine: quickjs-ng 0.15.1 through rquickjs 0.12.2;
- pack: `examples/proof` built from the pinned b8fee349 toolchain
  (TypeScript 5.9.3, Bun 1.4.0), sha256
  `2e60319fb3164d960bb5a78d821dca38f7475b3a7c08dd8b69843c1681c70cc3`;
- route: `GET /hello/Rafi`, status 200, dynamic JS handler;
- warmup: 300 requests per cell;
- cells: 10 s, 5 repetitions, concurrency 1/10/50;
- pairing: INTERLEAVED — within each repetition the old/new candidates
  alternate per cell, so time drift applies to both equally;
- errors: zero across all 30 cells.

Raw JSONL: `benchmarks/raw/c3-probe/c3-matched-20260916-final.jsonl`
Summary: `benchmarks/raw/c3-probe/c3-matched-20260916-final.summary.json`

## End-to-end results (interleaved, medians)

| candidate | c | median req/s | median p50 us | median p95 us | paired new/old per rep | paired median |
|---|---:|---:|---:|---:|---|---:|
| pre-C3 request-store | 1 | 4,437 | 147.6 | 630.6 | — | — |
| C3 slotless | 1 | 4,247 | 153.9 | 664.0 | 1.119 / 0.957 / 1.253 / 1.071 / 0.841 | ×1.071 |
| pre-C3 request-store | 10 | 15,414 | 472.9 | 1,624.2 | — | — |
| C3 slotless | 10 | 14,109 | 529.2 | 1,809.4 | 1.305 / 1.473 / 0.915 / 1.117 / 1.056 | ×1.117 |
| pre-C3 request-store | 50 | 20,520 | 2,207.7 | 4,691.3 | — | — |
| C3 slotless | 50 | 21,896 | 1,916.2 | 4,533.4 | 1.578 / 1.163 / 1.115 / 1.049 / 0.851 | ×1.115 |

The C3 candidate wins 11 of 15 paired cells. **Between-repetition host
variance is large (identical cells range over ~3×, e.g. pre-C3 c=10:
8,074→25,737 req/s across reps), so cross-rep medians of raw RPS are not a
precise estimator on this host.** The paired ratio per repetition is the
fair statistic, and its median is ×1.07–1.12.

### Correction against the earlier probe

The first probe (`c3-matched-20260916-v3`, 3 s × 2, blocked ordering — all
old cells, then all new cells) reported +72.6% / +59.7% / +23.1%. The host
sped up materially during that run, and the blocked design credited the drift
to the candidate measured second. The interleaved run is the honest
magnitude: mechanism real, end-to-end gain roughly +7–12% (paired median) on
this host under concurrent soak load. Both raw runs are retained.

## Stage evidence (mechanism)

Per-request means from the benchmark-feature accumulators, averaged over the
five repetitions; these are in-process per-request means over ≥30k requests
per cell and are insensitive to host throughput drift.

| stage (us/req) | pre-C3 c=1 | C3 c=1 | pre-C3 c=10 | C3 c=10 | pre-C3 c=50 | C3 c=50 |
|---|---:|---:|---:|---:|---:|---:|
| native params/query validation | 1.708 | 1.692 | 1.265 | 1.178 | 1.147 | 1.086 |
| request-meta construction | 0.299 | 0.045 | 0.274 | 0.038 | 0.270 | 0.030 |
| prevalidated JSON → JS | 1.333 | 1.381 | 0.777 | 0.747 | 0.696 | 0.654 |
| context construction | 16.103 | 8.352 | 12.904 | 5.976 | 13.129 | 5.200 |
| handler sync stage | n/a (baseline dump) | 15.530 | n/a | 10.953 | n/a | 9.231 |

The stages reproduce the earlier probe exactly: request-meta construction is
~90% removed and context construction roughly halves, while validation and
the Value→JS conversion are unchanged by design. This confirms the removed
work is precisely the request-store/slot lifecycle and the lazy-context
machinery, not the handler or codec.

## Remaining-path cost and the bridge decision

Remaining per-request costs in the C3 path (c=10/c=50): context construction
~5.2–6.0 us, handler round-trip ~9.2–11.0 us, Value→JS conversion
~0.65–0.75 us (single-param object; c=1 shows ~1.3–1.4 us cold-cache noise).

Decision boundary as set by the owner: a direct typed bridge is justified at
1–2+ us of conversion cost and not justified at 0.2–0.4 us. Measured cost is
between, nearer the low end at concurrency, and the two stages above it are
4–15× larger. **Disposition: do not build the ValidatedScalar bridge yet.**
The next measurement targets are decomposed below.

## Slotless-path decomposition (follow-up probe)

Four additional benchmark-feature timers (`fn_restore`, `pre_object_total`,
`handler_invoke`, `resp_convert`) split the engine round trip. Run
`c3-decompose-20260916` (same pack and protocol, 5 s × 3 reps, zero errors);
`handler_sync` now excludes `resp_convert` (timed separately), and the parts
reconcile with the whole at c=10/50 (0.12 + 2.75 + 5.65 + 1.53 + ~0.4 glue
≈ 10.48 us measured).

| stage (us/req) | c=1 | c=10 | c=50 | share of engine round trip (c=10/50) |
|---|---:|---:|---:|---:|
| fn restore (3 Persistent restores) | 0.202 | 0.121 | 0.119 | ~1% |
| pre + routePlan object assembly (incl. Value→JS) | 4.132 | 2.753 | 2.804 | ~26% |
| — of which Value→JS conversion | 1.232 | 0.694 | 0.646 | ~7% |
| `__velquMakeCtx` JS execution | 7.829 | 5.652 | 5.625 | **~54%** |
| handler invocation (`run_fn.call`) | 2.052 | 1.525 | 1.438 | ~14% |
| response conversion (`value_to_outcome`) | 3.178 | 1.966 | 1.932 | ~18% (separate from handler_sync) |
| handler_sync total (restore+pre+ctx+invoke+glue) | 14.766 | 10.481 | 10.403 | 100% |

**Reading:** the actual JavaScript handler costs ~1.4–1.5 us, while building
its invocation context costs ~8.4 us at concurrency (makeCtx 5.6 + pre/plan
shell ~2.1 + Value→JS 0.65) — roughly 5.6× the handler itself. The dominant
single stage is `__velquMakeCtx`, which for the slotless case performs
`Object.create`, two non-enumerable `defineProperty` calls, two closure
allocations (`lazy`, `hasPre`; `lazy` is unused when slotless), four
`hasPre` probes, and 1–4 property stores.

Optimization candidates this evidence supports (NOT implemented here;
each is a separate packet with its own decision):

1. Slotless makeCtx fast path — skip the `lazy` closure and the two
   `defineProperty` slot/generation writes when `slot === -1` (nothing on
   the slotless path reads them; `webRequest()` — the only reader — requires
   a valid slot regardless).
2. Per-route cached `routePlan` JS object — the plan object is identical
   for every invocation of a route and is rebuilt per request inside the
   ~2.1 us pre/plan shell. Caching carries a shared-mutation hazard
   (handlers could mutate `ctx.routePlan`), so it needs an explicit
   owner decision on the mutability contract.
3. Response conversion (1.9 us) is response-path territory (C1 follow-up),
   not input-path.

Raw: `benchmarks/raw/c3-probe/c3-decompose-20260916.jsonl` /
`.summary.json` (sha256 below).

## Clean-install verification disposition

`scripts/verify` locally reports one TypeScript failure:
`clean-install.test.ts` step 4 (`bun test` inside a freshly scaffolded
starter app). Classification:

- the failing starter test targets `http://127.0.0.1:3000` and is designed
  to skip when no dev server listens;
- on this workstation a local service (`zcode-web`) listens on 0.0.0.0:3000
  and answers `GET /health/live` with `200 text/html`, so Treaty returns
  `{ data: undefined, error: null }` and the skip guard does not trigger;
- the same failure reproduces on clean master `5170c5a6` (stash/apply) and
  CI is green on master for #1374–#1376, where port 3000 is free and the
  test skips as designed;
- the Rust runtime is never booted by this test, so the C3 change cannot be
  causal.

No test was weakened and no default port was changed for a host-local
collision; the failure is recorded here as environmental. The benchmark
manifest hash rejection in `verify` is also expected: the runtime binary
hash changed and the canonical manifest must only be refreshed through the
established evidence process once this packet is accepted.

## Provenance hashes

- baseline pack: `2e60319fb3164d960bb5a78d821dca38f7475b3a7c08dd8b69843c1681c70cc3`
- pre-C3 instrumented runtime (b8fee349 + stage timers only):
  `26acaba6b28cad6c89fd1ecdd03a0c5abd591c0461cadedb4987a245ed18b441`
- C3 instrumented runtime (working tree incl. JSON-null→JS-null fix):
  `00188acbf311369048f033987e0f6bee9f375abd0893d5993463ccc7a82ad6f1`
- interleaved raw JSONL: `1053e43c588c4b217ad4242b10358e3c48b15407aa1ceb581997f0c4b4cbaaa7`
- interleaved summary: `968d03b43d82a597246bc028eefe2949f6c8c9496923000ce8bd94c0b3908768`
- decomposition raw JSONL: see `benchmarks/raw/c3-probe/c3-decompose-20260916.jsonl` (this follow-up probe was captured after the interleaved run; hashes below)
- decomposition summary: `benchmarks/raw/c3-probe/c3-decompose-20260916.summary.json`
  - jsonl `2e30dfb8c1caf38752dc1206c90b7525708e66e84873bd769fc5a8cb7d7e59df`, summary `dc632b55fcb0b68914ae8409c0676b66a65a8d2cad566bff49916042b49db8fc`
- probe raw JSONL (superseded magnitude, retained):
  `464a4e2b0fcce5799c03431bf033b007a98aa9e8321e1b8ffa63cc2b715ce8ba`

The baseline worktree (`velqu-caphost` at b8fee349) carries only the
benchmark stage timers and a local feature alias; its runtime logic is
untouched upstream of measurement.
