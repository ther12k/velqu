//! Bridge microbenchmark (M1 §11.6): compares JSON strategies inside a
//! controlled host process (no network), over the frozen input/output matrix.
//!
//! Strategies:
//!   A — engine JSON: bytes → JS string → JSON.parse (engine) → handler →
//!       JSON.stringify (engine) → bytes
//!   B — native JSON: bytes → serde_json parse → recursive object construction
//!       into QuickJS → handler → traversal back to serde_json → serialize
//!
//! Outputs raw JSONL (one line per sample) and a summary JSON to stdout-path
//! or files given by --out-dir. Correctness is asserted per operation.

use std::io::Write;
use std::sync::Arc;
use std::time::Instant;

use q_engine::Engine as _;
use q_engine::{BodyOut, InvocationSpec, Outcome, ResponseStrategy};
use q_engine_quickjs::{IdentityMapper, QuickJsConfig, QuickJsEngine};
use serde_json::{json, Value};

const BUNDLE: &str = r#"
"use strict";
// strategy A handlers: parse/stringify entirely inside the engine
async function a_pass(ctx) { const v = ctx.json(); return v; }
async function a_touch(ctx) { const v = ctx.json(); return { n: v.items.length }; }
async function a_nested(ctx) { return (await ctx.json()).wrapper.inner.list[0].id; }
// strategy B handlers: receive pre-built objects, return values traversed by host
async function b_pass(ctx) { return ctx.body; }
async function b_touch(ctx) { return { n: ctx.body.items.length }; }
async function b_nested(ctx) { return ctx.body.wrapper.inner.list[0].id; }
// scalars / outputs
async function out_int() { return 42; }
async function out_str() { return "velqu"; }
async function out_obj() { return { ok: true, n: 1 }; }
async function out_nested() { return { a: { b: { c: [1, 2, 3] } } }; }
async function out_array(ctx) { const out = []; for (let i = 0; i < 100; i++) out.push({ id: i, name: "item" + i, qty: i * 2 }); return out; }
async function out_problem() { return { __problem: true, problem: "not-found", status: 404 }; }
async function out_bytes() { return new Uint8Array([104, 101, 108, 108, 111]); }
async function promise_int(ctx) { const v = await ctx.native.timer.delay(1); return 7; }
async function five_scalars(ctx) { return { sum: ctx.params.a + ctx.params.b + ctx.query.c + 0 }; }
__velquRegister("a.pass", a_pass);
__velquRegister("a.touch", a_touch);
__velquRegister("a.nested", a_nested);
__velquRegister("b.pass", b_pass);
__velquRegister("b.touch", b_touch);
__velquRegister("b.nested", b_nested);
__velquRegister("out.int", out_int);
__velquRegister("out.str", out_str);
__velquRegister("out.obj", out_obj);
__velquRegister("out.nested", out_nested);
__velquRegister("out.array", out_array);
__velquRegister("out.problem", out_problem);
__velquRegister("out.bytes", out_bytes);
__velquRegister("promise.int", promise_int);
__velquRegister("five.scalars", five_scalars);
"#;

fn small_object() -> Value {
    json!({"name": "Ada Lovelace", "id": 42, "active": true})
}

fn nested_object() -> Value {
    json!({
        "wrapper": { "inner": { "list": [ {"id": "itm_1", "qty": 3}, {"id": "itm_2", "qty": 5} ] } },
        "meta": {"page": 1, "total": 2}
    })
}

fn records_100() -> Value {
    Value::Array(
        (0..100)
            .map(|i| json!({"id": i, "name": format!("item{i}"), "qty": i * 2, "active": i % 2 == 0}))
            .collect(),
    )
}

struct Case {
    name: &'static str,
    handler: &'static str,
    body: Option<Value>,
    params: Option<Value>,
    query: Option<Value>,
    strategy: ResponseStrategy,
    native_body: bool, // strategy B materializes ctx.body; A reads lazily
    check: Box<dyn Fn(&Outcome) -> bool>,
}

