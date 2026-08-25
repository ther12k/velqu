//! velqu-runtime library surface — shared startup pipeline for both
//! deployment modes (M26-009):
//!
//! - **shared**: `velqu-runtime --pack app.qpack` (small app updates)
//! - **standalone** (`velqu-standalone`, feature `standalone`): one
//!   executable with the verified pack embedded at compile time.
//!
//! Both modes run the IDENTICAL load-verify-serve pipeline below; they
//! differ only in where the pack bytes come from.

pub mod problems;
pub mod serve;
pub mod source_map;

use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;

use q_engine::Engine as _;
use q_engine_quickjs::{QuickJsConfig, QuickJsEngine};
use q_http::{HttpHost, Limits};
use q_pack::QPack;

/// Where the application pack comes from. Mode dispatch is explicit:
/// no hidden fallback between sources (ADR-0024 policy).
pub enum PackSource {
    /// Shared mode: read + verify from disk.
    Path(PathBuf),
    /// Standalone mode: pack embedded in the executable at compile time
    /// (already the exact artifact; still fully re-verified at startup).
    Embedded(&'static [u8]),
}

/// Runtime configuration shared by both binaries (CLI flags map 1:1).
#[derive(Debug, Clone)]
pub struct RunConfig {
    pub port: Option<u16>,
    pub host: String,
    pub config: Option<PathBuf>,
    pub log: String,
    pub log_sample: u64,
    /// M26-002-C: explicit source-rebuild recovery path.
    pub no_bytecode: bool,
}

/// Run the server to completion; returns the process exit code.
#[allow(clippy::needless_return)]
pub fn run(source: PackSource, cfg: RunConfig) -> i32 {
    let t0 = Instant::now();
    let mut stages: Vec<(String, f64)> = Vec::new();

    // ---- stage: pack.load (verify integrity/versions before ANYTHING else)
    let t = Instant::now();
    let policy = if cfg.no_bytecode {
        q_pack::BytecodePolicy::Skip
    } else {
        q_pack::BytecodePolicy::Enforce
    };
    let mut pack = match &source {
        PackSource::Path(path) => QPack::load_and_verify_with(path, policy),
        PackSource::Embedded(bytes) => QPack::verify_from_slice(bytes, policy),
    }
    .unwrap_or_else(|e| {
        eprintln!(
            "{}",
            serde_json::json!({
                "level": "error", "event": "startup.rejected",
                "stage": "pack.load", "error": e.to_string(),
            })
        );
        std::process::exit(2);
    });
    stages.push(("pack.load".into(), t.elapsed().as_secs_f64() * 1000.0));

    // ---- stage: router.build (consume pre-compiled automaton or segments; no runtime path parsing)
    let t = Instant::now();
    let router = match q_router::Router::from_pack(&pack) {
        Ok(r) => r,
        Err(e) => {
            eprintln!(
                "{}",
                serde_json::json!({"level":"error","event":"startup.rejected","stage":"router.build","error":e.to_string()})
            );
            return 2;
        }
    };
    stages.push(("router.build".into(), t.elapsed().as_secs_f64() * 1000.0));

    // ---- limits (config file may override)
    let mut limits = Limits::default();
    if let Some(cfg_path) = &cfg.config {
        match std::fs::read_to_string(cfg_path) {
            Ok(text) => match serde_json::from_str::<serde_json::Value>(&text) {
                Ok(v) => {
                    if let Some(b) = v.get("maxBodyBytes").and_then(|x| x.as_u64()) {
                        limits.max_body_bytes = b as usize;
                    }
                    if let Some(q) = v.get("maxQueue").and_then(|x| x.as_u64()) {
                        limits.max_queue = q as usize;
                    }
                }
                Err(e) => {
                    eprintln!("config parse error: {e}");
                    return 2;
                }
            },
            Err(e) => {
                eprintln!("config read error: {e}");
                return 2;
            }
        }
    }

    let port = cfg
        .port
        .or_else(|| std::env::var("PORT").ok().and_then(|p| p.parse().ok()))
        .unwrap_or(3000);

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(2)
        .build()
        .expect("tokio runtime");

    rt.block_on(async move {
        // ---- stage: engine.spawn + bundle.load (one QuickJS worker)
        let t = Instant::now();
        let mapper = source_map::mapper_for(&pack);
        let config = QuickJsConfig {
            request_slot_capacity: limits.max_queue.max(1),
            // M26-004-D: embedded-prelude bytecode skips host prelude eval;
            // the explicit source path always evaluates it.
            embedded_prelude: pack.bundle_prelude.as_deref() == Some("embedded")
                && !cfg.no_bytecode,
            ..Default::default()
        };
        let mut engine =
            QuickJsEngine::spawn(config, tokio::runtime::Handle::current(), mapper);
        stages.push(("engine.spawn".into(), t.elapsed().as_secs_f64() * 1000.0));

        let t = Instant::now();
        // ADR-0017: if the pack carries verified bytecode, skip source eval.
        // M26-004-B: the single base64 decode already happened inside
        // verify (hash + handoff share one buffer); take it, no re-decode.
        let bytecode_decoded: Option<Vec<u8>> = if cfg.no_bytecode {
            None
        } else {
            pack.decoded_bytecode.take()
        };
        let load_plan = if !pack.functions.is_empty() {
            q_engine::EngineLoadPlan::Numeric {
                functions: pack.functions.clone(),
            }
        } else {
            q_engine::EngineLoadPlan::Legacy {
                expected_handlers: pack.handler_table.clone(),
            }
        };
        let load_stats = match engine.load(&pack.bundle, bytecode_decoded.as_deref(), load_plan) {
            Ok(s) => s,
            Err(e) => {
                eprintln!(
                    "{}",
                    serde_json::json!({"level":"error","event":"startup.rejected","stage":"bundle.load","error":e})
                );
                return 3;
            }
        };
        stages.push(("bundle.load".into(), t.elapsed().as_secs_f64() * 1000.0));

        // ---- stage: listen
        let t = Instant::now();
        let addr = format!("{}:{}", cfg.host, port);
        let listener = match tokio::net::TcpListener::bind(&addr).await {
            Ok(l) => l,
            Err(e) => {
                eprintln!(
                    "{}",
                    serde_json::json!({"level":"error","event":"startup.rejected","stage":"listen","error":e.to_string()})
                );
                return 2;
            }
        };
        stages.push(("listen".into(), t.elapsed().as_secs_f64() * 1000.0));

        let host = HttpHost::new(limits);
        let (shutdown_tx, shutdown_rx) = tokio::sync::watch::channel(false);

        // SIGTERM/SIGINT → graceful shutdown (RUN-008)
        let sig_tx = shutdown_tx.clone();
        tokio::spawn(async move {
            #[cfg(unix)]
            {
                let mut term = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
                    .expect("SIGTERM handler");
                let mut int = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::interrupt())
                    .expect("SIGINT handler");
                tokio::select! {
                    _ = term.recv() => {}
                    _ = int.recv() => {}
                }
            }
            let _ = sig_tx.send(true);
        });

        let mode = match source {
            PackSource::Path(_) => "shared",
            PackSource::Embedded(_) => "standalone",
        };
        let startup_line = serde_json::json!({
            "level": "info",
            "event": "ready",
            "appId": pack.app_id,
            "routes": pack.routes.len(),
            "handlers": load_stats.handlers_registered,
            "addr": addr,
            "mode": mode,
            "engine": format!("{}/{}", q_pack::ENGINE_NAME, q_pack::ENGINE_VERSION),
            "runtimeAbi": q_pack::RUNTIME_ABI,
            "contractHash": pack.contract_hash,
            "startupMs": t0.elapsed().as_secs_f64() * 1000.0,
            "stages": stages.iter()
                .map(|(k, v)| serde_json::json!({"stage": k, "ms": v}))
                .collect::<Vec<_>>(),
            "bundleEvalMs": load_stats.eval_ms,
        });
        println!("{startup_line}");

        let health = engine.health();
        // Dense schema vector indexed by SchemaId (verification guarantees the
        // manifest is dense and complete whenever the pack carries schemas)
        let schema_vector: Vec<q_schema_runtime::SchemaIr> =
            pack.schema_manifest.iter().map(|s| s.ir.clone()).collect();
        let decoder_table = q_schema_runtime::DecoderTable::from_schemas(&schema_vector);
        // M25-005-A: direct response encoders from the same dense schema
        // vector, plus each route's declared response statuses resolved to
        // SchemaIds once at startup (production startup performs zero
        // compilation — both tables are precompiled programs keyed by id).
        let encoder_table = q_schema_runtime::EncoderTable::from_schemas(&schema_vector);
        let schema_id_by_key: std::collections::BTreeMap<&str, u32> = pack
            .schema_manifest
            .iter()
            .map(|s| (s.key.as_str(), s.id))
            .collect();
        let response_schema_ids: Vec<std::collections::BTreeMap<u16, u32>> = pack
            .routes
            .iter()
            .map(|route| {
                route
                    .responses
                    .iter()
                    .filter_map(|(status, decl)| {
                        let sid = decl.schema.as_ref().and_then(|k| schema_id_by_key.get(k.as_str()))?;
                        let status: u16 = status.parse().ok()?;
                        Some((status, *sid))
                    })
                    .collect()
            })
            .collect();
        let state = Arc::new(serve::ServeState {
            pack: Arc::new(pack),
            router,
            schema_vector,
            decoder_table,
            encoder_table,
            response_schema_ids,
            engine: std::sync::Mutex::new(engine),
            health,
            invocation_clock: std::sync::atomic::AtomicU64::new(1),
            log_mode: serve::LogMode::parse_mode(&cfg.log),
            log_sample: cfg.log_sample,
            log_sequence: std::sync::atomic::AtomicU64::new(0),
            metrics: std::sync::Arc::new(serve::StageMetrics::default()),
        });
        let handler = serve::make_handler(Arc::clone(&state));
        let _ = host.serve(listener, handler, shutdown_rx).await;

        // deterministic engine teardown after connections drain
        {
            let mut eng = state.engine.lock().unwrap();
            eng.shutdown();
            let done = serde_json::json!({
                "level": "info",
                "event": "shutdown.complete",
                "stats": eng.stats(),
                "stageMetrics": state.metrics.snapshot(),
            });
            println!("{done}");
        }
        0
    })
}
