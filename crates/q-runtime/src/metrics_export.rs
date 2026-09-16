//! M8-002: native Prometheus text exposition over lock-free bounded counters.
//!
//! Design constraints (owner-reviewed):
//! - NO engine mutex on the scrape path: every value here is an atomic
//!   read from `ServeState` atomics, `EngineHealth` (worker-shared block +
//!   bridge counters), or the drain/load-shed/drain counters. The only
//!   lock taken is `InvocationOwnership::stats()`'s short bookkeeping
//!   mutex — never the engine mutex.
//! - NO invented latency histograms: `RouteStatusMetrics` keeps totals +
//!   µs max, not buckets; runtime p95 stays proxy-side until a bounded
//!   histogram is added deliberately.
//! - NO dynamic labels: the only label sets are the fixed `status_class`
//!   (4 values) and the closed load-shed reason set (7 values). No URL
//!   path, request id, error message, or header-derived labels.
//! - Excluded (documented choice): QuickJS `heapUsedBytes` requires the
//!   engine mutex; it stays in the drain-time `ops.worker.status` event
//!   until a lock-free cached snapshot is justified.

use std::fmt::Write as _;
use std::sync::atomic::Ordering;

use crate::serve::ServeState;
use q_capabilities::LOAD_SHED_KINDS;

/// Closed load-shed reason label set — index order matches
/// `LoadShedCounters::snapshot_counts` (`kind()` strings).
pub const LOAD_SHED_REASONS: [&str; LOAD_SHED_KINDS] = [
    "worker_queue_full",
    "all_workers_full",
    "global_admission_full",
    "class_ceiling",
    "long_running_slots",
    "draining",
    "tracking_full",
];

/// Render the full Prometheus 0.0.4 text exposition for one scrape.
pub fn render(state: &ServeState) -> String {
    let mut out = String::with_capacity(2048);

    // ---- runtime request/status counters (always-on aggregation)
    let agg = state.route_metrics.aggregate_status_snapshot();
    out.push_str("# HELP velqu_http_requests_total HTTP requests completed, aggregated by response status class.\n");
    out.push_str("# TYPE velqu_http_requests_total counter\n");
    for (class, n) in [
        ("2xx", agg.ok_2xx),
        ("3xx", agg.redirect_3xx),
        ("4xx", agg.client_error_4xx),
        ("5xx", agg.server_error_5xx),
    ] {
        let _ = writeln!(
            out,
            "velqu_http_requests_total{{status_class=\"{class}\"}} {n}"
        );
    }

    // ---- pressure: dispatcher queue + request slots
    let queue_pending = state.metrics.queue_pending.load(Ordering::Relaxed);
    out.push_str("# HELP velqu_dispatcher_queue_length Dispatched invocations currently pending across worker queues.\n");
    out.push_str("# TYPE velqu_dispatcher_queue_length gauge\n");
    let _ = writeln!(out, "velqu_dispatcher_queue_length {queue_pending}");
    out.push_str(
        "# HELP velqu_dispatcher_queue_capacity Configured admission queue bound (--max-queue).\n",
    );
    out.push_str("# TYPE velqu_dispatcher_queue_capacity gauge\n");
    let _ = writeln!(
        out,
        "velqu_dispatcher_queue_capacity {}",
        state.dispatcher_queue_capacity
    );
    let slots_live = state.health.live_slots();
    out.push_str("# HELP velqu_request_slots_live Live request-store slots.\n");
    out.push_str("# TYPE velqu_request_slots_live gauge\n");
    let _ = writeln!(out, "velqu_request_slots_live {slots_live}");
    out.push_str("# HELP velqu_request_slots_capacity Configured request-store slot capacity.\n");
    out.push_str("# TYPE velqu_request_slots_capacity gauge\n");
    let _ = writeln!(
        out,
        "velqu_request_slots_capacity {}",
        state.request_slot_capacity
    );

    // ---- invocations pending (short bookkeeping mutex, never the engine mutex)
    let ownership = state.ownership.stats();
    out.push_str("# HELP velqu_invocations_pending Invocations awaiting settlement.\n");
    out.push_str("# TYPE velqu_invocations_pending gauge\n");
    let _ = writeln!(out, "velqu_invocations_pending {}", ownership.pending);

    // ---- load shedding: closed reason set
    let shed = state.load_shed.snapshot_counts();
    out.push_str("# HELP velqu_load_shed_total Requests refused by bounded admission, by closed reason set.\n");
    out.push_str("# TYPE velqu_load_shed_total counter\n");
    for (reason, n) in LOAD_SHED_REASONS.iter().zip(shed.iter()) {
        let _ = writeln!(out, "velqu_load_shed_total{{reason=\"{reason}\"}} {n}");
    }
    let drain_refused = state.drain_gate.refused();
    out.push_str(
        "# HELP velqu_drain_refused_total Requests refused because the instance is draining.\n",
    );
    out.push_str("# TYPE velqu_drain_refused_total counter\n");
    let _ = writeln!(out, "velqu_drain_refused_total {drain_refused}");

    // ---- worker lifecycle / health (lock-free worker-shared atomics)
    let quarantined = u8::from(state.health.is_quarantined());
    out.push_str("# HELP velqu_engine_quarantined Whether the QuickJS worker is quarantined (1 = dynamic JS routes fail closed).\n");
    out.push_str("# TYPE velqu_engine_quarantined gauge\n");
    let _ = writeln!(out, "velqu_engine_quarantined {quarantined}");
    let poison = state.health.poison_events();
    out.push_str(
        "# HELP velqu_worker_poison_events_total Cumulative worker quarantine/poison events.\n",
    );
    out.push_str("# TYPE velqu_worker_poison_events_total counter\n");
    let _ = writeln!(out, "velqu_worker_poison_events_total {poison}");
    let started = state.health.native_tasks_started();
    let completed = state.health.native_tasks_completed();
    let aborted = state.health.native_tasks_aborted();
    out.push_str("# HELP velqu_native_tasks_total Native operations by terminal state (started is cumulative; completed/aborted are terminal).\n");
    out.push_str("# TYPE velqu_native_tasks_total counter\n");
    let _ = writeln!(
        out,
        "velqu_native_tasks_total{{state=\"started\"}} {started}"
    );
    let _ = writeln!(
        out,
        "velqu_native_tasks_total{{state=\"completed\"}} {completed}"
    );
    let _ = writeln!(
        out,
        "velqu_native_tasks_total{{state=\"aborted\"}} {aborted}"
    );

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_shed_reason_labels_match_the_closed_set() {
        assert_eq!(LOAD_SHED_REASONS.len(), LOAD_SHED_KINDS);
        for r in LOAD_SHED_REASONS {
            assert!(!r.is_empty());
            assert_eq!(r, r.to_ascii_lowercase());
        }
    }
}