fn json_ok(out: &Outcome, expected: &Value) -> bool {
    match out {
        Outcome::Response {
            body: BodyOut::Json(v),
            ..
        } => v == expected,
        Outcome::Response {
            body: BodyOut::JsonText(t),
            ..
        } => serde_json::from_str::<Value>(t)
            .map(|v| &v == expected)
            .unwrap_or(false),
        _ => false,
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let out_dir = args
        .iter()
        .position(|a| a == "--out-dir")
        .and_then(|i| args.get(i + 1))
        .cloned()
        .unwrap_or_else(|| "benchmarks/raw/bridge".into());
    let iters: usize = args
        .iter()
        .position(|a| a == "--iters")
        .and_then(|i| args.get(i + 1))
        .and_then(|v| v.parse().ok())
        .unwrap_or(2000);
    let warmup: usize = 200;
    let _ = std::fs::create_dir_all(&out_dir);

    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .unwrap();
    // NO rt.enter(): an entered context makes Handle::block_on skip driving
    // spawned tasks (timer ops would never fire)
    let mut engine = QuickJsEngine::spawn(
        QuickJsConfig::default(),
        rt.handle().clone(),
        Arc::new(IdentityMapper),
    );
    let table: std::collections::BTreeMap<String, String> = [
        "a.pass",
        "a.touch",
        "a.nested",
        "b.pass",
        "b.touch",
        "b.nested",
        "out.int",
        "out.str",
        "out.obj",
        "out.nested",
        "out.array",
        "out.problem",
        "out.bytes",
        "promise.int",
        "five.scalars",
    ]
    .iter()
    .map(|k| (k.to_string(), String::new()))
    .collect();
    engine
        .load(
            BUNDLE,
            None,
            q_engine::EngineLoadPlan::Legacy {
                expected_handlers: table,
            },
        )
        .expect("bundle loads");

    let mut cases: Vec<Case> = Vec::new();
    // input strategies on small object
    for (name, handler, native) in [
        ("input.small.a", "a.pass", false),
        ("input.small.b", "b.pass", true),
    ] {
        let expected = small_object();
        cases.push(Case {
            name,
            handler,
            body: Some(small_object()),
            params: None,
            query: None,
            strategy: if native {
                ResponseStrategy::Native
            } else {
                ResponseStrategy::Js
            },
            native_body: native,
            check: Box::new(move |o| json_ok(o, &expected)),
        });
    }
    // nested object
    for (name, handler, native) in [
        ("input.nested.a", "a.pass", false),
        ("input.nested.b", "b.pass", true),
    ] {
        let expected = nested_object();
        cases.push(Case {
            name,
            handler,
            body: Some(nested_object()),
            params: None,
            query: None,
            strategy: if native {
                ResponseStrategy::Native
            } else {
                ResponseStrategy::Js
            },
            native_body: native,
            check: Box::new(move |o| json_ok(o, &expected)),
        });
    }
    // array of 100 records (round-trip)
    for (name, handler, native) in [
        ("input.array100.a", "a.pass", false),
        ("input.array100.b", "b.pass", true),
    ] {
        let expected = records_100();
        cases.push(Case {
            name,
            handler,
            body: Some(records_100()),
            params: None,
            query: None,
            strategy: if native {
                ResponseStrategy::Native
            } else {
                ResponseStrategy::Js
            },
            native_body: native,
            check: Box::new(move |o| json_ok(o, &expected)),
        });
    }
    // outputs
    cases.push(Case {
        name: "output.int",
        handler: "out.int",
        body: None,
        params: None,
        query: None,
        strategy: ResponseStrategy::Js,
        native_body: false,
        check: Box::new(|o| json_ok(o, &json!(42))),
    });
    cases.push(Case {
        name: "output.str",
        handler: "out.str",
        body: None,
        params: None,
        query: None,
        strategy: ResponseStrategy::Js,
        native_body: false,
        check: Box::new(
            |o| matches!(o, Outcome::Response { body: BodyOut::Text(ref t), .. } if t == "velqu"),
        ),
    });
    cases.push(Case {
        name: "output.obj.a",
        handler: "out.obj",
        body: None,
        params: None,
        query: None,
        strategy: ResponseStrategy::Js,
        native_body: false,
        check: Box::new(|o| json_ok(o, &json!({"ok": true, "n": 1}))),
    });
    cases.push(Case {
        name: "output.obj.b",
        handler: "out.obj",
        body: None,
        params: None,
        query: None,
        strategy: ResponseStrategy::Native,
        native_body: false,
        check: Box::new(|o| json_ok(o, &json!({"ok": true, "n": 1}))),
    });
    cases.push(Case {
        name: "output.nested.b",
        handler: "out.nested",
        body: None,
        params: None,
        query: None,
        strategy: ResponseStrategy::Native,
        native_body: false,
        check: Box::new(|o| json_ok(o, &json!({"a": {"b": {"c": [1, 2, 3]}}}))),
    });
    cases.push(Case {
        name: "output.array100.a",
        handler: "out.array",
        body: None,
        params: None,
        query: None,
        strategy: ResponseStrategy::Js,
        native_body: false,
        check: Box::new(|o| match o {
            Outcome::Response {
                body: BodyOut::JsonText(t),
                ..
            } => serde_json::from_str::<Value>(t)
                .map(|v| matches!(v.as_array(), Some(a) if a.len() == 100))
                .unwrap_or(false),
            _ => false,
        }),
    });
    cases.push(Case {
        name: "output.array100.b",
        handler: "out.array",
        body: None,
        params: None,
        query: None,
        strategy: ResponseStrategy::Native,
        native_body: false,
        check: Box::new(json_ok_arr100),
    });
    cases.push(Case {
        name: "output.problem",
        handler: "out.problem",
        body: None,
        params: None,
        query: None,
        strategy: ResponseStrategy::Js,
        native_body: false,
        check: Box::new(
            |o| matches!(o, Outcome::Problem(p) if p.problem_id == "not-found" && p.status == 404),
        ),
    });
    cases.push(Case {
        name: "output.bytes",
        handler: "out.bytes",
        body: None,
        params: None,
        query: None,
        strategy: ResponseStrategy::Js,
        native_body: false,
        check: Box::new(
            |o| matches!(o, Outcome::Response { body: BodyOut::Bytes(ref b), .. } if b == b"hello"),
        ),
    });
    cases.push(Case {
        name: "promise.completion",
        handler: "promise.int",
        body: None,
        params: None,
        query: None,
        strategy: ResponseStrategy::Js,
        native_body: false,
        check: Box::new(|o| json_ok(o, &json!(7))),
    });
    cases.push(Case {
        name: "input.scalars5",
        handler: "five.scalars",
        body: None,
        params: Some(json!({"a": 10, "b": 20})),
        query: Some(json!({"c": 12})),
        strategy: ResponseStrategy::Js,
        native_body: false,
        check: Box::new(|o| json_ok(o, &json!({"sum": 42}))),
    });

    let mut raw_file = std::fs::File::create(format!("{out_dir}/bridge.jsonl")).expect("raw out");
    let mut summary: Vec<Value> = Vec::new();

    for case in &cases {
        // correctness once (both counted)
        let ok = run_case(rt.handle(), &mut engine, case, 1_000_000).1;
        if !ok {
            summary.push(json!({"case": case.name, "status": "INVALID", "correct": false}));
            continue;
        }
        // warmup
        let _ = run_case(rt.handle(), &mut engine, case, 1_000_000);
        let mut durations: Vec<f64> = Vec::with_capacity(iters);
        for i in 0..iters {
            let (d, ok) = run_case(rt.handle(), &mut engine, case, 1_000_000 + i as u64);
            debug_assert!(ok);
            durations.push(d);
        }
        durations.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let p = |q: f64| durations[((q * (durations.len() - 1) as f64).round()) as usize];
        let stats = json!({
            "case": case.name,
            "status": "OK",
            "samples": durations.len(),
            "mean_us": durations.iter().sum::<f64>() / durations.len() as f64,
            "p50_us": p(0.50),
            "p95_us": p(0.95),
            "p99_us": p(0.99),
        });
        summary.push(stats);
        for d in &durations {
            let _ = writeln!(raw_file, "{}", json!({"case": case.name, "us": d}));
        }
    }

    let mut sum_file =
        std::fs::File::create(format!("{out_dir}/bridge-summary.json")).expect("summary out");
    let _ = writeln!(
        sum_file,
        "{}",
        json!({
            "format": "velqu-bridge-bench-v1",
            "engine": format!("{}/{}", q_pack_engine(), "0.15.1"),
            "iters": iters,
            "warmup": warmup,
            "cases": summary,
        })
    );
    println!("bridge bench complete: {out_dir}/bridge.jsonl + bridge-summary.json");
    engine.shutdown();
}

fn q_pack_engine() -> &'static str {
    "quickjs-ng"
}

