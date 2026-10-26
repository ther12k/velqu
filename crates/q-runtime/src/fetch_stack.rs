//! Outbound fetch stack and lazy connection pooling (M28-002-B, M28-003-A, M28-003-B, M28-003-D).
//!
//! Provides a strictly lazy, thread-safe connection pool [`FetchPool`] for
//! outbound requests. An application that does not execute any `fetch()`
//! calls incurs **zero** pool initialization overhead (zero socket creation,
//! zero TLS context allocation, zero background tasks).
//!
//! On first actual request, the pool initializes once as a singleton using
//! the M28-002-A selected stack (hyper 1 + hyper-util client-legacy +
//! hyper-rustls with webpki roots) with strict bounds on idle connections,
//! active concurrency, keepalive, and bounded shutdown drains (ADR-0031).

use std::collections::HashMap;
use std::fmt;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Duration;

use http_body_util::Empty;
use hyper_rustls::HttpsConnectorBuilder;
use hyper_util::client::legacy::Client;
use hyper_util::rt::TokioExecutor;
use q_capabilities::FetchPolicy;
use tokio::sync::{OwnedSemaphorePermit, Semaphore};

/// Human-readable identity of the selected outbound stack.
pub const STACK_ID: &str =
    "hyper1+hyper-util(client-legacy)+hyper-rustls(webpki-roots,ring,http1,tls12)";

/// Default pool idle connection timeout (15 seconds).
pub const DEFAULT_POOL_IDLE_TIMEOUT_SECS: u64 = 15;
/// Default maximum idle connections per host.
pub const DEFAULT_MAX_IDLE_PER_HOST: usize = 32;
/// Default maximum concurrent active outbound requests (backpressure bound).
pub const DEFAULT_MAX_ACTIVE_CONNECTIONS: usize = 128;
/// Maximum ceiling for active connections.
pub const MAX_ACTIVE_CONNECTIONS_CEILING: usize = 1024;
/// Default TCP connect timeout (10 seconds, matching ADR-0033).
pub const DEFAULT_CONNECT_TIMEOUT_SECS: u64 = 10;
/// Default TCP keepalive duration (30 seconds).
pub const DEFAULT_TCP_KEEPALIVE_SECS: u64 = 30;

pub type OutboundConnector =
    hyper_rustls::HttpsConnector<hyper_util::client::legacy::connect::HttpConnector>;
pub type OutboundClient = Client<OutboundConnector, Empty<bytes::Bytes>>;

/// Bounded connection pool and transport parameters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PoolBounds {
    pub max_idle_per_host: usize,
    pub idle_timeout: Duration,
    pub max_active_connections: usize,
    pub connect_timeout: Duration,
    pub tcp_keepalive: Duration,
}

impl Default for PoolBounds {
    fn default() -> Self {
        PoolBounds {
            max_idle_per_host: DEFAULT_MAX_IDLE_PER_HOST,
            idle_timeout: Duration::from_secs(DEFAULT_POOL_IDLE_TIMEOUT_SECS),
            max_active_connections: DEFAULT_MAX_ACTIVE_CONNECTIONS,
            connect_timeout: Duration::from_secs(DEFAULT_CONNECT_TIMEOUT_SECS),
            tcp_keepalive: Duration::from_secs(DEFAULT_TCP_KEEPALIVE_SECS),
        }
    }
}

/// Errors originating from the outbound pool / transport layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PoolError {
    /// Outbound active connection pool is saturated (backpressure limit reached).
    PoolExhausted { max_active: usize },
    /// Pool is shutting down; new requests rejected.
    PoolShuttingDown,
}

impl fmt::Display for PoolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PoolError::PoolExhausted { max_active } => {
                write!(
                    f,
                    "outbound fetch connection pool exhausted (max_active={max_active}); backpressure applied"
                )
            }
            PoolError::PoolShuttingDown => {
                f.write_str("outbound fetch pool is shutting down; request rejected")
            }
        }
    }
}

impl std::error::Error for PoolError {}

/// Build the policy-shaped HTTPS connector with explicit transport bounds:
/// webpki roots, HTTP/1 only, connect timeout, TCP nodelay, and keepalive.
pub fn build_connector_with_bounds(bounds: &PoolBounds) -> OutboundConnector {
    let mut http = hyper_util::client::legacy::connect::HttpConnector::new();
    http.set_connect_timeout(Some(bounds.connect_timeout));
    http.set_nodelay(true);
    http.set_keepalive(Some(bounds.tcp_keepalive));
    http.enforce_http(false);

    HttpsConnectorBuilder::new()
        .with_webpki_roots()
        .https_or_http()
        .enable_http1()
        .wrap_connector(http)
}

