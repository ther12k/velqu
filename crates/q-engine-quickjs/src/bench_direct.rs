//! Diagnostic-only in-worker invocation harness (feature `bench-direct`).
//!
//! Constructs the SAME `WorkerInner` the spawned worker thread runs — same
//! runtime/context creation, same prelude, same native installation — but on
//! the CALLING thread, and executes invocations directly through
//! `begin_invocation`: no mpsc channel, no thread wakeup, no scheduler
//! participation. Tier B of the C3 decomposition
//! (`q-c3-decompose`): B − A isolates in-worker Velqu glue, C − B isolates
//! the host↔worker handoff.
//!
//! Nothing here is reachable from production builds: the module exists only
//! under the explicit `bench-direct` feature, which no default or production
//! crate enables.

use std::sync::{Arc, Mutex};

use q_engine::{EngineLoadPlan, InvocationSpec, Outcome};

use crate::worker::{InvokeJob, WorkerInner, WorkerShared};
use crate::{BridgeCounters, IdentityMapper, QuickJsConfig, SourceMapper};

pub struct DirectWorker {
    inner: WorkerInner,
}

impl DirectWorker {
    /// Build the worker engine on the calling thread, mirroring the exact
    /// construction of the spawned worker (config, counters, mapper, shared
    /// state). A tokio handle is still required because the natives install
    /// bridges to it; it is never driven by direct invocations of
    /// synchronous handlers.
    pub fn new(
        config: QuickJsConfig,
        tokio_handle: tokio::runtime::Handle,
    ) -> Result<Self, String> {
        let shared = Arc::new(WorkerShared::new());
        let last_error = Arc::new(Mutex::new(None));
        let bridge_counters = Arc::new(BridgeCounters::default());
        let mapper: Arc<dyn SourceMapper> = Arc::new(IdentityMapper);
        let inner = WorkerInner::new(
            config,
            bridge_counters,
            tokio_handle,
            mapper,
            shared,
            last_error,
        )?;
        Ok(Self { inner })
    }

    pub fn load(&mut self, bundle: &str, plan: &EngineLoadPlan) -> Result<(), String> {
        self.inner.load(bundle, None, plan).map(|_| ())
    }

    /// Shared engine counters (diagnostic): watcher registrations,
    /// drain/scans, immediate-vs-promise results, handler calls.
    pub fn stats(&self) -> q_engine::EngineStats {
        self.inner.shared_stats()
    }

    /// Execute one invocation synchronously on this thread. The reply
    /// oneshot is created and consumed here (same completion signalling as
    /// production); only the cross-thread scheduling is absent.
    pub fn invoke_direct(&mut self, spec: InvocationSpec) -> Outcome {
        let (tx, mut rx) = tokio::sync::oneshot::channel();
        let job = InvokeJob {
            spec,
            reply: Some(tx),
        };
        let _disposition = self.inner.begin_invocation(job);
        // Mirror the worker loop's synchronous postlude: settle background
        // work only when something is actually pending (a sync handler on
        // the canonical route leaves `pending` empty).
        if self.inner.has_pending() {
            self.inner.settle_background();
        }
        match rx.try_recv() {
            Ok(outcome) => outcome,
            Err(_) => Outcome::EngineFailure {
                detail: "bench-direct: synchronous invocation did not reply".into(),
                source: None,
            },
        }
    }
}
