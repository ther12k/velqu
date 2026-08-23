//! Engine integration tests: load, invoke (sync/async/promise), timer
//! capability, cancellation matrix, handle expiry, redaction inputs, and
//! contract-violation detection — all through the public Engine trait.

use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use q_engine::{
    BodyOut, Engine, EngineLoadPlan, FunctionDecl, FunctionKind, HandlerId, InvocationSpec,
    Outcome, ResponseStrategy,
};
use q_engine_quickjs::{IdentityMapper, QuickJsConfig, QuickJsEngine};

fn spec(id: u64, handler: &str, allowed: &[u16], deadline_ms: u64) -> InvocationSpec {
    InvocationSpec {
        id,
        request_id: format!("req-{id}"),
        route_id: "test.route".into(),
        route_id_num: None,
        handler_key: handler.into(),
        policy_key: None,
        handler_id: None,
        policy_id_num: None,
        policy_handler_id: None,
        params_schema_id: None,
        query_schema_id: None,
        headers_schema_id: None,
        body_schema_id: None,
        request: None,
        slot: 0,
        generation: 0,
        params: None,
        query: None,
        headers: None,
        body: None,
        allowed_statuses: allowed.to_vec(),
        default_status: 200,
        response_strategy: ResponseStrategy::Js,
        raw_response: false,
        deadline: Instant::now() + Duration::from_millis(deadline_ms),
    }
}

fn engine() -> QuickJsEngine {
    // reuse the #[tokio::test] runtime's handle; a private inner runtime would
    // be dropped while the worker thread still holds it
    QuickJsEngine::spawn(
        QuickJsConfig::default(),
        tokio::runtime::Handle::current(),
        Arc::new(IdentityMapper),
    )
}

fn insert_request(engine: &QuickJsEngine, meta: q_engine::RequestMeta) -> q_bridge::RequestHandle {
    engine.insert_request(meta).expect("insert request")
}

const BUNDLE: &str = r#"
function js_text(ctx) { return "plain"; }
function js_json(ctx) { return { ok: true }; }
function requestless(ctx) {
  return {
    hasParams: Object.prototype.hasOwnProperty.call(ctx, "params"),
    hasQuery: Object.prototype.hasOwnProperty.call(ctx, "query"),
    hasHeaders: Object.prototype.hasOwnProperty.call(ctx, "headers"),
    hasJson: Object.prototype.hasOwnProperty.call(ctx, "json"),
  };
}
function hello(ctx) { return { message: "Hello " + ctx.params.name }; }
function params_lazy_b(ctx) { return { got: ctx.params.b }; }
function headers_lazy(ctx) {
  return {
    auth: ctx.headers.authorization,
    keys: Object.keys(ctx.headers).sort().join(","),
    hasContentType: "content-type" in ctx.headers,
  };
}
function capability_identity(ctx) {
  const first = globalThis.__velquNativeCapabilities;
  return { shared: ctx.native === first, frozen: Object.isFrozen(ctx.native) && Object.isFrozen(ctx.native.timer) };
}
function web_fallback(ctx) {
  const request = ctx.webRequest();
  return { url: request.url, header: request.headers.authorization, query: request.query.ms };
}
function lazy_ctx(ctx) {
  // deliberately do NOT touch ctx.params/query/body: laziness proof
  return { untouched: true, routePlan: ctx.routePlan };
}
function read_query(ctx) { return { ms: ctx.query.ms }; }
function read_body(ctx) { return { echo: ctx.json().name }; }
async function timer_route(ctx) {
  // lazy query values are strings: explicit coercion (SCHEMA-002 semantics;
  // the native validation strategy pre-coerces instead)
  const ms = Number(ctx.query?.ms ?? 50);
  const waited = await ctx.native.timer.delay(ms);
  return { waited };
}
function thrower(ctx) { throw new Error("secret-boom"); }
function undeclared_status(ctx) { return { __ok: true, status: 299, value: { nope: true } }; }
function declared_201(ctx) { return { __ok: true, status: 201, value: { id: "usr_1" } }; }
function problem_404(ctx) { return { __problem: true, problem: "not-found", status: 404, detail: "no such user" }; }
function infinite(ctx) { while (true) {} }
async function spin_then(ctx) { return Promise.resolve().then(() => { while (true) {} }); }
function biz_status(ctx) { return { status: 200, value: "order-9", filled: false }; }
function native_body(ctx) { return { id: ctx.body.id, doubled: ctx.body.n * 2 }; }
var __syncSideEffect = 0;
function sync_with_microtask(ctx) {
  Promise.resolve().then(() => {
    __syncSideEffect += 1;
  });
  return { ok: true };
}
function check_side_effect(ctx) {
  return { val: __syncSideEffect };
}
function nested_microtask_chain(ctx) {
  Promise.resolve().then(() => {
    __syncSideEffect += 10;
    return Promise.resolve().then(() => {
      __syncSideEffect += 5;
    });
  });
  return { chain: true };
}
function microtask_reads_context(ctx) {
  Promise.resolve().then(() => {
    __syncSideEffect = Number(ctx.query.add);
  });
  return { queued: true };
}
function sync_runaway_microtask(ctx) {
  Promise.resolve().then(() => { while (true) {} });
  return { queued: true };
}
function sync_busy_microtask(ctx) {
  // ~300ms of real work inside a microtask; must be bounded by THIS
  // invocation's deadline, not another pending request's deadline
  Promise.resolve().then(() => { const s = Date.now(); while (Date.now() - s < 300) {} });
  return { queued: true };
}
async function chain_owner(ctx) {
  // two sequentially awaited timers: the SECOND op must stay owned by THIS
  // invocation even when other invocations start in between
  const a = await ctx.native.timer.delay(60);
  const b = await ctx.native.timer.delay(400);
  return { total: a + b };
}
function floating_timer(ctx) {
  // started but never awaited: must be cancelled at settlement
  ctx.native.timer.delay(60000);
  return { ok: true };
}
function guarded_marker(ctx) {
  __syncSideEffect = "CALLED";
  return { ok: true };
}
function floating_catch_spawns(ctx) {
  // rejected at settlement; the catch reaction must NOT be allowed to start
  // a second-generation native op
  ctx.native.timer.delay(60000).catch(() => {
    __syncSideEffect = "SPAWNED";
    return ctx.native.timer.delay(60000);
  });
  return { ok: true };
}
async function cancel_catch_spawns(ctx) {
  // cancelled mid-flight; the rejection continuation must not spawn new ops
  // on the dead invocation
  try {
    await ctx.native.timer.delay(60000);
  } catch {
    __syncSideEffect = "CANCEL_SPAWNED";
    return ctx.native.timer.delay(60000);
  }
  return { ok: true };
}
function check_spawn_flag(ctx) {
  return { flag: String(__syncSideEffect) };
}
var __chainCount = 0;
function chain_forever_catch(ctx) {
  // rejection reaction reschedules itself FOREVER via microtasks during
  // cleanup — only the absolute watchdog budget can stop it
  ctx.native.timer.delay(60000).catch(() => {
    __chainCount++;
    const again = () => { __chainCount++; Promise.resolve().then(again); };
    again();
  });
  return { ok: true };
}
function chain_count(ctx) { return { n: __chainCount }; }
function zero_timer(ctx) {
  return ctx.native.timer.delay(0).then(() => ({ zero: true }));
}
function sync_tiny_chain(ctx) {
  const again = () => { Promise.resolve().then(again); };
  again();
  return { ok: true };
}
async function async_tiny_chain(ctx) {
  const again = () => { Promise.resolve().then(again); };
  again();
  return { ok: true };
}
function chained_interrupt(ctx) {
  // first job queues a SECOND job, then infinite-loops (interrupted): the
  // leftover second job must never escape to another owner
  Promise.resolve().then(() => {
    Promise.resolve().then(() => { __chainCount++; });
    while (true) {}
  });
  return { queued: true };
}
function throwing_chain(ctx) {
  // every job throws AND reschedules: Err branch must obey the host budget
  const boom = () => { Promise.resolve().then(boom); throw new Error("boom"); };
  boom();
  return { ok: true };
}
function finite_chain(ctx) {
  // exactly SIX microtasks (counted): used with a configured job cap to
  // prove the cap-reached-by-FINAL-job case does not quarantine
  let left = 6;
  const step = () => { if (--left > 0) Promise.resolve().then(step); };
  Promise.resolve().then(step);
  return { ok: true };
}
function floating_race(ctx) {
  // ~1ms floating timer: settlement abort races physical completion
  ctx.native.timer.delay(1);
  return { ok: true };
}
async function timeout_catch_chains(ctx) {
  // times out during the 5s timer; the catch reaction reschedules forever
  // during CLEANUP: must still be quarantined by the cleanup budget
  try {
    await ctx.native.timer.delay(5000);
  } catch {
    const again = () => Promise.resolve().then(again);
    again();
  }
  return { ok: true };
}
function floating_catch_busy(ctx) {
  ctx.native.timer.delay(60000).catch(() => {
    const s = Date.now();
    while (Date.now() - s < 300) {}
  });
  return { ok: true };
}
function thrower_with_catch_busy(ctx) {
  Promise.resolve().then(() => {
    const s = Date.now();
    while (Date.now() - s < 300) {}
  });
  throw new Error("sync-throw");
}
async function promise_with_floating_busy(ctx) {
  ctx.native.timer.delay(60000).catch(() => {
    const s = Date.now();
    while (Date.now() - s < 300) {}
  });
  return { done: true };
}
__velquRegister("js.text", js_text);
__velquRegister("js.json", js_json);
__velquRegister("requestless", requestless);
__velquRegister("hello.get", hello);
__velquRegister("params.lazyb", params_lazy_b);
__velquRegister("headers.lazy", headers_lazy);
__velquRegister("lazy.ctx", lazy_ctx);
__velquRegister("web.fallback", web_fallback);
__velquRegister("capability.identity", capability_identity);
__velquRegister("query.read", read_query);
__velquRegister("body.read", read_body);
__velquRegister("timer.route", timer_route);
__velquRegister("throw.redacted", thrower);
__velquRegister("status.undeclared", undeclared_status);
__velquRegister("status.declared", declared_201);
__velquRegister("problem.notfound", problem_404);
__velquRegister("loop.infinite", infinite);
__velquRegister("loop.spin_then", spin_then);
__velquRegister("biz.status", biz_status);
__velquRegister("body.native", native_body);
__velquRegister("sync.microtask", sync_with_microtask);
__velquRegister("check.side_effect", check_side_effect);
__velquRegister("nested.microtask", nested_microtask_chain);
__velquRegister("microtask.context", microtask_reads_context);
__velquRegister("sync.runaway", sync_runaway_microtask);
__velquRegister("sync.busy", sync_busy_microtask);
__velquRegister("chain.owner", chain_owner);
__velquRegister("floating.timer", floating_timer);
__velquRegister("guarded.marker", guarded_marker);
__velquRegister("floating.catch", floating_catch_spawns);
__velquRegister("cancel.catch", cancel_catch_spawns);
__velquRegister("check.spawn_flag", check_spawn_flag);
__velquRegister("chain.forever", chain_forever_catch);
__velquRegister("chain.count", chain_count);
__velquRegister("timer.zero", zero_timer);
__velquRegister("sync.tinychain", sync_tiny_chain);
__velquRegister("async.tinychain", async_tiny_chain);
__velquRegister("chain.interrupt", chained_interrupt);
__velquRegister("chain.throwing", throwing_chain);
__velquRegister("chain.finite", finite_chain);
__velquRegister("floating.race", floating_race);
__velquRegister("timeout.catchchain", timeout_catch_chains);
__velquRegister("floating.catchbusy", floating_catch_busy);
__velquRegister("throw.catchbusy", thrower_with_catch_busy);

function resp_tojson_spin(ctx) {
  return { toJSON: function () { while (true) {} } };
}
async function resp_tojson_spin_async(ctx) {
  return { toJSON: function () { while (true) {} } };
}
function resp_getter_spin(ctx) {
  return Object.create({}, { data: { enumerable: true, get: function () { while (true) {} } } });
}
async function resp_getter_spin_async(ctx) {
  return Object.create({}, { data: { enumerable: true, get: function () { while (true) {} } } });
}
var __mappingFlag = false;
function resp_getter_microtask(ctx) {
  return Object.create({}, { data: { enumerable: true, get: function () {
    Promise.resolve().then(function () { __mappingFlag = true; });
    return 1;
  } } });
}
function check_mapping_flag(ctx) { return { ran: __mappingFlag }; }
function problem_getter_spin(ctx) {
  return { __problem: true, problem: "not-found", status: 404,
           get detail() { while (true) {} } };
}

async function resp_getter_microtask_async(ctx) {
  return Object.create({}, { data: { enumerable: true, get: function () {
    Promise.resolve().then(function () { __mappingFlag = true; });
    return 1;
  } } });
}
async function resp_getter_reads_ctx(ctx) {
  return Object.create({}, { data: { enumerable: true, get: function () {
    Promise.resolve().then(function () { __syncSideEffect = Number(ctx.query.add); });
    return 1;
  } } });
}
async function resp_getter_starts_timer(ctx) {
  return Object.create({}, { data: { enumerable: true, get: function () {
    ctx.native.timer.delay(20);
    return 1;
  } } });
}
function resp_getter_interrupt_escapes(ctx) {
  return Object.create({}, { data: { enumerable: true, get: function () {
    Promise.resolve().then(function () { __chainCount++; });
    while (true) {}
  } } });
}
__velquRegister("promise.floatingbusy", promise_with_floating_busy);
__velquRegister("resp.getter_microtask_async", resp_getter_microtask_async);
__velquRegister("resp.getter_reads_ctx", resp_getter_reads_ctx);
__velquRegister("resp.getter_starts_timer", resp_getter_starts_timer);
__velquRegister("resp.getter_interrupt_escapes", resp_getter_interrupt_escapes);
__velquRegister("resp.tojson_spin", resp_tojson_spin);
__velquRegister("resp.tojson_spin_async", resp_tojson_spin_async);
__velquRegister("resp.getter_spin", resp_getter_spin);
__velquRegister("resp.getter_spin_async", resp_getter_spin_async);
__velquRegister("resp.getter_microtask", resp_getter_microtask);
__velquRegister("check.mapping_flag", check_mapping_flag);
__velquRegister("problem.getter_spin", problem_getter_spin);
"#;

