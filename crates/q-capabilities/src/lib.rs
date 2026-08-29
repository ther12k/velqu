//! Capability ABI lifecycle state machine (ADR-0028, M27-001-A).
//!
//! A native capability moves through a closed set of phases. The
//! transition table is normative: work may start only in `Ready`,
//! `Failed` and `Quiesced` are terminal, and every illegal transition is
//! a typed error — never a panic, never a silent no-op. Detailed
//! identity/version rules (M27-001-B), per-operation owner/deadline
//! state (M27-001-C), and bounded-shutdown semantics (M27-001-D) build
//! on this skeleton without changing its phase vocabulary. Identity,
//! versioning, and requirements live in [`identity`] (ADR-0029,
//! M27-001-B).
//!
//! # Trust model
//!
//! Same-process QuickJS runs **trusted application code only**; it is
//! never a hostile-code sandbox (ADR-0035, AGENTS.md constraint 14).
//! The security policy in [`fetch_policy`] addresses the network, not
//! the process interior.

pub mod abort;
pub mod compat;
pub mod console;
pub mod crypto;
pub mod diagnostics;
pub mod fetch_policy;
pub mod harness;
pub mod identity;
pub mod inventory;
pub mod operations;
pub mod resolver;
pub mod sdk;
pub mod shutdown;
pub mod stream_buffer;
pub mod text_encoding;
pub mod url_model;

pub use abort::{AbortControllerModel, AbortSignalModel, SignalState};
pub use compat::{
    classify, CompatError, Compatibility, PackedSemVer, VersionSelector, MAX_SEMVER_COMPONENT,
    SDK_ABI_REVISION,
};
pub use console::{
    redact_sensitive_text, BoundedLogSink, ConsoleLevel, ConsoleRecord, LogSinkStats,
    DEFAULT_LOG_SINK_CAP, MAX_CONSOLE_ARGS, MAX_CONSOLE_MSG_LEN,
};
pub use crypto::{CryptoError, CryptoRandom, MAX_RANDOM_BYTES_LEN};
pub use diagnostics::{CapabilityDiagnostic, CapabilityDiagnostics, DiagnosticsError};
pub use fetch_policy::{
    headers_surviving_redirect, is_credential_header, is_cross_origin_redirect,
    is_metadata_hostname, resolve_and_validate, url_origin, DecompressionGuard, RedirectLimiter,
    RedirectOutcome, CREDENTIAL_REDIRECT_HEADERS, DECOMPRESSION_RATIO_THRESHOLD,
    HOSTNAME_METADATA_ENDPOINTS, MAX_BODY_HELPER_BYTES, MAX_DECOMPRESSION_RATIO,
};
pub use fetch_policy::{
    is_untrusted_forward_header, AddressClass, CompressionPolicy, FetchPolicy, FetchPolicyError,
    RedirectPolicy, TimeoutPolicy, TrustMode, ALLOWED_SCHEMES, FETCH_CAPABILITY_ID,
    FETCH_CAPABILITY_VERSION, MAX_FETCH_DEADLINE_MS, MAX_FETCH_REQUEST_BODY_BYTES,
    MAX_FETCH_RESPONSE_BODY_BYTES, MAX_REDIRECT_HOPS, METADATA_ENDPOINTS, TRUSTED_CODE_ASSUMPTION,
    UNTRUSTED_FORWARD_HEADERS,
};
pub use harness::{run_expired_drain, run_full_lifecycle, LifecycleReport};
pub use identity::{
    resolve_and_install, resolve_requirement, CapabilityDescriptor, CapabilityId,
    CapabilityIdError, CapabilityRequirement, CapabilityVersion, InstallError, ResolveError,
};
pub use inventory::{CapabilityInventory, InventoryEntry, InventoryError};
pub use operations::{CancellationClass, NativeOp, OpError, OpOwner, OpState, MAX_OP_DEADLINE_MS};
pub use resolver::{resolve_closure, DependencyDag};
pub use shutdown::{
    begin_shutdown, drain_step, finish_shutdown, DrainOutcome, ShutdownError, SHUTDOWN_BUDGET_MS,
};
pub use stream_buffer::{
    BoundedStream, StreamError, DEFAULT_STREAM_BUFFER_BYTES, MAX_STREAM_CHUNK_BYTES,
};
pub use text_encoding::{
    TextDecoderModel, TextDecoderOptions, TextEncoderModel, TextEncodingError, MAX_TEXT_BUFFER_LEN,
};
pub use url_model::{
    decode_path_segment, encode_path_segment, normalize_host, ParsedSearchParams, ParsedUrl,
    UrlError, MAX_SEARCH_PARAMS_COUNT, MAX_SEARCH_PARAMS_LEN, MAX_URL_LEN, MAX_URL_PATH_SEGMENTS,
};

