//! Native fetch security policy (M28-001-A, ADR-0033) and ingress/
//! outbound trust model (M28-001-B, ADR-0034).
//!
//! Single source of truth for the outbound-fetch trust boundary:
//! scheme allowlist, SSRF address classification (deny-by-default),
//! DNS-rebinding controls (validate-after-resolve, connect-to-validated),
//! redirect revalidation, TLS posture, layered timeouts, compression and
//! body limits; plus the trust model: fetch is a declared capability
//! (`runtime:fetch@1`), outbound policy is runtime-owned, and reverse
//! proxy forwarded headers are never identity. Later M28 packets (stack,
//! pooling, streaming, redirects, address validation, surface) consume
//! this policy — they never re-derive it.

use std::fmt;
use std::net::IpAddr;

/// The only fetchable schemes (ADR-0033 §1). Widening is an ADR decision.
pub const ALLOWED_SCHEMES: [&str; 2] = ["http", "https"];

/// Cloud metadata endpoints denied by explicit name (ADR-0033 §2) —
/// distinct from generic link-local so the error is auditable.
pub const METADATA_ENDPOINTS: [IpAddr; 2] = [
    IpAddr::V4(std::net::Ipv4Addr::new(169, 254, 169, 254)),
    IpAddr::V6(std::net::Ipv6Addr::new(
        0xfd00, 0x0ec2, 0, 0, 0, 0, 0, 0x0235,
    )),
];

/// Outbound fetch is a declared capability under the ADR-0029 identity
/// system (ADR-0034 §1): `runtime:fetch@1`. No ambient global exists for
/// routes without the grant.
pub const FETCH_CAPABILITY_ID: &str = "runtime:fetch";
/// Version of the fetch capability surface shipped in M28.
pub const FETCH_CAPABILITY_VERSION: u32 = 1;

/// Ingress headers that MUST NOT be trusted for identity, authentication,
/// authorization, or scheme decisions (ADR-0034 §3). They are ordinary
/// readable data — nothing more.
pub const UNTRUSTED_FORWARD_HEADERS: [&str; 6] = [
    "x-forwarded-for",
    "x-forwarded-proto",
    "x-forwarded-host",
    "x-forwarded-port",
    "x-forwarded-all",
    "forwarded",
];

/// Total fetch deadline ceiling; matches `MAX_OP_DEADLINE_MS` (ADR-0030).
pub const MAX_FETCH_DEADLINE_MS: u64 = 300_000;
/// Default total fetch deadline.
pub const DEFAULT_FETCH_DEADLINE_MS: u64 = 30_000;
/// Default TCP connect timeout.
pub const DEFAULT_CONNECT_TIMEOUT_MS: u64 = 10_000;
/// Default TLS handshake timeout.
pub const DEFAULT_TLS_TIMEOUT_MS: u64 = 10_000;

/// Request body ceiling (matches the text-encoding buffer bound).
pub const MAX_FETCH_REQUEST_BODY_BYTES: u64 = 16 * 1024 * 1024;
/// Response body ceiling and default cap.
pub const MAX_FETCH_RESPONSE_BODY_BYTES: u64 = 16 * 1024 * 1024;

/// Maximum followed redirect hops (browser-aligned; ADR-0033 §4).
pub const MAX_REDIRECT_HOPS: u32 = 20;

/// Where a fetch may dial. Classification runs on resolved IPs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressClass {
    Public,
    Private,
    Loopback,
    LinkLocal,
    Metadata,
    Unspecified,
    Multicast,
    Reserved,
}

impl fmt::Display for AddressClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            AddressClass::Public => "public",
            AddressClass::Private => "private",
            AddressClass::Loopback => "loopback",
            AddressClass::LinkLocal => "link-local",
            AddressClass::Metadata => "cloud-metadata",
            AddressClass::Unspecified => "unspecified",
            AddressClass::Multicast => "multicast",
            AddressClass::Reserved => "reserved",
        };
        f.write_str(name)
    }
}

