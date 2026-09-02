//! q-engine-quickjs — quickjs-ng worker behind the q-engine boundary.
//!
//! One OS thread owns exactly one Runtime+Context (ADR-0008). All JS execution
//! happens there; the host side communicates via a channel. Native operations
//! (timer capability) run on tokio and complete back onto the worker loop as
//! messages, carrying no JS values across threads. Late completions for
//! settled/cancelled invocations are dropped (RUN-006, SEC-003).
//!
//! # State ownership (ADR-0036, M3-001-C)
//!
//! JavaScript values NEVER cross workers — and the type system enforces it:
//! [`rquickjs::Value`] holds reference-counted pointers into one runtime's
//! heap, so it is neither [`Send`] nor [`Sync`]. Moving one into another
//! thread is a compile error, demonstrated below. The only things that cross
//! worker boundaries are plain data: bytes, numbers, and channel senders.
//!
//! ```compile_fail
//! // Moving a JS value into another thread must not compile (ADR-0036 §5).
//! use rquickjs::{Context, Runtime};
//!
//! let rt = Runtime::new().unwrap();
//! let ctx = Context::full(&rt).unwrap();
//! let value = ctx.with(|ctx| ctx.eval("({ a: 1 })").unwrap());
//! std::thread::spawn(move || {
//!     // ERROR: `Value` contains `Rc` pointers and cannot be sent.
//!     let _stolen = value;
//! });
//! ```
//!
//! The dispatcher message contract is the positive half of the same rule:
//! only plain-data messages may cross worker boundaries.

mod convert;
pub mod prelude;
mod worker;

use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

use q_bridge::{BridgeCounters, CountersSnapshot};
use q_engine::{Engine, EngineStats, InvocationSpec, LoadStats, Outcome};
use worker::{WorkerMsg, WorkerShared};

/// Narrow read-only handle for lock-free readiness checks (M2.2.1-r4.2.1).
/// Exposes only the health queries needed by the HTTP host, keeping mutable
/// scheduler internals encapsulated.
#[derive(Clone)]
pub struct EngineHealth {
    shared: Arc<WorkerShared>,
}

impl EngineHealth {
    #[inline]
    pub fn is_quarantined(&self) -> bool {
        self.shared
            .queue_poisoned
            .load(std::sync::atomic::Ordering::Acquire)
    }

    #[inline]
    pub fn is_ready(&self) -> bool {
        !self.is_quarantined()
    }
}

/// M27-003-A: closed vocabulary of QuickJS intrinsic profiles. The
/// profile is selected at engine construction and can never change at
/// runtime; `Full` (JS_NewContext, every standard builtin) is the
/// default and matches all pre-M27 behavior exactly.
///
/// Profile contents (base objects are always present — rquickjs adds
/// them for every context):
/// - `Full` — JS_NewContext: Date, Eval, RegExp(+compiler), JSON,
///   Proxy, Map/Set, TypedArrays, Promise, Performance, WeakRef.
/// - `Web` — the Web-runtime reductions: everything except Date and
///   Performance (measured candidate; selected only if meaningful).
/// - `Minimal` — Eval + JSON + Promise + TypedArrays only: what the
///   Velqu host bridge itself needs (request/response JSON, bytes,
///   async ops). Apps touching Date/RegExp/Proxy/Map/Set see
///   undefined and fail loudly — that visibility is the point of
///   the later measurement packets (M27-011).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ContextProfile {
    #[default]
    Full,
    Web,
    Minimal,
}

impl ContextProfile {
    /// Closed-vocabulary parser. Fail-closed: no guessing, no aliasing.
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "full" => Some(ContextProfile::Full),
            "web" => Some(ContextProfile::Web),
            "minimal" => Some(ContextProfile::Minimal),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            ContextProfile::Full => "full",
            ContextProfile::Web => "web",
            ContextProfile::Minimal => "minimal",
        }
    }
}

/// Engine configuration (limits are robustness controls, not a sandbox).
#[derive(Debug, Clone)]
pub struct QuickJsConfig {
    pub heap_limit_bytes: usize,
    pub stack_limit_bytes: usize,
    pub pending_op_cap: usize,
    /// Watchdog for promise-continuation jobs when no invocation is pending
    /// (drain-time interrupt arming). Resource robustness control, not a sandbox.
    pub job_deadline_ms: u64,
    /// Per-drain microtask job cap (M2.2.1-r4.1). A live invocation whose
    /// queued microtasks exceed this count without quiescing quarantines the
    /// runtime through the unified terminal path.
    pub max_invocation_jobs: usize,
    /// Maximum number of live request slots owned by this worker. Runtime
    /// admission supplies the HTTP queue bound so the slab cannot outgrow it.
    pub request_slot_capacity: usize,
    /// M26-004-D: the pack's compiled module bytecode already contains the
    /// prelude and handler manifest, so startup performs zero prelude
    /// source evaluation — handles are collected after module eval.
    pub embedded_prelude: bool,
    /// M27-003-A: intrinsic set for the worker's QuickJS context.
    /// Default `Full` preserves all prior behavior byte-for-byte.
    pub profile: ContextProfile,
    /// M4A-007-A: maximum deferred callbacks retained per worker. Deferred
    /// work is best-effort and bounded; it is not a durable job queue.
    pub defer_queue_capacity: usize,
    /// M4A-007-A: deadline for one deferred callback during shutdown/drain.
    pub defer_deadline_ms: u64,
}