use std::fmt;

/// Closed phase vocabulary for a linked capability module.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapabilityPhase {
    /// Declared in the pack manifest; nothing is linked yet.
    Declared,
    /// Host linked the module and its resolution succeeded (version
    /// conflicts fail before ready — they land in `Failed`, never in
    /// `Ready`).
    Installed,
    /// Serving: the only phase in which operations may start.
    Ready,
    /// Shutdown began: no new operations, in-flight work settles.
    Draining,
    /// All operations settled; safe to drop. Terminal.
    Quiesced,
    /// Terminal failure with a reason. Terminal.
    Failed,
}

impl fmt::Display for CapabilityPhase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            CapabilityPhase::Declared => "declared",
            CapabilityPhase::Installed => "installed",
            CapabilityPhase::Ready => "ready",
            CapabilityPhase::Draining => "draining",
            CapabilityPhase::Quiesced => "quiesced",
            CapabilityPhase::Failed => "failed",
        })
    }
}

/// Typed lifecycle violations. Closed set; no stringly-typed errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecycleError {
    /// The transition is not legal from the current phase.
    IllegalTransition {
        from: CapabilityPhase,
        to: CapabilityPhase,
    },
    /// An operation start was attempted outside `Ready`.
    OpsOutsideReady { from: CapabilityPhase },
    /// The phase is terminal; no further transitions exist.
    Terminal { from: CapabilityPhase },
}

impl fmt::Display for LifecycleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LifecycleError::IllegalTransition { from, to } => {
                write!(f, "illegal capability transition {from} -> {to}")
            }
            LifecycleError::OpsOutsideReady { from } => {
                write!(
                    f,
                    "capability operations may only start in ready, not in {from}"
                )
            }
            LifecycleError::Terminal { from } => {
                write!(f, "capability phase {from} is terminal")
            }
        }
    }
}

impl std::error::Error for LifecycleError {}

/// One capability's lifecycle state. Transitions return the new phase
/// or a typed error; they never mutate on failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapabilityLifecycle {
    phase: CapabilityPhase,
}

impl CapabilityLifecycle {
    /// New lifecycle at `Declared` — the state a manifest entry has
    /// before the host links anything.
    pub fn declared() -> Self {
        CapabilityLifecycle {
            phase: CapabilityPhase::Declared,
        }
    }

    pub fn phase(&self) -> CapabilityPhase {
        self.phase
    }

    fn is_terminal(&self) -> bool {
        matches!(
            self.phase,
            CapabilityPhase::Quiesced | CapabilityPhase::Failed
        )
    }

    /// Guard for guardrail 1: no capability can start work outside the
    /// allowed phase. Only `Ready` admits operation starts.
    pub fn can_start_ops(&self) -> bool {
        self.phase == CapabilityPhase::Ready
    }

    /// `Declared -> Installed`. Linking succeeded (resolution/version
    /// checks passed — conflicts must have been routed to `fail`
    /// instead, so they never reach `Ready`).
    pub fn install(&mut self) -> Result<CapabilityPhase, LifecycleError> {
        self.transition(CapabilityPhase::Installed)
    }