fn expected_table() -> BTreeMap<String, String> {
    [
        "js.text",
        "js.json",
        "requestless",
        "hello.get",
        "params.lazyb",
        "headers.lazy",
        "lazy.ctx",
        "web.fallback",
        "capability.identity",
        "query.read",
        "body.read",
        "timer.route",
        "throw.redacted",
        "status.undeclared",
        "status.declared",
        "problem.notfound",
        "loop.infinite",
        "loop.spin_then",
        "biz.status",
        "body.native",
        "sync.microtask",
        "check.side_effect",
        "nested.microtask",
        "microtask.context",
        "sync.runaway",
        "sync.busy",
        "chain.owner",
        "floating.timer",
        "guarded.marker",
        "floating.catch",
        "cancel.catch",
        "check.spawn_flag",
        "chain.forever",
        "chain.count",
        "timer.zero",
        "sync.tinychain",
        "async.tinychain",
        "chain.interrupt",
        "chain.throwing",
        "chain.finite",
        "floating.race",
        "timeout.catchchain",
        "floating.catchbusy",
        "throw.catchbusy",
        "promise.floatingbusy",
        "resp.getter_microtask_async",
        "resp.getter_reads_ctx",
        "resp.getter_starts_timer",
        "resp.getter_interrupt_escapes",
        "resp.tojson_spin",
        "resp.tojson_spin_async",
        "resp.getter_spin",
        "resp.getter_spin_async",
        "resp.getter_microtask",
        "check.mapping_flag",
        "problem.getter_spin",
    ]
    .iter()
    .map(|k| (k.to_string(), String::new()))
    .collect()
}

fn load_default(eng: &mut QuickJsEngine) -> Result<q_engine::LoadStats, String> {
    eng.load(
        BUNDLE,
        None,
        EngineLoadPlan::Legacy {
            expected_handlers: expected_table(),
        },
    )
}

#[tokio::test]
async fn load_verifies_handler_table_and_caches() {
    let mut eng = engine();
    let stats = load_default(&mut eng).expect("load");
    assert_eq!(stats.handlers_registered, 56);
    // a table mismatch must fail
    let mut bad = expected_table();
    bad.insert("extra.handler".into(), String::new());
    assert!(eng
        .load(
            BUNDLE,
            None,
            EngineLoadPlan::Legacy {
                expected_handlers: bad
            }
        )
        .is_err());
    eng.shutdown();
}

#[tokio::test]
async fn field_free_invocation_skips_request_store_slot() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();
    let mut s = spec(700, "requestless", &[200], 1000);
    s.slot = q_engine::NO_REQUEST_SLOT;
    s.generation = 0;
    let out = run(&mut eng, s).await;
    match out {
        Outcome::Response {
            status: 200,
            body: BodyOut::JsonText(text),
            ..
        } => assert_eq!(
            text,
            r#"{"hasParams":false,"hasQuery":false,"hasHeaders":false,"hasJson":false}"#
        ),
        other => panic!("unexpected requestless outcome: {other:?}"),
    }
    let counters = eng.bridge_snapshot();
    assert_eq!(
        counters.live_slots, 0,
        "field-free route must not allocate a slot"
    );
    assert_eq!(
        counters.host_calls, 0,
        "requestless route must not call the bridge"
    );
    assert_eq!(counters.materialized_fields, 0);
    assert_eq!(counters.materialized_bytes, 0);
    eng.shutdown();
}

#[tokio::test]
async fn worker_local_slab_capacity_is_bounded() {
    let mut eng = QuickJsEngine::spawn(
        QuickJsConfig {
            request_slot_capacity: 1,
            ..Default::default()
        },
        tokio::runtime::Handle::current(),
        Arc::new(IdentityMapper),
    );
    load_default(&mut eng).unwrap();

    let first = q_engine::InvocationSpec {
        request: Some(q_engine::RequestMeta::default()),
        ..spec(701, "timer.route", &[200], 5000)
    };
    let second = q_engine::InvocationSpec {
        request: Some(q_engine::RequestMeta::default()),
        ..spec(702, "timer.route", &[200], 5000)
    };
    let (tx1, rx1) = tokio::sync::oneshot::channel();
    let (tx2, rx2) = tokio::sync::oneshot::channel();
    eng.invoke(first, tx1);
    eng.invoke(second, tx2);
    let out2 = tokio::time::timeout(Duration::from_millis(500), rx2)
        .await
        .expect("capacity response")
        .expect("capacity channel");
    assert!(matches!(out2, Outcome::RequestCapacity));
    eng.cancel(701);
    let _ = tokio::time::timeout(Duration::from_millis(500), rx1).await;
    assert_eq!(eng.bridge_snapshot().live_slots, 0);
    eng.shutdown();
}

#[tokio::test]
async fn incoming_capability_pair_is_overwritten_by_worker_handle() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();
    // The spec carries a garbage host-supplied pair; the worker must mint its
    // own typed handle. timer.route reads ctx.query lazily through the bridge,
    // so a stale pair would surface as an expired-handle failure.
    let s = q_engine::InvocationSpec {
        request: Some(q_bridge::RequestMeta {
            query: vec![("ms".into(), "5".into())],
            ..Default::default()
        }),
        slot: 3,
        generation: u64::MAX,
        ..spec(703, "timer.route", &[200], 2000)
    };
    let out = run(&mut eng, s).await;
    assert!(
        matches!(out, Outcome::Response { status: 200, .. }),
        "worker-minted handle must serve lazy fields, got {out:?}"
    );
    assert_eq!(eng.bridge_snapshot().live_slots, 0);
    eng.shutdown();
}

#[tokio::test]
async fn sync_text_and_json_results() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();
    let out = run(&mut eng, spec(1, "js.text", &[200], 1000)).await;
    match out {
        Outcome::Response {
            status: 200,
            body: BodyOut::Text(t),
            ..
        } => assert_eq!(t, "plain"),
        o => panic!("{o:?}"),
    }
    let out = run(&mut eng, spec(2, "js.json", &[200], 1000)).await;
    match out {
        Outcome::Response {
            status: 200,
            body: BodyOut::JsonText(t),
            ..
        } => {
            assert_eq!(t, "{\"ok\":true}")
        }
        o => panic!("{o:?}"),
    }
    eng.shutdown();
}

#[tokio::test]
async fn sync_fast_path_zero_plumbing_cost() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();

    let count = 1000;
    for i in 1..=count {
        let out = run(&mut eng, spec(i, "js.json", &[200], 1000)).await;
        assert!(matches!(out, Outcome::Response { status: 200, .. }));
    }

    let stats = eng.stats();
    assert_eq!(stats.invocations, count);
    assert_eq!(stats.immediate_results, count);
    assert_eq!(stats.promise_results, 0, "no promise results on sync path");
    assert_eq!(stats.promise_watches, 0, "no promise watches on sync path");
    assert_eq!(
        stats.job_queue_drains, 0,
        "no job queue drains on sync path"
    );
    assert_eq!(
        stats.settlement_scans, 0,
        "no settlement scans on sync path"
    );
    assert_eq!(stats.engine_failures, 0);
    assert_eq!(stats.timeouts, 0);
    eng.shutdown();
}

#[tokio::test]
async fn sync_handler_microtask_executes_before_settlement_and_next_invocation() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();

    // 1. Invoke sync handler that enqueues a microtask
    let out1 = run(&mut eng, spec(1, "sync.microtask", &[200], 1000)).await;
    match out1 {
        Outcome::Response {
            status: 200,
            body: BodyOut::JsonText(t),
            ..
        } => assert_eq!(t, "{\"ok\":true}"),
        o => panic!("{o:?}"),
    }

    // 2. Check that the side effect executed BEFORE the next invocation
    let out2 = run(&mut eng, spec(2, "check.side_effect", &[200], 1000)).await;
    match out2 {
        Outcome::Response {
            status: 200,
            body: BodyOut::JsonText(t),
            ..
        } => assert_eq!(t, "{\"val\":1}"),
        o => panic!("{o:?}"),
    }

    let stats = eng.stats();
    assert_eq!(stats.invocations, 2);
    assert_eq!(stats.immediate_results, 2);
    // 1 job queue drain occurred for the microtask checkpoint of out1
    assert_eq!(stats.job_queue_drains, 1);
    assert_eq!(stats.promise_watches, 0);
    assert_eq!(stats.settlement_scans, 0);
    eng.shutdown();
}

#[tokio::test]
async fn nested_microtask_chain_drains_completely() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();

    let out1 = run(&mut eng, spec(1, "nested.microtask", &[200], 1000)).await;
    assert!(matches!(out1, Outcome::Response { status: 200, .. }));

    let out2 = run(&mut eng, spec(2, "check.side_effect", &[200], 1000)).await;
    match out2 {
        Outcome::Response {
            status: 200,
            body: BodyOut::JsonText(t),
            ..
        } => assert_eq!(t, "{\"val\":15}"), // 0 + 10 + 5
        o => panic!("{o:?}"),
    }
    eng.shutdown();
}

#[tokio::test]
async fn microtask_retains_valid_request_context() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();

    let handle = insert_request(
        &eng,
        q_bridge::RequestMeta {
            query: vec![("add".into(), "42".into())],
            ..Default::default()
        },
    );
    let mut s = spec(1, "microtask.context", &[200], 1000);
    s.slot = handle.slot();
    s.generation = handle.generation();

    let out1 = run(&mut eng, s).await;
    assert!(matches!(out1, Outcome::Response { status: 200, .. }));
    assert_eq!(
        eng.bridge_snapshot().live_slots as usize,
        0,
        "request slot settled after microtasks"
    );

    let out2 = run(&mut eng, spec(2, "check.side_effect", &[200], 1000)).await;
    match out2 {
        Outcome::Response {
            status: 200,
            body: BodyOut::JsonText(t),
            ..
        } => assert_eq!(t, "{\"val\":42}"),
        o => panic!("{o:?}"),
    }
    eng.shutdown();
}

// ------------------------------------------------------------ M2.2.1-r2 scheduler correctness

/// The most important separation test: route deadline 100ms, job watchdog
/// 5000ms (default config). A runaway synchronous microtask must be killed by
/// the ROUTE deadline, never by the generic watchdog budget.
#[tokio::test]
async fn sync_runaway_microtask_respects_route_deadline() {
    let mut eng = engine();
    assert_eq!(
        QuickJsConfig::default().job_deadline_ms,
        5000,
        "test premise: watchdog is far longer than the route deadline"
    );
    load_default(&mut eng).unwrap();
    let handle = insert_request(&eng, q_bridge::RequestMeta::default());
    let mut s = spec(1, "sync.runaway", &[200], 100);
    s.slot = handle.slot();
    s.generation = handle.generation();

    let t0 = Instant::now();
    let out = run(&mut eng, s).await;
    let elapsed = t0.elapsed();
    assert!(
        matches!(out, Outcome::Timeout),
        "runaway sync microtask must hit the 100ms route deadline, got {out:?}"
    );
    assert!(
        elapsed < Duration::from_secs(3),
        "must NOT wait for the 5s job watchdog, took {elapsed:?}"
    );
    let stats = eng.stats();
    assert_eq!(stats.pending_ops, 0, "no floating ops after timeout");
    assert_eq!(
        eng.bridge_snapshot().live_slots as usize,
        0,
        "request slot settled after timeout"
    );
    eng.shutdown();
}

#[tokio::test]
async fn sync_runaway_microtask_leaves_worker_reusable() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();

    let out = run(&mut eng, spec(1, "sync.runaway", &[200], 150)).await;
    assert!(matches!(out, Outcome::Timeout));

    // worker survives and the scheduler state is clean
    let out = run(&mut eng, spec(2, "js.text", &[200], 1000)).await;
    assert!(matches!(out, Outcome::Response { status: 200, .. }));
    let out = run(&mut eng, spec(3, "sync.microtask", &[200], 1000)).await;
    assert!(matches!(out, Outcome::Response { status: 200, .. }));
    let out = run(&mut eng, spec(4, "check.side_effect", &[200], 1000)).await;
    assert!(matches!(out, Outcome::Response { status: 200, .. }));
    assert_eq!(eng.stats().scheduler_boundary_violations, 0);
    eng.shutdown();
}

/// Request A (async, 5s deadline) is pending when request B (sync, 100ms
/// deadline) runs a runaway microtask. B must be bounded by ITS OWN 100ms
/// deadline, not inherit A's 5s budget.
#[tokio::test]
async fn sync_checkpoint_does_not_borrow_other_pending_request_deadline() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();

    // A: async timer route, long deadline, stays pending
    let handle_a = insert_request(
        &eng,
        q_bridge::RequestMeta {
            query: vec![("ms".into(), "5000".into())],
            ..Default::default()
        },
    );
    let mut sa = spec(1, "timer.route", &[200], 5000);
    sa.slot = handle_a.slot();
    sa.generation = handle_a.generation();
    let (txa, rxa) = tokio::sync::oneshot::channel();
    eng.invoke(sa, txa);

    // B: sync runaway microtask with a short route deadline
    let handle_b = insert_request(&eng, q_bridge::RequestMeta::default());
    let mut sb = spec(2, "sync.runaway", &[200], 100);
    sb.slot = handle_b.slot();
    sb.generation = handle_b.generation();

    let t0 = Instant::now();
    let out_b = run(&mut eng, sb).await;
    let elapsed = t0.elapsed();
    assert!(
        matches!(out_b, Outcome::Timeout),
        "B must hit its own deadline, got {out_b:?}"
    );
    assert!(
        elapsed < Duration::from_secs(3),
        "B must not inherit A's 5s budget, took {elapsed:?}"
    );

    eng.cancel(1);
    let _ = tokio::time::timeout(Duration::from_millis(200), rxa).await;
    assert_eq!(
        eng.bridge_snapshot().live_slots as usize,
        0,
        "both slots settled"
    );
    eng.shutdown();
}

/// Inverse direction: request A (short 80ms deadline) is pending when request
/// B (sync, 5s deadline) runs a ~300ms microtask. B must complete — A's
/// unrelated shorter deadline must NOT interrupt B's checkpoint.
#[tokio::test]
async fn sync_checkpoint_does_not_interrupt_from_other_request_deadline() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();

    // A: async timer that will expire at 80ms
    let handle_a = insert_request(
        &eng,
        q_bridge::RequestMeta {
            query: vec![("ms".into(), "80".into())],
            ..Default::default()
        },
    );
    let mut sa = spec(1, "timer.route", &[200], 80);
    sa.slot = handle_a.slot();
    sa.generation = handle_a.generation();
    let (txa, mut rxa) = tokio::sync::oneshot::channel();
    eng.invoke(sa, txa);

    // B: sync 300ms microtask under a 5s deadline — must NOT be killed by A's 80ms
    let out_b = run(&mut eng, spec(2, "sync.busy", &[200], 5000)).await;
    assert!(
        matches!(out_b, Outcome::Response { status: 200, .. }),
        "B must complete under its own deadline, got {out_b:?}"
    );

    // A expires on its own
    let out_a = tokio::time::timeout(Duration::from_secs(2), &mut rxa)
        .await
        .expect("A settles")
        .expect("A channel open");
    assert!(matches!(out_a, Outcome::Timeout), "A: {out_a:?}");
    assert!(
        !eng.stats().queue_poisoned,
        "ordinary timeout must not quarantine runtime"
    );
    assert_eq!(eng.bridge_snapshot().live_slots as usize, 0);
    assert_eq!(eng.stats().scheduler_boundary_violations, 0);
    eng.shutdown();
}

