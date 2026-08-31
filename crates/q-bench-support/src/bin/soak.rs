//! Sustained mixed-load soak (M3-010-A): drives N independent QuickJS
//! runtimes behind the M3-002 bounded Dispatcher with the C2/C3 work
//! mix (light + CPU + controlled 1 ms timer I/O) CONTINUOUSLY for a
//! configured duration, sampling per-window throughput, errors, process
//! RSS, and queue stats — the raw data for the leak analysis and the
//! sustained-stability claim.
//!
//! Duration is parameterized (`--duration-secs`); the committed
//! evidence run's exact duration is recorded in its summary. This is a
//! soak harness, not a perf claim (constraint 12).
//!
//! Output: benchmarks/raw/worker-scaling/soak.jsonl (one line per
//! window) + soak-summary.json (totals + leak analysis).

use std::collections::BTreeMap;
use std::io::Write;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc;
use std::sync::Arc;
use std::time::{Duration, Instant};

use q_engine::{BodyOut, Engine as _, InvocationSpec, Outcome, ResponseStrategy};
use q_engine_quickjs::{IdentityMapper, QuickJsConfig};
use serde_json::{json, Value};

const BUNDLE: &str = r#"
"use strict";
async function cpu_work() {
  const iters = 20000;
  let sum = 0;
  let s = "";
  for (let i = 0; i < iters; i++) {
    sum += (i % 7) * (i % 3) + (i & 1);
    if ((i & 1023) === 0) s += "x";
  }
  return { sum: sum, len: s.length };
}
async function light_work() {
  return { ok: true };
}
async function io_delay(ctx) {
  const waited = await ctx.native.timer.delay(1);
  return { waited: waited };
}
__velquRegister("cpu.work", cpu_work);
__velquRegister("light.work", light_work);
__velquRegister("io.delay", io_delay);
"#;

/// Soak mix: deterministic per-id (consumer verifies every response):
/// 60% light, 25% CPU, 15% controlled 1 ms timer I/O.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    Light,
    Cpu,
    Io,
}

fn kind_for(id: u64) -> Kind {
    match id % 20 {
        0..=11 => Kind::Light, // 60%
        12..=16 => Kind::Cpu,  // 25%
        _ => Kind::Io,         // 15%
    }
}

fn handler_key(kind: Kind) -> &'static str {
    match kind {
        Kind::Light => "light.work",
        Kind::Cpu => "cpu.work",
        Kind::Io => "io.delay",
    }
}

fn expected_cpu() -> (f64, f64) {
    let mut sum = 0i64;
    let mut s = String::new();
    for i in 0..20_000i64 {
        sum += (i % 7) * (i % 3) + (i & 1);
        if (i & 1023) == 0 {
            s.push('x');
        }
    }
    (sum as f64, s.len() as f64)
}

