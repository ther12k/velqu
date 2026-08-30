//! Invocation-to-worker ownership tracking (M3-007-A, ADR-0036 §4).
//!
//! Every admitted invocation is bound to exactly one owning worker for
//! its entire lifetime. The registry is the routing primitive the
//! cancellation and shutdown paths (M3-007-B..D) build on: a cancel or
//! drain signal for invocation `id` goes to the worker that recorded it
//! — never to a guessed engine, never twice.
//!
//! Discipline (ADR-0036 §4, lifecycle/infrastructure shape): plain
//! host-side data behind one mutex, bounded by construction (at most
//! `capacity` live entries — the tracked set can never exceed the
//! dispatcher's total admission bound), saturating counters only, no
//! JS values, no locks held across JS execution. Shared across host
//! threads; the owning worker is an index, not a handle.

use std::collections::HashMap;
use std::sync::Mutex;

use crate::shared_handles::SharedAcrossWorkers;

/// Default live-invocation tracking capacity.
pub const DEFAULT_INVOCATION_TRACKING_CAPACITY: usize = 4_096;

/// Hard ceiling on configured tracking capacity (bounded configuration;
/// matches the dispatcher queue ceiling — tracking never becomes the
/// largest bound in the system).
pub const MAX_INVOCATION_TRACKING_CAPACITY: usize = 65_536;

/// Typed tracking violation. Closed set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrackError {
    /// The registry is at its live-entry bound: the invocation was NOT
    /// tracked; the caller must fail the admission closed (this mirrors
    /// queue exhaustion — bounded memory over silent growth).
    AtCapacity { capacity: usize },
    /// A live entry already records this invocation under `worker`.
    /// Re-tracking a live id is a contract violation (one admission,
    /// one registration).
    AlreadyTracked { worker: usize },
    /// `worker` names no worker in this topology.
    UnknownWorker { worker: usize, workers: usize },
}

impl std::fmt::Display for TrackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TrackError::AtCapacity { capacity } => {
                write!(
                    f,
                    "invocation tracking at capacity ({capacity}); admission rejected"
                )
            }
            TrackError::AlreadyTracked { worker } => {
                write!(f, "invocation already tracked by worker {worker}")
            }
            TrackError::UnknownWorker { worker, workers } => {
                write!(f, "worker {worker} out of range ({} workers)", workers)
            }
        }
    }
}

impl std::error::Error for TrackError {}

struct OwnershipInner {
    /// invocation id -> owning worker. Bounded by `capacity`.
    owners: HashMap<u64, usize>,
    registered: u64,
    settled: u64,
    rejected_at_capacity: u64,
    rejected_duplicate: u64,
    rejected_unknown_worker: u64,
}

/// Bounded registry binding each live invocation to its owning worker
/// (M3-007-A). The registry is the single source of truth for "which
/// worker owns invocation X": admission records the binding exactly
/// once, the terminal transition (response delivered OR cancellation
/// routed) settles it exactly once, and a settled id can never be
/// settled or cancelled again.
pub struct InvocationOwnership {
    workers: usize,
    capacity: usize,
    inner: Mutex<OwnershipInner>,
}

impl SharedAcrossWorkers for InvocationOwnership {}

impl InvocationOwnership {
    /// Registry for a `workers`-worker topology with a live-entry bound
    /// of `capacity` (clamped to `1..=MAX_INVOCATION_TRACKING_CAPACITY`).
    pub fn with_workers(workers: usize, capacity: usize) -> Self {
        assert!(workers >= 1, "a topology needs at least one worker");
        InvocationOwnership {
            workers,
            capacity: capacity.clamp(1, MAX_INVOCATION_TRACKING_CAPACITY),
            inner: Mutex::new(OwnershipInner {
                owners: HashMap::new(),
                registered: 0,
                settled: 0,
                rejected_at_capacity: 0,
                rejected_duplicate: 0,
                rejected_unknown_worker: 0,
            }),
        }
    }

    /// Registry with the default tracking capacity.
    pub fn new(workers: usize) -> Self {
        Self::with_workers(workers, DEFAULT_INVOCATION_TRACKING_CAPACITY)
    }

