//! M25-002-A/B/C codec strategy benchmark: compares three JSON input→handler→
//! output strategies across the frozen payload matrix inside a controlled host
//! process (no network), with per-sample CPU, allocation, and bridge evidence.
//!
//! Candidates (input direction):
//!   quickjs-json     — bounded bytes enter the engine; the handler parses via
//!                      ctx.json() (engine JSON.parse) and the response leaves
//!                      via engine JSON.stringify. No schema validation runs
//!                      on this path (it is the ADR-0015 escape hatch).
//!   generic-rust     — serde_json parse + generic tree-walk validation
//!                      (q_schema_runtime::validate) + recursive object
//!                      construction into QuickJS; response via native
//!                      traversal (current production default, ADR-0015).
//!   generated-schema — serde_json parse + GENERATED fused decode/validate
//!                      projection (codec_bench/generated.rs) + the same
//!                      recursive construction; response identical to
//!                      generic-rust. PROTOTYPE: it still parses through
//!                      serde_json; the direct byte scanner/decoder is the
//!                      M25-003/M25-004 deliverable, not this benchmark.
//!
//! Fairness (also recorded in codec-summary.json):
//!   - quickjs-json performs no schema validation while both host candidates
//!     fully validate inputs; that asymmetry IS the strategy question.
//!   - generated-schema shares generic-rust's serde_json parse and JS
//!     boundary, so the measured delta isolates validation/projection only.
//!   - M25-002-C instrumentation: CPU is getrusage(RUSAGE_SELF) deltas (the
//!     host denies perf counters; no hardware-counter claim is made);
//!     allocation is LD_PRELOAD allocator-event/requested-byte deltas (not
//!     live heap); bridge time is q-bridge `bench-instrumentation` timing of
//!     RequestStore access only, so engine_us remains the full bridge/JS
//!     round trip. No strategy is selected here (M25-002-D).
//!
//! Outputs under --out-dir: codec.jsonl (one row per timed sample),
//! codec-summary.json, and evidence.json (sha256 of the binary, the generated
//! module source, the raw file, and the summary). Correctness is asserted per
//! cell before any timing sample is taken.

use std::io::Write as _;
use std::sync::Arc;
use std::time::Instant;

use q_engine::Engine as _;
use q_engine::{BodyOut, InvocationSpec, Outcome, ResponseStrategy};
use q_engine_quickjs::{IdentityMapper, QuickJsConfig, QuickJsEngine};
use serde_json::{json, Value};
use sha2::Digest as _;

mod generated;
mod generator;
mod schemas;

const BUNDLE: &str = r#"
"use strict";
// quickjs-json handler: the engine parses and stringifies (lazy body access)
async function pass_json(ctx) { return ctx.json(); }
// host-strategy handler: returns the pre-validated native body object
async function pass_body(ctx) { return ctx.body; }
__velquRegister("codec.pass_json", pass_json);
__velquRegister("codec.pass_body", pass_body);
"#;

const WARMUP: usize = 200;
const GENERATED_MODULE_PATH: &str = "crates/q-bench-support/src/bin/codec_bench/generated.rs";
const COMMAND: &str = "LD_PRELOAD=target/alloc-tracer.so VELQU_ALLOC_PROFILE=benchmarks/raw/codec-c/codec.alloc.json /usr/bin/time -v -o benchmarks/raw/codec-c/codec.process.time.txt target/m25-002-c-bench/release/q-codec-bench --out-dir benchmarks/raw/codec-c --iters 2000";
const PACKET: &str = "M25-002-C";

/// M25-002-C allocator-event snapshot ABI (scripts/alloc-tracer.c).
#[repr(C)]
#[derive(Clone, Copy, Default)]
struct AllocCounters {
    malloc_calls: u64,
    calloc_calls: u64,
    realloc_calls: u64,
    free_calls: u64,
    allocated_bytes: u64,
    reallocated_bytes: u64,
}

