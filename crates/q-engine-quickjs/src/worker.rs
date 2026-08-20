//! The single QuickJS worker thread: message loop, invocation lifecycle,
//! native timer capability, cancellation, and outcome mapping.
//!
//! rquickjs scoping rule: JS values live only inside `ctx.with(...)` closures,
//! so every code path below converts to `'static` data (Outcome, LoadStats)
//! before leaving the closure.

use q_bridge::{BridgeCounters, RequestStore};
use std::collections::{BTreeMap, HashMap};
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicU8, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use rquickjs::{
    Context, Function, Module, Object, Persistent, Promise, Runtime, TypedArray, Value,
};
use serde_json::Value as Json;

use crate::convert::{any_js_to_json, engine_stringify, js_to_bytes};
use crate::prelude::PRELUDE;
use crate::{QuickJsConfig, SourceMapper};

use q_engine::{
    BodyOut, EngineStats, FieldErrorOut, InvocationSpec, LoadStats, Outcome, ProblemOut,
    ResponseStrategy, SourceLocation,
};

pub(crate) struct InvokeJob {
    pub spec: InvocationSpec,
    pub reply: Option<tokio::sync::oneshot::Sender<Outcome>>,
}

pub(crate) enum WorkerMsg {
    Load {
        bundle: String,
        /// pre-compiled QuickJS module bytecode (endianness-checked; sha256 already verified)
        bytecode: Option<Vec<u8>>,
        plan: q_engine::EngineLoadPlan,
        reply: std::sync::mpsc::Sender<Result<LoadStats, String>>,
    },
    Invoke(Box<InvokeJob>),
    InsertRequest {
        meta: q_engine::RequestMeta,
        reply: std::sync::mpsc::Sender<Result<q_bridge::RequestHandle, String>>,
    },
    SettleRequest {
        handle: q_bridge::RequestHandle,
    },
    Cancel {
        id: u64,
    },
    TimerFired {
        op_id: u64,
        result: Result<u64, String>,
    },
    QuerySettlementTableSize {
        reply: std::sync::mpsc::Sender<usize>,
    },
    Shutdown,
}

/// M2.2.1-r4: Atomic task state machine ensuring exactly one terminal transition
/// (Running -> Completed OR Running -> AbortRequested).
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NativeTaskState {
    Running = 0,
    Completed = 1,
    AbortRequested = 2,
}

/// Native-side op bookkeeping: which invocation owns this pending operation,
/// plus the physical Tokio task handle so cancellation is REAL (M2.2.1-r3/r4).
/// The promise callbacks stay in the JS-side op table (see prelude).
pub(crate) struct PendingOp {
    pub invocation_id: u64,
    pub deadline: Instant,
    pub abort_handle: tokio::task::AbortHandle,
    pub state: Arc<AtomicU8>,
}

/// Bounded pending-op registry. Worker-thread only (holds Persistent JS
/// functions which are not Send); referenced by the timer native closure.
pub(crate) struct OpRegistry {
    pub ops: Mutex<HashMap<u64, PendingOp>>,
    pub op_cap: usize,
    pub op_clock: AtomicU64,
    /// Fallback watchdog budget for ops started outside any scoped execution
    /// (defense in depth; scoped paths always carry a real deadline).
    pub job_watchdog: Duration,
}

impl OpRegistry {
    fn new(op_cap: usize, job_watchdog: Duration) -> Self {
        OpRegistry {
            ops: Mutex::new(HashMap::new()),
            op_cap,
            op_clock: AtomicU64::new(1),
            job_watchdog,
        }
    }
}

/// M2.2.1-r3 execution phase: native capabilities may only start inside live
/// invocations. Cleanup/Shutdown phases reject new ops so cancellation and
/// settlement reactions cannot spawn second-generation (potentially ownerless)
/// native operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ExecutionPhase {
    Idle,
    Invocation,
    Cleanup,
    Shutdown,
}

thread_local! {
    static CURRENT_PHASE: std::cell::Cell<ExecutionPhase> =
        const { std::cell::Cell::new(ExecutionPhase::Idle) };
}

pub(crate) struct PhaseScope {
    previous: ExecutionPhase,
}

impl PhaseScope {
    pub(crate) fn enter(phase: ExecutionPhase) -> Self {
        let previous = CURRENT_PHASE.with(|p| p.replace(phase));
        Self { previous }
    }
}

impl Drop for PhaseScope {
    fn drop(&mut self) {
        CURRENT_PHASE.with(|p| p.set(self.previous));
    }
}

/// Defense in depth: cleanup drains also stop after this many jobs even when
/// every job completes before the absolute deadline.
const MAX_CLEANUP_JOBS: usize = 10_000;

/// After an engine interrupt kills a job, terminal settlement jobs (watch
/// reactions of the killed chain) get this bounded grace to drain before the
/// runtime declares the queue unquiesceable (M2.2.1-r4.1).
const SETTLEMENT_GRACE: Duration = Duration::from_millis(100);

#[inline]
fn cleanup_budget(invocation_id: u64) -> JobBudget {
    JobBudget {
        invocation_id,
        deadline: Instant::now() + SETTLEMENT_GRACE,
    }
}

/// M2.2.1-r4.2.2 RAII guard keeping the interrupt deadline armed across ALL
/// synchronous JS work (handler, response conversion, error extraction).
/// `value_to_outcome` may execute user JavaScript via toJSON()/getters/proxy
/// traps, so the invocation deadline must stay armed until the Step is built.
/// Restores the previous armed state on drop — no manual Some/None juggling.
pub(crate) struct InterruptDeadlineScope {
    cell: Arc<Mutex<Option<Instant>>>,
    previous: Option<Instant>,
}

impl InterruptDeadlineScope {
    pub(crate) fn enter(cell: Arc<Mutex<Option<Instant>>>, deadline: Instant) -> Self {
        let previous = cell.lock().unwrap().replace(deadline);
        Self { cell, previous }
    }
}

impl Drop for InterruptDeadlineScope {
    fn drop(&mut self) {
        *self.cell.lock().unwrap() = self.previous;
    }
}

/// M2.2.1-r4.1 terminal drain contract: every drain finishes in exactly one
/// of these states — the queue is empty, or the runtime is fully quarantined
/// (pending work failed, ops aborted, future dynamic requests rejected).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DrainOutcome {
    /// queue reached empty inside the budget (a final job finishing after
    /// the deadline or at the exact job cap still counts as quiesced)
    Quiesced,
    /// queue could not be emptied; the unified terminal path ran
    RuntimeQuarantined,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct DrainReport {
    pub outcome: DrainOutcome,
    pub interrupted: bool,
}

/// Stats shared between the handle and the worker (Send-safe atomics only).
pub(crate) struct WorkerShared {
    pub invocations: AtomicU64,
    pub policy_calls: AtomicU64,
    pub handler_calls: AtomicU64,
    pub immediate_results: AtomicU64,
    pub promise_results: AtomicU64,
    pub promise_watches: AtomicU64,
    pub job_queue_drains: AtomicU64,
    pub settlement_scans: AtomicU64,
    pub timer_ops_started: AtomicU64,
    pub timer_ops_completed: AtomicU64,
    pub pending_ops: AtomicU64,
    pub native_tasks_started: AtomicU64,
    pub native_tasks_alive: AtomicU64,
    pub native_tasks_completed: AtomicU64,
    pub native_tasks_aborted: AtomicU64,
    pub boundary_violations: AtomicU64,
    /// M2.2.1-r3/r4: set when a cleanup or invocation drain could not quiesce
    /// the job queue (pathological self-rescheduling microtask chain).
    /// Once poisoned, dynamic JS invocations fail closed immediately and
    /// the worker is quarantined.
    pub queue_poisoned: AtomicBool,
    pub poison_events: AtomicU64,
    pub late_completions_dropped: AtomicU64,
    pub cancelled: AtomicU64,
    pub timeouts: AtomicU64,
    pub engine_failures: AtomicU64,
    pub contract_violations: AtomicU64,
    pub numeric_dispatches: AtomicU64,
    pub legacy_map_dispatches: AtomicU64,
    pub heap_used: AtomicU64,
    /// set by the interrupt handler when it actually fired; distinguishes a
    /// deadline kill from a genuine error that merely happened near the deadline
    pub interrupted: AtomicBool,
}

impl WorkerShared {
    pub(crate) fn new() -> Self {
        WorkerShared {
            invocations: AtomicU64::new(0),
            policy_calls: AtomicU64::new(0),
            handler_calls: AtomicU64::new(0),
            immediate_results: AtomicU64::new(0),
            promise_results: AtomicU64::new(0),
            promise_watches: AtomicU64::new(0),
            job_queue_drains: AtomicU64::new(0),
            settlement_scans: AtomicU64::new(0),
            timer_ops_started: AtomicU64::new(0),
            timer_ops_completed: AtomicU64::new(0),
            pending_ops: AtomicU64::new(0),
            native_tasks_started: AtomicU64::new(0),
            native_tasks_alive: AtomicU64::new(0),
            native_tasks_completed: AtomicU64::new(0),
            native_tasks_aborted: AtomicU64::new(0),
            boundary_violations: AtomicU64::new(0),
            queue_poisoned: AtomicBool::new(false),
            poison_events: AtomicU64::new(0),
            late_completions_dropped: AtomicU64::new(0),
            cancelled: AtomicU64::new(0),
            timeouts: AtomicU64::new(0),
            engine_failures: AtomicU64::new(0),
            contract_violations: AtomicU64::new(0),
            numeric_dispatches: AtomicU64::new(0),
            legacy_map_dispatches: AtomicU64::new(0),
            heap_used: AtomicU64::new(0),
            interrupted: AtomicBool::new(false),
        }
    }

    pub(crate) fn stats(&self) -> EngineStats {
        EngineStats {
            invocations: self.invocations.load(Ordering::Relaxed),
            policy_calls: self.policy_calls.load(Ordering::Relaxed),
            handler_calls: self.handler_calls.load(Ordering::Relaxed),
            immediate_results: self.immediate_results.load(Ordering::Relaxed),
            promise_results: self.promise_results.load(Ordering::Relaxed),
            promise_watches: self.promise_watches.load(Ordering::Relaxed),
            job_queue_drains: self.job_queue_drains.load(Ordering::Relaxed),
            settlement_scans: self.settlement_scans.load(Ordering::Relaxed),
            timer_ops_started: self.timer_ops_started.load(Ordering::Relaxed),
            timer_ops_completed: self.timer_ops_completed.load(Ordering::Relaxed),
            pending_ops: self.pending_ops.load(Ordering::Relaxed),
            native_tasks_started: self.native_tasks_started.load(Ordering::Relaxed),
            native_tasks_alive: self.native_tasks_alive.load(Ordering::Relaxed),
            native_tasks_completed: self.native_tasks_completed.load(Ordering::Relaxed),
            native_tasks_aborted: self.native_tasks_aborted.load(Ordering::Relaxed),
            scheduler_boundary_violations: self.boundary_violations.load(Ordering::Relaxed),
            queue_poisoned: self.queue_poisoned.load(Ordering::Relaxed),
            poison_events: self.poison_events.load(Ordering::Relaxed),
            late_completions_dropped: self.late_completions_dropped.load(Ordering::Relaxed),
            cancelled_invocations: self.cancelled.load(Ordering::Relaxed),
            timeouts: self.timeouts.load(Ordering::Relaxed),
            engine_failures: self.engine_failures.load(Ordering::Relaxed),
            contract_violations: self.contract_violations.load(Ordering::Relaxed),
            numeric_dispatches: self.numeric_dispatches.load(Ordering::Relaxed),
            legacy_map_dispatches: self.legacy_map_dispatches.load(Ordering::Relaxed),
            heap_used: self.heap_used.load(Ordering::Relaxed) as usize,
        }
    }
}

