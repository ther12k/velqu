//! The single QuickJS worker thread: message loop, invocation lifecycle,
//! native timer capability, cancellation, and outcome mapping.
//!
//! rquickjs scoping rule: JS values live only inside `ctx.with(...)` closures,
//! so every code path below converts to `'static` data (Outcome, LoadStats)
//! before leaving the closure.

use std::collections::{BTreeMap, HashMap};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use rquickjs::{Context, Function, Object, Persistent, Promise, Runtime, TypedArray, Value};
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
        expected: BTreeMap<String, String>,
        reply: std::sync::mpsc::Sender<Result<LoadStats, String>>,
    },
    Invoke(Box<InvokeJob>),
    Cancel { id: u64 },
    TimerFired { op_id: u64, result: Result<u64, String> },
    Shutdown,
}

/// Native-side op bookkeeping: which invocation owns this pending operation.
/// The promise callbacks stay in the JS-side op table (see prelude).
pub(crate) struct PendingOp {
    invocation_id: u64,
}

/// Bounded pending-op registry. Worker-thread only (holds Persistent JS
/// functions which are not Send); referenced by the timer native closure.
pub(crate) struct OpRegistry {
    pub ops: Mutex<HashMap<u64, PendingOp>>,
    pub op_cap: usize,
    pub op_clock: AtomicU64,
}

impl OpRegistry {
    fn new(op_cap: usize) -> Self {
        OpRegistry { ops: Mutex::new(HashMap::new()), op_cap, op_clock: AtomicU64::new(1) }
    }
}

/// Stats shared between the handle and the worker (Send-safe atomics only).
pub(crate) struct WorkerShared {
    pub invocations: AtomicU64,
    pub policy_calls: AtomicU64,
    pub handler_calls: AtomicU64,
    pub timer_ops_started: AtomicU64,
    pub timer_ops_completed: AtomicU64,
    pub late_completions_dropped: AtomicU64,
    pub cancelled: AtomicU64,
    pub timeouts: AtomicU64,
    pub engine_failures: AtomicU64,
    pub contract_violations: AtomicU64,
    pub heap_used: AtomicU64,
}

impl WorkerShared {
    pub(crate) fn new() -> Self {
        WorkerShared {
            invocations: AtomicU64::new(0),
            policy_calls: AtomicU64::new(0),
            handler_calls: AtomicU64::new(0),
            timer_ops_started: AtomicU64::new(0),
            timer_ops_completed: AtomicU64::new(0),
            late_completions_dropped: AtomicU64::new(0),
            cancelled: AtomicU64::new(0),
            timeouts: AtomicU64::new(0),
            engine_failures: AtomicU64::new(0),
            contract_violations: AtomicU64::new(0),
            heap_used: AtomicU64::new(0),
        }
    }

    pub(crate) fn stats(&self) -> EngineStats {
        EngineStats {
            invocations: self.invocations.load(Ordering::Relaxed),
            policy_calls: self.policy_calls.load(Ordering::Relaxed),
            handler_calls: self.handler_calls.load(Ordering::Relaxed),
            timer_ops_started: self.timer_ops_started.load(Ordering::Relaxed),
            timer_ops_completed: self.timer_ops_completed.load(Ordering::Relaxed),
            late_completions_dropped: self.late_completions_dropped.load(Ordering::Relaxed),
            cancelled_invocations: self.cancelled.load(Ordering::Relaxed),
            timeouts: self.timeouts.load(Ordering::Relaxed),
            engine_failures: self.engine_failures.load(Ordering::Relaxed),
            contract_violations: self.contract_violations.load(Ordering::Relaxed),
            heap_used: self.heap_used.load(Ordering::Relaxed) as usize,
        }
    }
}

/// Field order matters: Rust drops fields in declaration order, so the
/// Persistent handler cache and the Context MUST drop before the Runtime
/// (QuickJS asserts on live objects at JS_FreeRuntime otherwise).
pub(crate) struct WorkerInner {
    handler_cache: BTreeMap<String, Persistent<Function<'static>>>,
    ctx: Context,
    rt: Runtime,
    store: Arc<q_bridge::RequestStore>,
    shared: Arc<WorkerShared>,
    last_error: Arc<Mutex<Option<String>>>,
    sync_deadline: Arc<Mutex<Option<Instant>>>,
    ops: Arc<OpRegistry>,
}