impl AddressClass {
    /// Classify a resolved address. IPv4-mapped IPv6 is normalized to its
    /// IPv4 form first — mapped forms must not evade the classifier.
    pub fn classify(addr: IpAddr) -> AddressClass {
        let addr = normalize(addr);
        if METADATA_ENDPOINTS.contains(&addr) {
            return AddressClass::Metadata;
        }
        match addr {
            IpAddr::V4(v4) => {
                if v4.is_loopback() {
                    AddressClass::Loopback
                } else if v4.is_link_local() {
                    AddressClass::LinkLocal
                } else if v4.is_private() {
                    AddressClass::Private
                } else if v4.is_unspecified() {
                    AddressClass::Unspecified
                } else if v4.is_multicast() {
                    AddressClass::Multicast
                } else if v4.is_broadcast()
                    || v4.is_documentation()
                    || v4.octets()[0] & 0xFC == 0x64
                {
                    // 255.255.255.255, 192.0.2.0/24 & friends, 100.64.0.0/10 (CGNAT)
                    AddressClass::Reserved
                } else {
                    AddressClass::Public
                }
            }
            IpAddr::V6(v6) => {
                if v6.is_loopback() {
                    AddressClass::Loopback
                } else if v6.is_unicast_link_local() {
                    AddressClass::LinkLocal
                } else if v6.is_unique_local() {
                    AddressClass::Private
                } else if v6.is_unspecified() {
                    AddressClass::Unspecified
                } else if v6.is_multicast() {
                    AddressClass::Multicast
                } else {
                    // IPv4-mapped forms were normalized above, so any
                    // remaining global-scope IPv6 address is public.
                    AddressClass::Public
                }
            }
        }
    }

    /// May an address of this class be dialed under the given trust mode?
    pub fn dialable(&self, mode: TrustMode) -> bool {
        match mode {
            TrustMode::Default => *self == AddressClass::Public,
            TrustMode::ExplicitLoopbackTesting => {
                matches!(self, AddressClass::Public | AddressClass::Loopback)
            }
        }
    }
}

fn normalize(addr: IpAddr) -> IpAddr {
    match addr {
        IpAddr::V6(v6) => match v6.to_ipv4_mapped() {
            Some(v4) => IpAddr::V4(v4),
            None => addr,
        },
        other => other,
    }
}

/// Trust mode for address dialing. The default has no escape hatch;
/// the explicit loopback mode exists for opt-in local integration tests
/// and is auditable by name at the call site (ADR-0033 §2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TrustMode {
    #[default]
    Default,
    ExplicitLoopbackTesting,
}

/// Redirect handling (ADR-0033 §4).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RedirectPolicy {
    /// Return the 3xx to the caller; follow nothing.
    Manual,
    /// Follow up to `max_hops`, revalidating every hop from scratch.
    Follow { max_hops: u32 },
}

impl Default for RedirectPolicy {
    fn default() -> Self {
        RedirectPolicy::Follow {
            max_hops: MAX_REDIRECT_HOPS,
        }
    }
}

/// Layered timeouts (ADR-0033 §7). One total budget covers all hops.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimeoutPolicy {
    pub total_deadline_ms: u64,
    pub connect_ms: u64,
    pub tls_ms: u64,
}

impl Default for TimeoutPolicy {
    fn default() -> Self {
        TimeoutPolicy {
            total_deadline_ms: DEFAULT_FETCH_DEADLINE_MS,
            connect_ms: DEFAULT_CONNECT_TIMEOUT_MS,
            tls_ms: DEFAULT_TLS_TIMEOUT_MS,
        }
    }
}

/// Compression posture (ADR-0033 §8): off by default; decompression is
/// always bounded by the response body limit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CompressionPolicy {
    #[default]
    Off,
    Gzip {
        enabled: bool,
    },
}

/// Typed policy violations. Closed set; every denial names its reason.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FetchPolicyError {
    SchemeNotAllowed { scheme: String },
    AddressDenied { addr: IpAddr, class: AddressClass },
    HostnameDenied { host: String, reason: String },
    DowngradeRedirect { from: String, to: String },
    TooManyRedirects { max_hops: u32 },
    InvalidRedirectHops { requested: u32, max: u32 },
    InvalidDeadline { ms: u64, max: u64 },
    InvalidBodyLimit { bytes: u64, max: u64 },
}

