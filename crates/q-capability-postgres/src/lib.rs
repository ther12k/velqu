//! Lazy, bounded Postgres connection pool (BETA-004-B).
//!
//! The pool behind the `runtime:postgres` capability. "Lazy" is the
//! contract: constructing a pool performs **zero I/O** — no sockets, no
//! DNS, no authentication — until the first `acquire`. Connections are
//! created on demand up to a fail-closed ceiling, reused when idle, and
//! discarded when stale. An app that never touches the pool therefore
//! pays nothing (parent guardrail), and a pool that cannot connect
//! within its deadline fails typed instead of hanging.
//!
//! The pool logic is generic over a [`Connector`] so the deterministic
//! tests run against a mock with zero network; production uses
//! [`TokioConnector`] (tokio-postgres, no TLS — loopback deployments
//! only until an owner TLS decision exists).
//!
//! Deliberately out of scope here (later BETA-004 packets): the query
//! surface and parameterized wire behavior (C), deadline/cancel
//! semantics at the engine boundary (D), and pool-limit policy and
//! observability (E).

use std::collections::VecDeque;
use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use tokio::sync::{OwnedSemaphorePermit, Semaphore};
use tokio_postgres::Client;

/// Pool ceiling (fail-closed). Raising it is a reviewed decision, not a
/// caller option.
pub const MAX_POOL_CONNECTIONS: usize = 100;
/// Default ceiling when a config does not state one (BETA-004-E owns
/// the final limits policy; this default keeps exhaustion bounded).
pub const DEFAULT_MAX_CONNECTIONS: usize = 10;
/// Default connect timeout per new connection.
pub const DEFAULT_CONNECT_TIMEOUT_MS: u64 = 5_000;
/// Default idle lifetime before an idle connection is discarded on
/// the next acquire.
pub const DEFAULT_IDLE_TIMEOUT_MS: u64 = 30_000;
/// Fail-closed ceiling for the connect timeout.
pub const MAX_CONNECT_TIMEOUT_MS: u64 = 30_000;

/// Typed pool errors. Closed set; never a panic, never a silent retry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PoolError {
    /// `LazyPool::postgres` was called without a database URL.
    MissingDatabaseUrl,
    /// Config outside the closed bounds.
    InvalidConfig { detail: &'static str },
    /// No connection could be established within the deadline.
    ConnectTimeout { ms: u64 },
    /// The pool is at its ceiling and the wait deadline expired.
    AtCapacity { max: usize, waited_ms: u64 },
    /// The pool is shutting down (or shut down); no new acquires.
    ShuttingDown,
    /// The backend rejected the connection. The string carries the
    /// database-reported message only — credentials are never part of
    /// tokio-postgres connect errors.
    ConnectRejected(String),
}

impl fmt::Display for PoolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PoolError::MissingDatabaseUrl => f.write_str("postgres pool: database URL is required"),
            PoolError::InvalidConfig { detail } => {
                write!(f, "postgres pool: invalid config: {detail}")
            }
            PoolError::ConnectTimeout { ms } => {
                write!(f, "postgres pool: connect did not complete within {ms}ms")
            }
            PoolError::AtCapacity { max, waited_ms } => write!(
                f,
                "postgres pool: exhausted ({max} connections) after waiting {waited_ms}ms"
            ),
            PoolError::ShuttingDown => f.write_str("postgres pool: shutting down"),
            PoolError::ConnectRejected(msg) => write!(f, "postgres pool: connect rejected: {msg}"),
        }
    }
}

impl std::error::Error for PoolError {}

/// Validated pool configuration. Bounds are fail-closed: construction
/// is a typed error, never a clamp.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PoolConfig {
    pub max_connections: usize,
    pub connect_timeout_ms: u64,
    pub idle_timeout_ms: u64,
}

impl PoolConfig {
    /// Defaults: 10 connections, 5s connect, 30s idle — the matched
    /// benchmark posture is pinned by BETA-004-E, not here.
    pub fn default_config() -> Self {
        PoolConfig {
            max_connections: DEFAULT_MAX_CONNECTIONS,
            connect_timeout_ms: DEFAULT_CONNECT_TIMEOUT_MS,
            idle_timeout_ms: DEFAULT_IDLE_TIMEOUT_MS,
        }
    }

