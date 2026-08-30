//! Load-shed reason vocabulary (M3-008-C).
//!
//! Every refusal the runtime hands a client is one of a CLOSED set of
//! load-shed reasons. The vocabulary maps each typed rejection from the
//! M3 components (bounded queues, fairness admission, long-running
//! policy, drain gate, invocation tracking) to:
//!
//! - a stable `kind` label (metrics/logs);
//! - a redacted client-facing detail (bounds only — never which worker,
//!   class, or caller; same topology-stays-internal bar as M3-002-C);
//! - the frozen `overload` problem kind with the shared Retry-After
//!   posture (M3-002-C / M3-007-B precedent: one consistent
//!   client-facing verdict for every capacity refusal).
//!
//! Contract violations (`FairnessReject::UnknownClass`,
//! `TrackError::AlreadyTracked/UnknownWorker`) are deliberately NOT
//! load-shed reasons — they are host bugs, exposed as their own typed
//! errors, never laundered into "server is busy".

use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::dispatch::QueueError;
use crate::dispatch::RETRY_AFTER_OVERLOAD_SECS;
use crate::fairness::FairnessReject;
use crate::invocation::TrackError;
use crate::long_running::LongSlotsExhausted;

/// The closed set of load-shed reasons.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadShedReason {
    /// One worker's dispatch queue is at capacity (M3-002-A).
    WorkerQueueFull { worker: usize, capacity: usize },
    /// Every worker queue is at capacity (M3-002-B).
    AllWorkersQueuesFull { workers: usize, capacity: usize },
    /// The global fairness admission pool is exhausted (M3-008-A).
    GlobalAdmissionFull { capacity: usize },
    /// A class reached its fairness ceiling (share + headroom;
    /// M3-008-A) while other classes hold their guaranteed shares.
    ClassFairnessCeiling { class: usize, ceiling: usize },
    /// The long-running slot budget is exhausted (M3-008-B).
    LongRunningSlotsExhausted { limit: usize },
    /// The instance is draining and refuses new admissions (M3-007-B).
    DrainInProgress,
    /// The invocation ownership registry is at its bound (M3-007-A).
    InvocationTrackingFull { capacity: usize },
}

impl LoadShedReason {
    /// Stable label for metrics and structured logs. Closed set.
    pub fn kind(&self) -> &'static str {
        match self {
            LoadShedReason::WorkerQueueFull { .. } => "worker_queue_full",
            LoadShedReason::AllWorkersQueuesFull { .. } => "all_workers_full",
            LoadShedReason::GlobalAdmissionFull { .. } => "global_admission_full",
            LoadShedReason::ClassFairnessCeiling { .. } => "class_ceiling",
            LoadShedReason::LongRunningSlotsExhausted { .. } => "long_running_slots",
            LoadShedReason::DrainInProgress => "draining",
            LoadShedReason::InvocationTrackingFull { .. } => "tracking_full",
        }
    }

    /// Redacted client-facing detail: bounds only, no topology, no
    /// class indices, no caller identity.
    pub fn client_detail(&self) -> &'static str {
        match self {
            LoadShedReason::WorkerQueueFull { .. }
            | LoadShedReason::AllWorkersQueuesFull { .. } => {
                "worker queues at capacity; retry after the advertised delay"
            }
            LoadShedReason::GlobalAdmissionFull { .. } => {
                "admission capacity exhausted; retry after the advertised delay"
            }
            LoadShedReason::ClassFairnessCeiling { .. } => {
                "this workload reached its fairness ceiling; retry after the advertised delay"
            }
            LoadShedReason::LongRunningSlotsExhausted { .. } => {
                "long-running capacity is busy; retry after the advertised delay"
            }
            LoadShedReason::DrainInProgress => "server is draining",
            LoadShedReason::InvocationTrackingFull { .. } => {
                "admission capacity exhausted; retry after the advertised delay"
            }
        }
    }

    /// The frozen problem registry kind this refusal renders as.
    /// Every capacity refusal is the SAME client-facing verdict (503
    /// overload) per M3-002-C; drain reuses it per M3-007-B (the
    /// registry is frozen).
    pub fn problem_kind(&self) -> &'static str {
        "overload"
    }

    /// Shared Retry-After posture (seconds).
    pub fn retry_after_secs(&self) -> u32 {
        RETRY_AFTER_OVERLOAD_SECS
    }
}

/// A refusal is convertible when it is a GENUINE capacity refusal.
/// Contract violations must not become load-shed events.
impl From<QueueError> for LoadShedReason {
    fn from(err: QueueError) -> Self {
        match err {
            QueueError::Full {
                worker, capacity, ..
            } => LoadShedReason::WorkerQueueFull { worker, capacity },
            QueueError::AllFull { workers, capacity } => {
                LoadShedReason::AllWorkersQueuesFull { workers, capacity }
            }
        }
    }
}

impl From<LongSlotsExhausted> for LoadShedReason {
    fn from(err: LongSlotsExhausted) -> Self {
        LoadShedReason::LongRunningSlotsExhausted { limit: err.limit }
    }
}

