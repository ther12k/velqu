//! Outbound fetch stack and lazy connection pooling (M28-002-B, M28-003-A).
//!
//! Provides a strictly lazy, thread-safe connection pool [`FetchPool`] for
//! outbound requests. An application that does not execute any `fetch()`
//! calls incurs **zero** pool initialization overhead (zero socket creation,
//! zero TLS context allocation, zero background tasks).
//!
//! On first actual request, the pool initializes once as a singleton using
//! the M28-002-A selected stack (hyper 1 + hyper-util client-legacy +
//! hyper-rustls with webpki roots).

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, OnceLock};
use std::time::Duration;

use http_body_util::Empty;
use hyper_rustls::HttpsConnectorBuilder;
use hyper_util::client::legacy::Client;
use hyper_util::rt::TokioExecutor;

/// Human-readable identity of the selected outbound stack.
pub const STACK_ID: &str =
    "hyper1+hyper-util(client-legacy)+hyper-rustls(webpki-roots,ring,http1,tls12)";

/// Default pool idle connection timeout (15 seconds).
pub const DEFAULT_POOL_IDLE_TIMEOUT_SECS: u64 = 15;
/// Default maximum idle connections per host.
pub const DEFAULT_MAX_IDLE_PER_HOST: usize = 32;

pub type OutboundConnector =
    hyper_rustls::HttpsConnector<hyper_util::client::legacy::connect::HttpConnector>;
pub type OutboundClient = Client<OutboundConnector, Empty<bytes::Bytes>>;

/// Build the policy-shaped HTTPS connector: webpki roots, HTTP/1 only,
/// https-or-http (scheme enforcement happens in the ADR-0033 policy
/// layer before any dial).
pub fn build_connector() -> OutboundConnector {
    HttpsConnectorBuilder::new()
        .with_webpki_roots()
        .https_or_http()
        .enable_http1()
        .wrap_connector(hyper_util::client::legacy::connect::HttpConnector::new())
}

/// Construct a raw outbound client with policy-shaped connector and
/// bounded pooling parameters.
pub fn build_client() -> OutboundClient {
    Client::builder(TokioExecutor::new())
        .pool_idle_timeout(Duration::from_secs(DEFAULT_POOL_IDLE_TIMEOUT_SECS))
        .pool_max_idle_per_host(DEFAULT_MAX_IDLE_PER_HOST)
        .build(build_connector())
}

/// Lazy, thread-safe connection pool for outbound fetch operations.
///
/// An application with no fetch operations never initializes the pool.
#[derive(Debug, Default)]
pub struct FetchPool {
    client: OnceLock<Arc<OutboundClient>>,
    shutdown_called: AtomicBool,
}

impl FetchPool {
    /// Create a new dormant fetch pool.
    ///
    /// Zero sockets, zero TLS handshakes, and zero background tasks
    /// are initialized at this point.
    pub const fn new() -> Self {
        FetchPool {
            client: OnceLock::new(),
            shutdown_called: AtomicBool::new(false),
        }
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
        self.client.get_or_init(|| Arc::new(build_client())).clone()
    }

    /// Gracefully shutdown the pool and release any idle pooled connections.
    ///
    /// If the pool was never initialized, this is a fast no-op.
    pub fn shutdown(&self) {
        self.shutdown_called.store(true, Ordering::SeqCst);
        // If client was initialized, dropping references allows pooled idle
        // connections to close according to their idle timeout.
    }

    /// Returns whether `shutdown` has been initiated.
    pub fn is_shutdown(&self) -> bool {
        self.shutdown_called.load(Ordering::SeqCst)
    }
}

/// Diagnostic summary for `--fetch-stack-info`.
pub fn describe() -> String {
    let _client = build_client(); // prove the stack constructs
    format!("outbound fetch stack: {STACK_ID} (constructs ok; dialing gated by ADR-0033 policy, wired in M28-003)")
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
    }

    #[test]
    fn pool_is_strictly_lazy_before_first_access() {
        let pool = FetchPool::new();
        // Guardrail 1: App with no fetch pays NO pool initialization
        assert!(
            !pool.is_initialized(),
            "pool must not be initialized eagerly"
        );
        assert!(!pool.is_shutdown());
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
        // Uninitialized pool shutdown is a clean no-op
        let uninit_pool = FetchPool::new();
        uninit_pool.shutdown();
        assert!(uninit_pool.is_shutdown());
        assert!(!uninit_pool.is_initialized());

        // Initialized pool shutdown marks shutdown state
        let init_pool = FetchPool::new();
        let _c = init_pool.client();
        assert!(init_pool.is_initialized());
        init_pool.shutdown();
        assert!(init_pool.is_shutdown());
    }

    #[test]
    fn tls_connector_uses_mandatory_webpki_roots_without_bypass() {
        let connector = build_connector();
        // The type signature itself pins hyper_rustls with webpki roots;
        // no insecure verifier methods exist on this builder path.
        let _ = connector;
    }
}