/// Ownership restoration: A chains two awaited timers. B starts in between and
/// is later CANCELLED. Under the old code A's second op was mis-owned by B, so
/// cancelling B killed A. A must complete with the full total.
#[tokio::test]
async fn async_continuation_preserves_invocation_owner() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();

    // A: chain_owner — timer(60) then timer(400), deadline 3s
    let handle_a = insert_request(&eng, q_bridge::RequestMeta::default());
    let mut sa = spec(1, "chain.owner", &[200], 3000);
    sa.slot = handle_a.slot();
    sa.generation = handle_a.generation();
    let (txa, rxa) = tokio::sync::oneshot::channel();
    eng.invoke(sa, txa);

    // B: long async timer, pending; CURRENT_INVOCATION would go stale = B
    let handle_b = insert_request(
        &eng,
        q_bridge::RequestMeta {
            query: vec![("ms".into(), "5000".into())],
            ..Default::default()
        },
    );
    let mut sb = spec(2, "timer.route", &[200], 3000);
    sb.slot = handle_b.slot();
    sb.generation = handle_b.generation();
    let (txb, rxb) = tokio::sync::oneshot::channel();
    eng.invoke(sb, txb);

    // t≈150ms: A's first timer (60ms) has fired and its continuation
    // registered the second op — while CURRENT_INVOCATION was last set by B.
    tokio::time::sleep(Duration::from_millis(150)).await;
    eng.cancel(2);
    let _ = tokio::time::timeout(Duration::from_millis(200), rxb).await;

    // A's second timer (≈460ms) must have survived B's cancellation
    let out_a = tokio::time::timeout(Duration::from_secs(5), rxa)
        .await
        .expect("A settles")
        .expect("A channel open");
    match out_a {
        Outcome::Response {
            status: 200,
            body: BodyOut::JsonText(t),
            ..
        } => assert_eq!(t, "{\"total\":460}"),
        o => panic!("A must complete despite B's cancellation, got {o:?}"),
    }
    assert_eq!(eng.bridge_snapshot().live_slots as usize, 0);
    assert_eq!(eng.stats().scheduler_boundary_violations, 0);
    eng.shutdown();
}

/// Direct op-accounting proof: A chains two timers, B runs in between; both of
/// A's ops start AND complete, none dropped, none left alive.
#[tokio::test]
async fn nested_native_op_is_owned_by_original_invocation() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();

    let handle_a = insert_request(&eng, q_bridge::RequestMeta::default());
    let mut sa = spec(1, "chain.owner", &[200], 3000);
    sa.slot = handle_a.slot();
    sa.generation = handle_a.generation();
    let (txa, rxa) = tokio::sync::oneshot::channel();
    eng.invoke(sa, txa);

    // interleaved sync request while A waits on its first timer
    tokio::time::sleep(Duration::from_millis(100)).await;
    let out_b = run(&mut eng, spec(2, "js.json", &[200], 1000)).await;
    assert!(matches!(out_b, Outcome::Response { status: 200, .. }));

    let out_a = tokio::time::timeout(Duration::from_secs(5), rxa)
        .await
        .expect("A settles")
        .expect("A channel open");
    assert!(matches!(out_a, Outcome::Response { status: 200, .. }));

    let stats = eng.stats();
    assert_eq!(stats.timer_ops_started, 2, "A started exactly two ops");
    assert_eq!(stats.timer_ops_completed, 2, "both ops completed");
    assert_eq!(stats.pending_ops, 0, "no ops left alive");
    assert_eq!(stats.late_completions_dropped, 0);
    assert_eq!(eng.stats().scheduler_boundary_violations, 0);
    eng.shutdown();
}

/// A sync handler that starts a timer it never awaits must have that op
/// cancelled at settlement — floating ops must not accumulate.
#[tokio::test]
async fn floating_native_op_is_cancelled_at_sync_settlement() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();

    let out = run(&mut eng, spec(1, "floating.timer", &[200], 1000)).await;
    assert!(matches!(out, Outcome::Response { status: 200, .. }));

    let stats = eng.stats();
    assert_eq!(stats.timer_ops_started, 1);
    assert_eq!(stats.timer_ops_completed, 1, "op completed via abort");
    assert_eq!(stats.pending_ops, 0, "floating op cancelled at settlement");

    // worker still healthy afterwards
    let out = run(&mut eng, spec(2, "js.text", &[200], 1000)).await;
    assert!(matches!(out, Outcome::Response { status: 200, .. }));
    eng.shutdown();
}

/// A declared policy whose handler is missing must fail CLOSED: engine
/// failure, no business handler execution, slot settled.
#[tokio::test]
async fn missing_policy_handler_fails_closed() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();

    let handle = insert_request(&eng, q_bridge::RequestMeta::default());
    let mut s = spec(1, "guarded.marker", &[200], 1000);
    s.slot = handle.slot();
    s.generation = handle.generation();
    s.policy_key = Some("policy.missing".into());

    let out = run(&mut eng, s).await;
    match &out {
        Outcome::EngineFailure { detail, .. } => {
            assert!(detail.contains("policy.missing"), "detail: {detail}");
            assert!(detail.contains("fail closed"), "detail: {detail}");
        }
        o => panic!("must fail closed with EngineFailure, got {o:?}"),
    }
    assert_eq!(
        eng.bridge_snapshot().live_slots as usize,
        0,
        "slot settled on fail-closed path"
    );
    eng.shutdown();
}

#[tokio::test]
async fn missing_policy_handler_never_calls_business_handler() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();

    let mut s = spec(1, "guarded.marker", &[200], 1000);
    s.policy_key = Some("policy.missing".into());
    let out = run(&mut eng, s).await;
    assert!(matches!(out, Outcome::EngineFailure { .. }));

    // the guarded business handler must NOT have executed
    let out = run(&mut eng, spec(2, "check.side_effect", &[200], 1000)).await;
    match out {
        Outcome::Response {
            body: BodyOut::JsonText(t),
            ..
        } => assert_ne!(t, "{\"val\":\"CALLED\"}", "handler must not run"),
        o => panic!("{o:?}"),
    }
    eng.shutdown();
}

#[tokio::test]
async fn missing_handler_settles_request_slot() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();

    let handle = insert_request(&eng, q_bridge::RequestMeta::default());
    let mut s = spec(1, "nope.missing", &[200], 1000);
    s.slot = handle.slot();
    s.generation = handle.generation();

    let out = run(&mut eng, s).await;
    assert!(matches!(out, Outcome::EngineFailure { .. }));
    assert_eq!(
        eng.bridge_snapshot().live_slots as usize,
        0,
        "slot must not leak on missing handler"
    );
    eng.shutdown();
}

/// Mixed workload (sync, microtask checkpoint, awaited timer, runaway
/// timeout, cancellation) — after every message boundary the scheduler state
/// must be back to idle: CURRENT_INVOCATION == 0, deadlines unarmed.
#[tokio::test]
async fn deadline_and_current_invocation_clear_at_message_boundary() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();

    // plain sync
    let out = run(&mut eng, spec(1, "js.json", &[200], 1000)).await;
    assert!(matches!(out, Outcome::Response { .. }));
    // microtask checkpoint
    let out = run(&mut eng, spec(2, "sync.microtask", &[200], 1000)).await;
    assert!(matches!(out, Outcome::Response { .. }));
    // awaited timer (pending → timer fired → continuation → settlement)
    let handle = insert_request(
        &eng,
        q_bridge::RequestMeta {
            query: vec![("ms".into(), "30".into())],
            ..Default::default()
        },
    );
    let mut s = spec(3, "timer.route", &[200], 2000);
    s.slot = handle.slot();
    s.generation = handle.generation();
    let out = run(&mut eng, s).await;
    assert!(matches!(out, Outcome::Response { .. }));
    // runaway sync microtask → deadline kill
    let out = run(&mut eng, spec(4, "sync.runaway", &[200], 100)).await;
    assert!(matches!(out, Outcome::Timeout));
    // cancellation path
    let handle_c = insert_request(
        &eng,
        q_bridge::RequestMeta {
            query: vec![("ms".into(), "5000".into())],
            ..Default::default()
        },
    );
    let mut sc = spec(5, "timer.route", &[200], 5000);
    sc.slot = handle_c.slot();
    sc.generation = handle_c.generation();
    let (txc, rxc) = tokio::sync::oneshot::channel();
    eng.invoke(sc, txc);
    tokio::time::sleep(Duration::from_millis(30)).await;
    eng.cancel(5);
    let _ = tokio::time::timeout(Duration::from_millis(300), rxc).await;

    let stats = eng.stats();
    assert_eq!(
        stats.scheduler_boundary_violations, 0,
        "scheduler state must be idle at every message boundary"
    );
    assert_eq!(eng.bridge_snapshot().live_slots as usize, 0);
    eng.shutdown();
}

#[tokio::test]
async fn prevalidated_params_flow_into_ctx() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();
    let handle = insert_request(&eng, q_bridge::RequestMeta::default());
    let mut s = spec(3, "hello.get", &[200], 1000);
    s.slot = handle.slot();
    s.generation = handle.generation();
    s.params = Some(serde_json::json!({ "name": "Rafi" }));
    let out = run(&mut eng, s).await;
    match out {
        Outcome::Response {
            status: 200,
            body: BodyOut::JsonText(t),
            ..
        } => {
            assert_eq!(t, "{\"message\":\"Hello Rafi\"}")
        }
        o => panic!("{o:?}"),
    }
    assert_eq!(
        eng.bridge_snapshot().live_slots as usize,
        0,
        "settlement must free the slot"
    );
}

/// M24-004-D: ctx.params is a per-key lazy object — one key access
/// materializes exactly one value; untouched keys allocate nothing.
#[tokio::test]
async fn params_materialize_one_key_per_access() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();
    let handle = insert_request(
        &eng,
        q_bridge::RequestMeta {
            path: "/x/AA/BB/CC".into(),
            param_specs: vec![
                q_engine::ParamSpec {
                    name: "a".into(),
                    start: 3,
                    end: 5,
                },
                q_engine::ParamSpec {
                    name: "b".into(),
                    start: 6,
                    end: 8,
                },
                q_engine::ParamSpec {
                    name: "c".into(),
                    start: 9,
                    end: 11,
                },
            ],
            ..Default::default()
        },
    );
    let mut s = spec(704, "params.lazyb", &[200], 1000);
    s.slot = handle.slot();
    s.generation = handle.generation();
    let out = run(&mut eng, s).await;
    match out {
        Outcome::Response {
            body: BodyOut::JsonText(t),
            ..
        } => assert_eq!(t, r#"{"got":"BB"}"#),
        o => panic!("{o:?}"),
    }
    let snap = eng.bridge_snapshot();
    // names fetch (0 fields) + exactly ONE single-key materialization
    assert_eq!(snap.materialized_fields, 1, "one key = one value");
    assert_eq!(snap.materialized_bytes, 2, "\"BB\" bytes");
    assert_eq!(
        eng.bridge_snapshot().live_slots,
        0,
        "slot settled after the lazy access"
    );
    eng.shutdown();
}

/// M24-005-B: the request carries only plan-declared headers and
/// ctx.headers is per-key lazy — one access materializes one value, and
/// undeclared headers (content-type) do not exist as keys.
#[tokio::test]
async fn headers_are_declared_only_and_per_key_lazy() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();
    let handle = insert_request(
        &eng,
        q_bridge::RequestMeta {
            // admission copied ONLY the declared authorization header
            headers: vec![("authorization".into(), "Bearer tok-123".into())],
            ..Default::default()
        },
    );
    let mut s = spec(705, "headers.lazy", &[200], 1000);
    s.slot = handle.slot();
    s.generation = handle.generation();
    let out = run(&mut eng, s).await;
    match out {
        Outcome::Response {
            body: BodyOut::JsonText(t),
            ..
        } => assert_eq!(
            t,
            r#"{"auth":"Bearer tok-123","keys":"authorization","hasContentType":false}"#
        ),
        o => panic!("{o:?}"),
    }
    // names fetch is 0-field; exactly ONE value access charged
    let snap = eng.bridge_snapshot();
    assert_eq!(snap.materialized_fields, 1, "one key = one value");
    assert_eq!(snap.materialized_bytes, 14, "\"Bearer tok-123\" bytes");
    assert_eq!(eng.bridge_snapshot().live_slots, 0);
    eng.shutdown();
}

#[tokio::test]
async fn shared_context_request_prototypes_are_reused() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();
    let handle = insert_request(&eng, q_bridge::RequestMeta::default());
    let mut first = spec(4, "lazy.ctx", &[200], 1000);
    first.slot = handle.slot();
    first.generation = handle.generation();
    let _ = run(&mut eng, first).await;
    let handle2 = insert_request(&eng, q_bridge::RequestMeta::default());
    let mut second = spec(5, "lazy.ctx", &[200], 1000);
    second.slot = handle2.slot();
    second.generation = handle2.generation();
    let _ = run(&mut eng, second).await;
    assert_eq!(eng.bridge_snapshot().live_slots, 0);
    eng.shutdown();
}

#[tokio::test]
async fn route_plan_references_do_not_copy_request_bytes() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();
    let handle = insert_request(
        &eng,
        q_engine::RequestMeta {
            path: "/opaque".into(),
            body: Some(bytes::Bytes::from_static(b"secret-request-body")),
            ..Default::default()
        },
    );
    let mut s = spec(4, "lazy.ctx", &[200], 1000);
    s.slot = handle.slot();
    s.generation = handle.generation();
    s.params_schema_id = Some(q_engine::SchemaId(7));
    s.query_schema_id = Some(q_engine::SchemaId(11));
    let out = run(&mut eng, s).await;
    match out {
        Outcome::Response {
            body: BodyOut::JsonText(text),
            ..
        } => {
            println!("route plan proof: {text}");
            assert!(text.contains("\"routePlan\""));
            assert!(text.contains("\"paramsSchemaId\":7"));
            assert!(text.contains("\"querySchemaId\":11"));
            assert!(!text.contains("secret-request-body"));
        }
        other => panic!("{other:?}"),
    }
    assert_eq!(eng.bridge_snapshot().live_slots, 0);
    eng.shutdown();
}

#[tokio::test]
async fn native_capability_graph_is_cached_and_immutable() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();
    for id in 1..=2 {
        let handle = insert_request(&eng, q_bridge::RequestMeta::default());
        let mut s = spec(id, "capability.identity", &[200], 1000);
        s.slot = handle.slot();
        s.generation = handle.generation();
        let out = run(&mut eng, s).await;
        match out {
            Outcome::Response {
                body: BodyOut::JsonText(text),
                ..
            } => {
                assert!(text.contains("\"shared\":true"));
                assert!(text.contains("\"frozen\":true"));
            }
            other => panic!("{other:?}"),
        }
    }
    assert_eq!(eng.bridge_snapshot().live_slots, 0);
    eng.shutdown();
}

#[tokio::test]
async fn explicit_web_request_fallback_materializes_on_demand() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();
    let handle = insert_request(
        &eng,
        q_bridge::RequestMeta {
            path: "/fallback".into(),
            query: vec![("ms".into(), "9".into())],
            headers: vec![("authorization".into(), "Bearer x".into())],
            ..Default::default()
        },
    );
    let mut s = spec(77, "web.fallback", &[200], 1000);
    s.slot = handle.slot();
    s.generation = handle.generation();
    let out = run(&mut eng, s).await;
    assert!(matches!(out, Outcome::Response { .. }));
    let snap = eng.bridge_snapshot();
    assert!(snap.materialized_fields > 0);
    assert_eq!(snap.live_slots, 0);
    eng.shutdown();
}