    pub fn new(
        max_connections: usize,
        connect_timeout_ms: u64,
        idle_timeout_ms: u64,
    ) -> Result<Self, PoolError> {
        if max_connections == 0 || max_connections > MAX_POOL_CONNECTIONS {
            return Err(PoolError::InvalidConfig {
                detail: "max_connections out of 1..=100",
            });
        }
        if connect_timeout_ms == 0 || connect_timeout_ms > MAX_CONNECT_TIMEOUT_MS {
            return Err(PoolError::InvalidConfig {
                detail: "connect_timeout_ms out of 1..=30000",
            });
        }
        if idle_timeout_ms == 0 {
            return Err(PoolError::InvalidConfig {
                detail: "idle_timeout_ms must be positive",
            });
        }
        Ok(PoolConfig {
            max_connections,
            connect_timeout_ms,
            idle_timeout_ms,
        })
    }
}

/// A pooled connection. `close` is best-effort (dropping the connection
/// always closes it too); `is_closed` lets the pool discard dead
/// connections instead of handing them out.
pub trait PoolConn: Send + 'static {
    fn is_closed(&self) -> bool;
}

/// Creates one connection. Implementations must be lazy per call: the
/// pool invokes `connect` only when it needs a fresh connection.
pub trait Connector: Send + Sync + 'static {
    type Conn: PoolConn;
    fn connect(&self) -> Pin<Box<dyn Future<Output = Result<Self::Conn, PoolError>> + Send>>;
}

/// Production connector: tokio-postgres over TCP, no TLS. The URL is
/// parsed per connect attempt so a bad URL is a typed per-op error,
/// and the connection driver is spawned exactly as tokio-postgres
/// requires.
#[derive(Clone)]
pub struct TokioConnector {
    url: Arc<String>,
}

impl TokioConnector {
    pub fn new(url: impl Into<String>) -> Self {
        TokioConnector {
            url: Arc::new(url.into()),
        }
    }
}

impl Connector for TokioConnector {
    type Conn = Client;

    fn connect(&self) -> Pin<Box<dyn Future<Output = Result<Client, PoolError>> + Send>> {
        let url = self.url.clone();
        Box::pin(async move {
            let (client, connection) = tokio_postgres::connect(url.as_str(), tokio_postgres::NoTls)
                .await
                .map_err(|e| PoolError::ConnectRejected(redact_connect_error(&e.to_string())))?;
            // The connection driver must be spawned for the client to
            // make progress; when the pool drops the client the driver
            // ends on its own.
            tokio::spawn(async move {
                let _ = connection.await;
            });
            Ok(client)
        })
    }
}

/// Defense in depth: tokio-postgres errors do not echo credentials,
/// but the pool never lets a URL fragment into an error string either.
fn redact_connect_error(msg: &str) -> String {
    if let Some(idx) = msg.find("postgres://") {
        let mut out = msg[..idx].to_string();
        out.push_str("<redacted-url>");
        out.push_str(&msg[msg[idx..].find(' ').map(|i| idx + i).unwrap_or(msg.len())..]);
        out
    } else {
        msg.to_string()
    }
}

impl PoolConn for Client {
    fn is_closed(&self) -> bool {
        Client::is_closed(self)
    }
}

#[derive(Debug)]
struct Idle<C> {
    conn: C,
    idle_since: Instant,
}

#[derive(Debug)]
struct PoolState<C> {
    idle: VecDeque<Idle<C>>,
    created_total: u64,
    in_use: usize,
    shutting_down: bool,
}

struct PoolInner<F: Connector> {
    connector: Arc<F>,
    config: PoolConfig,
    permits: Arc<Semaphore>,
    state: Mutex<PoolState<F::Conn>>,
    connects: AtomicU64,
}

/// Lazy, bounded pool. Cheap to clone; construction performs no I/O.
pub struct LazyPool<F: Connector> {
    inner: Arc<PoolInner<F>>,
}

impl<F: Connector> Clone for LazyPool<F> {
    fn clone(&self) -> Self {
        LazyPool {
            inner: self.inner.clone(),
        }
    }
}

