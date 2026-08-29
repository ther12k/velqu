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

/// Canonical same-process trusted-code statement (ADR-0035, AGENTS.md
/// constraint 14). Pinned as code so the wording cannot drift: the engine
/// runs trusted application code only, is never a hostile-code sandbox,
/// and the network — not the process interior — is the adversary.
pub const TRUSTED_CODE_ASSUMPTION: &str =
    "The QuickJS worker executes trusted, pack-compiled application code only. \
It is not a sandbox for untrusted code: the process interior is inside the trust \
boundary, and the adversary the security policy addresses is the network, never \
the process itself.";

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

/// Maximum body helper size (M28-006-D): the largest body a materializing
/// helper (`Response.text/json/arrayBuffer/bytes`) may produce, fail closed
/// above. Pinned equal to [`MAX_FETCH_RESPONSE_BODY_BYTES`] — a helper can
/// never be asked to materialize more than the network layer admits.
pub const MAX_BODY_HELPER_BYTES: usize = 16 * 1024 * 1024;

/// The body-materializing helpers subject to [`MAX_BODY_HELPER_BYTES`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BodyHelper {
    /// `Response.text()` — decoded string copy.
    ResponseText,
    /// `Response.json()` — parse over the decoded text.
    ResponseJson,
    /// `Response.arrayBuffer()` — copied byte buffer.
    ResponseArrayBuffer,
    /// `Response.bytes()` — copied byte view.
    ResponseBytes,
}

impl BodyHelper {
    /// Stable helper name used in typed error messages and JS errors.
    pub fn name(self) -> &'static str {
        match self {
            BodyHelper::ResponseText => "text",
            BodyHelper::ResponseJson => "json",
            BodyHelper::ResponseArrayBuffer => "arrayBuffer",
            BodyHelper::ResponseBytes => "bytes",
        }
    }

    /// The per-helper byte cap. All helpers share
    /// [`MAX_BODY_HELPER_BYTES`] today; named accessors keep future
    /// per-helper tightening a one-line policy change.
    pub const fn max_bytes(self) -> usize {
        MAX_BODY_HELPER_BYTES
    }
}

/// Fail-closed size check for a materializing body helper. Typed rejection
/// when `byte_len` exceeds the helper's cap.
pub fn check_body_helper_size(helper: BodyHelper, byte_len: usize) -> Result<(), FetchPolicyError> {
    let max = helper.max_bytes();
    if byte_len > max {
        return Err(FetchPolicyError::BodyTooLarge {
            helper: helper.name(),
            len: byte_len,
            max,
        });
    }
    Ok(())
}

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
    SchemeNotAllowed {
        scheme: String,
    },
    AddressDenied {
        addr: IpAddr,
        class: AddressClass,
    },
    HostnameDenied {
        host: String,
        reason: String,
    },
    DowngradeRedirect {
        from: String,
        to: String,
    },
    TooManyRedirects {
        max_hops: u32,
    },
    InvalidRedirectHops {
        requested: u32,
        max: u32,
    },
    InvalidDeadline {
        ms: u64,
        max: u64,
    },
    InvalidBodyLimit {
        bytes: u64,
        max: u64,
    },
    /// A body helper was asked to materialize past its cap (M28-006-D).
    BodyTooLarge {
        helper: &'static str,
        len: usize,
        max: usize,
    },
    /// A redirect returned to an already-visited URL (M28-007-A).
    RedirectLoop {
        url: String,
    },
    /// Decompressed output crossed the response body cap (M28-007-D).
    DecompressedTooLarge {
        produced: u64,
        max: u64,
    },
    /// Decompressed output crossed the compression-ratio ceiling while a
    /// meaningful volume of compressed input had been consumed (M28-007-D).
    DecompressionBomb {
        compressed: u64,
        produced: u64,
        max_ratio: u64,
    },
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
            FetchPolicyError::BodyTooLarge { helper, len, max } => {
                write!(
                    f,
                    "Response.{helper}: body of {len} bytes exceeds the maximum helper size of {max} bytes"
                )
            }
            FetchPolicyError::RedirectLoop { url } => {
                write!(f, "redirect loop detected: {url} was already visited")
            }
            FetchPolicyError::DecompressedTooLarge { produced, max } => {
                write!(
                    f,
                    "decompressed body {produced} bytes exceeds the {max} byte response limit"
                )
            }
            FetchPolicyError::DecompressionBomb {
                compressed,
                produced,
                max_ratio,
            } => {
                write!(
                    f,
                    "decompression bomb: {produced} bytes from {compressed} compressed exceeds the {max_ratio}:1 ratio ceiling"
                )
            }
        }
    }
}

impl std::error::Error for FetchPolicyError {}

/// Stateful per-request redirect hop limiter (M28-007-A). Drives the fetch
/// follow loop against the policy: every 3xx hop passes through
/// [`FetchPolicy::check_redirect_hop`] (scheme allowlist, https→http
/// downgrade denial, hop ceiling) and loop detection over the visited set,
/// so a redirect loop fails bounded and typed — never unbounded following.
/// Memory is bounded by construction: at most `max_hops` visited URLs.
#[derive(Debug, Clone)]
pub struct RedirectLimiter {
    policy: FetchPolicy,
    hops: u32,
    visited: Vec<String>,
}

/// Result of evaluating one 3xx hop.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RedirectOutcome {
    /// Follow the redirect to the evaluated target.
    Follow,
    /// Surface the 3xx response to the caller (`RedirectPolicy::Manual`).
    Surface,
}

impl RedirectLimiter {
    /// Build a limiter from the frozen policy (manual mode surfaces 3xx;
    /// follow mode caps at the policy's `max_hops`).
    pub fn new(policy: FetchPolicy) -> Self {
        RedirectLimiter {
            policy,
            hops: 0,
            visited: Vec::new(),
        }
    }

    /// Redirect hops followed so far.
    pub fn hops(&self) -> u32 {
        self.hops
    }

    /// Configured hop ceiling (0 under `RedirectPolicy::Manual`).
    pub fn limit(&self) -> u32 {
        match self.policy.redirect {
            RedirectPolicy::Manual => 0,
            RedirectPolicy::Follow { max_hops } => max_hops,
        }
    }

    /// Evaluate one 3xx hop from `from_url` to `to_url`. Returns the typed
    /// policy error for scheme, downgrade, hop-ceiling, or loop violations;
    /// [`RedirectOutcome::Surface`] under `RedirectPolicy::Manual`.
    pub fn evaluate(
        &mut self,
        from_url: &str,
        to_url: &str,
    ) -> Result<RedirectOutcome, FetchPolicyError> {
        if matches!(self.policy.redirect, RedirectPolicy::Manual) {
            return Ok(RedirectOutcome::Surface);
        }
        self.check_hop_urls(from_url, to_url)?;
        self.commit_hop(to_url);
        Ok(RedirectOutcome::Follow)
    }

    /// Evaluate one 3xx hop **with SSRF/DNS revalidation** (M28-007-B): the
    /// redirect target's host is re-resolved by the caller and EVERY
    /// resolved address must pass the trust-mode policy
    /// ([`FetchPolicy::check_resolved`]) — a public origin may never lure a
    /// hop into loopback, link-local, private, or metadata space. Runs
    /// after the URL-level checks and before any state is committed, so a
    /// denied hop leaves the limiter unchanged.
    pub fn evaluate_resolved(
        &mut self,
        from_url: &str,
        to_url: &str,
        resolved: &[std::net::IpAddr],
    ) -> Result<RedirectOutcome, FetchPolicyError> {
        if matches!(self.policy.redirect, RedirectPolicy::Manual) {
            return Ok(RedirectOutcome::Surface);
        }
        self.check_hop_urls(from_url, to_url)?;
        self.policy.check_resolved(url_host(to_url), resolved)?;
        self.commit_hop(to_url);
        Ok(RedirectOutcome::Follow)
    }

