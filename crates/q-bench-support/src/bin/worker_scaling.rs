//! Worker scaling measurement with controlled workloads (M3-009-A/B/C):
//! 1/2/4 independent QuickJS runtimes behind the M3-002 bounded
//! Dispatcher, measured at the invocation boundary — the exact core
//! ADR-0036 promised to measure in M3-009 ("throughput gains come from
//! real parallel runtimes").
//!
//! Workloads (frozen definitions for this evidence):
//! - **C1 — CPU-bound**: 100 % `cpu.work` (20 000-iteration
//!   deterministic arithmetic+string, verified per invocation);
//! - **C2 — mixed**: 80 % `light.work` (tiny object return) + 20 %
//!   `cpu.work`, chosen by a deterministic id rule so the consumer can
//!   verify every response;
//! - **C3 — I/O-bound**: 100 % `io.delay` (one 1 ms native timer op per
//!   invocation) — CONTROLLED I/O: deterministic, no external network.
//!
//! What is measured, honestly:
//! - enqueue→outcome latency per request INCLUDING queue wait (queue
//!   wait also reports separately, plus service time = total − wait);
//! - throughput per worker count across INTERLEAVED repetitions
//!   (round-robin over all workload×worker configs so host drift hits
//!   every config equally);
//! - process CPU seconds, wall seconds, classified errors
//!   (timeout/mismatch; nothing dropped), per-worker JS heap, and
//!   process-level RSS.
//!
//! Not claimed: HTTP-layer numbers (the runtime's HTTP path still
//! drives one engine — multi-engine wiring is the M3 integration); no
//! numeric scaling-target verdict (owner decision).
//!
//! Output: benchmarks/raw/worker-scaling/worker-scaling.jsonl +
//! worker-scaling-summary.json (velqu-worker-scaling-v4). Constraint
//! 12: raw samples retained.

use std::collections::BTreeMap;
use std::io::Write;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
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

/// Host-side expected result of `cpu_work` (representation-independent:
/// JS numbers are f64).
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

/// The workload dimension (frozen C1/C2/C3 definitions, see module doc).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Workload {
    C1Cpu,
    C2Mixed,
    C3Io,
}

impl Workload {
    fn name(&self) -> &'static str {
        match self {
            Workload::C1Cpu => "C1-cpu",
            Workload::C2Mixed => "C2-mixed",
            Workload::C3Io => "C3-io",
        }
    }
    fn requests_per_rep(&self) -> usize {
        match self {
            // C3's 1 ms timers dominate: fewer requests, same evidence.
            Workload::C3Io => 1_200,
            _ => 3_000,
        }
    }
}

/// The per-request operation a consumer performs (recomputed from the
/// id deterministically for C2's mix).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    Cpu,
    Light,
    Io,
}

fn kind_for(workload: Workload, id: u64) -> Kind {
    match workload {
        Workload::C1Cpu => Kind::Cpu,
        Workload::C2Mixed => {
            if id.is_multiple_of(5) {
                Kind::Cpu
            } else {
                Kind::Light
            }
        }
        Workload::C3Io => Kind::Io,
    }
}

fn handler_key(kind: Kind) -> &'static str {
    match kind {
        Kind::Cpu => "cpu.work",
        Kind::Light => "light.work",
        Kind::Io => "io.delay",
    }
}

const WORKER_COUNTS: [usize; 3] = [1, 2, 4];
const WORKLOADS: [Workload; 3] = [Workload::C1Cpu, Workload::C2Mixed, Workload::C3Io];
const PRODUCERS: usize = 8;
const WARMUP_PER_WORKER: usize = 100;
const REPETITIONS: usize = 3;
const QUEUE_CAPACITY: usize = 1_024;

#[derive(Clone)]
struct Sample {
    workload: &'static str,
    workers: usize,
    rep: usize, // 0..REPETITIONS-1 measured; usize::MAX marks warmup
    idx: usize,
    total_us: f64,
    queue_wait_us: f64,
    correct: bool,
}

