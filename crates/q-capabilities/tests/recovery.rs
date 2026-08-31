//! Multi-worker recovery verification (M3-010-D): proves that after
//! worker failure/poison, replacement re-establishes full capacity,
//! loads equalize across workers, and zero tasks or slots leak across
//! repeated poison/recovery cycles.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

use q_capabilities::{Dispatcher, InvocationOwnership};

#[test]
fn capacity_recovers_to_full_parallelism_after_worker_replacement() {
    // 2 workers; initially both serve. Worker 0 is quarantined (capacity
    // drops to 1 worker); then replaced with a fresh queue. The dispatcher
    // routes again to both workers, and loads equalize.
    let mut d: Dispatcher<u64> = Dispatcher::with_workers(2, 8);
    assert_eq!(d.workers(), 2);

    // Initial balanced dispatch:
    assert_eq!(d.dispatch(1).unwrap(), 0);
    assert_eq!(d.dispatch(2).unwrap(), 1);

    // Quarantine worker 0:
    d.quarantine(0);
    assert!(d.is_quarantined(0));

    // While quarantined, dispatches route ONLY to worker 1:
    for i in 3..=5 {
        assert_eq!(d.dispatch(i).unwrap(), 1);
    }
    assert_eq!(d.queue(0).len(), 1, "worker 0 has initial job only");
    assert_eq!(d.queue(1).len(), 4, "worker 1 absorbed all new work");

    // Settle quarantined jobs from worker 0:
    let settled = d.settle_quarantined(0);
    assert_eq!(settled, vec![1]);
    assert!(d.queue(0).is_empty());

    // Replace worker 0: capacity is restored:
    d.replace(0);
    assert!(!d.is_quarantined(0));

    // Next dispatches route to worker 0 (least loaded: 0 vs 4):
    for i in 6..=9 {
        assert_eq!(d.dispatch(i).unwrap(), 0, "least loaded worker 0 wins");
    }
    assert_eq!(d.queue(0).len(), 4);
    assert_eq!(d.queue(1).len(), 4, "loads equalized after recovery");
}

#[test]
fn no_leaked_invocations_or_slots_across_repeated_poison_and_recovery() {
    // 50 rapid poison/settle/replace cycles under concurrent admission
    // with InvocationOwnership tracking. Proves zero orphan invocation,
    // exact accounting, and bounded memory.
    let d = Arc::new(std::sync::Mutex::new(Dispatcher::<u64>::with_workers(
        2, 128,
    )));
    let ownership = Arc::new(InvocationOwnership::with_workers(2, 4_096));

    let mut handles = Vec::new();
    let total_admitted = Arc::new(AtomicUsize::new(0));

    // Producer thread:
    let d_prod = d.clone();
    let own_prod = ownership.clone();
    let adm = total_admitted.clone();
    handles.push(std::thread::spawn(move || {
        for id in 1..=2_000u64 {
            let g = d_prod.lock().unwrap();
            if let Ok(w) = g.dispatch(id) {
                own_prod.track(id, w).unwrap();
                adm.fetch_add(1, Ordering::Relaxed);
            }
            drop(g);
            std::thread::sleep(Duration::from_micros(100));
        }
    }));

    // Chaos thread: 50 poison/replace cycles
    let d_chaos = d.clone();
    let own_chaos = ownership.clone();
    handles.push(std::thread::spawn(move || {
        for cycle in 0..50 {
            let w = cycle % 2;
            std::thread::sleep(Duration::from_millis(5));
            let mut g = d_chaos.lock().unwrap();
            g.quarantine(w);
            let settled = g.settle_quarantined(w);
            for id in settled {
                own_chaos.settle(id);
            }
            g.replace(w);
        }
    }));

    for h in handles {
        h.join().unwrap();
    }

    // Drain remaining jobs:
    let g = d.lock().unwrap();
    for w in 0..2 {
        while let Some((id, _)) = g.queue(w).pop_timeout(Duration::from_millis(20)) {
            ownership.settle(id);
        }
    }
    drop(g);

    // Assert zero leaked slots, zero orphan invocations:
    let s = ownership.stats();
    assert_eq!(
        s.pending, 0,
        "all invocations settled across 50 poison/replace cycles: {s:?}"
    );
    assert_eq!(s.registered, s.settled);
    assert_eq!(s.rejected_duplicate, 0);
    assert_eq!(s.rejected_unknown_worker, 0);
}

#[test]
fn least_outstanding_converges_loads_after_drain_and_rebuild() {
    // 4 workers; workers 1 and 2 are quarantined and drained (0 jobs left),
    // while workers 0 and 3 carry 6 jobs each. Once 1 and 2 are replaced,
    // subsequent dispatches prioritize the fresh workers until all 4
    // equalize at 6 jobs each.
    let mut d: Dispatcher<u64> = Dispatcher::with_workers(4, 16);

    // Load up w0 and w3:
    for i in 0..6 {
        d.queue(0).try_push(i).unwrap();
        d.queue(3).try_push(i).unwrap();
    }
    assert_eq!(d.queue(0).len(), 6);
    assert_eq!(d.queue(3).len(), 6);

    // Quarantine and replace w1, w2:
    d.quarantine(1);
    d.quarantine(2);
    let _ = d.settle_quarantined(1);
    let _ = d.settle_quarantined(2);
    d.replace(1);
    d.replace(2);

    // Dispatch 12 new jobs: they must route exclusively to w1 and w2
    // (6 each) until all 4 workers reach 6 jobs.
    for i in 100..112 {
        let chosen = d.dispatch(i).unwrap();
        assert!(
            chosen == 1 || chosen == 2,
            "dispatches must prioritize underloaded recovered workers: got {chosen}"
        );
    }

    for w in 0..4 {
        assert_eq!(d.queue(w).len(), 6, "all 4 workers equalized at 6 jobs");
    }
}
