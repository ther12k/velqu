//! Engine integration tests: load, invoke (sync/async/promise), timer
//! capability, cancellation matrix, handle expiry, redaction inputs, and
//! contract-violation detection — all through the public Engine trait.

use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use q_engine::{BodyOut, Engine, InvocationSpec, Outcome, ResponseStrategy};
use q_engine_quickjs::{IdentityMapper, QuickJsConfig, QuickJsEngine};

fn spec(id: u64, handler: &str, allowed: &[u16], deadline_ms: u64) -> InvocationSpec {
    InvocationSpec {
        id,
        request_id: format!("req-{id}"),
        route_id: "test.route".into(),
        handler_key: handler.into(),
        policy_key: None,
        slot: 0,
        generation: 0,
        params: None,
        query: None,
        headers: None,
        body: None,
        allowed_statuses: allowed.to_vec(),
        default_status: 200,
        response_strategy: ResponseStrategy::Js,
        deadline: Instant::now() + Duration::from_millis(deadline_ms),
    }
}

fn engine() -> (QuickJsEngine, Arc<q_bridge::RequestStore>) {
    let store = Arc::new(q_bridge::RequestStore::new());
    // reuse the #[tokio::test] runtime's handle; a private inner runtime would
    // be dropped while the worker thread still holds it
    let engine = QuickJsEngine::spawn(
        QuickJsConfig::default(),
        Arc::clone(&store),
        tokio::runtime::Handle::current(),
        Arc::new(IdentityMapper),
    );
    (engine, store)
}

const BUNDLE: &str = r#"
async function js_text(ctx) { return "plain"; }
async function js_json(ctx) { return { ok: true }; }
async function hello(ctx) { return { message: "Hello " + ctx.params.name }; }
async function lazy_ctx(ctx) {
  // deliberately do NOT touch ctx.params/query/body: laziness proof
  return { untouched: true };
}
async function read_query(ctx) { return { ms: ctx.query.ms }; }
async function read_body(ctx) { return { echo: ctx.json().name }; }
async function timer_route(ctx) {
  // lazy query values are strings: explicit coercion (SCHEMA-002 semantics;
  // the native validation strategy pre-coerces instead)
  const ms = Number(ctx.query?.ms ?? 50);
  const waited = await ctx.native.timer.delay(ms);
  return { waited };
}
async function thrower(ctx) { throw new Error("secret-boom"); }
async function undeclared_status(ctx) { return { __ok: true, status: 299, value: { nope: true } }; }
async function declared_201(ctx) { return { __ok: true, status: 201, value: { id: "usr_1" } }; }
async function problem_404(ctx) { return { __problem: true, problem: "not-found", status: 404, detail: "no such user" }; }
async function infinite(ctx) { while (true) {} }
async function spin_then(ctx) { return Promise.resolve().then(() => { while (true) {} }); }
async function biz_status(ctx) { return { status: 200, value: "order-9", filled: false }; }
async function native_body(ctx) { return { id: ctx.body.id, doubled: ctx.body.n * 2 }; }
__velquRegister("js.text", js_text);
__velquRegister("js.json", js_json);
__velquRegister("hello.get", hello);
__velquRegister("lazy.ctx", lazy_ctx);
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
"#;

fn expected_table() -> BTreeMap<String, String> {
    [
        "js.text",
        "js.json",
        "hello.get",
        "lazy.ctx",
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
    ]
    .iter()
    .map(|k| (k.to_string(), String::new()))
    .collect()
}

#[tokio::test]
async fn load_verifies_handler_table_and_caches() {
    let (mut eng, _store) = engine();
    let stats = eng.load(BUNDLE, None, &expected_table()).expect("load");
    assert_eq!(stats.handlers_registered, 15);
    // a table mismatch must fail
    let mut bad = expected_table();
    bad.insert("extra.handler".into(), String::new());
    assert!(eng.load(BUNDLE, None, &bad).is_err());
    eng.shutdown();
}

