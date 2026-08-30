//! Admission drain gate (M3-007-B, ADR-0036 §4 lifecycle-flag discipline).
//!
//! One atomic flag flips the runtime from Serving to Draining the
//! instant the shutdown signal fires. Every dynamic admission checks
//! the gate lock-free: after the flip, requests that would enter JS
//! are refused with the frozen overload problem (503, retry-after)
//! while in-flight work completes. Native liveness keeps answering so
//! load balancers observe the instance going away.
//!
//! Plain host-side state: two atomics, saturating counters, no JS
//! values, no locks. Shared across host threads; the flip happens
//! exactly once (the swap's winner logs the drain.begin event).

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use crate::shared_handles::SharedAcrossWorkers;

/// Typed admission refusal during drain. Redacted by construction: it
/// carries no request data — only the fact of the drain.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmissionDrained;

impl std::fmt::Display for AdmissionDrained {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "server is draining; not accepting new work")
    }
}

impl std::error::Error for AdmissionDrained {}

/// The serving/draining lifecycle flag (M3-007-B).
#[derive(Debug)]
pub struct DrainGate {
    draining: AtomicBool,
    refused: AtomicU64,
}

impl SharedAcrossWorkers for DrainGate {}

impl Default for DrainGate {
    fn default() -> Self {
        Self::new()
    }
}

impl DrainGate {
    pub fn new() -> Self {
        DrainGate {
            draining: AtomicBool::new(false),
            refused: AtomicU64::new(0),
        }
    }

    /// Flip Serving -> Draining. Returns `true` for EXACTLY one caller
    /// (the flip's winner — the runtime logs its `drain.begin` event);
    /// every later call is an idempotent no-op reporting `false`.
    pub fn begin(&self) -> bool {
        // AcqRel: the flip is the shutdown boundary — acquires every
        // admission decision sequenced before it, releases the drained
        // state to every checker after it.
        !self.draining.swap(true, Ordering::AcqRel)
    }

    /// Lock-free drain state (hot path — same posture as the engine
    /// health quarantine check).
    pub fn is_draining(&self) -> bool {
        self.draining.load(Ordering::Acquire)
    }

    /// Admission check. `Ok(())` while serving; once draining, the
    /// refusal is counted (saturating) and returned typed.
    pub fn check_admission(&self) -> Result<(), AdmissionDrained> {
        if self.is_draining() {
            // Saturating: the counter never wraps (house rule).
            let _ = self
                .refused
                .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |v| v.checked_add(1));
            return Err(AdmissionDrained);
        }
        Ok(())
    }

    /// Admissions refused during drain (the shutdown report carries
    /// this — it is the count of clients honestly told to retry).
    pub fn refused(&self) -> u64 {
        self.refused.load(Ordering::Relaxed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn serving_by_default_and_admission_flows() {
        let gate = DrainGate::new();
        assert!(!gate.is_draining());
        assert_eq!(gate.refused(), 0);
        assert!(gate.check_admission().is_ok());
        assert!(gate.check_admission().is_ok());
        assert_eq!(gate.refused(), 0, "serving admissions are not counted");
    }

    #[test]
    fn begin_flips_exactly_once_and_refuses_admission() {
        let gate = DrainGate::new();
        assert!(gate.begin(), "the first flip wins");
        assert!(!gate.begin(), "the second is an idempotent no-op");
        assert!(!gate.begin());
        assert!(gate.is_draining());
        let err = gate.check_admission().unwrap_err();
        assert_eq!(err, AdmissionDrained);
        assert!(
            err.to_string().contains("draining"),
            "redacted, honest refusal text: {err}"
        );
        assert_eq!(gate.check_admission().unwrap_err(), AdmissionDrained);
        assert_eq!(gate.refused(), 2, "every refusal is counted");
    }

    #[test]
    fn refused_count_saturates_without_panicking() {
        let gate = DrainGate::new();
        gate.begin();
        gate.refused.store(u64::MAX, Ordering::Relaxed);
        assert!(gate.check_admission().is_err());
        assert_eq!(gate.refused(), u64::MAX, "saturating, never wraps");
    }

    #[test]
    fn concurrent_begins_have_exactly_one_winner() {
        let gate = Arc::new(DrainGate::new());
        let handles: Vec<_> = (0..8)
            .map(|_| {
                let g = gate.clone();
                std::thread::spawn(move || g.begin())
            })
            .collect();
        let winners = handles
            .into_iter()
            .map(|h| h.join().unwrap())
            .filter(|won| *won)
            .count();
        assert_eq!(winners, 1, "exactly one thread flips the gate");
        assert!(gate.is_draining());
    }

    #[test]
    fn drain_state_is_visible_across_threads_immediately() {
        let gate = Arc::new(DrainGate::new());
        let observer = {
            let g = gate.clone();
            std::thread::spawn(move || {
                // Bounded spin: the flip must become visible without
                // arbitrary delay (AcqRel swap -> Acquire load).
                for _ in 0..1_000_000 {
                    if g.is_draining() {
                        return true;
                    }
                    std::hint::spin_loop();
                }
                false
            })
        };
        std::thread::sleep(std::time::Duration::from_millis(10));
        assert!(gate.begin());
        assert!(observer.join().unwrap(), "drain state crossed threads");
        assert_eq!(gate.check_admission(), Err(AdmissionDrained));
    }

    #[test]
    fn redaction_debug_carries_no_request_data() {
        let gate = DrainGate::new();
        gate.begin();
        let _ = gate.check_admission();
        let dbg = format!("{gate:?}");
        assert!(
            !dbg.contains("request") && !dbg.contains("path"),
            "gate debug output is counters and state only: {dbg}"
        );
    }
}
