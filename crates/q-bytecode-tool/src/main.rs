//! velqu-bytecode — compile a Velqu QPack's bundle to QuickJS module bytecode
//! and embed it back into the pack.
//!
//! Usage:
//!   velqu-bytecode embed --pack <app.qpack> [--out <out.qpack>]
//!
//! The resulting pack loads ~30–50% faster at cold start: the QuickJS parser
//! and compiler are bypassed; the module is evaluated directly from bytecode.
//! The runtime falls back to source evaluation when:
//!   - the engine version does not match exactly (version-gated in QPack::verify)
//!   - no bundleBytecode field is present (plain source packs remain fully valid)

use sha2::{Digest, Sha256};
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let subcommand = args.get(1).map(|s| s.as_str()).unwrap_or("");
    if subcommand != "embed" {
        eprintln!("velqu-bytecode embed --pack <app.qpack> [--out <out.qpack>]");
        std::process::exit(1);
    }

    let pack_path = flag(&args, "--pack").unwrap_or_else(|| {
        eprintln!("--pack is required");
        std::process::exit(1);
    });
    let out_path = flag(&args, "--out").unwrap_or_else(|| pack_path.clone());

    // 1. Load and verify the source pack
    let mut pack = q_pack::QPack::load_and_verify(&PathBuf::from(&pack_path)).unwrap_or_else(|e| {
        eprintln!("pack verification failed: {e}");
        std::process::exit(1);
    });

    // 2. Compile bundle source → QuickJS module bytecode
    let bytecode_bytes = compile_bytecode(&pack.bundle);

    // 3. Hash + encode
    let bc_sha256 = hex(&Sha256::digest(&bytecode_bytes));
    let bc_b64 = q_pack::base64_encode(&bytecode_bytes);

    let bundle_len = pack.bundle.len();

    // 4. Embed
    pack.bundle_form = Some("module".to_string());
    pack.bundle_bytecode = Some(q_pack::BundleBytecode {
        quickjs: q_pack::ENGINE_VERSION.to_string(),
        binding: q_pack::ENGINE_BINDING.to_string(),
        endianness: if cfg!(target_endian = "big") {
            "big"
        } else {
            "little"
        }
        .to_string(),
        data: bc_b64,
    });
    pack.integrity.bytecode_sha256 = Some(bc_sha256);

    // 5. Write
    let out = serde_json::to_vec(&pack).expect("pack serialisation failed");
    std::fs::write(&out_path, &out).unwrap_or_else(|e| {
        eprintln!("write failed: {e}");
        std::process::exit(1);
    });

    println!(
        "velqu-bytecode embed: {} bytes bundle → {} bytes bytecode → {}",
        bundle_len,
        bytecode_bytes.len(),
        out_path
    );
}

// ---------------------------------------------------------------------------

fn compile_bytecode(bundle: &str) -> Vec<u8> {
    use rquickjs::{Context, Module, Runtime, WriteOptions, WriteOptionsEndianness};

    let rt = Runtime::new().expect("rquickjs Runtime::new");
    let ctx = Context::full(&rt).expect("rquickjs Context::full");

    ctx.with(|ctx| -> rquickjs::Result<Vec<u8>> {
        let module = Module::declare(ctx.clone(), "app.js", bundle)?;
        module.write(WriteOptions {
            endianness: WriteOptionsEndianness::Native,
            ..Default::default()
        })
    })
    .unwrap_or_else(|e| {
        eprintln!("QuickJS bytecode compilation failed: {e:?}");
        std::process::exit(1);
    })
}

fn flag(args: &[String], name: &str) -> Option<String> {
    for i in 0..args.len().saturating_sub(1) {
        if args[i] == name {
            return Some(args[i + 1].clone());
        }
    }
    None
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
