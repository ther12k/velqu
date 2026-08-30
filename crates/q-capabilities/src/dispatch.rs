//! Bounded per-worker dispatch queues (M3-002-A, ADR-0036 §4).
//!
//! One queue per worker, bounded by construction. The dispatcher pushes
//! plain-data jobs (never JS values — ADR-0036 §5, enforced by the
//! `Send` bound); the owning worker thread pops. Overflow is a typed,
//! immediate rejection — never a block, never unbounded growth. Queue
//! wait is measured per item so per-worker queue latency is observable
//! without logging secrets.

use std::collections::VecDeque;
use std::sync::{Condvar, Mutex};
use std::time::{Duration, Instant};

use crate::shared_handles::SharedAcrossWorkers;

/// Default per-worker queue capacity.
pub const DEFAULT_WORKER_QUEUE_CAPACITY: usize = 256;

/// Hard ceiling on configured capacity (bounded configuration).
pub const MAX_WORKER_QUEUE_CAPACITY: usize = 65_536;

/// Typed dispatch-queue violations. Closed set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueueError {
    /// The queue is at capacity: the push was rejected immediately.
    /// Overload fails fast and observably (M3-002 guardrail).
    Full {
        worker: usize,
        len: usize,
        capacity: usize,
    },
    /// Every worker queue is at capacity: admission rejected globally.
    /// (M3-002-B selection verdict; M3-002-C formalizes the response.)
    AllFull { workers: usize, capacity: usize },
}

impl std::fmt::Display for QueueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QueueError::Full {
                worker,
                len,
                capacity,
            } => write!(
                f,
                "worker {worker} queue full ({len}/{capacity}); admission rejected"
            ),
            QueueError::AllFull { workers, capacity } => write!(
                f,
                "all {workers} worker queues full (capacity {capacity} each); admission rejected"
            ),
        }
    }
}

impl std::error::Error for QueueError {}

struct Inner<T> {
    items: VecDeque<(T, Instant)>,
    capacity: usize,
    closed: bool,
    pushed: u64,
    popped: u64,
    rejected: u64,
    /// Sum of queue-wait durations of popped items (for mean wait).
    wait_total: Duration,
    /// Max queue-wait observed.
    wait_max: Duration,
}

/// A bounded FIFO dispatch queue owned by one worker thread (M3-002-A).
///
/// The sharing discipline (ADR-0036 §4, queue shape): producers may be any
/// host thread; the consumer is exactly one worker. `T` must be `Send`
/// plain data — the type system keeps JS values out.
pub struct BoundedWorkerQueue<T> {
    worker: usize,
    inner: Mutex<Inner<T>>,
    not_empty: Condvar,
}

impl<T: Send + 'static> SharedAcrossWorkers for BoundedWorkerQueue<T> {}

impl<T: Send> BoundedWorkerQueue<T> {
    /// Create the queue for `worker` with an explicit capacity.
    /// Capacity is clamped to [`MAX_WORKER_QUEUE_CAPACITY`] and must be
    /// at least 1.
    pub fn with_capacity(worker: usize, capacity: usize) -> Self {
        let capacity = capacity.clamp(1, MAX_WORKER_QUEUE_CAPACITY);
        BoundedWorkerQueue {
            worker,
            inner: Mutex::new(Inner {
                items: VecDeque::with_capacity(capacity.min(1024)),
                capacity,
                closed: false,
                pushed: 0,
                popped: 0,
                rejected: 0,
                wait_total: Duration::ZERO,
                wait_max: Duration::ZERO,
            }),
            not_empty: Condvar::new(),
        }
    }

    /// Create with the default capacity.
    pub fn new(worker: usize) -> Self {
        Self::with_capacity(worker, DEFAULT_WORKER_QUEUE_CAPACITY)
    }

    /// Owning worker index.
    pub fn worker(&self) -> usize {
        self.worker
    }

    /// Configured capacity.
    pub fn capacity(&self) -> usize {
        self.inner.lock().unwrap().capacity
    }