impl AllocCounters {
    fn delta(from: AllocCounters, to: AllocCounters) -> AllocCounters {
        AllocCounters {
            malloc_calls: to.malloc_calls.saturating_sub(from.malloc_calls),
            calloc_calls: to.calloc_calls.saturating_sub(from.calloc_calls),
            realloc_calls: to.realloc_calls.saturating_sub(from.realloc_calls),
            free_calls: to.free_calls.saturating_sub(from.free_calls),
            allocated_bytes: to.allocated_bytes.saturating_sub(from.allocated_bytes),
            reallocated_bytes: to.reallocated_bytes.saturating_sub(from.reallocated_bytes),
        }
    }
    fn call_count(&self) -> u64 {
        self.malloc_calls + self.calloc_calls + self.realloc_calls + self.free_calls
    }
}

/// Resolve the tracer's snapshot symbol from the preloaded global scope.
/// None means the tracer is absent: allocation evidence is then recorded as
/// unavailable, never silently zero.
fn alloc_snapshot_fn() -> Option<unsafe extern "C" fn(*mut AllocCounters)> {
    unsafe {
        let sym = libc::dlsym(libc::RTLD_DEFAULT, c"velqu_alloc_snapshot".as_ptr());
        if sym.is_null() {
            None
        } else {
            Some(std::mem::transmute::<
                *mut core::ffi::c_void,
                unsafe extern "C" fn(*mut AllocCounters),
            >(sym))
        }
    }
}

/// getrusage(RUSAGE_SELF) CPU time in microseconds: (user, system).
fn cpu_time_us() -> (u64, u64) {
    unsafe {
        let mut ru: libc::rusage = std::mem::zeroed();
        libc::getrusage(libc::RUSAGE_SELF, &mut ru);
        let user = (ru.ru_utime.tv_sec as u64) * 1_000_000 + ru.ru_utime.tv_usec as u64;
        let sys = (ru.ru_stime.tv_sec as u64) * 1_000_000 + ru.ru_stime.tv_usec as u64;
        (user, sys)
    }
}

/// One fully instrumented strategy execution. Stage boundaries:
///   codec_us  — host-side parse + validate/project (0 for quickjs-json,
///               whose parse happens inside the engine)
///   engine_us — engine.invoke through outcome receipt (worker queue, JS
///               execution, conversion, and native bridge round trip)
///   total_us  — the whole sample, including instrumentation reads
struct Sample {
    total_us: f64,
    codec_us: f64,
    engine_us: f64,
    cpu_user_us: u64,
    cpu_system_us: u64,
    bridge_access_ns: u64,
    bridge_host_calls: u64,
    bridge_materialized_fields: u64,
    bridge_materialized_bytes: u64,
    alloc: Option<AllocCounters>,
    outcome: Option<Outcome>,
}

#[derive(Clone, Copy, PartialEq)]
enum Cand {
    QuickJsJson,
    GenericRust,
    GeneratedSchema,
}

impl Cand {
    fn name(self) -> &'static str {
        match self {
            Cand::QuickJsJson => "quickjs-json",
            Cand::GenericRust => "generic-rust",
            Cand::GeneratedSchema => "generated-schema",
        }
    }
    fn strategy(self) -> ResponseStrategy {
        match self {
            Cand::QuickJsJson => ResponseStrategy::Js,
            _ => ResponseStrategy::Native,
        }
    }
    fn handler(self) -> &'static str {
        match self {
            Cand::QuickJsJson => "codec.pass_json",
            _ => "codec.pass_body",
        }
    }
}

const CANDS: [Cand; 3] = [Cand::QuickJsJson, Cand::GenericRust, Cand::GeneratedSchema];

fn sha256_hex(data: &[u8]) -> String {
    let mut h = sha2::Sha256::new();
    h.update(data);
    h.finalize().iter().map(|b| format!("{b:02x}")).collect()
}