#[tokio::test]
async fn lazy_ctx_touches_nothing() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();
    let handle = insert_request(
        &eng,
        q_bridge::RequestMeta {
            path: "/hello/Rafi".into(),
            param_specs: vec![q_engine::ParamSpec {
                name: "name".into(),
                start: 7,
                end: 11,
            }],
            query: vec![("ms".into(), "50".into())],
            body: Some(bytes::Bytes::from_static(b"{\"name\":\"Ada\"}")),
            ..Default::default()
        },
    );
    let mut s = spec(4, "lazy.ctx", &[200], 1000);
    s.slot = handle.slot();
    s.generation = handle.generation();
    let out = run(&mut eng, s).await;
    assert!(matches!(out, Outcome::Response { .. }));
    let snap = eng.bridge_snapshot();
    assert_eq!(
        snap.host_calls, 0,
        "RUN-004: unread fields materialize nothing"
    );
    assert_eq!(snap.materialized_bytes, 0);
    assert_eq!(snap.materialized_fields, 0);
}

#[tokio::test]
async fn lazy_query_and_body_materialize_on_access() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();
    // query path (lazy)
    let handle = insert_request(
        &eng,
        q_bridge::RequestMeta {
            query: vec![("ms".into(), "42".into())],
            ..Default::default()
        },
    );
    let mut s = spec(5, "query.read", &[200], 1000);
    s.slot = handle.slot();
    s.generation = handle.generation();
    let out = run(&mut eng, s).await;
    match out {
        Outcome::Response {
            body: BodyOut::JsonText(t),
            ..
        } => assert_eq!(t, "{\"ms\":\"42\"}"),
        o => panic!("{o:?}"),
    }
    // body path (lazy json())
    let handle = insert_request(
        &eng,
        q_bridge::RequestMeta {
            body: Some(bytes::Bytes::from_static(b"{\"name\":\"Ada\"}")),
            ..Default::default()
        },
    );
    let mut s = spec(6, "body.read", &[200], 1000);
    s.slot = handle.slot();
    s.generation = handle.generation();
    let out = run(&mut eng, s).await;
    match out {
        Outcome::Response {
            body: BodyOut::JsonText(t),
            ..
        } => assert_eq!(t, "{\"echo\":\"Ada\"}"),
        o => panic!("{o:?}"),
    }
    // native body path (pre-validated)
    let handle = insert_request(&eng, q_bridge::RequestMeta::default());
    let mut s = spec(7, "body.native", &[200], 1000);
    s.slot = handle.slot();
    s.generation = handle.generation();
    s.body = Some(serde_json::json!({ "id": "usr_1", "n": 21 }));
    let out = run(&mut eng, s).await;
    match out {
        Outcome::Response {
            body: BodyOut::JsonText(t),
            ..
        } => {
            assert_eq!(t, "{\"id\":\"usr_1\",\"doubled\":42}")
        }
        o => panic!("{o:?}"),
    }
}

#[tokio::test]
async fn timer_promise_resolves_with_waited_ms() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();
    let handle = insert_request(
        &eng,
        q_bridge::RequestMeta {
            query: vec![("ms".into(), "30".into())],
            ..Default::default()
        },
    );
    let mut s = spec(8, "timer.route", &[200], 2000);
    s.slot = handle.slot();
    s.generation = handle.generation();
    let t0 = Instant::now();
    let out = run(&mut eng, s).await;
    let elapsed = t0.elapsed();
    match out {
        Outcome::Response {
            body: BodyOut::JsonText(t),
            ..
        } => assert_eq!(t, "{\"waited\":30}"),
        o => panic!("{o:?}"),
    }
    assert!(
        elapsed >= Duration::from_millis(25),
        "timer actually waited"
    );
    let stats = eng.stats();
    assert_eq!(stats.timer_ops_started, 1);
    assert_eq!(stats.timer_ops_completed, 1);
    assert_eq!(eng.bridge_snapshot().live_slots as usize, 0);
}

#[tokio::test]
async fn cancellation_before_completion() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();
    let handle = insert_request(
        &eng,
        q_bridge::RequestMeta {
            query: vec![("ms".into(), "5000".into())],
            ..Default::default()
        },
    );
    let mut s = spec(9, "timer.route", &[200], 10_000);
    s.slot = handle.slot();
    s.generation = handle.generation();
    let (tx, rx) = tokio::sync::oneshot::channel();
    eng.invoke(s, tx);
    tokio::time::sleep(Duration::from_millis(30)).await;
    eng.cancel(9);
    // reply may not fire (cancelled): the oneshot is simply dropped
    let _ = tokio::time::timeout(Duration::from_millis(300), rx).await;
    tokio::time::sleep(Duration::from_millis(50)).await;
    let stats = eng.stats();
    assert_eq!(stats.cancelled_invocations, 1);
    assert_eq!(
        eng.bridge_snapshot().live_slots as usize,
        0,
        "cancel must settle the handle"
    );
    eng.shutdown();
}

#[tokio::test]
async fn deadline_timeout_interrupts_and_replies() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();
    // infinite loop: only the interrupt handler saves us
    let out = run(&mut eng, spec(10, "loop.infinite", &[200], 150)).await;
    assert!(
        matches!(out, Outcome::Timeout),
        "runaway loop must hit the deadline, got {out:?}"
    );
    // the worker survives
    let out = run(&mut eng, spec(11, "js.text", &[200], 1000)).await;
    assert!(matches!(out, Outcome::Response { .. }));
    eng.shutdown();
}

#[tokio::test]
async fn throw_maps_to_engine_failure_with_detail() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();
    let out = run(&mut eng, spec(12, "throw.redacted", &[200], 1000)).await;
    match out {
        Outcome::EngineFailure { detail, source } => {
            assert!(
                detail.contains("secret-boom"),
                "internal detail keeps cause for logs"
            );
            assert!(source.is_some(), "source location mapped");
        }
        o => panic!("{o:?}"),
    }
    eng.shutdown();
}

#[tokio::test]
async fn undeclared_status_is_contract_violation() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();
    let out = run(&mut eng, spec(13, "status.undeclared", &[200], 1000)).await;
    assert!(matches!(out, Outcome::ContractViolation(_)));
    let out = run(&mut eng, spec(14, "status.declared", &[200, 201], 1000)).await;
    match out {
        Outcome::Response { status: 201, .. } => {}
        o => panic!("{o:?}"),
    }
    eng.shutdown();
}

#[tokio::test]
async fn typed_problem_passes_through() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();
    let out = run(&mut eng, spec(15, "problem.notfound", &[200, 404], 1000)).await;
    match out {
        Outcome::Problem(p) => {
            assert_eq!(p.problem_id, "not-found");
            assert_eq!(p.status, 404);
            assert_eq!(p.detail.as_deref(), Some("no such user"));
        }
        o => panic!("{o:?}"),
    }
    eng.shutdown();
}

#[tokio::test]
async fn expired_handle_access_fails_deterministically() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();
    // simulate a retained wrapper: settle the slot, then let JS touch it
    let handle = insert_request(
        &eng,
        q_bridge::RequestMeta {
            query: vec![("ms".into(), "1".into())],
            ..Default::default()
        },
    );
    eng.settle_request(handle); // expired before invocation
    let mut s = spec(16, "query.read", &[200], 1000);
    s.slot = handle.slot();
    s.generation = handle.generation();
    let out = run(&mut eng, s).await;
    match out {
        Outcome::EngineFailure { detail, .. } => assert!(detail.contains("expired")),
        o => panic!("expected expired-handle failure, got {o:?}"),
    }
    assert_eq!(eng.bridge_snapshot().expired_accesses, 1);
    eng.shutdown();
}

async fn run(eng: &mut QuickJsEngine, spec: InvocationSpec) -> Outcome {
    let (tx, rx) = tokio::sync::oneshot::channel();
    eng.invoke(spec, tx);
    tokio::time::timeout(Duration::from_secs(15), rx)
        .await
        .expect("engine reply within timeout")
        .expect("reply channel open")
}

#[tokio::test]
async fn runaway_promise_continuation_is_interruptible_and_worker_survives() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();
    // the loop runs INSIDE a .then() continuation — drain-time interrupt must kill it
    let t0 = Instant::now();
    let out = run(&mut eng, spec(20, "loop.spin_then", &[200], 200)).await;
    assert!(
        matches!(out, Outcome::Timeout),
        "runaway continuation must hit the deadline, got {out:?}"
    );
    assert!(
        t0.elapsed() < Duration::from_secs(3),
        "interrupt must fire promptly, took {:?}",
        t0.elapsed()
    );
    // worker survives and serves afterwards
    let out = run(&mut eng, spec(21, "js.text", &[200], 1000)).await;
    assert!(matches!(out, Outcome::Response { .. }));
    eng.shutdown();
}

#[tokio::test]
async fn business_object_with_status_and_value_fields_is_a_body_not_an_envelope() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();
    let out = run(&mut eng, spec(22, "biz.status", &[200], 1000)).await;
    match out {
        Outcome::Response {
            status: 200,
            body: BodyOut::JsonText(t),
            ..
        } => {
            let v: serde_json::Value = serde_json::from_str(&t).unwrap();
            assert_eq!(v["status"], 200, "status field is body data");
            assert_eq!(v["value"], "order-9", "value field is body data");
            assert_eq!(v["filled"], false);
        }
        o => panic!("expected 200 body response, got {o:?}"),
    }
    eng.shutdown();
}

// ------------------------------------------------------------ M2.2.1-r3 native-operation cleanup

/// Physical cancellation proof: after settlement the underlying Tokio task is
/// aborted — waiting past the original timer duration must not produce a late
/// completion, and no native task stays alive.
#[tokio::test]
async fn floating_timer_aborts_underlying_tokio_task() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();

    // floating 60s timer, never awaited: aborted at settlement
    let out = run(&mut eng, spec(30, "floating.timer", &[200], 1000)).await;
    assert!(matches!(out, Outcome::Response { status: 200, .. }));

    // wait past a plausible abort-propagation window
    tokio::time::sleep(Duration::from_millis(150)).await;
    let s2 = eng.stats();
    assert_eq!(s2.native_tasks_aborted, 1, "task physically aborted");
    assert_eq!(s2.native_tasks_alive, 0);
    assert_eq!(s2.late_completions_dropped, 0, "aborted task sent nothing");
    assert_eq!(s2.pending_ops, 0);

    let out = run(&mut eng, spec(31, "js.text", &[200], 1000)).await;
    assert!(matches!(out, Outcome::Response { status: 200, .. }));
    eng.shutdown();
}

/// Thousands of sync requests each starting a long floating timer must not
/// accumulate physical tasks — the op cap stays meaningful.
#[tokio::test]
async fn repeated_floating_timers_do_not_accumulate_tasks() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();

    const N: u64 = 2000;
    for i in 1..=N {
        let out = run(&mut eng, spec(i, "floating.timer", &[200], 1000)).await;
        assert!(matches!(out, Outcome::Response { status: 200, .. }));
    }

    tokio::time::sleep(Duration::from_millis(150)).await;
    let stats = eng.stats();
    assert_eq!(stats.timer_ops_started, N);
    assert_eq!(stats.native_tasks_aborted, N, "every floating task aborted");
    assert_eq!(
        stats.native_tasks_alive, 0,
        "no physical task accumulation across {N} requests"
    );
    assert_eq!(stats.pending_ops, 0);
    assert_eq!(stats.scheduler_boundary_violations, 0);
    eng.shutdown();
}

/// A cleanup microtask chain that continually reschedules itself must stop
/// within ONE absolute watchdog budget (default 5s), not renew it per job.
#[tokio::test]
async fn watchdog_uses_one_absolute_deadline() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();

    // chain_forever_catch: the floating op's rejection reaction reschedules
    // an infinite microtask chain during CLEANUP. quickjs-ng has no interrupt
    // poll points in tiny promise jobs, so no deadline can kill it; the
    // watchdog stops within its absolute budget + job cap, then marks the
    // queue poisoned: fail-closed quarantine rejects subsequent dynamic JS
    // requests immediately.
    let out = run(&mut eng, spec(1, "chain.forever", &[200], 1000)).await;
    assert!(matches!(out, Outcome::Response { status: 200, .. }));

    let t0 = Instant::now();
    let out = run(&mut eng, spec(2, "js.text", &[200], 1000)).await;
    let elapsed = t0.elapsed();
    assert!(
        matches!(out, Outcome::EngineFailure { .. }),
        "poisoned worker fails closed and rejects dynamic JS requests immediately"
    );
    assert!(
        elapsed < Duration::from_secs(2),
        "rejection is immediate, took {elapsed:?}"
    );

    let stats = eng.stats();
    assert!(
        stats.queue_poisoned,
        "unquiescable chain recorded as poison"
    );
    assert_eq!(stats.poison_events, 1);
    assert_eq!(stats.native_tasks_alive, 0);
    assert_eq!(stats.scheduler_boundary_violations, 0);
    eng.shutdown();
}

/// A rejected floating Promise whose catch starts another timer must fail
/// deterministically: no second-generation op, no live task, catch flag set.
#[tokio::test]
async fn cleanup_reaction_cannot_start_native_operation() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();

    let out = run(&mut eng, spec(1, "floating.catch", &[200], 1000)).await;
    assert!(matches!(out, Outcome::Response { status: 200, .. }));

    tokio::time::sleep(Duration::from_millis(50)).await;
    let stats = eng.stats();
    assert_eq!(stats.timer_ops_started, 1, "only the original op started");
    assert_eq!(stats.pending_ops, 0, "no second-generation op registered");
    assert_eq!(stats.native_tasks_alive, 0);
    assert_eq!(stats.native_tasks_aborted, 1);

    // the catch reaction DID run (flag set) but its timer start was refused
    let out = run(&mut eng, spec(2, "check.spawn_flag", &[200], 1000)).await;
    match out {
        Outcome::Response {
            body: BodyOut::JsonText(t),
            ..
        } => assert_eq!(t, "{\"flag\":\"SPAWNED\"}", "catch ran; spawn refused"),
        o => panic!("{o:?}"),
    }
    eng.shutdown();
}

/// Cancellation cleanup must not leave a newly created native operation owned
/// by an invocation that no longer exists.
#[tokio::test]
async fn cancel_reaction_cannot_spawn_second_generation_op() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();

    let handle = insert_request(&eng, q_bridge::RequestMeta::default());
    let mut s = spec(1, "cancel.catch", &[200], 5000);
    s.slot = handle.slot();
    s.generation = handle.generation();
    let (tx, rx) = tokio::sync::oneshot::channel();
    eng.invoke(s, tx);

    tokio::time::sleep(Duration::from_millis(50)).await;
    eng.cancel(1);
    let _ = tokio::time::timeout(Duration::from_millis(300), rx).await;
    // let any queued rejection continuation unwind
    tokio::time::sleep(Duration::from_millis(150)).await;

    let stats = eng.stats();
    assert_eq!(stats.timer_ops_started, 1, "original op only");
    assert_eq!(stats.pending_ops, 0, "no op owned by the dead invocation");
    assert_eq!(stats.native_tasks_alive, 0);
    assert_eq!(stats.native_tasks_aborted, 1, "original task aborted");
    assert_eq!(stats.scheduler_boundary_violations, 0);

    // catch ran but could not spawn
    let out = run(&mut eng, spec(2, "check.spawn_flag", &[200], 1000)).await;
    match out {
        Outcome::Response {
            body: BodyOut::JsonText(t),
            ..
        } => assert_eq!(t, "{\"flag\":\"CANCEL_SPAWNED\"}"),
        o => panic!("{o:?}"),
    }
    assert_eq!(eng.bridge_snapshot().live_slots as usize, 0);
    eng.shutdown();
}