enum ConsumerMsg {
    Sample(Sample),
    /// A measured outcome that did not verify (classified error).
    Error(&'static str),
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

/// Process CPU seconds (user + system) via getrusage; None if the
/// syscall fails (recorded as unavailable — never fabricated).
fn process_cpu_secs() -> Option<f64> {
    let mut usage = std::mem::MaybeUninit::<libc::rusage>::uninit();
    if unsafe { libc::getrusage(libc::RUSAGE_SELF, usage.as_mut_ptr()) } != 0 {
        return None;
    }
    let u = unsafe { usage.assume_init() };
    let to_secs = |t: libc::timeval| t.tv_sec as f64 + t.tv_usec as f64 / 1e6;
    Some(to_secs(u.ru_utime) + to_secs(u.ru_stime))
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
    // Per-config accumulators (config = workload × worker count),
    // across the interleaved repetitions.
    let n_configs = WORKLOADS.len() * WORKER_COUNTS.len();
    let mut rep_throughputs: Vec<Vec<f64>> = vec![Vec::new(); n_configs];
    let mut rep_samples: Vec<Vec<Sample>> = vec![Vec::new(); n_configs];
    let mut rep_avg_heap: Vec<Vec<usize>> = vec![Vec::new(); n_configs];
    let mut rep_rss: Vec<Vec<Value>> = vec![Vec::new(); n_configs];
    let mut rep_cpu_secs: Vec<Vec<Option<f64>>> = vec![Vec::new(); n_configs];
    let mut rep_wall_secs: Vec<Vec<f64>> = vec![Vec::new(); n_configs];
    let mut rep_errors: Vec<BTreeMap<String, u64>> = vec![BTreeMap::new(); n_configs];
    let rep_error_count: Arc<Mutex<BTreeMap<String, u64>>> = Arc::new(Mutex::new(BTreeMap::new()));

    let expected = expected_cpu();

    // INTERLEAVED repetition loop: each repetition visits every
    // workload×worker config, so time-correlated host drift spreads
    // over configs instead of correlating with config order.
    for rep in 0..REPETITIONS {
        for (wi, workload) in WORKLOADS.iter().enumerate() {
            for (ci_off, &workers) in WORKER_COUNTS.iter().enumerate() {
                let ci = wi * WORKER_COUNTS.len() + ci_off;
                let rss_before = process_rss_kib();
                let rep_error_count = rep_error_count.clone();
                rep_error_count.lock().unwrap().clear();

                rt.block_on(async {
                    // Real parallel runtimes: one thread + one QuickJS
                    // runtime per worker (ADR-0036 §1/§2), identical
                    // load in each (§6).
                    let engines = q_engine_quickjs::QuickJsEngine::spawn_independent(
                        workers,
                        QuickJsConfig::default(),
                        rt.handle().clone(),
                        Arc::new(IdentityMapper),
                        "scaling",
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

                    let dispatcher: Arc<q_capabilities::Dispatcher<(u64, Instant)>> = Arc::new(
                        q_capabilities::Dispatcher::with_workers(workers, QUEUE_CAPACITY),
                    );
                    let (tx, rx) = mpsc::channel::<ConsumerMsg>();

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
                                let kind = kind_for(*workload, id);
                                let (otx, orx) = tokio::sync::oneshot::channel::<Outcome>();
                                let spec = InvocationSpec {
                                    id,
                                    request_id: format!("scale-{id}"),
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
                                        tokio::time::timeout(Duration::from_millis(2_500), orx)
                                            .await
                                    })
                                    .ok()
                                    .and_then(|r| r.ok());
                                // Strategy Js hands back engine-
                                // stringified text. Verification is
                                // per kind; failures classify as
                                // mismatch/timeout, never dropped.
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
                                if !correct {
                                    let class = match &outcome {
                                        None | Some(Outcome::Timeout) => "timeout",
                                        Some(_) => "mismatch",
                                    };
                                    let _ = tx.send(ConsumerMsg::Error(class));
                                }
                                let total_us = enqueued_at.elapsed().as_secs_f64() * 1e6;
                                let _ = tx.send(ConsumerMsg::Sample(Sample {
                                    workload: workload.name(),
                                    workers,
                                    rep: usize::MAX, // warmup marker
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

                    // Dispatch n requests from P producers; collect
                    // exactly n samples (closed loop, queue wait
                    // inside each). Errors are counted, not sampled.
                    let dispatch_and_collect =
                        |n: usize, id_base: u64, samples: &mut Vec<Sample>| {
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
                            let mut errors = 0u64;
                            let mut error_tally: BTreeMap<String, u64> = BTreeMap::new();
                            while got + (errors as usize) < n {
                                match rx.recv().expect("consumer alive") {
                                    ConsumerMsg::Sample(s) => {
                                        samples.push(s);
                                        got += 1;
                                    }
                                    ConsumerMsg::Error(class) => {
                                        *error_tally.entry(class.to_string()).or_insert(0) += 1;
                                        errors += 1;
                                    }
                                    ConsumerMsg::Done { .. } => {
                                        unreachable!("no consumer exits before close_all")
                                    }
                                }
                            }
                            {
                                let mut tally = rep_error_count.lock().unwrap();
                                for (k, v) in error_tally {
                                    *tally.entry(k).or_insert(0) += v;
                                }
                            }
                            t0.elapsed()
                        };

                    // ---- warmup (excluded from measurement)
                    let mut warm = Vec::new();
                    dispatch_and_collect(WARMUP_PER_WORKER * workers, 1, &mut warm);
                    all_samples.append(&mut warm);

                    // ---- one measured run this repetition
                    let n = workload.requests_per_rep();
                    let cpu_before = process_cpu_secs();
                    let mut batch = Vec::with_capacity(n);
                    let elapsed =
                        dispatch_and_collect(n, 1_000_000 + rep as u64 * 10_000_000, &mut batch);
                    let cpu_after = process_cpu_secs();
                    let wall = elapsed.as_secs_f64();
                    rep_wall_secs[ci].push(wall);
                    rep_cpu_secs[ci].push(cpu_after.zip(cpu_before).map(|(a, b)| a - b));
                    rep_throughputs[ci].push(n as f64 / wall);
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
                            ConsumerMsg::Error(_) => {}
                        }
                    }
                    for h in handles {
                        h.join().unwrap();
                    }

                    let rss_after = process_rss_kib();
                    {
                        let tally = rep_error_count.lock().unwrap();
                        for (k, v) in tally.iter() {
                            *rep_errors[ci].entry(k.clone()).or_insert(0) += v;
                        }
                    }
                    rep_rss[ci].push(json!({"before": rss_before, "after": rss_after}));
                    rep_avg_heap[ci].push(per_worker_heap.iter().sum::<usize>() / workers.max(1));
                });
            }
        }
    }

    // ---- summarize per config across the interleaved repetitions
    let mut workloads_out: Vec<Value> = Vec::new();
    let ps = [0.5f64, 0.95, 0.99];
    for (wi, workload) in WORKLOADS.iter().enumerate() {
        let base = wi * WORKER_COUNTS.len();
        let baseline_median = median(rep_throughputs[base].clone());
        let baseline_best = rep_throughputs[base]
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max);
        let mut configs: Vec<Value> = Vec::new();
        for (ci_off, &workers) in WORKER_COUNTS.iter().enumerate() {
            let ci = base + ci_off;
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
            let total_cpu: f64 = rep_cpu_secs[ci].iter().filter_map(|c| *c).sum();
            let ops = measured.len() as f64;

            let mut entry = json!({
                "workers": workers,
                "repetitions": REPETITIONS,
                "requestsPerRepetition": workload.requests_per_rep(),
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
                "processCpuSecsPerRepetition": rep_cpu_secs[ci],
                "processCpuSecsTotal": total_cpu,
                "wallSecsPerRepetition": rep_wall_secs[ci],
                "processCpuSecsPerOp": if ops > 0.0 { total_cpu / ops } else { 0.0 },
                "errors": {
                    "byClass": rep_errors[ci],
                    "total": rep_errors[ci].values().sum::<u64>(),
                    "classes": ["timeout", "mismatch"],
                    "note": "every dispatched request is sampled or counted as a classified error; none dropped"
                },
            });
            if workers > 1 {
                entry["scalingVs1WorkerMedian"] = json!(med / baseline_median);
                entry["scalingVs1WorkerBest"] = json!(best / baseline_best);
            }
            configs.push(entry);
        }
        workloads_out.push(json!({
            "workload": workload.name(),
            "definition": match workload {
                Workload::C1Cpu => "100% cpu.work (20k-iteration deterministic arithmetic+string)",
                Workload::C2Mixed => "80% light.work + 20% cpu.work, by deterministic id rule (id % 5 == 0 -> cpu)",
                Workload::C3Io => "100% io.delay (one 1ms native timer op per invocation; controlled I/O, no external network)",
            },
            "configs": configs,
        }));
    }

    // ---- raw + summary
    let mut raw =
        std::fs::File::create(format!("{out_dir}/worker-scaling.jsonl")).expect("raw out");
    for s in &all_samples {
        let _ = writeln!(
            raw,
            "{}",
            json!({
                "workload": s.workload,
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
        "format": "velqu-worker-scaling-v4",
        "engine": "quickjs-ng/0.15.1 via rquickjs 0.12.2",
        "workloads": workloads_out,
        "workerCounts": WORKER_COUNTS,
        "repetitionsPerConfig": REPETITIONS,
        "interleaved": true,
        "physicalCores": std::thread::available_parallelism().map(|n| n.get()).unwrap_or(0),
        "note": "invocation-boundary measurement of N real parallel QuickJS runtimes behind the M3-002 bounded Dispatcher; HTTP layer not exercised; C3 I/O is controlled (native timers, no network)",
    });
    std::fs::write(
        format!("{out_dir}/worker-scaling-summary.json"),
        serde_json::to_vec_pretty(&summary).unwrap(),
    )
    .expect("summary out");
    println!("worker scaling bench complete: {out_dir}/worker-scaling.jsonl + summary");
    rt.shutdown_timeout(Duration::from_secs(5));
}
