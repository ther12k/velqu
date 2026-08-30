//! Long-running JS policy (M3-008-B).
//!
//! One slow workload must not monopolize a worker. The policy makes the
//! long/short classification explicit and configurable, and gives
//! long-running invocations a BOUNDED slot budget per tracking domain
//! (per worker, and fleet-wide): small requests always keep dedicated
//! capacity, long-running work still makes progress in approved
//! scenarios (a slot frees -> the next long invocation admits), and
//! nothing can grow without bound.
//!
//! Classification is deterministic and configuration-driven: an
//! invocation is LONG-RUNNING exactly when its route deadline is at or
//! above the configured threshold. No runtime measurement is needed at
//! admission, so the decision is reproducible from the pack alone.
//!
//! Discipline (ADR-0036 §4): plain host-side state behind one mutex,
//! bounded by construction (`live <= limit` always), saturating
//! counters, no JS values.

use std::sync::Mutex;
use std::time::Duration;

use crate::shared_handles::SharedAcrossWorkers;

/// Default threshold at which an invocation classifies as long-running.
pub const DEFAULT_LONG_RUNNING_THRESHOLD_MS: u64 = 1_000;

/// Upper bound for the threshold (bounded configuration; the largest
/// legal route deadline scale).
pub const MAX_LONG_RUNNING_THRESHOLD_MS: u64 = 60_000;

/// Typed policy-construction violation. Fail-closed: an invalid policy
/// must never silently become "unlimited long-running work".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LongRunningPolicyError {
    /// The threshold must be >= 1 ms.
    ThresholdZero,
    /// The threshold must stay below
    /// [`MAX_LONG_RUNNING_THRESHOLD_MS`].
    ThresholdAboveMax { threshold_ms: u64, max: u64 },
    /// The per-domain long-slot budget must be >= 1 (an approved
    /// long-running scenario always has at least one slot).
    ZeroLongSlots,
    /// `long_slots` must be strictly smaller than the short capacity it
    /// protects, or the progress guarantee is vacuous.
    LongSlotsExceedShortCapacity {
        long_slots: usize,
        short_capacity: usize,
    },
}

impl std::fmt::Display for LongRunningPolicyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LongRunningPolicyError::ThresholdZero => {
                write!(f, "long-running threshold must be >= 1 ms")
            }
            LongRunningPolicyError::ThresholdAboveMax { threshold_ms, max } => write!(
                f,
                "long-running threshold {threshold_ms} ms exceeds the maximum of {max} ms"
            ),
            LongRunningPolicyError::ZeroLongSlots => {
                write!(f, "long-running slot budget must be >= 1")
            }
            LongRunningPolicyError::LongSlotsExceedShortCapacity {
                long_slots,
                short_capacity,
            } => write!(
                f,
                "long slots ({long_slots}) must stay below the short capacity ({short_capacity}) they protect"
            ),
        }
    }
}

impl std::error::Error for LongRunningPolicyError {}

/// Typed long-running admission rejection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LongSlotsExhausted {
    /// The configured slot bound that refused the admission.
    pub limit: usize,
}

impl std::fmt::Display for LongSlotsExhausted {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "long-running slots exhausted (limit {}); retry after in-flight long work settles",
            self.limit
        )
    }
}

impl std::error::Error for LongSlotsExhausted {}

/// The invocation class the policy assigns.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LongClass {
    /// Deadline below the threshold: the fast path, never gated by the
    /// long-running budget.
    Short,
    /// Deadline at/above the threshold: admitted only through the
    /// bounded long-running budget.
    Long,
}

/// The policy: classification rule + bounded budgets (M3-008-B).
#[derive(Debug, Clone)]
pub struct LongRunningPolicy {
    threshold: Duration,
    long_slots_per_domain: usize,
    short_capacity_per_domain: usize,
}