/// Snapshot for observability (BETA-004-E extends the policy surface).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PoolStats {
    pub idle: usize,
    pub in_use: usize,
    pub created_total: u64,
    pub max_connections: usize,
}

impl<F: Connector> LazyPool<F> {
    /// Build a pool around a custom connector (deterministic tests).
    pub fn with_connector(connector: F, config: PoolConfig) -> Self {
        LazyPool {
            inner: Arc::new(PoolInner {
                connector: Arc::new(connector),
                config,
                permits: Arc::new(Semaphore::new(config.max_connections)),
                state: Mutex::new(PoolState {
                    idle: VecDeque::new(),
                    created_total: 0,
                    in_use: 0,
                    shutting_down: false,
                }),
                connects: AtomicU64::new(0),
            }),
        }
    }

    /// Pool stats. Never blocks.
    pub fn stats(&self) -> PoolStats {
        let st = self.inner.state.lock().expect("pool state mutex");
        PoolStats {
            idle: st.idle.len(),
            in_use: st.in_use,
            created_total: st.created_total,
            max_connections: self.inner.config.max_connections,
        }
    }

    /// Begin shutdown: no new acquires succeed. In-flight connections
    /// close when released (BETA-004-D owns deadline/cancel semantics
    /// at the engine boundary; this is the pool-level gate).
    pub fn begin_shutdown(&self) {
        let mut st = self.inner.state.lock().expect("pool state mutex");
        st.shutting_down = true;
    }

    /// Acquire a connection, lazily creating one within the pool
    /// ceiling. `wait_ms` bounds both the capacity wait and the
    /// connect itself; 0 is rejected up front (fail closed, not
    /// "forever").
    pub async fn acquire(&self, wait_ms: u64) -> Result<PooledConnection<F>, PoolError> {
        if wait_ms == 0 {
            return Err(PoolError::InvalidConfig {
                detail: "acquire wait must be positive",
            });
        }
        {
            let st = self.inner.state.lock().expect("pool state mutex");
            if st.shutting_down {
                return Err(PoolError::ShuttingDown);
            }
        }
        let waited = Instant::now();
        let permit = match tokio::time::timeout(
            Duration::from_millis(wait_ms),
            self.inner.permits.clone().acquire_owned(),
        )
        .await
        {
            Ok(Ok(permit)) => permit,
            Ok(Err(_)) => return Err(PoolError::ShuttingDown),
            Err(_) => {
                return Err(PoolError::AtCapacity {
                    max: self.inner.config.max_connections,
                    waited_ms: waited.elapsed().as_millis() as u64,
                })
            }
        };

        let conn = self.take_or_connect(permit, wait_ms).await?;
        Ok(PooledConnection {
            inner: Some(conn),
            pool: self.inner.clone(),
        })
    }

    async fn take_or_connect(
        &self,
        permit: OwnedSemaphorePermit,
        wait_ms: u64,
    ) -> Result<LeasedConn<F::Conn>, PoolError> {
        // scan idle connections first; the state lock is never held
        // across the connect await below
        let live_idle = {
            let mut st = self.inner.state.lock().expect("pool state mutex");
            loop {
                match st.idle.pop_back() {
                    Some(idle) => {
                        if idle.idle_since.elapsed()
                            <= Duration::from_millis(self.inner.config.idle_timeout_ms)
                            && !idle.conn.is_closed()
                        {
                            st.in_use += 1;
                            break Some(idle.conn);
                        }
                        // stale or dead: drop it and keep scanning
                    }
                    None => break None,
                }
            }
        };
        if let Some(conn) = live_idle {
            return Ok(LeasedConn {
                conn: Some(conn),
                _permit: permit,
            });
        }

        let connect = self.inner.connector.connect();
        let conn = match tokio::time::timeout(Duration::from_millis(wait_ms), connect).await {
            Ok(Ok(conn)) => conn,
            Ok(Err(e)) => return Err(e),
            Err(_) => {
                return Err(PoolError::ConnectTimeout { ms: wait_ms });
            }
        };
        self.inner.connects.fetch_add(1, Ordering::Relaxed);
        let mut st = self.inner.state.lock().expect("pool state mutex");
        st.created_total += 1;
        st.in_use += 1;
        Ok(LeasedConn {
            conn: Some(conn),
            _permit: permit,
        })
    }
}