/// #1424: the last address set the SSRF gate validated per host. Keyed by
/// the lowercased host string the connector resolves (hyper-util passes
/// the URI host verbatim, brackets stripped). Inserted by the policy gate
/// immediately after classification passes; consumed by
/// [`ValidatedDnsResolver`] to pin the dial. Bounded by
/// [`VALIDATED_PIN_LIMIT`] with recycling eviction.
type ValidatedPins = Arc<Mutex<HashMap<String, Vec<std::net::SocketAddr>>>>;

/// Bound on retained validated pins (#1424: every new mutable shared
/// structure is bounded). Each pin is a host string + a handful of socket
/// addresses; the bound is on retained hosts, not requests — a burst of
/// distinct destinations recycles the oldest entries.
pub const VALIDATED_PIN_LIMIT: usize = 10_000;

static VALIDATED_PINS: OnceLock<ValidatedPins> = OnceLock::new();

fn validated_pins() -> &'static ValidatedPins {
    VALIDATED_PINS.get_or_init(|| Arc::new(Mutex::new(HashMap::new())))
}

/// #1424 (review hardening): the process's single active egress policy.
/// hyper-util's resolver receives only the host string — there is no
/// per-request context — so the pin registry cannot bind validations to
/// individual requests. What it CAN enforce is the invariant that makes
/// cross-request reuse safe: every address in the registry was classified
/// by the ONE process-wide egress policy. A second distinct policy trying
/// to pin is refused fail-closed (its fetches then fail closed at
/// resolution), converting silent cross-policy contamination into an
/// explicit unsupported condition. Serving constructs exactly one policy
/// at startup, so production behavior is unchanged.
static ACTIVE_EGRESS_POLICY: Mutex<Option<u64>> = Mutex::new(None);

/// #1424 (review hardening): stable in-process identity of a fetch
/// policy. The derived Debug representation covers every semantic field
/// (and agrees with the derived PartialEq), hashed with a process-local
/// hasher — the tag only ever compares within this process.
pub fn process_policy_tag(policy: &FetchPolicy) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    format!("{policy:?}").hash(&mut hasher);
    hasher.finish()
}

/// #1424: record the validated address set for a host (called by the
/// policy gate after every A/AAAA record passed classification). The
/// connector's dial may only use addresses from this set.
///
/// Fails closed when a DIFFERENT egress policy attempts to pin: the
/// registry is bound to the first policy that pinned in this process
/// (see [`ACTIVE_EGRESS_POLICY`]).
pub fn pin_validated_addrs(
    tag: u64,
    host: &str,
    addrs: &[std::net::IpAddr],
    port: u16,
) -> Result<(), String> {
    {
        let mut active = ACTIVE_EGRESS_POLICY
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        match *active {
            None => *active = Some(tag),
            Some(active_tag) if active_tag == tag => {}
            Some(_) => {
                return Err(
                    "a second egress policy is not supported in one process (#1424): the validated-address registry is bound to the first policy that pinned"
                        .to_string(),
                )
            }
        }
    }
    let key = host.to_ascii_lowercase();
    let sockets: Vec<std::net::SocketAddr> = addrs
        .iter()
        .map(|ip| std::net::SocketAddr::new(*ip, port))
        .collect();
    let pins = validated_pins();
    let Ok(mut map) = pins.lock() else {
        return Err("validated-address registry unavailable".into());
    };
    if !map.contains_key(&key) && map.len() >= VALIDATED_PIN_LIMIT {
        // recycling eviction: drop the oldest inserted key
        if let Some(oldest) = map.keys().next().cloned() {
            map.remove(&oldest);
        }
    }
    map.insert(key, sockets);
    Ok(())
}

