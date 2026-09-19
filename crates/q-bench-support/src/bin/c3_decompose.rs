//! C3 async-completion decomposition (diagnostic; #1394 follow-up).
//!
//! One narrow question (owner-set): how much additional work does Velqu
//! perform to deliver an immediately-resolved ASYNC result, beyond the
//! engine's own equivalent completion work? Canonical C3's handler is
//! `async`, so the prior tier-A comparison (a synchronous bare call)
//! mixed "Velqu glue" with "the async contract itself".
//!
//! Six tiers, ALL ending at the same logical boundary (result consumed /
//! invocation outcome finalized):
//!
//!   A_sync   bare engine, ordinary function, result field read
//!   A_async  bare engine, same body declared async — Promise driven to
//!            settlement via the job queue, result field read
//!   P_sync   production worker path (DirectWorker), SYNCHRONOUS
//!            diagnostic handler, outcome finalized
//!   P_async  production worker path, canonical ASYNC handler, outcome
//!            finalized
//!   C_async  full channel dispatch, canonical async handler
//!   P_async_r  P_async repeated after C (run-order guard)
//!
//! Derived (differences of MEASURED tiers — never sums of sub-timers):
//!   A_async − A_sync   = async contract cost in the raw embedding
//!   P_async − P_sync   = async handling cost in Velqu
//!   P_sync  − A_sync   = in-worker glue, SAME contract (clean)
//!   P_async − A_async  = in-worker glue, async contract (clean)
//!   C_async − P_async  = host↔worker handoff
//!
//! Timing discipline: each tier is ONE inclusive measurement to its
//! boundary; this binary builds without bench-instrumentation, so no
//! nested stage timers exist to double-count. Operation counts (watcher
//! registrations, job-queue drains, settlement scans, immediate-vs-
//! promise results) come from the engine's shared counters around each
//! timed phase, reported per completed op.
//!
//! The synchronous handler is DIAGNOSTIC ONLY; the canonical proof route
//! is unchanged and stays async.

use std::io::Write;
use std::sync::Arc;
use std::time::Instant;

use q_engine::Engine as _;
use q_engine::{EngineStats, InvocationSpec, Outcome, ResponseStrategy, NO_REQUEST_SLOT};
use q_engine_quickjs::bench_direct::DirectWorker;
use q_engine_quickjs::{IdentityMapper, QuickJsConfig, QuickJsEngine};
use serde_json::{json, Value};

/// Canonical C3-shaped handler (async — exactly the proof-route contract)
/// plus a diagnostic synchronous twin with an identical body.
const BUNDLE: &str = r#"
"use strict";
async function helloC3(ctx) { return { message: `Hello ${ctx.params.name}` }; }
function helloC3Sync(ctx) { return { message: `Hello ${ctx.params.name}` }; }
__velquRegister("hello.c3", helloC3);
__velquRegister("hello.c3.sync", helloC3Sync);
"#;

/// Bare-engine twins: same computation, sync and async declarations.
const RAW_SCRIPT: &str = r#"
"use strict";
globalThis.__tierASync = ({ params }) => ({ message: `Hello ${params.name}` });
globalThis.__tierAAsync = async ({ params }) => ({ message: `Hello ${params.name}` });
"#;

const EXPECTED_MESSAGE: &str = "Hello Rafi";

fn params() -> Value {
    json!({ "name": "Rafi" })
}

struct TierStats {
    name: &'static str,
    samples: Vec<f64>, // per-op microseconds, one per timed batch
    ops_per_batch: usize,
    checked: usize,
    /// per-completed-op operation counts (raw tiers: jobs pumped;
    /// production tiers: engine shared counters)
    counts: Option<Value>,
}

fn quantile(sorted: &[f64], q: f64) -> f64 {
    sorted[((q * (sorted.len() - 1) as f64).round()) as usize]
}

