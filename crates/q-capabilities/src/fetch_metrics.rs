//! Fetch metrics schema (M28-009-A).
//!
//! Per-request fetch observations for operational diagnosis — pool wait,
//! DNS, connect, TLS, TTFB, and body stages plus error/cancellation
//! counts. The schema is bounded (fixed stage array, saturating counters)
//! and redacted by construction: no URLs, no header values, no timing
//! side-channel beyond stage durations. The observation path is plain
//! saturating integer adds — no allocation, no locks, no strings — so
//! disabled/ enabled instrumentation costs nothing but the adds.

use serde::Serialize;

/// The fetch lifecycle stages, in observation order. The order is the
/// schema: snapshots serialize stages under these stable names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FetchStage {
    /// Waiting for a pooled connection (0 when a new connection was used).
    PoolWait,
    /// DNS resolution of the target host.
    Dns,
    /// TCP connect.
    Connect,
    /// TLS handshake.
    Tls,
    /// Time to first body byte.
    Ttfb,
    /// Body transfer (first byte to last).
    Body,
}

impl FetchStage {
    /// All stages in schema order.
    pub const ALL: [FetchStage; 6] = [
        FetchStage::PoolWait,
        FetchStage::Dns,
        FetchStage::Connect,
        FetchStage::Tls,
        FetchStage::Ttfb,
        FetchStage::Body,
    ];

    /// Stable snake_case name used in snapshots.
    pub fn name(self) -> &'static str {
        match self {
            FetchStage::PoolWait => "pool_wait",
            FetchStage::Dns => "dns",
            FetchStage::Connect => "connect",
            FetchStage::Tls => "tls",
            FetchStage::Ttfb => "ttfb",
            FetchStage::Body => "body",
        }
    }

    fn slot(self) -> usize {
        match self {
            FetchStage::PoolWait => 0,
            FetchStage::Dns => 1,
            FetchStage::Connect => 2,
            FetchStage::Tls => 3,
            FetchStage::Ttfb => 4,
            FetchStage::Body => 5,
        }
    }
}

/// Bounded, redacted fetch observations (M28-009-A). Everything saturates —
/// a pathological run can never overflow-panic or grow the structure.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FetchMetrics {
    stage_nanos: [u64; 6],
    requests: u32,
    errors: u32,
    cancellations: u32,
}

/// The redacted, serializable snapshot of [`FetchMetrics`]. Field set is
/// the schema and is pinned by test: stages plus three counters — nothing
/// else can leak in.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FetchMetricsSnapshot {
    pub pool_wait_ns: u64,
    pub dns_ns: u64,
    pub connect_ns: u64,
    pub tls_ns: u64,
    pub ttfb_ns: u64,
    pub body_ns: u64,
    pub requests: u32,
    pub errors: u32,
    pub cancellations: u32,
}

impl FetchMetrics {
    /// Empty observations.
    pub fn new() -> Self {
        FetchMetrics::default()
    }

    /// Accumulate `nanos` for `stage` (saturating).
    #[inline]
    pub fn observe_stage(&mut self, stage: FetchStage, nanos: u64) {
        self.stage_nanos[stage.slot()] = self.stage_nanos[stage.slot()].saturating_add(nanos);
    }

    /// Record one completed request.
    #[inline]
    pub fn record_request(&mut self) {
        self.requests = self.requests.saturating_add(1);
    }

    /// Record one typed fetch error (saturating).
    #[inline]
    pub fn record_error(&mut self) {
        self.errors = self.errors.saturating_add(1);
    }

    /// Record one cancellation (saturating).
    #[inline]
    pub fn record_cancellation(&mut self) {
        self.cancellations = self.cancellations.saturating_add(1);
    }

    /// Nanos accumulated for `stage`.
    pub fn stage_nanos(&self, stage: FetchStage) -> u64 {
        self.stage_nanos[stage.slot()]
    }

    /// Completed requests observed.
    pub fn requests(&self) -> u32 {
        self.requests
    }

    /// Errors observed.
    pub fn errors(&self) -> u32 {
        self.errors
    }

    /// Cancellations observed.
    pub fn cancellations(&self) -> u32 {
        self.cancellations
    }