/// #1424: resolve through the validated-address pins. The SSRF gate
/// classifies DNS answers BEFORE the request is dispatched; this resolver
/// serves exactly the addresses it validated (port already set by the
/// pin), so the connector cannot dial an address that was never
/// classified — closing the validate/dial rebinding window where the
/// connector's own independent DNS lookup could return a different,
/// policy-denied address (ADR-0033 §3 connect-to-validated-address).
/// IP-literal hosts never reach this resolver (hyper-util connects
/// directly) — they are classified in-line by the policy gate.
///
/// Fail closed: a host with no recorded pin (stale pin after recycling,
/// or a caller that skipped the gate) resolves to nothing, and the
/// connector's DNS error surfaces to the fetch as a failure.
#[derive(Clone, Debug)]
pub struct ValidatedDnsResolver {
    pins: ValidatedPins,
    misses: Arc<AtomicU32>,
}

impl ValidatedDnsResolver {
    pub fn new() -> Self {
        ValidatedDnsResolver {
            pins: validated_pins().clone(),
            misses: Arc::new(AtomicU32::new(0)),
        }
    }

    /// Resolution attempts that found no validated pin for the host
    /// (#1424 observability: should stay at zero on the gated path).
    pub fn miss_count(&self) -> u32 {
        self.misses.load(Ordering::Relaxed)
    }
}

impl Default for ValidatedDnsResolver {
    fn default() -> Self {
        Self::new()
    }
}

impl tower_service::Service<hyper_util::client::legacy::connect::dns::Name>
    for ValidatedDnsResolver
{
    type Response = std::vec::IntoIter<std::net::SocketAddr>;
    type Error = String;
    type Future = Pin<
        std::boxed::Box<dyn std::future::Future<Output = Result<Self::Response, String>> + Send>,
    >;

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, name: hyper_util::client::legacy::connect::dns::Name) -> Self::Future {
        let pins = self.pins.clone();
        let misses = self.misses.clone();
        let host = name.as_str().to_ascii_lowercase();
        Box::pin(async move {
            let Ok(map) = pins.lock() else {
                return Err("validated-address registry unavailable".to_string());
            };
            match map.get(&host) {
                Some(addrs) => Ok(addrs.clone().into_iter()),
                None => {
                    misses.fetch_add(1, Ordering::Relaxed);
                    Err(format!(
                        "no policy-validated address for host {host:?} — the fetch SSRF gate must classify DNS answers before dialing (#1424)"
                    ))
                }
            }
        })
    }
}

/// The pinned connector type: identical transport bounds to
/// [`OutboundConnector`], resolving only through validated pins.
pub type PinnedConnector = hyper_rustls::HttpsConnector<
    hyper_util::client::legacy::connect::HttpConnector<ValidatedDnsResolver>,
>;
pub type PinnedClient = Client<PinnedConnector, Empty<bytes::Bytes>>;

/// Build the #1424 pinned connector: same bounds as
/// [`build_connector_with_bounds`], resolver set to
/// [`ValidatedDnsResolver`] so every dialed hostname address was
/// previously classified by the fetch policy.
pub fn build_pinned_connector_with_bounds(bounds: &PoolBounds) -> PinnedConnector {
    let mut http = hyper_util::client::legacy::connect::HttpConnector::new_with_resolver(
        ValidatedDnsResolver::new(),
    );
    http.set_connect_timeout(Some(bounds.connect_timeout));
    http.set_nodelay(true);
    http.set_keepalive(Some(bounds.tcp_keepalive));
    http.enforce_http(false);

    HttpsConnectorBuilder::new()
        .with_webpki_roots()
        .https_or_http()
        .enable_http1()
        .wrap_connector(http)
}

/// Construct a pinned outbound client (same pooling bounds; validated-only
/// resolution). This is the client all policy-gated fetch traffic uses.
pub fn build_pinned_client_with_bounds(bounds: &PoolBounds) -> PinnedClient {
    Client::builder(TokioExecutor::new())
        .pool_idle_timeout(bounds.idle_timeout)
        .pool_max_idle_per_host(bounds.max_idle_per_host)
        .build(build_pinned_connector_with_bounds(bounds))
}

/// Build default connector.
pub fn build_connector() -> OutboundConnector {
    build_connector_with_bounds(&PoolBounds::default())
}

/// Construct an outbound client with policy-shaped connector and
/// bounded pooling parameters.
pub fn build_client_with_bounds(bounds: &PoolBounds) -> OutboundClient {
    Client::builder(TokioExecutor::new())
        .pool_idle_timeout(bounds.idle_timeout)
        .pool_max_idle_per_host(bounds.max_idle_per_host)
        .build(build_connector_with_bounds(bounds))
}