/// mean/p50/p95/p99 over one metric's per-sample values (sorted in place;
/// same nearest-rank percentile rule the B report used).
fn metric_stats(values: &mut [f64]) -> Value {
    values.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let p = |q: f64| values[((q * (values.len() - 1) as f64).round()) as usize];
    json!({
        "n": values.len(),
        "mean": values.iter().sum::<f64>() / values.len() as f64,
        "p50": p(0.50),
        "p95": p(0.95),
        "p99": p(0.99),
    })
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if let Some(pos) = args.iter().position(|a| a == "--emit-generated") {
        let path = args.get(pos + 1).expect("--emit-generated requires a path");
        let src = generator::generate(&schemas::corpus());
        std::fs::write(path, src).expect("write generated module");
        println!("generated module written: {path}");
        return;
    }
    let out_dir = args
        .iter()
        .position(|a| a == "--out-dir")
        .and_then(|i| args.get(i + 1))
        .cloned()
        .unwrap_or_else(|| "benchmarks/raw/codec".into());
    let iters: usize = args
        .iter()
        .position(|a| a == "--iters")
        .and_then(|i| args.get(i + 1))
        .and_then(|v| v.parse().ok())
        .unwrap_or(2000);
    let _ = std::fs::create_dir_all(&out_dir);

    let run_id = format!(
        "m25-002-c-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0)
    );

    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .unwrap();
    // NO rt.enter(): an entered context makes Handle::block_on skip driving
    // spawned tasks (timer ops would never fire) — same contract as the M1
    // bridge benchmark.
    let mut engine = QuickJsEngine::spawn(
        QuickJsConfig::default(),
        rt.handle().clone(),
        Arc::new(IdentityMapper),
    );
    let table: std::collections::BTreeMap<String, String> = ["codec.pass_json", "codec.pass_body"]
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

    let corpus = schemas::corpus();
    let alloc_fn = alloc_snapshot_fn();
    let allocator_status = if alloc_fn.is_some() {
        "captured"
    } else {
        "unavailable (run under LD_PRELOAD=target/alloc-tracer.so)"
    };
    let mut raw_file = std::fs::File::create(format!("{out_dir}/codec.jsonl")).expect("raw out");
    let mut summary_cases: Vec<Value> = Vec::new();

    for (schema_idx, schema) in corpus.iter().enumerate() {
        if !generated::supports(&schema.ir) {
            for cand in CANDS {
                if cand == Cand::GeneratedSchema {
                    summary_cases.push(json!({
                        "case": schema.name,
                        "candidate": cand.name(),
                        "status": "UNAVAILABLE",
                        "reason": "schema outside the generated-decoder subset (fails closed)",
                    }));
                }
            }
            continue;
        }
        let body_bytes = bytes::Bytes::from(serde_json::to_vec(&schema.valid).unwrap());
        let in_bytes = body_bytes.len() as u64;
        for (cand_idx, cand) in CANDS.iter().enumerate() {
            let id_base = (schema_idx * 100 + cand_idx) as u64 * 1_000_000;
            let expected = expected_output(schema, *cand, &body_bytes);
            // correctness pass (untimed rows are never recorded)
            let probe = invoke(
                rt.handle(),
                &mut engine,
                schema,
                *cand,
                &body_bytes,
                id_base,
                alloc_fn,
            );
            if !outcome_ok(&probe.outcome, &expected) {
                summary_cases.push(json!({
                    "case": schema.name,
                    "candidate": cand.name(),
                    "status": "INVALID",
                    "correct": false,
                }));
                continue;
            }
            for w in 0..WARMUP {
                let _ = invoke(
                    rt.handle(),
                    &mut engine,
                    schema,
                    *cand,
                    &body_bytes,
                    id_base + 1 + w as u64,
                    alloc_fn,
                );
            }
            let mut total_us: Vec<f64> = Vec::with_capacity(iters);
            let mut codec_us: Vec<f64> = Vec::with_capacity(iters);
            let mut engine_us: Vec<f64> = Vec::with_capacity(iters);
            let mut cpu_us: Vec<f64> = Vec::with_capacity(iters);
            let mut bridge_us: Vec<f64> = Vec::with_capacity(iters);
            let mut alloc_bytes: Vec<f64> = Vec::with_capacity(iters);
            let mut alloc_calls: Vec<f64> = Vec::with_capacity(iters);
            let mut correct = 0usize;
            let mut last_out_bytes = 0u64;
            for i in 0..iters {
                let s = invoke(
                    rt.handle(),
                    &mut engine,
                    schema,
                    *cand,
                    &body_bytes,
                    id_base + 100_000 + i as u64,
                    alloc_fn,
                );
                let ok = outcome_ok(&s.outcome, &expected);
                if ok {
                    correct += 1;
                }
                let out_bytes = outcome_out_bytes(&s.outcome);
                last_out_bytes = out_bytes;
                let _ = writeln!(
                    raw_file,
                    "{}",
                    json!({
                        "runId": run_id,
                        "case": schema.name,
                        "candidate": cand.name(),
                        "i": i,
                        "ok": ok,
                        "inBytes": in_bytes,
                        "outBytes": out_bytes,
                        "us": s.total_us,
                        "totalUs": s.total_us,
                        "codecUs": s.codec_us,
                        "engineUs": s.engine_us,
                        "cpuUserUs": s.cpu_user_us,
                        "cpuSystemUs": s.cpu_system_us,
                        "cpuUs": s.cpu_user_us + s.cpu_system_us,
                        "bridgeAccessUs": s.bridge_access_ns as f64 / 1000.0,
                        "bridgeHostCalls": s.bridge_host_calls,
                        "bridgeMaterializedFields": s.bridge_materialized_fields,
                        "bridgeMaterializedBytes": s.bridge_materialized_bytes,
                        "allocMallocCalls": s.alloc.map(|a| a.malloc_calls),
                        "allocCallocCalls": s.alloc.map(|a| a.calloc_calls),
                        "allocReallocCalls": s.alloc.map(|a| a.realloc_calls),
                        "allocFreeCalls": s.alloc.map(|a| a.free_calls),
                        "allocAllocatedBytes": s.alloc.map(|a| a.allocated_bytes),
                        "allocReallocatedBytes": s.alloc.map(|a| a.reallocated_bytes),
                    })
                );
                total_us.push(s.total_us);
                codec_us.push(s.codec_us);
                engine_us.push(s.engine_us);
                cpu_us.push((s.cpu_user_us + s.cpu_system_us) as f64);
                bridge_us.push(s.bridge_access_ns as f64 / 1000.0);
                alloc_bytes.push(s.alloc.map(|a| a.allocated_bytes).unwrap_or(0) as f64);
                alloc_calls.push(s.alloc.map(|a| a.call_count()).unwrap_or(0) as f64);
            }
            let all_ok = correct == iters && last_out_bytes > 0;
            summary_cases.push(json!({
                "case": schema.name,
                "candidate": cand.name(),
                "status": if all_ok { "OK" } else { "INVALID" },
                "samples": total_us.len(),
                "correct": correct,
                "inBytes": in_bytes,
                "outBytes": last_out_bytes,
                "metrics": {
                    "totalUs": metric_stats(&mut total_us),
                    "codecUs": metric_stats(&mut codec_us),
                    "engineUs": metric_stats(&mut engine_us),
                    "cpuUs": metric_stats(&mut cpu_us),
                    "bridgeAccessUs": metric_stats(&mut bridge_us),
                    "allocAllocatedBytes": metric_stats(&mut alloc_bytes),
                    "allocCalls": metric_stats(&mut alloc_calls),
                },
            }));
        }
    }

    let summary = json!({
        "format": "velqu-codec-bench-v2",
        "runId": run_id,
        "engine": "quickjs-ng/0.15.1",
        "iters": iters,
        "warmup": WARMUP,
        "packet": PACKET,
        "corpusCases": corpus.iter().map(|s| s.name).collect::<Vec<_>>(),
        "generatedModule": GENERATED_MODULE_PATH,
        "instrumentation": {
            "cpu": "getrusage(RUSAGE_SELF) deltas per sample; perf_event_paranoid=4 denies hardware counters on this host",
            "bridgeTiming": if cfg!(feature = "bench-instrumentation") { "enabled (q-bridge access timing compiled in)" } else { "absent (bench-instrumentation feature off; bridgeAccessUs rows are 0)" },
            "allocator": allocator_status,
            "allocatorMetric": "allocator events and requested bytes, not live heap",
        },
        "prototype": "generated-schema is a fused decode/validate projection over the serde_json parse; the direct byte scanner/decoder is M25-003/M25-004 work and is NOT measured here",
        "fairness": [
            "quickjs-json performs no schema validation; host candidates fully validate inputs (that asymmetry is the strategy question)",
            "generated-schema shares generic-rust's serde_json parse and QuickJS boundary; the delta isolates validation/projection only",
            "response pairing mirrors current production defaults (engine stringify for quickjs-json, native traversal for host candidates)",
            "cpuUs includes all threads (QuickJS worker + tokio) via RUSAGE_SELF; no strategy selection (M25-002-D)",
        ],
        "cases": summary_cases,
    });
    std::fs::write(
        format!("{out_dir}/codec-summary.json"),
        format!("{summary}\n"),
    )
    .expect("summary out");

    let exe_path =
        std::env::current_exe().unwrap_or_else(|_| "target/release/q-codec-bench".into());
    let exe_hash = std::fs::read(&exe_path)
        .map(|b| sha256_hex(&b))
        .unwrap_or_default();
    let module_hash = sha256_hex(include_str!("generated.rs").as_bytes());
    let raw_hash = sha256_hex(&std::fs::read(format!("{out_dir}/codec.jsonl")).expect("raw read"));
    let sum_hash =
        sha256_hex(&std::fs::read(format!("{out_dir}/codec-summary.json")).expect("summary read"));
    let mut evidence_files = vec![
        json!({ "path": "benchmarks/raw/codec-c/codec.jsonl", "sha256": raw_hash }),
        json!({ "path": "benchmarks/raw/codec-c/codec-summary.json", "sha256": sum_hash }),
    ];
    // Final process-wide allocator profile (written by the tracer at exit, so
    // it cannot be hashed here); record its path relative to the repo when
    // possible so committed evidence carries no absolute worktree prefix.
    if let Ok(profile) = std::env::var("VELQU_ALLOC_PROFILE") {
        let cwd = std::env::current_dir().unwrap_or_default();
        let path = std::path::Path::new(&profile)
            .strip_prefix(&cwd)
            .map(|p| p.to_string_lossy().into_owned())
            .unwrap_or(profile);
        evidence_files.push(json!({ "path": path, "sha256": "written-at-tracer-exit" }));
    }
    let evidence = json!({
        "format": "velqu-codec-evidence-v2",
        "runId": run_id,
        "command": COMMAND,
        "exePath": exe_path,
        "binary": { "path": "target/release/q-codec-bench", "sha256": exe_hash },
        "generatedModule": { "path": GENERATED_MODULE_PATH, "sha256": module_hash },
        "allocatorStatus": allocator_status,
        "files": evidence_files,
    });
    std::fs::write(format!("{out_dir}/evidence.json"), format!("{evidence}\n"))
        .expect("evidence out");

    println!("codec bench complete: {out_dir}/codec.jsonl + codec-summary.json + evidence.json");
    engine.shutdown();
}

