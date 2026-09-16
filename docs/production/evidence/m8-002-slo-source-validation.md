# M8-002 — SLO Measurement-Source Validation (evidence)

Issue #1321 / task M8-002. Owner review question: **does every SLI/SLO in
`docs/production/operational/SLOS_AND_ALERTS.md` correspond to a metric
Velqu really exports, with the exact labels/units/cardinality the PromQL
and dashboards assume?**

Answer after source inspection (`crates/q-runtime/src/serve.rs`
`worker_ops_status`, `crates/q-capabilities/src/load_shed.rs`,
`crates/q-runtime/src/config.rs`, `crates/q-engine` EngineStats surface,
`examples/proof/src/modules/ops/routes.ts`): **No — partially.** The
alert rules reference a planned exporter surface that does not exist
today, and two doc claims contradict the implementation. This report is
the source-of-truth binding; the SLO document is corrected in the same
packet.

## Headline finding

**Velqu ships NO Prometheus exposition surface.** There is no
`/metrics` endpoint, no text-exposition encoder, and no `velqu_*` series
anywhere in the runtime. Every series named in the alert rules
(`velqu_http_requests_total`, `velqu_http_request_duration_seconds_bucket`,
`velqu_dispatcher_queue_length`, `velqu_dispatcher_queue_rejected_total`,
`velqu_worker_restarts_total`) is a **target specification for a future
exporter packet, not a measured source today.** The SLO doc previously
presented them as if they existed — exactly the "plausible PromQL over
nonexistent metrics" failure mode this validation was commissioned to
catch.

Two further contradictions found and corrected:

1. `VelquQueueSaturation` annotation claimed "capacity 1024". The
   admission queue default is `--max-queue` **256**
   (`DEFAULT_MAX_QUEUE`, `crates/q-runtime/src/config.rs:36`);
   1024 is neither the default nor a constant — a threshold must be set
   relative to the configured bound, not a hardcoded number.
2. The drain runbook claimed "bounded request deadline (default 15s)".
   The compiler-emitted default route deadline is **5000 ms**
   (`deadlineMs: 5000`, `packages/compiler/src/emit.ts`).

## What IS exported today (exact surface)

**There is no on-demand runtime telemetry pull surface.** The only HTTP
telemetry endpoints are the native `/health/live` and `/health/ready`
probes. The rich counters below exist in memory and are exposed ONLY as
the structured log event `ops.worker.status` — emitted ONCE, at drain
time (`crates/q-runtime/src/lib.rs` drain watcher; single call site of
`worker_ops_status`). A scraper cannot poll it; at exit,
`shutdown.complete` carries the same engine stats once. Exact keys of
that snapshot (`worker_ops_status`, serve.rs):

| path | type | unit / cardinality |
|---|---|---|
| `queue.pending` | gauge | tasks in worker queues |
| `queue.slabLive`, `slots.live`/`slots.capacity` | gauge | live request slots vs configured cap |
| `queue.invocationsPending` | gauge | invocations awaiting settlement |
| `worker.quarantined`, `worker.queuePoisoned` | bool | instantaneous health |
| `worker.poisonEvents` | counter | cumulative quarantine/poison events |
| `memory.heapUsedBytes` | gauge | QuickJS heap bytes (NOT RSS) |
| `tasks.nativeStarted/Alive/Completed/Aborted` | counters | native op lifecycle |
| `drain.draining`, `drain.refused` | bool / counter | drain gate state and refusals |
| `loadShed.{worker_queue_full, all_workers_full, global_admission_full, class_ceiling, long_running_slots, tracking_full, draining}` | counters | cumulative sheds per closed reason set (7 kinds, `load_shed.rs`) |
| `pools.fetch.*`, `pools.postgres.*` | gauges/counters | outbound pool state |

Other real sources: `/health/live` + `/health/ready` HTTP probes;
structured log events (`ready` startup line, `drain.begin`,
`handler.error`, `shutdown.complete` with cumulative engine stats);
process RSS only via OS `/proc/<pid>/status` (external poller — the
soak harness does exactly this).

## Per-SLI binding

| SLO doc SLI | verdict | real source today |
|---|---|---|
| Availability (5xx ratio) | PROXY-SIDE OK / runtime GAP | measured at the ingress reverse proxy (the doc's own declared measurement point); runtime cannot express it until an exporter exists |
| P95 latency (C0/C1, C2/C3) | PROXY-SIDE OK / runtime GAP | histogram must come from proxy or a future runtime exporter; runtime keeps internal per-route µs metrics but exposes no pull surface |
| Readiness recovery ≤ 5 s | OK | `/health/ready` probe poller; endpoints exist and are the declared measurement point |
| Queue health (shed ratio ≤ 0.01%) | GAP via pull; DERIVABLE from drain/exit logs | `loadShed.*` cumulative counters + request count from proxy — but today they surface only in the drain-time `ops.worker.status` log event and `shutdown.complete`; no scrapeable endpoint |
| Memory retention (flat RSS) | OK (external) | OS `/proc/<pid>/status` poller (soak harness precedent); `memory.heapUsedBytes` supplements with in-engine heap |
| Worker restarts | GAP via pull; DERIVABLE from drain/exit logs | `worker.poisonEvents` + service-profile `replacements` counters are the real quarantine-loop signal, exposed only in the drain-time/exit log events; `velqu_worker_restarts_total` does not exist |

## Disposition

- `SLOS_AND_ALERTS.md` corrected in this packet: exporter-surface status
  callout, §3.1 real-source bindings with exact keys, capacity and
  deadline fixes. Alert expressions KEPT as the target specification
  (they are directionally sound and pre-aggregated correctly), now
  explicitly labeled as forward-looking until the exporter packet.
- M8-002 rebinds **TODO → IN_PROGRESS** with this report as evidence.
  PASS remains gated on `M8-001` (its ledger dependency, itself gated on
  M7-GATE) and on the dashboard-validation acceptance item, which needs
  the exporter or the poller bridge actually running against a live
  instance with recorded dashboard config.
- A runtime Prometheus exporter (or at minimum an on-demand status route
  a json_exporter bridge can scrape) is the natural follow-up packet and
  would close most of the GAP column without new runtime semantics;
  until then, the only continuous observability is proxy-side metrics,
  health polling, and OS RSS polling, plus drain/exit-time log events.