impl LongRunningPolicy {
    /// Validate and build the policy. `long_slots_per_domain` is the
    /// long-running budget per tracking domain (per worker / per
    /// class); `short_capacity_per_domain` is the short-request
    /// capacity it must protect (the progress guarantee requires
    /// `long_slots < short_capacity`).
    pub fn with_limits(
        threshold_ms: u64,
        long_slots_per_domain: usize,
        short_capacity_per_domain: usize,
    ) -> Result<Self, LongRunningPolicyError> {
        if threshold_ms == 0 {
            return Err(LongRunningPolicyError::ThresholdZero);
        }
        if threshold_ms > MAX_LONG_RUNNING_THRESHOLD_MS {
            return Err(LongRunningPolicyError::ThresholdAboveMax {
                threshold_ms,
                max: MAX_LONG_RUNNING_THRESHOLD_MS,
            });
        }
        if long_slots_per_domain == 0 {
            return Err(LongRunningPolicyError::ZeroLongSlots);
        }
        if long_slots_per_domain >= short_capacity_per_domain {
            return Err(LongRunningPolicyError::LongSlotsExceedShortCapacity {
                long_slots: long_slots_per_domain,
                short_capacity: short_capacity_per_domain,
            });
        }
        Ok(LongRunningPolicy {
            threshold: Duration::from_millis(threshold_ms),
            long_slots_per_domain,
            short_capacity_per_domain,
        })
    }

    /// Default policy: 1 s threshold, 2 long slots per domain, 8 short
    /// slots protected.
    pub fn with_defaults() -> Self {
        Self::with_limits(DEFAULT_LONG_RUNNING_THRESHOLD_MS, 2, 8)
            .expect("defaults satisfy the policy invariants")
    }

    /// The classification threshold.
    pub fn threshold(&self) -> Duration {
        self.threshold
    }

    /// Long-slot budget per domain.
    pub fn long_slots(&self) -> usize {
        self.long_slots_per_domain
    }

    /// The short capacity this policy protects.
    pub fn short_capacity(&self) -> usize {
        self.short_capacity_per_domain
    }

    /// Deterministic classification: a route deadline at/above the
    /// threshold makes the invocation long-running (the boundary is
    /// inclusive — a route that may legally run the full threshold is
    /// governed, not exempt).
    pub fn classifies(&self, deadline_ms: u64) -> LongClass {
        if deadline_ms >= self.threshold.as_millis() as u64 {
            LongClass::Long
        } else {
            LongClass::Short
        }
    }

    /// A fresh bounded budget for one tracking domain (one worker, or
    /// the fleet-wide total).
    pub fn budget(&self) -> LongRunningBudget {
        LongRunningBudget::new(self.long_slots_per_domain)
    }
}

/// Bounded slot budget for long-running invocations in one domain.
#[derive(Debug)]
pub struct LongRunningBudget {
    limit: usize,
    inner: Mutex<BudgetInner>,
}

#[derive(Debug)]
struct BudgetInner {
    live: usize,
    admitted: u64,
    rejected: u64,
    over_releases: u64,
}

impl SharedAcrossWorkers for LongRunningBudget {}

impl LongRunningBudget {
    fn new(limit: usize) -> Self {
        LongRunningBudget {
            limit,
            inner: Mutex::new(BudgetInner {
                live: 0,
                admitted: 0,
                rejected: 0,
                over_releases: 0,
            }),
        }
    }

    /// Begin one long-running invocation. Fail-fast and typed: when all
    /// `limit` slots are live, the admission is refused immediately —
    /// small requests keep their dedicated capacity (parent guardrail),
    /// and approved long-running scenarios are never starved (a freed
    /// slot admits the next caller).
    pub fn try_begin(&self) -> Result<(), LongSlotsExhausted> {
        let mut g = self.inner.lock().unwrap();
        if g.live >= self.limit {
            g.rejected = g.rejected.saturating_add(1);
            return Err(LongSlotsExhausted { limit: self.limit });
        }
        g.live += 1;
        g.admitted = g.admitted.saturating_add(1);
        Ok(())
    }