impl FairnessReject {
    /// The load-shed reason for a genuine capacity refusal; `None` for
    /// contract violations (`UnknownClass` is a host bug, never "busy").
    pub fn load_shed_reason(&self) -> Option<LoadShedReason> {
        match *self {
            FairnessReject::GlobalFull { capacity } => {
                Some(LoadShedReason::GlobalAdmissionFull { capacity })
            }
            FairnessReject::ClassCeiling { class, ceiling } => {
                Some(LoadShedReason::ClassFairnessCeiling { class, ceiling })
            }
            FairnessReject::UnknownClass { .. } => None,
        }
    }
}

impl TrackError {
    /// The load-shed reason for a genuine capacity refusal; `None` for
    /// contract violations (duplicate/unknown-worker tracking).
    pub fn load_shed_reason(&self) -> Option<LoadShedReason> {
        match *self {
            TrackError::AtCapacity { capacity } => {
                Some(LoadShedReason::InvocationTrackingFull { capacity })
            }
            TrackError::AlreadyTracked { .. } | TrackError::UnknownWorker { .. } => None,
        }
    }
}

/// Number of distinct load-shed kinds (the closed set above).
pub const LOAD_SHED_KINDS: usize = 7;

/// Index of a reason in the fixed counter array (stable order).
fn kind_index(reason: &LoadShedReason) -> usize {
    match reason {
        LoadShedReason::WorkerQueueFull { .. } => 0,
        LoadShedReason::AllWorkersQueuesFull { .. } => 1,
        LoadShedReason::GlobalAdmissionFull { .. } => 2,
        LoadShedReason::ClassFairnessCeiling { .. } => 3,
        LoadShedReason::LongRunningSlotsExhausted { .. } => 4,
        LoadShedReason::DrainInProgress => 5,
        LoadShedReason::InvocationTrackingFull { .. } => 6,
    }
}

/// Fixed-size per-reason counters (M3-008-C exposure channel). Shared
/// host state; saturating adds; bounded by the closed reason set —
/// never a growable structure (ADR-0036 §4 metrics discipline).
#[derive(Debug)]
pub struct LoadShedCounters {
    counts: [AtomicU64; LOAD_SHED_KINDS],
}

impl Default for LoadShedCounters {
    fn default() -> Self {
        Self::new()
    }
}

impl LoadShedCounters {
    pub fn new() -> Self {
        LoadShedCounters {
            counts: [
                AtomicU64::new(0),
                AtomicU64::new(0),
                AtomicU64::new(0),
                AtomicU64::new(0),
                AtomicU64::new(0),
                AtomicU64::new(0),
                AtomicU64::new(0),
            ],
        }
    }