    /// Worker count of the topology this registry tracks.
    pub fn workers(&self) -> usize {
        self.workers
    }

    /// Live-entry bound.
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Record that invocation `id` is owned by `worker`. Called once at
    /// admission, before the job reaches the worker. Typed rejection —
    /// never a block, never silent growth.
    pub fn track(&self, id: u64, worker: usize) -> Result<(), TrackError> {
        if worker >= self.workers {
            let err = TrackError::UnknownWorker {
                worker,
                workers: self.workers,
            };
            let mut g = self.inner.lock().unwrap();
            g.rejected_unknown_worker = g.rejected_unknown_worker.saturating_add(1);
            return Err(err);
        }
        let mut g = self.inner.lock().unwrap();
        if let Some(&existing) = g.owners.get(&id) {
            g.rejected_duplicate = g.rejected_duplicate.saturating_add(1);
            return Err(TrackError::AlreadyTracked { worker: existing });
        }
        if g.owners.len() >= self.capacity {
            g.rejected_at_capacity = g.rejected_at_capacity.saturating_add(1);
            return Err(TrackError::AtCapacity {
                capacity: self.capacity,
            });
        }
        g.owners.insert(id, worker);
        g.registered = g.registered.saturating_add(1);
        Ok(())
    }

    /// Owning worker of `id`, if live. The cancel-route lookup: a
    /// cancellation for `id` is delivered to exactly this worker.
    pub fn owner_of(&self, id: u64) -> Option<usize> {
        self.inner.lock().unwrap().owners.get(&id).copied()
    }

    /// Perform the terminal transition for `id`: remove the binding and
    /// return its owning worker. `Some(owner)` exactly once — a second
    /// settlement (or a cancel arriving after settlement) observes
    /// `None` and must not re-deliver. This is the exactly-once gate
    /// for cancellation and shutdown reporting.
    pub fn settle(&self, id: u64) -> Option<usize> {
        let mut g = self.inner.lock().unwrap();
        let owner = g.owners.remove(&id)?;
        g.settled = g.settled.saturating_add(1);
        Some(owner)
    }

    /// Live (tracked, unsettled) invocations.
    pub fn pending(&self) -> usize {
        self.inner.lock().unwrap().owners.len()
    }

    /// Live invocations owned by `worker` (drain paths enumerate what
    /// they must settle or abort; M3-007-B..D).
    pub fn pending_of_worker(&self, worker: usize) -> usize {
        self.inner
            .lock()
            .unwrap()
            .owners
            .values()
            .filter(|&&w| w == worker)
            .count()
    }

    /// Ids of live invocations owned by `worker`, ascending (drain
    /// enumeration is deterministic; bounded by `capacity`).
    pub fn invocations_of_worker(&self, worker: usize) -> Vec<u64> {
        let g = self.inner.lock().unwrap();
        let mut ids: Vec<u64> = g
            .owners
            .iter()
            .filter(|(_, &w)| w == worker)
            .map(|(&id, _)| id)
            .collect();
        ids.sort_unstable();
        ids
    }

    /// Every live binding as `(id, worker)`, ascending by id (shutdown
    /// audit enumerates the exact orphan set if any).
    pub fn snapshot(&self) -> Vec<(u64, usize)> {
        let g = self.inner.lock().unwrap();
        let mut all: Vec<(u64, usize)> = g.owners.iter().map(|(&id, &w)| (id, w)).collect();
        all.sort_unstable();
        all
    }

    /// Redacted observability snapshot: counters only, no invocation
    /// ids. `registered - settled == pending` holds between calls.
    pub fn stats(&self) -> InvocationOwnershipStats {
        let g = self.inner.lock().unwrap();
        InvocationOwnershipStats {
            workers: self.workers,
            capacity: self.capacity,
            pending: g.owners.len(),
            registered: g.registered,
            settled: g.settled,
            rejected_at_capacity: g.rejected_at_capacity,
            rejected_duplicate: g.rejected_duplicate,
            rejected_unknown_worker: g.rejected_unknown_worker,
        }
    }
}

