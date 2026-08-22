use q_engine::{BodyOut, Engine as _, InvocationSpec, Outcome, ResponseStrategy};
use q_engine_quickjs::{IdentityMapper, QuickJsConfig, QuickJsEngine};
use std::sync::Arc;
use std::time::{Duration, Instant};

#[test]
fn timer_promise_in_bench_context() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .unwrap();
    let _enter = rt.enter();
    let mut engine = QuickJsEngine::spawn(
        QuickJsConfig::default(),
        rt.handle().clone(),
        Arc::new(IdentityMapper),
    );
    let bundle = r#"
async function promise_int(ctx) { const v = await ctx.native.timer.delay(1); return 7; }
__velquRegister("promise.int", promise_int);
"#;
    let table: std::collections::BTreeMap<String, String> =
        [("promise.int".to_string(), String::new())].into();
    engine
        .load(
            bundle,
            None,
            q_engine::EngineLoadPlan::Legacy {
                expected_handlers: table,
            },
        )
        .unwrap();

    let spec = InvocationSpec {
        id: 1,
        request_id: "r".into(),
        route_id: "t".into(),
        route_id_num: None,
        handler_key: "promise.int".into(),
        policy_key: None,
        handler_id: None,
        policy_id_num: None,
        policy_handler_id: None,
        params_schema_id: None,
        query_schema_id: None,
        headers_schema_id: None,
        body_schema_id: None,
        request: Some(q_engine::RequestMeta::default()),
        slot: 0,
        generation: 0,
        params: None,
        query: None,
        headers: None,
        body: None,
        allowed_statuses: vec![200],
        default_status: 200,
        response_strategy: ResponseStrategy::Js,
        raw_response: false,
        deadline: Instant::now() + Duration::from_millis(1000),
    };
    let (tx, rx) = tokio::sync::oneshot::channel();
    engine.invoke(spec, tx);
    let t0 = Instant::now();
    let outcome = rt
        .handle()
        .block_on(async { tokio::time::timeout(Duration::from_millis(1500), rx).await })
        .ok()
        .and_then(|r| r.ok());
    println!("outcome in {:?}: {:?}", t0.elapsed(), outcome);
    assert!(
        matches!(
            outcome,
            Some(Outcome::Response {
                body: BodyOut::JsonText(_),
                ..
            })
        ),
        "timer promise must settle in bench context"
    );
    engine.shutdown();
}
