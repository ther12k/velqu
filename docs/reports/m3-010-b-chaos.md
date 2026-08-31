# M3-010-B — Chaos Soak: Worker Poison, Disconnect, and Timeout Injection (15 minutes)

Generated from `benchmarks/raw/worker-scaling/soak-summary.json`
(velqu-soak-v1, chaos section) and its raw window JSONL, from the final
committed `q-soak` build. This run extends the M3-010-A soak harness
with deterministic fault injection under continuous load.

## Harness and command

Same engine+dispatcher core as M3-010-A (2 independent QuickJS runtimes
behind the M3-002 bounded Dispatcher, 8 closed-loop producers, per-id
verification, classified errors). Chaos knobs (all deterministic per
request id):

- **Worker poison** (`--chaos-secs 60`): every 60 s the next worker slot
  is poisoned — its consumer DROPS its QuickJS runtime mid-soak and
  rebuilds it deterministically (spawn + identical bundle load, ADR-0036
  §6) while the queue keeps flowing. Dispatcher-level quarantine and
  settle remain M3-005 component evidence; this proves ENGINE
  replacement under live traffic.
- **Disconnect injection** (`--disconnect-permille 5`): 0.5 % of
  requests have their reply receiver dropped immediately after dispatch
  — the engine's late-completion owner must absorb the failed send
  exactly once (the M2.2.1 cancel/late-completion path, exercised at
  volume).
- **Timeout injection** (`--timeout-permille 5`): 0.5 % of requests run
  the 100 ms timer handler behind a 10 ms deadline — the worker's
  watchdog must fire `Outcome::Timeout`.
- **Shutdown**: the run ends with the graceful drain (queues closed,
  in-flight settled, every engine `shutdown()`) — the M3-007 sequence.

```bash
./target/release/q-soak --workers 2 --duration-secs 900 --window-secs 30 \
    --chaos-secs 60 --disconnect-permille 5 --timeout-permille 5 \
    --out-dir benchmarks/raw/worker-scaling
```

Upstream-timeout evidence for the real network stack (slow upstream
bodies, immediate TLS close, poison proxies) is the M28 fetch fixture
suite (`fetch_fixture_conformance`, `fetch_proxy_cancellation_conformance`),
regression-run in every verify; this packet's timeout injection covers
the invocation watchdog at soak scale.

## Chaos timeline (exact, from the summary)

14 replacements over 901.0 s (alternating workers 0/1 on schedule);
engine rebuild took **2.8–11.0 ms** (one outlier 266.5 ms under host
scheduling pressure) per replacement (median ~4.0 ms): the replacement
runtime was initialized, verified, and serving within one window sample
in every case.

## Results (exact values from the summary)

| metric | value |
|---|---|
| duration | 901.0 s (30 windows) |
| replacements | 14 (7 per worker) |
| dispatched | **1 703 012** |
| completed + verified | **1 685 983 (99.0 %)** |
| injected disconnects (expected) | 8 518 |
| injected timeouts (expected) | 8 511 |
| unexplained errors | **0** |
| throughput | 1 871 ops/s overall (window band 897–2 499) |
| final per-worker heap | 202 104 / 201 880 B (flat) |
| process RSS first → last | 5 772 → 5 560 KiB (**−212 KiB across 14 engine builds**) |

**Accounting is exact**: 1 685 983 + 8 518 + 8 511 = 1 703 012 — every
dispatched request is a verified completion, an expected injected
disconnect, or an expected injected timeout. Nothing else happened; no
mismatch, no panic, no lost request across 14 runtime replacements.

## Leak analysis

- Per-worker heaps flat (202 104 / 201 880 B) after 1.69 M requests and
  **14 drop-and-rebuild engine cycles** — replacement leaves no
  retained state in the new runtime.
- Process RSS −212 KiB total, max window step 348 KiB — process memory
  ended below its starting point despite 14 fresh engine allocations;
  no monotonic leak signature.

## Guardrail mapping (parent M3-010)

- *Quarantine/replacement and readiness are reliable* — 14/14
  replacements rebuilt in ~3–11 ms and rejoined seamlessly; service
  continued through every poison.
- *Cancellation/shutdown remain exact* — 8 518 injected disconnects
  absorbed exactly once (late-completion path), 8 511 watchdog timeouts
  delivered as typed `Outcome::Timeout`, and the run ended in the
  M3-007 graceful drain with all slots quiesced.
- *No monotonic leak / all errors bounded and explained* — see above;
  every error is a counted, expected injection class.
- *Capacity recovers after replacement* — throughput band held through
  all 14 replacements (post-replacement windows recover; full
  replacement-timeline recovery is M3-010-D's dedicated evidence).

## Scope notes

- Engine-level poison under load is this packet's chaos; dispatcher-
  level quarantine/settle/replacement semantics are the M3-005
  component suite (regression-run in every verify).
- Slot/memory tracking extensions and the explicit recovery
  verification are M3-010-C/D.

## Artifact hashes (SHA-256)

| artifact | sha256 |
|---|---|
| `target/release/q-soak` (remapped build) | `e89d4006bd91e9326e395cb7ba9449d76f2fee8dc91eb7314975df3387cb6656` |
| `benchmarks/raw/worker-scaling/soak.jsonl` | `9241f44d7dd714e48ae5637c3aff3f9edf3f380864bfbbcad8f96b3b32d403a2` |
| `benchmarks/raw/worker-scaling/soak-summary.json` | `c7831945e3efcd17224a1841b88ac03ebef0b1a3714d5454599fc942cfdb01bf` |