impl fmt::Display for FetchPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FetchPolicyError::SchemeNotAllowed { scheme } => {
                write!(
                    f,
                    "fetch scheme '{scheme}' not allowed (http/https only, fail closed)"
                )
            }
            FetchPolicyError::AddressDenied { addr, class } => {
                write!(f, "fetch to {addr} denied: {class} address is not dialable under the current trust mode")
            }
            FetchPolicyError::HostnameDenied { host, reason } => {
                write!(f, "fetch host '{host}' denied: {reason}")
            }
            FetchPolicyError::DowngradeRedirect { from, to } => {
                write!(f, "redirect downgrade {from} -> {to} denied")
            }
            FetchPolicyError::TooManyRedirects { max_hops } => {
                write!(f, "redirect chain exceeded {max_hops} hops")
            }
            FetchPolicyError::InvalidRedirectHops { requested, max } => {
                write!(
                    f,
                    "invalid redirect hop limit {requested}; must be 1..={max}"
                )
            }
            FetchPolicyError::InvalidDeadline { ms, max } => {
                write!(
                    f,
                    "fetch deadline {ms}ms is zero or exceeds the {max}ms ceiling"
                )
            }
            FetchPolicyError::InvalidBodyLimit { bytes, max } => {
                write!(
                    f,
                    "fetch body limit {bytes} bytes is zero or exceeds the {max} byte ceiling"
                )
            }
        }
    }
}

impl std::error::Error for FetchPolicyError {}

/// The frozen outbound-fetch policy object (ADR-0033). All M28 fetch
/// paths construct their behavior through this; nothing dials without it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FetchPolicy {
    trust: TrustMode,
    redirect: RedirectPolicy,
    timeouts: TimeoutPolicy,
    compression: CompressionPolicy,
    max_request_body_bytes: u64,
    max_response_body_bytes: u64,
    /// Ambient proxy trust is always disabled (ADR-0033 §5); the field
    /// exists so the posture is queryable, never configurable here.
    ambient_proxy: bool,
}

impl Default for FetchPolicy {
    fn default() -> Self {
        FetchPolicy {
            trust: TrustMode::Default,
            redirect: RedirectPolicy::default(),
            timeouts: TimeoutPolicy::default(),
            compression: CompressionPolicy::Off,
            max_request_body_bytes: MAX_FETCH_REQUEST_BODY_BYTES,
            max_response_body_bytes: MAX_FETCH_RESPONSE_BODY_BYTES,
            ambient_proxy: false,
        }
    }
}

impl FetchPolicy {
    /// Opt-in, auditable loopback trust for local integration tests.
    /// Private, link-local, and metadata addresses remain denied.
    pub fn trusted_loopback_explicit() -> Self {
        FetchPolicy {
            trust: TrustMode::ExplicitLoopbackTesting,
            ..FetchPolicy::default()
        }
    }

    pub fn trust_mode(&self) -> TrustMode {
        self.trust
    }

    pub fn redirect_policy(&self) -> RedirectPolicy {
        self.redirect
    }

    pub fn timeouts(&self) -> TimeoutPolicy {
        self.timeouts
    }

    pub fn compression(&self) -> CompressionPolicy {
        self.compression
    }

    pub fn max_request_body_bytes(&self) -> u64 {
        self.max_request_body_bytes
    }

    pub fn max_response_body_bytes(&self) -> u64 {
        self.max_response_body_bytes
    }

    /// Ambient proxy trust (environment variables) is never enabled.
    pub fn ambient_proxy_enabled(&self) -> bool {
        self.ambient_proxy
    }

    /// Validate a URL scheme (case-insensitive) before any resolution.
    pub fn check_scheme(&self, scheme: &str) -> Result<(), FetchPolicyError> {
        let lower = scheme.to_ascii_lowercase();
        if ALLOWED_SCHEMES.contains(&lower.as_str()) {
            Ok(())
        } else {
            Err(FetchPolicyError::SchemeNotAllowed {
                scheme: scheme.to_string(),
            })
        }
    }

    /// Validate a resolved address against the trust mode.
    pub fn check_address(&self, addr: IpAddr) -> Result<(), FetchPolicyError> {
        let class = AddressClass::classify(addr);
        if class.dialable(self.trust) {
            Ok(())
        } else {
            Err(FetchPolicyError::AddressDenied { addr, class })
        }
    }

    /// Validate every resolved address — one bad record fails the whole
    /// fetch (ADR-0033 §3: no partial retry, no rebinding window).
    pub fn check_resolved(&self, host: &str, addrs: &[IpAddr]) -> Result<(), FetchPolicyError> {
        if addrs.is_empty() {
            return Err(FetchPolicyError::HostnameDenied {
                host: host.to_string(),
                reason: "no addresses resolved".to_string(),
            });
        }
        for &addr in addrs {
            self.check_address(addr)?;
        }
        Ok(())
    }

