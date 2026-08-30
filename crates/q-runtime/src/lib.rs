//! velqu-runtime library surface — shared startup pipeline for both
//! deployment modes (M26-009):
//!
//! - **shared**: `velqu-runtime --pack app.qpack` (small app updates)
//! - **standalone** (`velqu-standalone`, feature `standalone`): one
//!   executable with the verified pack embedded at compile time.
//!
//! Both modes run the IDENTICAL load-verify-serve pipeline below; they
//! differ only in where the pack bytes come from.

pub mod fetch_stack;
pub mod problems;
pub mod serve;
pub mod service_profile;
pub use service_profile::{
    aggregate_readiness, startup_batches, startup_parallelism, AdaptiveWorkers, FleetReadiness,
    Readiness, ScaleThresholds, ScaleTick, ServiceProfile, WorkerBounds, MAX_STARTUP_PARALLELISM,
    MAX_WORKERS, MIN_WORKERS, WORKER_INIT_DEADLINE_MS,
};
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
    /// M27-003-D: explicit context-profile override for compatibility
    /// testing. `None` keeps the engine default (full) — the full
    /// profile is always available and is the default; a value here
    /// forces that exact profile. Unknown names fail closed at
    /// startup (before serving), never fall back silently.
    pub context_profile: Option<String>,
    /// M3-003-A/D: worker startup posture — `serverless` (default, one
    /// worker, ready immediately) or `service:N` (N workers, ready when
    /// all N are initialized). Unknown names fail closed at startup.
    pub service_profile: Option<String>,
}