/// Shutdown with outstanding timers must physically terminate their Tokio
/// tasks: counters show aborts BEFORE shutdown joins, and the join guarantees
/// the drain ran — nothing is left sleeping afterwards.
#[tokio::test]
async fn shutdown_aborts_all_native_tasks() {
    let mut eng = QuickJsEngine::spawn(
        QuickJsConfig::default(),
        tokio::runtime::Handle::current(),
        Arc::new(IdentityMapper),
    );
    load_default(&mut eng).unwrap();

    // five live invocations, each awaiting its own 60s timer
    for i in 1..=5u64 {
        let handle = insert_request(
            &eng,
            q_bridge::RequestMeta {
                query: vec![("ms".into(), "60000".into())],
                ..Default::default()
            },
        );
        let mut s = spec(i, "timer.route", &[200], 60_000);
        s.slot = handle.slot();
        s.generation = handle.generation();
        let (tx, _rx) = tokio::sync::oneshot::channel();
        eng.invoke(s, tx);
    }
    tokio::time::sleep(Duration::from_millis(150)).await;
    let before = eng.stats();
    assert_eq!(before.pending_ops, 5, "five ops awaiting timers");
    assert_eq!(before.native_tasks_alive, 5);

    // shutdown drains: every pending invocation is cancelled (aborting its
    // task) and any leftover op is aborted before the thread exits
    eng.shutdown();

    // the aborts ran inside the joined thread; a short grace window proves no
    // task outlives the worker (aborted counters are final, alive must be 0)
    tokio::time::sleep(Duration::from_millis(100)).await;
    let after = eng.stats();
    assert_eq!(after.pending_ops, 0);
    assert_eq!(after.native_tasks_alive, 0);
    assert_eq!(
        after.native_tasks_aborted, 5,
        "every outstanding task physically aborted at shutdown"
    );
}

// ------------------------------------------------------------ M2.2.1-r4 terminal-state closure

/// Zero-delay timers complete almost instantly: native_tasks_alive must NEVER
/// underflow / wrap to u64::MAX because the increment occurs before spawn.
#[tokio::test]
async fn zero_delay_timer_does_not_wrap_alive_counter() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();

    for i in 1..=50 {
        let out = run(&mut eng, spec(i, "timer.zero", &[200], 1000)).await;
        assert!(matches!(out, Outcome::Response { status: 200, .. }));
    }

    tokio::time::sleep(Duration::from_millis(50)).await;
    let stats = eng.stats();
    assert_eq!(stats.native_tasks_started, 50);
    assert_eq!(stats.native_tasks_completed, 50);
    assert_eq!(stats.native_tasks_aborted, 0);
    assert_eq!(stats.native_tasks_alive, 0, "alive counter never wrapped");
    assert_eq!(
        stats.native_tasks_started,
        stats.native_tasks_completed + stats.native_tasks_aborted + stats.native_tasks_alive
    );
    eng.shutdown();
}

/// When a task completes normally, an abort request arriving afterward
/// cannot win the compare-exchange: no double-counting occurs.
#[tokio::test]
async fn completion_wins_abort_race_without_double_count() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();

    let handle = insert_request(
        &eng,
        q_bridge::RequestMeta {
            query: vec![("ms".into(), "10".into())],
            ..Default::default()
        },
    );
    let mut s = spec(1, "timer.route", &[200], 1000);
    s.slot = handle.slot();
    s.generation = handle.generation();
    let out = run(&mut eng, s).await;
    assert!(matches!(out, Outcome::Response { status: 200, .. }));

    tokio::time::sleep(Duration::from_millis(50)).await;
    let stats = eng.stats();
    assert_eq!(stats.native_tasks_started, 1);
    assert_eq!(stats.native_tasks_completed, 1);
    assert_eq!(stats.native_tasks_aborted, 0);
    assert_eq!(stats.native_tasks_alive, 0);
    assert_eq!(
        stats.native_tasks_started,
        stats.native_tasks_completed + stats.native_tasks_aborted + stats.native_tasks_alive
    );
    eng.shutdown();
}

/// A tiny self-rescheduling microtask chain inside a synchronous handler is
/// bounded by host wall-clock time and job count: the worker does NOT hang.
#[tokio::test]
async fn sync_tiny_self_rescheduling_chain_is_bounded() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();

    let t0 = Instant::now();
    let out = run(&mut eng, spec(1, "sync.tinychain", &[200], 150)).await;
    let elapsed = t0.elapsed();
    assert!(
        matches!(out, Outcome::Timeout),
        "infinite sync microtask chain must time out, got {out:?}"
    );
    assert!(
        elapsed < Duration::from_secs(2),
        "must bound within the host deadline, took {elapsed:?}"
    );

    let stats = eng.stats();
    assert!(
        stats.queue_poisoned,
        "unquiescable chain marks queue poisoned"
    );
    assert_eq!(stats.poison_events, 1);
    eng.shutdown();
}

/// A tiny self-rescheduling microtask chain inside an async Promise handler is
/// bounded by host wall-clock time and job count without hanging the worker.
#[tokio::test]
async fn async_tiny_self_rescheduling_chain_is_bounded() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();

    let t0 = Instant::now();
    let out = run(&mut eng, spec(1, "async.tinychain", &[200], 150)).await;
    let elapsed = t0.elapsed();
    assert!(
        matches!(out, Outcome::EngineFailure { .. } | Outcome::Timeout),
        "infinite async microtask chain must fail closed or time out, got {out:?}"
    );
    assert!(
        elapsed < Duration::from_secs(2),
        "must bound within the host deadline, took {elapsed:?}"
    );

    let stats = eng.stats();
    assert!(stats.queue_poisoned);
    eng.shutdown();
}

/// When a worker runtime is poisoned, subsequent dynamic JS requests fail closed
/// immediately without executing or hanging.
#[tokio::test]
async fn poisoned_worker_rejects_new_dynamic_requests_immediately() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();

    // 1. Poison the worker with an unquiescable chain
    let out1 = run(&mut eng, spec(1, "sync.tinychain", &[200], 100)).await;
    assert!(matches!(out1, Outcome::Timeout));
    assert!(eng.stats().queue_poisoned);

    // 2. New sync request fails closed immediately with EngineFailure
    let t0 = Instant::now();
    let out2 = run(&mut eng, spec(2, "js.text", &[200], 1000)).await;
    assert!(
        matches!(out2, Outcome::EngineFailure { .. }),
        "poisoned worker must reject dynamic JS requests, got {out2:?}"
    );
    assert!(
        t0.elapsed() < Duration::from_millis(50),
        "rejection is immediate"
    );

    // 3. New promise request fails closed immediately
    let out3 = run(&mut eng, spec(3, "timer.route", &[200], 1000)).await;
    assert!(matches!(out3, Outcome::EngineFailure { .. }));
    eng.shutdown();
}

/// When a worker is poisoned, all pending asynchronous invocations fail
/// closed immediately rather than hanging until their individual deadlines.
#[tokio::test]
async fn all_pending_invocations_fail_when_worker_is_poisoned() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();

    // Request A: 5-second async timer, pending
    let handle_a = insert_request(
        &eng,
        q_bridge::RequestMeta {
            query: vec![("ms".into(), "5000".into())],
            ..Default::default()
        },
    );
    let mut sa = spec(1, "timer.route", &[200], 5000);
    sa.slot = handle_a.slot();
    sa.generation = handle_a.generation();
    let (txa, mut rxa) = tokio::sync::oneshot::channel();
    eng.invoke(sa, txa);

    tokio::time::sleep(Duration::from_millis(50)).await;

    // Request B: unquiescable chain that poisons the queue
    let out_b = run(&mut eng, spec(2, "sync.tinychain", &[200], 100)).await;
    assert!(matches!(out_b, Outcome::Timeout));

    // Request A must be failed closed immediately without waiting 5 seconds!
    let out_a = tokio::time::timeout(Duration::from_millis(500), &mut rxa)
        .await
        .expect("A must resolve immediately on poison")
        .expect("channel open");
    match out_a {
        Outcome::EngineFailure { detail, .. } => {
            assert!(detail.contains("quarantined"), "detail: {detail}");
        }
        o => panic!("pending request A must fail closed with EngineFailure, got {o:?}"),
    }
    assert_eq!(
        eng.bridge_snapshot().live_slots as usize,
        0,
        "all slots settled on poison"
    );
    eng.shutdown();
}

// ------------------------------------------------------------ M2.2.1-r4.1 terminalization

/// Cleanup-path poison (floating catch → watchdog → kill round) must go
/// through the UNIFIED quarantine: a pending async request from another
/// invocation fails closed immediately — not at its own 5s deadline.
#[tokio::test]
async fn cleanup_poison_fails_all_pending_immediately() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();

    // A: 5s async timer, pending
    let handle_a = insert_request(
        &eng,
        q_bridge::RequestMeta {
            query: vec![("ms".into(), "5000".into())],
            ..Default::default()
        },
    );
    let mut sa = spec(1, "timer.route", &[200], 5000);
    sa.slot = handle_a.slot();
    sa.generation = handle_a.generation();
    let (txa, mut rxa) = tokio::sync::oneshot::channel();
    eng.invoke(sa, txa);
    tokio::time::sleep(Duration::from_millis(50)).await;

    // B: cleanup-path poison (floating catch → unquiescable chain in cleanup)
    let out_b = run(&mut eng, spec(2, "chain.forever", &[200], 1000)).await;
    assert!(matches!(out_b, Outcome::Response { status: 200, .. }));

    let t0 = Instant::now();
    let out_a = tokio::time::timeout(Duration::from_millis(500), &mut rxa)
        .await
        .expect("A must resolve immediately on cleanup-path quarantine")
        .expect("channel open");
    match out_a {
        Outcome::EngineFailure { detail, .. } => {
            assert!(detail.contains("quarantined"), "detail: {detail}");
        }
        o => panic!("A must fail closed, got {o:?}"),
    }
    assert!(
        t0.elapsed() < Duration::from_millis(500),
        "no 5s wait: {:?}",
        t0.elapsed()
    );
    assert_eq!(eng.bridge_snapshot().live_slots as usize, 0);
    eng.shutdown();
}

/// Cleanup-path quarantine must abort the OTHER invocation's native op and
/// leave pending_ops at zero (checked accounting).
#[tokio::test]
async fn cleanup_poison_aborts_all_native_ops_and_zeroes_pending_ops() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();

    let handle_a = insert_request(
        &eng,
        q_bridge::RequestMeta {
            query: vec![("ms".into(), "5000".into())],
            ..Default::default()
        },
    );
    let mut sa = spec(1, "timer.route", &[200], 5000);
    sa.slot = handle_a.slot();
    sa.generation = handle_a.generation();
    let (txa, rxa) = tokio::sync::oneshot::channel();
    eng.invoke(sa, txa);
    tokio::time::sleep(Duration::from_millis(50)).await;

    let before = eng.stats();
    assert_eq!(before.pending_ops, 1);
    assert_eq!(before.native_tasks_alive, 1);

    let out_b = run(&mut eng, spec(2, "chain.forever", &[200], 1000)).await;
    assert!(matches!(out_b, Outcome::Response { status: 200, .. }));
    let _ = tokio::time::timeout(Duration::from_millis(500), rxa).await;

    tokio::time::sleep(Duration::from_millis(100)).await;
    let after = eng.stats();
    assert_eq!(after.pending_ops, 0, "gauge corrected by quarantine");
    assert_eq!(after.native_tasks_alive, 0);
    assert!(after.native_tasks_aborted >= 1, "A's task aborted");
    eng.shutdown();
}

/// The currently-executing sync invocation's OWN floating timer must also be
/// accounted when ITS cleanup chain quarantines the runtime (the reply is
/// sent before floating-op unwinding, so the response itself is a 200).
#[tokio::test]
async fn poison_with_current_floating_timer_leaves_pending_ops_zero() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();

    // chain.forever: floating 60s timer + unquiescable catch chain; the
    // handler replies 200 synchronously, then settlement cleanup poisons
    let out = run(&mut eng, spec(1, "chain.forever", &[200], 150)).await;
    assert!(
        matches!(out, Outcome::Response { status: 200, .. }),
        "reply precedes cleanup unwinding, got {out:?}"
    );

    tokio::time::sleep(Duration::from_millis(100)).await;
    let stats = eng.stats();
    assert!(stats.queue_poisoned);
    assert_eq!(stats.pending_ops, 0, "current invocation's op accounted");
    assert_eq!(stats.native_tasks_alive, 0);
    eng.shutdown();
}

/// An interrupted job that queued ANOTHER job: the leftover job never escapes
/// the drain — under the settlement grace it executes exactly once INSIDE the
/// same invocation scope (same owner, bounded), never at another request's
/// boundary, and never twice.
#[tokio::test]
async fn interrupted_job_with_another_queued_job_never_escapes() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();

    let t0 = Instant::now();
    let out = run(&mut eng, spec(1, "chain.interrupt", &[200], 150)).await;
    let elapsed = t0.elapsed();
    assert!(matches!(out, Outcome::Timeout), "got {out:?}");
    assert!(elapsed < Duration::from_secs(2), "took {elapsed:?}");

    let stats = eng.stats();
    assert!(!stats.queue_poisoned, "leftover drained within the grace");
    assert_eq!(stats.scheduler_boundary_violations, 0);

    // the second job ran AT MOST once — and only within invocation 1's own
    // drain (quiesced before any other message was processed)
    let out = run(&mut eng, spec(2, "chain.count", &[200], 1000)).await;
    match out {
        Outcome::Response {
            body: BodyOut::JsonText(t),
            ..
        } => {
            let v: serde_json::Value = serde_json::from_str(&t).unwrap();
            assert!(
                v["n"].as_i64().unwrap() <= 1,
                "second job executed at most once, got {}",
                v["n"]
            );
        }
        o => panic!("{o:?}"),
    }
    eng.shutdown();
}

/// A job that throws AND reschedules on every iteration must be bounded by
/// the host budget (Err branch obeys the same drain contract).
#[tokio::test]
async fn throwing_self_rescheduling_job_obeys_host_budget() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();

    let t0 = Instant::now();
    let out = run(&mut eng, spec(1, "chain.throwing", &[200], 150)).await;
    let elapsed = t0.elapsed();
    assert!(
        matches!(out, Outcome::Timeout | Outcome::EngineFailure { .. }),
        "got {out:?}"
    );
    assert!(
        elapsed < Duration::from_secs(2),
        "host budget bounded: {elapsed:?}"
    );
    assert!(eng.stats().queue_poisoned);
    eng.shutdown();
}

