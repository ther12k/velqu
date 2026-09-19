//! C3 context-vs-engine decomposition (diagnostic, #1392 follow-up).
//!
//! Splits the remaining post-slotless C3 cost into engine execution,
//! in-worker Velqu glue, and host↔worker handoff — engine-only (no
//! network):
//!
//!   A  raw_call          — cached JS handler invoked directly inside a bare
//!                          QuickJS context with a PRE-CREATED `{params}`
//!                          wrapper object; result discarded (engine floor)
//!   A2 raw_call_extract  — A + per-op boundary read of the result field
//!                          back to a Rust String (minimal result extraction)
//!   B  worker_direct     — the production worker code path
//!                          (`WorkerInner` + `begin_invocation`: makeCtx,
//!                          run(), conversions, settlement postlude) executed
//!                          DIRECTLY on the calling thread — no channel, no
//!                          wakeup, no scheduler participation
//!   C  channel_invoke    — the full production dispatch: host →
//!                          `QuickJsEngine::invoke` → mpsc → worker wakeup →
//!                          (B) → oneshot reply back to the host
//!
//!   B - A  = context construction + in-worker Velqu glue
//!   C - B  = channel + queue + wakeup + scheduling (the handoff)
//!   A      = actual engine execution of the C3-shaped handler
//! HTTP/router/queue/encode/socket overhead is NOT measured here; it is
//! read from the committed #1392 cross-host evidence in the report.
//!
//! Raw JSONL (one line per timed batch) + summary JSON into --out-dir.
//! Correctness is asserted periodically and fail-closes the run.

use std::io::Write;
use std::sync::Arc;
use std::time::Instant;

use q_engine::Engine as _;
use q_engine::{InvocationSpec, Outcome, ResponseStrategy, NO_REQUEST_SLOT};
use q_engine_quickjs::bench_direct::DirectWorker;
use q_engine_quickjs::{IdentityMapper, QuickJsConfig, QuickJsEngine};
use serde_json::{json, Value};

/// Tier-B bundle mirrors the canonical proof C3 handler shape exactly
/// (template literal over ctx.params.name), registered the legacy way.
const BUNDLE: &str = r#"
"use strict";
async function helloC3(ctx) { return { message: `Hello ${ctx.params.name}` }; }
__velquRegister("hello.c3", helloC3);
"#;

/// Tier-A script: same computation, invoked as a plain function with a
/// pre-created argument object (no Velqu context machinery at all).
const RAW_SCRIPT: &str = r#"
"use strict";
globalThis.__tierAHandler = ({ params }) => ({ message: `Hello ${params.name}` });
"#;

fn params() -> Value {
    json!({ "name": "Rafi" })
}
const EXPECTED_MESSAGE: &str = "Hello Rafi";