/// Redacted ownership counters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvocationOwnershipStats {
    pub workers: usize,
    pub capacity: usize,
    pub pending: usize,
    pub registered: u64,
    pub settled: u64,
    pub rejected_at_capacity: u64,
    pub rejected_duplicate: u64,
    pub rejected_unknown_worker: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn capacity_is_bounded_and_clamped() {
        assert_eq!(
            InvocationOwnership::new(2).capacity(),
            DEFAULT_INVOCATION_TRACKING_CAPACITY
        );
        assert_eq!(InvocationOwnership::with_workers(2, 8).capacity(), 8);
        assert_eq!(
            InvocationOwnership::with_workers(2, MAX_INVOCATION_TRACKING_CAPACITY * 10).capacity(),
            MAX_INVOCATION_TRACKING_CAPACITY
        );
        assert_eq!(InvocationOwnership::with_workers(2, 0).capacity(), 1);
        assert_eq!(InvocationOwnership::new(3).workers(), 3);
    }

    #[test]
    fn track_and_settle_round_trip_records_the_owner() {
        let reg = InvocationOwnership::new(2);
        assert!(reg.track(1, 0).is_ok());
        assert!(reg.track(2, 1).is_ok());
        assert_eq!(reg.owner_of(1), Some(0), "cancel routes to the recorder");
        assert_eq!(reg.owner_of(2), Some(1));
        assert_eq!(reg.pending(), 2);
        // Terminal transition returns the owner exactly once.
        assert_eq!(reg.settle(1), Some(0));
        assert_eq!(reg.owner_of(1), None, "settled id has no owner");
        assert_eq!(reg.settle(1), None, "second settle is a no-op");
        assert_eq!(reg.pending(), 1);
    }

    #[test]
    fn unknown_worker_is_typed_and_counted() {
        let reg = InvocationOwnership::new(2);
        let err = reg.track(1, 2).unwrap_err();
        assert_eq!(
            err,
            TrackError::UnknownWorker {
                worker: 2,
                workers: 2
            }
        );
        assert!(err.to_string().contains("out of range"));
        assert_eq!(reg.owner_of(1), None, "nothing tracked on rejection");
        assert_eq!(reg.stats().rejected_unknown_worker, 1);
    }

    #[test]
    fn duplicate_track_is_typed_and_names_the_existing_owner() {
        let reg = InvocationOwnership::new(2);
        reg.track(7, 1).unwrap();
        let err = reg.track(7, 0).unwrap_err();
        assert_eq!(err, TrackError::AlreadyTracked { worker: 1 });
        assert_eq!(
            reg.owner_of(7),
            Some(1),
            "original binding survives the duplicate attempt"
        );
        let stats = reg.stats();
        assert_eq!(stats.rejected_duplicate, 1);
        assert_eq!(stats.registered, 1, "duplicate never inflates registered");
    }

    #[test]
    fn tracking_is_bounded_and_rejects_closed() {
        let reg = InvocationOwnership::with_workers(1, 2);
        reg.track(1, 0).unwrap();
        reg.track(2, 0).unwrap();
        let err = reg.track(3, 0).unwrap_err();
        assert_eq!(err, TrackError::AtCapacity { capacity: 2 });
        assert!(err.to_string().contains("at capacity"));
        assert_eq!(reg.stats().rejected_at_capacity, 1);
        // Settling frees the bound slot: bounded memory, reusable.
        reg.settle(1).unwrap();
        assert!(reg.track(3, 0).is_ok());
    }

    #[test]
    fn per_worker_enumeration_is_deterministic_and_bounded() {
        let reg = InvocationOwnership::with_workers(3, 16);
        for id in 5u64..8 {
            reg.track(id, 0).unwrap();
        }
        for id in 1u64..4 {
            reg.track(id, 2).unwrap();
        }
        assert_eq!(reg.pending_of_worker(0), 3);
        assert_eq!(reg.pending_of_worker(1), 0);
        assert_eq!(reg.pending_of_worker(2), 3);
        assert_eq!(reg.invocations_of_worker(0), [5, 6, 7], "ascending ids");
        assert_eq!(reg.invocations_of_worker(2), [1, 2, 3]);
        assert_eq!(
            reg.snapshot(),
            [(1, 2), (2, 2), (3, 2), (5, 0), (6, 0), (7, 0)],
            "whole-fleet audit, ascending by id"
        );
    }

    #[test]
    fn stats_balance_registered_settled_pending() {
        let reg = InvocationOwnership::new(2);
        for id in 1u64..=10 {
            reg.track(id, (id % 2) as usize).unwrap();
        }
        for id in 1u64..=6 {
            reg.settle(id).unwrap();
        }
        let s = reg.stats();
        assert_eq!(
            (s.registered, s.settled, s.pending),
            (10, 6, 4),
            "registered - settled == pending"
        );
        assert_eq!(s.workers, 2);
        assert_eq!(s.capacity, DEFAULT_INVOCATION_TRACKING_CAPACITY);
        // Redaction: no invocation ids in the debug output.
        let dbg = format!("{s:?}");
        for id in 7u64..=10 {
            assert!(!dbg.contains(&format!("id{id}")));
        }
    }

    #[test]
    fn settle_is_the_exactly_once_gate_for_cancel_routing() {
        // The M3-007 guard shape: first terminal transition (here the
        // cancel guard) consumes the binding; a racing second transition
        // observes None and never re-cancels.
        let reg = Arc::new(InvocationOwnership::new(2));
        reg.track(42, 1).unwrap();
        let (first, second) = (reg.clone(), reg.clone());
        let h1 = std::thread::spawn(move || first.settle(42));
        let h2 = std::thread::spawn(move || second.settle(42));
        let results = [h1.join().unwrap(), h2.join().unwrap()];
        assert_eq!(
            results.iter().filter(|r| r.is_some()).count(),
            1,
            "exactly one transition wins: {results:?}"
        );
        assert_eq!(results.iter().find_map(|r| *r), Some(1));
        assert_eq!(reg.stats().settled, 1);
        assert_eq!(reg.pending(), 0);
    }

    #[test]
    fn concurrent_admission_and_settlement_stays_consistent() {
        // 4 producer threads × 250 admissions, interleaved settlements
        // from 2 other threads. Invariant under any interleaving:
        // registered - settled == pending, and every id settles once.
        let reg = Arc::new(InvocationOwnership::with_workers(2, 2_000));
        let mut handles = Vec::new();
        for t in 0..4usize {
            let r = reg.clone();
            handles.push(std::thread::spawn(move || {
                for i in 0..250u64 {
                    r.track(t as u64 * 1_000 + i, (t + i as usize) % 2).unwrap();
                }
            }));
        }
        let settler = {
            let r = reg.clone();
            std::thread::spawn(move || {
                let mut settled = 0u64;
                for id in 0..1_000u64 {
                    if r.settle(id).is_some() {
                        settled += 1;
                    }
                }
                settled
            })
        };
        for h in handles {
            h.join().unwrap();
        }
        let settled = settler.join().unwrap();
        let s = reg.stats();
        assert_eq!(s.registered, 1_000);
        assert_eq!(s.settled, settled);
        assert_eq!(
            s.registered - s.settled,
            s.pending as u64,
            "balance invariant holds after the race"
        );
        // Whatever remains is enumerable and settles exactly once each.
        let remaining = reg.snapshot();
        assert_eq!(remaining.len(), s.pending);
        for (id, _) in remaining {
            assert!(reg.settle(id).is_some());
        }
        assert_eq!(reg.pending(), 0);
        assert_eq!(reg.stats().settled, 1_000);
    }

    #[test]
    fn no_orphan_audit_over_a_full_admit_settle_cycle() {
        // Resource invariant: after every admission reaches a terminal
        // transition, the live set is empty — no orphan invocation.
        let reg = InvocationOwnership::with_workers(3, 64);
        for id in 1u64..=200 {
            let worker = (id as usize) % 3;
            reg.track(id, worker).unwrap();
            assert_eq!(reg.settle(id), Some(worker));
        }
        let s = reg.stats();
        assert_eq!((s.registered, s.settled, s.pending), (200, 200, 0));
        assert!(reg.snapshot().is_empty());
    }
}
