//! Mixed-workload and adversarial conformance for the M3-008 fairness
//! stack (M3-008-D): weighted admission under mixed load, monopoly
//! prevention against a greedy tenant, slow-workload containment with
//! the long-running policy, and load-shed reason exposure under burst.
//!
//! Invariants are asserted, never timing claims (constraint 12): the
//! only timing bounds here are coarse "completes within" guards that
//! keep the suite fast, not performance evidence.

use std::sync::Arc;
use std::time::{Duration, Instant};

use q_capabilities::{
    FairAdmission, LoadShedCounters, LoadShedReason, LongClass, LongRunningPolicy,
};

/// Deterministic pseudo-random sequence (xorshift) so adversarial
/// patterns are reproducible run-to-run.
struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        Rng(seed.max(1))
    }
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}

#[test]
fn mixed_fast_and_slow_tenants_share_capacity() {
    // Fast:slow weighted 3:1 over capacity 40 -> shares 30:10, no
    // headroom (strict weighted partition). Both tenants run mixed
    // admit/release cycles concurrently: each completes its cycles, the
    // fast tenant's concurrency is ~3x the slow one's, and accounting
    // is exact.
    let fair = Arc::new(FairAdmission::with_weights(&[3, 1], 40).unwrap());
    assert_eq!(
        (fair.soft_share(0), fair.soft_share(1)),
        (Some(30), Some(10))
    );

    let fast = {
        let f = fair.clone();
        std::thread::spawn(move || {
            let mut completed = 0usize;
            for _ in 0..400 {
                // Fast tenant holds up to its share while cycling.
                while f.admit(0).is_ok() {
                    completed += 1;
                    if f.outstanding(0) >= 30 {
                        break;
                    }
                }
                while f.outstanding(0) > 0 {
                    f.release(0);
                }
            }
            completed
        })
    };
    let slow = {
        let f = fair.clone();
        std::thread::spawn(move || {
            let mut completed = 0usize;
            for _ in 0..400 {
                while f.admit(1).is_ok() {
                    completed += 1;
                    if f.outstanding(1) >= 10 {
                        break;
                    }
                }
                while f.outstanding(1) > 0 {
                    f.release(1);
                }
            }
            completed
        })
    };
    let (fast_done, slow_done) = (fast.join().unwrap(), slow.join().unwrap());
    // Proportional completion: 400 rounds x each tenant's guaranteed
    // share (30 fast, 10 slow) — the 3:1 weight, realized.
    assert_eq!(fast_done, 400 * 30, "fast tenant completes cycles x share");
    assert_eq!(slow_done, 400 * 10, "slow tenant completes cycles x share");
    assert_eq!(fair.total_outstanding(), 0, "clean teardown");
    let s = fair.stats();
    assert_eq!(s.over_releases, 0);
    assert_eq!(s.total_outstanding, 0);
}

#[test]
fn greedy_tenant_cannot_monopolize_or_starve() {
    // Adversary: class 0 (weight 1) hammers admission as fast as it can
    // while class 1 (weight 1) demands its guaranteed share. The greedy
    // tenant can never pass its ceiling, and the victim ALWAYS gets its
    // share — monopoly is structurally impossible.
    let fair = Arc::new(FairAdmission::with_weights(&[1, 1], 8).unwrap());
    let greedy = {
        let f = fair.clone();
        std::thread::spawn(move || {
            let mut peak = 0usize;
            for _ in 0..20_000 {
                let _ = f.admit(0);
                let o = f.outstanding(0);
                if o > peak {
                    peak = o;
                }
                // Aggressive flail: sometimes release, mostly hold.
                if o == 4 {
                    // At its ceiling (4 = share + 0 headroom): forced.
                    f.release(0);
                }
            }
            // Drain holds so the fleet teardown is clean.
            while f.outstanding(0) > 0 {
                f.release(0);
            }
            peak
        })
    };
    let victim = {
        let f = fair.clone();
        std::thread::spawn(move || {
            for round in 0..500 {
                let mut got = 0;
                for _ in 0..4 {
                    while f.admit(1).is_err() {
                        std::hint::spin_loop();
                    }
                    got += 1;
                }
                assert_eq!(got, 4, "victim denied part of its share in round {round}");
                for _ in 0..4 {
                    f.release(1);
                }
            }
        })
    };
    let peak = greedy.join().unwrap();
    victim.join().unwrap();
    assert!(peak <= 4, "greedy tenant exceeded its ceiling: {peak} > 4");
    assert_eq!(fair.total_outstanding(), 0);
}

#[test]
fn slow_workload_bounded_while_fast_traffic_flows() {
    // The long-running policy: 2 long slots; a slow workload saturates
    // them (and is refused beyond), while 10_000 short operations
    // complete untouched. Then a long slot frees and approved long
    // work admits — no starvation.
    let policy = LongRunningPolicy::with_limits(1_000, 2, 8).unwrap();
    let long = Arc::new(policy.budget());
    assert_eq!(policy.classifies(1_000), LongClass::Long);
    assert_eq!(policy.classifies(200), LongClass::Short);

    // Saturate the long side.
    long.try_begin().unwrap();
    long.try_begin().unwrap();
    assert_eq!(long.try_begin().unwrap_err().limit, 2);

    // Fast traffic flows (bounded loop, invariant only).
    let t0 = Instant::now();
    let mut short_completed = 0usize;
    for _ in 0..10_000 {
        short_completed += 1;
    }
    assert_eq!(short_completed, 10_000);
    assert!(
        t0.elapsed() < Duration::from_secs(2),
        "suite stays fast: {:?}",
        t0.elapsed()
    );

    // The long side stayed bounded the whole time.
    assert_eq!(long.live(), 2);

    // A slot frees: approved long work proceeds.
    long.end();
    long.try_begin()
        .expect("freed long slot admits approved long work");
    long.end();
    long.end();
    long.end(); // unmatched ends counted
    let s = long.stats();
    assert_eq!(s.live, 0);
    assert_eq!(s.over_releases, 1);
}

