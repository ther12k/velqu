use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

fn measure<F: FnMut()>(mut f: F, iters: usize) -> (f64, f64, f64, f64) {
    let mut samples = Vec::with_capacity(iters);
    for _ in 0..iters {
        let start = Instant::now();
        f();
        samples.push(start.elapsed().as_nanos() as f64 / 1_000.0);
    }
    samples.sort_by(f64::total_cmp);
    let mean = samples.iter().sum::<f64>() / samples.len() as f64;
    let at = |q: f64| samples[((samples.len() - 1) as f64 * q) as usize];
    (mean, at(0.50), at(0.95), at(0.99))
}

fn main() {
    let iters = std::env::args()
        .position(|arg| arg == "--iters")
        .and_then(|i| std::env::args().nth(i + 1))
        .and_then(|v| v.parse().ok())
        .unwrap_or(100_000usize);
    let warmup = 10_000usize;
    let counter = AtomicU64::new(0);
    for _ in 0..warmup {
        std::hint::black_box(())
    }
    let disabled = measure(|| std::hint::black_box(()), iters);
    let enabled = measure(
        || {
            counter.fetch_add(1, Ordering::Relaxed);
            std::hint::black_box(())
        },
        iters,
    );
    println!(
        "{{\"format\":\"velqu-observability-overhead-v1\",\"iters\":{},\"warmup\":{},\"counter\":{},\"disabled_us\":{{\"mean\":{:.6},\"p50\":{:.6},\"p95\":{:.6},\"p99\":{:.6}}},\"enabled_us\":{{\"mean\":{:.6},\"p50\":{:.6},\"p95\":{:.6},\"p99\":{:.6}}}}}",
        iters, warmup, counter.load(Ordering::Relaxed), disabled.0, disabled.1, disabled.2,
        disabled.3, enabled.0, enabled.1, enabled.2, enabled.3
    );
}