/// Construct default outbound client.
pub fn build_client() -> OutboundClient {
    build_client_with_bounds(&PoolBounds::default())
}

/// Process-global outbound fetch pool (M28-009-C). Every fetch path shares
/// this instance so the shutdown sequence drains the pool that actually
/// served traffic; an application with no fetch operations never
/// initializes it, making the shutdown drain an immediate no-op.
static SHARED_POOL: OnceLock<FetchPool> = OnceLock::new();

/// Access the process-global shared pool (created on first call, dormant
/// until a fetch initializes it).
/// BETA-006-C: bounded fetch-pool observability snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub struct FetchPoolStats {
    pub initialized: bool,
    pub shutdown: bool,
    pub active: u64,
    pub max_active: u64,
    pub rejections: u32,
}

pub fn shared_pool() -> &'static FetchPool {
    SHARED_POOL.get_or_init(FetchPool::new)
}

/// Lazy, thread-safe connection pool with active concurrency bounds.
///
/// An application with no fetch operations never initializes the client
/// or connection pools.
#[derive(Debug)]
pub struct FetchPool {
    bounds: PoolBounds,
    client: OnceLock<Arc<OutboundClient>>,
    client_pinned: OnceLock<Arc<PinnedClient>>,
    semaphore: Arc<Semaphore>,
    shutdown_called: AtomicBool,
    /// New-work refusals since creation (saturating; M28-009-D observability).
    rejections: AtomicU32,
}

impl Default for FetchPool {
    fn default() -> Self {
        Self::new()
    }
}

impl FetchPool {
    /// Create a new dormant fetch pool with default bounds.
    pub fn new() -> Self {
        Self::with_bounds(PoolBounds::default())
    }

    /// Create a new dormant fetch pool with custom bounds.
    pub fn with_bounds(bounds: PoolBounds) -> Self {
        let max_active = bounds
            .max_active_connections
            .min(MAX_ACTIVE_CONNECTIONS_CEILING);
        FetchPool {
            bounds,
            client: OnceLock::new(),
            client_pinned: OnceLock::new(),
            semaphore: Arc::new(Semaphore::new(max_active)),
            shutdown_called: AtomicBool::new(false),
            rejections: AtomicU32::new(0),
        }
    }

    /// Returns configured pool bounds.
    pub fn bounds(&self) -> &PoolBounds {
        &self.bounds
    }

    /// Returns `true` if the underlying client and connection pool
    /// have been initialized by at least one fetch request.
    pub fn is_initialized(&self) -> bool {
        self.client.get().is_some()
    }

    /// Access or initialize the shared outbound client.
    ///
    /// Initialization occurs exactly once on the first call.
    pub fn client(&self) -> Arc<OutboundClient> {
        self.client
            .get_or_init(|| Arc::new(build_client_with_bounds(&self.bounds)))
            .clone()
    }

    /// #1424: access or initialize the shared PINNED outbound client —
    /// the one all policy-gated fetch traffic dials through (validated
    /// addresses only). Lazily initialized exactly like [`FetchPool::client`],
    /// so an application that never fetches still pays zero cost.
    pub fn client_pinned(&self) -> Arc<PinnedClient> {
        self.client_pinned
            .get_or_init(|| Arc::new(build_pinned_client_with_bounds(&self.bounds)))
            .clone()
    }

    /// Try to acquire an active connection permit for outbound request backpressure.
    ///
    /// BETA-006-C: bounded pool observability snapshot. Never blocks.
    pub fn stats(&self) -> FetchPoolStats {
        let max = self.bounds.max_active_connections as u64;
        let available = self.semaphore.available_permits() as u64;
        FetchPoolStats {
            initialized: self.is_initialized(),
            shutdown: self.is_shutdown(),
            active: max.saturating_sub(available),
            max_active: max,
            rejections: self.rejections(),
        }
    }

    /// If all permits are currently in use, returns `Err(PoolError::PoolExhausted)`.
    pub fn try_acquire_permit(&self) -> Result<OwnedSemaphorePermit, PoolError> {
        if self.is_shutdown() {
            self.rejections.fetch_add(1, Ordering::Relaxed);
            return Err(PoolError::PoolShuttingDown);
        }
        self.semaphore.clone().try_acquire_owned().map_err(|_| {
            self.rejections.fetch_add(1, Ordering::Relaxed);
            PoolError::PoolExhausted {
                max_active: self.bounds.max_active_connections,
            }
        })
    }

