# BETA-003-B — First Request Through Steady State

Status: **MEASURED** (generated evidence) + **DRAFT wording** (not approved
public positioning). Companion to BETA-003-A (controlled I/O / payload / CPU
cells) on the loopback harness, with the Velqu candidate included.

## Reproducible method

- Command (from `benchmarks/harness/`): `bun ramp.ts` (defaults: 3 reps,
  400-request cap, classes C0 = `health.live` native-transport floor and
  C2 = `js.json` JS-handler work).
- Protocol per sample: fresh process → TCP-accept poll → sequential
  validated requests from request #0 (every response byte-checked against
  the frozen fixture contract) → deterministic steady-state criterion →
  terminate. The criterion is wall-clock-independent: windows of 25
  requests; a transition is flat when the window median is within
  [0.8×, 1.25×] of the previous window median; steady onset = the first
  window k ≥ 2 whose transition and the preceding one are both flat, and
  the run continues until ≥ 50 requests past onset. A series still decaying
  or regressing at the cap is reported as **no onset** (never extrapolated).
- Raw evidence: `benchmarks/raw/ramp/` — per-request JSONL rows
  (`ramp-*.jsonl`, phase-tagged), `summary.json` (`velqu-ramp-v1`), and the
  generated `ramp-report.md`. Pinned baseline deps install from the
  committed lockfile when absent; nothing resolves unpinned versions.
- Environment: Bun 1.4.0, Node v22.23.2, i5-13420H, linux, commit `dcf4264`
  (master head this packet branched from). Single run on a shared
  development host — the method is reproducible; canonical claims need a
  quiet-host rerun (BETA-014).

## Results (generated; verbatim from `benchmarks/raw/ramp/ramp-report.md`)

| candidate | class | first p50 (µs) | steady p50 (µs) | first/steady | steady onset (req #) | errors | RSS (kB) |
|---|---|---:|---:|---:|---:|---:|---:|
| velqu | C0 | 268 | 34 | 7.88 | 100 | 0 | 9768 |
| velqu | C2 | 270 | 59 | 4.58 | 50 | 0 | 9792 |
| raw-rust | C0 | 248 | 61 | 4.07 | 150 | 0 | 3456 |
| raw-rust | C2 | 238 | 45 | 5.29 | 150 | 0 | 3404 |
| raw-bun | C0 | 3258 | 48 | 67.88 | 175 | 0 | 26516 |
| raw-bun | C2 | 3902 | 41 | 95.17 | 150 | 0 | 26280 |
| elysia2 | C0 | 10060 | 35 | 287.43 | 75 | 0 | 46296 |
| elysia2 | C2 | 14811 | 37 | 400.30 | 200 | 0 | 46544 |

## Honest reading (measured-run scope, shared host)

1. **All candidates reach an equivalent steady state** (34–61µs p50) — at
   steady state, per-request cost on these routes is dominated by transport
   + handler dispatch, and every runtime is competitive.
2. **The first request is where they differ by orders of magnitude**: the
   JIT/AOT JS runtimes pay 3.3–14.8ms on the first request (engine + AOT
   codegen warmup), reaching steady state after ~50–200 requests. Velqu's
   first request (~270µs) is much closer to its steady state because
   handlers are pre-compiled QPack bytecode and the worker exists before
   the first request; raw-rust behaves likewise (native, nothing to warm).
3. **First/steady ratios**: velqu ≈ 4.6–7.9×; raw-rust ≈ 4.1–5.3×;
   raw-bun ≈ 68–95×; elysia2 ≈ 287–400×. Under short-lived processes or
   burst-after-deploy traffic, this is the cost profile that matters.
4. **RSS** differs by runtime (velqu ~9.8MB, raw-rust ~3.4MB, Bun ~26MB,
   Elysia ~46MB) — sampled per process, single run.
5. Where Velqu **loses**: its steady-state p50 (34–59µs) sits at the same
   level as, but not below, the JS runtimes here; the QuickJS handler is
   not faster than warmed JIT code — the advantage is the near-absence of a
   warmup cliff, not a lower floor. This sentence is part of the honest
   framing BETA-003-D will enforce across all reports.

## Public wording draft (DRAFT — not approved; do not publish)

> In our harness, all measured frameworks settled to equivalent per-request
> latencies at steady state. They differed sharply on the first requests
> after a cold start: runtimes that compile code at startup needed tens to
> hundreds of requests to reach that steady state, while Velqu's
> pre-compiled handlers were within a few times of steady state from the
> first request. Single-host measurements, published with raw per-request
> data; not a production-environment claim.

This draft must not be published: it requires quiet-host reruns, the
BETA-003-C cumulative crossover-request analysis, the BETA-003-D
honest-losses pass, and owner review. No production-readiness or
"fastest" claims are made anywhere in this packet.