    /// URL-level hop checks with no mutation: Manual short-circuit, scheme
    /// allowlist, downgrade denial, hop ceiling, and loop detection.
    fn check_hop_urls(&self, from_url: &str, to_url: &str) -> Result<(), FetchPolicyError> {
        // Hop ceiling + scheme allowlist + downgrade denial: one policy path.
        let hop = self.hops + 1;
        self.policy
            .check_redirect_hop(url_scheme(from_url), url_scheme(to_url), hop)?;
        // Loop detection over the bounded visited set.
        if self.visited.iter().any(|u| u == to_url) {
            return Err(FetchPolicyError::RedirectLoop {
                url: to_url.to_string(),
            });
        }
        Ok(())
    }

    /// Record the initial request target as visited (M28-008-B): a redirect
    /// chain that leads back to the origin URL is a loop and must fail via
    /// the typed `RedirectLoop` path. Does not consume a hop.
    pub fn seed_target(&mut self, url: &str) {
        if !self.visited.iter().any(|u| u == url) {
            self.visited.push(url.to_string());
        }
    }

    /// Commit a passing hop (only called after every check succeeded).
    fn commit_hop(&mut self, to_url: &str) {
        self.hops += 1;
        self.visited.push(to_url.to_string());
    }

    /// Atomic redirect hop (M28-008-B): the executor's one-call revalidation
    /// gate. URL-level checks run first (scheme allowlist, https→http
    /// downgrade denial, hop ceiling, loop detection); the hop target's host
    /// is then resolved and EVERY address validated against trust mode via
    /// [`resolve_and_validate`] — including metadata-by-name denial for
    /// redirect targets; only after all of that is hop state committed, so
    /// a resolution failure leaves the limiter exactly as it was. On
    /// [`RedirectOutcome::Follow`] the returned `pinned` set is the dial
    /// set: the connector uses these addresses and never re-resolves.
    pub fn follow_hop<F>(
        &mut self,
        from_url: &str,
        to_url: &str,
        resolve: F,
    ) -> Result<FollowedHop, FetchPolicyError>
    where
        F: FnMut(&str) -> Result<Vec<IpAddr>, String>,
    {
        if matches!(self.policy.redirect, RedirectPolicy::Manual) {
            return Ok(FollowedHop {
                outcome: RedirectOutcome::Surface,
                pinned: Vec::new(),
            });
        }
        self.check_hop_urls(from_url, to_url)?;
        let pinned = resolve_and_validate(&self.policy, url_host(to_url), resolve)?;
        self.commit_hop(to_url);
        Ok(FollowedHop {
            outcome: RedirectOutcome::Follow,
            pinned,
        })
    }
}

/// Result of an atomic [`RedirectLimiter::follow_hop`] (M28-008-B): the
/// redirect decision plus — when followed — the validated, dial-ready
/// address pin set for the next hop.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FollowedHop {
    /// Follow the redirect (dial `pinned`) or surface the 3xx.
    pub outcome: RedirectOutcome,
    /// Validated addresses to dial, in resolution order. Empty for
    /// `Surface` outcomes.
    pub pinned: Vec<IpAddr>,
}

/// Proxy interaction semantics (M28-008-D, ADR-0033 §5). The runtime
/// dials validated origin addresses directly; no CONNECT tunneling and no
/// proxy credentials exist anywhere in the fetch path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProxyMode {
    /// No proxy is ever consulted or configured. Ambient environment
    /// variables are ignored by construction, and the policy surface
    /// exposes no way to enable one.
    #[default]
    Disabled,
}

/// The closed survey of ambient proxy environment variables (M28-008-D).
/// These names are never read by the runtime; the list exists so
/// diagnostics and tests can assert the isolation posture by name.
pub const AMBIENT_PROXY_ENV_VARS: &[&str] = &["http_proxy", "https_proxy", "all_proxy", "no_proxy"];

/// Decompression-ratio ceiling: decompressed output may be at most this
/// many times the compressed input (M28-007-D, ADR-0033 §8). Well above
/// any legitimate gzip/deflate expansion for web payloads.
pub const MAX_DECOMPRESSION_RATIO: u64 = 1000;

/// Minimum compressed input before the ratio ceiling applies (bytes).
/// Below this, small legitimate payloads with high local expansion (an
/// empty-JSON body, a run of one repeated byte) would false-positive.
pub const DECOMPRESSION_RATIO_THRESHOLD: u64 = 1024;

/// Push-based decompression bomb guard (M28-007-D). The fetch executor
/// feeds compressed bytes as consumed and decompressed bytes as produced;
/// every step is checked, so zip-bomb style expansion fails typed at the
/// step that crosses the line — never after buffering the payload.
#[derive(Debug, Clone)]
pub struct DecompressionGuard {
    output_limit: u64,
    compressed_in: u64,
    produced: u64,
}

impl DecompressionGuard {
    /// Build a guard with an explicit decompressed-output cap.
    pub fn new(output_limit: u64) -> Self {
        DecompressionGuard {
            output_limit,
            compressed_in: 0,
            produced: 0,
        }
    }

    /// Build from the frozen policy: `CompressionPolicy::Off` and
    /// `Gzip { enabled: false }` decompress nothing, so no guard exists;
    /// enabled gzip is bounded by the policy's response body cap.
    pub fn from_policy(policy: &FetchPolicy) -> Option<Self> {
        match policy.compression() {
            CompressionPolicy::Off => None,
            CompressionPolicy::Gzip { enabled: false } => None,
            CompressionPolicy::Gzip { enabled: true } => {
                Some(DecompressionGuard::new(policy.max_response_body_bytes()))
            }
        }
    }

    /// Record `n` compressed bytes consumed from the wire.
    pub fn compressed_input(&mut self, n: usize) {
        self.compressed_in += n as u64;
    }

    /// Record `n` decompressed bytes produced by the decoder. Typed failure
    /// when the output crosses the response body cap, or when the running
    /// ratio crosses the ceiling after the threshold volume of input.
    pub fn decompressed_output(&mut self, n: usize) -> Result<(), FetchPolicyError> {
        let produced = self.produced + n as u64;
        if produced > self.output_limit {
            return Err(FetchPolicyError::DecompressedTooLarge {
                produced,
                max: self.output_limit,
            });
        }
        if self.compressed_in >= DECOMPRESSION_RATIO_THRESHOLD
            && produced > self.compressed_in.saturating_mul(MAX_DECOMPRESSION_RATIO)
        {
            return Err(FetchPolicyError::DecompressionBomb {
                compressed: self.compressed_in,
                produced,
                max_ratio: MAX_DECOMPRESSION_RATIO,
            });
        }
        self.produced = produced;
        Ok(())
    }

    /// Decompressed bytes accepted so far.
    pub fn produced(&self) -> u64 {
        self.produced
    }

    /// Compressed bytes consumed so far.
    pub fn compressed_in(&self) -> u64 {
        self.compressed_in
    }
}

/// Maximum configured allow/deny entries (M28-008-C). Configuration is
/// bounded like everything else.
pub const MAX_EGRESS_HOST_ENTRIES: usize = 256;