enum ConsumerMsg {
    Error(&'static str),
    /// Consumer finished draining; carries its engine's final heap.
    Done {
        worker: usize,
        heap_used: usize,
    },
}

fn process_rss_kib() -> Option<u64> {
    let status = std::fs::read_to_string("/proc/self/status").ok()?;
    for line in status.lines() {
        if let Some(rest) = line.strip_prefix("VmRSS:") {
            return rest
                .trim()
                .trim_end_matches("kB")
                .trim()
                .parse::<u64>()
                .ok();
        }
    }
    None
}

fn process_cpu_secs() -> Option<f64> {
    let mut usage = std::mem::MaybeUninit::<libc::rusage>::uninit();
    if unsafe { libc::getrusage(libc::RUSAGE_SELF, usage.as_mut_ptr()) } != 0 {
        return None;
    }
    let u = unsafe { usage.assume_init() };
    let to_secs = |t: libc::timeval| t.tv_sec as f64 + t.tv_usec as f64 / 1e6;
    Some(to_secs(u.ru_utime) + to_secs(u.ru_stime))
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let arg_of = |name: &str| -> Option<u64> {
        args.iter()
            .position(|a| a == name)
            .and_then(|i| args.get(i + 1))
            .and_then(|v| v.parse().ok())
    };
    let workers = arg_of("--workers").unwrap_or(2) as usize;
    let duration_secs = arg_of("--duration-secs").unwrap_or(1_500);
    let window_secs = arg_of("--window-secs").unwrap_or(30);
    let out_dir = args
        .iter()
        .position(|a| a == "--out-dir")
        .and_then(|i| args.get(i + 1))
        .cloned()
        .unwrap_or_else(|| "benchmarks/raw/worker-scaling".to_string());
    std::fs::create_dir_all(&out_dir).expect("create out dir");

    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .expect("tokio runtime");

    rt.block_on(async {
        let engines = q_engine_quickjs::QuickJsEngine::spawn_independent(
            workers,
            QuickJsConfig::default(),
            rt.handle().clone(),
            Arc::new(IdentityMapper),
            "soak",
        );
        let mut consumer_engines = Vec::new();
        for (w, mut e) in engines.into_iter().enumerate() {
            let table: std::collections::BTreeMap<String, String> = [
                ("cpu.work".to_string(), "cpu.work".to_string()),
                ("light.work".to_string(), "light.work".to_string()),
                ("io.delay".to_string(), "io.delay".to_string()),
            ]
            .into_iter()
            .collect();
            e.load(
                BUNDLE,
                None,
                q_engine::EngineLoadPlan::Legacy {
                    expected_handlers: table,
                },
            )
            .expect("bundle load");
            consumer_engines.push((w, e));
        }

        let dispatcher: Arc<q_capabilities::Dispatcher<(u64, Instant)>> =
            Arc::new(q_capabilities::Dispatcher::with_workers(workers, 1_024));
        let (tx, rx) = mpsc::channel::<ConsumerMsg>();
        let expected = expected_cpu();
        let completed = Arc::new(AtomicU64::new(0));
        let errors = Arc::new(std::sync::Mutex::new(BTreeMap::<String, u64>::new()));

        let mut handles = Vec::new();
        for (w, mut engine) in consumer_engines {
            let dispatcher = dispatcher.clone();
            let tx = tx.clone();
            let completed = completed.clone();
            let handle = rt.handle().clone();
            handles.push(std::thread::spawn(move || {
                let queue = dispatcher.queue(w);
                loop {
                    let Some(((id, _enqueued_at), _wait)) =
                        queue.pop_timeout(Duration::from_millis(100))
                    else {
                        if queue.is_closed() && queue.is_empty() {
                            break;
                        }
                        continue;
                    };
                    let kind = kind_for(id);
                    let (otx, orx) = tokio::sync::oneshot::channel::<Outcome>();
                    let spec = InvocationSpec {
                        id,
                        request_id: format!("soak-{id}"),
                        route_id: handler_key(kind).into(),
                        route_id_num: None,
                        handler_key: handler_key(kind).into(),
                        policy_key: None,
                        handler_id: None,
                        policy_id_num: None,
                        policy_handler_id: None,
                        params_schema_id: None,
                        query_schema_id: None,
                        headers_schema_id: None,
                        body_schema_id: None,
                        request: None,
                        slot: q_engine::NO_REQUEST_SLOT,
                        generation: 0,
                        params: None,
                        query: None,
                        headers: None,
                        body: None,
                        allowed_statuses: vec![200],
                        default_status: 200,
                        response_strategy: ResponseStrategy::Js,
                        raw_response: false,
                        deadline: Instant::now() + Duration::from_millis(2_000),
                    };
                    engine.invoke(spec, otx);
                    let outcome = handle
                        .block_on(async {
                            tokio::time::timeout(Duration::from_millis(2_500), orx).await
                        })
                        .ok()
                        .and_then(|r| r.ok());
                    let parsed = match &outcome {
                        Some(Outcome::Response {
                            body: BodyOut::JsonText(t),
                            status: 200,
                            ..
                        }) => serde_json::from_str::<Value>(t).ok(),
                        _ => None,
                    };
                    let correct = match (&parsed, kind) {
                        (Some(v), Kind::Cpu) => {
                            v["sum"].as_f64() == Some(expected.0)
                                && v["len"].as_f64() == Some(expected.1)
                        }
                        (Some(v), Kind::Light) => v["ok"] == Value::Bool(true),
                        (Some(v), Kind::Io) => v["waited"].is_number(),
                        (None, _) => false,
                    };
                    if correct {
                        completed.fetch_add(1, Ordering::Relaxed);
                    } else {
                        let class = match &outcome {
                            None | Some(Outcome::Timeout) => "timeout",
                            Some(_) => "mismatch",
                        };
                        let _ = tx.send(ConsumerMsg::Error(class));
                    }
                }
                let heap = engine.stats().heap_used;
                engine.shutdown();
                let _ = tx.send(ConsumerMsg::Done {
                    worker: w,
                    heap_used: heap,
                });
            }));
        }
        drop(tx);

        // ---- continuous load + per-window sampling
        let t0 = Instant::now();
        let deadline = t0 + Duration::from_secs(duration_secs);
        let mut producers = Vec::new();
        let produced_counts: Arc<Vec<AtomicU64>> =
            Arc::new((0..8).map(|_| AtomicU64::new(0)).collect());
        for p in 0..8 {
            let d = dispatcher.clone();
            let counts = produced_counts.clone();
            producers.push(std::thread::spawn(move || {
                // Per-producer id stride: unique ids, and the produced
                // count reflects REAL dispatches (queue-full spins do
                // not consume ids).
                let mut id = (p + 1) as u64;
                while Instant::now() < deadline {
                    if d.dispatch((id, Instant::now())).is_ok() {
                        counts[p].fetch_add(1, Ordering::Relaxed);
                        id += 8;
                    } else {
                        std::hint::spin_loop();
                    }
                }
            }));
        }

        let mut windows: Vec<Value> = Vec::new();
        let mut window_start = Instant::now();
        let mut window_base_completed = 0u64;
        let mut window_seq = 0u64;
        let mut cpu_before = process_cpu_secs();
        while Instant::now() < deadline {
            // Drain error reports non-blockingly so the tally is live
            // and the channel never grows unboundedly.
            while let Ok(msg) = rx.try_recv() {
                if let ConsumerMsg::Error(class) = msg {
                    *errors.lock().unwrap().entry(class.to_string()).or_insert(0) += 1;
                }
            }
            std::thread::sleep(Duration::from_millis(200));
            if window_start.elapsed() >= Duration::from_secs(window_secs) {
                let now_completed = completed.load(Ordering::Relaxed);
                let window_reqs = now_completed - window_base_completed;
                let window_elapsed = window_start.elapsed().as_secs_f64();
                let rss = process_rss_kib();
                let cpu_now = process_cpu_secs();
                let queue_stats = dispatcher.stats();
                windows.push(json!({
                    "seq": window_seq,
                    "elapsedSecs": t0.elapsed().as_secs_f64(),
                    "windowSecs": window_elapsed,
                    "requests": window_reqs,
                    "throughputOpsPerSec": window_reqs as f64 / window_elapsed,
                    "processRssKib": rss,
                    "processCpuSecsCumulative": cpu_now,
                    "queueLens": queue_stats.iter().map(|s| s.len).collect::<Vec<_>>(),
                    "queueRejectedTotal": queue_stats.iter().map(|s| s.rejected).sum::<u64>(),
                }));
                window_base_completed = now_completed;
                window_start = Instant::now();
                window_seq += 1;
                cpu_before = cpu_now;
            }
        }
        let _ = cpu_before;
        for p in producers {
            p.join().unwrap();
        }

        // Drain: close queues, wait for consumers to finish their
        // in-flight invocations (bounded by the 2s invocation deadline).
        dispatcher.close_all();
        let mut final_heaps = vec![0usize; workers];
        let mut done = 0usize;
        for msg in rx {
            match msg {
                ConsumerMsg::Done { worker, heap_used } => {
                    final_heaps[worker] = heap_used;
                    done += 1;
                    if done == workers {
                        break;
                    }
                }
                ConsumerMsg::Error(class) => {
                    *errors.lock().unwrap().entry(class.to_string()).or_insert(0) += 1;
                }
            }
        }
        for h in handles {
            h.join().unwrap();
        }

        let total_requests: u64 = produced_counts
            .iter()
            .map(|c| c.load(Ordering::Relaxed))
            .sum();
        // The drain loop above waits for every dispatched request to
        // settle, so the final completed count covers all of them.
        let total_completed = completed.load(Ordering::Relaxed);
        let elapsed = t0.elapsed().as_secs_f64();
        let error_tally = errors.lock().unwrap().clone();
        let rss_series: Vec<Option<u64>> =
            windows.iter().map(|w| w["processRssKib"].as_u64()).collect();
        // Leak analysis: first/last window RSS and max window-to-window
        // growth (windows are equal-length; a monotonic climb is a leak
        // signal, allocator noise is bounded jitter).
        let known: Vec<u64> = rss_series.iter().filter_map(|r| *r).collect();
        let (first_rss, last_rss) = (known.first().copied(), known.last().copied());
        let max_step = known
            .windows(2)
            .map(|p| p[1].saturating_sub(p[0]))
            .max()
            .unwrap_or(0);

        let summary = json!({
            "format": "velqu-soak-v1",
            "engine": "quickjs-ng/0.15.1 via rquickjs 0.12.2",
            "workers": workers,
            "finalPerWorkerHeapBytes": final_heaps,
            "configuredDurationSecs": duration_secs,
            "windowSecs": window_secs,
            "actualDurationSecs": elapsed,
            "mix": "60% light.work / 25% cpu.work / 15% io.delay(1ms timer), deterministic per id",
            "totalDispatched": total_requests,
            "totalCompletedVerified": total_completed,
            "totalErrorsByClass": error_tally,
            "completionRate": if total_requests > 0 {
                total_completed as f64 / total_requests as f64
            } else { 0.0 },
            "throughputOpsPerSecOverall": total_completed as f64 / elapsed,
            "leakAnalysis": {
                "firstWindowRssKib": first_rss,
                "lastWindowRssKib": last_rss,
                "rssGrowthKib": first_rss.zip(last_rss).map(|(a, b)| b as i64 - a as i64),
                "maxWindowToWindowGrowthKib": max_step,
                "windows": rss_series.len(),
                "note": "RSS is process-level (producers+tokio+engines); small positive drift with bounded per-window steps is allocator retention, a monotonic climb across the full window set would be a leak signal",
            },
            "windows": windows,
        });
        std::fs::write(
            format!("{out_dir}/soak-summary.json"),
            serde_json::to_vec_pretty(&summary).unwrap(),
        )
        .expect("summary out");
        let mut raw = std::fs::File::create(format!("{out_dir}/soak.jsonl")).expect("raw out");
        for w in &windows {
            let _ = writeln!(raw, "{}", w);
        }
        println!(
            "soak complete: {out_dir}/soak.jsonl + summary ({elapsed:.0}s, {total_completed} verified)"
        );
    });
    rt.shutdown_timeout(Duration::from_secs(10));
}