    /// Items currently queued.
    pub fn len(&self) -> usize {
        self.inner.lock().unwrap().items.len()
    }

    /// True when no items are queued.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Try to enqueue `job`. Returns `Err(QueueError::Full)` IMMEDIATELY
    /// when at capacity — overload fails fast, never blocks the producer.
    pub fn try_push(&self, job: T) -> Result<(), QueueError> {
        let mut g = self.inner.lock().unwrap();
        if g.items.len() >= g.capacity {
            g.rejected = g.rejected.saturating_add(1);
            return Err(QueueError::Full {
                worker: self.worker,
                len: g.items.len(),
                capacity: g.capacity,
            });
        }
        g.items.push_back((job, Instant::now()));
        g.pushed = g.pushed.saturating_add(1);
        drop(g);
        self.not_empty.notify_one();
        Ok(())
    }

    /// Blocking pop for the owning worker thread. Returns `None` once the
    /// queue is closed and drained. Wait is measured per item.
    pub fn pop(&self) -> Option<(T, Duration)> {
        let mut g = self.inner.lock().unwrap();
        loop {
            if let Some((job, pushed_at)) = g.items.pop_front() {
                let wait = pushed_at.elapsed();
                g.popped = g.popped.saturating_add(1);
                g.wait_total = g.wait_total.saturating_add(wait);
                if wait > g.wait_max {
                    g.wait_max = wait;
                }
                return Some((job, wait));
            }
            if g.closed {
                return None;
            }
            g = self.not_empty.wait(g).unwrap();
        }
    }

    /// Pop with a bounded wait; `None` on timeout or closed+drained.
    pub fn pop_timeout(&self, timeout: Duration) -> Option<(T, Duration)> {
        let mut g = self.inner.lock().unwrap();
        let deadline = Instant::now() + timeout;
        loop {
            if let Some((job, pushed_at)) = g.items.pop_front() {
                let wait = pushed_at.elapsed();
                g.popped = g.popped.saturating_add(1);
                g.wait_total = g.wait_total.saturating_add(wait);
                if wait > g.wait_max {
                    g.wait_max = wait;
                }
                return Some((job, wait));
            }
            if g.closed {
                return None;
            }
            let now = Instant::now();
            if now >= deadline {
                return None;
            }
            let (guard, _timed_out) = self.not_empty.wait_timeout(g, deadline - now).unwrap();
            g = guard;
        }
    }

    /// Close the queue: producers see no change (pushes still bounded),
    /// the consumer drains remaining items then receives `None`.
    /// Idempotent.
    pub fn close(&self) {
        let mut g = self.inner.lock().unwrap();
        g.closed = true;
        drop(g);
        self.not_empty.notify_all();
    }

    /// True once closed.
    pub fn is_closed(&self) -> bool {
        self.inner.lock().unwrap().closed
    }

    /// Bounded observability snapshot (M3-002 guardrail: overload is
    /// observable; queue latency measured). All counters saturate.
    pub fn stats(&self) -> QueueStats {
        let g = self.inner.lock().unwrap();
        QueueStats {
            worker: self.worker,
            len: g.items.len(),
            capacity: g.capacity,
            pushed: g.pushed,
            popped: g.popped,
            rejected: g.rejected,
            mean_wait: if g.popped > 0 {
                g.wait_total / g.popped as u32
            } else {
                Duration::ZERO
            },
            max_wait: g.wait_max,
        }
    }
}

/// Redacted queue observability snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueueStats {
    pub worker: usize,
    pub len: usize,
    pub capacity: usize,
    pub pushed: u64,
    pub popped: u64,
    pub rejected: u64,
    pub mean_wait: Duration,
    pub max_wait: Duration,
}