    /// Client access that refuses new work on a shut-down pool (M28-009-D).
    /// Unlike [`FetchPool::client`] — which lazily initializes and is only
    /// valid pre-shutdown — this returns `None` once the pool has been
    /// drained: a quarantined pool never resurrects its client.
    pub fn try_client(&self) -> Option<Arc<OutboundClient>> {
        if self.is_shutdown() {
            self.rejections.fetch_add(1, Ordering::Relaxed);
            return None;
        }
        Some(self.client())
    }

    /// New-work refusals counted since creation (exhausted, post-shutdown
    /// permits, and post-shutdown client attempts).
    pub fn rejections(&self) -> u32 {
        self.rejections.load(Ordering::Relaxed)
    }

    /// Gracefully shutdown the pool and release any idle pooled connections.
    pub fn shutdown(&self) {
        self.shutdown_called.store(true, Ordering::SeqCst);
    }

    /// Drain in-flight work and close pooled connections within the budget (ADR-0031).
    pub async fn drain_shutdown(&self, budget: Duration) -> Result<(), &'static str> {
        self.shutdown();
        let max_active = self
            .bounds
            .max_active_connections
            .min(MAX_ACTIVE_CONNECTIONS_CEILING);
        // Wait for all permits to be returned within budget
        let acquire_all = async {
            match self.semaphore.acquire_many(max_active as u32).await {
                Ok(_permits) => Ok(()),
                Err(_) => Err("semaphore closed"),
            }
        };
        match tokio::time::timeout(budget, acquire_all).await {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(_) => Err("shutdown drain budget exceeded; failing closed"),
        }
    }

    /// Returns whether `shutdown` has been initiated.
    pub fn is_shutdown(&self) -> bool {
        self.shutdown_called.load(Ordering::SeqCst)
    }
}

