//! velqu-runtime — the SHARED deployment mode: one runtime binary plus
//! app.qpack on disk (M26-009-A). Thin CLI over the shared startup
//! pipeline in `velqu_runtime` (lib.rs); the standalone mode
//! (`velqu-standalone`, feature `standalone`) embeds the pack instead.

use std::path::PathBuf;

use clap::Parser;
use velqu_runtime::{run, PackSource, RunConfig};

#[derive(Debug, Parser)]
#[command(
    name = "velqu-runtime",
    about = "VelquJS production host (Rust + QuickJS) — shared mode"
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
    /// Sample successful completion logs every N requests; 0 disables sampling.
    #[arg(long, default_value_t = 0)]
    log_sample: u64,
    /// M26-002-C: the explicit source-rebuild path — ignore embedded
    /// bytecode and evaluate the verified SOURCE bundle (sanctioned
    /// recovery for cross-target bytecode; rebuild the pack otherwise).
    #[arg(long)]
    no_bytecode: bool,
}

fn main() {
    let args = Args::parse();
    let code = run(
        PackSource::Path(args.pack),
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