    /// `Installed -> Ready`. First demand may lazily initialize here;
    /// nothing initializes at build or pack load (G-004).
    pub fn activate(&mut self) -> Result<CapabilityPhase, LifecycleError> {
        self.transition(CapabilityPhase::Ready)
    }

    /// `Ready -> Draining`. No new operations after this point.
    pub fn begin_drain(&mut self) -> Result<CapabilityPhase, LifecycleError> {
        self.transition(CapabilityPhase::Draining)
    }

    /// `Draining -> Quiesced`. All operations settled (cancelled or
    /// explicitly non-cancellable and completed) within the shutdown
    /// deadline. Missing the deadline fails closed via `fail`.
    pub fn quiesce(&mut self) -> Result<CapabilityPhase, LifecycleError> {
        self.transition(CapabilityPhase::Quiesced)
    }

    /// Any non-terminal phase -> `Failed`, with the cause recorded by
    /// the caller. Version conflicts at install, drain-deadline
    /// expiry, and init errors all land here — before `Ready` when the
    /// failure precedes activation.
    pub fn fail(&mut self) -> Result<CapabilityPhase, LifecycleError> {
        if self.is_terminal() {
            return Err(LifecycleError::Terminal { from: self.phase });
        }
        self.phase = CapabilityPhase::Failed;
        Ok(self.phase)
    }

    /// Operation-start guard. Fails typed outside `Ready`.
    pub fn start_op(&mut self) -> Result<(), LifecycleError> {
        if self.can_start_ops() {
            Ok(())
        } else {
            Err(LifecycleError::OpsOutsideReady { from: self.phase })
        }
    }

    fn transition(&mut self, to: CapabilityPhase) -> Result<CapabilityPhase, LifecycleError> {
        if self.is_terminal() {
            return Err(LifecycleError::Terminal { from: self.phase });
        }
        let legal = matches!(
            (self.phase, to),
            (CapabilityPhase::Declared, CapabilityPhase::Installed)
                | (CapabilityPhase::Installed, CapabilityPhase::Ready)
                | (CapabilityPhase::Ready, CapabilityPhase::Draining)
                | (CapabilityPhase::Draining, CapabilityPhase::Quiesced)
        );
        if !legal {
            return Err(LifecycleError::IllegalTransition {
                from: self.phase,
                to,
            });
        }
        self.phase = to;
        Ok(self.phase)
    }
}