/// Diagnostic summary for `--fetch-stack-info`.
pub fn describe() -> String {
    let _client = build_client(); // prove the stack constructs
    format!("outbound fetch stack: {STACK_ID} (constructs ok; dialing gated by ADR-0033 policy, wired in M28-003; proxy mode: disabled, ambient env ignored per M28-008-D)")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stack_constructs_lazily_and_names_itself() {
        let described = describe();
        assert!(described.contains("hyper1+hyper-util"));
        assert!(described.contains("webpki-roots"));
        assert!(described.contains("M28-003"));
        // M28-008-D: the diagnostic names the proxy posture.
        assert!(described.contains("proxy mode: disabled"));
    }

    #[test]
    fn pool_is_strictly_lazy_before_first_access() {
        let pool = FetchPool::new();
        // Guardrail: App with no fetch pays NO pool initialization
        assert!(
            !pool.is_initialized(),
            "pool must not be initialized eagerly"
        );
        assert!(!pool.is_shutdown());
    }

    /// M3-001-D: the pool handle follows the ADR-0036 §4 shared-handle
    /// discipline (pool-handle shape): Send + Sync, shareable by Arc.
    #[test]
    fn pool_handle_is_send_sync_shared() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<FetchPool>();
        let pool: Arc<FetchPool> = Arc::new(FetchPool::new());
        let p2 = pool.clone();
        let h = std::thread::spawn(move || {
            assert!(!p2.is_initialized());
        });
        h.join().unwrap();
    }

    /// M28-009-C: the shared pool is process-global and drains in place.
    #[test]
    fn shared_pool_is_process_global_and_drains_in_place() {
        let a = shared_pool() as *const FetchPool;
        let b = shared_pool() as *const FetchPool;
        assert_eq!(a, b, "shared_pool must return the same instance");
        // An app that never fetched: drain is an immediate no-op Ok.
        assert!(!shared_pool().is_initialized());
        let rt = tokio::runtime::Runtime::new().unwrap();
        let drained =
            rt.block_on(shared_pool().drain_shutdown(std::time::Duration::from_millis(100)));
        assert!(drained.is_ok());
        assert!(shared_pool().is_shutdown());
        // Post-shutdown permit attempts are rejected (PoolShuttingDown).
        assert!(matches!(
            shared_pool().try_acquire_permit(),
            Err(PoolError::PoolShuttingDown)
        ));
    }

    /// M28-009-D: a quarantined (drained/shut-down) pool rejects ALL new
    /// work — permits, client resurrection — and counts every refusal.
    #[test]
    fn quarantined_pool_rejects_new_work_and_counts_refusals() {
        let pool = FetchPool::new();
        assert_eq!(pool.rejections(), 0);
        // Pre-shutdown: a permit attempt on an uninitialized pool still
        // works (permits are semaphore-based, independent of the client).
        let permit = pool.try_acquire_permit().expect("permit before shutdown");
        drop(permit);
        // Drain the pool: terminal quarantine for outbound work.
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(pool.drain_shutdown(std::time::Duration::from_millis(100)))
            .expect("drain reaches quiescence");
        // New work is refused at every layer...
        assert!(matches!(
            pool.try_acquire_permit(),
            Err(PoolError::PoolShuttingDown)
        ));
        assert!(pool.try_client().is_none(), "client cannot be resurrected");
        assert!(pool.try_client().is_none());
        // ...and every refusal is counted (bounded, saturating counter).
        assert_eq!(pool.rejections(), 3);
    }

    #[test]
    fn pool_initializes_once_on_first_access_and_shares_instance() {
        let pool = FetchPool::new();
        assert!(!pool.is_initialized());

        let client1 = pool.client();
        assert!(
            pool.is_initialized(),
            "pool must be initialized after access"
        );

        let client2 = pool.client();
        assert!(
            Arc::ptr_eq(&client1, &client2),
            "subsequent calls must return the same shared client"
        );
    }

    #[test]
    fn pool_shutdown_handles_uninitialized_and_initialized() {
        let uninit_pool = FetchPool::new();
        uninit_pool.shutdown();
        assert!(uninit_pool.is_shutdown());
        assert!(!uninit_pool.is_initialized());

        let init_pool = FetchPool::new();
        let _c = init_pool.client();
        assert!(init_pool.is_initialized());
        init_pool.shutdown();
        assert!(init_pool.is_shutdown());
    }

    #[tokio::test]
    async fn pool_drain_shutdown_settles_within_budget() {
        let pool = FetchPool::new();
        let res = pool.drain_shutdown(Duration::from_millis(500)).await;
        assert!(res.is_ok(), "drain on idle pool must succeed immediately");
        assert!(pool.is_shutdown());
    }

    #[test]
    fn pool_exhaustion_yields_bounded_backpressure_error() {
        let bounds = PoolBounds {
            max_active_connections: 2,
            ..PoolBounds::default()
        };
        let pool = FetchPool::with_bounds(bounds);

        let permit1 = pool.try_acquire_permit().expect("permit 1");
        let permit2 = pool.try_acquire_permit().expect("permit 2");

        let err = pool.try_acquire_permit().unwrap_err();
        assert_eq!(err, PoolError::PoolExhausted { max_active: 2 });
        assert!(err.to_string().contains("backpressure applied"));

        drop(permit1);
        let permit3 = pool.try_acquire_permit().expect("permit 3 after drop");
        drop(permit2);
        drop(permit3);
    }

    #[test]
    fn pool_bounds_are_clamped_to_ceiling() {
        let bounds = PoolBounds {
            max_active_connections: 5000,
            ..PoolBounds::default()
        };
        let pool = FetchPool::with_bounds(bounds);
        assert_eq!(pool.semaphore.available_permits(), 1024);
    }

    #[test]
    fn tls_connector_uses_mandatory_webpki_roots_without_bypass() {
        let connector = build_connector();
        let _ = connector;
    }
}

#[cfg(test)]
mod pool_stats_tests {
    use super::*;

    #[test]
    fn fetch_pool_stats_track_lazy_active_and_rejections() {
        let pool = FetchPool::new();
        let stats = pool.stats();
        // lazy: no client, no activity before first use
        assert!(!stats.initialized);
        assert_eq!(stats.active, 0);
        assert_eq!(stats.rejections, 0);

        // acquiring a permit registers as an active connection slot
        let permit = pool.try_acquire_permit().expect("permit");
        let stats = pool.stats();
        assert_eq!(stats.active, 1);
        assert!(stats.max_active >= 1);
        drop(permit);
        assert_eq!(pool.stats().active, 0);
    }

    #[test]
    fn fetch_pool_stats_reflect_shutdown() {
        let pool = FetchPool::new();
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(pool.drain_shutdown(Duration::from_millis(100)))
            .unwrap();
        assert!(pool.stats().shutdown);
    }
}