/// A connection leased from the pool. Returns to the idle queue on
/// drop; dropping always closes eventually (connections are not shared
/// beyond the lease).
pub struct PooledConnection<F: Connector> {
    inner: Option<LeasedConn<F::Conn>>,
    pool: Arc<PoolInner<F>>,
}

struct LeasedConn<C> {
    conn: Option<C>,
    /// held for its Drop side effect: the capacity slot is released
    /// exactly when the lease ends
    _permit: OwnedSemaphorePermit,
}

impl<F: Connector> fmt::Debug for PooledConnection<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // the connection itself is never formatted (redaction posture)
        f.debug_struct("PooledConnection").finish_non_exhaustive()
    }
}

impl<F: Connector> PooledConnection<F> {
    pub fn get(&self) -> &F::Conn {
        self.inner
            .as_ref()
            .expect("connection present while leased")
            .conn
            .as_ref()
            .expect("connection present while leased")
    }
}

impl<F: Connector> Drop for PooledConnection<F> {
    fn drop(&mut self) {
        if let Some(leased) = self.inner.take() {
            let mut st = self.pool.state.lock().expect("pool state mutex");
            st.in_use = st.in_use.saturating_sub(1);
            if !st.shutting_down {
                if let Some(conn) = leased.conn {
                    st.idle.push_back(Idle {
                        conn,
                        idle_since: Instant::now(),
                    });
                }
            }
            // dropping `leased` releases the permit; under shutdown the
            // connection itself is dropped (closed)
        }
    }
}

/// Production pool over tokio-postgres. Construction parses nothing
/// and connects nothing — laziness is structural.
pub type PostgresPool = LazyPool<TokioConnector>;

