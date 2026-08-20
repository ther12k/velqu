//! q-runtime — the production host binary. Composes pack verification, the
//! native router, the single QuickJS worker, and the HTTP layer.
//!
//! Startup stages (all timed and logged):
//!   pack.load → router.build → engine.spawn → bundle.load → listen → ready
//! No route/schema/OpenAPI/TS compilation happens here (G-004).

mod problems;
mod serve;
mod source_map;

use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;

use clap::Parser;

use q_engine::Engine as _;
use q_engine_quickjs::{QuickJsConfig, QuickJsEngine};
use q_http::{HttpHost, Limits};
use q_pack::QPack;

#[derive(Debug, Parser)]
#[command(
    name = "velqu-runtime",
    about = "VelquJS production host (Rust + QuickJS)"
)]
struct Args {
    /// Path to the application pack (velqu.qpack v1)
    #[arg(long)]
    pack: PathBuf,
    /// TCP port (default 3000, or PORT env)
    #[arg(long)]
    port: Option<u16>,
    #[arg(long, default_value = "127.0.0.1")]
    host: String,
    /// Optional limits/config JSON overriding defaults
    #[arg(long)]
    config: Option<PathBuf>,
    /// Request logging mode: off | errors | full (default: errors)
    #[arg(long, default_value = "errors")]
    log: String,
}

#[allow(clippy::needless_return)]
fn main() {
    let args = Args::parse();
    let code = run(args);
    std::process::exit(code);
}

fn run(args: Args) -> i32 {
    let t0 = Instant::now();
    let mut stages: Vec<(String, f64)> = Vec::new();

    // ---- stage: pack.load (verify integrity/versions before ANYTHING else)
    let t = Instant::now();
    let pack = match QPack::load_and_verify(&args.pack) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "{}",
                serde_json::json!({
                    "level": "error", "event": "startup.rejected",
                    "stage": "pack.load", "error": e.to_string(),
                })
            );
            return 2;
        }
    };
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
    if let Some(cfg_path) = &args.config {
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

    let port = args
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
            ..Default::default()
        };
        let mut engine = QuickJsEngine::spawn(
            config,
            tokio::runtime::Handle::current(),
            mapper,
        );
        stages.push(("engine.spawn".into(), t.elapsed().as_secs_f64() * 1000.0));

        let t = Instant::now();
        // ADR-0017: if the pack carries verified bytecode, skip source eval
        let bytecode_decoded: Option<Vec<u8>> = pack
            .bundle_bytecode
            .as_ref()
            .and_then(|bc| q_pack::base64_decode(&bc.data));
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
        let addr = format!("{}:{}", args.host, port);
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

        let startup_line = serde_json::json!({
            "level": "info",
            "event": "ready",
            "appId": pack.app_id,
            "routes": pack.routes.len(),
            "handlers": load_stats.handlers_registered,
            "addr": addr,
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
        let schema_vector: Vec<q_schema_runtime::SchemaIr> = pack
            .schema_manifest
            .iter()
            .map(|s| s.ir.clone())
            .collect();
        let state = Arc::new(serve::ServeState {
            pack: Arc::new(pack),
            router,
            schema_vector,
            engine: std::sync::Mutex::new(engine),
            health,
            invocation_clock: std::sync::atomic::AtomicU64::new(1),
            log_mode: serve::LogMode::from_str(&args.log),
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
            });
            println!("{done}");
        }
        0
    })
}