    /// Revalidate a redirect hop from scratch (ADR-0033 §4): scheme,
    /// downgrade protection. Address revalidation happens through
    /// `check_resolved` on the new host before dialing.
    pub fn check_redirect_hop(
        &self,
        from_scheme: &str,
        to_scheme: &str,
        hop: u32,
    ) -> Result<(), FetchPolicyError> {
        self.check_scheme(to_scheme)?;
        let from = from_scheme.to_ascii_lowercase();
        let to = to_scheme.to_ascii_lowercase();
        if from == "https" && to == "http" {
            return Err(FetchPolicyError::DowngradeRedirect { from, to });
        }
        let max_hops = match self.redirect {
            RedirectPolicy::Manual => 0,
            RedirectPolicy::Follow { max_hops } => max_hops,
        };
        if hop > max_hops {
            return Err(FetchPolicyError::TooManyRedirects { max_hops });
        }
        Ok(())
    }

    /// Validate redirect construction limits (fail closed at build time).
    pub fn validate_redirect_policy(&self) -> Result<(), FetchPolicyError> {
        if let RedirectPolicy::Follow { max_hops } = self.redirect {
            if max_hops == 0 || max_hops > MAX_REDIRECT_HOPS {
                return Err(FetchPolicyError::InvalidRedirectHops {
                    requested: max_hops,
                    max: MAX_REDIRECT_HOPS,
                });
            }
        }
        Ok(())
    }

    /// Validate the layered timeouts (fail closed before any I/O).
    pub fn validate_timeouts(&self) -> Result<(), FetchPolicyError> {
        let t = &self.timeouts;
        if t.total_deadline_ms == 0 || t.total_deadline_ms > MAX_FETCH_DEADLINE_MS {
            return Err(FetchPolicyError::InvalidDeadline {
                ms: t.total_deadline_ms,
                max: MAX_FETCH_DEADLINE_MS,
            });
        }
        if t.connect_ms == 0 || t.connect_ms > t.total_deadline_ms {
            return Err(FetchPolicyError::InvalidDeadline {
                ms: t.connect_ms,
                max: t.total_deadline_ms,
            });
        }
        if t.tls_ms == 0 || t.tls_ms > t.total_deadline_ms {
            return Err(FetchPolicyError::InvalidDeadline {
                ms: t.tls_ms,
                max: t.total_deadline_ms,
            });
        }
        Ok(())
    }

    /// Validate body limits against the compile-time ceilings.
    pub fn validate_body_limits(&self) -> Result<(), FetchPolicyError> {
        if self.max_request_body_bytes == 0
            || self.max_request_body_bytes > MAX_FETCH_REQUEST_BODY_BYTES
        {
            return Err(FetchPolicyError::InvalidBodyLimit {
                bytes: self.max_request_body_bytes,
                max: MAX_FETCH_REQUEST_BODY_BYTES,
            });
        }
        if self.max_response_body_bytes == 0
            || self.max_response_body_bytes > MAX_FETCH_RESPONSE_BODY_BYTES
        {
            return Err(FetchPolicyError::InvalidBodyLimit {
                bytes: self.max_response_body_bytes,
                max: MAX_FETCH_RESPONSE_BODY_BYTES,
            });
        }
        Ok(())
    }

    /// Full construction-time validation: everything fails closed here,
    /// before any DNS or socket work can start.
    pub fn validate(&self) -> Result<(), FetchPolicyError> {
        self.validate_redirect_policy()?;
        self.validate_timeouts()?;
        self.validate_body_limits()?;
        Ok(())
    }
}