#[tokio::test]
async fn sync_text_and_json_results() {
    let (mut eng, _store) = engine();
    eng.load(BUNDLE, None, &expected_table()).unwrap();
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
async fn prevalidated_params_flow_into_ctx() {
    let (mut eng, store) = engine();
    eng.load(BUNDLE, None, &expected_table()).unwrap();
    let (slot, gen) = store.insert(q_bridge::RequestMeta::default());
    let mut s = spec(3, "hello.get", &[200], 1000);
    s.slot = slot;
    s.generation = gen;
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
    assert_eq!(store.live_slots(), 0, "settlement must free the slot");
}

#[tokio::test]
async fn lazy_ctx_touches_nothing() {
    let (mut eng, store) = engine();
    eng.load(BUNDLE, None, &expected_table()).unwrap();
    let (slot, gen) = store.insert(q_bridge::RequestMeta {
        params: vec![("name".into(), "Rafi".into())],
        query: vec![("ms".into(), "50".into())],
        body: Some(b"{\"name\":\"Ada\"}".to_vec()),
        ..Default::default()
    });
    let mut s = spec(4, "lazy.ctx", &[200], 1000);
    s.slot = slot;
    s.generation = gen;
    let out = run(&mut eng, s).await;
    assert!(matches!(out, Outcome::Response { .. }));
    let snap = store.snapshot();
    assert_eq!(
        snap.host_calls, 0,
        "RUN-004: unread fields materialize nothing"
    );
    assert_eq!(snap.materialized_bytes, 0);
    assert_eq!(snap.materialized_fields, 0);
}

#[tokio::test]
async fn lazy_query_and_body_materialize_on_access() {
    let (mut eng, store) = engine();
    eng.load(BUNDLE, None, &expected_table()).unwrap();
    // query path (lazy)
    let (slot, gen) = store.insert(q_bridge::RequestMeta {
        query: vec![("ms".into(), "42".into())],
        ..Default::default()
    });
    let mut s = spec(5, "query.read", &[200], 1000);
    s.slot = slot;
    s.generation = gen;
    let out = run(&mut eng, s).await;
    match out {
        Outcome::Response {
            body: BodyOut::JsonText(t),
            ..
        } => assert_eq!(t, "{\"ms\":\"42\"}"),
        o => panic!("{o:?}"),
    }
    // body path (lazy json())
    let (slot, gen) = store.insert(q_bridge::RequestMeta {
        body: Some(b"{\"name\":\"Ada\"}".to_vec()),
        ..Default::default()
    });
    let mut s = spec(6, "body.read", &[200], 1000);
    s.slot = slot;
    s.generation = gen;
    let out = run(&mut eng, s).await;
    match out {
        Outcome::Response {
            body: BodyOut::JsonText(t),
            ..
        } => assert_eq!(t, "{\"echo\":\"Ada\"}"),
        o => panic!("{o:?}"),
    }
    // native body path (pre-validated)
    let (slot, gen) = store.insert(q_bridge::RequestMeta::default());
    let mut s = spec(7, "body.native", &[200], 1000);
    s.slot = slot;
    s.generation = gen;
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
    let (mut eng, store) = engine();
    eng.load(BUNDLE, None, &expected_table()).unwrap();
    let (slot, gen) = store.insert(q_bridge::RequestMeta {
        query: vec![("ms".into(), "30".into())],
        ..Default::default()
    });
    let mut s = spec(8, "timer.route", &[200], 2000);
    s.slot = slot;
    s.generation = gen;
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
    assert_eq!(store.live_slots(), 0);
}

#[tokio::test]
async fn cancellation_before_completion() {
    let (mut eng, store) = engine();
    eng.load(BUNDLE, None, &expected_table()).unwrap();
    let (slot, gen) = store.insert(q_bridge::RequestMeta {
        query: vec![("ms".into(), "5000".into())],
        ..Default::default()
    });
    let mut s = spec(9, "timer.route", &[200], 10_000);
    s.slot = slot;
    s.generation = gen;
    let (tx, rx) = tokio::sync::oneshot::channel();
    eng.invoke(s, tx);
    tokio::time::sleep(Duration::from_millis(30)).await;
    eng.cancel(9);
    // reply may not fire (cancelled): the oneshot is simply dropped
    let _ = tokio::time::timeout(Duration::from_millis(300), rx).await;
    tokio::time::sleep(Duration::from_millis(50)).await;
    let stats = eng.stats();
    assert_eq!(stats.cancelled_invocations, 1);
    assert_eq!(store.live_slots(), 0, "cancel must settle the handle");
    eng.shutdown();
}

#[tokio::test]
async fn deadline_timeout_interrupts_and_replies() {
    let (mut eng, _store) = engine();
    eng.load(BUNDLE, None, &expected_table()).unwrap();
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
    let (mut eng, _store) = engine();
    eng.load(BUNDLE, None, &expected_table()).unwrap();
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
    let (mut eng, _store) = engine();
    eng.load(BUNDLE, None, &expected_table()).unwrap();
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
    let (mut eng, _store) = engine();
    eng.load(BUNDLE, None, &expected_table()).unwrap();
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
    let (mut eng, store) = engine();
    eng.load(BUNDLE, None, &expected_table()).unwrap();
    // simulate a retained wrapper: settle the slot, then let JS touch it
    let (slot, gen) = store.insert(q_bridge::RequestMeta {
        query: vec![("ms".into(), "1".into())],
        ..Default::default()
    });
    store.settle(slot, gen); // expired before invocation
    let mut s = spec(16, "query.read", &[200], 1000);
    s.slot = slot;
    s.generation = gen;
    let out = run(&mut eng, s).await;
    match out {
        Outcome::EngineFailure { detail, .. } => assert!(detail.contains("expired")),
        o => panic!("expected expired-handle failure, got {o:?}"),
    }
    assert_eq!(store.snapshot().expired_accesses, 1);
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
    let (mut eng, _store) = engine();
    eng.load(BUNDLE, None, &expected_table()).unwrap();
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
    let (mut eng, _store) = engine();
    eng.load(BUNDLE, None, &expected_table()).unwrap();
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