fn write_tier(raw_file: &mut std::fs::File, t: &TierStats) -> Value {
    let mut s = t.samples.clone();
    s.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let entry = json!({
        "tier": t.name,
        "samples": s.len(),
        "ops_per_batch": t.ops_per_batch,
        "correctness_checks": t.checked,
        "mean_us": s.iter().sum::<f64>() / s.len() as f64,
        "p50_us": quantile(&s, 0.50),
        "p95_us": quantile(&s, 0.95),
        "p99_us": quantile(&s, 0.99),
        "counts_per_op": t.counts.clone().unwrap_or(Value::Null),
    });
    for v in &t.samples {
        let _ = writeln!(raw_file, "{}", json!({"tier": t.name, "us": v}));
    }
    entry
}

/// Table for a fresh worker that only ever invokes the canonical async
/// handler — but it MUST still declare both handlers the bundle registers
/// (load verifies the full table).
fn table_full_async_only() -> std::collections::BTreeMap<String, String> {
    [("hello.c3", ""), ("hello.c3.sync", "")]
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

fn counts_json(before: &EngineStats, after: &EngineStats, ops: u64) -> Value {
    let d = |b: u64, a: u64| ((a - b) as f64 / ops as f64 * 100.0).round() / 100.0;
    json!({
        "handler_calls": d(before.handler_calls, after.handler_calls),
        "immediate_results": d(before.immediate_results, after.immediate_results),
        "promise_results": d(before.promise_results, after.promise_results),
        "promise_watches": d(before.promise_watches, after.promise_watches),
        "job_queue_drains": d(before.job_queue_drains, after.job_queue_drains),
        "settlement_scans": d(before.settlement_scans, after.settlement_scans),
    })
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let out_dir = args
        .iter()
        .position(|a| a == "--out-dir")
        .and_then(|i| args.get(i + 1))
        .cloned()
        .unwrap_or_else(|| "benchmarks/raw/c3-decompose-engine".into());
    let batches: usize = args
        .iter()
        .position(|a| a == "--batches")
        .and_then(|i| args.get(i + 1))
        .and_then(|v| v.parse().ok())
        .unwrap_or(120);
    let a_ops: usize = 1000;
    let b_ops: usize = 200;
    let _ = std::fs::create_dir_all(&out_dir);

    // ---------------- tiers A_sync / A_async: bare rquickjs ----------------
    let rt = rquickjs::Runtime::new().expect("quickjs runtime");
    rt.set_memory_limit(64 * 1024 * 1024);
    let ctx = rquickjs::Context::full(&rt).expect("quickjs context");
    let mut tier_a_sync = TierStats {
        name: "A_sync",
        samples: Vec::new(),
        ops_per_batch: a_ops,
        checked: 0,
        counts: None,
    };
    let mut tier_a_async = TierStats {
        name: "A_async",
        samples: Vec::new(),
        ops_per_batch: a_ops,
        checked: 0,
        counts: None,
    };
    let mut jobs_pumped: u64 = 0;
    ctx.with(|ctx| -> rquickjs::Result<()> {
        ctx.eval::<(), _>(RAW_SCRIPT)?;
        let sync_fn: rquickjs::Function = ctx.globals().get("__tierASync")?;
        let async_fn: rquickjs::Function = ctx.globals().get("__tierAAsync")?;
        let params = rquickjs::Object::new(ctx.clone())?;
        params.set("name", "Rafi")?;
        let wrapper = rquickjs::Object::new(ctx.clone())?;
        wrapper.set("params", params)?;
        let wrapper_value = rquickjs::Value::from_object(wrapper);
        let read_message = |out: rquickjs::Value| -> rquickjs::Result<String> {
            out.as_object()
                .expect("handler returns object")
                .get::<_, String>("message")
        };
        // correctness gates
        assert_eq!(
            read_message(sync_fn.call((wrapper_value.clone(),))?)?,
            EXPECTED_MESSAGE
        );
        tier_a_sync.checked += 1;
        let promise: rquickjs::Promise = async_fn
            .call::<_, rquickjs::Value>((wrapper_value.clone(),))?
            .get()
            .unwrap();
        while ctx.execute_pending_job() {
            jobs_pumped += 1;
        }
        let obj = promise
            .result::<rquickjs::Object>()
            .expect("settled")
            .expect("fulfilled");
        assert_eq!(obj.get::<_, String>("message")?, EXPECTED_MESSAGE);
        tier_a_async.checked += 1;
        // warmup
        for _ in 0..(a_ops * 20) {
            let _ = sync_fn.call::<_, rquickjs::Value>((wrapper_value.clone(),))?;
        }
        for _ in 0..(a_ops * 20) {
            let _: rquickjs::Promise = async_fn
                .call::<_, rquickjs::Value>((wrapper_value.clone(),))?
                .get()
                .unwrap();
            while ctx.execute_pending_job() {}
        }
        // timed: A_sync — call, read field (boundary: result consumed)
        for _ in 0..batches {
            let t0 = Instant::now();
            let mut sink = 0u64;
            for _ in 0..a_ops {
                let out = sync_fn.call::<_, rquickjs::Value>((wrapper_value.clone(),))?;
                sink += read_message(out)?.len() as u64;
            }
            assert!(sink > 0);
            tier_a_sync
                .samples
                .push(t0.elapsed().as_secs_f64() * 1e6 / a_ops as f64);
        }
        // timed: A_async — call, pump jobs to settlement, read field
        // (boundary: promise result consumed — same logical boundary)
        let jobs_before = jobs_pumped;
        let mut ops_done = 0u64;
        for _ in 0..batches {
            let t0 = Instant::now();
            let mut sink = 0u64;
            for _ in 0..a_ops {
                let p: rquickjs::Promise = async_fn
                    .call::<_, rquickjs::Value>((wrapper_value.clone(),))?
                    .get()
                    .unwrap();
                while ctx.execute_pending_job() {
                    jobs_pumped += 1;
                }
                let obj = p.result::<rquickjs::Object>().expect("settled").expect("fulfilled");
                sink += obj.get::<_, String>("message")?.len() as u64;
                ops_done += 1;
            }
            assert!(sink > 0);
            tier_a_async
                .samples
                .push(t0.elapsed().as_secs_f64() * 1e6 / a_ops as f64);
        }
        tier_a_async.counts = Some(json!({
            "jobs_pumped": ((jobs_pumped - jobs_before) as f64 / ops_done as f64 * 100.0).round() / 100.0,
        }));
        Ok(())
    })
    .expect("tier A evaluation");

    // -------- production tiers: DirectWorker (sync/async) + channel --------
    let tokio_rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .unwrap();
    let table: std::collections::BTreeMap<String, String> =
        [("hello.c3", ""), ("hello.c3.sync", "")]
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
    let mut next_id: u64 = 0;
    let make_spec = |next_id: &mut u64, handler: &'static str| {
        *next_id += 1;
        let id = *next_id;
        InvocationSpec {
            id,
            request_id: format!("c3d-{id}"),
            route_id: handler.into(),
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
            slot: NO_REQUEST_SLOT,
            generation: 0,
            params: Some(params()),
            query: None,
            headers: None,
            body: None,
            allowed_statuses: vec![200],
            default_status: 200,
            response_strategy: ResponseStrategy::Native,
            raw_response: false,
            context_plan: q_engine::ContextPlan::ValidatedParamsOnly,
            deadline: Instant::now() + std::time::Duration::from_millis(2000),
        }
    };
    let check_outcome = |outcome: &Outcome| match outcome {
        Outcome::Response { body, status, .. } => {
            let ok = *status == 200
                && match body {
                    q_engine::BodyOut::Json(v) => v["message"] == EXPECTED_MESSAGE,
                    q_engine::BodyOut::JsonText(t) => serde_json::from_str::<Value>(t.as_str())
                        .map(|v| v["message"] == EXPECTED_MESSAGE)
                        .unwrap_or(false),
                    _ => false,
                };
            assert!(ok, "correctness: {body:?}");
        }
        other => panic!("expected Response, got {other:?}"),
    };

    // One DirectWorker carries BOTH handlers (same worker state); each
    // handler is measured in its own timed phase with counter deltas.
    let mut direct = DirectWorker::new(QuickJsConfig::default(), tokio_rt.handle().clone())
        .expect("direct worker constructs");
    direct
        .load(
            BUNDLE,
            &q_engine::EngineLoadPlan::Legacy {
                expected_handlers: table.clone(),
            },
        )
        .expect("direct bundle loads");
    let mut direct_once = |direct: &mut DirectWorker, handler: &'static str, keep: bool| -> f64 {
        let spec = make_spec(&mut next_id, handler);
        let t0 = Instant::now();
        let outcome = direct.invoke_direct(spec);
        let us = t0.elapsed().as_secs_f64() * 1e6;
        if keep {
            check_outcome(&outcome);
        }
        us
    };
    let mut run_direct_phase =
        |direct: &mut DirectWorker, handler: &'static str, name: &'static str| -> TierStats {
            let mut t = TierStats {
                name,
                samples: Vec::new(),
                ops_per_batch: b_ops,
                checked: 0,
                counts: None,
            };
            direct_once(direct, handler, true);
            t.checked += 1;
            for _ in 0..(b_ops * 5) {
                let _ = direct_once(direct, handler, false);
            }
            let before = direct.stats();
            for _ in 0..batches {
                let t0 = Instant::now();
                for _ in 0..b_ops {
                    let _ = direct_once(direct, handler, false);
                }
                t.samples
                    .push(t0.elapsed().as_secs_f64() * 1e6 / b_ops as f64);
            }
            let after = direct.stats();
            let ops = after.invocations - before.invocations;
            t.counts = Some(counts_json(&before, &after, ops));
            direct_once(direct, handler, true);
            t.checked += 1;
            t
        };
    let tier_p_sync = run_direct_phase(&mut direct, "hello.c3.sync", "P_sync");
    let tier_p_async = run_direct_phase(&mut direct, "hello.c3", "P_async");
    drop(direct);

    // Channel tier: canonical async through the full dispatch.
    let mut engine = QuickJsEngine::spawn(
        QuickJsConfig::default(),
        tokio_rt.handle().clone(),
        Arc::new(IdentityMapper),
    );
    engine
        .load(
            BUNDLE,
            None,
            q_engine::EngineLoadPlan::Legacy {
                expected_handlers: table,
            },
        )
        .expect("bundle loads");
    let mut tier_c_async = TierStats {
        name: "C_async",
        samples: Vec::new(),
        ops_per_batch: b_ops,
        checked: 0,
        counts: None,
    };
    let mut invoke_once = |engine: &mut QuickJsEngine, keep: bool| -> f64 {
        let spec = make_spec(&mut next_id, "hello.c3");
        let (tx, rx) = tokio::sync::oneshot::channel();
        let t0 = Instant::now();
        engine.invoke(spec, tx);
        let outcome = tokio_rt
            .handle()
            .block_on(async {
                tokio::time::timeout(std::time::Duration::from_millis(1900), rx).await
            })
            .expect("reply within deadline")
            .expect("channel open");
        let us = t0.elapsed().as_secs_f64() * 1e6;
        if keep {
            check_outcome(&outcome);
        }
        us
    };
    invoke_once(&mut engine, true);
    tier_c_async.checked += 1;
    for _ in 0..(b_ops * 5) {
        invoke_once(&mut engine, false);
    }
    let before = engine.stats();
    for _ in 0..batches {
        let t0 = Instant::now();
        for _ in 0..b_ops {
            let _ = invoke_once(&mut engine, false);
        }
        tier_c_async
            .samples
            .push(t0.elapsed().as_secs_f64() * 1e6 / b_ops as f64);
    }
    let after = engine.stats();
    let ops = after.invocations - before.invocations;
    tier_c_async.counts = Some(counts_json(&before, &after, ops));
    invoke_once(&mut engine, true);
    tier_c_async.checked += 1;
    engine.shutdown();

    // Guard: P_async repeated on a fresh worker AFTER the channel tier.
    let mut direct3 = DirectWorker::new(QuickJsConfig::default(), tokio_rt.handle().clone())
        .expect("direct worker 3 constructs");
    direct3
        .load(
            BUNDLE,
            &q_engine::EngineLoadPlan::Legacy {
                expected_handlers: table_full_async_only(),
            },
        )
        .expect("direct bundle 3 loads");
    let mut direct3_once = |direct: &mut DirectWorker, keep: bool| -> f64 {
        let spec = make_spec(&mut next_id, "hello.c3");
        let t0 = Instant::now();
        let outcome = direct.invoke_direct(spec);
        let us = t0.elapsed().as_secs_f64() * 1e6;
        if keep {
            check_outcome(&outcome);
        }
        us
    };
    let mut tier_p_async_r = TierStats {
        name: "P_async_repeat",
        samples: Vec::new(),
        ops_per_batch: b_ops,
        checked: 0,
        counts: None,
    };
    direct3_once(&mut direct3, true);
    tier_p_async_r.checked += 1;
    for _ in 0..(b_ops * 5) {
        let _ = direct3_once(&mut direct3, false);
    }
    for _ in 0..batches {
        let t0 = Instant::now();
        for _ in 0..b_ops {
            let _ = direct3_once(&mut direct3, false);
        }
        tier_p_async_r
            .samples
            .push(t0.elapsed().as_secs_f64() * 1e6 / b_ops as f64);
    }
    direct3_once(&mut direct3, true);
    tier_p_async_r.checked += 1;
    drop(direct3);

    // ---------------- output ----------------
    let mut raw_file =
        std::fs::File::create(format!("{out_dir}/decompose.jsonl")).expect("raw out");
    let tiers = [
        write_tier(&mut raw_file, &tier_a_sync),
        write_tier(&mut raw_file, &tier_a_async),
        write_tier(&mut raw_file, &tier_p_sync),
        write_tier(&mut raw_file, &tier_p_async),
        write_tier(&mut raw_file, &tier_c_async),
        write_tier(&mut raw_file, &tier_p_async_r),
    ];
    let get = |t: &str, k: &str| {
        tiers
            .iter()
            .find(|e| e["tier"] == t)
            .map(|e| e[k].as_f64().unwrap_or(f64::NAN))
            .unwrap_or(f64::NAN)
    };
    let p = |t: &str| get(t, "p50_us");
    let summary = json!({
        "format": "velqu-c3-decompose-v3-completion-matrix",
        "diagnosticOnly": true,
        "boundary": "all tiers end at result-consumed / outcome-finalized; deltas are differences of measured tiers (no sub-timer sums)",
        "hostLabel": std::env::var("C3D_LABEL").unwrap_or_else(|_| "unspecified".into()),
        "sourceCommit": std::env::var("C3D_COMMIT").unwrap_or_else(|_| "unknown".into()),
        "handlers": {
            "canonical": "async ({params}) => ({message: `Hello ${params.name}`})",
            "diagnostic_sync": "same body, non-async — diagnostic only; canonical route unchanged",
        },
        "batches": batches,
        "tiers": tiers,
        "derived": {
            "raw_async_contract_us": p("A_async") - p("A_sync"),
            "prod_async_handling_us": p("P_async") - p("P_sync"),
            "inworker_glue_sync_us": p("P_sync") - p("A_sync"),
            "inworker_glue_async_us": p("P_async") - p("A_async"),
            "handoff_us": p("C_async") - p("P_async"),
            "guard_repeat_delta_us": p("P_async_repeat") - p("P_async"),
        },
    });
    let mut sum_file =
        std::fs::File::create(format!("{out_dir}/decompose-summary.json")).expect("summary out");
    let _ = writeln!(sum_file, "{summary}");
    println!("c3 decompose complete: {out_dir}/decompose.jsonl + decompose-summary.json");
    for t in &tiers {
        println!(
            "{}: p50 {:.3}us p95 {:.3}us ({}x{} ops, {} checks, counts {})",
            t["tier"].as_str().unwrap(),
            t["p50_us"].as_f64().unwrap(),
            t["p95_us"].as_f64().unwrap(),
            t["samples"].as_u64().unwrap(),
            t["ops_per_batch"].as_u64().unwrap(),
            t["correctness_checks"].as_u64().unwrap(),
            t["counts_per_op"],
        );
    }
    let d = &summary["derived"];
    println!(
        "derived: raw-async {:.3} | prod-async {:.3} | glue(sync) {:.3} | glue(async) {:.3} | handoff {:.3} | guard {:+.3}",
        d["raw_async_contract_us"].as_f64().unwrap(),
        d["prod_async_handling_us"].as_f64().unwrap(),
        d["inworker_glue_sync_us"].as_f64().unwrap(),
        d["inworker_glue_async_us"].as_f64().unwrap(),
        d["handoff_us"].as_f64().unwrap(),
        d["guard_repeat_delta_us"].as_f64().unwrap(),
    );
}
