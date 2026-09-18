# C3 slotless A/B — three-host paired run (2026-09-18, diagnostic evidence)

## Question

Does removing request-store/slot lifecycle from validated dynamic-JS
invocations (the C3 slotless packets) generalize beyond one machine?
This is the C3 **A/B evidence class**: what ONE Velqu optimization changes
relative to its own baseline. It must not be spliced into the canonical
framework comparison (#1391, five-framework position of `f716c153`); the
two snapshots here bracket the change and are not that snapshot.

## Snapshots (user-pinned rules, verified by ancestry)

- **OLD = `5170c5a6`** — exact pre-slotless master (parent of #1377 "c3:
  slotless prevalidated context"). `f716c153` was considered as baseline
  per the original suggestion but **already contains both slotless
  packets** (#1377, #1379 are its ancestors), so it is NOT a valid
  baseline; `5170c5a6` is the correct "before".
- **NEW = `b51927ed`** — post-slotless master (#1377 + #1379 makeCtx fast
  path, merged on top of the decomposition probe `11e27a6b`).
- Runtime diff is exactly the slotless packet set (q-bridge,
  q-engine-quickjs, q-runtime serve/lib, tests) — no compiler, QuickJS
  upgrade, ValidatedScalar, or DirectTextResponsePlan changes are mixed
  in. Route: canonical dynamic `GET /hello/Rafi` only. Pack built from
  the NEW tree (compiler unchanged between the snapshots — no
  `packages/` diff in the packet range): **identical pack on all hosts**
  (`9ca122bd…`).

Both binaries built with `--features bench-instrumentation` (identical
instrumentation on both sides, diagnostic stage timing only; production
builds carry none). x86_64 binaries are byte-identical across the two
x86 hosts; the aarch64 pair differs by architecture as expected.
Hashes in each run's `host-*.txt` under `benchmarks/raw/c3-probe/`.

## Protocol

Balanced interleaved pairing (v3 harness, `benchmarks/harness/c3-probe.ts`):
within each repetition the old/new pair runs back-to-back per concurrency
level and the ORDER alternates per repetition (odd old→new, even new→old),
so ambient drift cannot systematically favor a candidate. 10 s cells,
c = 1/10/50, fresh runtime process per cell, zero errors required (and
achieved: every cell of every run). Repetitions: 10 pairs local, 6 pairs
Halotec, 6 pairs Oracle — the recommended 8–10 on the noisy local host.
Halotec/Oracle measured inside the same digest-pinned `ubuntu:24.04`
toolchain image as the multihost lane; local used the same image. Halotec
cap `--cpus=2`; Oracle uncapped (rootless crun rejects `--cpus`; the VM
itself is 1 vCPU, the same effective condition as the 2026-09-15 lane).

## Results (paired median new/old throughput ratio per host)

| host | old c=1 | new c=1 | old c=10 | new c=10 | old c=50 | new c=50 |
|---|---|---|---|---|---|---|
| local (10 pairs) | 7584 | 9008 | 14331 | 17668 | 16331 | 19302 |
| Halotec (6) | 2208 | 2351 | 9569 | 10237 | 13248 | 14550 |
| Oracle (6) | 6104 | 6746 | 13838 | 18149 | 16513 | 23432 |

| host | c=1 ratio | c=10 ratio | c=50 ratio |
|---|---|---|---|
| local | **1.187** (0.96–1.44) | **1.210** (1.04–1.36) | **1.210** (1.08–1.51) |
| Halotec | **1.062** (0.99–1.15) | **1.073** (1.05–1.11) | **1.103** (1.08–1.12) |
| Oracle | **1.119** (0.97–1.30) | **1.308** (1.16–1.53) | **1.412** (1.24–1.45) |

p50/p95 improve in the same direction on every host (e.g. Oracle c=10
p50 689.5→506.6 µs; local c=10 p95 799→644 µs). Full raw JSONL,
summaries, per-cell stage dumps, and host records:

- `benchmarks/raw/c3-probe/c3x-local-20260918T162021Z*`
- `benchmarks/raw/c3-probe/c3x-halotec-20260918T155522Z*`
- `benchmarks/raw/c3-probe/c3x-oracle-20260918T161129Z*`

### Reading (per the owner's decision rules)

- **Direction is consistent on all three hosts and at all three
  concurrencies** — every paired median is above 1.0 (one individual pair
  each dipped below 1.0 at c=1 on local and Oracle; paired medians did
  not). The change is architectural, not a host artifact.
- **The earlier +72.6% / +59.7% / +23.1% do NOT reproduce.** Those cells
  were ordered (all-old then all-new) on a noisy desktop and absorbed
  ambient drift. Under balanced pairing the same machine shows
  +18.7/+21.0/+21.0%. This re-confirms the M24 lesson: only paired,
  order-alternating cells are trustworthy.
- **Magnitude is host-dependent**: +6–10% on the Halotec Xeon, +19–21%
  flat on the local hybrid-core laptop, and +12%→+31%→+41% GROWING with
  concurrency on the 1-vCPU Oracle (removing per-invocation bookkeeping
  relieves a single-CPU contention bottleneck). The user's "one host +2%
  while others +50%" inconsistency case did not occur; no investigation
  blocker. Because the c=50 shrinkage seen locally in 2026-09-16 did NOT
  appear cross-host (Oracle gains most at c=50), the earlier "gains
  shrink with concurrency" theory is retired as host-specific.
- Diagnostic evidence only (AGENTS constraint 12): no acceptance
  threshold, gate, or production claim derives from these numbers.

## Stage decomposition (median µs/op; ns/n over the measured window)

The two legacy stages are the only directly comparable pair (the
request/context timers were introduced BY the C3 packet series, so the
old side instrumented only these two — the pre-slotless request_meta /
context_construct costs come from the committed decomposition evidence,
`c3-decompose-20260916` + `docs/reports/c3-slotless-context-findings.md`):

| stage (c=1 medians) | local old→new | Halotec old→new | Oracle old→new |
|---|---|---|---|
| handler_sync (engine invocation) | 21.3→8.9 | 50.2→25.9 | 22.8→11.1 |
| resp_encode_direct (sanity pair) | 1.41→1.14 | 2.92→2.62 | 1.13→1.10 |

New-side stage budget at c=1 (local/Halotec/Oracle µs):
`request_meta` 0.04/0.08/0.04 — the slot lifecycle is effectively gone;
`context_construct` 2.8/8.4/4.1; `prevalidated_json_to_js` (the
serde_json::Value → QuickJS bridge) 1.06/3.35/1.43; handler body ~1.8–4.9;
`resp_encode_direct` 1.1–2.6.

### ValidatedScalar decision input (per the owner's rule)

The Value→JS bridge costs **0.45–3.35 µs** across hosts/concurrency —
at c=1 that is <1% of the ~85–360 µs request cost, and the share shrinks
further at c=10/c=50. Per the agreed rule ("if it remains tiny, do not
build ValidatedScalar merely because it sounds architecturally
elegant"), **the typed bridge stays unbuilt**. The remaining engine-side
stages (context_construct, handler invocation) are likewise one to two
orders of magnitude below total request cost; no engine-side hotspot
justifies the next packet from this evidence — remaining C3 cost is
dominated by HTTP/client/queue machinery outside the engine.

## Disclosed deviations and incidents

1. **Oracle launch deviation**: `run.sh --cpus=2` rejected by rootless
   crun (no cpu controller delegated); build AND measurement containers
   ran uncapped on the 1-vCPU VM (recorded in `host-oracle.txt`).
2. **Oracle artifact recovery**: the wrapper's in-container
   `chown -R` failed under the rootless subuid mapping (work-dir files
   owned by host uid 100999; opc unmapped), so the automated artifact
   copy/host-file/postcondition never ran and the cleanup trap deleted
   the opc-owned build/measure logs. `out/` and `bin/` survived and were
   recovered via `podman unshare` tar extraction; the JSONL/summary/
   stage files are the unmodified container-written artifacts, and every
   cell was cross-checked against the captured run transcript. No
   measurement was re-run or altered.
3. **Wrapper defects fixed in this packet** (post-run steps only; the
   measurement loop was never affected): postcondition read the summary
   from the wrong path; `systemd-detect-virt`'s nonzero exit on bare
   metal aborted the host-file write under `set -e`; phase-1 container
   missed the `:Z` SELinux relabel; chown failure now falls back to
   `chmod a+rX`. The host files record which harness revision ran where
   (local: commit `00723a20`; Halotec/Oracle: transferred kit files,
   identical harness).
4. **Halotec image rebuild**: the pinned benchmark image had been pruned
   on Halotec; it was rebuilt from the same Dockerfile and same
   `f716c153` tree (new image id `e43bad1f…`, recorded in
   `host-halotec.txt`).
5. A first full local attempt was killed by a session-restore mid-build
   and produced no measurements; the recorded local run is a clean
   relaunch. Smoke runs (`c3x-smoke*`) were deleted and never used as
   evidence.

## Disposition

- C3 slotless improvement **generalizes**: consistent positive paired
  effect on all three hosts.
- Magnitudes are host-specific; the canonical lane's normative numbers
  remain those of the framework comparison, not these.
- **ValidatedScalar: not justified** by measured share; move on.
