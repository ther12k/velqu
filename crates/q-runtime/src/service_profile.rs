//! Service profiles (M3-003, ADR-0018 M3 track): cold start versus
//! immediate throughput as an explicit deployment choice.
//!
//! - `Serverless`: starts EXACTLY ONE worker. Cold start is the whole
//!   point; additional workers may be added adaptively later (M3-003-B)
//!   but never pre-spawned at startup.
//! - `Service`: starts the CONFIGURED worker count up front — readiness
//!   is declared only when every configured worker is ready (M3-003-C).
//!
//! No hidden worker creation exists anywhere: the profile is the only
//! input to startup worker count, and the ready line reflects it
//! (M3-003-D exposes the profile in inspect/config output).

use std::fmt;

/// Worker startup posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ServiceProfile {
    /// Start with exactly ONE worker. Minimal cold start; the runtime
    /// may add workers adaptively after ready (M3-003-B), never before.
    #[default]
    Serverless,
    /// Start with the full CONFIGURED worker count before declaring
    /// readiness; deterministic startup, immediate full throughput.
    Service { workers: usize },
}

/// Bounds for configured worker counts (bounded configuration).
pub const MIN_WORKERS: usize = 1;
pub const MAX_WORKERS: usize = 64;

impl ServiceProfile {
    /// How many workers this profile starts AT STARTUP (before ready).
    /// Serverless is always exactly 1 — the M3-003-A guarantee.
    pub fn initial_workers(&self) -> usize {
        match self {
            ServiceProfile::Serverless => 1,
            ServiceProfile::Service { workers } => (*workers).clamp(MIN_WORKERS, MAX_WORKERS),
        }
    }

    /// Parse a profile from CLI/config text. Fail closed on unknown
    /// names — no silent fallback to any default.
    pub fn parse(text: &str) -> Result<Self, String> {
        let lower = text.trim().to_ascii_lowercase();
        match lower.as_str() {
            "serverless" => Ok(ServiceProfile::Serverless),
            _ => {
                if let Some(rest) = lower.strip_prefix("service:") {
                    let workers: usize = rest
                        .parse()
                        .map_err(|_| format!("invalid worker count: {rest:?}"))?;
                    if !(MIN_WORKERS..=MAX_WORKERS).contains(&workers) {
                        return Err(format!(
                            "worker count {workers} outside [{MIN_WORKERS},{MAX_WORKERS}]"
                        ));
                    }
                    return Ok(ServiceProfile::Service { workers });
                }
                if lower == "service" {
                    return Err(
                        "service profile requires an explicit worker count: service:N".into(),
                    );
                }
                Err(format!("unknown service profile: {text:?}"))
            }
        }
    }

    /// Stable name for ready-line / inspect output (M3-003-D).
    pub fn as_str(&self) -> String {
        match self {
            ServiceProfile::Serverless => "serverless".to_string(),
            ServiceProfile::Service { workers } => format!("service:{workers}"),
        }
    }
}

impl fmt::Display for ServiceProfile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.as_str())
    }
}

/// Adaptive worker-add policy state (M3-003-B).
///
/// Serverless declares ready after worker 0 is ready (one worker IS the
/// whole service at that moment), then adds workers only on observed
/// pressure. Bounded: adds are capped by `max_workers` and rate-limited
/// by `cooldown`, so no load pattern can spawn unboundedly.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdaptiveWorkers {
    /// Workers currently running (worker 0 counts).
    pub running: usize,
    /// Hard ceiling — never exceeded.
    pub max_workers: usize,
    /// Ready flag: true the moment worker 0 is ready.
    pub ready: bool,
    /// Number of add-events so far (observability; saturating).
    pub add_events: u64,
    /// Ticks since the last add (cooldown gate; saturating).
    pub ticks_since_add: u64,
    /// Ticks required between adds.
    pub cooldown_ticks: u64,
}

/// Outcome of one policy tick.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScaleTick {
    /// Add one worker (bounded by max, gated by cooldown).
    AddWorker,
    /// Stay at the current count.
    Hold,
}

impl AdaptiveWorkers {
    /// Policy starting point: worker 0 running, ready declared, nothing
    /// added yet. Ready-after-worker-0 is the initial state, not an
    /// event that has to happen.
    pub fn starting(max_workers: usize, cooldown_ticks: u64) -> Self {
        AdaptiveWorkers {
            running: 1,
            max_workers: max_workers.clamp(1, MAX_WORKERS),
            ready: true,
            add_events: 0,
            ticks_since_add: 0,
            cooldown_ticks,
        }
    }