    /// Redacted snapshot: stages under stable names plus the three
    /// counters. The field set is the schema — nothing else can leak in.
    pub fn snapshot(&self) -> FetchMetricsSnapshot {
        FetchMetricsSnapshot {
            pool_wait_ns: self.stage_nanos[FetchStage::PoolWait.slot()],
            dns_ns: self.stage_nanos[FetchStage::Dns.slot()],
            connect_ns: self.stage_nanos[FetchStage::Connect.slot()],
            tls_ns: self.stage_nanos[FetchStage::Tls.slot()],
            ttfb_ns: self.stage_nanos[FetchStage::Ttfb.slot()],
            body_ns: self.stage_nanos[FetchStage::Body.slot()],
            requests: self.requests,
            errors: self.errors,
            cancellations: self.cancellations,
        }
    }

    /// Merge `other` into `self` (saturating) for aggregation.
    pub fn merge(&mut self, other: &FetchMetrics) {
        for (dst, src) in self.stage_nanos.iter_mut().zip(other.stage_nanos.iter()) {
            *dst = dst.saturating_add(*src);
        }
        self.requests = self.requests.saturating_add(other.requests);
        self.errors = self.errors.saturating_add(other.errors);
        self.cancellations = self.cancellations.saturating_add(other.cancellations);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metrics_schema_covers_all_stages_in_order() {
        // The stage order IS the schema; the snapshot field order matches.
        assert_eq!(
            FetchStage::ALL.iter().map(|s| s.name()).collect::<Vec<_>>(),
            ["pool_wait", "dns", "connect", "tls", "ttfb", "body"]
        );
        let mut m = FetchMetrics::new();
        for stage in FetchStage::ALL {
            m.observe_stage(stage, 100);
        }
        m.record_request();
        let snap = m.snapshot();
        assert_eq!(
            snap,
            FetchMetricsSnapshot {
                pool_wait_ns: 100,
                dns_ns: 100,
                connect_ns: 100,
                tls_ns: 100,
                ttfb_ns: 100,
                body_ns: 100,
                requests: 1,
                errors: 0,
                cancellations: 0
            }
        );
    }

    #[test]
    fn stage_and_counter_observations_saturate_without_panicking() {
        let mut m = FetchMetrics::new();
        m.observe_stage(FetchStage::Body, u64::MAX);
        m.observe_stage(FetchStage::Body, u64::MAX); // saturates, no panic
        assert_eq!(m.stage_nanos(FetchStage::Body), u64::MAX);
        for _ in 0..100_000 {
            m.record_error();
            m.record_cancellation();
            m.record_request();
        }
        assert_eq!(m.errors(), 100_000);
        assert_eq!(m.cancellations(), 100_000);
        assert_eq!(m.requests(), 100_000);
    }

    #[test]
    fn snapshot_field_set_is_the_redaction_boundary() {
        // The serialized snapshot must contain EXACTLY the schema keys —
        // no URL, no header, no host data can ever appear.
        let mut m = FetchMetrics::new();
        m.observe_stage(FetchStage::Ttfb, 7);
        m.record_request();
        m.record_error();
        m.record_cancellation();
        let json = serde_json::to_string(&m.snapshot()).unwrap();
        let keys: Vec<&str> = vec![
            "pool_wait_ns",
            "dns_ns",
            "connect_ns",
            "tls_ns",
            "ttfb_ns",
            "body_ns",
            "requests",
            "errors",
            "cancellations",
        ];
        for key in keys {
            assert!(json.contains(key), "missing schema key {key}");
        }
        assert_eq!(json.matches(',').count(), 8, "unexpected extra fields");
        assert!(!json.contains("url") && !json.contains("header"));
    }

    #[test]
    fn merge_aggregates_saturating() {
        let mut a = FetchMetrics::new();
        a.observe_stage(FetchStage::Dns, 5);
        a.record_request();
        let mut b = FetchMetrics::new();
        b.observe_stage(FetchStage::Dns, 7);
        b.record_error();
        a.merge(&b);
        assert_eq!(a.stage_nanos(FetchStage::Dns), 12);
        assert_eq!(a.requests(), 1);
        assert_eq!(a.errors(), 1);
        // Merging an empty observation changes nothing.
        let before = a.snapshot();
        a.merge(&FetchMetrics::new());
        assert_eq!(a.snapshot(), before);
    }
}
