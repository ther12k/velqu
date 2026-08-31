//! Worker scaling measurement (M3-009-A): 1/2/4 independent QuickJS
//! runtimes behind the M3-002 bounded Dispatcher, measured at the
//! invocation boundary — the exact core ADR-0036 promised to measure in
//! M3-009 ("throughput gains come from real parallel runtimes").
//!
//! What is measured, honestly:
//! - enqueue→outcome latency per request INCLUDING queue wait (queue
//!   latency is never hidden; queue wait also reports separately, plus
//!   service time = total − queue wait);
//! - throughput (ops/s) per worker count across repetitions. The
//!   repetition loop is INTERLEAVED round-robin over the worker counts
//!   so host drift between phases hits every config equally instead of
//!   correlating with config order (sequential phases produced
//!   impossible >linear ratios on a noisy shared host);
//! - per-worker JS heap (per-worker memory visibility; identical
//!   across workers by construction) and process-level RSS per
//!   repetition (attribution disclosed as process-level);
//! - correctness of every measured response (deterministic CPU
//!   handler, result verified host-side per invocation).
//!
//! What is NOT claimed:
//! - no HTTP-layer numbers (the runtime's HTTP path still drives one
//!   engine; multi-engine HTTP wiring is the M3 integration) — this
//!   measures the engine + dispatcher core;
//! - no target verdict — scaling efficiency is reported against the
//!   1-worker baseline; the numeric approval target is an owner
//!   decision (no approved number exists in the docs today).
//!
//! Output: benchmarks/raw/worker-scaling/worker-scaling.jsonl (one line
//! per sample) + worker-scaling-summary.json. Constraint 12: raw
//! samples retained.

use std::io::Write;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc;
use std::sync::Arc;
use std::time::{Duration, Instant};

use q_engine::{BodyOut, Engine as _, InvocationSpec, Outcome, ResponseStrategy};
use q_engine_quickjs::{IdentityMapper, QuickJsConfig};
use serde_json::{json, Value};

/// Deterministic CPU handler: fixed work, verifiable result.
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
__velquRegister("cpu.work", cpu_work);
"#;

