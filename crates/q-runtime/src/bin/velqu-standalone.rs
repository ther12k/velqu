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
use velqu_runtime::{run, PackSource, RunConfig};

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
    /// Sample successful completion logs every N requests; 0 disables sampling.
    #[arg(long, default_value_t = 0)]
    log_sample: u64,
    /// M26-002-C: the explicit source-rebuild path — ignore embedded
    /// bytecode and evaluate the verified SOURCE bundle.
    #[arg(long)]
    no_bytecode: bool,
}

fn main() {
    let args = Args::parse();
    let code = run(
        PackSource::Embedded(EMBEDDED_PACK),
        RunConfig {
            port: args.port,
            host: args.host,
            config: args.config,
            log: args.log,
            log_sample: args.log_sample,
            no_bytecode: args.no_bytecode,
        },
    );
    std::process::exit(code);
}