    /// One policy tick with the observed pressure (queue depth summed
    /// over workers). Pressure above `pressure_threshold` may add one
    /// worker — bounded by max and cooldown; everything else holds.
    pub fn tick(&mut self, pressure: usize, pressure_threshold: usize) -> ScaleTick {
        self.ticks_since_add = self.ticks_since_add.saturating_add(1);
        // No cooldown before the first add ever — only between adds.
        let cooled = self.add_events == 0 || self.ticks_since_add > self.cooldown_ticks;
        let room = self.running < self.max_workers;
        if pressure > pressure_threshold && cooled && room {
            self.running = self.running.saturating_add(1);
            self.add_events = self.add_events.saturating_add(1);
            self.ticks_since_add = 0;
            return ScaleTick::AddWorker;
        }
        ScaleTick::Hold
    }
}

/// Deterministic readiness tracking (M3-003-C): counts initialized
/// workers against the profile's startup requirement. Readiness flips
/// exactly when the requirement is met — serverless needs worker 0,
/// service needs all configured workers — and never un-flips.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Readiness {
    profile: ServiceProfile,
    initialized: usize,
    ready: bool,
}

impl Readiness {
    /// Fresh tracker for `profile`: nothing initialized, not ready.
    pub fn starting(profile: ServiceProfile) -> Self {
        Readiness {
            profile,
            initialized: 0,
            ready: false,
        }
    }

    /// Profile requirement: how many workers must initialize before
    /// ready. Serverless = 1 (worker 0 only); service = the full
    /// configured count (clamped exactly like the profile itself).
    pub fn required(&self) -> usize {
        self.profile.initial_workers()
    }

    /// Record one worker finishing initialization. Returns `true` only on
    /// the call that CAUSES the ready transition — exactly one caller gets
    /// `true` (that caller announces readiness); calls before it get
    /// `false`, and calls after it get `false` again (never re-triggered,
    /// never un-set).
    pub fn worker_initialized(&mut self) -> bool {
        self.initialized = self.initialized.saturating_add(1);
        if !self.ready && self.initialized >= self.required() {
            self.ready = true;
            return true;
        }
        false
    }

    /// Whether the readiness requirement is met.
    pub fn is_ready(&self) -> bool {
        self.ready
    }

    /// Workers initialized so far.
    pub fn initialized(&self) -> usize {
        self.initialized
    }
}

/// Startup parallelism bounds (M3-004-D): initializing N workers may run
/// in parallel, but never unbounded — the concurrency is clamped to the
/// physical core count (up to [`MAX_STARTUP_PARALLELISM`]) and each
/// in-flight worker has a bounded init deadline. This keeps a service:64
/// deployment from spawning 64 simultaneous engine evaluations on a
/// 2-core box, while still amortizing cold start.
pub const MAX_STARTUP_PARALLELISM: usize = 8;

/// Per-worker initialization deadline (ms) under the bounded policy.
pub const WORKER_INIT_DEADLINE_MS: u64 = 10_000;

/// Compute the effective startup parallelism for `workers` on a machine
/// with `cores` logical cores. Deterministic: min(workers, cores, cap),
/// floored at 1.
pub fn startup_parallelism(workers: usize, cores: usize) -> usize {
    workers.min(cores.max(1)).clamp(1, MAX_STARTUP_PARALLELISM)
}

/// The bounded batch plan: how many concurrent "lanes" the startup uses
/// and how many workers each lane initializes (last lane may be short).
/// Sum of lane sizes == workers.
pub fn startup_batches(workers: usize, cores: usize) -> (usize, Vec<usize>) {
    let lanes = startup_parallelism(workers, cores);
    let per_lane = workers / lanes;
    let remainder = workers % lanes;
    let mut sizes = vec![per_lane; lanes];
    for size in sizes.iter_mut().take(remainder) {
        *size += 1;
    }
    (lanes, sizes)
}

/// Replacement policy state (M3-005-C): initializes quarantined-worker
/// replacements under a bounded policy — a cooldown between replacements
/// and a hard replacement budget — so repeated poison cannot create a
/// restart storm.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplacementPolicy {
    /// Replacements performed so far (saturating; the restart metric).
    pub replacements: u64,
    /// Maximum replacements within one budget window.
    pub budget: u64,
    /// Ticks in the replacement budget window.
    pub budget_window_ticks: u64,
    /// Replacements consumed in the current window.
    pub window_used: u64,
    /// Ticks remaining in the current window (0 = reset due).
    pub window_ticks_remaining: u64,
    /// Ticks since the last replacement (cooldown gate; saturating).
    pub ticks_since_replacement: u64,
    /// Ticks required between replacements.
    pub cooldown_ticks: u64,
}