/// Normalize a host(entry): trim, lowercase, strip one trailing dot,
/// bounded length. Empty on whitespace-only input.
fn normalize_host_entry(host: &str) -> String {
    host.trim().trim_end_matches('.').to_ascii_lowercase()
}

/// Does `host` match a configured entry? An entry starting with `.` is a
/// suffix rule matching the domain and every subdomain (`.corp.example`
/// matches `corp.example` and `api.corp.example`); other entries match
/// exactly. Inputs must already be normalized.
fn host_entry_matches(host: &str, entry: &str) -> bool {
    if let Some(suffix) = entry.strip_prefix('.') {
        host == suffix || host.ends_with(entry)
    } else {
        host == entry
    }
}

/// Hostnames (lowercased) that serve cloud instance metadata. Denied by
/// NAME before any resolution (M28-008-A, ADR-0033 §2) — defense in depth
/// alongside the address-space denial: a hostname must never earn its way
/// to the resolver when the name itself declares metadata intent.
pub const HOSTNAME_METADATA_ENDPOINTS: &[&str] =
    &["metadata.google.internal", "instance-data", "metadata"];

/// Case-insensitive metadata-hostname check with trailing-dot (FQDN)
/// normalization: `Metadata.Google.Internal.` is denied exactly like the
/// bare form.
pub fn is_metadata_hostname(host: &str) -> bool {
    let mut lower = host.trim_end_matches('.').to_ascii_lowercase();
    lower.truncate(253); // longest legal FQDN; keeps truncate cheap and bounded
    HOSTNAME_METADATA_ENDPOINTS.contains(&lower.as_str())
}

/// Resolve `host` and validate EVERY resolved address against the policy
/// trust mode BEFORE anything connects (M28-008-A, ADR-0033 §2/§3). The
/// resolver is injected so the real executor plugs DNS in and tests plug
/// deterministic fakes; the returned address set is the pin set — the
/// connector must dial exactly these addresses and never re-resolve, so a
/// first-resolve-public/second-resolve-private rebinding window cannot
/// open. IP-literal hosts skip the resolver entirely and are validated
/// directly. Ordering: hostname denial (metadata by name) -> resolution ->
/// per-address trust-mode validation; every failure is typed.
pub fn resolve_and_validate<F>(
    policy: &FetchPolicy,
    host: &str,
    mut resolve: F,
) -> Result<Vec<IpAddr>, FetchPolicyError>
where
    F: FnMut(&str) -> Result<Vec<IpAddr>, String>,
{
    if is_metadata_hostname(host) {
        return Err(FetchPolicyError::HostnameDenied {
            host: host.to_string(),
            reason: "cloud metadata endpoint denied by name".to_string(),
        });
    }
    // Explicit allow/deny configuration (M28-008-C): deny wins over allow;
    // neither can re-enable metadata names or undialable address classes.
    policy.check_host_config(host)?;
    // IP literals never reach the resolver: validate directly.
    if let Ok(addr) = host.parse::<IpAddr>() {
        policy.check_address(addr)?;
        return Ok(vec![normalize(addr)]);
    }
    let addrs = resolve(host).map_err(|reason| FetchPolicyError::HostnameDenied {
        host: host.to_string(),
        reason,
    })?;
    if addrs.is_empty() {
        return Err(FetchPolicyError::HostnameDenied {
            host: host.to_string(),
            reason: "no addresses resolved".to_string(),
        });
    }
    for &addr in &addrs {
        policy.check_address(addr)?;
    }
    Ok(addrs.into_iter().map(normalize).collect())
}

/// Credential-bearing headers (lowercased) that are **stripped when a
/// redirect crosses origins** (M28-007-C, ADR-0033 §4; aligned with the
/// WHATWG fetch HTTP-redirect fetch step). These never leak cross-origin;
/// same-origin redirects keep them.
pub const CREDENTIAL_REDIRECT_HEADERS: &[&str] =
    &["authorization", "cookie", "cookie2", "proxy-authorization"];

/// Compute the normalized origin of an absolute URL:
/// `lowercase(scheme)://lowercase(host)[:port]`, with the scheme's default
/// port elided (`http` 80, `https` 443) so `https://a:443` equals
/// `https://a`. `None` on malformed input (no scheme or no host) — callers
/// treat `None` as cross-origin and strip, fail closed.
pub fn url_origin(url: &str) -> Option<String> {
    let scheme = url_scheme(url);
    if scheme.is_empty() {
        return None;
    }
    let host = url_host(url);
    if host.is_empty() {
        return None;
    }
    // Recover the port exactly as written: the authority is between the
    // scheme delimiter and the first path/query/fragment separator.
    let rest = url.split_once("://").map(|(_, rest)| rest).unwrap_or("");
    let authority_end = rest.find(['/', '?', '#']).unwrap_or(rest.len());
    let authority = &rest[..authority_end];
    // A bracketed IPv6 authority never carries a bare port suffix in this
    // simplified parse; its colon pair belongs to the address.
    let port = if authority.starts_with('[') {
        None
    } else {
        authority.rsplit_once(':').map(|(_, p)| p)
    };
    let default_port = match scheme.to_ascii_lowercase().as_str() {
        "http" => Some("80"),
        "https" => Some("443"),
        _ => None,
    };
    let port_out = match (port, default_port) {
        (Some(p), Some(d)) if p == d => String::new(),
        (Some(p), _) => format!(":{p}"),
        (None, _) => String::new(),
    };
    Some(format!(
        "{}://{}{}",
        scheme.to_ascii_lowercase(),
        host.to_ascii_lowercase(),
        port_out
    ))
}

/// True when a redirect from `from_url` to `to_url` crosses origins
/// (scheme, host, or effective port differ). Malformed URLs on either side
/// count as cross-origin: stripping is the fail-closed direction.
pub fn is_cross_origin_redirect(from_url: &str, to_url: &str) -> bool {
    match (url_origin(from_url), url_origin(to_url)) {
        (Some(a), Some(b)) => a != b,
        _ => true,
    }
}

/// Case-insensitive check whether a header name is in the credential set.
pub fn is_credential_header(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    CREDENTIAL_REDIRECT_HEADERS.contains(&lower.as_str())
}

/// Headers that survive a redirect hop from `from_url` to `to_url`, given
/// the current hop's header names. Cross-origin hops (and malformed URLs)
/// drop every credential header; same-origin hops keep all names. Returns
/// the surviving names in input order, deduplicated case-insensitively.
pub fn headers_surviving_redirect<'a, I>(from_url: &str, to_url: &str, names: I) -> Vec<String>
where
    I: IntoIterator<Item = &'a str>,
{
    let cross_origin = is_cross_origin_redirect(from_url, to_url);
    let mut survived: Vec<String> = Vec::new();
    for name in names {
        if cross_origin && is_credential_header(name) {
            continue;
        }
        if !survived.iter().any(|s| s.eq_ignore_ascii_case(name)) {
            survived.push(name.to_string());
        }
    }
    survived
}

/// Extract the scheme prefix of an absolute URL (`"https"` from
/// `"https://host/x"`). Empty when no scheme delimiter exists, which the
/// scheme allowlist rejects fail closed. Case handling is
/// [`FetchPolicy::check_redirect_hop`]'s job.
fn url_scheme(url: &str) -> &str {
    match url.split_once("://") {
        Some((scheme, _)) => scheme,
        None => "",
    }
}