#[allow(clippy::too_many_arguments)]
fn invoke(
    rt_handle: &tokio::runtime::Handle,
    engine: &mut QuickJsEngine,
    schema: &schemas::BenchSchema,
    cand: Cand,
    body_bytes: &bytes::Bytes,
    id: u64,
    alloc_fn: Option<unsafe extern "C" fn(*mut AllocCounters)>,
) -> Sample {
    let take_alloc = move || {
        alloc_fn.map(|f| {
            let mut c = AllocCounters::default();
            unsafe { f(&mut c) };
            c
        })
    };
    let t0 = Instant::now();
    let (cpu0_user, cpu0_sys) = cpu_time_us();
    let alloc0 = take_alloc();
    let bridge0 = engine.bridge_snapshot();

    let t_codec = Instant::now();
    let mut validated: Option<Value> = None;
    match cand {
        Cand::QuickJsJson => {}
        Cand::GenericRust => {
            let parsed: Value = serde_json::from_slice(body_bytes).expect("fixture parses");
            let v = q_schema_runtime::validate(&schema.ir, &parsed, q_schema_runtime::Source::Body)
                .expect("fixture validates");
            validated = Some(v);
        }
        Cand::GeneratedSchema => {
            let parsed: Value = serde_json::from_slice(body_bytes).expect("fixture parses");
            let v = generated::decode(schema.name, &parsed)
                .expect("schema has a generated decoder")
                .expect("fixture decodes");
            validated = Some(v);
        }
    }
    let codec_us = t_codec.elapsed().as_secs_f64() * 1e6;

    let spec = InvocationSpec {
        id,
        request_id: format!("codec-bench-{id}"),
        route_id: schema.name.into(),
        route_id_num: None,
        handler_key: cand.handler().into(),
        policy_key: None,
        handler_id: None,
        policy_id_num: None,
        policy_handler_id: None,
        params_schema_id: None,
        query_schema_id: None,
        headers_schema_id: None,
        body_schema_id: None,
        request: Some(q_engine::RequestMeta {
            body: Some(body_bytes.clone()),
            ..Default::default()
        }),
        slot: 0,
        generation: 0,
        params: None,
        query: None,
        headers: None,
        body: validated,
        allowed_statuses: vec![200],
        default_status: 200,
        response_strategy: cand.strategy(),
        raw_response: false,
        deadline: Instant::now() + std::time::Duration::from_millis(1000),
    };
    let t_engine = Instant::now();
    let (tx, rx) = tokio::sync::oneshot::channel();
    engine.invoke(spec, tx);
    let outcome = rt_handle
        .block_on(async { tokio::time::timeout(std::time::Duration::from_millis(1500), rx).await })
        .ok()
        .and_then(|r| r.ok());
    let engine_us = t_engine.elapsed().as_secs_f64() * 1e6;

    let bridge1 = engine.bridge_snapshot();
    let alloc1 = take_alloc();
    let (cpu1_user, cpu1_sys) = cpu_time_us();
    Sample {
        total_us: t0.elapsed().as_secs_f64() * 1e6,
        codec_us,
        engine_us,
        cpu_user_us: cpu1_user.saturating_sub(cpu0_user),
        cpu_system_us: cpu1_sys.saturating_sub(cpu0_sys),
        #[cfg(feature = "bench-instrumentation")]
        bridge_access_ns: bridge1
            .access_time_ns
            .saturating_sub(bridge0.access_time_ns),
        #[cfg(not(feature = "bench-instrumentation"))]
        bridge_access_ns: 0,
        bridge_host_calls: bridge1.host_calls.saturating_sub(bridge0.host_calls),
        bridge_materialized_fields: bridge1
            .materialized_fields
            .saturating_sub(bridge0.materialized_fields),
        bridge_materialized_bytes: bridge1
            .materialized_bytes
            .saturating_sub(bridge0.materialized_bytes),
        alloc: alloc0.zip(alloc1).map(|(a, b)| AllocCounters::delta(a, b)),
        outcome,
    }
}