/// Outcome of requesting a replacement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplacementDecision {
    /// Initialize the replacement now.
    Initialize,
    /// Denied: the replacement budget for this window is exhausted.
    /// The worker stays quarantined until the window resets.
    BudgetExhausted,
    /// Denied: the cooldown between replacements has not elapsed.
    /// The worker stays quarantined until the cooldown elapses.
    CoolingDown,
}

impl ReplacementPolicy {
    /// Policy starting point.
    pub fn starting(budget: u64, budget_window_ticks: u64, cooldown_ticks: u64) -> Self {
        ReplacementPolicy {
            replacements: 0,
            budget,
            budget_window_ticks,
            window_used: 0,
            window_ticks_remaining: budget_window_ticks,
            ticks_since_replacement: 0,
            cooldown_ticks,
        }
    }

    /// Advance the window/clock one tick (call once per policy period).
    pub fn tick(&mut self) {
        self.ticks_since_replacement = self.ticks_since_replacement.saturating_add(1);
        // Fixed window: when the window's ticks are consumed, the budget
        // is replenished and a fresh window starts.
        self.window_ticks_remaining = self.window_ticks_remaining.saturating_sub(1);
        if self.window_ticks_remaining == 0 {
            self.window_used = 0;
            self.window_ticks_remaining = self.budget_window_ticks;
        }
    }