/// Is `header` a forwarded/proxy header that must never be trusted for
/// identity or scheme decisions? Case-insensitive (ADR-0034 §3).
pub fn is_untrusted_forward_header(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    UNTRUSTED_FORWARD_HEADERS.contains(&lower.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::IpAddr;

    fn ip(s: &str) -> IpAddr {
        s.parse().expect("test address")
    }

    // --- §1 scheme allowlist -------------------------------------------------

    #[test]
    fn http_and_https_are_the_only_allowed_schemes() {
        let p = FetchPolicy::default();
        assert!(p.check_scheme("http").is_ok());
        assert!(p.check_scheme("https").is_ok());
        assert!(p.check_scheme("HTTP").is_ok()); // case-insensitive
        assert!(p.check_scheme("Https").is_ok());
    }

    #[test]
    fn dangerous_schemes_fail_closed() {
        let p = FetchPolicy::default();
        for scheme in ["file", "data", "ftp", "ws", "wss", "gopher", "unix", ""] {
            let err = p.check_scheme(scheme).unwrap_err();
            assert!(
                matches!(err, FetchPolicyError::SchemeNotAllowed { .. }),
                "{scheme} must be rejected, got {err:?}"
            );
            assert!(err.to_string().contains("fail closed"));
        }
    }

    // --- §2 SSRF classification ----------------------------------------------

    #[test]
    fn public_addresses_are_dialable_by_default() {
        let p = FetchPolicy::default();
        for s in [
            "8.8.8.8",
            "1.1.1.1",
            "93.184.216.34",
            "2001:4860::8888",
            "2606:4700::1111",
        ] {
            let addr = ip(s);
            assert_eq!(AddressClass::classify(addr), AddressClass::Public, "{s}");
            assert!(p.check_address(addr).is_ok(), "{s} must be dialable");
        }
    }

    #[test]
    fn private_ranges_are_denied_by_default() {
        let p = FetchPolicy::default();
        for s in [
            "10.0.0.1",
            "172.16.0.1",
            "172.31.255.255",
            "192.168.1.1",
            "fd00::1",
            "fd12:3456::1",
        ] {
            let addr = ip(s);
            assert_eq!(AddressClass::classify(addr), AddressClass::Private, "{s}");
            let err = p.check_address(addr).unwrap_err();
            assert!(matches!(err, FetchPolicyError::AddressDenied { .. }));
            assert!(err.to_string().contains("private"), "{err}");
        }
    }

    #[test]
    fn loopback_linklocal_and_metadata_are_denied_with_named_classes() {
        let p = FetchPolicy::default();
        for (s, class) in [
            ("127.0.0.1", AddressClass::Loopback),
            ("127.8.8.8", AddressClass::Loopback),
            ("::1", AddressClass::Loopback),
            ("169.254.1.1", AddressClass::LinkLocal),
            ("fe80::1", AddressClass::LinkLocal),
            ("169.254.169.254", AddressClass::Metadata),
            ("fd00:ec2::235", AddressClass::Metadata),
        ] {
            let addr = ip(s);
            assert_eq!(AddressClass::classify(addr), class, "{s}");
            let err = p.check_address(addr).unwrap_err();
            assert!(err.to_string().contains(&class.to_string()), "{err}");
        }
    }

    #[test]
    fn unspecified_multicast_and_reserved_are_denied() {
        let p = FetchPolicy::default();
        for (s, class) in [
            ("0.0.0.0", AddressClass::Unspecified),
            ("::", AddressClass::Unspecified),
            ("224.0.0.1", AddressClass::Multicast),
            ("ff02::1", AddressClass::Multicast),
            ("255.255.255.255", AddressClass::Reserved),
            ("100.64.0.1", AddressClass::Reserved), // CGNAT
            ("192.0.2.1", AddressClass::Reserved),  // TEST-NET
        ] {
            let addr = ip(s);
            assert_eq!(AddressClass::classify(addr), class, "{s}");
            assert!(p.check_address(addr).is_err(), "{s}");
        }
    }

    #[test]
    fn ipv4_mapped_ipv6_cannot_evade_classification() {
        let p = FetchPolicy::default();
        // ::ffff:10.0.0.1 normalizes to the private 10.0.0.1 and is denied.
        let mapped_private = ip("::ffff:10.0.0.1");
        assert_eq!(
            AddressClass::classify(mapped_private),
            AddressClass::Private
        );
        assert!(p.check_address(mapped_private).is_err());
        let mapped_loopback = ip("::ffff:127.0.0.1");
        assert_eq!(
            AddressClass::classify(mapped_loopback),
            AddressClass::Loopback
        );
        // A mapped public address stays public.
        let mapped_public = ip("::ffff:8.8.8.8");
        assert_eq!(AddressClass::classify(mapped_public), AddressClass::Public);
        assert!(p.check_address(mapped_public).is_ok());
    }

    #[test]
    fn explicit_loopback_mode_is_auditable_and_still_blocks_private() {
        let p = FetchPolicy::trusted_loopback_explicit();
        assert_eq!(p.trust_mode(), TrustMode::ExplicitLoopbackTesting);
        assert!(p.check_address(ip("127.0.0.1")).is_ok(), "opt-in loopback");
        // Private, link-local, metadata stay denied even in loopback mode.
        assert!(p.check_address(ip("10.0.0.1")).is_err());
        assert!(p.check_address(ip("169.254.169.254")).is_err());
        assert!(p.check_address(ip("fe80::1")).is_err());
    }

    // --- §3 DNS rebinding -----------------------------------------------------

    #[test]
    fn one_bad_resolved_record_fails_the_whole_fetch() {
        let p = FetchPolicy::default();
        // Rebinding shape: hostname answers one public and one private record.
        let addrs = [ip("93.184.216.34"), ip("10.0.0.7")];
        let err = p.check_resolved("example.com", &addrs).unwrap_err();
        assert!(matches!(err, FetchPolicyError::AddressDenied { .. }));
        // All-public resolution passes.
        assert!(p
            .check_resolved("example.com", &[ip("93.184.216.34")])
            .is_ok());
    }

    #[test]
    fn empty_resolution_fails_closed() {
        let p = FetchPolicy::default();
        let err = p.check_resolved("example.com", &[]).unwrap_err();
        assert!(matches!(err, FetchPolicyError::HostnameDenied { .. }));
    }

    // --- §4 redirects ----------------------------------------------------------

    #[test]
    fn redirect_revalidation_rejects_scheme_and_downgrade_and_hops() {
        let p = FetchPolicy::default();
        // Same-scheme hop within budget is fine.
        assert!(p.check_redirect_hop("https", "https", 1).is_ok());
        assert!(p.check_redirect_hop("https", "http", 1).is_err()); // downgrade
        assert!(p.check_redirect_hop("http", "file", 1).is_err()); // scheme revalidated
                                                                   // Hop beyond the bound fails.
        let err = p
            .check_redirect_hop("https", "https", MAX_REDIRECT_HOPS + 1)
            .unwrap_err();
        assert!(matches!(err, FetchPolicyError::TooManyRedirects { .. }));
    }

    #[test]
    fn manual_policy_follows_nothing() {
        let p = FetchPolicy {
            redirect: RedirectPolicy::Manual,
            ..FetchPolicy::default()
        };
        assert!(p.check_redirect_hop("https", "https", 1).is_err());
    }

    #[test]
    fn invalid_hop_limits_fail_at_construction() {
        for bad in [0u32, MAX_REDIRECT_HOPS + 1] {
            let p = FetchPolicy {
                redirect: RedirectPolicy::Follow { max_hops: bad },
                ..FetchPolicy::default()
            };
            let err = p.validate().unwrap_err();
            assert!(matches!(err, FetchPolicyError::InvalidRedirectHops { .. }));
        }
    }

    // --- §5 proxy ----------------------------------------------------------------

    #[test]
    fn ambient_proxy_trust_is_never_enabled() {
        assert!(!FetchPolicy::default().ambient_proxy_enabled());
        assert!(!FetchPolicy::trusted_loopback_explicit().ambient_proxy_enabled());
    }

    // --- §7 timeouts --------------------------------------------------------------

    #[test]
    fn default_timeouts_are_bounded_and_valid() {
        let p = FetchPolicy::default();
        assert!(p.validate_timeouts().is_ok());
        assert_eq!(p.timeouts().total_deadline_ms, DEFAULT_FETCH_DEADLINE_MS);
        assert!(p.timeouts().total_deadline_ms <= MAX_FETCH_DEADLINE_MS);
    }

    #[test]
    fn zero_or_over_ceiling_deadlines_fail_closed() {
        for bad_ms in [0u64, MAX_FETCH_DEADLINE_MS + 1, u64::MAX] {
            let p = FetchPolicy {
                timeouts: TimeoutPolicy {
                    total_deadline_ms: bad_ms,
                    ..TimeoutPolicy::default()
                },
                ..FetchPolicy::default()
            };
            let err = p.validate().unwrap_err();
            assert!(matches!(err, FetchPolicyError::InvalidDeadline { .. }));
        }
    }

    #[test]
    fn connect_and_tls_timeouts_cannot_exceed_total_budget() {
        let p = FetchPolicy {
            timeouts: TimeoutPolicy {
                total_deadline_ms: 10_000,
                connect_ms: 20_000, // > total
                tls_ms: 5_000,
            },
            ..FetchPolicy::default()
        };
        assert!(matches!(
            p.validate().unwrap_err(),
            FetchPolicyError::InvalidDeadline { .. }
        ));
    }

    // --- §8/§9 compression and bodies ------------------------------------------

    #[test]
    fn compression_is_off_by_default_and_bounded_decompression_is_pinned() {
        let p = FetchPolicy::default();
        assert_eq!(p.compression(), CompressionPolicy::Off);
        // Decompression budget is the response body limit (ADR-0033 §8).
        assert_eq!(p.max_response_body_bytes(), MAX_FETCH_RESPONSE_BODY_BYTES);
    }

    #[test]
    fn body_limits_reject_zero_and_over_ceiling() {
        for (req, resp) in [
            (0u64, MAX_FETCH_RESPONSE_BODY_BYTES),
            (
                MAX_FETCH_REQUEST_BODY_BYTES + 1,
                MAX_FETCH_RESPONSE_BODY_BYTES,
            ),
            (MAX_FETCH_REQUEST_BODY_BYTES, 0),
            (
                MAX_FETCH_REQUEST_BODY_BYTES,
                MAX_FETCH_RESPONSE_BODY_BYTES + 1,
            ),
        ] {
            let p = FetchPolicy {
                max_request_body_bytes: req,
                max_response_body_bytes: resp,
                ..FetchPolicy::default()
            };
            assert!(matches!(
                p.validate().unwrap_err(),
                FetchPolicyError::InvalidBodyLimit { .. }
            ));
        }
        // Legal limits pass, including tightened ones.
        let tight = FetchPolicy {
            max_request_body_bytes: 1024,
            max_response_body_bytes: 4096,
            ..FetchPolicy::default()
        };
        assert!(tight.validate().is_ok());
    }

    // --- full matrix ------------------------------------------------------------

    #[test]
    fn default_policy_passes_full_validation() {
        assert!(FetchPolicy::default().validate().is_ok());
        assert!(FetchPolicy::trusted_loopback_explicit().validate().is_ok());
    }

    // --- ADR-0034 trust model (M28-001-B) ------------------------------------

    #[test]
    fn fetch_is_a_declared_capability_under_the_identity_system() {
        // The identity must parse under the closed runtime: namespace.
        let id = crate::identity::CapabilityId::parse(FETCH_CAPABILITY_ID)
            .expect("fetch capability id is valid");
        assert_eq!(id.as_str(), "runtime:fetch");
        assert_eq!(FETCH_CAPABILITY_VERSION, 1);
    }

    #[test]
    fn forwarded_headers_are_never_trusted_identity() {
        for name in [
            "X-Forwarded-For",
            "x-forwarded-for",
            "X-FORWARDED-PROTO",
            "X-Forwarded-Host",
            "x-forwarded-port",
            "X-Forwarded-All",
            "Forwarded",
            "forwarded",
        ] {
            assert!(
                is_untrusted_forward_header(name),
                "{name} must be flagged untrusted"
            );
        }
        // Ordinary headers (even proxy-adjacent ones) are not in the list.
        assert!(!is_untrusted_forward_header("Authorization"));
        assert!(!is_untrusted_forward_header("Content-Type"));
        assert!(!is_untrusted_forward_header("X-Request-Id"));
    }

    #[test]
    fn outbound_trust_is_runtime_owned_not_application_owned() {
        // No JS-facing widening exists: the default policy is the only
        // production surface, and its trust mode has no escape hatch
        // beyond the auditable loopback constructor (pinned above).
        let p = FetchPolicy::default();
        assert_eq!(p.trust_mode(), TrustMode::Default);
        // Ambient proxy trust stays off regardless of construction path.
        assert!(!p.ambient_proxy_enabled());
    }

    #[test]
    fn host_header_never_participates_in_policy_decisions() {
        // Routing authority is method+path; the policy surface exposes no
        // host-based branching. Structural pin: every policy method that
        // takes a host treats it as data (resolution input), never as a
        // routing/identity grant. The absence of any host-trust API is
        // the assertion — checked by compiling this against the surface.
        let p = FetchPolicy::default();
        // A host header value used as a fetch destination still goes
        // through the full address policy — no trust shortcut exists.
        assert!(p
            .check_resolved("evil.example", &[ip("169.254.169.254")])
            .is_err());
        assert!(p
            .check_resolved("example.com", &[ip("93.184.216.34")])
            .is_ok());
    }
}