fn run_case(
    rt_handle: &tokio::runtime::Handle,
    engine: &mut QuickJsEngine,
    case: &Case,
    id: u64,
) -> (f64, bool) {
    let body_bytes = case
        .body
        .as_ref()
        .map(|b| bytes::Bytes::from(serde_json::to_vec(b).unwrap()));
    let request = Some(q_engine::RequestMeta {
        body: body_bytes,
        ..Default::default()
    });
    let spec = InvocationSpec {
        id,
        request_id: format!("bench-{id}"),
        route_id: case.name.into(),
        route_id_num: None,
        handler_key: case.handler.into(),
        policy_key: None,
        handler_id: None,
        policy_id_num: None,
        policy_handler_id: None,
        params_schema_id: None,
        query_schema_id: None,
        headers_schema_id: None,
        body_schema_id: None,
        request,
        slot: 0,
        generation: 0,
        params: case.params.clone(),
        query: case.query.clone(),
        headers: None,
        body: if case.native_body {
            case.body.clone()
        } else {
            None
        },
        allowed_statuses: vec![200, 201, 404],
        default_status: 200,
        response_strategy: case.strategy,
        raw_response: false,
        deadline: Instant::now() + std::time::Duration::from_millis(1000),
    };
    let (tx, rx) = tokio::sync::oneshot::channel();
    let t0 = Instant::now();
    engine.invoke(spec, tx);
    let _ = rt_handle
        .block_on(async { tokio::time::timeout(std::time::Duration::from_millis(1500), rx).await });
    let d = t0.elapsed().as_secs_f64() * 1e6;
    // correctness via a second invocation whose outcome we keep
    let request = Some(q_engine::RequestMeta {
        body: case
            .body
            .as_ref()
            .map(|b| bytes::Bytes::from(serde_json::to_vec(b).unwrap())),
        ..Default::default()
    });
    let spec = InvocationSpec {
        id: id + 10_000_000,
        request_id: format!("bench-check-{id}"),
        route_id: case.name.into(),
        route_id_num: None,
        handler_key: case.handler.into(),
        policy_key: None,
        handler_id: None,
        policy_id_num: None,
        policy_handler_id: None,
        params_schema_id: None,
        query_schema_id: None,
        headers_schema_id: None,
        body_schema_id: None,
        request,
        slot: 0,
        generation: 0,
        params: case.params.clone(),
        query: case.query.clone(),
        headers: None,
        body: if case.native_body {
            case.body.clone()
        } else {
            None
        },
        allowed_statuses: vec![200, 201, 404],
        default_status: 200,
        response_strategy: case.strategy,
        raw_response: false,
        deadline: Instant::now() + std::time::Duration::from_millis(1000),
    };
    let (tx, rx) = tokio::sync::oneshot::channel();
    engine.invoke(spec, tx);
    let outcome: Option<Outcome> = rt_handle
        .block_on(async { tokio::time::timeout(std::time::Duration::from_millis(1500), rx).await })
        .ok()
        .and_then(|r| r.ok());
    let ok = outcome.as_ref().map(|o| (case.check)(o)).unwrap_or(false);
    if std::env::var("VELQU_BENCH_DEBUG").is_ok() && !ok {
        eprintln!("case {} outcome: {:?}", case.name, outcome);
    }
    (d, ok)
}

fn json_ok_arr100(out: &Outcome) -> bool {
    match out {
        Outcome::Response {
            body: BodyOut::Json(v),
            ..
        } => matches!(v.as_array(), Some(a) if a.len() == 100),
        _ => false,
    }
}