/// Least-outstanding-load worker selection over N bounded per-worker
/// queues (M3-002-B). Selection is pure host-side state: queue lengths
/// only, no JS values, no locks held across pushes.
///
/// Strategy: pick the worker with the SMALLEST queue length that still
/// has capacity. Ties break round-robin (the cursor advances per
/// selection) so equal load spreads evenly instead of pinning worker 0.
pub struct Dispatcher<T> {
    queues: Vec<BoundedWorkerQueue<T>>,
    /// Round-robin cursor for tie-breaking among least-loaded workers.
    cursor: std::sync::atomic::AtomicUsize,
}

impl<T: Send + 'static> Dispatcher<T> {
    /// N workers, each with the given queue capacity.
    pub fn with_workers(workers: usize, capacity: usize) -> Self {
        assert!(workers >= 1, "a dispatcher needs at least one worker");
        Dispatcher {
            queues: (0..workers)
                .map(|w| BoundedWorkerQueue::with_capacity(w, capacity))
                .collect(),
            cursor: std::sync::atomic::AtomicUsize::new(0),
        }
    }

    /// Worker count.
    pub fn workers(&self) -> usize {
        self.queues.len()
    }

    /// Queue for `worker` (the owning thread pops from it).
    pub fn queue(&self, worker: usize) -> &BoundedWorkerQueue<T> {
        &self.queues[worker]
    }

    /// Select the worker with the least outstanding load that has
    /// capacity. `None` when EVERY queue is full.
    pub fn select(&self) -> Option<usize> {
        let n = self.queues.len();
        let start = self.cursor.load(std::sync::atomic::Ordering::Relaxed) % n;
        let mut best: Option<usize> = None;
        let mut best_len = usize::MAX;
        for i in 0..n {
            // Visit in rotation order starting at the cursor so equal
            // loads win round-robin.
            let idx = (start + i) % n;
            let q = &self.queues[idx];
            let len = q.len();
            if len >= q.capacity() {
                continue; // full: not a candidate
            }
            if len < best_len {
                best = Some(idx);
                best_len = len;
            }
        }
        if let Some(chosen) = best {
            self.cursor
                .store((chosen + 1) % n, std::sync::atomic::Ordering::Relaxed);
        }
        best
    }

    /// Select the least-loaded worker with capacity and push `job` there.
    /// Typed `AllFull` when every queue is at capacity.
    pub fn dispatch(&self, job: T) -> Result<usize, QueueError> {
        match self.select() {
            Some(worker) => self.queues[worker]
                .try_push(job)
                .map(|()| worker)
                .map_err(|_| QueueError::AllFull {
                    workers: self.queues.len(),
                    capacity: self.queues[worker].capacity(),
                }),
            None => Err(QueueError::AllFull {
                workers: self.queues.len(),
                capacity: self.queues[0].capacity(),
            }),
        }
    }

    /// Close every queue (shutdown path).
    pub fn close_all(&self) {
        for q in &self.queues {
            q.close();
        }
    }

    /// Aggregated per-worker stats.
    pub fn stats(&self) -> Vec<QueueStats> {
        self.queues.iter().map(|q| q.stats()).collect()
    }
}

impl<T: Send + 'static> SharedAcrossWorkers for Dispatcher<T> {}

/// Retry-After (seconds) advertised on dispatch overload. Matches the
/// runtime's existing quarantine retry-after posture (M2.2.1-r4.2.1) so
/// clients see one consistent backoff hint.
pub const RETRY_AFTER_OVERLOAD_SECS: u32 = 1;

/// The typed admission verdict the HTTP layer renders when dispatch
/// refuses work (M3-002-C). Redacted by construction: it names the
/// congestion reason and bounds, never the job or the client.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmissionDecision {
    /// HTTP status for the rejection (503 for every overload class —
    /// retryable server-side congestion, never a client error).
    pub status: u16,
    /// Problem identifier the runtime registry resolves
    /// (`overload` -> RFC 9457 `.../problems/overload`, 503).
    pub problem: &'static str,
    /// Retry-After hint in seconds.
    pub retry_after_secs: u32,
    /// Redacted human-readable detail (queue bounds, no payloads).
    pub detail: &'static str,
}