struct TierStats {
    name: &'static str,
    samples: Vec<f64>, // per-op microseconds, one per timed batch
    ops_per_batch: usize,
    checked: usize,
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
    });
    for v in &t.samples {
        let _ = writeln!(raw_file, "{}", json!({"tier": t.name, "us": v}));
    }
    entry
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
    let a_ops: usize = 1000; // engine-floor tier needs many ops per batch
    let b_ops: usize = 200;
    let _ = std::fs::create_dir_all(&out_dir);

    // ---------------- tier A / A2: bare rquickjs ----------------
    let rt = rquickjs::Runtime::new().expect("quickjs runtime");
    rt.set_memory_limit(64 * 1024 * 1024);
    let ctx = rquickjs::Context::full(&rt).expect("quickjs context");
    let (mut tier_a, mut tier_a2) = (
        TierStats {
            name: "A_raw_call",
            samples: Vec::new(),
            ops_per_batch: a_ops,
            checked: 0,
        },
        TierStats {
            name: "A2_raw_call_extract",
            samples: Vec::new(),
            ops_per_batch: a_ops,
            checked: 0,
        },
    );
    ctx.with(|ctx| -> rquickjs::Result<()> {
        ctx.eval::<(), _>(RAW_SCRIPT)?;
        let handler: rquickjs::Function = ctx.globals().get("__tierAHandler")?;
        let global_this = ctx.globals();
        // pre-created argument: { params: { name: "Rafi" } } — built ONCE
        let params = rquickjs::Object::new(ctx.clone())?;
        params.set("name", "Rafi")?;
        let wrapper = rquickjs::Object::new(ctx.clone())?;
        wrapper.set("params", params)?;
        let wrapper_value = rquickjs::Value::from_object(wrapper);
        // correctness gate before timing
        let out = handler.call::<_, rquickjs::Value>((wrapper_value.clone(),))?;
        let obj = out.as_object().expect("handler returns object");
        assert_eq!(obj.get::<_, String>("message")?, EXPECTED_MESSAGE);
        tier_a.checked += 1;
        // warmup
        for _ in 0..(a_ops * 20) {
            let _ = handler.call::<_, rquickjs::Value>((wrapper_value.clone(),))?;
        }
        // timed batches — tier A: discard result
        for _ in 0..batches {
            let t0 = Instant::now();
            for _ in 0..a_ops {
                let _ = handler.call::<_, rquickjs::Value>((wrapper_value.clone(),))?;
            }
            tier_a
                .samples
                .push(t0.elapsed().as_secs_f64() * 1e6 / a_ops as f64);
        }
        // timed batches — tier A2: extract one string field per op
        let mut sink: u64 = 0;
        for _ in 0..batches {
            let t0 = Instant::now();
            for _ in 0..a_ops {
                let out = handler.call::<_, rquickjs::Value>((wrapper_value.clone(),))?;
                let m: String = out.as_object().unwrap().get("message")?;
                sink += m.len() as u64;
            }
            tier_a2
                .samples
                .push(t0.elapsed().as_secs_f64() * 1e6 / a_ops as f64);
        }
        assert!(sink > 0);
        tier_a2.checked += 1;
        let _ = global_this; // keep scope alive across the loops
        Ok(())
    })
    .expect("tier A evaluation");

    // -------- tiers B and C: in-worker direct, then full channel --------
    let tokio_rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .unwrap();
    // NO tokio_rt.enter(): an entered context makes Handle::block_on skip
    // driving spawned tasks (same caveat as bridge_bench).
    let table: std::collections::BTreeMap<String, String> = [("hello.c3", "")]
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();

    // Tier B: the SAME WorkerInner a spawned thread would run, constructed
    // on THIS thread; begin_invocation called directly — no channel, no
    // wakeup, no scheduler participation.
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
    let mut next_id: u64 = 0;
    let make_spec = |next_id: &mut u64| {
        *next_id += 1;
        let id = *next_id;
        InvocationSpec {
            id,
            request_id: format!("c3d-{id}"),
            route_id: "hello.c3".into(),
            route_id_num: None,
            handler_key: "hello.c3".into(),
            policy_key: None,
            handler_id: None,
            policy_id_num: None,
            policy_handler_id: None,
            params_schema_id: None,
            query_schema_id: None,
            headers_schema_id: None,
            body_schema_id: None,
            request: None, // slotless: no request store entry at all
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
            // ADR-0045: the production C3 shape — params-only slotless
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
    let mut tier_b = TierStats {
        name: "B_worker_direct",
        samples: Vec::new(),
        ops_per_batch: b_ops,
        checked: 0,
    };
    let mut direct_once = |direct: &mut DirectWorker, keep: bool| -> f64 {
        // spec construction hoisted OUT of the timed window — identical to
        // tier C, so B and C time exactly invoke→outcome.
        let spec = make_spec(&mut next_id);
        let t0 = Instant::now();
        let outcome = direct.invoke_direct(spec);
        let us = t0.elapsed().as_secs_f64() * 1e6;
        if keep {
            check_outcome(&outcome);
        }
        us
    };
    direct_once(&mut direct, true);
    tier_b.checked += 1;
    for _ in 0..(b_ops * 5) {
        let _ = direct_once(&mut direct, false);
    }
    for _ in 0..batches {
        let t0 = Instant::now();
        for _ in 0..b_ops {
            let _ = direct_once(&mut direct, false);
        }
        tier_b
            .samples
            .push(t0.elapsed().as_secs_f64() * 1e6 / b_ops as f64);
    }
    direct_once(&mut direct, true);
    tier_b.checked += 1;
    drop(direct);

    // Tier C: the full production dispatch — host → QuickJsEngine::invoke →
    // mpsc channel → worker wakeup → (tier B work) → oneshot reply.
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
                expected_handlers: table.clone(),
            },
        )
        .expect("bundle loads");
    let mut tier_c = TierStats {
        name: "C_channel_invoke",
        samples: Vec::new(),
        ops_per_batch: b_ops,
        checked: 0,
    };
    let mut invoke_once = |engine: &mut QuickJsEngine, keep: bool| -> f64 {
        let spec = make_spec(&mut next_id);
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
    tier_c.checked += 1;
    for _ in 0..(b_ops * 5) {
        invoke_once(&mut engine, false);
    }
    for _ in 0..batches {
        let t0 = Instant::now();
        for _ in 0..b_ops {
            let _ = invoke_once(&mut engine, false);
        }
        tier_c
            .samples
            .push(t0.elapsed().as_secs_f64() * 1e6 / b_ops as f64);
    }
    invoke_once(&mut engine, true);
    tier_c.checked += 1;
    engine.shutdown();

    // Tier B3: in-worker direct REPEATED on a freshly constructed worker
    // AFTER tier C. Distinguishes a stable in-worker-direct cost from
    // run-order/warmup artifacts (guards the B/C comparison; on a 1-vCPU
    // host a single ordering produced an inverted B vs C).
    let mut direct3 = DirectWorker::new(QuickJsConfig::default(), tokio_rt.handle().clone())
        .expect("direct worker 3 constructs");
    direct3
        .load(
            BUNDLE,
            &q_engine::EngineLoadPlan::Legacy {
                expected_handlers: table,
            },
        )
        .expect("direct bundle 3 loads");
    let mut tier_b3 = TierStats {
        name: "B3_worker_direct_repeat",
        samples: Vec::new(),
        ops_per_batch: b_ops,
        checked: 0,
    };
    let mut direct3_once = |direct: &mut DirectWorker, keep: bool| -> f64 {
        let spec = make_spec(&mut next_id);
        let t0 = Instant::now();
        let outcome = direct.invoke_direct(spec);
        let us = t0.elapsed().as_secs_f64() * 1e6;
        if keep {
            check_outcome(&outcome);
        }
        us
    };
    direct3_once(&mut direct3, true);
    tier_b3.checked += 1;
    for _ in 0..(b_ops * 5) {
        let _ = direct3_once(&mut direct3, false);
    }
    for _ in 0..batches {
        let t0 = Instant::now();
        for _ in 0..b_ops {
            let _ = direct3_once(&mut direct3, false);
        }
        tier_b3
            .samples
            .push(t0.elapsed().as_secs_f64() * 1e6 / b_ops as f64);
    }
    direct3_once(&mut direct3, true);
    tier_b3.checked += 1;
    drop(direct3);

    // ---------------- output ----------------
    let mut raw_file =
        std::fs::File::create(format!("{out_dir}/decompose.jsonl")).expect("raw out");
    let tiers = [
        write_tier(&mut raw_file, &tier_a),
        write_tier(&mut raw_file, &tier_a2),
        write_tier(&mut raw_file, &tier_b),
        write_tier(&mut raw_file, &tier_c),
        write_tier(&mut raw_file, &tier_b3),
    ];
    let get = |t: &str, k: &str| {
        tiers
            .iter()
            .find(|e| e["tier"] == t)
            .map(|e| e[k].as_f64().unwrap_or(f64::NAN))
            .unwrap_or(f64::NAN)
    };
    let a = get("A_raw_call", "p50_us");
    let b = get("B_worker_direct", "p50_us");
    let c = get("C_channel_invoke", "p50_us");
    let b3 = get("B3_worker_direct_repeat", "p50_us");
    let summary = json!({
        "format": "velqu-c3-decompose-v2-three-tier",
        "diagnosticOnly": true,
        "hostLabel": std::env::var("C3D_LABEL").unwrap_or_else(|_| "unspecified".into()),
        "sourceCommit": std::env::var("C3D_COMMIT").unwrap_or_else(|_| "unknown".into()),
        "route": { "id": "C3-like", "handler": "({params}) => ({message: `Hello ${params.name}`})" },
        "batches": batches,
        "tiers": tiers,
        "derived": {
            "engine_A_us": a,
            "inworker_glue_B_minus_A_us": b - a,
            "handoff_C_minus_B_us": c - b,
            "extract_A2_minus_A_us": get("A2_raw_call_extract", "p50_us") - a,
            "inworker_glue_share_of_C_pct": (b - a) / c * 100.0,
            "handoff_share_of_C_pct": (c - b) / c * 100.0,
            "b3_repeat_us": b3,
            "b3_minus_b_us": b3 - b,
        },
    });
    let mut sum_file =
        std::fs::File::create(format!("{out_dir}/decompose-summary.json")).expect("summary out");
    let _ = writeln!(sum_file, "{summary}");
    println!("c3 decompose complete: {out_dir}/decompose.jsonl + decompose-summary.json");
    for t in &tiers {
        println!(
            "{}: p50 {:.3}us p95 {:.3}us ({} batches x {} ops, {} correctness checks)",
            t["tier"].as_str().unwrap(),
            t["p50_us"].as_f64().unwrap(),
            t["p95_us"].as_f64().unwrap(),
            t["samples"].as_u64().unwrap(),
            t["ops_per_batch"].as_u64().unwrap(),
            t["correctness_checks"].as_u64().unwrap(),
        );
    }
    let d = &summary["derived"];
    println!(
        "derived: engine(A) {:.3}us | in-worker glue(B-A) {:.3}us | handoff(C-B) {:.3}us | shares of C: glue {:.1}% / handoff {:.1}%",
        d["engine_A_us"].as_f64().unwrap(),
        d["inworker_glue_B_minus_A_us"].as_f64().unwrap(),
        d["handoff_C_minus_B_us"].as_f64().unwrap(),
        d["inworker_glue_share_of_C_pct"].as_f64().unwrap(),
        d["handoff_share_of_C_pct"].as_f64().unwrap(),
    );
}
