// BWASM-K-004 — Node WASI runner for wasm32-wasip1 test binaries.
//
// Lets `cargo test --target wasm32-wasip1` EXECUTE the real Rust test
// suite on-target (instantiated as WebAssembly) without a browser or a
// separate wasm runtime install. Cargo invokes this as the target
// runner (see .cargo/config.toml):
//
//   node --experimental-wasi-unstable-preview1 scripts/wasm-wasi-node-runner.mjs <wasm> [args...]
//
// Exit code propagation keeps cargo's pass/fail semantics intact.
import { WASI } from "node:wasi";
import { readFile } from "node:fs/promises";

const wasmPath = process.argv[2];
if (!wasmPath) {
  console.error("usage: wasm-wasi-node-runner.mjs <module.wasm> [args...]");
  process.exit(2);
}
const args = [wasmPath, ...process.argv.slice(3)];
const bytes = await readFile(wasmPath);
const wasi = new WASI({
  version: "preview1",
  args,
  env: process.env,
  preopens: {},
});
const { instance } = await WebAssembly.instantiate(bytes, wasi.getImportObject());
try {
  wasi.start(instance);
} catch (err) {
  if (err && typeof err.exitCode === "number") {
    process.exit(err.exitCode);
  }
  throw err;
}