/// Map a dispatch rejection to its admission response (M3-002-C).
/// Deterministic and total: every `QueueError` variant has exactly one
/// response. Both per-worker `Full` and global `AllFull` are the SAME
/// client-facing verdict — 503 overload with a backoff hint — because
/// which worker was full is scheduler topology, not the client's
/// business (and per-worker detail would leak it).
pub fn admission_response(err: &QueueError) -> AdmissionDecision {
    match err {
        QueueError::Full { .. } | QueueError::AllFull { .. } => AdmissionDecision {
            status: 503,
            problem: "overload",
            retry_after_secs: RETRY_AFTER_OVERLOAD_SECS,
            detail: "worker queues at capacity; retry after the advertised delay",
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn capacity_is_bounded_and_clamped() {
        assert_eq!(
            BoundedWorkerQueue::<u8>::new(0).capacity(),
            DEFAULT_WORKER_QUEUE_CAPACITY
        );
        assert_eq!(BoundedWorkerQueue::<u8>::with_capacity(3, 8).capacity(), 8);
        // Over-ceiling configs clamp; zero floors to 1.
        assert_eq!(
            BoundedWorkerQueue::<u8>::with_capacity(3, MAX_WORKER_QUEUE_CAPACITY * 10).capacity(),
            MAX_WORKER_QUEUE_CAPACITY
        );
        assert_eq!(BoundedWorkerQueue::<u8>::with_capacity(3, 0).capacity(), 1);
    }

    #[test]
    fn overflow_fails_fast_with_typed_error_and_counts() {
        let q = BoundedWorkerQueue::with_capacity(7, 2);
        assert!(q.try_push(1).is_ok());
        assert!(q.try_push(2).is_ok());
        let err = q.try_push(3).unwrap_err();
        assert_eq!(
            err,
            QueueError::Full {
                worker: 7,
                len: 2,
                capacity: 2
            }
        );
        assert!(err.to_string().contains("admission rejected"));
        // Rejection is immediate (no blocking) and counted.
        let t0 = Instant::now();
        assert!(q.try_push(4).is_err());
        assert!(t0.elapsed() < Duration::from_millis(5), "fail fast");
        let stats = q.stats();
        assert_eq!(stats.rejected, 2);
        assert_eq!(stats.pushed, 2);
    }

    #[test]
    fn fifo_order_and_wait_measurement() {
        let q = BoundedWorkerQueue::with_capacity(0, 4);
        // Push after a small delay so wait > 0 for the first item.
        q.try_push(10).unwrap();
        std::thread::sleep(Duration::from_millis(15));
        q.try_push(20).unwrap();
        let (a, wait_a) = q.pop_timeout(Duration::from_secs(1)).unwrap();
        let (b, _) = q.pop_timeout(Duration::from_secs(1)).unwrap();
        assert_eq!((a, b), (10, 20), "FIFO order preserved");
        assert!(
            wait_a >= Duration::from_millis(10),
            "wait measured: {wait_a:?}"
        );
        let stats = q.stats();
        assert_eq!(stats.popped, 2);
        assert!(stats.max_wait >= Duration::from_millis(10));
    }

    #[test]
    fn no_head_of_line_lock_across_workers() {
        // Two workers, two queues. Worker A's queue is FULL (pops blocked
        // behind a full queue); worker B's queue must still flow. Per-worker
        // queues mean no cross-worker HOL lock.
        let qa = Arc::new(BoundedWorkerQueue::with_capacity(0, 1));
        let qb = Arc::new(BoundedWorkerQueue::with_capacity(1, 1));
        qa.try_push("a1").unwrap();
        assert!(qa.try_push("a2").is_err(), "A full");
        // B flows freely while A is jammed.
        assert!(qb.try_push("b1").is_ok());
        let (job, _) = qb.pop_timeout(Duration::from_secs(1)).unwrap();
        assert_eq!(job, "b1");
        assert_eq!(qa.len(), 1);
    }

    #[test]
    fn closed_queue_drains_then_returns_none() {
        let q = BoundedWorkerQueue::with_capacity(2, 4);
        q.try_push(1).unwrap();
        q.close();
        assert!(q.is_closed());
        q.close(); // idempotent
                   // Remaining items drain...
        let (job, _) = q.pop_timeout(Duration::from_millis(50)).unwrap();
        assert_eq!(job, 1);
        // ...then None, promptly.
        let t0 = Instant::now();
        assert!(q.pop_timeout(Duration::from_secs(1)).is_none());
        assert!(t0.elapsed() < Duration::from_millis(50));
    }

    #[test]
    fn blocking_pop_wakes_on_push_and_close() {
        let q: Arc<BoundedWorkerQueue<u32>> = Arc::new(BoundedWorkerQueue::new(5));
        let qc = q.clone();
        let h = std::thread::spawn(move || {
            // Blocks until an item arrives.
            match qc.pop() {
                Some((v, _)) => v,
                None => panic!("expected an item before close"),
            }
        });
        std::thread::sleep(Duration::from_millis(20));
        q.try_push(42).unwrap();
        assert_eq!(h.join().unwrap(), 42);

        // And wakes with None on close.
        let qc2 = q.clone();
        let h2 = std::thread::spawn(move || qc2.pop());
        std::thread::sleep(Duration::from_millis(20));
        q.close();
        assert!(h2.join().unwrap().is_none());
    }

    #[test]
    fn stats_are_redacted_and_complete() {
        let q = BoundedWorkerQueue::with_capacity(3, 2);
        q.try_push(1).unwrap();
        let s = q.stats();
        assert_eq!(
            (s.worker, s.len, s.capacity, s.pushed, s.popped, s.rejected),
            (3, 1, 2, 1, 0, 0)
        );
        // Debug output carries counters only — no job payloads.
        let dbg = format!("{s:?}");
        assert!(dbg.contains("rejected") && !dbg.contains("job"));
    }

    #[test]
    fn overload_burst_is_rejected_fast_and_fully_counted() {
        // Overload load test (micro scale): 10k pushes against capacity 128
        // — exactly 128 accepted, the rest rejected immediately, total time
        // bounded (no per-push blocking).
        let q = BoundedWorkerQueue::with_capacity(0, 128);
        let t0 = Instant::now();
        let mut rejected = 0u32;
        for i in 0..10_000u32 {
            if q.try_push(i).is_err() {
                rejected += 1;
            }
        }
        let elapsed = t0.elapsed();
        assert_eq!(q.len(), 128);
        assert_eq!(rejected, 10_000 - 128);
        assert_eq!(q.stats().rejected as u32, rejected);
        assert!(
            elapsed < Duration::from_secs(2),
            "10k try_push must be fast: {elapsed:?}"
        );
    }

    #[test]
    fn selection_targets_least_outstanding_load() {
        let d: Dispatcher<u32> = Dispatcher::with_workers(3, 8);
        // Uneven fill: w0=3, w1=0, w2=1.
        for _ in 0..3 {
            d.queue(0).try_push(0).unwrap();
        }
        d.queue(2).try_push(0).unwrap();
        // dispatch() routes to the least-loaded worker, load-aware each
        // time: w1 (0), then the w1/w2 tie rotates to w2, then w1 again.
        let picks: Vec<usize> = (0..3).map(|_| d.dispatch(0).unwrap()).collect();
        assert_eq!(picks, vec![1, 2, 1]);
        // Loads converge: [3,2,2] — spread of 1 after three dispatches.
        let loads: Vec<usize> = (0..3).map(|w| d.queue(w).len()).collect();
        assert_eq!(loads, vec![3, 2, 2]);
    }

    #[test]
    fn equal_loads_break_round_robin() {
        let d: Dispatcher<u32> = Dispatcher::with_workers(3, 8);
        // All empty: consecutive selections must rotate 0,1,2,0,...
        let picks: Vec<usize> = (0..6).map(|_| d.select().unwrap()).collect();
        assert_eq!(picks, vec![0, 1, 2, 0, 1, 2]);
    }

    #[test]
    fn full_queues_are_skipped_until_only_choice() {
        let d: Dispatcher<u32> = Dispatcher::with_workers(2, 1);
        assert!(d.queue(0).try_push(1).is_ok()); // w0 full
                                                 // Selection skips full w0 and targets w1.
        assert_eq!(d.select().unwrap(), 1);
        assert!(d.queue(1).try_push(2).is_ok()); // both full now
        assert!(d.select().is_none(), "all full -> None");
    }

    #[test]
    fn dispatch_routes_to_least_loaded_and_reports_all_full() {
        let d: Dispatcher<u32> = Dispatcher::with_workers(2, 1);
        assert_eq!(d.dispatch(10).unwrap(), 0);
        assert_eq!(d.dispatch(20).unwrap(), 1);
        let err = d.dispatch(30).unwrap_err();
        assert_eq!(
            err,
            QueueError::AllFull {
                workers: 2,
                capacity: 1
            }
        );
        assert!(err.to_string().contains("all 2 worker queues full"));
        // Items landed on the right queues.
        assert_eq!(
            d.queue(0).pop_timeout(Duration::from_millis(50)).unwrap().0,
            10
        );
        assert_eq!(
            d.queue(1).pop_timeout(Duration::from_millis(50)).unwrap().0,
            20
        );
    }

    #[test]
    fn aggregated_stats_cover_every_worker() {
        let d: Dispatcher<u32> = Dispatcher::with_workers(3, 4);
        d.dispatch(1).unwrap();
        d.dispatch(2).unwrap();
        let stats = d.stats();
        assert_eq!(stats.len(), 3);
        let total_pushed: u64 = stats.iter().map(|s| s.pushed).sum();
        assert_eq!(total_pushed, 2);
        assert_eq!(stats[0].worker, 0);
    }

    #[test]
    fn close_all_shuts_every_queue_down() {
        let d: Dispatcher<u32> = Dispatcher::with_workers(3, 4);
        d.close_all();
        for w in 0..3 {
            assert!(d.queue(w).is_closed());
        }
    }

    #[test]
    fn admission_response_is_total_deterministic_and_redacted() {
        let full = QueueError::Full {
            worker: 3,
            len: 256,
            capacity: 256,
        };
        let all = QueueError::AllFull {
            workers: 4,
            capacity: 256,
        };
        for err in [&full, &all] {
            let d = admission_response(err);
            assert_eq!(
                d.status, 503,
                "overload is a server-side retryable condition"
            );
            assert_eq!(
                d.problem, "overload",
                "matches the runtime problem registry kind"
            );
            assert_eq!(d.retry_after_secs, RETRY_AFTER_OVERLOAD_SECS);
            // Redaction: no worker index, no queue internals in the detail.
            assert!(!d.detail.contains('3') || d.detail.contains("capacity"));
            assert!(!d.detail.contains("worker 3"));
        }
        // Determinism: same input, same verdict.
        assert_eq!(admission_response(&full), admission_response(&full));
        // Both classes render the SAME client-facing verdict (topology
        // stays internal).
        assert_eq!(admission_response(&full), admission_response(&all));
    }

    #[test]
    fn admission_verdict_composes_with_dispatcher_overload() {
        // End-to-end policy shape: a saturated dispatcher produces
        // AllFull, which maps to the 503/overload/retry-1 verdict the
        // HTTP layer renders as an RFC 9457 problem.
        let d: Dispatcher<u32> = Dispatcher::with_workers(2, 1);
        assert!(d.dispatch(1).is_ok());
        assert!(d.dispatch(2).is_ok());
        let err = d.dispatch(3).unwrap_err();
        let verdict = admission_response(&err);
        assert_eq!(
            (verdict.status, verdict.problem, verdict.retry_after_secs),
            (503, "overload", 1)
        );
    }
}
