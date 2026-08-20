//! q-engine-quickjs — quickjs-ng worker behind the q-engine boundary.
//!
//! One OS thread owns exactly one Runtime+Context (ADR-0008). All JS execution
//! happens there; the host side communicates via a channel. Native operations
//! (timer capability) run on tokio and complete back onto the worker loop as
//! messages, carrying no JS values across threads. Late completions for
//! settled/cancelled invocations are dropped (RUN-006, SEC-003).

mod convert;
mod prelude;
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
        }
    }

    /// Snapshot of worker-local request bridge counters. Only atomics cross
    /// the engine handle; request metadata and slab state remain worker-owned.
    pub fn bridge_snapshot(&self) -> CountersSnapshot {
        self.bridge_counters.snapshot()
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
