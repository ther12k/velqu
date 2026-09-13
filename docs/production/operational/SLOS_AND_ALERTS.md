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

| Objective | Metric / Indicator (SLI, 5-minute rolling) | SLO Target (30-day window) | Measurement Point |
|---|---|---|---|
| **Availability** | `sum(rate(http_requests_total{status!~"5.."}[5m])) / sum(rate(http_requests_total[5m]))` (aggregate first; per-series division is wrong — see above) | **≥ 99.9%** successful requests | Ingress reverse proxy & runtime metrics |
| **P95 Latency (Light/Static)** | P95 duration for C0 (liveness) and C1 (text) requests | **≤ 15 ms** | Host HTTP ingress listener |
| **P95 Latency (JSON/Validated)** | P95 duration for C2 (JSON) and C3 (schema-validated) | **≤ 35 ms** | Host HTTP ingress listener |
| **Readiness Recovery** | Time from startup or post-drain to `/health/ready` 200 OK | **≤ 5.0 s** | Health probe poller |
| **Queue Health** | `sum(rate(dispatcher_queue_rejected_total[5m])) / sum(rate(http_requests_total[5m]))` (aggregated the same way) | **≤ 0.01%** of requests shed | Dispatcher queue rejected counter |
| **Memory Retention** | Process RSS drift post-warmup | **Flat (no monotonic growth beyond tolerance over 24h)** | OS `/proc/<pid>/status` / cgroup memory |

---

## 3. Prometheus Alerting Rules

```yaml
groups:
  - name: velqu_production_alerts
    rules:
      # Availability Alerts
      - alert: VelquHigh5xxErrorRate
        expr: (sum(rate(velqu_http_requests_total{status=~"5.."}[5m])) / sum(rate(velqu_http_requests_total[5m]))) > 0.01
        for: 2m
        labels:
          severity: critical
        annotations:
          summary: "Velqu HTTP 5xx error rate exceeds 1% over 5m"
          runbook: "docs/production/operational/SLOS_AND_ALERTS.md#runbook-high-error-rate"

      - alert: VelquElevated5xxWarning
        expr: (sum(rate(velqu_http_requests_total{status=~"5.."}[5m])) / sum(rate(velqu_http_requests_total[5m]))) > 0.001
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "Velqu HTTP 5xx error rate exceeds 0.1% over 5m"

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
        expr: velqu_dispatcher_queue_length > 800
        for: 1m
        labels:
          severity: warning
        annotations:
          summary: "Dispatcher task queue depth exceeds 800 slots (capacity 1024)"
          runbook: "docs/production/operational/SLOS_AND_ALERTS.md#runbook-queue-saturation"

      - alert: VelquActiveLoadShedding
        expr: rate(velqu_dispatcher_queue_rejected_total[1m]) > 10
        for: 30s
        labels:
          severity: critical
        annotations:
          summary: "Velqu is actively shedding load (rejected requests due to bounded queue)"

      # Process Health
      - alert: VelquWorkerQuarantineLoop
        expr: rate(velqu_worker_restarts_total[5m]) > 0.1
        for: 2m
        labels:
          severity: critical
        annotations:
          summary: "QuickJS worker restarting repeatedly (quarantine loop detected)"
          runbook: "docs/production/operational/SLOS_AND_ALERTS.md#runbook-worker-quarantine"
```

---

## 4. Operator Runbook

### Runbook: Graceful Drain and Deployment
1. Set instance status to draining by signaling the host process or proxy.
2. The runtime sets `/health/ready` to `503 Service Unavailable`, prompting the upstream proxy to withdraw traffic.
3. Existing active request slots settle normally within the bounded request deadline (default 15s).
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
1. A saturated queue (> 800 slots) indicates ingress arrival rate exceeds single-worker processing capacity.
2. Check if downstream capabilities are stalling handlers (e.g., slow database queries holding worker slots).
3. Scale horizontal instances behind the load balancer to distribute request volume.

### Runbook: Worker Quarantine
1. If a worker panics or exceeds memory limits, the dispatcher isolates the faulty worker and restarts a fresh instance.
2. If `velqu_worker_restarts_total` fires, inspect stderr logs for the panic backtrace or unhandled runtime fault.
3. Verify that the QPack bundle has not been corrupted.