/// M26-009-C: print the runtime's exact fingerprint; when a pack is
/// available, run the FULL verification (every fingerprint dimension,
/// integrity, bytecode target) WITHOUT serving, and print the verdict.
/// Exit 0 = compatible/verified; exit 2 = rejected. Identical behavior
/// for both deployment modes (the pack source differs, nothing else).
pub fn print_fingerprint(source: &PackSource) -> i32 {
    let fp = q_pack::RuntimeFingerprint::current();
    let mut out = serde_json::json!({
        "event": "runtime.fingerprint",
        "runtime": fp,
    });
    // Read the pack bytes ONCE per mode (shared: from disk; standalone:
    // the embedded artifact) so the printed binding hash is exactly the
    // hash of the bytes that were verified — no TOCTOU between verify
    // and hash.
    let (bytes, sidecar_for): (Vec<u8>, &str) = match source {
        PackSource::Path(path) => (
            std::fs::read(path).unwrap_or_else(|e| {
                eprintln!(
                    "{}",
                    serde_json::json!({"level":"error","event":"fingerprint.read","error":e.to_string()})
                );
                std::process::exit(2);
            }),
            "<pack>.sources.json",
        ),
        PackSource::Embedded(bytes) => (bytes.to_vec(), "<executable>.sources.json"),
    };
    match QPack::verify_from_slice(&bytes, q_pack::BytecodePolicy::Enforce) {
        Ok(pack) => {
            out["pack"] = serde_json::json!({
                "appId": pack.app_id,
                "engine": pack.engine,
                "verdict": "compatible",
                // M26-009-D: binding key for this mode's debug sidecar
                "packSha256": q_pack::sources_sidecar::SourcesSidecar::pack_sha256_of(&bytes),
                "sidecar": sidecar_for,
            });
            println!("{out}");
            0
        }
        Err(e) => {
            out["verdict"] = serde_json::json!("rejected");
            out["error"] = serde_json::json!(e.to_string());
            eprintln!("{out}");
            2
        }
    }
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
        // M27-003-D: --context-profile overrides the engine default
        // (full). Unknown values fail closed before the engine spawns.
        let profile_override = cfg
            .context_profile
            .as_deref()
            .map(|s| {
                q_engine_quickjs::ContextProfile::parse(s).ok_or_else(|| {
                    format!(
                        "unknown context profile '{s}' (known: full, web, minimal)"
                    )
                })
            })
            .transpose();
        let profile = match profile_override {
            Ok(p) => p.unwrap_or_default(),
            Err(msg) => {
                let ready = serde_json::json!({
                    "event": "ready",
                    "ok": false,
                    "error": msg,
                });
                eprintln!("{ready}");
                return 2;
            }
        };
        // M3-003-A/D: resolve the service profile BEFORE any worker
        // spawns. Unknown names fail closed here (exit 2), never fall
        // back; serverless stays at exactly one worker.
        let service_profile =
            match cfg.service_profile.as_deref().map(service_profile::ServiceProfile::parse) {
                Some(Err(e)) => {
                    let ready = serde_json::json!({
                        "event": "ready",
                        "ok": false,
                        "error": e,
                    });
                    eprintln!("{ready}");
                    return 2;
                }
                Some(Ok(p)) => p,
                None => service_profile::ServiceProfile::default(),
            };
        let startup_workers = service_profile.initial_workers();
        let config = QuickJsConfig {
            request_slot_capacity: limits.max_queue.max(1),
            // M26-004-D: embedded-prelude bytecode skips host prelude eval;
            // the explicit source path always evaluates it.
            embedded_prelude: pack.bundle_prelude.as_deref() == Some("embedded")
                && !cfg.no_bytecode,
            profile,
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
            // M27-003-V: the serving context's profile identity is part
            // of the startup identity block — every ready line self-
            // describes which intrinsic set produced the measurements.
            "contextProfile": profile.as_str(),
            "serviceProfile": service_profile.as_str(),
            "startupWorkers": startup_workers,
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
            // M3-007-A: single-worker topology today; the multi-worker
            // runtime passes its fleet size here.
            ownership: q_capabilities::InvocationOwnership::new(1),
            // M3-007-B: starts Serving; flipped once by the signal task below.
            drain_gate: q_capabilities::DrainGate::new(),
            log_mode: serve::LogMode::parse_mode(&cfg.log),
            log_sample: cfg.log_sample,
            log_sequence: std::sync::atomic::AtomicU64::new(0),
            metrics: std::sync::Arc::new(serve::StageMetrics::default()),
        });
        let handler = serve::make_handler(Arc::clone(&state));
        // M3-007-B: the drain gate flips the INSTANT the shutdown signal
        // fires — new admissions are refused from that moment (the accept
        // loop also stops, but established keep-alive connections keep
        // being served, and every request they carry re-checks the gate).
        let mut drain_rx = shutdown_rx.clone();
        let drain_state = Arc::clone(&state);
        tokio::spawn(async move {
            let _ = drain_rx.changed().await;
            if *drain_rx.borrow() && drain_state.drain_gate.begin() {
                let ownership = drain_state.ownership.stats();
                println!(
                    "{}",
                    serde_json::json!({
                        "level": "info",
                        "event": "drain.begin",
                        "pending": ownership.pending,
                    })
                );
            }
        });
        // M3-007-C: in-flight connections are ALLOWED to complete — the
        // host awaits them, bounded by the same ADR-0031 shutdown budget
        // used for every other quiescence step.
        let serve_drain = host
            .serve(
                listener,
                handler,
                shutdown_rx,
                std::time::Duration::from_millis(q_capabilities::SHUTDOWN_BUDGET_MS),
            )
            .await;
        let (drain_completed, mut drain_aborted) = match serve_drain {
            Ok(d) => (d.completed, d.aborted),
            Err(_) => (0, 0),
        };
        // M3-007-D: defensive ownership sweep. Every aborted connection's
        // CancelOnDrop guard already settled its binding during the
        // abort; anything still live here would be a leaked cancel path.
        // Settle it and count it as a forced abort — the report never
        // hides an invocation and never shows a phantom orphan.
        for (id, _worker) in state.ownership.snapshot() {
            state.ownership.settle(id);
            drain_aborted += 1;
        }

        // M28-009-C: quiescence includes the outbound pool. Drain the
        // shared fetch pool within the shutdown budget (ADR-0031
        // 5s budget, q_capabilities::SHUTDOWN_BUDGET_MS); an app that
        // never fetched drains as an immediate no-op.
        let fetch_pool = fetch_stack::shared_pool();
        let fetch_pool_drained = fetch_pool
            .drain_shutdown(std::time::Duration::from_millis(5_000))
            .await
            .is_ok();

        // deterministic engine teardown after connections drain
        {
            let mut eng = state.engine.lock().unwrap();
            eng.shutdown();
            // M3-007-A: the shutdown report carries the ownership
            // invariant — a graceful drain leaves ZERO live bindings
            // (no orphan invocation); registered/settled expose whether
            // every admission reached a terminal transition.
            let ownership = state.ownership.stats();
            let done = serde_json::json!({
                "level": "info",
                "event": "shutdown.complete",
                "stats": eng.stats(),
                "stageMetrics": state.metrics.snapshot(),
                "invocations": {
                    "pending": ownership.pending,
                    "registered": ownership.registered,
                    "settled": ownership.settled,
                },
                // M3-007-B/C/D: admissions refused after the drain
                // flip; in-flight connections completed within the
                // budget; stragglers force-aborted at expiry (their
                // invocations cancelled through ownership exactly once —
                // no orphan, by construction and by sweep).
                "drain": {
                    "refused": state.drain_gate.refused(),
                    "completed": drain_completed,
                    "aborted": drain_aborted,
                },
                "fetchPool": {
                    "initialized": fetch_pool.is_initialized(),
                    "drained": fetch_pool_drained,
                },
            });
            println!("{done}");
        }
        0
    })
}
