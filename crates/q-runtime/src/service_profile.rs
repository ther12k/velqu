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
}
