# Service Level Objectives (SLOs), Alerts, and Operator Runbook (M8-002)

This document establishes operational Service Level Indicators (SLIs), Service Level Objectives (SLOs), alerting rules, and operator runbooks for Velqu production deployments.

---

## 1. Distinction: CI Benchmark Floors vs Service SLOs

A critical architectural distinction governs Velqu performance governance:

- **CI Benchmark Floors (`benchmarks/gate-thresholds.json`)**:
  Coarse merge-blocking regression checks designed to prevent catastrophic performance collapses (e.g. 10x-36x latency blowups or 70-90% throughput collapses) on noisy shared CI virtual machines without flaking.
- **Service Level Objectives (This Document)**:
  Operational reliability and latency commitments for production services under realistic production workloads and infrastructure, monitored via live telemetry.

---

## 2. Service Level Indicators (SLIs) and Objectives (SLOs)

The PromQL expressions below are the **rolling 5-minute service-level indicators** — the short-window signal operators watch live. The **SLO targets are evaluated over a 30-day window** by aggregating those recorded rates (e.g. via recording rules) into an error-budget ratio over the month; the 5-minute expression is not itself the 30-day evaluation.

Numerator and denominator are aggregated with `sum(...)` over the whole service scope **before** division. Without the aggregation, vector matching pairs each status-code series with itself (e.g. status=200 numerator over status=200 denominator), yielding a constant 1.0 for every positive-rate series and hiding 5xx traffic entirely.

> **Measurement-source status (validated 2026-09-16, updated by the native exporter packet — `docs/production/evidence/m8-002-slo-source-validation.md`):**
> the runtime now ships an **opt-in native `/metrics` endpoint**
> (`--metrics on` / `VELQU_METRICS=on`; default off) exposing the
> pressure/load-shed/lifecycle series below — `velqu_http_requests_total{status_class}`,
> `velqu_dispatcher_queue_length`, `velqu_dispatcher_queue_capacity`,
> `velqu_request_slots_live/capacity`, `velqu_invocations_pending`,
> `velqu_load_shed_total{reason}`, `velqu_drain_refused_total`,
> `velqu_engine_quarantined`, `velqu_worker_poison_events_total`,
> `velqu_native_tasks_total{state}`. **No runtime latency histogram exists**
> (request durations are totals+max internally, not buckets) — latency p95
> and end-to-end availability remain **ingress-proxy** measurements, and
> runtime p95 must not be synthesized from averages. RSS remains an OS
> `/proc/<pid>/status` poll. Production deployments should restrict
> `/metrics` at the reverse proxy or network boundary.

| Objective | Metric / Indicator (SLI, 5-minute rolling) | SLO Target (30-day window) | Measurement Point |
|---|---|---|---|
| **Availability** | **Authoritative: ingress reverse proxy** success ratio (30-day). A runtime-diagnostic ratio (`sum(rate(velqu_http_requests_total{status_class!="5xx"}[5m])) / sum(rate(velqu_http_requests_total[5m]))`) exists for alerting but includes scrape/health traffic and is NOT the SLO of record | **≥ 99.9%** successful requests | Ingress reverse proxy (authoritative); runtime `/metrics` (diagnostic) |
| **P95 Latency (Light/Static)** | P95 duration for C0 (liveness) and C1 (text) requests | **≤ 15 ms** | Host HTTP ingress listener |
| **P95 Latency (JSON/Validated)** | P95 duration for C2 (JSON) and C3 (schema-validated) | **≤ 35 ms** | Host HTTP ingress listener |
| **Readiness Recovery** | Time from startup or post-drain to `/health/ready` 200 OK | **≤ 5.0 s** | Health probe poller |
| **Queue Health** | `sum(rate(velqu_load_shed_total{reason!="draining"}[5m])) / sum(rate(velqu_http_requests_total[5m]))` (aggregated before division) | **≤ 0.01%** of requests shed | Runtime `/metrics` load-shed counters (closed reason set; planned drain excluded) |
| **Memory Retention** | Process RSS drift post-warmup | **Flat (no monotonic growth beyond tolerance over 24h)** | OS `/proc/<pid>/status` / cgroup memory |

---

## 3. Prometheus Alerting Rules

> **Series status:** every series referenced below except the latency
> histogram is now emitted by the native exporter (when enabled). The
> latency rules remain **proxy-side targets** — the runtime exports no
> `velqu_http_request_duration_seconds_bucket` (see the measurement-source
> status above); load those rules against your proxy's own histogram series.
> Loading the runtime rules against a runtime with `/metrics` disabled
> yields empty series (no false alerts, no coverage).

