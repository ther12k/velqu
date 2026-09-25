# r4 completion — ga-m6-soak-72h (b8fee349) on Halotec: PASS

## Terminal state (actual, not calendar-inferred)

The process ended at its natural expiry: `soak-summary.json` (the
velqu-soak-v2 completion artifact) exists, written 2026-09-23 13:47
WIB — 72 h after the 2026-09-20T06:47:58Z launch. Last raw window:
seq 2156, elapsedSecs 259089.85 (the final partial tail is covered by
the summary's `actualDurationSecs` 259201.02 ≥ 259200). stdout:
`soak complete: /soak/out/soak.jsonl + summary (259201s, 259325600
verified)`; stderr 0 bytes. First scheduled post-completion
observation (Sep 24 10:00 WIB) lagged the endpoint — an observation
delay, not a qualification input.

Host: 45 days uptime at observation — no power event during the run
(r1/r2/r3's failure mode did not recur). Provenance note: the
`velqu-soak-r4` container object no longer exists for inspection
(the launcher's docker daemon config lives under `/tmp` and was
cleaned after exit); terminal state is established by the process's
own artifacts above plus file mtimes. The bind-mounted evidence
directory was unaffected.

## Identity (unchanged from LAUNCH.md)

Candidate `b8fee349`; q-soak sha256 `6497fb16…6794d` (host-built,
verified on the launch host); `--workers 2 --duration-secs 259200
--window-secs 120`, chaos disabled; pinned `velqu-bench:multihost`
container, `--network none --cpus 3 --memory 4g --restart no`.

## Analyzer verdict (qualified, independent recomputation)

`python3 scripts/analyze-soak.py benchmarks/raw/ga-m6-soak-72h-r4/soak.jsonl --min-hours 72 --capacity 2048 --expected-workers 2` → **PASS**, exit 0:

```
samples                : 2157 windows
elapsed                : 71.97 h (summary actualDurationSecs 259201.02 >= 259200)
completed (window sum) : 259,212,928   (harness summary total 259,325,600; delta -112,672 = 0.04% tail)
throughput overall     : 1000 ops/s (window min/mean/max: 416/1000/1103)
RSS initial/final/peak : 5172 / 8752 / 8752 KiB
RSS growth (raw span)  : +3580 KiB
max window step        : +312 KiB
terminal-quarter slope : +28.9 KiB/h (last 539 windows)
peak queue slots       : 2048 (configured capacity 2048)
ownership pending      : peak 2

  PASS  recomputation agrees with harness summary
  PASS  T1 drift 0.014 B/completed-request <= 1.0
  PASS  T2 terminal slope 28.9 KiB/h <= 250 over final quarter
  PASS  T3 max window step 312 KiB <= 2048
  PASS  T4 >= 10,000,000 completed requests and >= 72 h duration
  PASS  T5 ownership pending peak 2 <= workers 2; queue peak 2048 <= capacity 2048
```

Harness summary corroborates: totalDispatched = totalCompletedVerified
= 259,325,600, completionRate 1.0, zero errors by class, ownership
pendingAtShutdown 0, per-worker heap delta ~0 B; process RSS drift
3580 KiB characterized by the harness as bounded allocator retention.

## Reading the RSS series (bounded conclusion)

Hourly means rose from 5.4 MB (h0–6) through ~8.3 MB (h42–48), then
held a 7.7–8.5 MB band for the final ~24 h. The acceptance question —
monotonic growth beyond tolerance — is answered by the analyzer's
numeric tolerances, all PASS: whole-run drift 0.014 B/request, and the
terminal-quarter slope (28.9 KiB/h vs the 250 KiB/h bound) shows the
rise decelerating, consistent with bounded retention rather than a
leak. This is the conclusion **for this candidate, in this recorded
CPU-capped shared-host configuration, over this completed workload and
duration** — acceptance thresholds were unchanged, and no environmental
immunity is claimed (kernel memory accounting and reclaim are stateful
and pressure-dependent). The r1–r3 flat 4.6–6.2 MB envelope differed;
the execution-environment delta (containerized userland, CPU cap) is
recorded context, offered provisionally without causal attribution.

## Artifacts in this directory

- `soak.jsonl` — 2157 raw windows (sha256
  `da72a17332c7717a6c24a3db27400912617933b7c55cfe9c8987391ed2d8d9c3`)
- `soak-summary.json` — harness completion summary (sha256
  `e8e26e3f6e06e80fdfa15db27d95ae32f8faf238fbea4c7f7ad90f19eb1888a2`)
- `rss-over-time.svg` — resource graph over all windows
- `soak-stdout.log` (76 B), `soak-stderr.log` (0 B)
- `candidate-commit.txt`, `LAUNCH.md` — identity, launch, qualifications

r1/r2/r3 partial hours are preserved separately and were **not**
combined with this run.