/// Same arithmetic host-side, for per-invocation verification.
fn expected_result_once() -> (f64, f64) {
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

const WORKER_COUNTS: [usize; 3] = [1, 2, 4];
const PRODUCERS: usize = 8;
const WARMUP_PER_WORKER: usize = 100;
const REPETITIONS: usize = 5;
const REQUESTS_PER_REP: usize = 3_000;
const QUEUE_CAPACITY: usize = 1_024;

#[derive(Clone)]
struct Sample {
    workers: usize,
    rep: usize, // 0..REPETITIONS-1 measured; usize::MAX marks warmup
    idx: usize,
    total_us: f64,
    queue_wait_us: f64,
    correct: bool,
}

enum ConsumerMsg {
    Sample(Sample),
    /// Per-worker engine stats, sent when the queue closes and drains.
    Done {
        worker: usize,
        heap_used: usize,
    },
}

fn percentiles(mut v: Vec<f64>, ps: &[f64]) -> Vec<f64> {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    ps.iter()
        .map(|&p| {
            if v.is_empty() {
                return 0.0;
            }
            let rank = ((v.len() as f64 - 1.0) * p).round() as usize;
            v[rank.min(v.len() - 1)]
        })
        .collect()
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[v.len() / 2]
}

fn process_rss_kib() -> Option<u64> {
    // Linux /proc/self/status VmRSS (KiB); None elsewhere (recorded as
    // unavailable — never fabricated).
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

fn main() {
    let out_dir = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "benchmarks/raw/worker-scaling".to_string());
    std::fs::create_dir_all(&out_dir).expect("create out dir");

    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .expect("tokio runtime");

    let mut all_samples: Vec<Sample> = Vec::new();
    // Per-config accumulators across the interleaved repetitions.
    let mut rep_throughputs: Vec<Vec<f64>> = vec![Vec::new(); WORKER_COUNTS.len()];
    let mut rep_samples: Vec<Vec<Sample>> = vec![Vec::new(); WORKER_COUNTS.len()];
    let mut rep_avg_heap: Vec<Vec<usize>> = vec![Vec::new(); WORKER_COUNTS.len()];
    let mut rep_rss: Vec<Vec<Value>> = vec![Vec::new(); WORKER_COUNTS.len()];

    // INTERLEAVED repetition loop: each repetition visits every worker
    // count, so time-correlated host drift spreads over configs
    // instead of correlating with config order.
    for rep in 0..REPETITIONS {
        for (ci, &workers) in WORKER_COUNTS.iter().enumerate() {
            let rss_before = process_rss_kib();

            rt.block_on(async {
                // Real parallel runtimes: one thread + one QuickJS
                // runtime per worker (ADR-0036 §1/§2), identical load
                // in each (§6).
                let engines = q_engine_quickjs::QuickJsEngine::spawn_independent(
                    workers,
                    QuickJsConfig::default(),
                    rt.handle().clone(),
                    Arc::new(IdentityMapper),
                    "scaling",
                );
                let mut consumer_engines = Vec::new();
                for (w, mut e) in engines.into_iter().enumerate() {
                    let table: std::collections::BTreeMap<String, String> =
                        [("cpu.work".to_string(), "cpu.work".to_string())]
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

                let dispatcher: Arc<q_capabilities::Dispatcher<(u64, Instant)>> = Arc::new(
                    q_capabilities::Dispatcher::with_workers(workers, QUEUE_CAPACITY),
                );
                let (tx, rx) = mpsc::channel::<ConsumerMsg>();
                let expected = expected_result_once();

                // One consumer thread per worker owns its engine (§1).
                let mut handles = Vec::new();
                for (w, mut engine) in consumer_engines {
                    let dispatcher = dispatcher.clone();
                    let tx = tx.clone();
                    let handle = rt.handle().clone();
                    handles.push(std::thread::spawn(move || {
                        let queue = dispatcher.queue(w);
                        loop {
                            let Some(((id, enqueued_at), queue_wait)) =
                                queue.pop_timeout(Duration::from_millis(50))
                            else {
                                if queue.is_closed() && queue.is_empty() {
                                    break;
                                }
                                continue;
                            };
                            let (otx, orx) = tokio::sync::oneshot::channel::<Outcome>();
                            let spec = InvocationSpec {
                                id,
                                request_id: format!("scale-{id}"),
                                route_id: "cpu.work".into(),
                                route_id_num: None,
                                handler_key: "cpu.work".into(),
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
                            if std::env::var("VELQU_BENCH_DEBUG").is_ok() && id == 1_000_000 {
                                eprintln!("DEBUG outcome: {outcome:?}");
                            }
                            // Strategy Js hands back engine-stringified text.
                            let correct = matches!(
                                &outcome,
                                Some(Outcome::Response {
                                    body: BodyOut::JsonText(t),
                                    status: 200,
                                    ..
                                }) if serde_json::from_str::<Value>(t)
                                    .map(|v| v["sum"].as_f64() == Some(expected.0)
                                        && v["len"].as_f64() == Some(expected.1))
                                    .unwrap_or(false)
                            );
                            let total_us = enqueued_at.elapsed().as_secs_f64() * 1e6;
                            let _ = tx.send(ConsumerMsg::Sample(Sample {
                                workers,
                                rep: usize::MAX, // warmup marker; re-tagged below
                                idx: id as usize,
                                total_us,
                                queue_wait_us: queue_wait.as_secs_f64() * 1e6,
                                correct,
                            }));
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

                // Dispatch n requests from P producers; collect exactly
                // n samples (closed loop, queue wait inside each).
                let dispatch_and_collect = |n: usize, id_base: u64, samples: &mut Vec<Sample>| {
                    let ids = Arc::new(AtomicU64::new(id_base));
                    let t0 = Instant::now();
                    let mut producers = Vec::new();
                    let per_producer = n / PRODUCERS;
                    let remainder = n % PRODUCERS;
                    for p in 0..PRODUCERS {
                        let d = dispatcher.clone();
                        let ids = ids.clone();
                        let count = per_producer + usize::from(p < remainder);
                        producers.push(std::thread::spawn(move || {
                            let mut produced = 0usize;
                            while produced < count {
                                let id = ids.fetch_add(1, Ordering::Relaxed);
                                if d.dispatch((id, Instant::now())).is_ok() {
                                    produced += 1;
                                } else {
                                    std::hint::spin_loop();
                                }
                            }
                        }));
                    }
                    for p in producers {
                        p.join().unwrap();
                    }
                    let mut got = 0usize;
                    while got < n {
                        match rx.recv().expect("consumer alive") {
                            ConsumerMsg::Sample(s) => {
                                samples.push(s);
                                got += 1;
                            }
                            ConsumerMsg::Done { .. } => {
                                unreachable!("no consumer exits before close_all")
                            }
                        }
                    }
                    t0.elapsed()
                };

                // ---- warmup (excluded from measurement)
                let mut warm = Vec::new();
                dispatch_and_collect(WARMUP_PER_WORKER * workers, 1, &mut warm);
                all_samples.append(&mut warm);

                // ---- one measured run this repetition
                let mut batch = Vec::with_capacity(REQUESTS_PER_REP);
                let elapsed = dispatch_and_collect(
                    REQUESTS_PER_REP,
                    1_000_000 + rep as u64 * 10_000_000,
                    &mut batch,
                );
                rep_throughputs[ci].push(REQUESTS_PER_REP as f64 / elapsed.as_secs_f64());
                for s in batch {
                    let tagged = Sample { rep, ..s };
                    rep_samples[ci].push(tagged.clone());
                    all_samples.push(tagged);
                }

                dispatcher.close_all();
                let mut per_worker_heap = vec![0usize; workers];
                let mut done = 0usize;
                for msg in rx {
                    match msg {
                        ConsumerMsg::Done { worker, heap_used } => {
                            per_worker_heap[worker] = heap_used;
                            done += 1;
                            if done == workers {
                                break;
                            }
                        }
                        ConsumerMsg::Sample(_) => {}
                    }
                }
                for h in handles {
                    h.join().unwrap();
                }

                let rss_after = process_rss_kib();
                rep_rss[ci].push(json!({"before": rss_before, "after": rss_after}));
                rep_avg_heap[ci].push(per_worker_heap.iter().sum::<usize>() / workers.max(1));
            });
        }
    }

    // ---- summarize per config across the interleaved repetitions
    let mut configs: Vec<Value> = Vec::new();
    let ps = [0.5f64, 0.95, 0.99];
    let baseline_median = median(rep_throughputs[0].clone());
    let baseline_best = rep_throughputs[0]
        .iter()
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max);
    for (ci, &workers) in WORKER_COUNTS.iter().enumerate() {
        let measured = &rep_samples[ci];
        let totals: Vec<f64> = measured.iter().map(|s| s.total_us).collect();
        let waits: Vec<f64> = measured.iter().map(|s| s.queue_wait_us).collect();
        let services: Vec<f64> = measured
            .iter()
            .map(|s| s.total_us - s.queue_wait_us)
            .collect();
        let correct_n = measured.iter().filter(|s| s.correct).count();
        let tps = percentiles(totals, &ps);
        let wps = percentiles(waits, &ps);
        let sps = percentiles(services, &ps);
        let med = median(rep_throughputs[ci].clone());
        let best = rep_throughputs[ci]
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max);
        let avg_heap = rep_avg_heap[ci].iter().sum::<usize>() / rep_avg_heap[ci].len().max(1);

        let mut entry = json!({
            "workers": workers,
            "repetitions": REPETITIONS,
            "requestsPerRepetition": REQUESTS_PER_REP,
            "producers": PRODUCERS,
            "queueCapacityPerWorker": QUEUE_CAPACITY,
            "throughputPerRepetitionOpsPerSec": rep_throughputs[ci],
            "throughputOpsPerSecMedian": med,
            "throughputOpsPerSecBest": best,
            "samples": measured.len(),
            "correct": correct_n,
            "latencyUs": { "p50": tps[0], "p95": tps[1], "p99": tps[2] },
            "serviceUs": { "p50": sps[0], "p95": sps[1], "p99": sps[2] },
            "queueWaitUs": { "p50": wps[0], "p95": wps[1], "p99": wps[2] },
            "avgPerWorkerHeapBytes": avg_heap,
            "totalHeapBytes": avg_heap * workers,
            "processRssKibPerRepetition": rep_rss[ci],
        });
        if workers > 1 {
            entry["scalingVs1WorkerMedian"] = json!(med / baseline_median);
            entry["scalingVs1WorkerBest"] = json!(best / baseline_best);
        }
        configs.push(entry);
    }

    // ---- raw + summary
    let mut raw =
        std::fs::File::create(format!("{out_dir}/worker-scaling.jsonl")).expect("raw out");
    for s in &all_samples {
        let _ = writeln!(
            raw,
            "{}",
            json!({
                "workers": s.workers,
                "rep": if s.rep == usize::MAX { Value::Null } else { json!(s.rep) },
                "idx": s.idx,
                "totalUs": s.total_us,
                "queueWaitUs": s.queue_wait_us,
                "correct": s.correct,
            })
        );
    }
    let summary = json!({
        "format": "velqu-worker-scaling-v2",
        "engine": "quickjs-ng/0.15.1 via rquickjs 0.12.2",
        "workload": "cpu.work: 20000-iteration deterministic arithmetic+string, verified per invocation",
        "workerCounts": WORKER_COUNTS,
        "repetitionsPerConfig": REPETITIONS,
        "interleaved": true,
        "physicalCores": std::thread::available_parallelism().map(|n| n.get()).unwrap_or(0),
        "note": "invocation-boundary measurement of N real parallel QuickJS runtimes behind the M3-002 bounded Dispatcher; HTTP layer not exercised",
        "configs": configs,
    });
    std::fs::write(
        format!("{out_dir}/worker-scaling-summary.json"),
        serde_json::to_vec_pretty(&summary).unwrap(),
    )
    .expect("summary out");
    println!("worker scaling bench complete: {out_dir}/worker-scaling.jsonl + summary");
    rt.shutdown_timeout(Duration::from_secs(5));
}