fn expected_output(schema: &schemas::BenchSchema, cand: Cand, body_bytes: &bytes::Bytes) -> Value {
    if cand == Cand::QuickJsJson {
        return schema.valid.clone();
    }
    let parsed: Value = serde_json::from_slice(body_bytes).expect("fixture parses");
    match cand {
        Cand::QuickJsJson => unreachable!(),
        Cand::GenericRust => {
            q_schema_runtime::validate(&schema.ir, &parsed, q_schema_runtime::Source::Body)
                .expect("fixture validates")
        }
        Cand::GeneratedSchema => generated::decode(schema.name, &parsed)
            .expect("schema has a generated decoder")
            .expect("fixture decodes"),
    }
}

fn outcome_ok(outcome: &Option<Outcome>, expected: &Value) -> bool {
    match outcome {
        Some(Outcome::Response {
            body: BodyOut::Json(v),
            ..
        }) => v == expected,
        Some(Outcome::Response {
            body: BodyOut::JsonText(t),
            ..
        }) => serde_json::from_str::<Value>(t)
            .map(|v| &v == expected)
            .unwrap_or(false),
        _ => false,
    }
}

fn outcome_out_bytes(outcome: &Option<Outcome>) -> u64 {
    match outcome {
        Some(Outcome::Response {
            body: BodyOut::Json(v),
            ..
        }) => serde_json::to_vec(v).map(|b| b.len() as u64).unwrap_or(0),
        Some(Outcome::Response {
            body: BodyOut::JsonText(t),
            ..
        }) => t.len() as u64,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use q_schema_runtime::{validate, SchemaIr, Source};
    use serde_json::json;

    fn probes() -> Vec<SchemaIr> {
        vec![
            SchemaIr::Boolean,
            SchemaIr::String {
                min_length: None,
                max_length: Some(3),
                pattern: None,
                format: None,
            },
            SchemaIr::String {
                min_length: None,
                max_length: None,
                pattern: Some("^a$".into()),
                format: None,
            },
            SchemaIr::String {
                min_length: None,
                max_length: None,
                pattern: None,
                format: Some("email".into()),
            },
            SchemaIr::Integer {
                minimum: None,
                maximum: None,
            },
            SchemaIr::Number {
                minimum: None,
                maximum: None,
            },
            SchemaIr::Literal { value: json!(1) },
            SchemaIr::Enum {
                values: vec![json!(1)],
            },
            SchemaIr::Optional {
                inner: Box::new(SchemaIr::Boolean),
                default: None,
            },
            SchemaIr::Nullable {
                inner: Box::new(SchemaIr::Boolean),
            },
            SchemaIr::Array {
                items: Box::new(SchemaIr::Boolean),
                min_items: None,
                max_items: None,
            },
            SchemaIr::Object {
                properties: std::collections::BTreeMap::new(),
                required: vec![],
            },
            SchemaIr::Transform {
                input: Box::new(SchemaIr::Boolean),
                output: Box::new(SchemaIr::Boolean),
                name: "x".into(),
            },
            SchemaIr::File {
                content_type: None,
                max_bytes: 1,
            },
            SchemaIr::Problem {
                type_uri: None,
                title: "t".into(),
                status: 400,
                detail: None,
            },
            SchemaIr::Fallback {
                reason: "explicit".into(),
                inner: None,
            },
            SchemaIr::Fallback {
                reason: "explicit".into(),
                inner: Some(Box::new(SchemaIr::Boolean)),
            },
            SchemaIr::Union {
                members: vec![Box::new(SchemaIr::Boolean)],
            },
        ]
    }

    #[test]
    fn generated_source_is_current() {
        assert_eq!(
            include_str!("generated.rs"),
            generator::generate(&schemas::corpus()),
            "generated.rs is stale: regenerate with --emit-generated"
        );
    }

    #[test]
    fn corpus_is_supported_by_generated_decoder() {
        for s in schemas::corpus() {
            assert!(generated::supports(&s.ir), "schema {}", s.name);
        }
    }

    #[test]
    fn generated_supports_matches_generator_guard() {
        for ir in probes() {
            assert_eq!(
                generated::supports(&ir),
                generator::schema_supported(&ir),
                "probe {ir:?}"
            );
        }
    }

    #[test]
    fn unsupported_schemas_fail_closed_in_supports() {
        for ir in probes() {
            let supported = generated::supports(&ir);
            match &ir {
                SchemaIr::Boolean
                | SchemaIr::String {
                    pattern: None,
                    format: None,
                    ..
                }
                | SchemaIr::Integer { .. }
                | SchemaIr::Number { .. }
                | SchemaIr::Optional { .. }
                | SchemaIr::Nullable { .. }
                | SchemaIr::Array { .. }
                | SchemaIr::Object { .. }
                | SchemaIr::Fallback { inner: Some(_), .. } => assert!(supported, "{ir:?}"),
                _ => assert!(!supported, "{ir:?}"),
            }
        }
    }

    #[test]
    fn differential_decode_matches_generic_validator() {
        for s in schemas::corpus() {
            let generated_ok = generated::decode(s.name, &s.valid).unwrap();
            let generic = validate(&s.ir, &s.valid, Source::Body);
            assert_eq!(generated_ok, generic, "{} valid fixture", s.name);

            for (label, value) in schemas::invalid_cases(s.name) {
                let generated_res = generated::decode(s.name, &value).unwrap();
                let generic_res = validate(&s.ir, &value, Source::Body);
                assert_eq!(generated_res, generic_res, "{} {}", s.name, label);
            }
        }
    }

    #[test]
    fn fallback_without_inner_stays_unavailable() {
        let ir = SchemaIr::Object {
            properties: std::collections::BTreeMap::from([(
                "x".to_string(),
                Box::new(SchemaIr::Fallback {
                    reason: "measured".into(),
                    inner: None,
                }),
            )]),
            required: vec![],
        };
        assert!(!generated::supports(&ir));
        // and the generic validator fails closed on it, so parity holds
        let generic = validate(&ir, &json!({"x": 1}), Source::Body);
        assert!(generic.is_err());
    }
}
