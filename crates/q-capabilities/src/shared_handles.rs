//! Shared-handle taxonomy (M3-001-D, ADR-0036 §4).
//!
//! ADR-0036 closes the vocabulary of mutable state that may cross worker
//! boundaries to exactly four disciplines. This module names the contract
//! in the type system: a [`SharedAcrossWorkers`] impl is a compile-time
//! declaration that a handle follows one of those disciplines — interior
//! mutability behind locks/atomics only, bounded growth, saturating or
//! dropped overflow, never a JS value inside.
//!
//! Explicit impls only (no blanket impl): sharing is an auditable decision
//! per type, not something a type gets by accident.

use crate::console::BoundedLogSink;
use crate::fetch_metrics::FetchMetricsCollector;

/// Marker for handles safe to share across worker threads.
///
/// Safety contract (ADR-0036 §4): all interior mutability is behind
/// locks or atomics; growth is bounded; overflow saturates or drops by
/// policy; no QuickJS value is ever stored inside.
pub trait SharedAcrossWorkers: Send + Sync + 'static {}

// Metric-shard discipline: fixed-size FetchMetrics behind a mutex shard,
// saturating adds only (M28-009-A/B).
impl SharedAcrossWorkers for FetchMetricsCollector {}

// Metric-shard discipline: the bounded log sink (M27-004-C) — mutex
// ring buffer with a hard cap; overflow drops and counts.
impl SharedAcrossWorkers for BoundedLogSink {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    fn assert_shared<T: SharedAcrossWorkers>() {}

    #[test]
    fn shared_handles_are_send_sync_static() {
        assert_shared::<FetchMetricsCollector>();
        assert_shared::<BoundedLogSink>();
    }

    #[test]
    fn shared_handles_work_behind_arc_from_any_thread() {
        // The sharing shape host code actually uses: Arc<T> handed to
        // worker threads and recorded into.
        let collector = Arc::new(FetchMetricsCollector::shared());
        let c2 = collector.clone();
        let sink = Arc::new(BoundedLogSink::new(crate::console::DEFAULT_LOG_SINK_CAP));
        let s2 = sink.clone();
        let h = std::thread::spawn(move || {
            c2.record_error();
            assert_eq!(c2.sample().errors, 1);
            let _ = s2;
        });
        h.join().unwrap();
        assert_eq!(collector.sample().errors, 1);
    }
}
