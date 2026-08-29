//! M28-009-V overhead report input: measures the enabled-instrumentation
//! observation path (saturating adds under a collector shard lock).
//! Informational only — prints ns/op; no timing assertions.

use q_capabilities::{FetchMetrics, FetchMetricsCollector, FetchStage};
use std::time::Instant;

#[test]
fn observe_path_overhead_measurement() {
    const N: u64 = 10_000_000;
    let mut plain = FetchMetrics::new();
    let t0 = Instant::now();
    for i in 0..N {
        plain.observe_stage(FetchStage::PoolWait, i & 0xFFFF);
    }
    let plain_ns = t0.elapsed().as_nanos() as u64 / N;

    let collector = FetchMetricsCollector::shared();
    let t1 = Instant::now();
    for i in 0..N {
        collector.observe_stage(FetchStage::PoolWait, i & 0xFFFF);
    }
    let collector_ns = t1.elapsed().as_nanos() as u64 / N;

    assert!(plain.stage_nanos(FetchStage::PoolWait) > 0);
    assert!(collector.sample().pool_wait_ns > 0);
    println!(
        "m28-009-v overhead: plain observe ~{plain_ns} ns/op; collector (mutex shard) ~{collector_ns} ns/op; disabled path = no call (0 ns)"
    );
}