pub(crate) struct CachedPrelude {
    run_fn: Persistent<Function<'static>>,
    make_ctx_fn: Persistent<Function<'static>>,
    make_req_fn: Persistent<Function<'static>>,
    watch_fn: Persistent<Function<'static>>,
    op_resolve_fn: Persistent<Function<'static>>,
    op_reject_fn: Persistent<Function<'static>>,
    stringify_fn: Persistent<Function<'static>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct JobBudget {
    pub invocation_id: u64,
    pub deadline: Instant,
}

/// RAII guard restoring CURRENT_INVOCATION / CURRENT_DEADLINE on every exit
/// path (M2.2.1-r2). At worker message boundaries both thread-locals hold
/// their idle values (0 / None).
pub(crate) struct InvocationScope {
    previous_id: u64,
    previous_deadline: Option<Instant>,
}

impl InvocationScope {
    pub(crate) fn enter(id: u64, deadline: Option<Instant>) -> Self {
        let previous_id = CURRENT_INVOCATION.with(|c| c.replace(id));
        let previous_deadline = CURRENT_DEADLINE.with(|c| c.replace(deadline));
        Self {
            previous_id,
            previous_deadline,
        }
    }
}

impl Drop for InvocationScope {
    fn drop(&mut self) {
        CURRENT_INVOCATION.with(|c| c.set(self.previous_id));
        CURRENT_DEADLINE.with(|c| c.set(self.previous_deadline));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum InvocationDisposition {
    Resolved,
    Pending(JobBudget),
}

/// Field order matters: Rust drops fields in declaration order, so the
/// Persistent handler cache, the CachedPrelude, and the Context MUST drop before the Runtime
/// (QuickJS asserts on live objects at JS_FreeRuntime otherwise).
pub(crate) struct WorkerInner {
    handler_cache: BTreeMap<String, Persistent<Function<'static>>>,
    /// M2.3: dense, indexed function vector for $O(1)$ direct vector dispatch
    function_vector: Vec<Persistent<Function<'static>>>,
    prelude: Option<CachedPrelude>,
    ctx: Context,
    rt: Runtime,
    store: Rc<RequestStore>,
    shared: Arc<WorkerShared>,
    last_error: Arc<Mutex<Option<String>>>,
    sync_deadline: Arc<Mutex<Option<Instant>>>,
    ops: Arc<OpRegistry>,
    /// in-flight invocations awaiting promise settlement
    pending: BTreeMap<u64, PendingInvocation>,
    /// watchdog for drain-time jobs when nothing is pending
    job_deadline: Duration,
    /// per-drain job cap (M2.2.1-r4.1, configurable for tests; default
    /// `QuickJsConfig::max_invocation_jobs`)
    max_invocation_jobs: usize,
}

thread_local! {
    static CURRENT_INVOCATION: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
    static CURRENT_DEADLINE: std::cell::Cell<Option<Instant>> = const { std::cell::Cell::new(None) };
    static WORKER_TX: std::cell::RefCell<Option<std::sync::mpsc::Sender<WorkerMsg>>> =
        const { std::cell::RefCell::new(None) };
    static MAPPER: std::cell::RefCell<Option<Arc<dyn SourceMapper>>> = const { std::cell::RefCell::new(None) };
}
impl WorkerInner {
    pub(crate) fn new(
        config: QuickJsConfig,
        bridge_counters: Arc<BridgeCounters>,
        tokio_handle: tokio::runtime::Handle,
        mapper: Arc<dyn SourceMapper>,
        shared: Arc<WorkerShared>,
        last_error: Arc<Mutex<Option<String>>>,
    ) -> Result<Self, String> {
        let rt = Runtime::new().map_err(|e| format!("quickjs runtime: {e}"))?;
        rt.set_memory_limit(config.heap_limit_bytes);
        rt.set_max_stack_size(config.stack_limit_bytes);
        let sync_deadline = Arc::new(Mutex::new(None::<Instant>));
        {
            let deadline = Arc::clone(&sync_deadline);
            let fired = Arc::clone(&shared);
            rt.set_interrupt_handler(Some(Box::new(move || {
                let hit = matches!(*deadline.lock().unwrap(), Some(d) if d <= Instant::now());
                if hit {
                    fired.interrupted.store(true, Ordering::SeqCst);
                }
                hit
            })));
        }
        let ctx = Context::full(&rt).map_err(|e| format!("quickjs context: {e}"))?;
        let ops = Arc::new(OpRegistry::new(
            config.pending_op_cap,
            Duration::from_millis(config.job_deadline_ms),
        ));
        let store = Rc::new(RequestStore::with_capacity_and_counters(
            config.request_slot_capacity,
            bridge_counters,
        ));
        MAPPER.with(|m| *m.borrow_mut() = Some(Arc::clone(&mapper)));
        let prelude = ctx.with(|ctx| -> Result<CachedPrelude, String> {
            install_natives(
                &ctx,
                Rc::clone(&store),
                Arc::clone(&shared),
                Arc::clone(&ops),
                tokio_handle,
            )
            .map_err(|e| format!("natives failed: {e:?}"))?;
            ctx.eval::<(), _>(PRELUDE)
                .map_err(|e| format!("prelude failed: {e:?}"))?;
            let run_fn = Persistent::save(
                &ctx,
                ctx.globals()
                    .get::<_, Function>("__velquRun")
                    .map_err(|e| e.to_string())?,
            );
            let make_ctx_fn = Persistent::save(
                &ctx,
                ctx.globals()
                    .get::<_, Function>("__velquMakeCtx")
                    .map_err(|e| e.to_string())?,
            );
            let make_req_fn = Persistent::save(
                &ctx,
                ctx.globals()
                    .get::<_, Function>("__velquMakeReq")
                    .map_err(|e| e.to_string())?,
            );
            let watch_fn = Persistent::save(
                &ctx,
                ctx.globals()
                    .get::<_, Function>("__velquWatch")
                    .map_err(|e| e.to_string())?,
            );
            let op_resolve_fn = Persistent::save(
                &ctx,
                ctx.globals()
                    .get::<_, Function>("__velquOpResolve")
                    .map_err(|e| e.to_string())?,
            );
            let op_reject_fn = Persistent::save(
                &ctx,
                ctx.globals()
                    .get::<_, Function>("__velquOpReject")
                    .map_err(|e| e.to_string())?,
            );
            let json_obj: Object = ctx.globals().get("JSON").map_err(|e| e.to_string())?;
            let stringify_func: Function = json_obj.get("stringify").map_err(|e| e.to_string())?;
            let stringify_fn = Persistent::save(&ctx, stringify_func);
            Ok(CachedPrelude {
                run_fn,
                make_ctx_fn,
                make_req_fn,
                watch_fn,
                op_resolve_fn,
                op_reject_fn,
                stringify_fn,
            })
        })?;
        Ok(WorkerInner {
            handler_cache: BTreeMap::new(),
            function_vector: Vec::new(),
            prelude: Some(prelude),
            ctx,
            rt,
            store,
            shared,
            last_error,
            sync_deadline,
            ops,
            pending: BTreeMap::new(),
            job_deadline: Duration::from_millis(config.job_deadline_ms),
            max_invocation_jobs: config.max_invocation_jobs,
        })
    }

    pub(crate) fn run(
        mut self,
        rx: std::sync::mpsc::Receiver<WorkerMsg>,
        tx: std::sync::mpsc::Sender<WorkerMsg>,
    ) {
        WORKER_TX.with(|c| *c.borrow_mut() = Some(tx));
        let mut msg_count: u64 = 0;
        loop {
            let next_deadline = self.pending.values().map(|p| p.spec.deadline).min();
            let msg = match next_deadline {
                Some(dl) => {
                    let now = Instant::now();
                    if dl <= now {
                        self.expire_timeouts();
                        continue;
                    }
                    match rx.recv_timeout(dl - now) {
                        Ok(m) => m,
                        Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                            self.expire_timeouts();
                            continue;
                        }
                        Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => break,
                    }
                }
                None => match rx.recv() {
                    Ok(m) => m,
                    Err(_) => break,
                },
            };
            match msg {
                WorkerMsg::Load {
                    bundle,
                    bytecode,
                    plan,
                    reply,
                } => {
                    let _ = reply.send(self.load(&bundle, bytecode.as_deref(), &plan));
                }
                WorkerMsg::Invoke(job) => {
                    let disposition = self.begin_invocation(*job);
                    match disposition {
                        InvocationDisposition::Resolved => {
                            // Synchronous path: outcome was settled and sent.
                            // Settle promise completions for still-pending work only.
                            if !self.pending.is_empty() {
                                self.settle_background();
                            }
                        }
                        InvocationDisposition::Pending(budget) => {
                            // Jobs queued during the handler call (watch reactions,
                            // then-chains) drain under the owning invocation's budget.
                            let drain_res = self.drain_jobs_for(budget, ExecutionPhase::Invocation);
                            if matches!(drain_res.outcome, DrainOutcome::RuntimeQuarantined) {
                                // quarantine already failed and settled the
                                // invocation (EngineFailure) — nothing to do
                            } else if drain_res.interrupted {
                                // a deadline interrupt killed the watched chain:
                                // reply Timeout, settle, abort floating ops, clear settled entry
                                if let Some(p) = self.pending.remove(&budget.invocation_id) {
                                    self.finish_timeout(budget.invocation_id, p);
                                }
                            }
                            // otherwise: the chain quiesced; finish_resolved
                            // settles whatever the watch table recorded
                            if !self.pending.is_empty() {
                                self.finish_resolved();
                            }
                        }
                    }
                }
                WorkerMsg::InsertRequest { meta, reply } => {
                    let _ = reply.send(self.store.try_insert(meta).map_err(|e| e.to_string()));
                }
                WorkerMsg::SettleRequest { handle } => {
                    self.settle_request(handle);
                }
                WorkerMsg::Cancel { id } => {
                    self.cancel_invocation(id);
                    self.settle_background();
                }
                WorkerMsg::TimerFired { op_id, result } => {
                    if let Some((budget, phase)) = self.complete_timer(op_id, result) {
                        // Continuation runs under the invocation that owns the op
                        // (M2.2.1-r2 ownership restoration), not whichever
                        // invocation happened to start last. A resolved timer's
                        // continuation is live-invocation work (ops permitted);
                        // a rejected one is cleanup work.
                        let drain_res = self.drain_jobs_for(budget, phase);
                        if matches!(drain_res.outcome, DrainOutcome::RuntimeQuarantined) {
                            // quarantine already failed and settled the owner
                        } else if drain_res.interrupted {
                            if let Some(p) = self.pending.remove(&budget.invocation_id) {
                                self.finish_timeout(budget.invocation_id, p);
                            }
                        }
                    }
                    if !self.pending.is_empty() {
                        self.finish_resolved();
                    }
                    // chained continuations queued by settlements unwind scoped
                    if self.rt.is_job_pending() && !self.pending.is_empty() {
                        self.settle_background();
                    }
                }
                WorkerMsg::QuerySettlementTableSize { reply } => {
                    let _ = reply.send(self.settlement_table_len());
                }
                WorkerMsg::Shutdown => break,
            }
            self.check_message_boundary();
            msg_count = msg_count.wrapping_add(1);
            if msg_count & 0x7F == 0 {
                self.shared.heap_used.store(
                    self.rt.memory_usage().memory_used_size as u64,
                    Ordering::Relaxed,
                );
            }
        }
        // deterministic cleanup: reject outstanding work so continuations unwind
        let ids: Vec<u64> = self.pending.keys().copied().collect();
        for id in ids {
            self.cancel_invocation(id);
        }
        {
            let _phase = PhaseScope::enter(ExecutionPhase::Shutdown);
            // orphaned rejection continuations unwind under the bounded watchdog
            self.drain_jobs_watchdog();
            // M2.2.1-r3: physically abort every remaining native task —
            // clear() alone would leave sleeping Tokio tasks alive with
            // retained channel senders.
            let ops: Vec<PendingOp> = self
                .ops
                .ops
                .lock()
                .unwrap()
                .drain()
                .map(|(_, v)| v)
                .collect();
            for op in ops {
                abort_op_task(&op.state, &op.abort_handle);
            }
        }
        // M24-003-C: shutdown is a terminal path — the worker-owned sweep
        // guarantees zero live slots even for slots no pending entry tracked.
        self.store.settle_all();
        self.shared.pending_ops.store(0, Ordering::Relaxed);
        self.shared.heap_used.store(
            self.rt.memory_usage().memory_used_size as u64,
            Ordering::Relaxed,
        );
    }

    #[allow(clippy::type_complexity)]
    fn load(
        &mut self,
        bundle: &str,
        bytecode: Option<&[u8]>,
        plan: &q_engine::EngineLoadPlan,
    ) -> Result<LoadStats, String> {
        let t0 = Instant::now();
        let (register_calls, cache, vec_fns): (
            usize,
            BTreeMap<String, Persistent<Function<'static>>>,
            Vec<Persistent<Function<'static>>>,
        ) = self.ctx.with(
            |ctx| -> Result<
                (
                    usize,
                    BTreeMap<String, Persistent<Function<'static>>>,
                    Vec<Persistent<Function<'static>>>,
                ),
                String,
            > {
                if let Some(bc) = bytecode {
                    // ADR-0017: load pre-compiled bytecode (skips parsing + compilation).
                    // Safety: caller (WorkerBuilder) verified sha256 and exact engine/ABI
                    // version match in QPack::verify() before handing us these bytes.
                    let module = unsafe {
                        Module::load(ctx.clone(), bc).map_err(|e| {
                            format!("bytecode load failed: {}", describe_error(&ctx, &e))
                        })?
                    };
                    let (_evaled, _promise) = module
                        .eval()
                        .map_err(|e| format!("module eval failed: {}", describe_error(&ctx, &e)))?;
                } else {
                    ctx.eval::<Value, _>(bundle).map_err(|e| {
                        format!("bundle evaluation failed: {}", describe_error(&ctx, &e))
                    })?;
                }

                match plan {
                    q_engine::EngineLoadPlan::Numeric { functions } => {
                        let count = functions.len();
                        let manifest_array = ctx
                            .globals()
                            .get::<_, rquickjs::Array>("__velquFunctionManifest")
                            .map_err(|_| {
                                "semantic function manifest (globalThis.__velquFunctionManifest) is missing from numeric bundle".to_string()
                            })?;
                        let len = manifest_array.len();
                        if len != count {
                            return Err(format!(
                                "function manifest length {len} != expected manifest count {count}"
                            ));
                        }
                        let mut vec_fns = Vec::with_capacity(len);
                        for (idx, expected_decl) in functions.iter().enumerate().take(len) {
                            let entry: rquickjs::Array = manifest_array.get(idx).map_err(|e| {
                                format!("function manifest entry {idx} is invalid: {e}")
                            })?;
                            let key: String = entry.get(0).map_err(|e| {
                                format!("function manifest entry {idx} missing key: {e}")
                            })?;
                            let kind_num: u32 = entry.get(1).map_err(|e| {
                                format!("function manifest entry {idx} missing kind: {e}")
                            })?;
                            let f: Function = entry.get(2).map_err(|e| {
                                format!(
                                    "function manifest entry {idx} ({key}) is not a callable function: {e}"
                                )
                            })?;

                            if key != expected_decl.key {
                                return Err(format!(
                                    "function manifest index {idx} key mismatch: bundle has '{key}', pack expected '{}'",
                                    expected_decl.key
                                ));
                            }
                            let expected_kind_num = match expected_decl.kind {
                                q_engine::FunctionKind::RouteHandler => 0,
                                q_engine::FunctionKind::PolicyHandler => 1,
                            };
                            if kind_num != expected_kind_num {
                                return Err(format!(
                                    "function manifest index {idx} ({key}) kind mismatch: bundle has {kind_num}, pack expected {expected_kind_num}",
                                ));
                            }
                            vec_fns.push(Persistent::save(&ctx, f));
                        }
                        Ok((0, BTreeMap::new(), vec_fns))
                    }
                    q_engine::EngineLoadPlan::Legacy { expected_handlers } => {
                        let handlers: Object = ctx
                            .globals()
                            .get::<_, Object>("__velquHandlers")
                            .map_err(|_| "prelude state missing".to_string())?;
                        let mut count = 0usize;
                        let mut cache = BTreeMap::new();
                        for key in handlers.keys::<String>() {
                            let key = key.map_err(|e| e.to_string())?;
                            let f: Function =
                                handlers.get(key.as_str()).map_err(|e| e.to_string())?;
                            count += 1;
                            cache.insert(key, Persistent::save(&ctx, f));
                        }
                        let missing: Vec<&String> = expected_handlers
                            .keys()
                            .filter(|k| !cache.contains_key(*k))
                            .collect();
                        let extra: Vec<&String> = cache
                            .keys()
                            .filter(|k| !expected_handlers.contains_key(*k))
                            .collect();
                        if !missing.is_empty() || !extra.is_empty() {
                            return Err(format!(
                                "handler table mismatch (missing={missing:?} extra={extra:?})"
                            ));
                        }
                        if count != expected_handlers.len() {
                            return Err(format!(
                                "handler registration count {count} != expected {}",
                                expected_handlers.len()
                            ));
                        }
                        Ok((count, cache, Vec::new()))
                    }
                }
            },
        )?;
        let eval_ms = t0.elapsed().as_secs_f64() * 1000.0;
        self.handler_cache = cache;
        self.function_vector = vec_fns;
        let registered_count = if !self.function_vector.is_empty() {
            self.function_vector.len()
        } else {
            register_calls
        };
        Ok(LoadStats {
            handlers_registered: registered_count,
            eval_ms,
            register_calls,
        })
    }

    fn begin_invocation(&mut self, job: InvokeJob) -> InvocationDisposition {
        let InvokeJob { mut spec, reply } = job;
        self.shared.interrupted.store(false, Ordering::SeqCst);

        // M24-003-B: the typed capability is minted here — any pair the spec
        // carried in is overwritten, so a caller can never present a
        // host-forged handle. Requestless specs keep the sentinel pair, whose
        // settle/access is a bounds-checked no-op.
        let mut handle = self.store.local_handle(spec.slot, spec.generation);
        if spec.slot != q_engine::NO_REQUEST_SLOT {
            if let Some(meta) = spec.request.take() {
                match self.store.try_insert(meta) {
                    Ok(minted) => {
                        handle = minted;
                        (spec.slot, spec.generation) = minted.js_pair();
                    }
                    Err(q_bridge::BridgeError::Capacity) => {
                        if let Some(r) = reply {
                            let _ = r.send(Outcome::RequestCapacity);
                        }
                        return InvocationDisposition::Resolved;
                    }
                    Err(e) => {
                        if let Some(r) = reply {
                            let _ = r.send(Outcome::EngineFailure {
                                detail: e.to_string(),
                                source: None,
                            });
                        }
                        return InvocationDisposition::Resolved;
                    }
                }
            }
        }

        self.shared.invocations.fetch_add(1, Ordering::Relaxed);

        // Fail closed immediately if runtime is quarantined / poisoned (M2.2.1-r4)
        if self.shared.queue_poisoned.load(Ordering::SeqCst) {
            self.shared.engine_failures.fetch_add(1, Ordering::Relaxed);
            self.settle_request(handle);
            if let Some(r) = reply {
                let _ = r.send(Outcome::EngineFailure {
                    detail: "runtime quarantined: worker quarantined".into(),
                    source: None,
                });
            }
            return InvocationDisposition::Resolved;
        }

        if spec.policy_key.is_some() || spec.policy_handler_id.is_some() {
            self.shared.policy_calls.fetch_add(1, Ordering::Relaxed);
        }
        self.shared.handler_calls.fetch_add(1, Ordering::Relaxed);

        // M2.3: Direct O(1) vector lookup when numeric handler_id is present,
        // falling back to handler_cache map lookup for legacy/string specs.
        let handler = if let Some(hid) = spec.handler_id {
            let h = self.function_vector.get(hid.0 as usize).cloned();
            if h.is_some() {
                self.shared
                    .numeric_dispatches
                    .fetch_add(1, Ordering::Relaxed);
            }
            h
        } else {
            let h = self.handler_cache.get(&spec.handler_key).cloned();
            if h.is_some() {
                self.shared
                    .legacy_map_dispatches
                    .fetch_add(1, Ordering::Relaxed);
            }
            h
        };
        let Some(handler) = handler else {
            // terminal failure: deterministic cleanup so the slot never leaks
            self.shared.engine_failures.fetch_add(1, Ordering::Relaxed);
            self.settle_request(handle);
            if let Some(r) = reply {
                let _ = r.send(Outcome::EngineFailure {
                    detail: format!(
                        "handler {:?} / {} not in vector/cache",
                        spec.handler_id, spec.handler_key
                    ),
                    source: None,
                });
            }
            return InvocationDisposition::Resolved;
        };

        // Fail closed: a route declaring a policy whose handler is missing
        // must NEVER run the business handler unauthenticated.
        let policy = if let Some(pid) = spec.policy_handler_id {
            let p = self.function_vector.get(pid.0 as usize).cloned();
            if p.is_none() {
                self.shared.engine_failures.fetch_add(1, Ordering::Relaxed);
                self.settle_request(handle);
                if let Some(r) = reply {
                    let _ = r.send(Outcome::EngineFailure {
                        detail: format!(
                            "required policy handler index {} not in vector (fail closed)",
                            pid.0
                        ),
                        source: None,
                    });
                }
                return InvocationDisposition::Resolved;
            }
            p
        } else if let Some(pkey) = &spec.policy_key {
            let p = self.handler_cache.get(pkey).cloned();
            if p.is_none() {
                self.shared.engine_failures.fetch_add(1, Ordering::Relaxed);
                self.settle_request(handle);
                if let Some(r) = reply {
                    let _ = r.send(Outcome::EngineFailure {
                        detail: format!(
                            "required policy handler {} not in cache (fail closed)",
                            pkey
                        ),
                        source: None,
                    });
                }
                return InvocationDisposition::Resolved;
            }
            p
        } else {
            None
        };
        enum Step {
            Immediate(Outcome),
            Watched,
            Failed(Outcome),
        }
        let budget = JobBudget {
            invocation_id: spec.id,
            deadline: spec.deadline,
        };
        let spec_id = spec.id;
        let prelude = self.prelude.as_ref().unwrap();
        let watch_fn_persistent = prelude.watch_fn.clone();
        let run_fn_persistent = prelude.run_fn.clone();
        let make_ctx_persistent = prelude.make_ctx_fn.clone();
        let make_req_persistent = prelude.make_req_fn.clone();

        let stringify_fn_persistent = prelude.stringify_fn.clone();

        let step = {
            // handler + its sync microtask checkpoint run as a live invocation
            // (native ops permitted); after the checkpoint the phase drops to
            // Cleanup for floating-op unwinding.
            let _scope = InvocationScope::enter(budget.invocation_id, Some(budget.deadline));
            let _phase = PhaseScope::enter(ExecutionPhase::Invocation);
            self.ctx.with(|ctx| {
                // M2.2.1-r4.2.2: keep the interrupt deadline armed through
                // ALL synchronous JS work — handler call, watch attachment,
                // response conversion (value_to_outcome may run user JS via
                // toJSON/getters/proxy traps), and error extraction. The RAII
                // guard restores the previous state on every exit path.
                let _deadline_guard =
                    InterruptDeadlineScope::enter(Arc::clone(&self.sync_deadline), budget.deadline);
                let out = call_runner(
                    &ctx,
                    &handler,
                    policy.as_ref(),
                    &run_fn_persistent,
                    &make_ctx_persistent,
                    &make_req_persistent,
                    &spec,
                );
                let stringify_fn = stringify_fn_persistent.restore(&ctx).ok();
                match out {
                    Err(e) => {
                        let (detail, source) = describe_exception(&ctx, &e);
                        Step::Failed(Outcome::EngineFailure { detail, source })
                    }
                    Ok(value) => {
                        if value.is_promise() {
                            let promise: Promise = value.get().unwrap();
                            let watch: Function = match watch_fn_persistent.restore(&ctx) {
                                Ok(w) => w,
                                Err(_) => {
                                    return Step::Failed(Outcome::EngineFailure {
                                        detail: "failed to restore watch function".into(),
                                        source: None,
                                    })
                                }
                            };
                            match watch.call::<_, ()>((promise, spec_id as f64)) {
                                Ok(()) => Step::Watched,
                                Err(_) => Step::Failed(Outcome::EngineFailure {
                                    detail: "failed to attach promise watch".into(),
                                    source: None,
                                }),
                            }
                        } else {
                            Step::Immediate(value_to_outcome(
                                &ctx,
                                &spec,
                                &value,
                                stringify_fn.as_ref(),
                            ))
                        }
                    }
                }
            })
        };
        let Some(reply) = reply else {
            return InvocationDisposition::Resolved;
        }; // cancelled before start
        match step {
            Step::Failed(o) => {
                // only a CONFIRMED interrupt (flag set by the handler) maps to
                // Timeout; a genuine error near the deadline stays an error
                let drain_report = if self.rt.is_job_pending() {
                    self.drain_jobs_for(
                        cleanup_budget(budget.invocation_id),
                        ExecutionPhase::Cleanup,
                    )
                } else {
                    DrainReport {
                        outcome: DrainOutcome::Quiesced,
                        interrupted: false,
                    }
                };
                let o = if self.shared.interrupted.swap(false, Ordering::SeqCst)
                    || drain_report.interrupted
                    || matches!(drain_report.outcome, DrainOutcome::RuntimeQuarantined)
                {
                    self.shared.timeouts.fetch_add(1, Ordering::Relaxed);
                    Outcome::Timeout
                } else {
                    self.shared.engine_failures.fetch_add(1, Ordering::Relaxed);
                    o
                };
                self.abort_floating_ops(spec.id);
                self.settle_request(handle);
                let _ = reply.send(o);
                InvocationDisposition::Resolved
            }
            Step::Immediate(outcome) => {
                self.shared
                    .immediate_results
                    .fetch_add(1, Ordering::Relaxed);
                if matches!(
                    outcome,
                    Outcome::EngineFailure { .. } | Outcome::ContractViolation(_)
                ) {
                    self.shared.engine_failures.fetch_add(1, Ordering::Relaxed);
                }
                // Microtask checkpoint (M2.2.1): if the sync handler scheduled
                // microtasks, execute them under THIS invocation's deadline —
                // still in the Invocation phase (ops permitted) and before
                // settling the request handle and responding.
                let drain_report = if self.rt.is_job_pending() {
                    self.drain_jobs_for(budget, ExecutionPhase::Invocation)
                } else {
                    DrainReport {
                        outcome: DrainOutcome::Quiesced,
                        interrupted: false,
                    }
                };
                let final_outcome = if self.shared.interrupted.swap(false, Ordering::SeqCst)
                    || drain_report.interrupted
                    || matches!(drain_report.outcome, DrainOutcome::RuntimeQuarantined)
                {
                    self.shared.timeouts.fetch_add(1, Ordering::Relaxed);
                    Outcome::Timeout
                } else {
                    outcome
                };
                // native ops not awaited by the returned Promise are cancelled
                // at settlement (until an explicit defer() mechanism exists)
                self.abort_floating_ops(spec.id);
                self.settle_request(handle);
                let _ = reply.send(final_outcome);
                InvocationDisposition::Resolved
            }
            Step::Watched => {
                self.shared.promise_watches.fetch_add(1, Ordering::Relaxed);
                self.pending.insert(
                    spec.id,
                    PendingInvocation {
                        spec,
                        handle,
                        reply: Some(reply),
                    },
                );
                InvocationDisposition::Pending(budget)
            }
        }
    }

    /// M2.2.1-r2 invariant probe: at every worker message boundary
    /// CURRENT_INVOCATION == 0, CURRENT_DEADLINE == None, sync_deadline is
    /// unarmed, and the execution phase is Idle. Violations are counted,
    /// never silently ignored.
    fn check_message_boundary(&mut self) {
        let cur = CURRENT_INVOCATION.with(|c| c.get());
        let cur_dl = CURRENT_DEADLINE.with(|c| c.get());
        let phase = CURRENT_PHASE.with(|p| p.get());
        let armed = self.sync_deadline.lock().unwrap().is_some();
        let jobs_pending = self.rt.is_job_pending();
        let quarantined = self.shared.queue_poisoned.load(Ordering::Acquire);

        if cur != 0
            || cur_dl.is_some()
            || armed
            || phase != ExecutionPhase::Idle
            || (jobs_pending && !quarantined)
        {
            self.shared
                .boundary_violations
                .fetch_add(1, Ordering::Relaxed);
            *self.last_error.lock().unwrap() = Some(format!(
                "scheduler boundary violation: invocation={cur} deadline_local={cur_dl:?} armed={armed} phase={phase:?} jobs_pending={jobs_pending}"
            ));
            // restore idle state so one violation does not poison every later message
            CURRENT_INVOCATION.with(|c| c.set(0));
            CURRENT_DEADLINE.with(|c| c.set(None));
            CURRENT_PHASE.with(|p| p.set(ExecutionPhase::Idle));
            *self.sync_deadline.lock().unwrap() = None;
            if jobs_pending && !quarantined {
                self.quarantine_runtime("QuickJS jobs escaped a worker message boundary");
            }
        }
    }

    /// Resolve/reject a native op's promise and return the owning budget (plus
    /// the phase its continuation should drain in) so the caller can run it
    /// under the right invocation scope and deadline (M2.2.1-r2 ownership
    /// restoration).
    ///
    /// M2.2.1-r3: an Err result is always host-initiated (timers cannot fail
    /// on their own) — the physical Tokio task is aborted so no sleeping task
    /// or stale completion survives cancellation. Resolutions keep the task
    /// alive accounting already done by the task itself.
    fn complete_timer(
        &mut self,
        op_id: u64,
        result: Result<u64, String>,
    ) -> Option<(JobBudget, ExecutionPhase)> {
        let op = self.ops.ops.lock().unwrap().remove(&op_id);
        let Some(op) = op else {
            self.shared
                .late_completions_dropped
                .fetch_add(1, Ordering::Relaxed);
            return None;
        };
        self.shared.pending_ops.fetch_sub(1, Ordering::Relaxed);
        self.shared
            .timer_ops_completed
            .fetch_add(1, Ordering::Relaxed);
        if result.is_err() {
            abort_op_task(&op.state, &op.abort_handle);
        }
        let budget = JobBudget {
            invocation_id: op.invocation_id,
            deadline: op.deadline,
        };
        let prelude = self.prelude.as_ref().unwrap();
        let resolve_fn = prelude.op_resolve_fn.clone();
        let reject_fn = prelude.op_reject_fn.clone();
        // resolution continuation runs as a live invocation (the owner may
        // legitimately await another op); a rejection continuation cannot
        // start new ops (Cleanup phase).
        let phase = if result.is_ok() {
            ExecutionPhase::Invocation
        } else {
            ExecutionPhase::Cleanup
        };
        let _scope = InvocationScope::enter(budget.invocation_id, Some(budget.deadline));
        let _phase_scope = PhaseScope::enter(phase);
        let deadline_cell = Arc::clone(&self.sync_deadline);
        let _ = self.ctx.with(|ctx| -> rquickjs::Result<()> {
            *deadline_cell.lock().unwrap() = Some(budget.deadline);
            let r = match result {
                Ok(ms) => {
                    let f: Function = resolve_fn.restore(&ctx)?;
                    f.call::<_, ()>((op_id as f64, ms as f64))
                }
                Err(reason) => {
                    let f: Function = reject_fn.restore(&ctx)?;
                    f.call::<_, ()>((op_id as f64, reason))
                }
            };
            *deadline_cell.lock().unwrap() = None;
            r
        });
        Some((budget, phase))
    }

    fn clear_settled_entry(&mut self, invocation_id: u64) {
        let _ = self.ctx.with(|ctx| -> rquickjs::Result<()> {
            if let Ok(table) = ctx.globals().get::<_, Object>("__velquSettled") {
                let _ = table.remove(invocation_id.to_string().as_str());
            }
            Ok(())
        });
    }

    /// M24-003-C: THE single per-handle settlement owner. Every terminal path
    /// — completion, failure, timeout, cancellation, quarantine of a pending
    /// entry — invalidates the capability through this one routine; the
    /// generation check inside makes a second arriver a checked no-op, and
    /// the worker-owned `settle_all` sweep (quarantine/shutdown) is idempotent
    /// against it. No other worker code may call the store's settle directly.
    fn settle_request(&mut self, handle: q_bridge::RequestHandle) {
        self.store.settle(handle);
    }

    fn cancel_invocation(&mut self, id: u64) {
        let Some(mut p) = self.pending.remove(&id) else {
            return;
        };
        self.shared.cancelled.fetch_add(1, Ordering::Relaxed);
        let cleanup_budget = JobBudget {
            invocation_id: p.spec.id,
            deadline: Instant::now() + SETTLEMENT_GRACE,
        };
        self.settle_request(p.handle);
        // rejection continuations unwind in the Cleanup phase with a fresh cleanup grace
        {
            let _phase = PhaseScope::enter(ExecutionPhase::Cleanup);
            self.reject_ops_of(id);
            if self.rt.is_job_pending() {
                self.drain_jobs_for(cleanup_budget, ExecutionPhase::Cleanup);
            }
        }
        self.clear_settled_entry(id);
        p.reply.take();
    }

    fn reject_ops_of(&mut self, invocation_id: u64) {
        let ids: Vec<u64> = self
            .ops
            .ops
            .lock()
            .unwrap()
            .iter()
            .filter(|(_, op)| op.invocation_id == invocation_id)
            .map(|(k, _)| *k)
            .collect();
        for op_id in ids {
            self.complete_timer(op_id, Err("aborted".into()));
        }
    }

    /// Cancel native operations still owned by a settled invocation and unwind
    /// their rejection continuations under the fresh cleanup budget (M2.2.1-r4.2).
    /// Runs in the Cleanup phase so reactions cannot spawn second-generation
    /// native ops (M2.2.1-r3).
    fn abort_floating_ops(&mut self, invocation_id: u64) {
        let ids: Vec<u64> = self
            .ops
            .ops
            .lock()
            .unwrap()
            .iter()
            .filter(|(_, op)| op.invocation_id == invocation_id)
            .map(|(k, _)| *k)
            .collect();
        if ids.is_empty() {
            return;
        }
        let _phase = PhaseScope::enter(ExecutionPhase::Cleanup);
        for op_id in ids {
            self.complete_timer(op_id, Err("aborted: invocation settled".into()));
        }
        if self.rt.is_job_pending() {
            self.drain_jobs_for(cleanup_budget(invocation_id), ExecutionPhase::Cleanup);
        }
    }

    /// M2.3-r1/r2: Unified terminal timeout cleanup helper.
    /// Clears any settled promise table entry, aborts floating ops under a bounded cleanup budget,
    /// clears any entry created by rejection reactions, settles the request slot, increments the timeout
    /// counter once, and sends Outcome::Timeout.
    fn finish_timeout(&mut self, id: u64, mut pending: PendingInvocation) {
        self.clear_settled_entry(id);
        self.abort_floating_ops(id);
        self.clear_settled_entry(id);
        self.settle_request(pending.handle);
        self.shared.timeouts.fetch_add(1, Ordering::Relaxed);
        if let Some(reply) = pending.reply.take() {
            let _ = reply.send(Outcome::Timeout);
        }
    }

    #[doc(hidden)]
    pub(crate) fn settlement_table_len(&self) -> usize {
        self.ctx.with(|ctx| -> usize {
            let table: Object = match ctx.globals().get("__velquSettled") {
                Ok(t) => t,
                Err(_) => return 0,
            };
            table.keys::<String>().count()
        })
    }

    fn expire_timeouts(&mut self) {
        let now = Instant::now();
        let due: Vec<u64> = self
            .pending
            .values()
            .filter(|p| p.spec.deadline <= now)
            .map(|p| p.spec.id)
            .collect();
        for id in due {
            let Some(p) = self.pending.remove(&id) else {
                continue;
            };
            let cleanup_budget = JobBudget {
                invocation_id: p.spec.id,
                deadline: Instant::now() + SETTLEMENT_GRACE,
            };
            {
                let _phase = PhaseScope::enter(ExecutionPhase::Cleanup);
                self.reject_ops_of(id);
                if self.rt.is_job_pending() {
                    self.drain_jobs_for(cleanup_budget, ExecutionPhase::Cleanup);
                }
            }
            self.finish_timeout(id, p);
        }
    }

    /// M2.2.1-r4.1/M2.3-r2: THE single terminal operation. Only this function may set
    /// `queue_poisoned = true`. Idempotent. Fails every pending invocation,
    /// settles all request slots, aborts and removes every native operation
    /// (correcting the `pending_ops` gauge), wholesale clears the settlement table,
    /// rejects all later dynamic JS invocations, and emits exactly one terminal event.
    fn quarantine_runtime(&mut self, reason: &str) {
        if self.shared.queue_poisoned.swap(true, Ordering::SeqCst) {
            return; // already quarantined — exactly one terminal event
        }
        self.shared.poison_events.fetch_add(1, Ordering::Relaxed);
        *self.last_error.lock().unwrap() = Some(format!("runtime quarantined: {reason}"));

        // Wholesale clear the settlement table
        self.ctx.with(|ctx| {
            if let Ok(table) = ctx.globals().get::<_, Object>("__velquSettled") {
                let keys: Vec<String> = table.keys::<String>().filter_map(|k| k.ok()).collect();
                for k in keys {
                    table.remove(k.as_str()).ok();
                }
            }
        });

        // 1. Fail all pending invocations with EngineFailure; settle their
        //    slots; reject their operations through the normal abort path.
        let pending_ids: Vec<u64> = self.pending.keys().copied().collect();
        for id in pending_ids {
            if let Some(mut p) = self.pending.remove(&id) {
                self.shared.engine_failures.fetch_add(1, Ordering::Relaxed);
                self.settle_request(p.handle);
                self.reject_ops_of(id);
                if let Some(reply) = p.reply.take() {
                    let _ = reply.send(Outcome::EngineFailure {
                        detail: format!("runtime quarantined: {reason}"),
                        source: None,
                    });
                }
            }
        }

        // 2. Abort ALL remaining native operations (including those owned by
        //    the currently-executing invocation, which never entered
        //    `pending`), with checked pending_ops accounting.
        let leftover_ops: Vec<PendingOp> = self
            .ops
            .ops
            .lock()
            .unwrap()
            .drain()
            .map(|(_, v)| v)
            .collect();
        let removed = leftover_ops.len() as u64;
        for op in &leftover_ops {
            abort_op_task(&op.state, &op.abort_handle);
        }
        // M2.2.1-r4.2.2: unconditionally swap(0) — the terminal gauge is zero
        // regardless of prior drift; unsigned fetch_sub could otherwise wrap.
        reset_pending_ops_after_quarantine(&self.shared, removed);

        // 3. M24-003-C: quarantine is a terminal path. The worker-owned sweep
        //    catches Active slots that no pending entry tracks (e.g. the
        //    invocation currently executing when the poison triggered); it is
        //    idempotent against the per-handle settles above.
        self.store.settle_all();
    }

    /// Drain queued QuickJS jobs under an explicit invocation-scoped budget.
    /// This function owns deadline arming and cleanup on every path — callers
    /// must not touch sync_deadline around it (M2.2.1-r2).
    ///
    /// M2.2.1-r4.2.1 contract:
    /// - Returns `DrainReport { outcome, interrupted }` where `interrupted` is
    ///   DRAIN-LOCAL (never leaks to another invocation).
    /// - Quiescence is checked BEFORE any budget enforcement.
    /// - Single-assignment settlement grace: `grace_deadline` is set at most
    ///   ONCE per drain, preventing repeated error jobs from renewing the grace.
    fn drain_jobs_for(&mut self, budget: JobBudget, phase: ExecutionPhase) -> DrainReport {
        if self.shared.queue_poisoned.load(Ordering::SeqCst) {
            return DrainReport {
                outcome: DrainOutcome::RuntimeQuarantined,
                interrupted: false,
            };
        }
        if !self.rt.is_job_pending() {
            return DrainReport {
                outcome: DrainOutcome::Quiesced,
                interrupted: false,
            };
        }
        self.shared.job_queue_drains.fetch_add(1, Ordering::Relaxed);
        let _scope = InvocationScope::enter(budget.invocation_id, Some(budget.deadline));
        let _phase_scope = PhaseScope::enter(phase);
        let mut executed: usize = 0;
        let mut grace_deadline: Option<Instant> = None;
        let mut was_interrupted = false;
        loop {
            // quiescence first: an empty queue is success regardless of how
            // close to (or past) the budget we are
            if !self.rt.is_job_pending() {
                return DrainReport {
                    outcome: DrainOutcome::Quiesced,
                    interrupted: was_interrupted,
                };
            }
            let now = Instant::now();
            let effective_deadline = grace_deadline.unwrap_or(budget.deadline);
            if now >= effective_deadline {
                self.quarantine_runtime(match grace_deadline {
                    Some(_) => "settlement grace expired during microtask drain",
                    None => "invocation deadline exceeded during microtask drain",
                });
                return DrainReport {
                    outcome: DrainOutcome::RuntimeQuarantined,
                    interrupted: was_interrupted,
                };
            }
            if executed >= self.max_invocation_jobs {
                self.quarantine_runtime("max invocation jobs exceeded during microtask drain");
                return DrainReport {
                    outcome: DrainOutcome::RuntimeQuarantined,
                    interrupted: was_interrupted,
                };
            }
            *self.sync_deadline.lock().unwrap() = Some(effective_deadline);
            let result = self.rt.execute_pending_job();
            *self.sync_deadline.lock().unwrap() = None;
            executed += 1;
            let job_interrupted = self.shared.interrupted.swap(false, Ordering::SeqCst);
            if job_interrupted {
                was_interrupted = true;
                // M2.2.1-r4.2.1: single-assignment grace — assigned at most ONCE per drain
                if grace_deadline.is_none() {
                    grace_deadline = Some(Instant::now() + SETTLEMENT_GRACE);
                }
            }
            if let Err(_job_exc) = result {
                *self.last_error.lock().unwrap() = Some("pending job exception".into());
            }
        }
    }

    /// Watchdog drain for ownerless work (shutdown cleanup, floating-op
    /// rejection unwinding). Same terminal contract as `drain_jobs_for`:
    /// queue empty or runtime quarantined — never a partial exit.
    fn drain_jobs_watchdog(&mut self) -> DrainOutcome {
        if self.shared.queue_poisoned.load(Ordering::SeqCst) {
            return DrainOutcome::RuntimeQuarantined;
        }
        if !self.rt.is_job_pending() {
            return DrainOutcome::Quiesced;
        }
        let _phase = PhaseScope::enter(ExecutionPhase::Cleanup);
        self.shared.job_queue_drains.fetch_add(1, Ordering::Relaxed);
        let deadline = Instant::now() + self.job_deadline;
        let _scope = InvocationScope::enter(0, Some(deadline));
        let mut executed: usize = 0;
        loop {
            if !self.rt.is_job_pending() {
                return DrainOutcome::Quiesced;
            }
            if Instant::now() >= deadline || executed >= MAX_CLEANUP_JOBS {
                // budget gone with work remaining: attempt a bounded kill
                // round, then quarantine through the unified terminal path
                self.kill_remaining_jobs();
                return DrainOutcome::RuntimeQuarantined;
            }
            *self.sync_deadline.lock().unwrap() = Some(deadline);
            let result = self.rt.execute_pending_job();
            *self.sync_deadline.lock().unwrap() = None;
            executed += 1;
            if result.is_err() {
                *self.last_error.lock().unwrap() = Some("pending job exception".into());
                if self.shared.interrupted.swap(false, Ordering::SeqCst) {
                    // deadline kill inside a job: remaining queued work (if
                    // any) still must not escape — kill round + quarantine
                    self.kill_remaining_jobs();
                    return DrainOutcome::RuntimeQuarantined;
                }
            }
        }
    }

    /// Force-terminate queued jobs by arming an already-expired deadline so
    /// every interrupt poll fires. Chains with poll points (loops) die at the
    /// killed reaction's rejection; tiny poll-less chains survive, which is
    /// why the caller ALWAYS quarantines afterwards when work remains. This
    /// function never poisons on its own (M2.2.1-r4.1).
    fn kill_remaining_jobs(&mut self) {
        let past = Instant::now()
            .checked_sub(Duration::from_millis(1))
            .unwrap_or_else(Instant::now);
        let mut handled: usize = 0;
        const MAX_KILL_ROUND: usize = 1_000;
        while self.rt.is_job_pending() && handled < MAX_KILL_ROUND {
            *self.sync_deadline.lock().unwrap() = Some(past);
            let result = self.rt.execute_pending_job();
            *self.sync_deadline.lock().unwrap() = None;
            handled += 1;
            if result.is_err() {
                // a poll-fired kill; clear the flag so later arming works
                self.shared.interrupted.swap(false, Ordering::SeqCst);
            }
        }
        if self.rt.is_job_pending() {
            // unified terminal path: fails pending invocations, aborts native
            // ops, corrects pending_ops, rejects future dynamic requests
            self.quarantine_runtime("cleanup queue unquiesceable");
        }
    }

    /// Drain remaining queued jobs under the nearest pending invocation's
    /// budget (or the watchdog when nothing is pending) and settle whatever
    /// promises completed. Used at message boundaries where multiple
    /// invocations may have queued work.
    fn settle_background(&mut self) {
        if self.rt.is_job_pending() {
            if let Some(budget) = self
                .pending
                .values()
                .min_by_key(|p| p.spec.deadline)
                .map(|p| JobBudget {
                    invocation_id: p.spec.id,
                    deadline: p.spec.deadline,
                })
            {
                self.drain_jobs_for(budget, ExecutionPhase::Invocation);
            } else {
                // ownerless leftover at a message boundary (all invocations
                // settled, jobs remain): bounded cleanup budget — the 5s
                // watchdog is reserved exclusively for shutdown (r4.2.2)
                self.drain_jobs_for(cleanup_budget(0), ExecutionPhase::Cleanup);
            }
        }
        if !self.pending.is_empty() {
            self.finish_resolved();
        }
    }

    /// Collect invocations recorded by the prelude watch table, clearing them,
    /// and reply. All JS access stays inside `with` scopes; only 'static
    /// Outcomes leave them.
    fn finish_resolved(&mut self) {
        if self.pending.is_empty() {
            return;
        }
        self.shared.settlement_scans.fetch_add(1, Ordering::Relaxed);
        let settled_ids: Vec<u64> = self.ctx.with(|ctx| -> Vec<u64> {
            let table: Object = match ctx.globals().get::<_, Object>("__velquSettled") {
                Ok(t) => t,
                Err(_) => return vec![],
            };
            table
                .keys::<String>()
                .filter_map(|k| k.ok().and_then(|k| k.parse::<u64>().ok()))
                .collect()
        });
        if settled_ids.is_empty() {
            return;
        }
        for id in settled_ids {
            let Some(p) = self.pending.remove(&id) else {
                // stale entry (cancelled invocation whose promise settled
                // afterwards) — remove it from the table
                self.clear_settled_entry(id);
                continue;
            };
            self.shared.promise_results.fetch_add(1, Ordering::Relaxed);
            let budget = JobBudget {
                invocation_id: p.spec.id,
                deadline: p.spec.deadline,
            };
            // M2.2.1-r4.2.2: route deadline already expired → the settled
            // response cannot be converted; clean up and reply Timeout.
            if Instant::now() >= budget.deadline {
                self.finish_timeout(id, p);
                continue;
            }

            // M2.3-A: Each settled Promise is processed as ONE owner-scoped unit:
            // 1. Enter InvocationScope and Invocation phase with the armed deadline.
            // 2. Convert response.
            // 3. Drain response-mapping microtasks under the OWNER's budget.
            // 4. Cancel floating ops.
            // 5. Settle request slot.
            // 6. Send response.
            let (raw_outcome, conversion_interrupted) = {
                let _scope = InvocationScope::enter(budget.invocation_id, Some(budget.deadline));
                let _phase = PhaseScope::enter(ExecutionPhase::Invocation);
                let _deadline_guard =
                    InterruptDeadlineScope::enter(Arc::clone(&self.sync_deadline), budget.deadline);
                let out = self.ctx.with(|ctx| -> Outcome {
                    let table: Object = match ctx.globals().get("__velquSettled") {
                        Ok(t) => t,
                        Err(_) => {
                            return Outcome::EngineFailure {
                                detail: "settled table missing".into(),
                                source: None,
                            }
                        }
                    };
                    let key = id.to_string();
                    let entry: Object = match table.get::<_, Object>(&key) {
                        Ok(e) => e,
                        Err(_) => {
                            return Outcome::EngineFailure {
                                detail: "settled entry vanished".into(),
                                source: None,
                            }
                        }
                    };
                    let ok: bool = entry.get("ok").unwrap_or(false);
                    table.remove(key.as_str()).ok();
                    let payload_key = if ok { "v" } else { "e" };
                    let payload: Value = entry
                        .get::<_, Value>(payload_key)
                        .unwrap_or_else(|_| Value::new_undefined(ctx.clone()));
                    let stringify_fn = self
                        .prelude
                        .as_ref()
                        .and_then(|pr| pr.stringify_fn.clone().restore(&ctx).ok());
                    if ok {
                        value_to_outcome(&ctx, &p.spec, &payload, stringify_fn.as_ref())
                    } else {
                        let exc = payload
                            .as_object()
                            .cloned()
                            .and_then(rquickjs::Exception::from_object);
                        let (msg, stack) = match exc {
                            Some(e) => (e.message().unwrap_or_default(), e.stack()),
                            None => (
                                format!("{:?}", any_js_to_json(&payload).unwrap_or(Json::Null)),
                                None,
                            ),
                        };
                        Outcome::EngineFailure {
                            detail: format!("{msg}\n{}", stack.clone().unwrap_or_default()),
                            source: stack.as_deref().and_then(map_first_frame),
                        }
                    }
                });
                let interrupted = self.shared.interrupted.swap(false, Ordering::SeqCst);
                (out, interrupted)
            };

            // M2.3-A: Drain response-mapping microtasks (from toJSON/getters)
            // under the original owner's budget BEFORE settling the request slot.
            let drain_report = if self.rt.is_job_pending() {
                if conversion_interrupted {
                    self.drain_jobs_for(cleanup_budget(id), ExecutionPhase::Cleanup)
                } else {
                    self.drain_jobs_for(budget, ExecutionPhase::Invocation)
                }
            } else {
                DrainReport {
                    outcome: DrainOutcome::Quiesced,
                    interrupted: false,
                }
            };

            // Single terminal timeout classification (P1 fix: never double-count)
            let timed_out = conversion_interrupted
                || drain_report.interrupted
                || matches!(drain_report.outcome, DrainOutcome::RuntimeQuarantined);
            let final_outcome = if timed_out {
                self.shared.timeouts.fetch_add(1, Ordering::Relaxed);
                Outcome::Timeout
            } else {
                if matches!(
                    raw_outcome,
                    Outcome::EngineFailure { .. } | Outcome::ContractViolation(_)
                ) {
                    self.shared.engine_failures.fetch_add(1, Ordering::Relaxed);
                }
                raw_outcome
            };

            self.abort_floating_ops(id);
            self.settle_request(p.handle);
            if let Some(reply) = p.reply {
                let _ = reply.send(final_outcome);
            }
        }
        if self.rt.is_job_pending() && !self.pending.is_empty() {
            self.settle_background();
        }
    }
}

struct PendingInvocation {
    spec: InvocationSpec,
    /// Typed capability minted by this worker at admission (M24-003-B); the
    /// only settlement identity for the invocation's request slot.
    handle: q_bridge::RequestHandle,
    reply: Option<tokio::sync::oneshot::Sender<Outcome>>,
}

/// Task liveness drop guard: decrements `native_tasks_alive` and increments
/// the matching terminal counter (`native_tasks_completed` or `native_tasks_aborted`)
/// ONLY when the Tokio task future is physically destroyed (M2.2.1-r4).
struct TaskLivenessGuard {
    shared: Arc<WorkerShared>,
    state: Arc<AtomicU8>,
}

impl Drop for TaskLivenessGuard {
    fn drop(&mut self) {
        dec_capped(&self.shared.native_tasks_alive);
        match self.state.load(Ordering::SeqCst) {
            1 => {
                self.shared
                    .native_tasks_completed
                    .fetch_add(1, Ordering::Relaxed);
            }
            2 => {
                self.shared
                    .native_tasks_aborted
                    .fetch_add(1, Ordering::Relaxed);
            }
            _ => {
                // Task dropped before completing or requesting abort (e.g. runtime shutdown/panic)
                self.shared
                    .native_tasks_aborted
                    .fetch_add(1, Ordering::Relaxed);
            }
        }
    }
}

/// Physically request abort for a native task (M2.2.1-r4).
/// Uses an atomic compare-exchange so exactly ONE transition wins:
/// if the task already finished (state == Completed), abort is not requested
/// and the task is never double-counted.
fn abort_op_task(state: &AtomicU8, handle: &tokio::task::AbortHandle) {
    if state
        .compare_exchange(
            NativeTaskState::Running as u8,
            NativeTaskState::AbortRequested as u8,
            Ordering::SeqCst,
            Ordering::SeqCst,
        )
        .is_ok()
    {
        handle.abort();
    }
}

fn dec_capped(counter: &AtomicU64) {
    let mut now = counter.load(Ordering::Relaxed);
    loop {
        if now == 0 {
            return;
        }
        match counter.compare_exchange_weak(now, now - 1, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => return,
            Err(actual) => now = actual,
        }
    }
}

// ---------------------------------------------------------------------------
// free functions operating inside a Ctx scope
// ---------------------------------------------------------------------------

fn call_runner<'js>(
    ctx: &rquickjs::Ctx<'js>,
    handler: &Persistent<Function<'static>>,
    policy: Option<&Persistent<Function<'static>>>,
    run_fn_persistent: &Persistent<Function<'static>>,
    make_ctx_persistent: &Persistent<Function<'static>>,
    make_req_persistent: &Persistent<Function<'static>>,
    spec: &InvocationSpec,
) -> rquickjs::Result<Value<'js>> {
    use rquickjs::IntoJs;
    let handler_fn: Function<'js> = handler.clone().restore(ctx)?;
    let run_fn: Function<'js> = run_fn_persistent.clone().restore(ctx)?;
    let make_ctx: Function<'js> = make_ctx_persistent.clone().restore(ctx)?;

    let pre = Object::new(ctx.clone())?;
    if let Some(ref val) = spec.params {
        pre.set("params", crate::convert::json_to_js(ctx, val)?)?;
    }
    if let Some(ref val) = spec.query {
        pre.set("query", crate::convert::json_to_js(ctx, val)?)?;
    }
    if let Some(ref val) = spec.headers {
        pre.set("headers", crate::convert::json_to_js(ctx, val)?)?;
    }
    if let Some(ref val) = spec.body {
        pre.set("body", crate::convert::json_to_js(ctx, val)?)?;
    }

    let slot = if spec.slot == q_engine::NO_REQUEST_SLOT {
        -1.0
    } else {
        spec.slot as f64
    };
    let gen = spec.generation as f64;
    let ctx_obj: Value<'js> = make_ctx.call((slot, gen, pre))?;

    let (policy_fn, req_obj) = match policy {
        Some(p) => {
            let p_fn: Function<'js> = p.clone().restore(ctx)?;
            let make_req: Function<'js> = make_req_persistent.clone().restore(ctx)?;
            let req: Value<'js> = make_req.call((slot, gen))?;
            (p_fn.into_value(), req)
        }
        None => (().into_js(ctx)?, Value::new_undefined(ctx.clone())),
    };

    run_fn.call::<_, Value<'js>>((handler_fn, policy_fn, ctx_obj, req_obj))
}