    /// Record one load-shed event (saturating).
    pub fn record(&self, reason: &LoadShedReason) {
        self.counts[kind_index(reason)]
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |v| v.checked_add(1))
            .ok();
    }

    /// Count for one reason.
    pub fn count(&self, reason: &LoadShedReason) -> u64 {
        self.counts[kind_index(reason)].load(Ordering::Relaxed)
    }

    /// Every kind with its count, kind-sorted (deterministic report).
    pub fn snapshot(&self) -> BTreeMap<&'static str, u64> {
        let reason_at = |i: usize| match i {
            0 => LoadShedReason::WorkerQueueFull {
                worker: 0,
                capacity: 0,
            },
            1 => LoadShedReason::AllWorkersQueuesFull {
                workers: 0,
                capacity: 0,
            },
            2 => LoadShedReason::GlobalAdmissionFull { capacity: 0 },
            3 => LoadShedReason::ClassFairnessCeiling {
                class: 0,
                ceiling: 0,
            },
            4 => LoadShedReason::LongRunningSlotsExhausted { limit: 0 },
            5 => LoadShedReason::DrainInProgress,
            _ => LoadShedReason::InvocationTrackingFull { capacity: 0 },
        };
        let mut map = BTreeMap::new();
        for i in 0..LOAD_SHED_KINDS {
            map.insert(reason_at(i).kind(), self.counts[i].load(Ordering::Relaxed));
        }
        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kinds_are_stable_deterministic_and_complete() {
        let all = [
            LoadShedReason::WorkerQueueFull {
                worker: 3,
                capacity: 256,
            },
            LoadShedReason::AllWorkersQueuesFull {
                workers: 4,
                capacity: 256,
            },
            LoadShedReason::GlobalAdmissionFull { capacity: 100 },
            LoadShedReason::ClassFairnessCeiling {
                class: 1,
                ceiling: 8,
            },
            LoadShedReason::LongRunningSlotsExhausted { limit: 2 },
            LoadShedReason::DrainInProgress,
            LoadShedReason::InvocationTrackingFull { capacity: 4_096 },
        ];
        for r in &all {
            let k = r.kind();
            assert!(!k.is_empty());
            // Deterministic across instances of the same variant.
            let again = match r {
                LoadShedReason::WorkerQueueFull { .. } => LoadShedReason::WorkerQueueFull {
                    worker: 9,
                    capacity: 1,
                },
                _ => continue,
            };
            assert_eq!(again.kind(), k);
        }
        // All seven kinds are distinct labels.
        let mut labels: Vec<&str> = all.iter().map(|r| r.kind()).collect();
        labels.sort_unstable();
        labels.dedup();
        assert_eq!(labels.len(), LOAD_SHED_KINDS);
        // Snapshot covers exactly the closed set.
        let counters = LoadShedCounters::new();
        assert_eq!(counters.snapshot().len(), LOAD_SHED_KINDS);
    }

    #[test]
    fn client_detail_is_redacted_and_verdict_is_frozen() {
        let topology_leaking = [
            LoadShedReason::WorkerQueueFull {
                worker: 3,
                capacity: 256,
            },
            LoadShedReason::ClassFairnessCeiling {
                class: 1,
                ceiling: 8,
            },
        ];
        for r in &topology_leaking {
            let d = r.client_detail();
            assert!(!d.contains('3'), "no worker index in client detail: {d}");
            assert!(!d.contains("worker 3") && !d.contains("class 1"));
            assert_eq!(r.problem_kind(), "overload");
            assert_eq!(r.retry_after_secs(), RETRY_AFTER_OVERLOAD_SECS);
        }
        // Every reason renders the frozen overload verdict.
        let all = [
            LoadShedReason::WorkerQueueFull {
                worker: 0,
                capacity: 1,
            },
            LoadShedReason::AllWorkersQueuesFull {
                workers: 1,
                capacity: 1,
            },
            LoadShedReason::GlobalAdmissionFull { capacity: 1 },
            LoadShedReason::ClassFairnessCeiling {
                class: 0,
                ceiling: 1,
            },
            LoadShedReason::LongRunningSlotsExhausted { limit: 1 },
            LoadShedReason::DrainInProgress,
            LoadShedReason::InvocationTrackingFull { capacity: 1 },
        ];
        for r in &all {
            assert_eq!(r.problem_kind(), "overload");
            assert_eq!(r.retry_after_secs(), 1);
        }
    }

    #[test]
    fn conversions_cover_the_component_rejections() {
        let q = QueueError::Full {
            worker: 2,
            len: 8,
            capacity: 8,
        };
        let r: LoadShedReason = q.into();
        assert_eq!(r.kind(), "worker_queue_full");
        let q = QueueError::AllFull {
            workers: 3,
            capacity: 8,
        };
        let r: LoadShedReason = q.into();
        assert_eq!(r.kind(), "all_workers_full");

        let r = FairnessReject::GlobalFull { capacity: 100 }
            .load_shed_reason()
            .unwrap();
        assert_eq!(r.kind(), "global_admission_full");
        let r = FairnessReject::ClassCeiling {
            class: 1,
            ceiling: 8,
        }
        .load_shed_reason()
        .unwrap();
        assert_eq!(r.kind(), "class_ceiling");
        // Contract violations are NOT load-shed.
        assert!(FairnessReject::UnknownClass {
            class: 9,
            classes: 2
        }
        .load_shed_reason()
        .is_none());

        let r: LoadShedReason = LongSlotsExhausted { limit: 2 }.into();
        assert_eq!(r.kind(), "long_running_slots");

        let r = TrackError::AtCapacity { capacity: 4_096 }
            .load_shed_reason()
            .unwrap();
        assert_eq!(r.kind(), "tracking_full");
        // Contract violations are NOT load-shed.
        assert!(TrackError::AlreadyTracked { worker: 0 }
            .load_shed_reason()
            .is_none());
        assert!(TrackError::UnknownWorker {
            worker: 5,
            workers: 2
        }
        .load_shed_reason()
        .is_none());
    }

    #[test]
    fn counters_record_saturate_and_snapshot_deterministically() {
        let c = LoadShedCounters::new();
        c.record(&LoadShedReason::DrainInProgress);
        c.record(&LoadShedReason::DrainInProgress);
        c.record(&LoadShedReason::WorkerQueueFull {
            worker: 0,
            capacity: 1,
        });
        assert_eq!(c.count(&LoadShedReason::DrainInProgress), 2);
        assert_eq!(
            c.count(&LoadShedReason::WorkerQueueFull {
                worker: 9,
                capacity: 9
            }),
            1
        );
        assert_eq!(
            c.count(&LoadShedReason::GlobalAdmissionFull { capacity: 0 }),
            0
        );
        // Saturating.
        let c = LoadShedCounters::new();
        c.counts[5].store(u64::MAX, Ordering::Relaxed);
        c.record(&LoadShedReason::DrainInProgress);
        assert_eq!(c.count(&LoadShedReason::DrainInProgress), u64::MAX);
        // Snapshot is kind-sorted and complete.
        let c = LoadShedCounters::new();
        c.record(&LoadShedReason::InvocationTrackingFull { capacity: 8 });
        let snap = c.snapshot();
        assert_eq!(snap.len(), LOAD_SHED_KINDS);
        assert_eq!(snap["tracking_full"], 1);
        assert_eq!(snap["draining"], 0);
        let keys: Vec<&str> = snap.keys().copied().collect();
        let mut sorted = keys.clone();
        sorted.sort_unstable();
        assert_eq!(keys, sorted, "BTreeMap: deterministic order");
    }
}
