# r4 launch record — ga-m6-soak-72h (b8fee349) on Halotec

Assignment (owner directive 2026-09-20: "assign the stable soak host and
its operator"): host = **Halotec VM** (149.129.48.59, halotec.my.id),
operator = the session that executed this launch. Rationale over the
 alternatives:

- The original host is barred (owner boundary 2026-09-15: three
  environment interruptions r1/r2/r3; "do not launch r4 on this host").
- Halotec: x86_64 (digest-compatible with the recorded q-soak binary —
  an aarch64 host would force a different binary identity), 4 cores /
  16 GB, **40 days uptime at assignment** (2026-08-11 boot), kernel
  4.15.0-211, docker 24.0.2 (overlay2), 132 GB free disk.
- Oracle VM rejected for this run: aarch64 (binary identity break) and
  1 vCPU co-resident with live services.

## Environment delta, stated plainly

r1–r3 executed as host processes on the original machine. r4 executes
inside the pinned `velqu-bench:multihost` container (image e1de7b18613f,
digest-pinned lineage, ubuntu 24.04 userland matching the build glibc —
the Halotec host userland is 18.04 and cannot run the binary directly).
The **binary is bit-identical** (sha256 verified on the launch host);
what changed is process containment only. Container parameters:
`--network none` (q-soak is fully in-process; no sockets), `--cpus 3`
(soak capped below the VM's 4 cores to protect co-resident production),
`--memory 4g`, `--restart no` — **no auto-restart by design**: a dead
run must surface as dead, never silently resume (checkpoint/resume is
explicitly rejected by the owner boundary).

## Launch facts (UTC)

- container: 655a7939fb4036d5387efea3a2e5b4e21e273726bdd8000dc5551884c
- PID (host namespace): 25087
- started: 2026-09-20T06:47:58Z (docker State.StartedAt)
- expected completion: 2026-09-23T06:47:58Z + summary write
- args: `--workers 2 --duration-secs 259200 --window-secs 120`
  (chaos disabled — defaults), out-dir `/soak/out` → `~/velqu-soak-r4/out`
  on the host (soak.jsonl, soak-stdout.log, soak-stderr.log)

## First-window health (seq 0, read 2026-09-20T06:50Z)

`{"seq":0,"elapsedSecs":120.09,…,"requests":131675,
"throughputOpsPerSec":1096.47,"processRssKib":5172,
"queueLens":[1024,1024],"queueTotal":2048,"queueRejectedTotal":45353,
"ownershipPendingSlots":2}` — throughput projects ≈285 M requests over
72 h (requirement ≥10 M); RSS in the r1–r3 envelope (5–6 MB); queues
bounded; container running, no OOM.

## Reporting qualifications (owner review 2026-09-20)

- **Build provenance ≠ execution provenance.** The host-toolchain build
  *reproduced the required executable* (`6497fb16…`); the pinned container
  *supplies its execution environment*. The container build recipe
  produced a different digest (`959c706d…`) and the two recipes are NOT
  interchangeable — only the host-built binary carries this candidate's
  artifact identity.
- **`--cpus=3` is a CPU-time ceiling, not three reserved cores.** The
  soak's consumption is capped; co-tenant contention on this shared
  production VM is not excluded. This is the recorded **CPU-capped,
  shared-host run** — throughput observations are not an uncontended
  baseline, and latency/throughput claims from r4 must carry that
  qualifier. **Acceptance thresholds are unchanged**; the eventual
  conclusion stays bounded to: whether this candidate, in this recorded
  shared-host configuration, over the completed workload and duration,
  satisfied the existing tolerances (kernel memory accounting and
  reclaim are stateful and pressure-dependent; no environmental
  immunity is claimed).
- **Host-observation wording (owner review 2026-09-20):** Linux load
  averages count runnable and uninterruptible-wait tasks — they are not
  CPU-utilization percentages and do not identify the cause of waiting.
  Observed at +50 min: host load ≈9.5; the soak's sampled CPU
  consumption stayed within its configured ceiling; co-tenant CPU
  usage was low. Quota-related queuing is a possible explanation;
  these observations alone establish neither the cause nor the absence
  of service impact. COMPLETION.md must keep causal attribution
  provisional unless throttling counters or other direct measurements
  support it. (Docker can update CPU limits on a running container:
  leaving the configuration unchanged is a protocol choice to preserve
  the recorded qualification configuration — not a technical
  impossibility.)
- **First-window evidence means "started successfully and producing
  observations."** The projected ≈285 M requests is a projection;
  qualification uses the actual completed totals and resource series.
- **Completion is the process's actual terminal state** — the summary
  file written by q-soak at natural expiry — never calendar inference.
  Nominal endpoint: 2026-09-23T06:47:58Z (13:47:58 WIB). With the
  10:00-WIB daily checks, the first scheduled observation *after*
  normal completion is Sep 24 10:00 WIB — an observation delay, not a
  qualification input.

## Acceptance — unchanged

Fresh continuous 72 h from launch; no concatenation with r1/r2/r3
partial hours; no synthesized summary; qualified analyzer with
`--min-hours 72` after actual completion; evidence packet + M6-009
evaluation per acceptance and ancestors' recorded residuals; no
automatic promotions. Qualification covers candidate b8fee349 only —
master has moved since (perf packets #1391–#1395); later runtime
changes require their own qualification decision.