impl Default for QuickJsConfig {
    fn default() -> Self {
        QuickJsConfig {
            heap_limit_bytes: 32 << 20,
            stack_limit_bytes: 512 << 10,
            pending_op_cap: 1024,
            job_deadline_ms: 5_000,
            max_invocation_jobs: 100_000,
            request_slot_capacity: 256,
            embedded_prelude: false,
            profile: ContextProfile::Full,
            defer_queue_capacity: 64,
            defer_deadline_ms: 100,
        }
    }
}

/// Maps a generated bundle location to its original source position.
/// Supplied by the runtime from the pack's embedded source map.
pub trait SourceMapper: Send + Sync {
    fn map(&self, line: u32, col: u32) -> Option<q_engine::OriginalLocation>;
}

pub struct IdentityMapper;
impl SourceMapper for IdentityMapper {
    fn map(&self, _line: u32, _col: u32) -> Option<q_engine::OriginalLocation> {
        None
    }
}

pub struct QuickJsEngine {
    tx: std::sync::mpsc::Sender<WorkerMsg>,
    shared: Arc<WorkerShared>,
    handle: Option<JoinHandle<()>>,
    last_error: Arc<Mutex<Option<String>>>,
    bridge_counters: Arc<BridgeCounters>,
    /// M3-004-B: owner-thread label for diagnostics (e.g. "svc-0").
    worker_label: Option<String>,
}

impl QuickJsEngine {
    /// Spawn the single worker thread. `tokio_handle` drives native ops.
    pub fn spawn(
        config: QuickJsConfig,
        tokio_handle: tokio::runtime::Handle,
        mapper: Arc<dyn SourceMapper>,
    ) -> QuickJsEngine {
        let (tx, rx) = std::sync::mpsc::channel::<WorkerMsg>();
        let worker_tx = tx.clone();
        let shared = Arc::new(WorkerShared::new());
        let last_error = Arc::new(Mutex::new(None));
        let bridge_counters = Arc::new(BridgeCounters::default());
        let handle = std::thread::Builder::new()
            .name("velqu-quickjs".into())
            .spawn({
                let tx = worker_tx;
                let shared = Arc::clone(&shared);
                let last_error = Arc::clone(&last_error);
                let bridge_counters = Arc::clone(&bridge_counters);
                move || {
                    let inner = worker::WorkerInner::new(
                        config,
                        bridge_counters,
                        tokio_handle,
                        mapper,
                        shared.clone(),
                        last_error.clone(),
                    );
                    match inner {
                        Ok(w) => w.run(rx, tx),
                        Err(e) => {
                            *last_error.lock().unwrap() = Some(format!("engine init failed: {e}"));
                            // drain until shutdown so senders do not block
                            for msg in rx.iter() {
                                if let WorkerMsg::Shutdown = msg {
                                    break;
                                }
                            }
                        }
                    }
                }
            })
            .expect("spawn quickjs worker thread");
        QuickJsEngine {
            tx,
            shared,
            handle: Some(handle),
            last_error,
            bridge_counters,
            worker_label: None,
        }
    }

    /// Snapshot of worker-local request bridge counters. Only atomics cross
    /// the engine handle; request metadata and slab state remain worker-owned.
    pub fn bridge_snapshot(&self) -> CountersSnapshot {
        self.bridge_counters.snapshot()
    }

    /// M3-004-B: spawn N fully independent runtimes — each with its own
    /// thread, context, heap, and module state (ADR-0036 §1/§2). The
    /// `worker_name` labels the owner thread for diagnostics. No state is
    /// shared between the spawned engines beyond the caller's own handles.
    pub fn spawn_independent(
        count: usize,
        config: QuickJsConfig,
        tokio_handle: tokio::runtime::Handle,
        mapper: Arc<dyn SourceMapper>,
        worker_name: &str,
    ) -> Vec<QuickJsEngine> {
        (0..count)
            .map(|i| {
                let mut e =
                    QuickJsEngine::spawn(config.clone(), tokio_handle.clone(), Arc::clone(&mapper));
                e.worker_label = Some(format!("{worker_name}-{i}"));
                e
            })
            .collect()
    }