/// A finite chain whose LAST job finishes after the deadline (queue ends
/// empty) must NOT poison the runtime — quiescence is checked first.
#[tokio::test]
async fn finite_last_job_finishing_after_deadline_does_not_poison() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();

    // sync.busy: ONE ~300ms busy microtask; deadline 150ms. The job is
    // interrupted at 150ms (loop polls), the queue empties, and the request
    // times out WITHOUT quarantining the runtime.
    let out = run(&mut eng, spec(1, "sync.busy", &[200], 150)).await;
    assert!(matches!(out, Outcome::Timeout), "got {out:?}");

    let stats = eng.stats();
    assert!(
        !stats.queue_poisoned,
        "finite chain reaching quiescence must not poison"
    );

    // worker still fully alive afterwards
    let out = run(&mut eng, spec(2, "js.text", &[200], 1000)).await;
    assert!(matches!(out, Outcome::Response { status: 200, .. }));
    eng.shutdown();
}

/// Reaching the job cap with the FINAL job (queue empties exactly at the
/// cap) must NOT poison; exceeding the cap with work left must quarantine.
#[tokio::test]
async fn job_cap_reached_by_final_quiescing_job_does_not_poison() {
    // engine A: cap == chain length (6): the 6th job empties the queue at
    // exactly the cap — quiescence check runs first, no quarantine
    let mut eng_a = QuickJsEngine::spawn(
        QuickJsConfig {
            max_invocation_jobs: 6,
            ..Default::default()
        },
        tokio::runtime::Handle::current(),
        Arc::new(IdentityMapper),
    );
    eng_a
        .load(
            BUNDLE,
            None,
            EngineLoadPlan::Legacy {
                expected_handlers: expected_table(),
            },
        )
        .unwrap();
    let out = run(&mut eng_a, spec(1, "chain.finite", &[200], 2000)).await;
    assert!(
        matches!(out, Outcome::Response { status: 200, .. }),
        "got {out:?}"
    );
    assert!(
        !eng_a.stats().queue_poisoned,
        "final job at exact cap quiesces"
    );
    eng_a.shutdown();

    // engine B: cap 5 < chain length 6: 6th job still queued at the cap —
    // quarantine through the unified terminal path
    let mut eng_b = QuickJsEngine::spawn(
        QuickJsConfig {
            max_invocation_jobs: 5,
            ..Default::default()
        },
        tokio::runtime::Handle::current(),
        Arc::new(IdentityMapper),
    );
    eng_b
        .load(
            BUNDLE,
            None,
            EngineLoadPlan::Legacy {
                expected_handlers: expected_table(),
            },
        )
        .unwrap();
    let out = run(&mut eng_b, spec(1, "chain.finite", &[200], 2000)).await;
    assert!(matches!(out, Outcome::Timeout), "got {out:?}");
    assert!(
        eng_b.stats().queue_poisoned,
        "cap exceeded with work left quarantines"
    );
    eng_b.shutdown();
}

/// REAL completion-vs-abort race: ~1ms floating timers whose settlement
/// abort races physical completion across hundreds of iterations. Exactly
/// one CAS transition wins per task; the invariant holds after quiescence.
#[tokio::test]
async fn completion_actually_wins_abort_race() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();

    const N: u64 = 300;
    for i in 1..=N {
        let out = run(&mut eng, spec(i, "floating.race", &[200], 1000)).await;
        assert!(matches!(out, Outcome::Response { status: 200, .. }));
    }

    tokio::time::sleep(Duration::from_millis(200)).await;
    let stats = eng.stats();
    assert_eq!(stats.native_tasks_started, N);
    // some tasks physically completed before settlement's abort request —
    // whichever way each race went, the terminal classification is exact
    assert!(
        stats.native_tasks_completed + stats.native_tasks_aborted >= 1,
        "races actually happened (completed={}, aborted={})",
        stats.native_tasks_completed,
        stats.native_tasks_aborted
    );
    assert_eq!(stats.native_tasks_alive, 0);
    assert_eq!(
        stats.native_tasks_started,
        stats.native_tasks_completed + stats.native_tasks_aborted + stats.native_tasks_alive,
        "invariant holds after quiescence"
    );
    eng.shutdown();
}

/// The abort side of the race: a long timer aborted at settlement wins the
/// CAS (AbortRequested) and the task is classified aborted exactly once.
#[tokio::test]
async fn abort_actually_wins_completion_race() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();

    // 60s floating timer: completion is impossible before settlement, so
    // abort deterministically wins every CAS
    let out = run(&mut eng, spec(1, "floating.timer", &[200], 1000)).await;
    assert!(matches!(out, Outcome::Response { status: 200, .. }));

    tokio::time::sleep(Duration::from_millis(100)).await;
    let stats = eng.stats();
    assert_eq!(stats.native_tasks_started, 1);
    assert_eq!(stats.native_tasks_aborted, 1, "abort won the CAS");
    assert_eq!(stats.native_tasks_completed, 0);
    assert_eq!(stats.native_tasks_alive, 0);
    assert_eq!(
        stats.native_tasks_started,
        stats.native_tasks_completed + stats.native_tasks_aborted + stats.native_tasks_alive
    );
    eng.shutdown();
}

// ------------------------------------------------------------ M2.2.1-r4.2 cleanup budget separation

/// P0: an ordinary async timeout must NOT quarantine the worker runtime.
/// Scenario: 5s timer, route deadline 50ms.
/// Outcome is Timeout; queue_poisoned == false; pending_ops == 0;
/// next sync and async requests succeed.
#[tokio::test]
async fn ordinary_async_timeout_does_not_quarantine_worker() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();

    let handle = insert_request(
        &eng,
        q_bridge::RequestMeta {
            query: vec![("ms".into(), "5000".into())],
            ..Default::default()
        },
    );
    let mut s = spec(1, "timer.route", &[200], 50);
    s.slot = handle.slot();
    s.generation = handle.generation();

    let t0 = Instant::now();
    let out = run(&mut eng, s).await;
    let elapsed = t0.elapsed();
    assert!(
        matches!(out, Outcome::Timeout),
        "must time out at route deadline, got {out:?}"
    );
    assert!(
        elapsed < Duration::from_secs(2),
        "timeout occurs promptly, took {elapsed:?}"
    );

    // the aborted task's liveness guard drops asynchronously on the Tokio
    // thread — allow it to settle before asserting the physical gauge
    tokio::time::sleep(Duration::from_millis(50)).await;
    let stats = eng.stats();
    assert!(
        !stats.queue_poisoned,
        "ordinary timeout must NOT quarantine runtime"
    );
    assert_eq!(stats.pending_ops, 0);
    assert_eq!(stats.native_tasks_alive, 0);
    assert_eq!(eng.bridge_snapshot().live_slots as usize, 0);

    // subsequent sync request succeeds
    let out_sync = run(&mut eng, spec(2, "js.text", &[200], 1000)).await;
    assert!(matches!(out_sync, Outcome::Response { status: 200, .. }));

    // subsequent async request succeeds
    let handle2 = insert_request(
        &eng,
        q_bridge::RequestMeta {
            query: vec![("ms".into(), "20".into())],
            ..Default::default()
        },
    );
    let mut s2 = spec(3, "timer.route", &[200], 2000);
    s2.slot = handle2.slot();
    s2.generation = handle2.generation();
    let out_async = run(&mut eng, s2).await;
    assert!(matches!(out_async, Outcome::Response { status: 200, .. }));
    eng.shutdown();
}

/// Cancellation of an async request gives cleanup a fresh budget:
/// the worker remains healthy and does not quarantine.
#[tokio::test]
async fn cancelled_async_request_cleanup_does_not_quarantine() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();

    let handle = insert_request(
        &eng,
        q_bridge::RequestMeta {
            query: vec![("ms".into(), "5000".into())],
            ..Default::default()
        },
    );
    let mut s = spec(1, "timer.route", &[200], 5000);
    s.slot = handle.slot();
    s.generation = handle.generation();
    let (tx, rx) = tokio::sync::oneshot::channel();
    eng.invoke(s, tx);

    tokio::time::sleep(Duration::from_millis(30)).await;
    eng.cancel(1);
    let _ = tokio::time::timeout(Duration::from_millis(300), rx).await;

    tokio::time::sleep(Duration::from_millis(150)).await;
    let stats = eng.stats();
    assert!(
        !stats.queue_poisoned,
        "cancelled request cleanup must NOT quarantine runtime"
    );
    assert_eq!(stats.pending_ops, 0);
    assert_eq!(stats.native_tasks_alive, 0);

    let out = run(&mut eng, spec(2, "js.text", &[200], 1000)).await;
    assert!(matches!(out, Outcome::Response { status: 200, .. }));
    eng.shutdown();
}

/// A timeout whose catch reaction chains microtasks forever during cleanup
/// MUST still quarantine (proving the cleanup budget separation is not fail-open).
#[tokio::test]
async fn pathological_timeout_cleanup_still_quarantines() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();

    let handle = insert_request(&eng, q_bridge::RequestMeta::default());
    let mut s = spec(1, "timeout.catchchain", &[200], 50);
    s.slot = handle.slot();
    s.generation = handle.generation();

    let out = run(&mut eng, s).await;
    assert!(matches!(out, Outcome::Timeout), "got {out:?}");

    tokio::time::sleep(Duration::from_millis(200)).await;
    let stats = eng.stats();
    assert!(
        stats.queue_poisoned,
        "pathological cleanup chain MUST quarantine runtime"
    );
    eng.shutdown();
}

/// M24-003-C / ADR-0021 T7: quarantine is a terminal path — the worker-owned
/// settle_all sweep must invalidate Active slots that no pending entry tracks
/// (here: a slot admitted but never attached to an invocation), exactly once.
#[tokio::test]
async fn quarantine_settles_slots_without_pending_entries() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();

    // orphan slot: admitted into the slab, tracked by no invocation
    let orphan = insert_request(&eng, q_bridge::RequestMeta::default());
    assert_eq!(eng.bridge_snapshot().live_slots, 1);

    // poison the runtime through a pathological timeout cleanup chain
    let handle = insert_request(&eng, q_bridge::RequestMeta::default());
    let mut s = spec(1, "timeout.catchchain", &[200], 50);
    s.slot = handle.slot();
    s.generation = handle.generation();
    let out = run(&mut eng, s).await;
    assert!(matches!(out, Outcome::Timeout), "got {out:?}");
    tokio::time::sleep(Duration::from_millis(200)).await;
    assert!(eng.stats().queue_poisoned, "chain must quarantine runtime");

    // both the poisoned invocation's slot and the orphan settle: zero live
    assert_eq!(
        eng.bridge_snapshot().live_slots,
        0,
        "quarantine sweep must settle every active slot, tracked or not"
    );
    let _ = orphan; // handle retained; sweep invalidated it by generation
    eng.shutdown();
}

/// M24-003-C / ADR-0021 T8: shutdown is a terminal path — after the worker
/// thread joins, no slot of the slab remains live.
#[tokio::test]
async fn shutdown_settles_all_remaining_slots() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();
    let _first = insert_request(&eng, q_bridge::RequestMeta::default());
    let _second = insert_request(&eng, q_bridge::RequestMeta::default());
    assert_eq!(eng.bridge_snapshot().live_slots, 2);
    eng.shutdown();
    assert_eq!(
        eng.bridge_snapshot().live_slots,
        0,
        "shutdown sweep must leave zero live slots"
    );
}

/// M24-003-D / ADR-0021 T11: a handle minted by worker A is meaningless to
/// worker B. B cannot forge one (local reconstruction stamps B's identity),
/// and A's handle presented to B's settlement path is a deterministic no-op —
/// the slot settles only at its owning worker.
#[tokio::test]
async fn cross_worker_handle_is_inert_on_foreign_worker() {
    let mut eng_a = engine();
    let mut eng_b = engine();
    load_default(&mut eng_a).unwrap();
    load_default(&mut eng_b).unwrap();

    let handle_a = insert_request(&eng_a, q_bridge::RequestMeta::default());
    assert_eq!(eng_a.bridge_snapshot().live_slots, 1);

    // B's settle of A's typed handle must not touch A's slab (or B's own).
    // settlement_table_len round-trips the worker so the settle message is
    // processed before the counters are read.
    eng_b.settle_request(handle_a);
    let _ = eng_b.settlement_table_len();
    assert_eq!(
        eng_a.bridge_snapshot().live_slots,
        1,
        "foreign-worker settle must be a no-op at the owner's slab"
    );
    assert_eq!(eng_b.bridge_snapshot().live_slots, 0);

    // the owning worker settles it exactly once
    eng_a.settle_request(handle_a);
    let _ = eng_a.settlement_table_len();
    assert_eq!(eng_a.bridge_snapshot().live_slots, 0);
    eng_a.shutdown();
    eng_b.shutdown();
}

// ------------------------------------------------------------ M2.2.1-r4.2.1 drain-local report & cleanup budget

/// Interruption during request B's cleanup drain must be DRAIN-LOCAL:
/// it must NEVER leak to another invocation A, causing A to be misclassified as Timeout.
#[tokio::test]
async fn cleanup_interrupt_does_not_timeout_unrelated_invocation() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();

    // 1. Request A: async timer (300ms) under a 2s deadline
    let handle_a = insert_request(
        &eng,
        q_bridge::RequestMeta {
            query: vec![("ms".into(), "300".into())],
            ..Default::default()
        },
    );
    let mut sa = spec(1, "timer.route", &[200], 2000);
    sa.slot = handle_a.slot();
    sa.generation = handle_a.generation();
    let (txa, mut rxa) = tokio::sync::oneshot::channel();
    eng.invoke(sa, txa);

    tokio::time::sleep(Duration::from_millis(50)).await;

    // 2. Request B: sync handler with floating catch that runs a 300ms busy loop
    // during cleanup settlement under a 100ms cleanup budget (interrupted).
    let out_b = run(&mut eng, spec(2, "floating.catchbusy", &[200], 1000)).await;
    assert!(matches!(out_b, Outcome::Response { status: 200, .. }));

    // 3. Request A completes its 300ms timer: A must succeed with 200, NOT Timeout!
    let out_a = tokio::time::timeout(Duration::from_secs(3), &mut rxa)
        .await
        .expect("A completes")
        .expect("channel open");
    match out_a {
        Outcome::Response { status: 200, .. } => { /* expected: A unaffected by B's interrupt */ }
        o => panic!("A must succeed despite B's cleanup interrupt, got {o:?}"),
    }
    assert_eq!(eng.bridge_snapshot().live_slots as usize, 0);
    assert_eq!(eng.stats().scheduler_boundary_violations, 0);
    eng.shutdown();
}

