//! Weighted fairness admission (M3-008-A, ADR-0036 §4).
//!
//! Prevents one route/tenant/slow workload from monopolizing workers:
//! classes (route groups) get WEIGHTED GUARANTEED SHARES of a bounded
//! global capacity, plus a shared borrow pool (headroom) any class may
//! use while it lasts — but never beyond a per-class ceiling, and never
//! beyond the global bound.
//!
//! The construction is validated fail-closed: `soft_total <= capacity`
//! is the load-bearing invariant. From it follow the provable
//! properties the tests pin:
//!
//! - **P1 — weighted shares**: `soft[c]` is proportional to `weight[c]`
//!   (at least 1), and every class's share is admissible whenever the
//!   global pool has room.
//! - **P2 — borrow pool**: `headroom = capacity - soft_total` is shared;
//!   a class may burst to `soft[c] + headroom` (its ceiling) but never
//!   further, even when the rest of the fleet is idle.
//! - **P3 — fleet protection**: when a below-share class is denied, the
//!   global pool is exhausted — and the other classes then COLLECTIVELY
//!   hold more than their combined guaranteed shares. No single class
//!   can push the rest of the fleet below its guarantee; denial below
//!   share happens only under true collective saturation.
//! - **P4 — global bound**: total outstanding never exceeds `capacity`.
//!
//! Discipline (ADR-0036 §4): plain host-side state behind one mutex,
//! bounded by construction (at most `capacity` live admissions across
//! all classes), saturating counters, no JS values. Shared across host
//! threads; admission/release are O(classes) worst case.

use std::sync::Mutex;

use crate::shared_handles::SharedAcrossWorkers;

/// Maximum number of admission classes (route groups).
pub const MAX_FAIR_CLASSES: usize = 256;

/// Maximum global capacity (bounded configuration; matches the
/// dispatcher/tracking ceilings — fairness never becomes the largest
/// bound in the system).
pub const MAX_FAIR_CAPACITY: usize = 65_536;

/// Typed construction violation. Closed set; fail-closed (ADR-0036
/// posture: an invalid fairness config must never silently become "no
/// fairness").
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FairnessError {
    /// At least one class is required.
    EmptyClasses,
    /// Weights must be >= 1.
    NonPositiveWeight { class: usize, weight: u64 },
    /// At most [`MAX_FAIR_CLASSES`] classes.
    TooManyClasses { count: usize, max: usize },
    /// Capacity above [`MAX_FAIR_CAPACITY`].
    CapacityAboveMax { capacity: usize, max: usize },
    /// The weighted shares must fit inside the capacity
    /// (`soft_total <= capacity`) — otherwise the guarantees P1..P3
    /// are vacuous and the config is rejected.
    SharesExceedCapacity { soft_total: usize, capacity: usize },
}

impl std::fmt::Display for FairnessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FairnessError::EmptyClasses => write!(f, "fairness needs at least one class"),
            FairnessError::NonPositiveWeight { class, weight } => {
                write!(f, "class {class} weight {weight} must be >= 1")
            }
            FairnessError::TooManyClasses { count, max } => {
                write!(f, "{count} classes exceed the maximum of {max}")
            }
            FairnessError::CapacityAboveMax { capacity, max } => {
                write!(f, "capacity {capacity} exceeds the maximum of {max}")
            }
            FairnessError::SharesExceedCapacity {
                soft_total,
                capacity,
            } => write!(
                f,
                "weighted shares total {soft_total} exceed capacity {capacity}"
            ),
        }
    }
}

impl std::error::Error for FairnessError {}

/// Typed admission rejection. Redacted: it names bounds, not callers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FairnessReject {
    /// The whole capacity is outstanding.
    GlobalFull { capacity: usize },
    /// The class reached its ceiling (share + headroom); further
    /// admission for it is refused even though other classes may still
    /// have room.
    ClassCeiling { class: usize, ceiling: usize },
    /// `class` names no configured class.
    UnknownClass { class: usize, classes: usize },
}

