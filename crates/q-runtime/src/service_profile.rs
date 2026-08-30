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
}