fn value_to_outcome<'js>(
    ctx: &rquickjs::Ctx<'js>,
    spec: &InvocationSpec,
    value: &Value<'js>,
    stringify_fn: Option<&Function<'js>>,
) -> Outcome {
    if value.is_undefined() || value.is_null() {
        return Outcome::Response {
            status: spec.default_status,
            body: BodyOut::Empty,
            headers: vec![],
        };
    }
    if value.is_string() {
        let s = value
            .clone()
            .get::<rquickjs::Coerced<String>>()
            .map(|c| c.0)
            .unwrap_or_default();
        return Outcome::Response {
            status: spec.default_status,
            body: BodyOut::Text(s),
            headers: vec![],
        };
    }
    if let Some(bytes) = js_to_bytes(value) {
        return Outcome::Response {
            status: spec.default_status,
            body: BodyOut::Bytes(bytes),
            headers: vec![],
        };
    }
    if let Some(obj) = value.as_object() {
        let is_problem: bool = obj.get("__problem").unwrap_or(false);
        if is_problem {
            return problem_from_object(obj);
        }
        // Result envelopes are TAGGED only: status().value() produces
        // {__ok, status, value}, problems produce {__problem, ...}. A plain
        // business object that happens to contain `status`/`value` fields is a
        // BODY, never an envelope.
        let is_envelope: bool = obj.get("__ok").unwrap_or(false);
        let status = if is_envelope {
            match obj.get::<_, Option<rquickjs::Coerced<f64>>>("status") {
                Ok(Some(c)) => c.0 as u16,
                _ => spec.default_status,
            }
        } else {
            spec.default_status
        };
        if !spec.allowed_statuses.contains(&status) {
            return Outcome::ContractViolation(format!(
                "route {} returned undeclared status {status} (declared: {:?})",
                spec.route_id, spec.allowed_statuses
            ));
        }
        let headers = obj
            .get::<_, Option<Object>>("headers")
            .ok()
            .flatten()
            .map(|h| {
                h.keys::<String>()
                    .filter_map(|k| {
                        k.ok().and_then(|k| {
                            let v: rquickjs::Coerced<String> = h.get(k.as_str()).ok()?;
                            Some((k, v.0))
                        })
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        let body_value: Value = if is_envelope {
            match obj.get::<_, Option<Value>>("value").ok().flatten() {
                Some(v) if !v.is_undefined() => v,
                _ => Value::new_undefined(ctx.clone()),
            }
        } else {
            value.clone()
        };
        let body = body_from_value(ctx, spec.response_strategy, &body_value, stringify_fn);
        return Outcome::Response {
            status,
            body,
            headers,
        };
    }
    // bare number/bool → JSON
    let body = body_from_value(ctx, spec.response_strategy, value, stringify_fn);
    Outcome::Response {
        status: spec.default_status,
        body,
        headers: vec![],
    }
}

fn body_from_value<'js>(
    ctx: &rquickjs::Ctx<'js>,
    strategy: ResponseStrategy,
    v: &Value<'js>,
    stringify_fn: Option<&Function<'js>>,
) -> BodyOut {
    if v.is_string() {
        let s = v
            .clone()
            .get::<rquickjs::Coerced<String>>()
            .map(|c| c.0)
            .unwrap_or_default();
        return BodyOut::Text(s);
    }
    if let Some(bytes) = js_to_bytes(v) {
        return BodyOut::Bytes(bytes);
    }
    match strategy {
        ResponseStrategy::Js => {
            if let Some(s_fn) = stringify_fn {
                match s_fn.call::<_, Option<rquickjs::Coerced<String>>>((v.clone(),)) {
                    Ok(Some(text)) => BodyOut::JsonText(text.0),
                    _ => BodyOut::Empty,
                }
            } else {
                match engine_stringify(ctx, v) {
                    Ok(Some(text)) => BodyOut::JsonText(text),
                    _ => BodyOut::Empty,
                }
            }
        }
        ResponseStrategy::Native => match any_js_to_json(v) {
            Ok(json) => BodyOut::Json(json),
            Err(_) => BodyOut::Empty,
        },
    }
}

fn problem_from_object(obj: &Object<'_>) -> Outcome {
    let status: u16 = obj
        .get::<_, rquickjs::Coerced<f64>>("status")
        .map(|c| c.0 as u16)
        .unwrap_or(500);
    let problem_id: String = obj
        .get::<_, rquickjs::Coerced<String>>("problem")
        .map(|c| c.0)
        .unwrap_or_else(|_| "internal".into());
    let detail: Option<String> = obj
        .get::<_, Option<rquickjs::Coerced<String>>>("detail")
        .ok()
        .flatten()
        .map(|c| c.0);
    let errors = obj
        .get::<_, Option<rquickjs::Array>>("errors")
        .ok()
        .flatten()
        .map(|arr| {
            arr.iter::<Value>()
                .filter_map(|item| item.ok())
                .filter_map(|item| {
                    let o = item.as_object()?;
                    let path: rquickjs::Coerced<String> = o.get("path").ok()?;
                    let code: rquickjs::Coerced<String> = o.get("code").ok()?;
                    let message: rquickjs::Coerced<String> = o.get("message").ok()?;
                    Some(FieldErrorOut {
                        path: path.0,
                        code: code.0,
                        message: message.0,
                    })
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    Outcome::Problem(ProblemOut {
        problem_id,
        status,
        detail,
        errors,
    })
}

fn describe_exception(
    ctx: &rquickjs::Ctx<'_>,
    e: &rquickjs::Error,
) -> (String, Option<SourceLocation>) {
    if matches!(e, rquickjs::Error::Exception) {
        let caught = ctx.catch();
        if let Some(o) = caught.as_object() {
            if let Some(exc) = rquickjs::Exception::from_object(o.clone()) {
                let msg = exc.message().unwrap_or_else(|| "unknown exception".into());
                let stack = exc.stack();
                let source = stack.as_deref().and_then(map_first_frame);
                return (
                    format!("{msg}\n{}", stack.clone().unwrap_or_default()),
                    source,
                );
            }
        }
    }
    (format!("{e:?}"), None)
}

fn describe_error(ctx: &rquickjs::Ctx<'_>, e: &rquickjs::Error) -> String {
    if matches!(e, rquickjs::Error::Exception) {
        let caught = ctx.catch();
        if let Some(o) = caught.as_object() {
            if let Some(exc) = rquickjs::Exception::from_object(o.clone()) {
                let msg = exc.message().unwrap_or_else(|| "unknown".into());
                let stack = exc.stack().unwrap_or_default();
                return format!("{msg}\n{stack}");
            }
        }
    }
    format!("{e:?}")
}

fn map_first_frame(stack: &str) -> Option<SourceLocation> {
    let mapper = MAPPER.with(|m| m.borrow().clone());
    map_first_frame_with(mapper?.as_ref(), stack)
}

/// Map the first engine stack frame to an original source location.
fn map_first_frame_with(mapper: &dyn SourceMapper, stack: &str) -> Option<SourceLocation> {
    let mut generated = None;
    for line in stack.lines() {
        let line = line.trim();
        if !line.starts_with("at ") {
            continue;
        }
        if let Some(open) = line.rfind('(') {
            let loc = line[open + 1..].trim_end_matches(')');
            let mut parts = loc.rsplitn(3, ':');
            let col: u32 = parts.next()?.parse().ok()?;
            let line_no: u32 = parts.next()?.parse().ok()?;
            let file = parts.next().unwrap_or("app.js").to_string();
            generated = Some((file, line_no, col));
            break;
        }
    }
    let (file, line_no, col) = generated?;
    // bundles evaluate as "eval_script" or the pack entry name; every
    // generated frame is mappable
    let original = if !file.ends_with(".rs") {
        mapper.map(line_no, col)
    } else {
        None
    };
    Some(SourceLocation {
        generated: Some((line_no, col)),
        original: original.or(Some(q_engine::OriginalLocation {
            source: file,
            line: line_no,
            column: col,
        })),
    })
}

fn meta_path_slice<'a>(m: &'a q_engine::RequestMeta, spec: &q_engine::ParamSpec) -> &'a str {
    m.path
        .get(spec.start as usize..spec.end as usize)
        .unwrap_or_default()
}

fn install_natives(
    ctx: &rquickjs::Ctx<'_>,
    store: Rc<RequestStore>,
    shared: Arc<WorkerShared>,
    ops: Arc<OpRegistry>,
    tokio_handle: tokio::runtime::Handle,
) -> rquickjs::Result<()> {
    let globals = ctx.globals();

    // request field access: JSON-encoded object string (engine-side JSON.parse)
    {
        let store = Rc::clone(&store);
        let f = move |ctx: rquickjs::Ctx,
                      slot: f64,
                      gen: f64,
                      what: String|
              -> rquickjs::Result<String> {
            let slot = slot as usize;
            if slot == q_engine::NO_REQUEST_SLOT {
                return Err(rquickjs::Exception::throw_message(
                    &ctx,
                    "request handle unavailable for field-free route",
                ));
            }
            let json = store
                .access(store.local_handle(slot, gen as u64), 1, 16, |m| {
                    let map = match what.as_str() {
                        // M24-004-D: values materialize from path byte ranges
                        // only at this whole-field access
                        "params" => serde_json::Map::from_iter(m.param_specs.iter().map(|spec| {
                            let value = meta_path_slice(m, spec);
                            (spec.name.clone(), Json::String(value.to_string()))
                        })),
                        "query" => serde_json::Map::from_iter(
                            m.query
                                .iter()
                                .map(|(k, v)| (k.clone(), Json::String(v.clone()))),
                        ),
                        "headers" => serde_json::Map::from_iter(
                            m.headers
                                .iter()
                                .map(|(k, v)| (k.clone(), Json::String(v.clone()))),
                        ),
                        _ => serde_json::Map::new(),
                    };
                    serde_json::to_string(&Json::Object(map)).unwrap_or_else(|_| "{}".into())
                })
                .map_err(|_| rquickjs::Exception::throw_message(&ctx, "request handle expired"))?;
            Ok(json)
        };
        globals.set("__velquReqRaw", Function::new(ctx.clone(), f)?)?;
    }
    // M24-004-D: declared parameter names (route identity, not request data)
    {
        let store = Rc::clone(&store);
        let f = move |ctx: rquickjs::Ctx, slot: f64, gen: f64| -> rquickjs::Result<String> {
            let slot = slot as usize;
            if slot == q_engine::NO_REQUEST_SLOT {
                return Err(rquickjs::Exception::throw_message(
                    &ctx,
                    "request handle unavailable for field-free route",
                ));
            }
            store
                .access(store.local_handle(slot, gen as u64), 0, 0, |m| {
                    let names: Vec<&str> = m.param_specs.iter().map(|s| s.name.as_str()).collect();
                    serde_json::to_string(&names).unwrap_or_else(|_| "[]".into())
                })
                .map_err(|_| rquickjs::Exception::throw_message(&ctx, "request handle expired"))
        };
        globals.set("__velquReqParamNames", Function::new(ctx.clone(), f)?)?;
    }
    // M24-004-D: single-key parameter access — materializes exactly one value
    {
        let store = Rc::clone(&store);
        let f = move |ctx: rquickjs::Ctx,
                      slot: f64,
                      gen: f64,
                      key: String|
              -> rquickjs::Result<Option<String>> {
            let slot = slot as usize;
            if slot == q_engine::NO_REQUEST_SLOT {
                return Err(rquickjs::Exception::throw_message(
                    &ctx,
                    "request handle unavailable for field-free route",
                ));
            }
            store
                .access(store.local_handle(slot, gen as u64), 1, 0, |m| {
                    m.param_specs
                        .iter()
                        .find(|spec| spec.name == key)
                        .map(|spec| meta_path_slice(m, spec).to_string())
                })
                .inspect(|value| {
                    // charge the materialized value's exact byte length
                    if let Some(v) = value.as_ref() {
                        store
                            .counters()
                            .materialized_bytes
                            .fetch_add(v.len() as u64, std::sync::atomic::Ordering::Relaxed);
                    }
                })
                .map_err(|_| rquickjs::Exception::throw_message(&ctx, "request handle expired"))
        };
        globals.set("__velquReqParam", Function::new(ctx.clone(), f)?)?;
    }
    {
        let store = Rc::clone(&store);
        let f = move |ctx: rquickjs::Ctx, slot: f64, gen: f64| -> rquickjs::Result<String> {
            let slot = slot as usize;
            if slot == q_engine::NO_REQUEST_SLOT {
                return Err(rquickjs::Exception::throw_message(
                    &ctx,
                    "request handle unavailable for field-free route",
                ));
            }
            let s = store
                .access(store.local_handle(slot, gen as u64), 1, 0, |m| {
                    String::from_utf8_lossy(m.body.as_deref().unwrap_or_default()).into_owned()
                })
                .map_err(|_| rquickjs::Exception::throw_message(&ctx, "request handle expired"))?;
            Ok(s)
        };
        globals.set("__velquReqBodyText", Function::new(ctx.clone(), f)?)?;
    }
    {
        let store = Rc::clone(&store);
        let f = move |ctx: rquickjs::Ctx, slot: f64, gen: f64| -> rquickjs::Result<f64> {
            let slot = slot as usize;
            if slot == q_engine::NO_REQUEST_SLOT {
                return Err(rquickjs::Exception::throw_message(
                    &ctx,
                    "request handle unavailable for field-free route",
                ));
            }
            let len = store
                .access(store.local_handle(slot, gen as u64), 0, 0, |m| {
                    m.body.as_deref().map_or(0, |b| b.len())
                })
                .map_err(|_| rquickjs::Exception::throw_message(&ctx, "request handle expired"))?;
            Ok(len as f64)
        };
        globals.set("__velquReqBodyLen", Function::new(ctx.clone(), f)?)?;
    }
    {
        let store = Rc::clone(&store);
        let f = move |ctx: rquickjs::Ctx,
                      slot: f64,
                      gen: f64,
                      target: TypedArray<'_, u8>|
              -> rquickjs::Result<()> {
            let slot = slot as usize;
            if slot == q_engine::NO_REQUEST_SLOT {
                return Err(rquickjs::Exception::throw_message(
                    &ctx,
                    "request handle unavailable for field-free route",
                ));
            }
            let body: Vec<u8> = store
                .access(store.local_handle(slot, gen as u64), 1, 0, |m| {
                    m.body.clone().unwrap_or_default()
                })
                .map_err(|_| rquickjs::Exception::throw_message(&ctx, "request handle expired"))?;
            // SAFETY (reviewed FFI boundary): `target` is a live Uint8Array
            // owned by this native call on this worker thread; its backing
            // store was pre-allocated from the same body snapshot and nothing
            // else can alias it while we hold the only reference.
            unsafe {
                if let Some(buf) = target.as_bytes() {
                    let n = body.len().min(buf.len());
                    if n > 0 {
                        std::ptr::copy_nonoverlapping(body.as_ptr(), buf.as_ptr() as *mut u8, n);
                    }
                }
            }
            Ok(())
        };
        globals.set("__velquFillBytes", Function::new(ctx.clone(), f)?)?;
    }
    // M24-005-B: declared header names + single-key header access — the
    // request carries only plan-declared headers; a key access materializes
    // exactly one value.
    {
        let store = Rc::clone(&store);
        let f = move |ctx: rquickjs::Ctx, slot: f64, gen: f64| -> rquickjs::Result<String> {
            let slot = slot as usize;
            if slot == q_engine::NO_REQUEST_SLOT {
                return Err(rquickjs::Exception::throw_message(
                    &ctx,
                    "request handle unavailable for field-free route",
                ));
            }
            store
                .access(store.local_handle(slot, gen as u64), 0, 0, |m| {
                    let names: Vec<&str> = m.headers.iter().map(|(k, _)| k.as_str()).collect();
                    serde_json::to_string(&names).unwrap_or_else(|_| "[]".into())
                })
                .map_err(|_| rquickjs::Exception::throw_message(&ctx, "request handle expired"))
        };
        globals.set("__velquReqHeaderNames", Function::new(ctx.clone(), f)?)?;
    }
    {
        let store = Rc::clone(&store);
        let f = move |ctx: rquickjs::Ctx,
                      slot: f64,
                      gen: f64,
                      key: String|
              -> rquickjs::Result<Option<String>> {
            let slot = slot as usize;
            if slot == q_engine::NO_REQUEST_SLOT {
                return Err(rquickjs::Exception::throw_message(
                    &ctx,
                    "request handle unavailable for field-free route",
                ));
            }
            store
                .access(store.local_handle(slot, gen as u64), 1, 0, |m| {
                    m.headers
                        .iter()
                        .find(|(k, _)| *k == key)
                        .map(|(_, v)| v.clone())
                })
                .inspect(|value| {
                    if let Some(v) = value.as_ref() {
                        store
                            .counters()
                            .materialized_bytes
                            .fetch_add(v.len() as u64, std::sync::atomic::Ordering::Relaxed);
                    }
                })
                .map_err(|_| rquickjs::Exception::throw_message(&ctx, "request handle expired"))
        };
        globals.set("__velquReqHeader", Function::new(ctx.clone(), f)?)?;
    }
    // timer capability: (ms) -> op id; completion returns on the worker loop
    {
        let shared_timer = Arc::clone(&shared);
        let ops2 = Arc::clone(&ops);
        let tokio = tokio_handle.clone();
        let f = move |ctx: rquickjs::Ctx, ms: f64| -> rquickjs::Result<f64> {
            // M2.2.1-r3 execution-phase guard: native operations may only
            // start inside a LIVE invocation. Cleanup/Shutdown reactions that
            // try to spawn second-generation ops fail deterministically, and
            // nothing can start ownerless ops while the worker is idle.
            match CURRENT_PHASE.with(|p| p.get()) {
                ExecutionPhase::Invocation => {}
                ExecutionPhase::Cleanup => {
                    return Err(rquickjs::Exception::throw_message(
                        &ctx,
                        "native operations are unavailable during invocation cleanup",
                    ));
                }
                ExecutionPhase::Shutdown => {
                    return Err(rquickjs::Exception::throw_message(
                        &ctx,
                        "native operations are unavailable during shutdown",
                    ));
                }
                ExecutionPhase::Idle => {
                    return Err(rquickjs::Exception::throw_message(
                        &ctx,
                        "native operation started outside an invocation",
                    ));
                }
            }
            let ops = &ops2;
            let op_id = ops.op_clock.fetch_add(1, Ordering::SeqCst);
            let ms = ms.max(0.0) as u64;
            if ops.ops.lock().unwrap().len() >= ops.op_cap {
                return Err(rquickjs::Exception::throw_message(
                    &ctx,
                    "pending operation limit reached",
                ));
            }
            // M2.2.1-r2: capture BOTH the owning invocation and its live
            // deadline so the completion continuation drains under the right
            // scope even when other invocations started meanwhile.
            let invocation_id = CURRENT_INVOCATION.with(|c| c.get());
            let deadline = CURRENT_DEADLINE
                .with(|c| c.get())
                .unwrap_or_else(|| Instant::now() + ops.job_watchdog);
            let tx = WORKER_TX.with(|c| c.borrow().clone());
            let Some(tx) = tx else {
                return Err(rquickjs::Exception::throw_message(
                    &ctx,
                    "timer capability unavailable",
                ));
            };
            // M2.2.1-r4: Task state machine and TaskLivenessGuard.
            // Increment native_tasks_started and native_tasks_alive BEFORE spawn
            // so zero-delay tasks can never underflow native_tasks_alive.
            let state = Arc::new(AtomicU8::new(NativeTaskState::Running as u8));
            shared_timer
                .native_tasks_started
                .fetch_add(1, Ordering::Relaxed);
            shared_timer
                .native_tasks_alive
                .fetch_add(1, Ordering::Relaxed);
            shared_timer
                .timer_ops_started
                .fetch_add(1, Ordering::Relaxed);
            shared_timer.pending_ops.fetch_add(1, Ordering::Relaxed);

            let guard = TaskLivenessGuard {
                shared: Arc::clone(&shared_timer),
                state: Arc::clone(&state),
            };
            let task_state = Arc::clone(&state);
            let task = tokio.spawn(async move {
                let _guard = guard;
                tokio::time::sleep(Duration::from_millis(ms)).await;
                if task_state
                    .compare_exchange(
                        NativeTaskState::Running as u8,
                        NativeTaskState::Completed as u8,
                        Ordering::SeqCst,
                        Ordering::SeqCst,
                    )
                    .is_ok()
                {
                    let _ = tx.send(WorkerMsg::TimerFired {
                        op_id,
                        result: Ok(ms),
                    });
                }
            });
            ops.ops.lock().unwrap().insert(
                op_id,
                PendingOp {
                    invocation_id,
                    deadline,
                    abort_handle: task.abort_handle(),
                    state,
                },
            );
            Ok(op_id as f64)
        };
        globals.set("__velquTimerStart", Function::new(ctx.clone(), f)?)?;
    }
    Ok(())
}

/// Reset pending_ops to 0 on quarantine while checking for accounting drift (M2.2.1-r4.2.2).
pub(crate) fn reset_pending_ops_after_quarantine(shared: &WorkerShared, removed: u64) {
    let before = shared.pending_ops.swap(0, Ordering::SeqCst);
    if before != removed {
        shared.boundary_violations.fetch_add(1, Ordering::Relaxed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// M2.2.1-r4.2.2: deliberate accounting drift — the op MAP has 1 entry
    /// while the pending_ops GAUGE reads 0. Quarantine must still end with
    /// pending_ops == 0 (no unsigned wrap), and the drift must be recorded as
    /// a boundary violation.
    #[test]
    fn quarantine_accounting_drift_resets_pending_ops_to_zero() {
        let shared = WorkerShared::new();
        // simulate drift: gauge says 0, map will contain 1 op
        assert_eq!(shared.pending_ops.load(Ordering::SeqCst), 0);

        // execute the actual production helper with a drifted gauge
        reset_pending_ops_after_quarantine(&shared, 1);

        // terminal gauge is zero, NOT u64::MAX (which fetch_sub would produce)
        assert_eq!(
            shared.pending_ops.load(Ordering::SeqCst),
            0,
            "swap(0) guarantees zero — fetch_sub(1) from 0 would wrap to u64::MAX"
        );
        assert_eq!(shared.boundary_violations.load(Ordering::Relaxed), 1);
    }
}