    /// Request replacement of a quarantined worker under the bounded
    /// policy. Deterministic: cooldown first, then budget.
    pub fn request_replacement(&mut self) -> ReplacementDecision {
        // Budget gate first: at most `budget` replacements per window.
        if self.window_used >= self.budget {
            return ReplacementDecision::BudgetExhausted;
        }
        // Cooldown gate (skipped before the first replacement ever, and
        // when cooldown_ticks == 0: no cooldown configured).
        if self.cooldown_ticks > 0
            && self.replacements > 0
            && self.ticks_since_replacement <= self.cooldown_ticks
        {
            return ReplacementDecision::CoolingDown;
        }
        self.replacements = self.replacements.saturating_add(1);
        self.window_used = self.window_used.saturating_add(1);
        self.ticks_since_replacement = 0;
        ReplacementDecision::Initialize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serverless_starts_exactly_one_worker() {
        // The M3-003-A guarantee, independent of any config value.
        assert_eq!(ServiceProfile::Serverless.initial_workers(), 1);
        assert_eq!(ServiceProfile::default(), ServiceProfile::Serverless);
        assert_eq!(
            ServiceProfile::parse("serverless")
                .unwrap()
                .initial_workers(),
            1
        );
        // Even absurd configuration cannot change it: Serverless carries
        // no worker count at all.
        assert_eq!(
            ServiceProfile::parse("Serverless").unwrap().as_str(),
            "serverless"
        );
    }

    #[test]
    fn service_profile_requires_explicit_count_and_clamps() {
        // Bare "service" fails closed — no hidden default worker count.
        assert!(ServiceProfile::parse("service").is_err());
        assert_eq!(
            ServiceProfile::parse("service:4").unwrap(),
            ServiceProfile::Service { workers: 4 }
        );
        assert_eq!(
            ServiceProfile::parse("service:4")
                .unwrap()
                .initial_workers(),
            4
        );
        // Count outside the bounds fails closed.
        assert!(ServiceProfile::parse("service:0").is_err());
        assert!(ServiceProfile::parse("service:65").is_err());
        // Within bounds: exact.
        assert_eq!(
            ServiceProfile::parse("service:1")
                .unwrap()
                .initial_workers(),
            1
        );
        assert_eq!(
            ServiceProfile::parse("service:64")
                .unwrap()
                .initial_workers(),
            MAX_WORKERS
        );
    }

    #[test]
    fn unknown_profiles_fail_closed() {
        // Parsing is case-INsensitive ("Service:4" == "service:4"); what
        // fails closed is unknown NAMES and out-of-range counts.
        for junk in [
            "",
            "server less",
            "SERVICE",
            "Service:4=extra",
            "auto",
            "workers=4",
            "service:",
        ] {
            assert!(
                ServiceProfile::parse(junk).is_err(),
                "{junk:?} must not parse"
            );
        }
    }

    #[test]
    fn names_round_trip_for_inspect_output() {
        for text in ["serverless", "service:1", "service:8"] {
            let parsed = ServiceProfile::parse(text).unwrap();
            assert_eq!(parsed.as_str(), text);
        }
    }

    #[test]
    fn adaptive_starts_ready_after_worker_zero() {
        // Serverless readiness: worker 0 IS the service. Ready is the
        // initial state — no extra event has to happen first.
        let a = AdaptiveWorkers::starting(8, 0);
        assert!(a.ready);
        assert_eq!(a.running, 1, "serverless starts exactly one worker");
        assert_eq!(a.add_events, 0);
    }

    #[test]
    fn pressure_adds_one_worker_per_tick() {
        let mut a = AdaptiveWorkers::starting(8, 0); // no cooldown
        assert_eq!(a.tick(10, 4), ScaleTick::AddWorker);
        assert_eq!(a.running, 2);
        assert_eq!(a.tick(10, 4), ScaleTick::AddWorker);
        assert_eq!(a.running, 3);
        assert_eq!(a.add_events, 2);
    }

    #[test]
    fn max_workers_bounds_growth_exactly() {
        let mut a = AdaptiveWorkers::starting(3, 0);
        for _ in 0..10 {
            a.tick(100, 4);
        }
        assert_eq!(
            a.running, 3,
            "never exceeds max even under extreme pressure"
        );
        assert_eq!(a.tick(100, 4), ScaleTick::Hold);
    }

    #[test]
    fn cooldown_gates_bursts_against_oscillation() {
        // Cooldown 2: after an add, two more ticks must Hold before the
        // next add is allowed — a burst cannot spawn a burst.
        let mut a = AdaptiveWorkers::starting(8, 2);
        assert_eq!(a.tick(100, 4), ScaleTick::AddWorker);
        assert_eq!(a.tick(100, 4), ScaleTick::Hold); // ticks_since_add=1
        assert_eq!(a.tick(100, 4), ScaleTick::Hold); // ticks_since_add=2
        assert_eq!(a.tick(100, 4), ScaleTick::AddWorker); // 3 > 2
        assert_eq!(a.running, 3, "one add at start + one after cooldown");
    }

    #[test]
    fn below_threshold_pressure_always_holds() {
        let mut a = AdaptiveWorkers::starting(8, 0);
        assert_eq!(
            a.tick(4, 4),
            ScaleTick::Hold,
            "pressure == threshold holds (strictly above adds)"
        );
        assert_eq!(a.tick(0, 4), ScaleTick::Hold);
        assert_eq!(a.running, 1);
    }

    #[test]
    fn serverless_readiness_needs_exactly_worker_zero() {
        let mut r = Readiness::starting(ServiceProfile::Serverless);
        assert_eq!(r.required(), 1);
        assert!(!r.is_ready());
        assert!(
            r.worker_initialized(),
            "ready flips ON the meeting transition"
        );
        assert!(r.is_ready());
        // Extra initializations never re-trigger (returns false) nor un-set.
        assert!(!r.worker_initialized());
        assert!(r.is_ready());
        assert_eq!(r.initialized(), 2);
    }

    #[test]
    fn throughput_readiness_needs_every_configured_worker() {
        let mut r = Readiness::starting(ServiceProfile::Service { workers: 4 });
        assert_eq!(r.required(), 4);
        assert!(!r.worker_initialized(), "1st: not ready");
        assert!(!r.worker_initialized(), "2nd: not ready");
        assert!(!r.worker_initialized(), "3rd: not ready");
        assert!(!r.is_ready(), "partial initialization is never ready");
        assert!(
            r.worker_initialized(),
            "4th call flips readiness and returns true"
        );
        assert!(r.is_ready());
        assert_eq!(r.initialized(), 4);
    }

    #[test]
    fn readiness_is_one_way() {
        let mut r = Readiness::starting(ServiceProfile::Serverless);
        r.worker_initialized();
        assert!(r.is_ready());
        // There is no un-ready API; the flag is structural.
        assert!(r.is_ready());
    }

    #[test]
    fn throughput_out_of_range_fails_closed_direct_variants_clamp() {
        // Parse fails closed outside [1,64].
        assert!(ServiceProfile::parse("service:65").is_err());
        // A directly constructed variant clamps its requirement.
        let mut r = Readiness::starting(ServiceProfile::Service { workers: 65 });
        assert_eq!(r.required(), MAX_WORKERS);
        for _ in 0..64 {
            r.worker_initialized();
        }
        assert!(r.is_ready(), "ready at exactly the clamped requirement");
    }

    #[test]
    fn startup_parallelism_is_bounded_by_cores_and_cap() {
        // More workers than cores: parallelism clamps to cores.
        assert_eq!(startup_parallelism(64, 8), 8);
        // Few workers on many cores: parallelism = worker count.
        assert_eq!(startup_parallelism(2, 16), 2);
        // Cap applies even on huge machines.
        assert_eq!(startup_parallelism(64, 128), MAX_STARTUP_PARALLELISM);
        // Degenerate inputs stay >= 1.
        assert_eq!(startup_parallelism(0, 0), 1);
        assert_eq!(startup_parallelism(1, 0), 1);
    }

    #[test]
    fn startup_batches_sum_exactly_to_workers() {
        for workers in [1usize, 2, 3, 7, 8, 9, 16, 64] {
            for cores in [1usize, 2, 4, 8, 16] {
                let (lanes, sizes) = startup_batches(workers, cores);
                assert_eq!(lanes, startup_parallelism(workers, cores));
                assert_eq!(sizes.len(), lanes);
                assert_eq!(
                    sizes.iter().sum::<usize>(),
                    workers,
                    "workers={workers} cores={cores}"
                );
                // No lane exceeds the cap.
                for size in &sizes {
                    assert!(*size <= workers);
                }
            }
        }
    }

    #[test]
    fn single_worker_startup_is_always_one_lane() {
        // The serverless guarantee: 1 worker -> exactly 1 lane of 1.
        assert_eq!(startup_batches(1, 16), (1, vec![1]));
    }

    #[test]
    fn replacement_initializes_under_budget() {
        let mut p = ReplacementPolicy::starting(3, 10, 0); // no cooldown
        for _ in 0..3 {
            assert_eq!(p.request_replacement(), ReplacementDecision::Initialize);
        }
        // Budget exhausted within the window.
        assert_eq!(
            p.request_replacement(),
            ReplacementDecision::BudgetExhausted
        );
        assert_eq!(p.replacements, 3);
    }

    #[test]
    fn budget_window_resets_after_elapsing() {
        let mut p = ReplacementPolicy::starting(2, 5, 0);
        assert_eq!(p.request_replacement(), ReplacementDecision::Initialize);
        assert_eq!(p.request_replacement(), ReplacementDecision::Initialize);
        assert_eq!(
            p.request_replacement(),
            ReplacementDecision::BudgetExhausted
        );
        // Window ticks past its length -> resets -> replacements allowed.
        for _ in 0..5 {
            p.tick();
        }
        assert_eq!(p.request_replacement(), ReplacementDecision::Initialize);
    }

    #[test]
    fn cooldown_blocks_immediate_re_replacement() {
        let mut p = ReplacementPolicy::starting(10, 100, 3);
        assert_eq!(p.request_replacement(), ReplacementDecision::Initialize);
        // Within cooldown: denied, worker stays quarantined.
        assert_eq!(p.request_replacement(), ReplacementDecision::CoolingDown);
        p.tick();
        assert_eq!(p.request_replacement(), ReplacementDecision::CoolingDown);
        p.tick();
        assert_eq!(p.request_replacement(), ReplacementDecision::CoolingDown);
        p.tick();
        p.tick();
        assert_eq!(p.request_replacement(), ReplacementDecision::Initialize);
    }

    #[test]
    fn restart_storm_scenario_stays_bounded() {
        // 100 rapid poison events against budget 5/window 10 (each
        // iteration = request + tick = 1 tick of window time): exactly
        // 5 replacements per 10-tick window, 10 windows -> exactly 50.
        // Bounded: a storm is rate-limited to budget/window, never
        // 100 replacements.
        let mut p = ReplacementPolicy::starting(5, 10, 0);
        let mut initialized = 0;
        for _ in 0..100 {
            if p.request_replacement() == ReplacementDecision::Initialize {
                initialized += 1;
            }
            p.tick();
        }
        assert_eq!(
            initialized, 50,
            "fixed-window budget: 5 per window x 10 windows"
        );
        assert!(
            initialized < 100,
            "restart storm is rate-limited, never 1:1 with poison events"
        );
    }
}