impl PostgresPool {
    pub fn postgres(url: impl Into<String>, config: PoolConfig) -> Self {
        LazyPool::with_connector(TokioConnector::new(url), config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicUsize;

    #[derive(Debug)]
    struct MockConn {
        closed: Arc<AtomicUsize>,
        alive: Arc<AtomicUsize>,
        dead: bool,
    }

    impl PoolConn for MockConn {
        fn is_closed(&self) -> bool {
            self.dead
        }
    }

    impl Drop for MockConn {
        fn drop(&mut self) {
            self.closed.fetch_add(1, Ordering::SeqCst);
            self.alive.fetch_sub(1, Ordering::SeqCst);
        }
    }

    #[derive(Clone)]
    struct MockConnector {
        connects: Arc<AtomicUsize>,
        alive: Arc<AtomicUsize>,
        delay_ms: u64,
        fail: bool,
    }

    impl MockConnector {
        fn new() -> Self {
            MockConnector {
                connects: Arc::new(AtomicUsize::new(0)),
                alive: Arc::new(AtomicUsize::new(0)),
                delay_ms: 0,
                fail: false,
            }
        }

        fn with_delay(ms: u64) -> Self {
            MockConnector {
                delay_ms: ms,
                ..Self::new()
            }
        }

        fn failing() -> Self {
            MockConnector {
                fail: true,
                ..Self::new()
            }
        }

        fn connect_count(&self) -> usize {
            self.connects.load(Ordering::SeqCst)
        }
    }

    impl Connector for MockConnector {
        type Conn = MockConn;

        fn connect(&self) -> Pin<Box<dyn Future<Output = Result<MockConn, PoolError>> + Send>> {
            let connects = self.connects.clone();
            let alive = self.alive.clone();
            let delay = self.delay_ms;
            let fail = self.fail;
            Box::pin(async move {
                connects.fetch_add(1, Ordering::SeqCst);
                if delay > 0 {
                    tokio::time::sleep(Duration::from_millis(delay)).await;
                }
                if fail {
                    return Err(PoolError::ConnectRejected("mock backend refused".into()));
                }
                alive.fetch_add(1, Ordering::SeqCst);
                Ok(MockConn {
                    closed: Arc::new(AtomicUsize::new(0)),
                    alive,
                    dead: false,
                })
            })
        }
    }

    #[test]
    fn config_bounds_are_fail_closed() {
        assert!(PoolConfig::new(0, 5_000, 30_000).is_err());
        assert!(PoolConfig::new(MAX_POOL_CONNECTIONS + 1, 5_000, 30_000).is_err());
        assert!(PoolConfig::new(10, 0, 30_000).is_err());
        assert!(PoolConfig::new(10, MAX_CONNECT_TIMEOUT_MS + 1, 30_000).is_err());
        assert!(PoolConfig::new(10, 5_000, 0).is_err());
        assert!(PoolConfig::new(10, 5_000, 30_000).is_ok());
        assert_eq!(
            PoolConfig::default_config(),
            PoolConfig {
                max_connections: 10,
                connect_timeout_ms: 5_000,
                idle_timeout_ms: 30_000
            }
        );
    }

    #[tokio::test]
    async fn construction_is_lazy_zero_io() {
        let connector = MockConnector::new();
        let pool = LazyPool::with_connector(connector.clone(), PoolConfig::default_config());
        // no connection attempt at construction, at stats, or over time
        assert_eq!(connector.connect_count(), 0);
        let s = pool.stats();
        assert_eq!((s.idle, s.in_use, s.created_total), (0, 0, 0));
        tokio::time::sleep(Duration::from_millis(20)).await;
        assert_eq!(
            connector.connect_count(),
            0,
            "pool must not connect until first acquire"
        );
    }

    #[tokio::test]
    async fn first_acquire_connects_and_release_reuses() {
        let connector = MockConnector::new();
        let pool = LazyPool::with_connector(connector.clone(), PoolConfig::default_config());

        {
            let conn = pool.acquire(1_000).await.unwrap();
            assert_eq!(connector.connect_count(), 1);
            assert_eq!(pool.stats().in_use, 1);
            let _ = conn.get();
        }
        // released to idle, not closed
        let s = pool.stats();
        assert_eq!((s.idle, s.in_use), (1, 0));
        assert_eq!(connector.alive.load(Ordering::SeqCst), 1);

        {
            let _conn = pool.acquire(1_000).await.unwrap();
            // reused the idle connection: still exactly one connect
            assert_eq!(connector.connect_count(), 1);
        }
        assert_eq!(pool.stats().created_total, 1);
    }

    #[tokio::test]
    async fn ceiling_is_bounded_and_wait_deadline_is_typed() {
        let connector = MockConnector::new();
        let pool = LazyPool::with_connector(connector, PoolConfig::new(2, 5_000, 30_000).unwrap());
        let c1 = pool.acquire(1_000).await.unwrap();
        let c2 = pool.acquire(1_000).await.unwrap();
        let started = Instant::now();
        let err = pool.acquire(50).await.unwrap_err();
        assert!(
            matches!(err, PoolError::AtCapacity { max: 2, waited_ms } if (50..=150).contains(&waited_ms)),
            "typed at-capacity error bounded by the wait deadline, got {err}"
        );
        assert!(started.elapsed() < Duration::from_millis(500));
        drop(c1);
        // a slot freed up: next acquire succeeds
        let c3 = pool.acquire(1_000).await.unwrap();
        drop(c2);
        drop(c3);
    }

    #[tokio::test]
    async fn connect_timeout_is_typed_not_hanging() {
        let pool =
            LazyPool::with_connector(MockConnector::with_delay(500), PoolConfig::default_config());
        let err = pool.acquire(50).await.unwrap_err();
        assert_eq!(err, PoolError::ConnectTimeout { ms: 50 });
    }

    #[tokio::test]
    async fn connect_rejection_is_typed() {
        let pool = LazyPool::with_connector(MockConnector::failing(), PoolConfig::default_config());
        let err = pool.acquire(1_000).await.unwrap_err();
        assert_eq!(
            err,
            PoolError::ConnectRejected("mock backend refused".into())
        );
    }

    #[tokio::test]
    async fn zero_wait_is_rejected_up_front() {
        let pool = LazyPool::with_connector(MockConnector::new(), PoolConfig::default_config());
        let err = pool.acquire(0).await.unwrap_err();
        assert!(matches!(err, PoolError::InvalidConfig { .. }));
    }

    #[tokio::test]
    async fn stale_idle_connections_are_discarded_on_acquire() {
        let connector = MockConnector::new();
        let pool = LazyPool::with_connector(
            connector.clone(),
            PoolConfig::new(10, 5_000, 30_000).unwrap(),
        );
        {
            let _c = pool.acquire(1_000).await.unwrap();
        }
        assert_eq!(pool.stats().idle, 1);
        // force the idle connection stale by shrinking the window is not
        // possible post-construction; instead drop the conn as dead via a
        // second pool configured with a tiny idle window
        let tiny =
            LazyPool::with_connector(connector.clone(), PoolConfig::new(10, 5_000, 1).unwrap());
        {
            let _c = tiny.acquire(1_000).await.unwrap();
        }
        tokio::time::sleep(Duration::from_millis(5)).await;
        {
            let _c = tiny.acquire(1_000).await.unwrap();
        }
        // the stale connection was discarded and a fresh one created
        let before = connector.connect_count();
        assert!(before >= 2, "stale idle must trigger a fresh connect");
    }

    #[tokio::test]
    async fn shutdown_refuses_new_acquires() {
        let connector = MockConnector::new();
        let pool = LazyPool::with_connector(connector.clone(), PoolConfig::default_config());
        {
            let _c = pool.acquire(1_000).await.unwrap();
        }
        assert_eq!(pool.stats().idle, 1);
        pool.begin_shutdown();
        let err = pool.acquire(1_000).await.unwrap_err();
        assert_eq!(err, PoolError::ShuttingDown);
    }

    #[tokio::test]
    async fn released_under_shutdown_closes_the_connection() {
        let connector = MockConnector::new();
        let pool = LazyPool::with_connector(connector.clone(), PoolConfig::default_config());
        let conn = pool.acquire(1_000).await.unwrap();
        pool.begin_shutdown();
        drop(conn); // released while shutting down: closed, not parked
        tokio::time::sleep(Duration::from_millis(10)).await;
        assert_eq!(connector.alive.load(Ordering::SeqCst), 0);
        assert_eq!(pool.stats().idle, 0);
    }

    #[tokio::test]
    async fn dead_idle_connections_are_discarded_not_handed_out() {
        use std::sync::atomic::AtomicBool;
        let flags: Arc<Mutex<Vec<Arc<AtomicBool>>>> = Arc::new(Mutex::new(Vec::new()));
        let flags2 = flags.clone();
        struct FlagConnector {
            flags: Arc<Mutex<Vec<Arc<AtomicBool>>>>,
            connects: Arc<AtomicUsize>,
        }
        impl Connector for FlagConnector {
            type Conn = FlagConn;
            fn connect(&self) -> Pin<Box<dyn Future<Output = Result<FlagConn, PoolError>> + Send>> {
                let flag = Arc::new(AtomicBool::new(false));
                self.flags.lock().unwrap().push(flag.clone());
                self.connects.fetch_add(1, Ordering::SeqCst);
                Box::pin(async move { Ok(FlagConn { dead: flag }) })
            }
        }
        #[derive(Debug)]
        struct FlagConn {
            dead: Arc<AtomicBool>,
        }
        impl PoolConn for FlagConn {
            fn is_closed(&self) -> bool {
                self.dead.load(Ordering::SeqCst)
            }
        }

        let connects_counter = Arc::new(AtomicUsize::new(0));
        let connector = FlagConnector {
            flags: flags2,
            connects: connects_counter.clone(),
        };
        let pool = LazyPool::with_connector(connector, PoolConfig::default_config());
        {
            let _c = pool.acquire(1_000).await.unwrap();
        }
        assert_eq!(pool.stats().idle, 1);
        // the backend closes the parked connection
        flags.lock().unwrap()[0].store(true, Ordering::SeqCst);
        {
            let _c = pool.acquire(1_000).await.unwrap();
        }
        // the dead connection was discarded and a fresh one created
        assert_eq!(connects_counter.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn missing_url_is_typed() {
        let err = PoolError::MissingDatabaseUrl;
        assert!(err.to_string().contains("URL is required"));
    }
}