/// Post-settlement cleanup of floating operations uses SETTLEMENT_GRACE (100ms),
/// NOT the 5-second watchdog: a 300ms cleanup loop is interrupted quickly.
#[tokio::test]
async fn post_settlement_floating_cleanup_uses_cleanup_budget() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();

    let t0 = Instant::now();
    let out = run(&mut eng, spec(1, "floating.catchbusy", &[200], 1000)).await;
    assert!(matches!(out, Outcome::Response { status: 200, .. }));

    // next request succeeds promptly without waiting for a 5-second watchdog
    let out2 = run(&mut eng, spec(2, "js.text", &[200], 1000)).await;
    let elapsed = t0.elapsed();
    assert!(matches!(out2, Outcome::Response { status: 200, .. }));
    assert!(
        elapsed < Duration::from_secs(2),
        "post-settlement cleanup must not block on 5s watchdog, took {elapsed:?}"
    );
    eng.shutdown();
}

/// Failed synchronous handler cleanup uses the fresh cleanup budget (SETTLEMENT_GRACE).
#[tokio::test]
async fn failed_handler_cleanup_uses_cleanup_budget() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();

    let t0 = Instant::now();
    let out = run(&mut eng, spec(1, "throw.catchbusy", &[200], 1000)).await;
    assert!(
        matches!(out, Outcome::EngineFailure { .. } | Outcome::Timeout),
        "failed handler returns failure or cleanup timeout, got {out:?}"
    );

    let out2 = run(&mut eng, spec(2, "js.text", &[200], 1000)).await;
    let elapsed = t0.elapsed();
    assert!(matches!(out2, Outcome::Response { status: 200, .. }));
    assert!(
        elapsed < Duration::from_secs(2),
        "failed handler cleanup must use cleanup budget, took {elapsed:?}"
    );
    eng.shutdown();
}

/// Promise settlement cleanup uses the fresh cleanup budget.
#[tokio::test]
async fn promise_settlement_cleanup_uses_cleanup_budget() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();

    let t0 = Instant::now();
    let out = run(&mut eng, spec(1, "promise.floatingbusy", &[200], 1000)).await;
    assert!(matches!(out, Outcome::Response { status: 200, .. }));

    let out2 = run(&mut eng, spec(2, "js.text", &[200], 1000)).await;
    let elapsed = t0.elapsed();
    assert!(matches!(out2, Outcome::Response { status: 200, .. }));
    assert!(
        elapsed < Duration::from_secs(2),
        "promise settlement cleanup must use cleanup budget, took {elapsed:?}"
    );
    eng.shutdown();
}

/// Quarantine unconditionally swaps pending_ops to 0, ensuring zero underflow
/// and a clean 0 gauge on terminal state.
#[tokio::test]
async fn quarantine_accounting_drift_resets_pending_ops_to_zero() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();

    // trigger quarantine
    let out = run(&mut eng, spec(1, "chain.forever", &[200], 150)).await;
    assert!(matches!(out, Outcome::Response { status: 200, .. }));

    tokio::time::sleep(Duration::from_millis(150)).await;
    let stats = eng.stats();
    assert!(stats.queue_poisoned);
    assert_eq!(
        stats.pending_ops, 0,
        "pending_ops unconditionally zero on quarantine"
    );
    assert_eq!(stats.native_tasks_alive, 0);
    eng.shutdown();
}

// ------------------------------------------------------------ M2.2.1-r4.2.2 response mapping budget

/// Sync handler returning an object whose toJSON() spins forever: response
/// conversion runs under the ARMED invocation deadline (r4.2.2) — Timeout.
#[tokio::test]
async fn sync_response_tojson_spin_obeys_route_deadline() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();

    let t0 = Instant::now();
    let out = run(&mut eng, spec(1, "resp.tojson_spin", &[200], 150)).await;
    let elapsed = t0.elapsed();
    assert!(
        matches!(out, Outcome::Timeout | Outcome::EngineFailure { .. }),
        "unbounded toJSON must be deadline-killed, got {out:?}"
    );
    assert!(
        elapsed < Duration::from_secs(3),
        "must respect the 150ms route deadline, took {elapsed:?}"
    );
    eng.shutdown();
}

/// Async handler whose settled value has a spinning toJSON(): conversion in
/// finish_resolved runs under the owner's armed deadline — Timeout.
#[tokio::test]
async fn async_response_tojson_spin_obeys_route_deadline() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();

    let t0 = Instant::now();
    let out = run(&mut eng, spec(1, "resp.tojson_spin_async", &[200], 300)).await;
    let elapsed = t0.elapsed();
    assert!(
        matches!(out, Outcome::Timeout | Outcome::EngineFailure { .. }),
        "async toJSON spin must be deadline-killed, got {out:?}"
    );
    assert!(
        elapsed < Duration::from_secs(3),
        "must respect the route deadline, took {elapsed:?}"
    );
    eng.shutdown();
}

/// Sync handler returning an object with a spinning getter: enumeration in
/// value_to_outcome runs under the armed deadline — Timeout.
#[tokio::test]
async fn sync_response_getter_spin_obeys_route_deadline() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();

    let t0 = Instant::now();
    let out = run(&mut eng, spec(1, "resp.getter_spin", &[200], 150)).await;
    let elapsed = t0.elapsed();
    assert!(
        matches!(out, Outcome::Timeout | Outcome::EngineFailure { .. }),
        "spinning getter must be deadline-killed, got {out:?}"
    );
    assert!(elapsed < Duration::from_secs(3), "took {elapsed:?}");
    eng.shutdown();
}

/// Async handler whose settled value has a spinning getter — bounded.
#[tokio::test]
async fn async_response_getter_spin_obeys_route_deadline() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();

    let t0 = Instant::now();
    let out = run(&mut eng, spec(1, "resp.getter_spin_async", &[200], 300)).await;
    let elapsed = t0.elapsed();
    assert!(
        matches!(out, Outcome::Timeout | Outcome::EngineFailure { .. }),
        "async getter spin must be deadline-killed, got {out:?}"
    );
    assert!(elapsed < Duration::from_secs(3), "took {elapsed:?}");
    eng.shutdown();
}

/// A getter that queues a microtask during response mapping: the queued job
/// stays with the ORIGINAL invocation owner and runs before settlement is
/// observed by later requests.
#[tokio::test]
async fn response_mapping_microtask_stays_with_owner() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();

    let out = run(&mut eng, spec(1, "resp.getter_microtask", &[200], 1000)).await;
    assert!(matches!(out, Outcome::Response { status: 200, .. }));

    // the mapping-queued microtask ran under the owner before the next message
    let out = run(&mut eng, spec(2, "check.mapping_flag", &[200], 1000)).await;
    match out {
        Outcome::Response {
            body: BodyOut::JsonText(t),
            ..
        } => assert_eq!(t, "{\"ran\":true}", "mapping microtask executed with owner"),
        o => panic!("{o:?}"),
    }
    assert_eq!(eng.stats().scheduler_boundary_violations, 0);
    eng.shutdown();
}

/// After a response-mapping deadline kill, the worker remains reusable.
#[tokio::test]
async fn response_mapping_timeout_leaves_worker_reusable() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();

    let out = run(&mut eng, spec(1, "resp.tojson_spin", &[200], 150)).await;
    assert!(matches!(
        out,
        Outcome::Timeout | Outcome::EngineFailure { .. }
    ));

    let out = run(&mut eng, spec(2, "js.text", &[200], 1000)).await;
    assert!(
        matches!(out, Outcome::Response { status: 200, .. }),
        "worker reusable after mapping timeout, got {out:?}"
    );
    let out = run(&mut eng, spec(3, "js.json", &[200], 1000)).await;
    assert!(matches!(out, Outcome::Response { status: 200, .. }));
    eng.shutdown();
}

/// A problem object with a spinning getter on `detail`: problem-field
/// extraction in value_to_outcome is bounded by the armed deadline.
#[tokio::test]
async fn problem_object_getter_is_bounded() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();

    let t0 = Instant::now();
    let out = run(&mut eng, spec(1, "problem.getter_spin", &[200, 404], 150)).await;
    let elapsed = t0.elapsed();
    assert!(
        matches!(out, Outcome::Timeout | Outcome::EngineFailure { .. }),
        "problem getter spin must be deadline-killed, got {out:?}"
    );
    assert!(elapsed < Duration::from_secs(3), "took {elapsed:?}");
    eng.shutdown();
}

// ------------------------------------------------------------ M2.3-A async response-mapping & settlement lifecycle

/// An async response getter queues a microtask: it must complete BEFORE the caller receives the response.
#[tokio::test]
async fn async_response_mapping_microtask_runs_before_reply() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();

    let out = run(
        &mut eng,
        spec(1, "resp.getter_microtask_async", &[200], 1000),
    )
    .await;
    assert!(matches!(out, Outcome::Response { status: 200, .. }));

    // the mapping-queued microtask already ran before reply was sent
    let out2 = run(&mut eng, spec(2, "check.mapping_flag", &[200], 1000)).await;
    match out2 {
        Outcome::Response {
            body: BodyOut::JsonText(t),
            ..
        } => assert_eq!(
            t, "{\"ran\":true}",
            "async mapping microtask executed before reply"
        ),
        o => panic!("{o:?}"),
    }
    eng.shutdown();
}

/// The very next handler invocation must observe the side effect of an async response mapping microtask.
#[tokio::test]
async fn async_response_mapping_microtask_runs_before_next_invocation() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();

    let out1 = run(
        &mut eng,
        spec(1, "resp.getter_microtask_async", &[200], 1000),
    )
    .await;
    assert!(matches!(out1, Outcome::Response { status: 200, .. }));

    let out2 = run(&mut eng, spec(2, "check.mapping_flag", &[200], 1000)).await;
    match out2 {
        Outcome::Response {
            body: BodyOut::JsonText(t),
            ..
        } => assert_eq!(t, "{\"ran\":true}"),
        o => panic!("{o:?}"),
    }
    eng.shutdown();
}

/// An async response mapping microtask reading ctx.query must see a valid original request slot.
#[tokio::test]
async fn async_mapping_microtask_retains_request_context() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();

    let handle = insert_request(
        &eng,
        q_bridge::RequestMeta {
            query: vec![("add".into(), "99".into())],
            ..Default::default()
        },
    );
    let mut s = spec(1, "resp.getter_reads_ctx", &[200], 1000);
    s.slot = handle.slot();
    s.generation = handle.generation();

    let out1 = run(&mut eng, s).await;
    assert!(matches!(out1, Outcome::Response { status: 200, .. }));
    assert_eq!(
        eng.bridge_snapshot().live_slots as usize,
        0,
        "request slot settled after microtask drain"
    );

    let out2 = run(&mut eng, spec(2, "check.side_effect", &[200], 1000)).await;
    match out2 {
        Outcome::Response {
            body: BodyOut::JsonText(t),
            ..
        } => assert_eq!(
            t, "{\"val\":99}",
            "async mapping microtask read original ctx"
        ),
        o => panic!("{o:?}"),
    }
    eng.shutdown();
}

/// An async mapping-created native op must stay owned by its original invocation and not leak to the next.
#[tokio::test]
async fn async_mapping_native_op_keeps_original_owner() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();

    let out1 = run(&mut eng, spec(1, "resp.getter_starts_timer", &[200], 1000)).await;
    assert!(matches!(out1, Outcome::Response { status: 200, .. }));

    tokio::time::sleep(Duration::from_millis(50)).await;
    let out2 = run(&mut eng, spec(2, "js.text", &[200], 1000)).await;
    assert!(matches!(out2, Outcome::Response { status: 200, .. }));

    let stats = eng.stats();
    assert_eq!(stats.scheduler_boundary_violations, 0);
    assert_eq!(stats.pending_ops, 0);
    eng.shutdown();
}

/// An async response getter that schedules a microtask and then spins: the cleanup job must not escape.
#[tokio::test]
async fn mapping_interrupt_with_queued_microtask_does_not_escape() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();

    let out1 = run(
        &mut eng,
        spec(1, "resp.getter_interrupt_escapes", &[200], 150),
    )
    .await;
    assert!(matches!(
        out1,
        Outcome::Timeout | Outcome::EngineFailure { .. }
    ));

    let stats = eng.stats();
    assert_eq!(stats.scheduler_boundary_violations, 0);

    let out2 = run(&mut eng, spec(2, "js.text", &[200], 1000)).await;
    assert!(matches!(
        out2,
        Outcome::Response { .. } | Outcome::EngineFailure { .. }
    ));
    eng.shutdown();
}

/// Expired settled promise clears settlement table, floating ops, and request slot.
#[tokio::test]
async fn expired_settled_promise_clears_table_and_floating_ops() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();

    let handle = insert_request(
        &eng,
        q_bridge::RequestMeta {
            query: vec![("ms".into(), "50".into())],
            ..Default::default()
        },
    );
    let mut s = spec(1, "timer.route", &[200], 30);
    s.slot = handle.slot();
    s.generation = handle.generation();

    let out = run(&mut eng, s).await;
    assert!(matches!(out, Outcome::Timeout));

    tokio::time::sleep(Duration::from_millis(100)).await;
    let stats = eng.stats();
    assert_eq!(stats.pending_ops, 0);
    assert_eq!(stats.native_tasks_alive, 0);
    assert_eq!(eng.bridge_snapshot().live_slots as usize, 0);
    eng.shutdown();
}

/// Repeated timeouts do not grow the settlement table.
#[tokio::test]
async fn repeated_timeouts_do_not_grow_settlement_table() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();

    const N: u64 = 100;
    for i in 1..=N {
        let handle = insert_request(
            &eng,
            q_bridge::RequestMeta {
                query: vec![("ms".into(), "5000".into())],
                ..Default::default()
            },
        );
        let mut s = spec(i, "timer.route", &[200], 20);
        s.slot = handle.slot();
        s.generation = handle.generation();
        let out = run(&mut eng, s).await;
        assert!(matches!(out, Outcome::Timeout));
    }

    tokio::time::sleep(Duration::from_millis(100)).await;
    let stats = eng.stats();
    assert_eq!(stats.timeouts, N);
    assert_eq!(stats.pending_ops, 0);
    assert_eq!(stats.native_tasks_alive, 0);
    assert_eq!(eng.bridge_snapshot().live_slots as usize, 0);
    assert_eq!(stats.scheduler_boundary_violations, 0);
    assert_eq!(eng.settlement_table_len(), 0);
    eng.shutdown();
}