impl std::fmt::Display for FairnessReject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FairnessReject::GlobalFull { capacity } => {
                write!(f, "global admission capacity {capacity} exhausted")
            }
            FairnessReject::ClassCeiling { class, ceiling } => {
                write!(f, "class {class} at its fairness ceiling ({ceiling})")
            }
            FairnessReject::UnknownClass { class, classes } => {
                write!(f, "class {class} out of range ({classes} classes)")
            }
        }
    }
}

impl std::error::Error for FairnessReject {}

#[derive(Debug)]
struct FairnessInner {
    outstanding: Vec<usize>,
    total_outstanding: usize,
    admitted: u64,
    rejected_global: u64,
    rejected_ceiling: u64,
    unknown_class: u64,
    over_releases: u64,
}

/// Weighted per-class admission controller (M3-008-A).
#[derive(Debug)]
pub struct FairAdmission {
    capacity: usize,
    soft: Vec<usize>,
    ceiling: Vec<usize>,
    inner: Mutex<FairnessInner>,
}

impl SharedAcrossWorkers for FairAdmission {}

impl FairAdmission {
    /// Build a controller for `weights.len()` classes over a global
    /// `capacity`. Each class's guaranteed share is
    /// `max(1, capacity * weight / total_weight)`; the shares must fit
    /// inside the capacity — the leftover is the shared borrow pool.
    pub fn with_weights(weights: &[u64], capacity: usize) -> Result<Self, FairnessError> {
        if weights.is_empty() {
            return Err(FairnessError::EmptyClasses);
        }
        if weights.len() > MAX_FAIR_CLASSES {
            return Err(FairnessError::TooManyClasses {
                count: weights.len(),
                max: MAX_FAIR_CLASSES,
            });
        }
        if capacity > MAX_FAIR_CAPACITY {
            return Err(FairnessError::CapacityAboveMax {
                capacity,
                max: MAX_FAIR_CAPACITY,
            });
        }
        for (class, &w) in weights.iter().enumerate() {
            if w == 0 {
                return Err(FairnessError::NonPositiveWeight { class, weight: w });
            }
        }
        let total_weight: u64 = weights.iter().sum();
        let soft: Vec<usize> = weights
            .iter()
            .map(|&w| {
                // capacity and weights are small enough that the product
                // fits u64 with huge margin; saturate defensively.
                let share = (capacity as u64).saturating_mul(w) / total_weight;
                share.max(1) as usize
            })
            .collect();
        let soft_total: usize = soft.iter().sum();
        if soft_total > capacity {
            return Err(FairnessError::SharesExceedCapacity {
                soft_total,
                capacity,
            });
        }
        let headroom = capacity - soft_total;
        let ceiling: Vec<usize> = soft.iter().map(|&s| s + headroom).collect();
        Ok(FairAdmission {
            capacity,
            soft,
            ceiling,
            inner: Mutex::new(FairnessInner {
                outstanding: vec![0; weights.len()],
                total_outstanding: 0,
                admitted: 0,
                rejected_global: 0,
                rejected_ceiling: 0,
                unknown_class: 0,
                over_releases: 0,
            }),
        })
    }

    /// Global capacity.
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Class count.
    pub fn classes(&self) -> usize {
        self.soft.len()
    }

    /// Guaranteed (weighted) share of `class`.
    pub fn soft_share(&self, class: usize) -> Option<usize> {
        self.soft.get(class).copied()
    }

    /// Absolute per-class ceiling: `share + headroom`. A class may
    /// borrow the whole pool, but never push others below their shares.
    pub fn ceiling(&self, class: usize) -> Option<usize> {
        self.ceiling.get(class).copied()
    }

    /// Shared borrow pool size: `capacity - soft_total`.
    pub fn headroom(&self) -> usize {
        self.capacity - self.soft.iter().sum::<usize>()
    }

