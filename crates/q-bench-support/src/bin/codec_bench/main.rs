//! M25-002-A/B codec strategy benchmark: compares three JSON input→handler→
//! output strategies across the frozen payload matrix inside a controlled host
//! process (no network).
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
//!   - No CPU/allocation capture here (M25-002-C); no strategy is selected
//!     here (M25-002-D).
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
const COMMAND: &str = "./target/release/q-codec-bench --out-dir benchmarks/raw/codec --iters 2000";
const PACKET: &str = "M25-002-B";

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
        "m25-002-b-{}",
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
            let (_, outcome) = invoke(
                rt.handle(),
                &mut engine,
                schema,
                *cand,
                &body_bytes,
                id_base,
            );
            if !outcome_ok(&outcome, &expected) {
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
                );
            }
            let mut durations: Vec<f64> = Vec::with_capacity(iters);
            let mut correct = 0usize;
            let mut last_out_bytes = 0u64;
            for i in 0..iters {
                let (us, outcome) = invoke(
                    rt.handle(),
                    &mut engine,
                    schema,
                    *cand,
                    &body_bytes,
                    id_base + 100_000 + i as u64,
                );
                let ok = outcome_ok(&outcome, &expected);
                if ok {
                    correct += 1;
                }
                let out_bytes = outcome_out_bytes(&outcome);
                last_out_bytes = out_bytes;
                let _ = writeln!(
                    raw_file,
                    "{}",
                    json!({
                        "runId": run_id,
                        "case": schema.name,
                        "candidate": cand.name(),
                        "i": i,
                        "us": us,
                        "ok": ok,
                        "inBytes": in_bytes,
                        "outBytes": out_bytes,
                    })
                );
                durations.push(us);
            }
            durations.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let p = |q: f64| durations[((q * (durations.len() - 1) as f64).round()) as usize];
            let all_ok = correct == iters && last_out_bytes > 0;
            summary_cases.push(json!({
                "case": schema.name,
                "candidate": cand.name(),
                "status": if all_ok { "OK" } else { "INVALID" },
                "samples": durations.len(),
                "correct": correct,
                "mean_us": durations.iter().sum::<f64>() / durations.len() as f64,
                "p50_us": p(0.50),
                "p95_us": p(0.95),
                "p99_us": p(0.99),
                "inBytes": in_bytes,
                "outBytes": last_out_bytes,
            }));
        }
    }

    let summary = json!({
        "format": "velqu-codec-bench-v1",
        "runId": run_id,
        "engine": "quickjs-ng/0.15.1",
        "iters": iters,
        "warmup": WARMUP,
        "packet": PACKET,
        "corpusCases": corpus.iter().map(|s| s.name).collect::<Vec<_>>(),
        "generatedModule": GENERATED_MODULE_PATH,
        "prototype": "generated-schema is a fused decode/validate projection over the serde_json parse; the direct byte scanner/decoder is M25-003/M25-004 work and is NOT measured here",
        "fairness": [
            "quickjs-json performs no schema validation; host candidates fully validate inputs (that asymmetry is the strategy question)",
            "generated-schema shares generic-rust's serde_json parse and QuickJS boundary; the delta isolates validation/projection only",
            "response pairing mirrors current production defaults (engine stringify for quickjs-json, native traversal for host candidates)",
            "no CPU/allocation capture in this packet (M25-002-C); no strategy selection (M25-002-D)",
        ],
        "cases": summary_cases,
    });
    std::fs::write(
        format!("{out_dir}/codec-summary.json"),
        format!("{summary}\n"),
    )
    .expect("summary out");

    let exe_hash = std::fs::read("/proc/self/exe")
        .map(|b| sha256_hex(&b))
        .unwrap_or_default();
    let module_hash = sha256_hex(include_str!("generated.rs").as_bytes());
    let raw_hash = sha256_hex(&std::fs::read(format!("{out_dir}/codec.jsonl")).expect("raw read"));
    let sum_hash =
        sha256_hex(&std::fs::read(format!("{out_dir}/codec-summary.json")).expect("summary read"));
    let evidence = json!({
        "format": "velqu-codec-evidence-v1",
        "runId": run_id,
        "command": COMMAND,
        "binary": { "path": "target/release/q-codec-bench", "sha256": exe_hash },
        "generatedModule": { "path": GENERATED_MODULE_PATH, "sha256": module_hash },
        "files": [
            { "path": "benchmarks/raw/codec/codec.jsonl", "sha256": raw_hash },
            { "path": "benchmarks/raw/codec/codec-summary.json", "sha256": sum_hash },
        ],
    });
    std::fs::write(format!("{out_dir}/evidence.json"), format!("{evidence}\n"))
        .expect("evidence out");

    println!("codec bench complete: {out_dir}/codec.jsonl + codec-summary.json + evidence.json");
    engine.shutdown();
}

/// One timed strategy execution. The timed region covers the host-side parse
/// and validation/projection for the host candidates (plus the invoke→outcome
/// round trip); for quickjs-json the parse happens inside the engine.
#[allow(clippy::too_many_arguments)]
fn invoke(
    rt_handle: &tokio::runtime::Handle,
    engine: &mut QuickJsEngine,
    schema: &schemas::BenchSchema,
    cand: Cand,
    body_bytes: &bytes::Bytes,
    id: u64,
) -> (f64, Option<Outcome>) {
    let t0 = Instant::now();
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
        deadline: Instant::now() + std::time::Duration::from_millis(1000),
    };
    let (tx, rx) = tokio::sync::oneshot::channel();
    engine.invoke(spec, tx);
    let outcome = rt_handle
        .block_on(async { tokio::time::timeout(std::time::Duration::from_millis(1500), rx).await })
        .ok()
        .and_then(|r| r.ok());
    let d = t0.elapsed().as_secs_f64() * 1e6;
    (d, outcome)
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