impl Default for CapabilityLifecycle {
    fn default() -> Self {
        CapabilityLifecycle::declared()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn phases() -> [CapabilityPhase; 6] {
        [
            CapabilityPhase::Declared,
            CapabilityPhase::Installed,
            CapabilityPhase::Ready,
            CapabilityPhase::Draining,
            CapabilityPhase::Quiesced,
            CapabilityPhase::Failed,
        ]
    }

    #[test]
    fn happy_path_declared_to_quiesced() {
        let mut lc = CapabilityLifecycle::declared();
        assert_eq!(lc.install(), Ok(CapabilityPhase::Installed));
        assert_eq!(lc.activate(), Ok(CapabilityPhase::Ready));
        assert_eq!(lc.begin_drain(), Ok(CapabilityPhase::Draining));
        assert_eq!(lc.quiesce(), Ok(CapabilityPhase::Quiesced));
    }

    #[test]
    fn ops_start_only_in_ready() {
        for phase in phases() {
            let mut lc = CapabilityLifecycle::declared();
            lc.phase = phase;
            assert_eq!(
                lc.start_op(),
                if phase == CapabilityPhase::Ready {
                    Ok(())
                } else {
                    Err(LifecycleError::OpsOutsideReady { from: phase })
                },
                "phase {phase}"
            );
        }
    }

    #[test]
    fn illegal_transitions_reject_without_mutation() {
        for from in phases() {
            for to in phases() {
                let legal = matches!(
                    (from, to),
                    (CapabilityPhase::Declared, CapabilityPhase::Installed)
                        | (CapabilityPhase::Installed, CapabilityPhase::Ready)
                        | (CapabilityPhase::Ready, CapabilityPhase::Draining)
                        | (CapabilityPhase::Draining, CapabilityPhase::Quiesced)
                );
                let mut lc = CapabilityLifecycle::declared();
                lc.phase = from;
                let before = lc.phase;
                match lc.transition(to) {
                    Ok(p) => assert!(legal && p == to, "unexpected {from} -> {to} accepted"),
                    Err(LifecycleError::IllegalTransition { from: f, to: t }) => {
                        assert!(!legal, "legal {from} -> {to} rejected");
                        assert_eq!((f, t), (from, to));
                    }
                    Err(LifecycleError::Terminal { .. }) => {
                        assert!(
                            matches!(from, CapabilityPhase::Quiesced | CapabilityPhase::Failed),
                            "terminal error for non-terminal {from} -> {to}"
                        );
                    }
                    Err(other) => panic!("wrong error for {from} -> {to}: {other:?}"),
                }
                if !legal {
                    assert_eq!(lc.phase, before, "failed transition mutated state");
                }
            }
        }
    }

    #[test]
    fn terminal_phases_reject_everything() {
        for phase in [CapabilityPhase::Quiesced, CapabilityPhase::Failed] {
            let mut lc = CapabilityLifecycle::declared();
            lc.phase = phase;
            for to in phases() {
                assert_eq!(
                    lc.transition(to),
                    Err(LifecycleError::Terminal { from: phase }),
                    "terminal {phase} allowed -> {to}"
                );
            }
            assert_eq!(lc.fail(), Err(LifecycleError::Terminal { from: phase }));
            assert_eq!(
                lc.start_op(),
                Err(LifecycleError::OpsOutsideReady { from: phase })
            );
        }
    }

    #[test]
    fn fail_is_reachable_from_every_non_terminal_phase() {
        for phase in phases() {
            let mut lc = CapabilityLifecycle::declared();
            lc.phase = phase;
            let res = lc.fail();
            if phase == CapabilityPhase::Quiesced || phase == CapabilityPhase::Failed {
                assert_eq!(res, Err(LifecycleError::Terminal { from: phase }));
            } else {
                assert_eq!(res, Ok(CapabilityPhase::Failed));
                assert_eq!(
                    lc.activate(),
                    Err(LifecycleError::Terminal {
                        from: CapabilityPhase::Failed
                    })
                );
            }
        }
    }

    #[test]
    fn drain_requires_ready_no_shortcut_from_installed() {
        let mut lc = CapabilityLifecycle::declared();
        lc.install().unwrap();
        assert_eq!(
            lc.begin_drain(),
            Err(LifecycleError::IllegalTransition {
                from: CapabilityPhase::Installed,
                to: CapabilityPhase::Draining
            })
        );
        assert_eq!(
            lc.quiesce(),
            Err(LifecycleError::IllegalTransition {
                from: CapabilityPhase::Installed,
                to: CapabilityPhase::Quiesced
            })
        );
    }

    #[test]
    fn version_conflict_fails_before_ready() {
        // install-time resolution conflicts route to fail(): the
        // capability must never observe Ready after a conflict
        let mut lc = CapabilityLifecycle::declared();
        assert_eq!(lc.install(), Ok(CapabilityPhase::Installed));
        // conflict discovered during linking: host fails the capability
        assert_eq!(lc.fail(), Ok(CapabilityPhase::Failed));
        assert!(!lc.can_start_ops());
        assert_eq!(
            lc.start_op(),
            Err(LifecycleError::OpsOutsideReady {
                from: CapabilityPhase::Failed
            })
        );
    }
}