    /// Admit one unit of work for `class`. Fail-fast and typed — the
    /// caller renders the same 503/backoff posture as queue overload.
    pub fn admit(&self, class: usize) -> Result<(), FairnessReject> {
        let mut g = self.inner.lock().unwrap();
        if class >= self.soft.len() {
            g.unknown_class = g.unknown_class.saturating_add(1);
            return Err(FairnessReject::UnknownClass {
                class,
                classes: self.soft.len(),
            });
        }
        if g.total_outstanding >= self.capacity {
            g.rejected_global = g.rejected_global.saturating_add(1);
            return Err(FairnessReject::GlobalFull {
                capacity: self.capacity,
            });
        }
        if g.outstanding[class] + 1 > self.ceiling[class] {
            g.rejected_ceiling = g.rejected_ceiling.saturating_add(1);
            return Err(FairnessReject::ClassCeiling {
                class,
                ceiling: self.ceiling[class],
            });
        }
        g.outstanding[class] += 1;
        g.total_outstanding += 1;
        g.admitted = g.admitted.saturating_add(1);
        Ok(())
    }

    /// Release one admitted unit for `class` (at the terminal
    /// transition). Returns the class's new outstanding count,
    /// saturating at 0; a release without a matching admit is counted
    /// (`over_releases`) — a host bug made observable, never a panic.
    pub fn release(&self, class: usize) -> usize {
        let mut g = self.inner.lock().unwrap();
        if class >= g.outstanding.len() {
            g.over_releases = g.over_releases.saturating_add(1);
            return 0;
        }
        if g.outstanding[class] == 0 {
            g.over_releases = g.over_releases.saturating_add(1);
            return 0;
        }
        g.outstanding[class] -= 1;
        g.total_outstanding -= 1;
        g.outstanding[class]
    }

    /// Live admissions for `class`.
    pub fn outstanding(&self, class: usize) -> usize {
        let g = self.inner.lock().unwrap();
        g.outstanding.get(class).copied().unwrap_or(0)
    }

    /// Live admissions across all classes.
    pub fn total_outstanding(&self) -> usize {
        self.inner.lock().unwrap().total_outstanding
    }

    /// Redacted observability snapshot: counters and bounds only.
    pub fn stats(&self) -> FairnessStats {
        let g = self.inner.lock().unwrap();
        FairnessStats {
            capacity: self.capacity,
            classes: self.soft.len(),
            headroom: self.capacity - self.soft.iter().sum::<usize>(),
            total_outstanding: g.total_outstanding,
            admitted: g.admitted,
            rejected_global: g.rejected_global,
            rejected_ceiling: g.rejected_ceiling,
            unknown_class: g.unknown_class,
            over_releases: g.over_releases,
        }
    }
}

