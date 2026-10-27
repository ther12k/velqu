//! Startup codec-table compilation cost across the route-count ladder
//! (#1428).
//!
//! `crates/q-runtime/src/lib.rs` builds `DecoderTable` and `EncoderTable`
//! from the pack's `SchemaIr` at startup, after the ready line has printed.
//! This is real schema-program compilation, it is NOT part of the recorded
//! startup stages (`startupMs` excludes it), and on the shipped default
//! load path (legacy-v1 JSON packs) it happens on every startup. This
//! example isolates and measures that cost across the route-count ladder,
//! emitting raw JSONL rows plus a summary with n/mean/p50/p95/p99 per size.
//!
//! Run from the repository root:
//!   cargo run --release -p velqu-runtime --example codec_table_cost -- \
//!     --out-dir benchmarks/raw/codec-table-startup [--samples 40]

use std::hint::black_box;
use std::path::Path;
use std::time::Instant;

use serde_json::json;

fn percentile(sorted: &[f64], p: f64) -> f64 {
    if sorted.is_empty() {
        return f64::NAN;
    }
    let idx = ((p / 100.0) * (sorted.len() - 1) as f64).round() as usize;
    sorted[idx.min(sorted.len() - 1)]
}

fn stats(mut values: Vec<f64>) -> serde_json::Value {
    values.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = values.len();
    let mean = values.iter().sum::<f64>() / n as f64;
    json!({
        "n": n,
        "mean": mean,
        "p50": percentile(&values, 50.0),
        "p95": percentile(&values, 95.0),
        "p99": percentile(&values, 99.0),
    })
}

fn main() {
    let mut out_dir = std::path::PathBuf::from("benchmarks/raw/codec-table-startup");
    let mut samples: usize = 40;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--out-dir" => {
                out_dir = std::path::PathBuf::from(args.next().expect("--out-dir needs a value"))
            }
            "--samples" => {
                samples = args
                    .next()
                    .expect("--samples needs a value")
                    .parse()
                    .expect("samples must be an integer")
            }
            other => panic!("unknown argument {other}"),
        }
    }
    std::fs::create_dir_all(&out_dir).expect("create out-dir");

    let run_id = format!(
        "codec-table-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
    );
    let ladder = [25usize, 100, 1000, 5000, 10000];

    let mut rows: Vec<serde_json::Value> = Vec::new();
    let mut results: Vec<serde_json::Value> = Vec::new();

    for n in ladder {
        let pack_path = format!("benchmarks/raw/packs/app-{n}.qpack");
        let pack = q_pack::QPack::load_and_verify(Path::new(&pack_path))
            .unwrap_or_else(|e| panic!("load {pack_path}: {e}"));
        // Exactly the construction the runtime performs at startup
        // (crates/q-runtime/src/lib.rs): a dense schema vector from the
        // verified manifest, then both tables compiled from it.
        let schema_vector: Vec<q_schema_runtime::SchemaIr> =
            pack.schema_manifest.iter().map(|s| s.ir.clone()).collect();
        let routes = pack.routes.len();
        let schemas = schema_vector.len();

        // Warm-up (first construction pays allocator growth the production
        // path also pays; measure it, then sample steady-state too).
        let warm_start = Instant::now();
        let warm_decoder = q_schema_runtime::DecoderTable::from_schemas(&schema_vector);
        let warm_encoder = q_schema_runtime::EncoderTable::from_schemas(&schema_vector);
        let warm_us = warm_start.elapsed().as_secs_f64() * 1e6;
        drop((warm_decoder, warm_encoder));

        let mut decoder_us: Vec<f64> = Vec::with_capacity(samples);
        let mut encoder_us: Vec<f64> = Vec::with_capacity(samples);
        for i in 0..samples {
            let t = Instant::now();
            let decoder = q_schema_runtime::DecoderTable::from_schemas(&schema_vector);
            let d_us = t.elapsed().as_secs_f64() * 1e6;
            black_box(&decoder);
            let t = Instant::now();
            let encoder = q_schema_runtime::EncoderTable::from_schemas(&schema_vector);
            let e_us = t.elapsed().as_secs_f64() * 1e6;
            black_box(&encoder);
            decoder_us.push(d_us);
            encoder_us.push(e_us);
            rows.push(json!({
                "runId": run_id,
                "n": n,
                "i": i,
                "routes": routes,
                "schemas": schemas,
                "decoderUs": d_us,
                "encoderUs": e_us,
                "totalUs": d_us + e_us,
            }));
        }
        let totals: Vec<f64> = decoder_us
            .iter()
            .zip(&encoder_us)
            .map(|(d, e)| d + e)
            .collect();
        let mut sorted_totals = totals.clone();
        sorted_totals.sort_by(|a, b| a.partial_cmp(b).unwrap());
        results.push(json!({
            "n": n,
            "routes": routes,
            "schemas": schemas,
            "warmFirstTotalUs": warm_us,
            "decoderUs": stats(decoder_us),
            "encoderUs": stats(encoder_us),
            "totalUs": stats(totals),
        }));
        println!("n={n:5} routes={routes:5} schemas={schemas:5} warmFirst={warm_us:9.1}us total p50={:9.1}us", percentile(&sorted_totals, 50.0));
    }

    let cpu = std::fs::read_to_string("/proc/cpuinfo")
        .ok()
        .and_then(|c| {
            c.lines()
                .find(|l| l.starts_with("model name"))
                .map(|l| l.trim().to_string())
        })
        .unwrap_or_else(|| "unknown".into());
    let summary = json!({
        "format": "velqu-codec-table-startup-cost-v1",
        "runId": run_id,
        "generatedAtUnixEpoch": unix_epoch_now(),
        "purpose": "#1428 — measured startup codec-table compilation cost on the default legacy-v1 JSON load path (raw samples retained)",
        "method": "DecoderTable::from_schemas + EncoderTable::from_schemas over the dense schema vector of each route-count ladder pack, mirroring crates/q-runtime/src/lib.rs startup construction; tables build AFTER the ready line so this cost is outside the recorded startupMs",
        "environment": {
            "cpu": cpu,
            "rustc": rustc_version(),
            "profile": "release",
        },
        "samples": samples,
        "results": results,
    });
    std::fs::write(
        out_dir.join("codec-table.jsonl"),
        rows.iter()
            .map(|r| r.to_string())
            .collect::<Vec<_>>()
            .join("\n")
            + "\n",
    )
    .expect("write jsonl");
    std::fs::write(
        out_dir.join("summary.json"),
        serde_json::to_string_pretty(&summary).unwrap() + "\n",
    )
    .expect("write summary");
    println!("wrote {}", out_dir.join("codec-table.jsonl").display());
    println!("wrote {}", out_dir.join("summary.json").display());
}

fn unix_epoch_now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn rustc_version() -> String {
    std::process::Command::new(std::env::var("RUSTC").unwrap_or_else(|_| "rustc".into()))
        .arg("--version")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".into())
}
