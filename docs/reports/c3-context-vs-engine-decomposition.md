# C3 decomposition — engine vs in-worker glue vs host↔worker handoff (2026-09-18, diagnostic)

Follow-up to the C3 slotless cross-host A/B (#1392). That run left one
question: the remaining dynamic-route cost is overwhelmingly Velqu
integration — but is the next-largest component **in-worker context
machinery** or the **host↔worker handoff**? This packet measures exactly
that split, engine-only (no network).

## Method

`q-c3-decompose` (feature-gated `bench-direct` kit, built in the pinned
multihost image), five tiers:

- **A `raw_call`** — the C3-shaped handler invoked as a plain function in
  a bare rquickjs context, `{params}` wrapper pre-created (engine floor).
- **A2 `raw_call_extract`** — A plus one string field read back per op.
- **B `worker_direct`** — the production worker code path (`WorkerInner`
  + `begin_invocation`: makeCtx, run(), conversions, promise settlement
  postlude) executed DIRECTLY on the calling thread via the diagnostic
  `bench_direct::DirectWorker` — no channel, no wakeup, no scheduler.
- **C `channel_invoke`** — full production dispatch: host →
  `QuickJsEngine::invoke` → mpsc → worker wakeup → (B's work) → oneshot
  reply.
- **B3 `worker_direct_repeat`** — B repeated on a freshly constructed
  worker AFTER C: a stability guard that separates real in-worker cost
  from run-order artifacts (added after the incident below).

Derived: **B − A = context construction + in-worker glue**;
**C − B = channel + queue + wakeup + scheduling**. Spec construction is
hoisted out of every timed window (identical for B and C). 120 timed
batches per tier (1000 ops/batch for A/A2, 200 for B/C/B3), correctness
asserted before and after every timed phase, fail-closed postconditions.

## Results (p50 µs/op)

| host | A engine | B in-worker | C channel | B3 repeat | glue (B−A) | handoff (C−B) |
|---|---|---|---|---|---|---|
| local (i5-13420H) | 0.192 | 7.186 | 11.927 | 7.169 | **6.99 (58.6% of C)** | **4.74 (39.8%)** |
| Oracle (Neoverse-N1, 1 vCPU) | 0.359 | 11.676 | 16.457 | 11.806 | **11.32 (68.8%)** | **4.78 (29.0%)** |
| Halotec (Xeon 8163, kernel 4.15) | 0.385 | 13.542 | 74.131 | 14.021 | **13.16 (17.7%)** | **60.59 (81.7%)** |

B3 agrees with B on every host (|Δ| ≤ 0.48 µs): the in-worker numbers
are stable, not ordering artifacts. Raw JSONL + summaries + host records:
`benchmarks/raw/c3-decompose-engine/c3d-{local,halotec,oracle}-20260918T21*`.

## Reading — different dominant costs by host

The owner's anticipated outcome materialized exactly:

- **On Halotec, the handoff dominates massively**: 60.6 µs of scheduler
  channel/wakeup per invocation (81.7%), versus 13.2 µs of in-worker
  glue. The old 4.15 kernel's thread-wakeup path is the single largest
  engine-side cost on that host — no context-construction optimization
  can touch it. This is the machine the owner hypothesized
  (A≈0.5, B≈12, C≈87).
- **On local and Oracle, in-worker glue dominates** (58.6% / 68.8%),
  with a modest ~4.8 µs handoff on both.
- Engine execution stays negligible everywhere (0.19–0.39 µs).
- The split also **explains the #1392 cross-host gain variation**: the
  slotless change removed per-invocation in-worker work, which is why
  the 1-vCPU Oracle gained most under concurrency while Halotec's gains
  stayed bounded by its handoff ceiling.

## Decision input (owner's rules, mixed outcome disclosed)

Both candidate tracks are justified, on different hosts:

1. **Host↔worker handoff (Halotec-class hosts)** — the largest single
   absolute lever measured (60.6 µs on Halotec). Targets per the owner's
   list: envelope reuse, dedicated per-worker SPSC path, cheaper
   synchronous completion signalling, no scheduler participation when
   the worker is already awake. Halotec runs a 2018-era kernel; treat it
   as the conservative-host design constraint, not a bug to chase.
2. **In-worker context specialization (local/Oracle-class hosts)** —
   7–11.3 µs; the `CTX_PLAN_VALIDATED_PARAMS_ONLY` direction
   (minimal context + params, per-route metadata hoisted to build time)
   with the protected semantics (own-property, nullability,
   `Object.keys()`, generic path retained for full-request/policy/lazy).
3. **SYNC-handler specialization** — the canonical handler is
   `async`; a compiler-known synchronous handler could simplify both the
   in-worker path and completion signalling. Deserves measurement after
   either track lands.
4. **QuickJS-NG upgrade / PGO / XS: parked.** Even a magical 2× engine
   (0.4→0.2 µs) moves a 16–74 µs path by noise.
5. **ValidatedScalar: parked** (revisit at ≥5–10% of C3 request cost).

Priority recommendation (owner to confirm): the handoff track first on
absolute impact and host coverage — it is 29–82% of C everywhere and
61 µs on the oldest host — with context specialization as the follow-up
once dispatch is leaner. Full-request p50 context stays far larger
(85–360 µs in #1392), so neither track changes the framework-comparison
tables on its own.

## C3 taxonomy note (frozen)

C0 native fast path; C1/C2 AOT constants; C3 dynamic validated JS — the
KPI for QuickJS integration efficiency. C3 stays engine-bound: no native
folding of `/hello/:name`.

## Incidents and disclosures

1. **Oracle tier-B inversion (first v2 run, superseded in-packet):** the
   initial two-tier→three-tier port timed tier B with spec construction
   INSIDE the window while tier C hoisted it; on the 1-vCPU host this
   produced a physically impossible B > C (29.7 vs 16.3 µs). Fixed by
   making both windows identical and adding the B3 repeat guard; the
   recorded runs are the corrected protocol with B ≈ B3 on every host.
   The superseded run was never merged and is not in-tree.
2. **Halotec raw-artifact stranding (resolved):** the 2026-09-18
   workstation reboot wiped system ssh/scp/docker and GitHub credentials
   mid-packet; the Halotec two-tier run's raw directory sat remotely
   overnight and was retrieved + postcondition-verified after the
   environment returned (identical-quality evidence, all three hosts;
   the preserved two-tier transcript remains in-tree as the incident trail). The recorded
   three-tier Halotec run above was measured after restoration and
   transferred immediately.
3. The `bench-direct` feature compiles nothing into production: no
   default or production crate enables it, and the q-c3-decompose bin
   carries `required-features = ["bench-direct"]` so workspace builds
   skip it entirely.
4. Wrapper defects fixed in-packet (bin-dir creation, cpus-limit record,
   source-commit flag, postcondition tier set); none touch the measured
   tiers.
5. Descriptive/diagnostic evidence only (AGENTS constraint 12).
