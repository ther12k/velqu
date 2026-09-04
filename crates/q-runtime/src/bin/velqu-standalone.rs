//! velqu-standalone — the STANDALONE deployment mode (M26-009-B): one
//! executable with the verified pack embedded at compile time.
//!
//! Build (the pack must be the exact verified artifact; it is still
//! fully re-verified at startup — embedding grants no trust):
//!
//! ```text
//! VELQU_STANDALONE_PACK=examples/proof/dist/app.qpack \
//!   cargo build --release -p q-runtime --features standalone
//! ```
//!
//! The binary contains NO compiler toolchain: no Bun, no TypeScript,
//! no route/schema/OpenAPI compilation — the identical load-verify-serve
//! pipeline from `velqu_runtime` (lib.rs) with `PackSource::Embedded`.

use std::path::PathBuf;

use clap::Parser;
use velqu_runtime::{print_fingerprint, run, PackSource, RunConfig};

/// The embedded pack bytes, baked in at compile time. `include_bytes!`
/// makes the artifact a static, read-only part of the executable image
/// (M26-005-D `PackBytes::Embedded` carrier path).
static EMBEDDED_PACK: &[u8] = include_bytes!(env!("VELQU_STANDALONE_PACK"));

#[derive(Debug, Parser)]
#[command(
    name = "velqu-standalone",
    about = "VelquJS production host (Rust + QuickJS) — standalone mode"
)]
struct Args {
    /// TCP port (default 3000, or VELQU_PORT / PORT env)
    #[arg(long)]
    port: Option<u16>,
    /// TCP host (default 127.0.0.1, or VELQU_HOST env)
    #[arg(long)]
    host: Option<String>,
    /// Versioned limits/config JSON (configVersion: 1); selects the
    /// file layer, overridable per-field by VELQU_* env
    #[arg(long)]
    config: Option<PathBuf>,
    /// Request logging mode: off | errors | full (default errors, or VELQU_LOG)
    #[arg(long)]
    log: Option<String>,
    /// Sample successful completion logs every N requests; 0 disables
    /// sampling (default 0, or VELQU_LOG_SAMPLE).
    #[arg(long)]
    log_sample: Option<u64>,
    /// Deployment boundary: reverse-proxy (default, loopback-only) or
    /// direct (explicit operator-owned public boundary; VELQU_PROXY_MODE).
    #[arg(long)]
    proxy_mode: Option<String>,
    /// M26-002-C: the explicit source-rebuild path — ignore embedded
    /// bytecode and evaluate the verified SOURCE bundle.
    #[arg(long)]
    no_bytecode: bool,
    /// M26-009-C: print the exact runtime fingerprint and the embedded
    /// pack's verification verdict WITHOUT serving; exit 0 when
    /// compatible, 2 when rejected.
    #[arg(long)]
    fingerprint: bool,
}

fn main() {
    let args = Args::parse();
    if args.fingerprint {
        let code = print_fingerprint(&PackSource::Embedded(EMBEDDED_PACK));
        std::process::exit(code);
    }
    let code = run(
        PackSource::Embedded(EMBEDDED_PACK),
        RunConfig {
            port: args.port,
            host: args.host,
            config: args.config,
            log: args.log,
            log_sample: args.log_sample,
            proxy_mode: args.proxy_mode,
            no_bytecode: args.no_bytecode,
            context_profile: None,
            // Standalone serves the serverless posture by default.
            service_profile: None,
        },
    );
    std::process::exit(code);
}