/// Redacted fairness counters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FairnessStats {
    pub capacity: usize,
    pub classes: usize,
    pub headroom: usize,
    pub total_outstanding: usize,
    pub admitted: u64,
    pub rejected_global: u64,
    pub rejected_ceiling: u64,
    pub unknown_class: u64,
    pub over_releases: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn construction_validates_fail_closed() {
        assert_eq!(
            FairAdmission::with_weights(&[], 8).unwrap_err(),
            FairnessError::EmptyClasses
        );
        assert_eq!(
            FairAdmission::with_weights(&[2, 0, 1], 8).unwrap_err(),
            FairnessError::NonPositiveWeight {
                class: 1,
                weight: 0
            }
        );
        let many = vec![1u64; MAX_FAIR_CLASSES + 1];
        assert_eq!(
            FairAdmission::with_weights(&many, MAX_FAIR_CAPACITY).unwrap_err(),
            FairnessError::TooManyClasses {
                count: MAX_FAIR_CLASSES + 1,
                max: MAX_FAIR_CLASSES
            }
        );
        assert_eq!(
            FairAdmission::with_weights(&[1], MAX_FAIR_CAPACITY + 1).unwrap_err(),
            FairnessError::CapacityAboveMax {
                capacity: MAX_FAIR_CAPACITY + 1,
                max: MAX_FAIR_CAPACITY
            }
        );
        // Tiny capacity: three classes each demand share 1, total 3 > 2.
        assert_eq!(
            FairAdmission::with_weights(&[1, 1, 1], 2).unwrap_err(),
            FairnessError::SharesExceedCapacity {
                soft_total: 3,
                capacity: 2
            }
        );
        assert!(FairnessError::SharesExceedCapacity {
            soft_total: 3,
            capacity: 2
        }
        .to_string()
        .contains("exceed capacity"));
    }

    #[test]
    fn shares_are_weighted_and_headroom_is_shared() {
        // 3:1 weights over capacity 8 -> shares 6:2, headroom 0.
        let fair = FairAdmission::with_weights(&[3, 1], 8).unwrap();
        assert_eq!(fair.soft_share(0), Some(6));
        assert_eq!(fair.soft_share(1), Some(2));
        assert_eq!(fair.headroom(), 0);
        assert_eq!(fair.ceiling(0), Some(6));
        // 1:1 over capacity 8 -> shares 4:4 with 0 headroom (strict
        // partition when shares exactly fill capacity).
        let fair = FairAdmission::with_weights(&[1, 1], 8).unwrap();
        assert_eq!((fair.soft_share(0), fair.soft_share(1)), (Some(4), Some(4)));
        assert_eq!(fair.headroom(), 0);
        // 1:1 over capacity 10 -> shares 5:5, headroom 0; and a
        // min-share floor: weights 100:1 over capacity 8 can't give the
        // tiny class 0.
        let fair = FairAdmission::with_weights(&[100, 1], 40).unwrap();
        assert_eq!(fair.soft_share(1), Some(1), "share floor is 1");
        assert!(fair.soft_share(0).unwrap() > 1);
        assert_eq!(fair.classes(), 2);
        assert_eq!(fair.capacity(), 40);
    }

    #[test]
    fn borrow_pool_gives_every_class_the_same_burst_room() {
        // Shares 2:1 over capacity 6 -> soft 4:2, headroom 0? No:
        // 2:1 => 4:2, total 6 == capacity, headroom 0. Use 7.
        let fair = FairAdmission::with_weights(&[2, 1], 7).unwrap();
        assert_eq!((fair.soft_share(0), fair.soft_share(1)), (Some(4), Some(2)));
        assert_eq!(fair.headroom(), 1);
        assert_eq!((fair.ceiling(0), fair.ceiling(1)), (Some(5), Some(3)));
    }

    #[test]
    fn class_below_share_always_admits_while_global_has_room() {
        // P1: guaranteed slice. Greedy class 0 saturates its ceiling;
        // class 1 still admits up to its share.
        let fair = FairAdmission::with_weights(&[1, 1], 8).unwrap();
        for _ in 0..4 {
            fair.admit(0).unwrap();
        }
        for _ in 0..4 {
            fair.admit(1).unwrap();
        }
        assert_eq!(fair.total_outstanding(), 8);
        // Global full: both are refused now.
        assert_eq!(
            fair.admit(0).unwrap_err(),
            FairnessReject::GlobalFull { capacity: 8 }
        );
        // Release one from class 1: class 1 (below share) admits again.
        fair.release(1);
        fair.admit(1).unwrap();
        // Release one from class 0: the freed slot serves class 0 (still
        // at its share boundary).
        fair.release(0);
        fair.admit(0).unwrap();
        assert_eq!(fair.total_outstanding(), 8);
    }

    #[test]
    fn ceiling_stops_bursting_even_when_global_has_room() {
        // P2: with shares 4:4 over capacity 8, ceiling = share + 0; use
        // headroom to see the ceiling bind: capacity 12, weights 1:2 ->
        // soft 4:8, total 12, headroom 0. Add capacity 14 -> soft 4:9? No:
        // 14*1/3=4, 14*2/3=9, total 13, headroom 1. Ceilings 5:10.
        let fair = FairAdmission::with_weights(&[1, 2], 14).unwrap();
        assert_eq!((fair.soft_share(0), fair.soft_share(1)), (Some(4), Some(9)));
        assert_eq!(fair.headroom(), 1);
        assert_eq!((fair.ceiling(0), fair.ceiling(1)), (Some(5), Some(10)));
        // Class 1 bursts to its ceiling of 10 (global allows 14).
        for _ in 0..10 {
            fair.admit(1).unwrap();
        }
        let err = fair.admit(1).unwrap_err();
        assert_eq!(
            err,
            FairnessReject::ClassCeiling {
                class: 1,
                ceiling: 10
            }
        );
        assert!(err.to_string().contains("ceiling"));
        // The greedy class consumed the only headroom slot: class 0
        // still gets its FULL share (4) — denied only at the true
        // global boundary, never by the greedy neighbor.
        for _ in 0..4 {
            fair.admit(0).unwrap();
        }
        assert_eq!(fair.total_outstanding(), 14);
        assert_eq!(
            fair.admit(0).unwrap_err(),
            FairnessReject::GlobalFull { capacity: 14 }
        );
    }

    #[test]
    fn fleet_protection_denial_only_under_collective_saturation() {
        // P3: a below-share class is denied ONLY when the global pool is
        // exhausted. A greedy class at its ceiling cannot cause the
        // denial of another class below its share.
        let fair = FairAdmission::with_weights(&[1, 1], 8).unwrap();
        // Class 0 saturates: 4 (share) + 0 headroom; shares 4:4 fill 8.
        for _ in 0..4 {
            fair.admit(0).unwrap();
        }
        // Class 1 demands: gets its full share, never denied below it.
        for i in 0..4 {
            fair.admit(1).unwrap_or_else(|e| {
                panic!("class 1 denied at {i} below/at share — fleet protection broken: {e:?}")
            });
        }
        // With headroom present, denial below share is impossible for
        // the protected class even while the other borrows: capacity 10,
        // weights 1:1 -> shares 5:5, headroom 0; capacity 12 -> 6:6 with
        // headroom 0. Make real headroom: 3 classes 1:1:1 over 12 ->
        // shares 4:4:4, headroom 0; over 15 -> 5:5:5, headroom 0.
        // Integer shares tend to fill capacity; force headroom via
        // weights 1:1:2 over 16 -> 4:4:8, total 16... use 1:1:1 over 14
        // -> 4:4:4, headroom 2. Ceilings 6:6:6.
        let fair = FairAdmission::with_weights(&[1, 1, 1], 14).unwrap();
        assert_eq!(fair.headroom(), 2);
        // Class 0 borrows to its ceiling 6.
        for _ in 0..6 {
            fair.admit(0).unwrap();
        }
        assert_eq!(
            fair.admit(0).unwrap_err(),
            FairnessReject::ClassCeiling {
                class: 0,
                ceiling: 6
            }
        );
        // Classes 1 and 2 each still get their full share (4 each),
        // plus their borrow room — none is denied below share.
        for _ in 0..4 {
            fair.admit(1).unwrap();
            fair.admit(2).unwrap();
        }
        assert_eq!(fair.total_outstanding(), 14);
        // Only the truly-global boundary denies a below-share class.
        assert_eq!(
            fair.admit(1).unwrap_err(),
            FairnessReject::GlobalFull { capacity: 14 }
        );
    }

    #[test]
    fn global_bound_is_never_exceeded_under_concurrency() {
        // P4 under racing threads: 4 classes, 2000 admit attempts each
        // against capacity 100 — total outstanding never passes 100 and
        // the accounting (admitted == outstanding) stays exact.
        let fair = Arc::new(FairAdmission::with_weights(&[1, 1, 1, 1], 100).unwrap());
        let mut handles = Vec::new();
        for c in 0..4usize {
            let f = fair.clone();
            handles.push(std::thread::spawn(move || {
                let mut ok = 0usize;
                for _ in 0..2000 {
                    if f.admit(c).is_ok() {
                        ok += 1;
                    }
                }
                ok
            }));
        }
        let admitted: usize = handles.into_iter().map(|h| h.join().unwrap()).sum();
        assert!(
            admitted <= 100,
            "the global bound must hold under any interleaving: {admitted}"
        );
        assert_eq!(fair.total_outstanding(), admitted);
        let s = fair.stats();
        assert_eq!(s.admitted as usize, admitted);
        assert_eq!(
            s.rejected_global as usize + s.rejected_ceiling as usize,
            8000 - admitted
        );
    }

    #[test]
    fn release_is_saturating_and_over_releases_are_counted() {
        let fair = FairAdmission::with_weights(&[1], 4).unwrap();
        fair.release(0);
        fair.release(0);
        assert_eq!(fair.outstanding(0), 0);
        assert_eq!(fair.total_outstanding(), 0);
        fair.admit(0).unwrap();
        fair.admit(0).unwrap();
        assert_eq!(fair.release(0), 1);
        assert_eq!(fair.release(0), 0);
        assert_eq!(fair.release(0), 0, "saturates at zero");
        let s = fair.stats();
        assert_eq!(s.over_releases, 3, "unmatched releases are observable");
        assert_eq!(fair.release(7), 0, "unknown class release is safe");
        assert_eq!(fair.stats().over_releases, 4);
    }

    #[test]
    fn unknown_class_is_typed_and_counted() {
        let fair = FairAdmission::with_weights(&[1, 1], 4).unwrap();
        let err = fair.admit(2).unwrap_err();
        assert_eq!(
            err,
            FairnessReject::UnknownClass {
                class: 2,
                classes: 2
            }
        );
        assert_eq!(fair.stats().unknown_class, 1);
        assert_eq!(fair.total_outstanding(), 0);
    }

    #[test]
    fn stats_balance_and_redaction() {
        let fair = FairAdmission::with_weights(&[3, 1], 8).unwrap();
        for _ in 0..6 {
            fair.admit(0).unwrap();
        }
        for _ in 0..2 {
            fair.admit(1).unwrap();
        }
        assert!(fair.admit(0).is_err());
        fair.release(0);
        let s = fair.stats();
        assert_eq!((s.capacity, s.classes, s.headroom), (8, 2, 0));
        assert_eq!((s.admitted, s.total_outstanding), (8, 7));
        assert_eq!(s.rejected_global + s.rejected_ceiling, 1);
        let dbg = format!("{s:?}");
        assert!(
            !dbg.contains("caller") && !dbg.contains("payload"),
            "stats carry counters only: {dbg}"
        );
    }

    #[test]
    fn admits_and_releases_race_exactly() {
        // Admit/release pairs from many threads: the outstanding count
        // returns to zero exactly, admitted == settled releases + live.
        let fair = Arc::new(FairAdmission::with_weights(&[1, 1], 50).unwrap());
        let mut handles = Vec::new();
        for c in 0..2usize {
            for _ in 0..4 {
                let f = fair.clone();
                handles.push(std::thread::spawn(move || {
                    for _ in 0..500 {
                        if f.admit(c).is_ok() {
                            f.release(c);
                        }
                    }
                }));
            }
        }
        for h in handles {
            h.join().unwrap();
        }
        assert_eq!(fair.total_outstanding(), 0);
        let s = fair.stats();
        assert_eq!(s.over_releases, 0, "every release matched an admit");
        assert_eq!(s.total_outstanding, 0);
    }
}