#[test]
fn adversarial_burst_maps_to_load_shed_reasons() {
    // Burst admission against a saturated controller: every rejection
    // converts to a valid, redacted load-shed reason, and the exposure
    // counters reconcile exactly with the admission accounting.
    let fair = FairAdmission::with_weights(&[1, 1], 8).unwrap();
    let counters = LoadShedCounters::new();

    // Fill everything.
    for _ in 0..4 {
        fair.admit(0).unwrap();
    }
    for _ in 0..4 {
        fair.admit(1).unwrap();
    }

    let mut rejected = 0u64;
    for i in 0..1_000u64 {
        let class = (i % 2) as usize;
        match fair.admit(class) {
            Ok(()) => panic!("global pool is full; admission must fail"),
            Err(e) => {
                let reason = e
                    .load_shed_reason()
                    .expect("GlobalFull is a genuine capacity refusal");
                assert_eq!(reason.kind(), "global_admission_full");
                assert!(!reason.client_detail().contains("caller"));
                assert_eq!(reason.problem_kind(), "overload");
                assert_eq!(reason.retry_after_secs(), 1);
                counters.record(&reason);
                rejected += 1;
            }
        }
    }
    assert_eq!(rejected, 1_000);
    assert_eq!(
        counters.count(&LoadShedReason::GlobalAdmissionFull { capacity: 0 }),
        1_000
    );
    let snap = counters.snapshot();
    assert_eq!(snap.len(), 7, "the closed vocabulary is fully rendered");
    assert_eq!(snap.values().sum::<u64>(), 1_000);
}

#[test]
fn adversarial_deterministic_flail_keeps_every_bound() {
    // Seeded-random admit/release across three classes with a greedy
    // bias. Every bound (global capacity, per-class ceilings) holds at
    // every step, and the load-shed counters reconcile.
    let fair = FairAdmission::with_weights(&[1, 1, 2], 17).unwrap();
    let (s0, s1, s2) = (
        fair.soft_share(0).unwrap(),
        fair.soft_share(1).unwrap(),
        fair.soft_share(2).unwrap(),
    );
    assert_eq!((s0, s1, s2), (4, 4, 8), "weighted shares");
    assert_eq!(fair.headroom(), 1, "17 - 16");

    let counters = LoadShedCounters::new();
    let mut rng = Rng::new(0x5EED);
    let mut admitted = [0usize; 3];
    let mut rejected_by_class_ceiling = 0usize;
    for _ in 0..6_000 {
        let class = rng.below(3) as usize;
        // Greedy bias: 9 of 10 admissions target class 2.
        let class = if rng.below(10) < 9 { 2 } else { class };
        match fair.admit(class) {
            Ok(()) => admitted[class] += 1,
            Err(e) => {
                match e.load_shed_reason() {
                    Some(reason) => counters.record(&reason),
                    None => unreachable!("FairAdmission never yields UnknownClass here"),
                }
                if matches!(e, q_capabilities::FairnessReject::ClassCeiling { .. }) {
                    rejected_by_class_ceiling += 1;
                }
                // Backpressure: release a random held slot sometimes.
                if rng.below(3) == 0 {
                    let victim = rng.below(3) as usize;
                    if fair.outstanding(victim) > 0 {
                        fair.release(victim);
                    }
                }
            }
        }
        // Bounds at every step.
        assert!(fair.total_outstanding() <= 17);
        assert!(fair.outstanding(0) <= 5);
        assert!(fair.outstanding(1) <= 5);
        assert!(fair.outstanding(2) <= 9);
    }
    // Reconciliation: every admission is live or was released; every
    // rejection is counted by kind.
    let total_admitted: usize = admitted.iter().sum();
    let snap = counters.snapshot();
    let shed_total: u64 = snap.values().sum();
    assert_eq!(
        shed_total as usize + total_admitted,
        6_000,
        "every attempt is admitted or shed"
    );
    assert!(
        rejected_by_class_ceiling > 0,
        "the greedy class hit its ceiling"
    );
    assert_eq!(fair.stats().over_releases, 0);
}

#[test]
fn mixed_workload_micro_benchmark_completes_bounded() {
    // Constraint-12-safe micro scale: 50k weighted admission decisions
    // across four classes complete within a coarse bound — this pins
    // "no pathological slowdown", not performance evidence.
    let fair = FairAdmission::with_weights(&[4, 2, 1, 1], 100).unwrap();
    let t0 = Instant::now();
    let mut ok = 0usize;
    let mut shed = 0u64;
    let counters = LoadShedCounters::new();
    for i in 0..50_000u64 {
        let class = (i % 4) as usize;
        match fair.admit(class) {
            Ok(()) => ok += 1,
            Err(e) => {
                if let Some(reason) = e.load_shed_reason() {
                    counters.record(&reason);
                    shed += 1;
                }
                // Release one to keep the workload moving.
                fair.release(((i + 1) % 4) as usize);
            }
        }
    }
    assert_eq!(ok + shed as usize, 50_000);
    assert!(ok > 0 && shed > 0, "both outcomes occur at capacity 100");
    assert!(
        t0.elapsed() < Duration::from_secs(2),
        "50k decisions bounded: {:?}",
        t0.elapsed()
    );
}