    /// M3-004-B: this engine's worker label (owner-thread diagnostics).
    pub fn worker_label(&self) -> Option<&str> {
        self.worker_label.as_deref()
    }

    /// Test/benchmark admission helper. Production request admission moves the
    /// metadata in `InvocationSpec`; this method exists only for suites that
    /// need to arrange a handle before constructing a legacy fixture spec.
    /// The returned typed handle is minted by the worker's own slab.
    #[doc(hidden)]
    pub fn insert_request(
        &self,
        meta: q_engine::RequestMeta,
    ) -> Result<q_bridge::RequestHandle, String> {
        let (tx, rx) = std::sync::mpsc::channel();
        self.tx
            .send(WorkerMsg::InsertRequest { meta, reply: tx })
            .map_err(|_| "engine worker gone".to_string())?;
        rx.recv().map_err(|_| "engine worker died".to_string())?
    }

    #[doc(hidden)]
    pub fn settle_request(&self, handle: q_bridge::RequestHandle) {
        let _ = self.tx.send(WorkerMsg::SettleRequest { handle });
    }

    /// M2.2.1-r4.2.1: narrow lock-free health handle for per-request readiness check
    pub fn health(&self) -> EngineHealth {
        EngineHealth {
            shared: Arc::clone(&self.shared),
        }
    }

    /// Test-support helper to query the current number of entries in `__velquSettled`
    #[doc(hidden)]
    pub fn settlement_table_len(&self) -> usize {
        let (tx, rx) = std::sync::mpsc::channel();
        if self
            .tx
            .send(WorkerMsg::QuerySettlementTableSize { reply: tx })
            .is_ok()
        {
            rx.recv().unwrap_or(0)
        } else {
            0
        }
    }
}

impl Engine for QuickJsEngine {
    fn load(
        &mut self,
        bundle: &str,
        bytecode: Option<&[u8]>,
        plan: q_engine::EngineLoadPlan,
    ) -> Result<LoadStats, String> {
        let (reply_tx, reply_rx) = std::sync::mpsc::channel();
        self.tx
            .send(WorkerMsg::Load {
                bundle: bundle.to_string(),
                bytecode: bytecode.map(Vec::from),
                plan,
                reply: reply_tx,
            })
            .map_err(|_| "engine worker gone".to_string())?;
        reply_rx
            .recv()
            .map_err(|_| "engine worker died during load".to_string())?
    }

    fn invoke(&mut self, spec: InvocationSpec, reply: tokio::sync::oneshot::Sender<Outcome>) {
        if self
            .tx
            .send(WorkerMsg::Invoke(Box::new(worker::InvokeJob {
                spec,
                reply: Some(reply),
            })))
            .is_err()
        {
            // worker gone: nothing sensible to reply; record and move on
            *self.last_error.lock().unwrap() = Some("engine worker gone during invoke".into());
        }
    }

    fn cancel(&mut self, invocation_id: u64) {
        let _ = self.tx.send(WorkerMsg::Cancel { id: invocation_id });
    }

    fn last_error(&self) -> Option<String> {
        self.last_error.lock().unwrap().clone()
    }

    fn stats(&self) -> EngineStats {
        self.shared.stats()
    }

    fn shutdown(&mut self) {
        let _ = self.tx.send(WorkerMsg::Shutdown);
        if let Some(h) = self.handle.take() {
            let _ = h.join();
        }
    }
}

impl Drop for QuickJsEngine {
    fn drop(&mut self) {
        self.shutdown();
    }
}

/// M3-001-C: type-level enforcement of the ADR-0036 sharing rules. The
/// negative half (JS values are not Send/Sync) is pinned by the crate-level
/// `compile_fail` doc test; this module pins the positive half — the only
/// types allowed to cross worker boundaries are plain data.
#[cfg(test)]
mod state_ownership_tests {
    use super::*;

    #[test]
    fn worker_messages_are_plain_data_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        // Every worker-boundary message is plain data: bytes, numbers,
        // senders. No JS value can hide inside a type that passes this.
        assert_send_sync::<WorkerMsg>();
    }

    #[test]
    fn engine_boundary_types_are_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<q_engine::InvocationSpec>();
        assert_send_sync::<q_engine::Outcome>();
        assert_send_sync::<EngineHealth>();
        // The concrete engine is the Send front door: it talks to the
        // owner thread through channels, never through runtime pointers.
        // (WorkerMsg must be Send for it to be Send at all.)
        assert_send_sync::<QuickJsEngine>();
    }
}