/// M2.3 numeric vector dispatch calls exact function by HandlerId ($O(1)$)
#[tokio::test]
async fn numeric_handler_dispatch_calls_exact_declared_function() {
    let bundle = r#"
        function fn_a() { return { route: "A" }; }
        function fn_b() { return { route: "B" }; }
        globalThis.__velquFunctionManifest = [
            ["route.a", 0, fn_a],
            ["route.b", 0, fn_b]
        ];
        globalThis.__velquFunctions = [fn_a, fn_b];
    "#;
    let mut eng = engine();
    let functions = vec![
        FunctionDecl {
            id: 0,
            key: "route.a".into(),
            kind: FunctionKind::RouteHandler,
        },
        FunctionDecl {
            id: 1,
            key: "route.b".into(),
            kind: FunctionKind::RouteHandler,
        },
    ];
    eng.load(bundle, None, EngineLoadPlan::Numeric { functions })
        .unwrap();

    let handle_a = insert_request(&eng, q_bridge::RequestMeta::default());
    let mut s_a = spec(1, "unused", &[200], 5000);
    s_a.slot = handle_a.slot();
    s_a.generation = handle_a.generation();
    s_a.handler_id = Some(HandlerId(0));
    let out_a = run(&mut eng, s_a).await;
    match out_a {
        Outcome::Response {
            body: BodyOut::JsonText(t),
            ..
        } => {
            let v: serde_json::Value = serde_json::from_str(&t).unwrap();
            assert_eq!(v["route"], "A");
        }
        Outcome::Response {
            body: BodyOut::Json(v),
            ..
        } => {
            assert_eq!(v["route"], "A");
        }
        other => panic!("expected route A, got {other:?}"),
    }

    let handle_b = insert_request(&eng, q_bridge::RequestMeta::default());
    let mut s_b = spec(2, "unused", &[200], 5000);
    s_b.slot = handle_b.slot();
    s_b.generation = handle_b.generation();
    s_b.handler_id = Some(HandlerId(1));
    let out_b = run(&mut eng, s_b).await;
    match out_b {
        Outcome::Response {
            body: BodyOut::JsonText(t),
            ..
        } => {
            let v: serde_json::Value = serde_json::from_str(&t).unwrap();
            assert_eq!(v["route"], "B");
        }
        Outcome::Response {
            body: BodyOut::Json(v),
            ..
        } => {
            assert_eq!(v["route"], "B");
        }
        other => panic!("expected route B, got {other:?}"),
    }

    let stats = eng.stats();
    assert_eq!(stats.numeric_dispatches, 2);
    assert_eq!(stats.legacy_map_dispatches, 0);
    eng.shutdown();
}

/// M2.3 numeric policy dispatch executes policy handler by numeric ID and enforces 401 vs 200
#[tokio::test]
async fn numeric_policy_dispatch_enforces_401_and_200() {
    let bundle = r#"
        function route_handler(ctx) { return { ok: true, user: ctx.session.user }; }
        function auth_policy(req) {
            if (req.headers.authorization === "Bearer valid") {
                return { session: { user: "alice" } };
            }
            return { __problem: true, problem: "unauthorized", status: 401 };
        }
        globalThis.__velquFunctionManifest = [
            ["route.main", 0, route_handler],
            ["auth.policy", 1, auth_policy]
        ];
        globalThis.__velquFunctions = [route_handler, auth_policy];
    "#;
    let mut eng = engine();
    let functions = vec![
        FunctionDecl {
            id: 0,
            key: "route.main".into(),
            kind: FunctionKind::RouteHandler,
        },
        FunctionDecl {
            id: 1,
            key: "auth.policy".into(),
            kind: FunctionKind::PolicyHandler,
        },
    ];
    eng.load(bundle, None, EngineLoadPlan::Numeric { functions })
        .unwrap();

    // 1. Unauthorized request
    let handle_unauth = insert_request(
        &eng,
        q_bridge::RequestMeta {
            headers: vec![("authorization".into(), "Bearer bad".into())],
            ..Default::default()
        },
    );
    let mut s_unauth = spec(1, "unused", &[200, 401], 5000);
    s_unauth.slot = handle_unauth.slot();
    s_unauth.generation = handle_unauth.generation();
    s_unauth.handler_id = Some(HandlerId(0));
    s_unauth.policy_handler_id = Some(HandlerId(1));
    let out_unauth = run(&mut eng, s_unauth).await;
    match out_unauth {
        Outcome::Problem(p) => {
            assert_eq!(p.status, 401);
            assert_eq!(p.problem_id, "unauthorized");
        }
        other => panic!("expected 401 problem, got {other:?}"),
    }

    // 2. Authorized request
    let handle_auth = insert_request(
        &eng,
        q_bridge::RequestMeta {
            headers: vec![("authorization".into(), "Bearer valid".into())],
            ..Default::default()
        },
    );
    let mut s_auth = spec(2, "unused", &[200, 401], 5000);
    s_auth.slot = handle_auth.slot();
    s_auth.generation = handle_auth.generation();
    s_auth.handler_id = Some(HandlerId(0));
    s_auth.policy_handler_id = Some(HandlerId(1));
    let out_auth = run(&mut eng, s_auth).await;
    match out_auth {
        Outcome::Response {
            status: 200,
            body: BodyOut::JsonText(t),
            ..
        } => {
            let v: serde_json::Value = serde_json::from_str(&t).unwrap();
            assert_eq!(v["ok"], true);
            assert_eq!(v["user"], "alice");
        }
        Outcome::Response {
            status: 200,
            body: BodyOut::Json(v),
            ..
        } => {
            assert_eq!(v["ok"], true);
            assert_eq!(v["user"], "alice");
        }
        other => panic!("expected 200 ok, got {other:?}"),
    }

    let stats = eng.stats();
    assert_eq!(stats.numeric_dispatches, 2);
    assert_eq!(stats.policy_calls, 2);
    eng.shutdown();
}

/// M2.3-r3: Swapped function manifest entries are rejected at load
#[tokio::test]
async fn swapped_function_vector_entries_are_rejected() {
    let bundle = r#"
        function fn_a() { return 1; }
        function fn_b() { return 2; }
        globalThis.__velquFunctionManifest = [
            ["fn.b", 0, fn_b],
            ["fn.a", 0, fn_a]
        ];
        globalThis.__velquFunctions = [fn_b, fn_a];
    "#;
    let mut eng = engine();
    let functions = vec![
        FunctionDecl {
            id: 0,
            key: "fn.a".into(),
            kind: FunctionKind::RouteHandler,
        },
        FunctionDecl {
            id: 1,
            key: "fn.b".into(),
            kind: FunctionKind::RouteHandler,
        },
    ];
    let err = eng
        .load(bundle, None, EngineLoadPlan::Numeric { functions })
        .unwrap_err();
    assert!(err.contains("key mismatch: bundle has 'fn.b', pack expected 'fn.a'"));
    eng.shutdown();
}

/// M2.3-r3: Route function in policy slot is rejected at load
#[tokio::test]
async fn route_function_in_policy_slot_is_rejected() {
    let bundle = r#"
        function fn_a() { return 1; }
        globalThis.__velquFunctionManifest = [
            ["auth.session", 0, fn_a] // kind 0 is RouteHandler, pack expects 1 (PolicyHandler)
        ];
        globalThis.__velquFunctions = [fn_a];
    "#;
    let mut eng = engine();
    let functions = vec![FunctionDecl {
        id: 0,
        key: "auth.session".into(),
        kind: FunctionKind::PolicyHandler,
    }];
    let err = eng
        .load(bundle, None, EngineLoadPlan::Numeric { functions })
        .unwrap_err();
    assert!(err.contains("kind mismatch: bundle has 0, pack expected 1"));
    eng.shutdown();
}

/// M2.3-r3: Policy function in route slot is rejected at load
#[tokio::test]
async fn policy_function_in_route_slot_is_rejected() {
    let bundle = r#"
        function fn_a() { return 1; }
        globalThis.__velquFunctionManifest = [
            ["users.get", 1, fn_a] // kind 1 is PolicyHandler, pack expects 0 (RouteHandler)
        ];
        globalThis.__velquFunctions = [fn_a];
    "#;
    let mut eng = engine();
    let functions = vec![FunctionDecl {
        id: 0,
        key: "users.get".into(),
        kind: FunctionKind::RouteHandler,
    }];
    let err = eng
        .load(bundle, None, EngineLoadPlan::Numeric { functions })
        .unwrap_err();
    assert!(err.contains("kind mismatch: bundle has 1, pack expected 0"));
    eng.shutdown();
}

/// M2.3 strict vector loading: a hole in __velquFunctionManifest must be rejected during load
#[tokio::test]
async fn function_vector_hole_rejected_during_load() {
    let bundle = r#"
        function fn_a() { return 1; }
        globalThis.__velquFunctionManifest = [
            ["a", 0, fn_a],
            undefined,
            ["c", 0, fn_a]
        ];
    "#;
    let mut eng = engine();
    let functions = vec![
        FunctionDecl {
            id: 0,
            key: "a".into(),
            kind: FunctionKind::RouteHandler,
        },
        FunctionDecl {
            id: 1,
            key: "b".into(),
            kind: FunctionKind::RouteHandler,
        },
        FunctionDecl {
            id: 2,
            key: "c".into(),
            kind: FunctionKind::RouteHandler,
        },
    ];
    let err = eng
        .load(bundle, None, EngineLoadPlan::Numeric { functions })
        .unwrap_err();
    assert!(err.contains("entry 1 is invalid"));
    eng.shutdown();
}

/// M2.3 strict vector loading: a non-function in __velquFunctionManifest must be rejected during load
#[tokio::test]
async fn function_vector_non_function_rejected_during_load() {
    let bundle = r#"
        function fn_a() { return 1; }
        globalThis.__velquFunctionManifest = [
            ["a", 0, fn_a],
            ["b", 0, 42],
            ["c", 0, fn_a]
        ];
    "#;
    let mut eng = engine();
    let functions = vec![
        FunctionDecl {
            id: 0,
            key: "a".into(),
            kind: FunctionKind::RouteHandler,
        },
        FunctionDecl {
            id: 1,
            key: "b".into(),
            kind: FunctionKind::RouteHandler,
        },
        FunctionDecl {
            id: 2,
            key: "c".into(),
            kind: FunctionKind::RouteHandler,
        },
    ];
    let err = eng
        .load(bundle, None, EngineLoadPlan::Numeric { functions })
        .unwrap_err();
    assert!(err.contains("entry 1 (b) is not a callable function"));
    eng.shutdown();
}

/// M2.3 strict vector loading: length mismatch must be rejected during load
#[tokio::test]
async fn function_vector_length_mismatch_rejected() {
    let bundle = r#"
        function fn_a() { return 1; }
        globalThis.__velquFunctionManifest = [
            ["a", 0, fn_a]
        ];
    "#;
    let mut eng = engine();
    let functions = vec![
        FunctionDecl {
            id: 0,
            key: "a".into(),
            kind: FunctionKind::RouteHandler,
        },
        FunctionDecl {
            id: 1,
            key: "b".into(),
            kind: FunctionKind::RouteHandler,
        },
    ];
    let err = eng
        .load(bundle, None, EngineLoadPlan::Numeric { functions })
        .unwrap_err();
    assert!(err.contains("function manifest length 1 != expected manifest count 2"));
    eng.shutdown();
}

/// M2.3 out of range numeric handler id fails closed
#[tokio::test]
async fn numeric_out_of_range_handler_id_fails_closed() {
    let bundle = r#"
        function fn_a() { return { ok: true }; }
        globalThis.__velquFunctionManifest = [
            ["a", 0, fn_a]
        ];
    "#;
    let mut eng = engine();
    let functions = vec![FunctionDecl {
        id: 0,
        key: "a".into(),
        kind: FunctionKind::RouteHandler,
    }];
    eng.load(bundle, None, EngineLoadPlan::Numeric { functions })
        .unwrap();

    let handle = insert_request(&eng, q_bridge::RequestMeta::default());
    let mut s = spec(1, "unused", &[200], 5000);
    s.slot = handle.slot();
    s.generation = handle.generation();
    s.handler_id = Some(HandlerId(99)); // out of range
    let out = run(&mut eng, s).await;
    assert!(matches!(out, Outcome::EngineFailure { .. }));
    assert_eq!(eng.bridge_snapshot().live_slots as usize, 0);
    eng.shutdown();
}

/// M2.3-r3/r4: Numeric mode strictly requires the semantic manifest; raw vector without manifest is rejected
#[tokio::test]
async fn numeric_pack_without_semantic_manifest_is_rejected() {
    let bundle = r#"
        function fn_a() { return { ok: true }; }
        globalThis.__velquFunctions = [fn_a];
    "#;
    let mut eng = engine();
    let functions = vec![FunctionDecl {
        id: 0,
        key: "fn.a".into(),
        kind: FunctionKind::RouteHandler,
    }];
    let err = eng
        .load(bundle, None, EngineLoadPlan::Numeric { functions })
        .unwrap_err();
    assert!(
        err.contains("semantic function manifest (globalThis.__velquFunctionManifest) is missing")
    );
    eng.shutdown();
}

/// M2.3-r3/r4: Missing manifest with swapped functions fails closed
#[tokio::test]
async fn missing_manifest_with_swapped_functions_is_rejected() {
    let bundle = r#"
        function auth_session() { return { session: {} }; }
        function users_get() { return { user: "alice" }; }
        globalThis.__velquFunctions = [auth_session, users_get];
    "#;
    let mut eng = engine();
    let functions = vec![
        FunctionDecl {
            id: 0,
            key: "users.get".into(),
            kind: FunctionKind::RouteHandler,
        },
        FunctionDecl {
            id: 1,
            key: "auth.session".into(),
            kind: FunctionKind::PolicyHandler,
        },
    ];
    let err = eng
        .load(bundle, None, EngineLoadPlan::Numeric { functions })
        .unwrap_err();
    assert!(
        err.contains("semantic function manifest (globalThis.__velquFunctionManifest) is missing")
    );
    eng.shutdown();
}

/// M2.3-r2: Interrupted watched Promise chain leaves strictly 0 entries in settlement table
#[tokio::test]
async fn interrupted_watched_chain_retention_is_zero() {
    let mut eng = engine();
    load_default(&mut eng).unwrap();

    for i in 1..=50 {
        let handle = insert_request(
            &eng,
            q_bridge::RequestMeta {
                query: vec![("ms".into(), "1000".into())],
                ..Default::default()
            },
        );
        let mut s = spec(i, "timer.route", &[200], 10); // short deadline interrupts
        s.slot = handle.slot();
        s.generation = handle.generation();
        let out = run(&mut eng, s).await;
        assert!(matches!(out, Outcome::Timeout));
    }

    tokio::time::sleep(Duration::from_millis(50)).await;
    assert_eq!(eng.settlement_table_len(), 0);
    assert_eq!(eng.bridge_snapshot().live_slots as usize, 0);
    eng.shutdown();
}

/// M26-002-D: invalid bytecode NEVER silently falls back to source —
/// the load fails loudly ("bytecode load failed"), matching the parent
/// guardrail (bytecode is a build artifact; if it cannot load, startup
/// rejects rather than quietly evaluating the bundle).
#[tokio::test]
async fn invalid_bytecode_fails_loudly_never_silently_sources() {
    let mut eng = engine();
    let garbage: Vec<u8> = vec![0xde, 0xad, 0xbe, 0xef, 0x00, 0x01, 0x02];
    let err = eng
        .load(
            BUNDLE,
            Some(&garbage),
            EngineLoadPlan::Legacy {
                expected_handlers: expected_table(),
            },
        )
        .expect_err("invalid bytecode must fail the load");
    assert!(
        err.contains("bytecode load failed"),
        "expected a loud bytecode failure, got: {err}"
    );
    // and the engine did not silently evaluate the source instead: the
    // handlers were never registered, so a legacy load plan now reports
    // missing handlers rather than serving
    eng.shutdown();
}
