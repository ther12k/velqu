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

## Addendum — native exporter landed (same day, follow-up packet)

The GAP column above is now partially closed by the native, opt-in
`/metrics` endpoint (`--metrics on` / `VELQU_METRICS=on`; default off):

- **Implemented (lock-free counters, no engine mutex, no QuickJS path):**
  `velqu_http_requests_total{status_class}` (aggregate 2xx/3xx/4xx/5xx),
  `velqu_dispatcher_queue_length`, `velqu_dispatcher_queue_capacity`,
  `velqu_request_slots_live`, `velqu_request_slots_capacity`,
  `velqu_invocations_pending` (short bookkeeping mutex — not the engine
  mutex), `velqu_load_shed_total{reason}` (closed 7-reason set),
  `velqu_drain_refused_total`, `velqu_engine_quarantined`,
  `velqu_worker_poison_events_total`, `velqu_native_tasks_total{state}`.
- **Deliberately excluded:** `heapUsedBytes` (engine mutex — stays in the
  drain-time `ops.worker.status` event until a lock-free cached snapshot
  is justified); runtime latency histograms (none exist — `record` keeps
  totals + µs max only; runtime p95 is NOT synthesized; latency remains
  proxy-side per the original validation).
- **Queue alert fixed to capacity-relative semantics:**
  `velqu_dispatcher_queue_length / velqu_dispatcher_queue_capacity > 0.8`
  — the previous absolute `> 800` target could never fire at the default
  `--max-queue` 256 (the runtime sheds first).
- The worker_ops_status doc comment ("available on demand") was
  corrected; `/metrics` is now the genuine on-demand surface.
- Label cardinality is fixed: 4 status classes + 7 shed reasons + 3 task
  states. No URL/request-id/header labels.

## Addendum 2 — live Prometheus validation (raw evidence)

`docs/production/evidence/raw/m8-002-prometheus-scrape-validation.txt`
(sha256 `b928295f7d3c5b17626cd598b7142f1a40e1b5287593556a444e22af2bfec04d`):
Prometheus v2.53.1 scraped a live release runtime with `--metrics on`;
target health `up`; all seven query checks passed (status-class counters
including scrape self-counting, capacity gauges = configured bounds, shed
and poison counters, and the capacity-relative queue-pressure expression
evaluates). The six alert rules in SLOS_AND_ALERTS.md §3 pass
`promtool check rules` (6 rules found, SUCCESS).

## Addendum 3 — alert-rule source binding corrected (owner review)

Three alert families still referenced wrong/nonexistent series after the
exporter landed (`status=~"5.."` instead of `status_class="5xx"`;
`velqu_dispatcher_queue_rejected_total` instead of the real closed-set
`velqu_load_shed_total`; `velqu_worker_restarts_total` — no such series —
instead of the real poison/quarantine semantics). promtool had validated
SYNTAX, not series existence; the live evidence had not executed those
expressions. Fixed in SLOS_AND_ALERTS.md §3:

- 5xx critical/warning: aggregate-first ratio over
  `velqu_http_requests_total{status_class="5xx"}`, annotated as
  runtime-DIAGNOSTIC (scrapes/health traffic dilute the ratio at low
  volume; the authoritative availability SLO stays proxy-side — §2
  Availability row updated the same way);
- load shedding: `sum(rate(velqu_load_shed_total{reason!="draining"}[1m])) > 10`
  (planned drain excluded);
- worker trouble split into `velqu_engine_quarantined == 1` (immediate)
  and `rate(velqu_worker_poison_events_total[5m]) > 0` (loop);
- §2 Queue Health SLI rebound to the real load-shed counter;
- runbook worker-quarantine step updated to the real series.

Validation: promtool SUCCESS (7 rules); every corrected expression
EXECUTED against the live Prometheus scrape (status=success, computed
values); counter accuracy proven by direct exposition diff (declared 502
→ `status_class="5xx"` 0→1). Raw transcript extended (supersedes the
earlier hash; current sha256
`061cda4a5344e71e49c03f5d0b1a73da396a6fc935467e308deb89365c9c4790`).
A rate()-on-fresh-series extrapolation artifact is documented in the
transcript as standard Prometheus semantics, not an exporter defect.