```yaml
groups:
  - name: velqu_production_alerts
    rules:
      # Availability Alerts — RUNTIME RATIO IS DIAGNOSTIC. The runtime
      # counter aggregates all requests INCLUDING /metrics scrapes and
      # native health traffic (the unknown/native fallback bucket), which
      # dilutes the ratio at low volume. The authoritative 30-day
      # availability SLO is measured at the ingress proxy; treat these
      # rules as a fast in-runtime smoke signal, not the SLO of record.
      - alert: VelquHigh5xxErrorRate
        expr: (sum(rate(velqu_http_requests_total{status_class="5xx"}[5m])) / sum(rate(velqu_http_requests_total[5m]))) > 0.01
        for: 2m
        labels:
          severity: critical
        annotations:
          summary: "Velqu HTTP 5xx error rate exceeds 1% over 5m (runtime-diagnostic; SLO of record is proxy-side)"
          runbook: "docs/production/operational/SLOS_AND_ALERTS.md#runbook-high-error-rate"

      - alert: VelquElevated5xxWarning
        expr: (sum(rate(velqu_http_requests_total{status_class="5xx"}[5m])) / sum(rate(velqu_http_requests_total[5m]))) > 0.001
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "Velqu HTTP 5xx error rate exceeds 0.1% over 5m (runtime-diagnostic)"

      # Latency Alerts
      - alert: VelquHighP95Latency
        expr: histogram_quantile(0.95, sum(rate(velqu_http_request_duration_seconds_bucket[5m])) by (le)) > 0.05
        for: 3m
        labels:
          severity: warning
        annotations:
          summary: "Velqu P95 latency exceeds 50ms for 3 consecutive minutes"
          runbook: "docs/production/operational/SLOS_AND_ALERTS.md#runbook-elevated-latency"

      # Queue & Load Shedding
      - alert: VelquQueueSaturation
        expr: velqu_dispatcher_queue_length / velqu_dispatcher_queue_capacity > 0.8
        for: 1m
        labels:
          severity: warning
        annotations:
          summary: "Dispatcher queue depth exceeds 80% of its configured capacity"
          detail: "Capacity-relative by design: the runtime sheds load at its configured bound (--max-queue, default 256), so an absolute threshold would either never fire or fire too late."
          runbook: "docs/production/operational/SLOS_AND_ALERTS.md#runbook-queue-saturation"

      - alert: VelquActiveLoadShedding
        expr: sum(rate(velqu_load_shed_total{reason!="draining"}[1m])) > 10
        for: 30s
        labels:
          severity: critical
        annotations:
          summary: "Velqu is actively shedding load (bounded admission refusals, excluding planned drain)"
          detail: "reason is the closed load-shed set (worker_queue_full, all_workers_full, global_admission_full, class_ceiling, long_running_slots, tracking_full); draining is excluded as planned behavior."

      # Process Health — the runtime's worker trouble signal is
      # quarantine/poison, not process restarts. Any poison event is
      # already severe (dynamic JS routes fail closed); an elevated
      # poison RATE indicates a replacement loop.
      - alert: VelquEngineQuarantined
        expr: velqu_engine_quarantined == 1
        for: 1m
        labels:
          severity: critical
        annotations:
          summary: "QuickJS worker quarantined — dynamic JS routes are failing closed (503)"
          runbook: "docs/production/operational/SLOS_AND_ALERTS.md#runbook-worker-quarantine"

      - alert: VelquWorkerPoisonLoop
        expr: rate(velqu_worker_poison_events_total[5m]) > 0
        for: 2m
        labels:
          severity: critical
        annotations:
          summary: "Repeated worker poison events (quarantine/replacement loop detected)"
          runbook: "docs/production/operational/SLOS_AND_ALERTS.md#runbook-worker-quarantine"
```

---

## 4. Operator Runbook

### Runbook: Graceful Drain and Deployment
1. Set instance status to draining by signaling the host process or proxy.
2. The runtime sets `/health/ready` to `503 Service Unavailable`, prompting the upstream proxy to withdraw traffic.
3. Existing active request slots settle normally within the bounded request deadline (compiler default 5s per route; a route's declared `deadlineMs` governs).
4. Send `SIGTERM` to the `velqu-runtime` process. The runtime flushes state and exits `0`.

### Runbook: High Error Rate
1. Check `/health/live` and `/health/ready` on each instance.
2. Inspect application error logs: look for unhandled exceptions or rejected input schemas (422 vs 500).
3. If errors correlate with a recent canary or pack update, initiate immediate rollback per `CANARY_PROGRAM.md`.

### Runbook: Elevated Latency
1. Check CPU utilization and thread count: verify no external noisy neighbors are contending for CPU cores.
2. Inspect outbound capability latency: if handlers call upstream HTTP or PostgreSQL, check downstream database connection pool wait times.
3. Verify garbage collection / memory state: check if QuickJS memory allocations are nearing configured quotas.

### Runbook: Queue Saturation and Load Shedding
1. A queue near its configured bound (`--max-queue`, default 256) indicates ingress arrival rate exceeds processing capacity; sustained saturation ends in load shedding (`velqu_load_shed_total{reason}` on `/metrics`, or the drain-time `ops.worker.status` event).
2. Check if downstream capabilities are stalling handlers (e.g., slow database queries holding worker slots).
3. Scale horizontal instances behind the load balancer to distribute request volume.

### Runbook: Worker Quarantine
1. If a worker panics or exceeds memory limits, the dispatcher isolates the faulty worker and restarts a fresh instance.
2. If `velqu_engine_quarantined == 1` or `velqu_worker_poison_events_total` is rising, inspect stderr logs for the panic backtrace or unhandled runtime fault.
3. Verify that the QPack bundle has not been corrupted.