thread_local! {
    static CURRENT_INVOCATION: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
    static WORKER_TX: std::cell::RefCell<Option<std::sync::mpsc::Sender<WorkerMsg>>> =
        const { std::cell::RefCell::new(None) };
    static MAPPER: std::cell::RefCell<Option<Arc<dyn SourceMapper>>> = const { std::cell::RefCell::new(None) };
}

impl WorkerInner {
    pub(crate) fn new(
        config: QuickJsConfig,
        store: Arc<q_bridge::RequestStore>,
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
            rt.set_interrupt_handler(Some(Box::new(move || {
                let dl = deadline.lock().unwrap();
                matches!(*dl, Some(d) if d <= Instant::now())
            })));
        }
        let ctx = Context::full(&rt).map_err(|e| format!("quickjs context: {e}"))?;
        let ops = Arc::new(OpRegistry::new(config.pending_op_cap));
        MAPPER.with(|m| *m.borrow_mut() = Some(Arc::clone(&mapper)));
        ctx.with(|ctx| -> Result<(), String> {
            install_natives(&ctx, Arc::clone(&store), Arc::clone(&shared), Arc::clone(&ops), tokio_handle)
                .map_err(|e| format!("natives failed: {e:?}"))?;
            ctx.eval::<(), _>(PRELUDE).map_err(|e| format!("prelude failed: {e:?}"))?;
            Ok(())
        })?;
        Ok(WorkerInner {
            handler_cache: BTreeMap::new(),
            ctx,
            rt,
            store,
            shared,
            last_error,
            sync_deadline,
            ops,
        })
    }

    pub(crate) fn run(mut self, rx: std::sync::mpsc::Receiver<WorkerMsg>, tx: std::sync::mpsc::Sender<WorkerMsg>) {
        WORKER_TX.with(|c| *c.borrow_mut() = Some(tx));
        let mut pending: BTreeMap<u64, PendingInvocation> = BTreeMap::new();
        loop {
            let next_deadline = pending.values().map(|p| p.spec.deadline).min();
            let msg = match next_deadline {
                Some(dl) => {
                    let now = Instant::now();
                    if dl <= now {
                        self.expire_timeouts(&mut pending);
                        continue;
                    }
                    match rx.recv_timeout(dl - now) {
                        Ok(m) => m,
                        Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                            self.expire_timeouts(&mut pending);
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
                WorkerMsg::Load { bundle, expected, reply } => {
                    let _ = reply.send(self.load(&bundle, &expected));
                }
                WorkerMsg::Invoke(job) => {
                    self.begin_invocation(*job, &mut pending);
                    self.drain_jobs();
                    self.finish_resolved(&mut pending);
                }
                WorkerMsg::Cancel { id } => {
                    self.cancel_invocation(id, &mut pending);
                    self.drain_jobs();
                    self.finish_resolved(&mut pending);
                }
                WorkerMsg::TimerFired { op_id, result } => {
                    self.complete_timer(op_id, result);
                    self.drain_jobs();
                    self.finish_resolved(&mut pending);
                }
                WorkerMsg::Shutdown => break,
            }
            self.shared
                .heap_used
                .store(self.rt.memory_usage().memory_used_size as u64, Ordering::Relaxed);
        }
        // deterministic cleanup: reject outstanding work so continuations unwind
        let ids: Vec<u64> = pending.keys().copied().collect();
        for id in ids {
            self.cancel_invocation(id, &mut pending);
            self.drain_jobs();
        }
        self.ops.ops.lock().unwrap().clear();
    }

    fn load(&mut self, bundle: &str, expected: &BTreeMap<String, String>) -> Result<LoadStats, String> {
        let t0 = Instant::now();
        let (register_calls, cache): (usize, BTreeMap<String, Persistent<Function<'static>>>) =
            self.ctx.with(|ctx| -> Result<(usize, BTreeMap<String, Persistent<Function<'static>>>), String> {
                ctx.eval::<Value, _>(bundle)
                    .map_err(|e| format!("bundle evaluation failed: {}", describe_error(&ctx, &e)))?;
                let handlers: Object = ctx
                    .globals()
                    .get::<_, Object>("__velquHandlers")
                    .map_err(|_| "prelude state missing".to_string())?;
                let mut count = 0usize;
                let mut cache = BTreeMap::new();
                for key in handlers.keys::<String>() {
                    let key = key.map_err(|e| e.to_string())?;
                    let f: Function = handlers.get(key.as_str()).map_err(|e| e.to_string())?;
                    count += 1;
                    cache.insert(key, Persistent::save(&ctx, f));
                }
                Ok((count, cache))
            })?;
        let eval_ms = t0.elapsed().as_secs_f64() * 1000.0;
        let missing: Vec<&String> = expected.keys().filter(|k| !cache.contains_key(*k)).collect();
        let extra: Vec<&String> = cache.keys().filter(|k| !expected.contains_key(*k)).collect();
        if !missing.is_empty() || !extra.is_empty() {
            return Err(format!("handler table mismatch (missing={missing:?} extra={extra:?})"));
        }
        if register_calls != expected.len() {
            return Err(format!("handler registration count {register_calls} != expected {}", expected.len()));
        }
        self.handler_cache = cache;
        Ok(LoadStats { handlers_registered: register_calls, eval_ms, register_calls })
    }

    fn begin_invocation(&mut self, job: InvokeJob, pending: &mut BTreeMap<u64, PendingInvocation>) {
        let InvokeJob { spec, reply } = job;
        self.shared.invocations.fetch_add(1, Ordering::Relaxed);
        if spec.policy_key.is_some() {
            self.shared.policy_calls.fetch_add(1, Ordering::Relaxed);
        }
        self.shared.handler_calls.fetch_add(1, Ordering::Relaxed);
        let Some(handler) = self.handler_cache.get(&spec.handler_key).cloned() else {
            if let Some(r) = reply {
                let _ = r.send(Outcome::EngineFailure {
                    detail: format!("handler {} not in cache", spec.handler_key),
                    source: None,
                });
            }
            return;
        };
        CURRENT_INVOCATION.with(|c| c.set(spec.id));
        enum Step {
            Immediate(Outcome),
            Watched,
            Failed(Outcome),
        }
        let deadline_cell = Arc::clone(&self.sync_deadline);
        let spec_deadline = spec.deadline;
        let spec_id = spec.id;
        let step = self.ctx.with(|ctx| {
            *deadline_cell.lock().unwrap() = Some(spec_deadline);
            let out = call_runner(&ctx, &handler, &spec);
            *deadline_cell.lock().unwrap() = None;
            match out {
                Err(e) => {
                    let (detail, source) = describe_exception(&ctx, &e);
                    Step::Failed(Outcome::EngineFailure { detail, source })
                }
                Ok(value) => {
                    if value.is_promise() {
                        let promise: Promise = value.get().unwrap();
                        let watch: Function = ctx.globals().get("__velquWatch").unwrap();
                        match watch.call::<_, ()>((promise, spec_id as f64)) {
                            Ok(()) => Step::Watched,
                            Err(_) => Step::Failed(Outcome::EngineFailure {
                                detail: "failed to attach promise watch".into(),
                                source: None,
                            }),
                        }
                    } else {
                        Step::Immediate(value_to_outcome(&ctx, &spec, &value))
                    }
                }
            }
        });
        let Some(reply) = reply else { return }; // cancelled before start
        match step {
            Step::Failed(o) => {
                // an engine failure whose deadline already passed is a timeout:
                // the interrupt handler is what broke the runaway execution
                let o = if spec.deadline <= Instant::now() {
                    self.shared.timeouts.fetch_add(1, Ordering::Relaxed);
                    Outcome::Timeout
                } else {
                    self.shared.engine_failures.fetch_add(1, Ordering::Relaxed);
                    o
                };
                self.store.settle(spec.slot, spec.generation);
                let _ = reply.send(o);
            }
            Step::Immediate(outcome) => {
                if matches!(outcome, Outcome::EngineFailure { .. } | Outcome::ContractViolation(_)) {
                    self.shared.engine_failures.fetch_add(1, Ordering::Relaxed);
                }
                self.store.settle(spec.slot, spec.generation);
                let _ = reply.send(outcome);
            }
            Step::Watched => {
                pending.insert(spec.id, PendingInvocation { spec, reply: Some(reply) });
            }
        }
    }

    fn complete_timer(&mut self, op_id: u64, result: Result<u64, String>) {
        let op = self.ops.ops.lock().unwrap().remove(&op_id);
        let Some(_op) = op else {
            self.shared.late_completions_dropped.fetch_add(1, Ordering::Relaxed);
            return;
        };
        self.shared.timer_ops_completed.fetch_add(1, Ordering::Relaxed);
        let _ = self.ctx.with(|ctx| -> rquickjs::Result<()> {
            match result {
                Ok(ms) => {
                    let f: Function = ctx.globals().get("__velquOpResolve")?;
                    f.call::<_, ()>((op_id as f64, ms as f64))
                }
                Err(reason) => {
                    let f: Function = ctx.globals().get("__velquOpReject")?;
                    f.call::<_, ()>((op_id as f64, reason))
                }
            }
        });
    }

    fn cancel_invocation(&mut self, id: u64, pending: &mut BTreeMap<u64, PendingInvocation>) {
        let Some(mut p) = pending.remove(&id) else {
            return;
        };
        self.shared.cancelled.fetch_add(1, Ordering::Relaxed);
        self.store.settle(p.spec.slot, p.spec.generation);
        self.reject_ops_of(id);
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

    fn expire_timeouts(&mut self, pending: &mut BTreeMap<u64, PendingInvocation>) {
        let now = Instant::now();
        let due: Vec<u64> = pending
            .values()
            .filter(|p| p.spec.deadline <= now)
            .map(|p| p.spec.id)
            .collect();
        for id in due {
            let Some(mut p) = pending.remove(&id) else { continue };
            self.shared.timeouts.fetch_add(1, Ordering::Relaxed);
            self.store.settle(p.spec.slot, p.spec.generation);
            self.reject_ops_of(id);
            if let Some(reply) = p.reply.take() {
                let _ = reply.send(Outcome::Timeout);
            }
        }
    }

    fn drain_jobs(&mut self) {
        while self.rt.is_job_pending() {
            match self.rt.execute_pending_job() {
                Ok(false) => break,
                Ok(true) => {}
                Err(_job_exc) => {
                    *self.last_error.lock().unwrap() = Some("pending job exception".into());
                    continue;
                }
            }
        }
    }

    /// Collect invocations recorded by the prelude watch table, clearing them,
    /// and reply. All JS access stays inside `with` scopes; only 'static
    /// Outcomes leave them.
    fn finish_resolved(&mut self, pending: &mut BTreeMap<u64, PendingInvocation>) {
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
        for id in settled_ids {
            let Some(p) = pending.remove(&id) else {
                // stale entry (cancelled invocation whose promise settled
                // afterwards) — remove it from the table
                self.ctx.with(|ctx| -> rquickjs::Result<()> {
                    let table: Object = ctx.globals().get("__velquSettled")?;
                    table.remove(id.to_string().as_str())?;
                    Ok(())
                })
                .ok();
                continue;
            };
            let outcome = self.ctx.with(|ctx| -> Outcome {
                let table: Object = ctx.globals().get("__velquSettled").unwrap();
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
                if ok {
                    value_to_outcome(&ctx, &p.spec, &payload)
                } else {
                    // rejection whose deadline already passed = deadline kill
                    if p.spec.deadline <= Instant::now() {
                        return Outcome::Timeout;
                    }
                    let exc = payload
                        .as_object()
                        .cloned()
                        .and_then(|o| rquickjs::Exception::from_object(o));
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
            if matches!(outcome, Outcome::Timeout) {
                self.shared.timeouts.fetch_add(1, Ordering::Relaxed);
            } else if matches!(outcome, Outcome::EngineFailure { .. } | Outcome::ContractViolation(_)) {
                self.shared.engine_failures.fetch_add(1, Ordering::Relaxed);
            }
            self.store.settle(p.spec.slot, p.spec.generation);
            if let Some(reply) = p.reply {
                let _ = reply.send(outcome);
            }
        }
    }
}

struct PendingInvocation {
    spec: InvocationSpec,
    reply: Option<tokio::sync::oneshot::Sender<Outcome>>,
}

// ---------------------------------------------------------------------------
// free functions operating inside a Ctx scope
// ---------------------------------------------------------------------------

fn call_runner<'js>(
    ctx: &rquickjs::Ctx<'js>,
    handler: &Persistent<Function<'static>>,
    spec: &InvocationSpec,
) -> rquickjs::Result<Value<'js>> {
    use rquickjs::IntoJs;
    let handler_fn: Function<'js> = handler.clone().restore(ctx)?;
    let run_fn: Function<'js> = ctx.globals().get("__velquRun")?;
    let make_ctx: Function<'js> = ctx.globals().get("__velquMakeCtx")?;
    let make_req: Function<'js> = ctx.globals().get("__velquMakeReq")?;

    let pre = Object::new(ctx.clone())?;
    let put = |obj: &Object<'js>, key: &str, v: &Option<Json>| -> rquickjs::Result<()> {
        match v {
            Some(val) => obj.set(key, crate::convert::json_to_js(ctx, val)?),
            None => obj.set(key, ().into_js(ctx)?),
        }
    };
    put(&pre, "params", &spec.params)?;
    put(&pre, "query", &spec.query)?;
    put(&pre, "headers", &spec.headers)?;
    put(&pre, "body", &spec.body)?;

    let slot = spec.slot as f64;
    let gen = spec.generation as f64;
    let ctx_obj: Value<'js> = make_ctx.call((slot, gen, pre))?;
    let req_obj: Value<'js> = make_req.call((slot, gen))?;

    let policy_fn: Value<'js> = match &spec.policy_key {
        Some(key) => {
            let handlers: Object = ctx.globals().get("__velquHandlers")?;
            handlers
                .get::<_, Option<Function<'js>>>(key.as_str())?
                .map(|f| f.into_value())
                .unwrap_or_else(|| ().into_js(ctx).unwrap())
        }
        None => ().into_js(ctx)?,
    };
    run_fn.call::<_, Value<'js>>((handler_fn, policy_fn, ctx_obj, req_obj))
}

fn value_to_outcome<'js>(ctx: &rquickjs::Ctx<'js>, spec: &InvocationSpec, value: &Value<'js>) -> Outcome {
    if value.is_undefined() || value.is_null() {
        return Outcome::Response { status: spec.default_status, body: BodyOut::Empty, headers: vec![] };
    }
    if value.is_string() {
        let s = value.clone().get::<rquickjs::Coerced<String>>().map(|c| c.0).unwrap_or_default();
        return Outcome::Response { status: spec.default_status, body: BodyOut::Text(s), headers: vec![] };
    }
    if let Some(bytes) = js_to_bytes(value) {
        return Outcome::Response { status: spec.default_status, body: BodyOut::Bytes(bytes), headers: vec![] };
    }
    if let Some(obj) = value.as_object() {
        let is_problem: bool = obj.get("__problem").unwrap_or(false);
        if is_problem {
            return problem_from_object(obj);
        }
        let explicit_status: Option<f64> = obj
            .get::<_, Option<rquickjs::Coerced<f64>>>("status")
            .ok()
            .flatten()
            .map(|c| c.0);
        let status = match explicit_status {
            Some(s) => s as u16,
            None => spec.default_status,
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
        let value_prop: Option<Value> = obj.get::<_, Option<Value>>("value").ok().flatten();
        let body_value: Value = match value_prop {
            Some(v) if !v.is_undefined() => v,
            _ => value.clone().into(),
        };
        let body = body_from_value(ctx, spec.response_strategy, &body_value);
        return Outcome::Response { status, body, headers };
    }
    // bare number/bool → JSON
    let body = body_from_value(ctx, spec.response_strategy, value);
    Outcome::Response { status: spec.default_status, body, headers: vec![] }
}

fn body_from_value<'js>(ctx: &rquickjs::Ctx<'js>, strategy: ResponseStrategy, v: &Value<'js>) -> BodyOut {
    if v.is_string() {
        let s = v.clone().get::<rquickjs::Coerced<String>>().map(|c| c.0).unwrap_or_default();
        return BodyOut::Text(s);
    }
    if let Some(bytes) = js_to_bytes(v) {
        return BodyOut::Bytes(bytes);
    }
    match strategy {
        ResponseStrategy::Js => match engine_stringify(ctx, v) {
            Ok(Some(text)) => BodyOut::JsonText(text),
            _ => BodyOut::Empty,
        },
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
                    Some(FieldErrorOut { path: path.0, code: code.0, message: message.0 })
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    Outcome::Problem(ProblemOut { problem_id, status, detail, errors })
}

fn describe_exception(ctx: &rquickjs::Ctx<'_>, e: &rquickjs::Error) -> (String, Option<SourceLocation>) {
    if matches!(e, rquickjs::Error::Exception) {
        let caught = ctx.catch();
        if let Some(o) = caught.as_object() {
            if let Some(exc) = rquickjs::Exception::from_object(o.clone()) {
                let msg = exc.message().unwrap_or_else(|| "unknown exception".into());
                let stack = exc.stack();
                let source = stack.as_deref().and_then(map_first_frame);
                return (format!("{msg}\n{}", stack.clone().unwrap_or_default()), source);
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
    let original = if file == "app.js" || file.ends_with(".js") {
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

fn install_natives(
    ctx: &rquickjs::Ctx<'_>,
    store: Arc<q_bridge::RequestStore>,
    shared: Arc<WorkerShared>,
    ops: Arc<OpRegistry>,
    tokio_handle: tokio::runtime::Handle,
) -> rquickjs::Result<()> {
    let globals = ctx.globals();

    // request field access: JSON-encoded object string (engine-side JSON.parse)
    {
        let store = Arc::clone(&store);
        let f = move |ctx: rquickjs::Ctx, slot: f64, gen: f64, what: String| -> rquickjs::Result<String> {
            let json = store
                .access(slot as usize, gen as u64, 1, 16, |m| {
                    let pairs: &[(String, String)] = match what.as_str() {
                        "params" => &m.params,
                        "query" => &m.query,
                        "headers" => &m.headers,
                        _ => &[],
                    };
                    let map = serde_json::Map::from_iter(
                        pairs.iter().map(|(k, v)| (k.clone(), Json::String(v.clone()))),
                    );
                    serde_json::to_string(&Json::Object(map)).unwrap_or_else(|_| "{}".into())
                })
                .map_err(|_| rquickjs::Exception::throw_message(&ctx, "request handle expired"))?;
            Ok(json)
        };
        globals.set("__velquReqRaw", Function::new(ctx.clone(), f)?)?;
    }
    {
        let store = Arc::clone(&store);
        let f = move |ctx: rquickjs::Ctx, slot: f64, gen: f64| -> rquickjs::Result<String> {
            let s = store
                .access(slot as usize, gen as u64, 1, 0, |m| {
                    String::from_utf8_lossy(m.body.as_deref().unwrap_or_default()).into_owned()
                })
                .map_err(|_| rquickjs::Exception::throw_message(&ctx, "request handle expired"))?;
            Ok(s)
        };
        globals.set("__velquReqBodyText", Function::new(ctx.clone(), f)?)?;
    }
    {
        let store = Arc::clone(&store);
        let f = move |ctx: rquickjs::Ctx, slot: f64, gen: f64| -> rquickjs::Result<f64> {
            let len = store
                .access(slot as usize, gen as u64, 0, 0, |m| m.body.as_deref().map_or(0, |b| b.len()))
                .map_err(|_| rquickjs::Exception::throw_message(&ctx, "request handle expired"))?;
            Ok(len as f64)
        };
        globals.set("__velquReqBodyLen", Function::new(ctx.clone(), f)?)?;
    }
    {
        let store = Arc::clone(&store);
        let f = move |ctx: rquickjs::Ctx, slot: f64, gen: f64, target: TypedArray<'_, u8>| -> rquickjs::Result<()> {
            let body: Vec<u8> = store
                .access(slot as usize, gen as u64, 1, 0, |m| m.body.clone().unwrap_or_default())
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
    // timer capability: (ms) -> op id; completion returns on the worker loop
    {
        let shared_timer = Arc::clone(&shared);
        let ops2 = Arc::clone(&ops);
        let tokio = tokio_handle.clone();
        let f = move |ctx: rquickjs::Ctx, ms: f64| -> rquickjs::Result<f64> {
            let ops = &ops2;
            let op_id = ops.op_clock.fetch_add(1, Ordering::SeqCst);
            let ms = ms.max(0.0) as u64;
            if ops.ops.lock().unwrap().len() >= ops.op_cap {
                return Err(rquickjs::Exception::throw_message(&ctx, "pending operation limit reached"));
            }
            ops.ops.lock().unwrap().insert(
                op_id,
                PendingOp {
                    invocation_id: CURRENT_INVOCATION.with(|c| c.get()),
                },
            );
            shared_timer.timer_ops_started.fetch_add(1, Ordering::Relaxed);
            let tx = WORKER_TX.with(|c| c.borrow().clone());
            let Some(tx) = tx else {
                return Err(rquickjs::Exception::throw_message(&ctx, "timer capability unavailable"));
            };
            tokio.spawn(async move {
                tokio::time::sleep(Duration::from_millis(ms)).await;
                // a failed send means the worker loop is gone; the promise never
                // settles, which shutdown drain already accounts for
                let _ = tx.send(WorkerMsg::TimerFired { op_id, result: Ok(ms) });
            });
            Ok(op_id as f64)
        };
        globals.set("__velquTimerStart", Function::new(ctx.clone(), f)?)?;
    }
    Ok(())
}