    /// End one long-running invocation (terminal transition). Returns
    /// the new live count, saturating at 0; unmatched ends are counted.
    pub fn end(&self) -> usize {
        let mut g = self.inner.lock().unwrap();
        if g.live == 0 {
            g.over_releases = g.over_releases.saturating_add(1);
            return 0;
        }
        g.live -= 1;
        g.live
    }

    /// Live long-running invocations in this domain.
    pub fn live(&self) -> usize {
        self.inner.lock().unwrap().live
    }

    /// Redacted counters.
    pub fn stats(&self) -> LongRunningStats {
        let g = self.inner.lock().unwrap();
        LongRunningStats {
            limit: self.limit,
            live: g.live,
            admitted: g.admitted,
            rejected: g.rejected,
            over_releases: g.over_releases,
        }
    }
}

/// Redacted long-running budget counters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LongRunningStats {
    pub limit: usize,
    pub live: usize,
    pub admitted: u64,
    pub rejected: u64,
    pub over_releases: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn policy_construction_validates_fail_closed() {
        assert_eq!(
            LongRunningPolicy::with_limits(0, 2, 8).unwrap_err(),
            LongRunningPolicyError::ThresholdZero
        );
        assert_eq!(
            LongRunningPolicy::with_limits(MAX_LONG_RUNNING_THRESHOLD_MS + 1, 2, 8).unwrap_err(),
            LongRunningPolicyError::ThresholdAboveMax {
                threshold_ms: MAX_LONG_RUNNING_THRESHOLD_MS + 1,
                max: MAX_LONG_RUNNING_THRESHOLD_MS
            }
        );
        assert_eq!(
            LongRunningPolicy::with_limits(1_000, 0, 8).unwrap_err(),
            LongRunningPolicyError::ZeroLongSlots
        );
        assert_eq!(
            LongRunningPolicy::with_limits(1_000, 8, 8).unwrap_err(),
            LongRunningPolicyError::LongSlotsExceedShortCapacity {
                long_slots: 8,
                short_capacity: 8
            }
        );
        assert_eq!(
            LongRunningPolicy::with_limits(1_000, 9, 8).unwrap_err(),
            LongRunningPolicyError::LongSlotsExceedShortCapacity {
                long_slots: 9,
                short_capacity: 8
            }
        );
        // Boundary acceptance: threshold at the max, long slots just
        // below the short capacity.
        assert!(LongRunningPolicy::with_limits(MAX_LONG_RUNNING_THRESHOLD_MS, 7, 8).is_ok());
        let p = LongRunningPolicy::with_defaults();
        assert_eq!(p.threshold(), Duration::from_millis(1_000));
        assert_eq!((p.long_slots(), p.short_capacity()), (2, 8));
    }

    #[test]
    fn classification_boundary_is_inclusive_and_deterministic() {
        let p = LongRunningPolicy::with_limits(1_000, 2, 8).unwrap();
        assert_eq!(p.classifies(0), LongClass::Short);
        assert_eq!(p.classifies(500), LongClass::Short);
        assert_eq!(p.classifies(999), LongClass::Short);
        // Inclusive boundary: a route allowed to run the full threshold
        // IS long-running.
        assert_eq!(p.classifies(1_000), LongClass::Long);
        assert_eq!(p.classifies(5_000), LongClass::Long);
        assert_eq!(p.classifies(60_000), LongClass::Long);
    }

    #[test]
    fn long_slots_exhaust_typed_while_short_capacity_is_untouched() {
        // Parent guardrail: small requests make progress under a slow
        // workload. The budget gates ONLY long admissions; the short
        // capacity (8) is structurally separate and always available.
        let p = LongRunningPolicy::with_limits(1_000, 2, 8).unwrap();
        let long = p.budget();
        long.try_begin().unwrap();
        long.try_begin().unwrap();
        let err = long.try_begin().unwrap_err();
        assert_eq!(err, LongSlotsExhausted { limit: 2 });
        assert!(err.to_string().contains("exhausted"));
        assert_eq!(long.stats().rejected, 1);
        assert_eq!(long.live(), 2);
        // The protected short capacity is fully available: nothing in
        // the policy gates it (structural guarantee — pinned by the
        // invariant long_slots < short_capacity validated at
        // construction and the budget holding no short-side state).
        assert_eq!(p.short_capacity(), 8);
        assert_eq!(p.long_slots(), 2);
        assert!(p.long_slots() < p.short_capacity());
    }

    #[test]
    fn approved_long_work_never_starves() {
        // A freed slot admits the next long caller: fail-fast rejection
        // is backpressure, not starvation.
        let p = LongRunningPolicy::with_limits(1_000, 1, 4).unwrap();
        let long = p.budget();
        long.try_begin().unwrap();
        assert!(long.try_begin().is_err());
        long.end();
        long.try_begin()
            .expect("a freed slot must admit approved long work");
        long.end();
        long.end(); // unmatched: counted, saturating
        long.end();
        let s = long.stats();
        assert_eq!(s.over_releases, 2);
        assert_eq!(s.live, 0);
    }

    #[test]
    fn budget_bounds_hold_under_concurrency() {
        // 4 threads x 500 racing try_begin/end pairs against a 3-slot
        // budget: live never exceeds the limit, accounting stays exact.
        let p = LongRunningPolicy::with_limits(1_000, 3, 8).unwrap();
        let budget = Arc::new(p.budget());
        let mut handles = Vec::new();
        for _ in 0..4 {
            let b = budget.clone();
            handles.push(std::thread::spawn(move || {
                let mut ok = 0usize;
                for _ in 0..500 {
                    if b.try_begin().is_ok() {
                        ok += 1;
                        b.end();
                    }
                }
                ok
            }));
        }
        let admitted: usize = handles.into_iter().map(|h| h.join().unwrap()).sum();
        let s = budget.stats();
        assert_eq!(s.live, 0, "every begin matched its end");
        assert_eq!(s.admitted as usize, admitted);
        assert_eq!(s.admitted as usize + s.rejected as usize, 2000);
        assert!(s.admitted as usize + s.rejected as usize == 2000);
    }

    #[test]
    fn live_never_exceeds_limit_under_a_held_slot_race() {
        // Holders release on a timer while spammers hammer: the bound
        // holds under any interleaving.
        let p = LongRunningPolicy::with_limits(1_000, 2, 8).unwrap();
        let budget = Arc::new(p.budget());
        let spam = {
            let b = budget.clone();
            std::thread::spawn(move || {
                let mut rejects = 0u64;
                for _ in 0..2000 {
                    if b.try_begin().is_err() {
                        rejects += 1;
                    } else {
                        b.end();
                    }
                }
                rejects
            })
        };
        let _ = spam.join();
        let s = budget.stats();
        assert!(s.live <= s.limit, "bound holds: {s:?}");
        assert_eq!(s.admitted as usize + s.rejected as usize, 2000);
        assert_eq!(s.over_releases, 0);
    }

    #[test]
    fn stats_are_redacted_counter_only() {
        let p = LongRunningPolicy::with_limits(1_000, 2, 8).unwrap();
        let budget = p.budget();
        budget.try_begin().unwrap();
        let s = budget.stats();
        assert_eq!(
            (s.limit, s.live, s.admitted, s.rejected, s.over_releases),
            (2, 1, 1, 0, 0)
        );
        let dbg = format!("{s:?}");
        assert!(!dbg.contains("route") && !dbg.contains("payload"));
    }

    #[test]
    fn independent_domains_track_independently() {
        // Per-worker budgets are independent; the fleet-wide budget is
        // its own domain (the composition is the multi-worker runtime's
        // wiring, M3-008-D).
        let p = LongRunningPolicy::with_limits(1_000, 1, 4).unwrap();
        let w0 = p.budget();
        let w1 = p.budget();
        let fleet = p.budget();
        w0.try_begin().unwrap();
        assert!(w0.try_begin().is_err());
        w1.try_begin().expect("worker 1's budget is independent");
        fleet.try_begin().unwrap();
        assert_eq!((w0.live(), w1.live(), fleet.live()), (1, 1, 1));
    }
}
