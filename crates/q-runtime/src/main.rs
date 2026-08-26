//! velqu-runtime — the SHARED deployment mode: one runtime binary plus
//! app.qpack on disk (M26-009-A). Thin CLI over the shared startup
//! pipeline in `velqu_runtime` (lib.rs); the standalone mode
//! (`velqu-standalone`, feature `standalone`) embeds the pack instead.

use std::path::PathBuf;

use clap::Parser;
use velqu_runtime::{print_fingerprint, run, PackSource, RunConfig};

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
    /// M26-009-C: print the exact runtime fingerprint and, with --pack,
    /// the full pack verification verdict WITHOUT serving; exit 0 when
    /// compatible, 2 when rejected.
    #[arg(long)]
    fingerprint: bool,
    /// M27-003-D: force a context profile (full | web | minimal) for
    /// compatibility testing; default full — the always-available
    /// baseline.
    #[arg(long)]
    context_profile: Option<String>,
}

fn main() {
    let args = Args::parse();
    if args.fingerprint {
        let code = print_fingerprint(&PackSource::Path(args.pack.clone()));
        std::process::exit(code);
    }
    let code = run(
        PackSource::Path(args.pack),
        RunConfig {
            port: args.port,
            host: args.host,
            config: args.config,
            log: args.log,
            log_sample: args.log_sample,
            no_bytecode: args.no_bytecode,
            context_profile: args.context_profile,
        },
    );
    std::process::exit(code);
}