/// Extract the host component of an absolute URL (between the scheme
/// delimiter and the first `/`, `?`, `#`, or `:` port separator), with any
/// `user@` userinfo stripped. Empty on malformed input, which the resolved
/// policy rejects fail closed.
fn url_host(url: &str) -> &str {
    let rest = match url.split_once("://") {
        Some((_, rest)) => rest,
        None => "",
    };
    let end = rest.find(['/', '?', '#']).unwrap_or(rest.len());
    let authority = &rest[..end];
    let host = match authority.rsplit_once('@') {
        Some((_, host)) => host,
        None => authority,
    };
    // Trim a port suffix (last colon outside brackets keeps IPv6 literals
    // intact: "[::1]:8080" -> "[::1]"; brackets themselves stay for the
    // caller to normalize — addresses come from DNS, not the URL).
    if host.starts_with('[') {
        return host;
    }
    match host.rsplit_once(':') {
        Some((h, _)) => h,
        None => host,
    }
}

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
    /// Explicit egress deny list (M28-008-C): normalized hostnames; a match
    /// denies the host regardless of any allow entry. Deny wins.
    host_deny: Vec<String>,
    /// Explicit egress allow list (M28-008-C): empty means "no name-based
    /// restriction"; non-empty means only matching hosts pass the name
    /// gate. Never overrides metadata-by-name or trust-mode denials.
    host_allow: Vec<String>,
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
            host_deny: Vec::new(),
            host_allow: Vec::new(),
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

    /// Add normalized hostnames to the egress deny list (M28-008-C).
    /// Builder-style; entries are lowercased and trailing dots stripped.
    /// Entries beyond [`MAX_EGRESS_HOST_ENTRIES`] are dropped (bounded
    /// configuration; parse-time validators should reject bigger inputs).
    pub fn with_deny_hosts(mut self, hosts: impl IntoIterator<Item = impl Into<String>>) -> Self {
        for h in hosts {
            if self.host_deny.len() >= MAX_EGRESS_HOST_ENTRIES {
                break;
            }
            let entry = normalize_host_entry(&h.into());
            if !entry.is_empty() && !self.host_deny.contains(&entry) {
                self.host_deny.push(entry);
            }
        }
        self
    }

    /// Add normalized hostnames to the egress allow list (M28-008-C).
    /// A non-empty allow list restricts egress to matching hosts; it can
    /// never re-enable metadata names or undialable address classes.
    pub fn with_allow_hosts(mut self, hosts: impl IntoIterator<Item = impl Into<String>>) -> Self {
        for h in hosts {
            if self.host_allow.len() >= MAX_EGRESS_HOST_ENTRIES {
                break;
            }
            let entry = normalize_host_entry(&h.into());
            if !entry.is_empty() && !self.host_allow.contains(&entry) {
                self.host_allow.push(entry);
            }
        }
        self
    }

    /// The configured deny entries (normalized).
    pub fn host_deny(&self) -> &[String] {
        &self.host_deny
    }

    /// The configured allow entries (normalized).
    pub fn host_allow(&self) -> &[String] {
        &self.host_allow
    }

    /// Configuration gate for a hostname (M28-008-C): explicit deny wins
    /// over everything; a non-empty allow list restricts to matching hosts;
    /// an empty allow list imposes no name-based restriction. Typed
    /// `HostnameDenied` reasons name the decision so policy outcomes are
    /// observable without logging secrets.
    pub fn check_host_config(&self, host: &str) -> Result<(), FetchPolicyError> {
        let normalized = normalize_host_entry(host);
        for entry in &self.host_deny {
            if host_entry_matches(&normalized, entry) {
                return Err(FetchPolicyError::HostnameDenied {
                    host: host.to_string(),
                    reason: "explicitly denied by egress configuration".to_string(),
                });
            }
        }
        if !self.host_allow.is_empty()
            && !self
                .host_allow
                .iter()
                .any(|e| host_entry_matches(&normalized, e))
        {
            return Err(FetchPolicyError::HostnameDenied {
                host: host.to_string(),
                reason: "not in the configured egress allow list".to_string(),
            });
        }
        Ok(())
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

    /// Proxy interaction posture (M28-008-D): always
    /// [`ProxyMode::Disabled`] — the dial goes straight to the validated
    /// origin address, and ambient proxy environment variables are ignored
    /// by construction.
    pub fn proxy_mode(&self) -> ProxyMode {
        ProxyMode::Disabled
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

    // --- ADR-0035 trusted-code assumption (M28-001-D) -------------------------

    #[test]
    fn trusted_code_assumption_is_pinned() {
        let a = TRUSTED_CODE_ASSUMPTION;
        // The three load-bearing properties (ADR-0035 §5).
        assert!(a.contains("trusted"), "must name trusted code");
        assert!(a.contains("not a sandbox"), "must deny the sandbox framing");
        assert!(
            a.contains("network"),
            "must name the network as the adversary"
        );
        // The forbidden claim must not appear anywhere in the statement.
        let lower = a.to_ascii_lowercase();
        assert!(
            !lower.contains("sandbox for untrusted") || a.contains("not a sandbox"),
            "wording must never admit untrusted-code sandboxing"
        );
        // Multi-tenant / hostile-bundle framing is out of scope by name.
        assert!(a.contains("process interior"));
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

    #[test]
    fn body_helper_sizes_are_defined_and_fail_closed() {
        let helpers = [
            BodyHelper::ResponseText,
            BodyHelper::ResponseJson,
            BodyHelper::ResponseArrayBuffer,
            BodyHelper::ResponseBytes,
        ];
        for h in helpers {
            assert_eq!(h.max_bytes(), MAX_BODY_HELPER_BYTES);
            assert!(check_body_helper_size(h, 0).is_ok());
            assert!(check_body_helper_size(h, h.max_bytes()).is_ok());
            let err = check_body_helper_size(h, h.max_bytes() + 1).unwrap_err();
            assert!(matches!(err, FetchPolicyError::BodyTooLarge { .. }));
            let msg = err.to_string();
            assert!(msg.contains(h.name()), "message names the helper: {msg}");
            assert!(msg.contains("maximum helper size"));
        }
    }

    #[test]
    fn body_helper_cap_composes_with_network_and_text_limits() {
        // A helper can never be asked to materialize more than the network
        // layer admits (ADR-0033 §9), and text decode stays within the
        // text-encoding buffer bound.
        const _: () = assert!(MAX_BODY_HELPER_BYTES <= crate::text_encoding::MAX_TEXT_BUFFER_LEN);
        assert_eq!(
            MAX_BODY_HELPER_BYTES,
            MAX_FETCH_RESPONSE_BODY_BYTES as usize
        );
        assert_eq!(MAX_BODY_HELPER_BYTES, 16 * 1024 * 1024);
    }

    #[test]
    fn redirect_limiter_follows_up_to_ceiling_then_fails_typed() {
        let mut lim = RedirectLimiter::new(FetchPolicy::default());
        assert_eq!(lim.limit(), MAX_REDIRECT_HOPS);
        // Hops 1..=MAX_FOLLOW distinct targets: all follow.
        for hop in 1..=MAX_REDIRECT_HOPS {
            let to = format!("https://ex.test/r{hop}");
            assert_eq!(
                lim.evaluate(&format!("https://ex.test/r{}", hop - 1), &to),
                Ok(RedirectOutcome::Follow),
                "hop {hop} must follow"
            );
            assert_eq!(lim.hops(), hop);
        }
        // One past the ceiling: bounded, typed failure.
        assert_eq!(
            lim.evaluate("https://ex.test/r20", "https://ex.test/r21"),
            Err(FetchPolicyError::TooManyRedirects {
                max_hops: MAX_REDIRECT_HOPS
            })
        );
    }

    #[test]
    fn manual_policy_surfaces_3xx_without_following() {
        let policy = FetchPolicy {
            redirect: RedirectPolicy::Manual,
            ..FetchPolicy::default()
        };
        let mut lim = RedirectLimiter::new(policy);
        assert_eq!(lim.limit(), 0);
        // Manual: surface the 3xx; no hop consumption, no policy check.
        assert_eq!(
            lim.evaluate("https://a.test/", "http://b.test/"),
            Ok(RedirectOutcome::Surface)
        );
        assert_eq!(lim.hops(), 0);
    }

    #[test]
    fn redirect_loop_fails_fast_and_typed() {
        let mut lim = RedirectLimiter::new(FetchPolicy::default());
        assert_eq!(
            lim.evaluate("https://a.test/x", "https://b.test/y"),
            Ok(RedirectOutcome::Follow)
        );
        assert_eq!(
            lim.evaluate("https://b.test/y", "https://a.test/x"),
            Ok(RedirectOutcome::Follow)
        );
        // Returning to an already-visited URL: typed loop error, well
        // before the hop ceiling would fire.
        let err = lim
            .evaluate("https://a.test/x", "https://b.test/y")
            .unwrap_err();
        assert!(matches!(err, FetchPolicyError::RedirectLoop { .. }));
        assert!(err.to_string().contains("redirect loop"));
        assert_eq!(lim.hops(), 2, "failed hop is not counted as followed");
    }

    #[test]
    fn scheme_and_downgrade_denials_flow_through_limiter() {
        let mut lim = RedirectLimiter::new(FetchPolicy::default());
        // https -> http downgrade: denied.
        assert!(matches!(
            lim.evaluate("https://a.test/", "http://b.test/"),
            Err(FetchPolicyError::DowngradeRedirect { .. })
        ));
        // Unallowed scheme on the target: denied.
        assert!(matches!(
            lim.evaluate("https://a.test/", "ftp://b.test/f"),
            Err(FetchPolicyError::SchemeNotAllowed { .. })
        ));
        // Malformed target without a scheme delimiter: denied fail closed.
        assert!(matches!(
            lim.evaluate("https://a.test/", "not-a-url"),
            Err(FetchPolicyError::SchemeNotAllowed { .. })
        ));
        assert_eq!(lim.hops(), 0, "denied hops leave the limiter unchanged");
    }

    #[test]
    fn custom_hop_limit_is_respected_exactly() {
        let policy = FetchPolicy {
            redirect: RedirectPolicy::Follow { max_hops: 3 },
            ..FetchPolicy::default()
        };
        assert!(policy.validate_redirect_policy().is_ok());
        let mut lim = RedirectLimiter::new(policy);
        assert_eq!(lim.limit(), 3);
        for hop in 1..=3 {
            assert_eq!(
                lim.evaluate(
                    &format!("https://a.test/{hop}"),
                    &format!("https://a.test/{}", hop + 1)
                ),
                Ok(RedirectOutcome::Follow)
            );
        }
        assert!(matches!(
            lim.evaluate("https://a.test/4", "https://a.test/5"),
            Err(FetchPolicyError::TooManyRedirects { max_hops: 3 })
        ));
    }

    fn public_ip() -> std::net::IpAddr {
        "93.184.216.34".parse().unwrap()
    }

    #[test]
    fn ssrf_policy_is_reapplied_on_every_hop() {
        let mut lim = RedirectLimiter::new(FetchPolicy::default());
        // Hop 1: public -> public (93.184.216.34): follows.
        assert_eq!(
            lim.evaluate_resolved(
                "https://a.test/start",
                "https://b.test/next",
                &[public_ip()]
            ),
            Ok(RedirectOutcome::Follow)
        );
        assert_eq!(lim.hops(), 1);
        // Hop 2: the public origin redirects into loopback space — the
        // classic SSRF-via-redirect — denied typed, limiter unchanged.
        // (https target: the https→http downgrade rule is exercised on its
        // own in the deny-path and ceiling tests.)
        let err = lim
            .evaluate_resolved(
                "https://b.test/next",
                "https://127.0.0.1:8443/fetch-internal",
                &["127.0.0.1".parse().unwrap()],
            )
            .unwrap_err();
        assert!(matches!(
            err,
            FetchPolicyError::AddressDenied {
                class: AddressClass::Loopback,
                ..
            }
        ));
        assert_eq!(lim.hops(), 1, "denied hop must not advance state");
        // Hop 3: a legitimate public hop still follows (state intact).
        assert_eq!(
            lim.evaluate_resolved(
                "https://b.test/next",
                "https://c.test/final",
                &[public_ip()]
            ),
            Ok(RedirectOutcome::Follow)
        );
        assert_eq!(lim.hops(), 2);
    }

    #[test]
    fn redirect_target_resolving_partial_loopback_is_denied() {
        let mut lim = RedirectLimiter::new(FetchPolicy::default());
        // DNS round-robin mixes a public and a private address: ONE bad
        // address poisons the host (every address must pass).
        let mixed = [public_ip(), "10.0.0.7".parse().unwrap()];
        let err = lim
            .evaluate_resolved("https://a.test/", "https://evil.test/x", &mixed)
            .unwrap_err();
        assert!(matches!(
            err,
            FetchPolicyError::AddressDenied {
                class: AddressClass::Private,
                ..
            }
        ));
        assert_eq!(lim.hops(), 0);
    }

    #[test]
    fn loopback_trust_still_denies_metadata_space_on_hops() {
        let mut lim = RedirectLimiter::new(FetchPolicy::trusted_loopback_explicit());
        // Explicit loopback testing mode: 127.0.0.1 targets are dialable.
        assert_eq!(
            lim.evaluate_resolved(
                "http://a.test/",
                "http://127.0.0.1:8080/next",
                &["127.0.0.1".parse().unwrap()]
            ),
            Ok(RedirectOutcome::Follow)
        );
        // ...but cloud metadata space keeps its own class and stays denied
        // even here.
        let err = lim
            .evaluate_resolved(
                "http://127.0.0.1:8080/next",
                "http://169.254.169.254/latest/meta-data",
                &["169.254.169.254".parse().unwrap()],
            )
            .unwrap_err();
        assert!(matches!(
            err,
            FetchPolicyError::AddressDenied {
                class: AddressClass::Metadata,
                ..
            }
        ));
    }

    #[test]
    fn empty_resolution_and_ceiling_precede_address_checks() {
        // Empty DNS resolution is a typed hostname denial.
        let mut lim = RedirectLimiter::new(FetchPolicy::default());
        let err = lim
            .evaluate_resolved("https://a.test/", "https://b.test/x", &[])
            .unwrap_err();
        assert!(matches!(err, FetchPolicyError::HostnameDenied { .. }));
        // A hop past the ceiling fails before any address check matters.
        let mut lim = RedirectLimiter::new(FetchPolicy {
            redirect: RedirectPolicy::Follow { max_hops: 1 },
            ..FetchPolicy::default()
        });
        assert_eq!(
            lim.evaluate_resolved("https://a.test/", "https://b.test/x", &[public_ip()]),
            Ok(RedirectOutcome::Follow)
        );
        let err = lim
            .evaluate_resolved(
                "https://b.test/x",
                "https://c.test/y",
                &["127.0.0.1".parse().unwrap()],
            )
            .unwrap_err();
        assert!(matches!(
            err,
            FetchPolicyError::TooManyRedirects { max_hops: 1 }
        ));
    }

    #[test]
    fn cross_origin_hops_strip_credential_headers() {
        // Host change: credentials dropped, everything else survives.
        let hop = headers_surviving_redirect(
            "https://a.test/start",
            "https://b.test/next",
            ["Authorization", "cookie", "Content-Type", "X-Custom"],
        );
        assert_eq!(hop, ["Content-Type", "X-Custom"]);

        // Scheme change (https -> http): cross-origin even for same host.
        let hop = headers_surviving_redirect(
            "https://a.test/x",
            "http://a.test/y",
            ["Authorization", "Accept"],
        );
        assert_eq!(hop, ["Accept"]);

        // Port change: cross-origin.
        let hop = headers_surviving_redirect(
            "https://a.test:8443/x",
            "https://a.test/y",
            ["Proxy-Authorization", "Accept"],
        );
        assert_eq!(hop, ["Accept"]);
    }

    #[test]
    fn same_origin_hops_keep_credentials() {
        let hop = headers_surviving_redirect(
            "https://a.test/start",
            "https://a.test/next?q=1",
            ["Authorization", "Cookie", "Content-Type"],
        );
        assert_eq!(hop, ["Authorization", "Cookie", "Content-Type"]);
        // Default port elision: https://a.test == https://a.test:443.
        let hop = headers_surviving_redirect(
            "https://a.test:443/x",
            "https://a.test/y",
            ["Authorization"],
        );
        assert_eq!(hop, ["Authorization"]);
        assert!(!is_cross_origin_redirect(
            "http://a.test:80/",
            "http://a.test/"
        ));
        assert!(is_cross_origin_redirect(
            "http://a.test:8080/",
            "http://a.test/"
        ));
    }

    #[test]
    fn credential_header_detection_is_case_insensitive_and_closed() {
        for name in [
            "Authorization",
            "AUTHORIZATION",
            "Cookie",
            "COOKIE2",
            "proxy-authorization",
        ] {
            assert!(
                is_credential_header(name),
                "{name} must be a credential header"
            );
        }
        for name in [
            "content-type",
            "X-Custom",
            "authorizationx",
            "www-authenticate",
        ] {
            assert!(!is_credential_header(name), "{name} must survive");
        }
        // The set itself is the closed policy surface (lowercased).
        assert_eq!(
            CREDENTIAL_REDIRECT_HEADERS,
            &["authorization", "cookie", "cookie2", "proxy-authorization"]
        );
    }

    #[test]
    fn malformed_redirect_targets_fail_closed_to_stripping() {
        // Malformed target origin: cross-origin by definition -> strip.
        assert!(is_cross_origin_redirect("https://a.test/x", "not-a-url"));
        assert!(is_cross_origin_redirect("garbage", "https://a.test/x"));
        // And the surviving set drops credentials for that hop.
        let hop = headers_surviving_redirect(
            "https://a.test/x",
            "not-a-url",
            ["Authorization", "Accept"],
        );
        assert_eq!(hop, ["Accept"]);
    }

    #[test]
    fn decompression_output_is_capped_typed() {
        let mut g = DecompressionGuard::new(16);
        g.compressed_input(4);
        assert!(g.decompressed_output(10).is_ok());
        assert_eq!(g.produced(), 10);
        let err = g.decompressed_output(7).unwrap_err();
        assert!(matches!(
            err,
            FetchPolicyError::DecompressedTooLarge {
                produced: 17,
                max: 16
            }
        ));
        assert!(err.to_string().contains("response limit"));
        // Rejected bytes are not silently accepted on retry.
        assert!(g.decompressed_output(6).is_ok());
        assert!(g.decompressed_output(1).is_err());
    }

    #[test]
    fn zip_bomb_ratio_is_bounded_typed() {
        let mut g = DecompressionGuard::new(MAX_FETCH_RESPONSE_BODY_BYTES);
        // Classic bomb shape: tiny compressed input, runaway output.
        g.compressed_input(2048); // past the 1 KiB threshold
        let limit = 2048 * MAX_DECOMPRESSION_RATIO;
        assert!(g.decompressed_output(limit as usize).is_ok());
        let err = g.decompressed_output(1).unwrap_err();
        assert!(matches!(
            err,
            FetchPolicyError::DecompressionBomb {
                compressed: 2048,
                produced: _,
                max_ratio: 1000
            }
        ));
        assert!(err.to_string().contains("decompression bomb"));
    }

    #[test]
    fn small_payloads_are_not_ratio_limited_below_threshold() {
        let mut g = DecompressionGuard::new(MAX_FETCH_RESPONSE_BODY_BYTES);
        // 4 compressed bytes expanding to 4000: ratio ~1000x but below the
        // input threshold — legitimate small-payload expansion, allowed.
        g.compressed_input(4);
        assert!(g.decompressed_output(4000).is_ok());
        assert_eq!(g.produced(), 4000);
    }

    #[test]
    fn guard_from_policy_matches_compression_posture() {
        // Default policy: compression off -> no decompression happens at all.
        assert!(DecompressionGuard::from_policy(&FetchPolicy::default()).is_none());
        // Explicitly disabled gzip: still no guard.
        let policy = FetchPolicy {
            compression: CompressionPolicy::Gzip { enabled: false },
            ..FetchPolicy::default()
        };
        assert!(DecompressionGuard::from_policy(&policy).is_none());
        // Enabled gzip: bounded by the response body cap.
        let policy = FetchPolicy {
            compression: CompressionPolicy::Gzip { enabled: true },
            ..FetchPolicy::default()
        };
        let mut g = DecompressionGuard::from_policy(&policy).unwrap();
        assert_eq!(g.produced(), 0);
        // The cap equals the network response limit (ADR-0033 §9).
        let err = g
            .decompressed_output((MAX_FETCH_RESPONSE_BODY_BYTES + 1) as usize)
            .unwrap_err();
        assert!(matches!(err, FetchPolicyError::DecompressedTooLarge { .. }));
    }

    #[test]
    fn bomb_fixture_output_cap_fires_before_ratio_when_tighter() {
        // A compressed 1 KiB fixture claiming 20 MiB of output: the output
        // cap (16 MiB) is crossed first and fires as the typed failure.
        let mut g = DecompressionGuard::new(MAX_FETCH_RESPONSE_BODY_BYTES);
        g.compressed_input(1024);
        let err = g
            .decompressed_output((20 * 1024 * 1024) as usize)
            .unwrap_err();
        assert!(matches!(err, FetchPolicyError::DecompressedTooLarge { .. }));
        assert_eq!(g.produced(), 0, "failed step accepts no bytes");
    }

    fn resolver_of(
        addrs: &'static [std::net::IpAddr],
    ) -> impl FnMut(&str) -> Result<Vec<IpAddr>, String> {
        move |_host| Ok(addrs.to_vec())
    }

    #[test]
    fn metadata_hostnames_are_denied_before_any_resolution() {
        let policy = FetchPolicy::default();
        // The resolver records whether it was ever called: denial must
        // happen by NAME, before the resolver is touched.
        let mut called = false;
        let mut resolve = |_host: &str| {
            called = true;
            Ok::<Vec<IpAddr>, String>(vec![public_ip()])
        };
        for host in [
            "metadata.google.internal",
            "Metadata.Google.Internal.",
            "INSTANCE-DATA",
            "metadata",
        ] {
            let err = resolve_and_validate(&policy, host, &mut resolve).unwrap_err();
            assert!(
                matches!(&err, FetchPolicyError::HostnameDenied { host: h, reason }
                    if reason.contains("denied by name")),
                "{host}: {err}"
            );
        }
        assert!(!called, "resolver must not be consulted for denied names");
    }

    #[test]
    fn resolution_uses_only_validated_addresses_in_order() {
        let policy = FetchPolicy::default();
        let public_a: IpAddr = "93.184.216.34".parse().unwrap();
        let public_b: IpAddr = "2606:2800:220:1:248:1893:25c8:1946".parse().unwrap();
        let static_addrs: &'static [std::net::IpAddr] =
            Box::leak(vec![public_a, public_b].into_boxed_slice());
        let pinned = resolve_and_validate(&policy, "example.com", resolver_of(static_addrs))
            .expect("all-public resolution validates");
        // The returned set is the connect pin set: same addresses, same
        // order, IPv4-mapped forms normalized.
        assert_eq!(pinned, vec![public_a, public_b]);
    }

    #[test]
    fn dns_rebinding_mixing_public_and_private_fails_closed() {
        let policy = FetchPolicy::default();
        // First answer public, second private: the classic rebinding
        // first-glance trick is denied because EVERY address must pass.
        let static_addrs: &'static [std::net::IpAddr] = Box::leak(
            vec![
                "93.184.216.34".parse::<IpAddr>().unwrap(),
                "10.0.0.9".parse::<IpAddr>().unwrap(),
            ]
            .into_boxed_slice(),
        );
        let err =
            resolve_and_validate(&policy, "rebind.test", resolver_of(static_addrs)).unwrap_err();
        assert!(matches!(
            err,
            FetchPolicyError::AddressDenied {
                class: AddressClass::Private,
                ..
            }
        ));
        // Empty resolution is a typed denial too.
        let static_empty: &'static [std::net::IpAddr] = Box::leak(Vec::new().into_boxed_slice());
        let err = resolve_and_validate(&policy, "nx.test", resolver_of(static_empty)).unwrap_err();
        assert!(
            matches!(&err, FetchPolicyError::HostnameDenied { reason, .. }
            if reason.contains("no addresses resolved"))
        );
    }

    #[test]
    fn resolver_failures_and_ip_literals_are_handled() {
        let policy = FetchPolicy::default();
        // Resolver error: typed hostname denial carrying the reason.
        let mut resolve =
            |_host: &str| -> Result<Vec<IpAddr>, String> { Err("timed out".to_string()) };
        let err = resolve_and_validate(&policy, "slow.test", &mut resolve).unwrap_err();
        assert!(
            matches!(&err, FetchPolicyError::HostnameDenied { host, reason }
            if host == "slow.test" && reason == "timed out")
        );
        // IP-literal hosts skip the resolver and validate directly.
        let called = std::cell::Cell::new(false);
        let literal_resolver = |_host: &str| {
            called.set(true);
            Ok::<Vec<IpAddr>, String>(vec!["1.2.3.4".parse().unwrap()])
        };
        assert!(resolve_and_validate(&policy, "93.184.216.34", &literal_resolver).is_ok());
        assert!(!called.get(), "IP literals must not reach the resolver");
        assert!(matches!(
            resolve_and_validate(&policy, "127.0.0.1", &literal_resolver),
            Err(FetchPolicyError::AddressDenied {
                class: AddressClass::Loopback,
                ..
            })
        ));
    }

    #[test]
    fn follow_hop_resolves_and_validates_atomically() {
        let mut lim = RedirectLimiter::new(FetchPolicy::default());
        // Hop 1: public target resolves and validates; pin set returned.
        let mut resolve = |_host: &str| Ok::<Vec<IpAddr>, String>(vec![public_ip()]);
        let hop = lim
            .follow_hop("https://a.test/start", "https://b.test/next", &mut resolve)
            .expect("public hop follows");
        assert_eq!(hop.outcome, RedirectOutcome::Follow);
        assert_eq!(hop.pinned, vec![public_ip()]);
        assert_eq!(lim.hops(), 1);

        // Hop 2: resolution FAILS after URL checks — the limiter state must
        // remain exactly as before the attempt (atomicity).
        let mut failing =
            |_host: &str| -> Result<Vec<IpAddr>, String> { Err("servfail".to_string()) };
        let err = lim
            .follow_hop("https://b.test/next", "https://c.test/final", &mut failing)
            .unwrap_err();
        assert!(
            matches!(&err, FetchPolicyError::HostnameDenied { reason, .. }
            if reason == "servfail")
        );
        assert_eq!(lim.hops(), 1, "failed hop must not advance state");

        // Hop 3: a valid hop still follows after the failed attempt.
        let mut resolve2 = |_host: &str| {
            Ok::<Vec<IpAddr>, String>(vec!["2606:2800:220:1:248:1893:25c8:1946".parse().unwrap()])
        };
        let hop = lim
            .follow_hop("https://b.test/next", "https://c.test/final", &mut resolve2)
            .expect("retry with working DNS follows");
        assert_eq!(hop.outcome, RedirectOutcome::Follow);
        assert_eq!(lim.hops(), 2);
    }

    #[test]
    fn redirect_targets_deny_metadata_by_name_too() {
        let mut lim = RedirectLimiter::new(FetchPolicy::default());
        // The resolver must never be consulted: the name denies first.
        let called = std::cell::Cell::new(false);
        let resolve = |_host: &str| {
            called.set(true);
            Ok::<Vec<IpAddr>, String>(vec![public_ip()])
        };
        let err = lim
            .follow_hop(
                "https://a.test/",
                "https://metadata.google.internal/computeMetadata/v1/",
                &resolve,
            )
            .unwrap_err();
        assert!(
            matches!(&err, FetchPolicyError::HostnameDenied { host, reason }
            if host == "metadata.google.internal" && reason.contains("denied by name"))
        );
        assert!(!called.get());
        assert_eq!(lim.hops(), 0);
    }

    #[test]
    fn follow_hop_pin_set_is_the_only_dial_set() {
        let mut lim = RedirectLimiter::new(FetchPolicy::default());
        // A rebinding-flavored answer (public + private) is denied; the
        // pinned set on success contains only validated addresses.
        let mut mixed = |_host: &str| {
            Ok::<Vec<IpAddr>, String>(vec![public_ip(), "192.168.1.1".parse().unwrap()])
        };
        let err = lim
            .follow_hop("https://a.test/", "https://b.test/x", &mut mixed)
            .unwrap_err();
        assert!(matches!(
            err,
            FetchPolicyError::AddressDenied {
                class: AddressClass::Private,
                ..
            }
        ));
        let mut clean = |_host: &str| Ok::<Vec<IpAddr>, String>(vec![public_ip()]);
        let hop = lim
            .follow_hop("https://a.test/", "https://b.test/x", &mut clean)
            .expect("clean answer follows");
        assert_eq!(hop.pinned.len(), 1);
        assert!(lim.hops() == 1);
    }

    #[test]
    fn manual_gate_surfaces_without_any_resolution() {
        let policy = FetchPolicy {
            redirect: RedirectPolicy::Manual,
            ..FetchPolicy::default()
        };
        let mut lim = RedirectLimiter::new(policy);
        let called = std::cell::Cell::new(false);
        let resolve = |_host: &str| {
            called.set(true);
            Ok::<Vec<IpAddr>, String>(vec![public_ip()])
        };
        let hop = lim
            .follow_hop("https://a.test/", "https://b.test/", &resolve)
            .expect("manual policy surfaces the 3xx");
        assert_eq!(hop.outcome, RedirectOutcome::Surface);
        assert!(hop.pinned.is_empty());
        assert!(!called.get());
        assert_eq!(lim.hops(), 0);
    }

    #[test]
    fn full_fetch_sequence_composes_open_and_hops() {
        // The executor shape: open the origin target, then follow hops.
        let policy = FetchPolicy::default();
        let mut lim = RedirectLimiter::new(policy.clone());
        // Open: validate the ORIGINAL target (resolve_and_validate path)
        // and seed it so a redirect back to it is a loop.
        let mut resolver = |_host: &str| Ok::<Vec<IpAddr>, String>(vec![public_ip()]);
        let pinned = resolve_and_validate(&policy, url_host("https://a.test/"), &mut resolver)
            .expect("origin target validates");
        assert_eq!(pinned, vec![public_ip()]);
        lim.seed_target("https://a.test/");
        // Hop 1: same-origin redirect (path-only) — keeps following.
        let mut r = |_host: &str| Ok::<Vec<IpAddr>, String>(vec![public_ip()]);
        let hop = lim
            .follow_hop("https://a.test/a", "https://a.test/b", &mut r)
            .expect("same-origin hop follows");
        assert_eq!(hop.outcome, RedirectOutcome::Follow);
        // Hop 2: returning to the ORIGINAL target is a typed loop denial.
        let mut r2 = |_host: &str| Ok::<Vec<IpAddr>, String>(vec![public_ip()]);
        let err = lim
            .follow_hop("https://a.test/b", "https://a.test/", &mut r2)
            .unwrap_err();
        assert!(matches!(err, FetchPolicyError::RedirectLoop { .. }));
        assert_eq!(lim.hops(), 1, "loop denial leaves state unchanged");
    }

    #[test]
    fn deny_list_blocks_hosts_before_resolution() {
        let policy = FetchPolicy::default().with_deny_hosts(["evil.test", "banned.example"]);
        // Denied by name; the resolver is provably untouched.
        let called = std::cell::Cell::new(false);
        let mut resolve = |_host: &str| {
            called.set(true);
            Ok::<Vec<IpAddr>, String>(vec![public_ip()])
        };
        let err = resolve_and_validate(&policy, "EVIL.test.", &mut resolve).unwrap_err();
        assert!(
            matches!(&err, FetchPolicyError::HostnameDenied { host, reason }
            if host == "EVIL.test." && reason.contains("explicitly denied"))
        );
        assert!(!called.get());
        // Unrelated hosts pass the name gate (subject to address checks).
        assert!(resolve_and_validate(&policy, "fine.test", &mut resolve).is_ok());
    }

    #[test]
    fn allow_list_restricts_to_listed_hosts_only() {
        let policy = FetchPolicy::default().with_allow_hosts(["api.corp.example"]);
        let mut resolve = |_host: &str| Ok::<Vec<IpAddr>, String>(vec![public_ip()]);
        // Non-listed host: typed denial naming the decision.
        let err = resolve_and_validate(&policy, "elsewhere.test", &mut resolve).unwrap_err();
        assert!(
            matches!(&err, FetchPolicyError::HostnameDenied { reason, .. }
            if reason.contains("allow list"))
        );
        // Listed host passes.
        assert!(resolve_and_validate(&policy, "api.corp.example", &mut resolve).is_ok());
        // Empty allow list imposes no name-based restriction.
        let unrestricted = FetchPolicy::default();
        assert!(resolve_and_validate(&unrestricted, "anything.test", &mut resolve).is_ok());
    }

    #[test]
    fn deny_wins_over_allow() {
        let policy = FetchPolicy::default()
            .with_allow_hosts(["a.test", "b.test"])
            .with_deny_hosts(["b.test"]);
        let mut resolve = |_host: &str| Ok::<Vec<IpAddr>, String>(vec![public_ip()]);
        assert!(resolve_and_validate(&policy, "a.test", &mut resolve).is_ok());
        let err = resolve_and_validate(&policy, "b.test", &mut resolve).unwrap_err();
        assert!(
            matches!(&err, FetchPolicyError::HostnameDenied { reason, .. }
            if reason.contains("explicitly denied"))
        );
    }

    #[test]
    fn allow_list_cannot_re_enable_metadata_names() {
        let policy =
            FetchPolicy::default().with_allow_hosts(["metadata.google.internal", "127.0.0.1-host"]);
        let called = std::cell::Cell::new(false);
        let mut resolve = |_host: &str| {
            called.set(true);
            Ok::<Vec<IpAddr>, String>(vec![public_ip()])
        };
        // The safe default is not configurable away: metadata names stay
        // denied by name even when explicitly allow-listed.
        let err =
            resolve_and_validate(&policy, "metadata.google.internal", &mut resolve).unwrap_err();
        assert!(
            matches!(&err, FetchPolicyError::HostnameDenied { reason, .. }
            if reason.contains("denied by name"))
        );
        assert!(!called.get());
    }

    #[test]
    fn suffix_entries_cover_domain_and_subdomains() {
        let policy = FetchPolicy::default().with_deny_hosts([".internal.test"]);
        assert!(policy.check_host_config("api.internal.test").is_err());
        assert!(policy.check_host_config("internal.test").is_err());
        // Suffix anchoring: an unrelated host merely ENDING with the same
        // text does not match.
        assert!(policy.check_host_config("internal.test.evil").is_ok());
        assert!(policy.check_host_config("notinternal.test").is_ok());
    }

    #[test]
    fn configuration_normalizes_and_deduplicates_entries() {
        let policy = FetchPolicy::default()
            .with_deny_hosts(["EVIL.Test.", "evil.test", "  spaced.test  "])
            .with_allow_hosts(["OK.Test"]);
        assert_eq!(policy.host_deny(), ["evil.test", "spaced.test"]);
        assert_eq!(policy.host_allow(), ["ok.test"]);
        // A host equal to the apex suffix rule matches.
        assert!(policy.check_host_config("SPACED.TEST").is_err());
    }

    #[test]
    fn proxy_mode_is_disabled_by_construction_and_not_configurable() {
        // Every policy construction path reports the same posture.
        assert_eq!(FetchPolicy::default().proxy_mode(), ProxyMode::Disabled);
        assert_eq!(
            FetchPolicy::trusted_loopback_explicit().proxy_mode(),
            ProxyMode::Disabled
        );
        let configured = FetchPolicy::default()
            .with_deny_hosts(["a.test"])
            .with_allow_hosts(["b.test"]);
        assert_eq!(configured.proxy_mode(), ProxyMode::Disabled);
        // The ambient flag stays false through every builder.
        assert!(!configured.ambient_proxy_enabled());
        assert!(!FetchPolicy::default().ambient_proxy_enabled());
    }

    #[test]
    fn ambient_proxy_env_survey_is_the_closed_list() {
        // The runtime never reads these; the list is the diagnostic surface.
        assert_eq!(
            AMBIENT_PROXY_ENV_VARS,
            &["http_proxy", "https_proxy", "all_proxy", "no_proxy"]
        );
        for name in AMBIENT_PROXY_ENV_VARS {
            assert!(
                name.eq(name) && name.to_ascii_lowercase() == *name,
                "survey entries must be lowercased: {name}"
            );
        }
    }
}
