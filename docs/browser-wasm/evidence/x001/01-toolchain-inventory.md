# BWASM-X-001 — Toolchain Inventory & Version Skew Classification

## 1. Native Velqu Baseline

Production Velqu execution strictly pins:
- **Crate**: `crates/q-engine-quickjs`
- **Rust Binding**: `rquickjs = "=0.12.2"` (features: `["full-async"]`)
- **Vendored Engine**: `quickjs-ng 0.15.1` (compiled via native C compiler into host machine code)
- **Supported Architecture**: Linux x86_64 glibc (and aarch64), single-worker OS thread owning the QuickJS runtime (ADR-0008, ADR-0018).

## 2. Upstream Rust/Cargo wasm32-unknown-unknown Portability

Attempting to compile `q-engine-quickjs` directly to `wasm32-unknown-unknown` fails at two distinct structural layers:

### Layer A: `rquickjs-sys` C Binding Gap
```text
Compiling rquickjs-sys v0.12.2
error: couldn't read `/home/ther12k/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rquickjs-sys-0.12.2/src/bindings/wasm32-unknown-unknown.rs`: No such file or directory (os error 2)
  --> /home/ther12k/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rquickjs-sys-0.12.2/src/lib.rs:19:1
   |
19 | include!(concat!("bindings/", bindings_env!("TARGET"), ".rs"));
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: rquickjs-sys@0.12.2: rquickjs probably doesn't ship bindings for platform `wasm32-unknown-unknown(n/a)`. try the `bindgen` feature instead.
```
- `rquickjs-sys` does not publish pre-generated Rust bindings for `wasm32-unknown-unknown`.
- Compiling with `bindgen` requires an Emscripten or WASI C sysroot that does not integrate cleanly into Cargo's default wasm32 cross-compilation target.

### Layer B: `tokio` / `mio` Incompatibility
```text
Checking mio v1.2.2
error: This wasm target is unsupported by mio. If using Tokio, disable the net feature.
  --> mio-1.2.2/src/lib.rs:44:1
   |
44 | compile_error!("This wasm target is unsupported by mio. If using Tokio, disable the net feature.");
```
- `q-engine-quickjs` depends on Tokio's async primitives. Even with network features pruned, Mio emits 48 unresolved symbol and type errors for IO abstractions on `wasm32-unknown-unknown`.

## 3. Upstream WebAssembly QuickJS Distribution

In the JavaScript/WebAssembly ecosystem, the primary distribution providing QuickJS-NG in WebAssembly is `@jitl/quickjs-ng-wasmfile-release-sync@0.32.0` (compiled via Emscripten with `-Oz -flto`):
- **Vendored Version**: `quickjs-ng 0.12.1`
- **Native Version**: `quickjs-ng 0.15.1`
- **Version Skew**: 3 minor releases out of sync.

### Classification of Engine Mismatch
1. **QPack Bytecode Format Incompatibility**:
   `q-bytecode-tool` compiles TypeScript handlers to QuickJS bytecode matching QuickJS-NG 0.15.1 opcode definitions. QuickJS-NG 0.12.1 rejects 0.15.1 bytecode as corrupted/invalid. Exact bytecode parity cannot be achieved without compiling 0.15.1 from source.
2. **ECMAScript Standards Gap**:
   Between 0.12.1 and 0.15.1, QuickJS-NG introduced new ES2023/ES2024 language primitives and regex engine updates. Handlers written against modern standards behave differently between the native server and the WASM engine.
3. **Double-Engine Divergence**:
   The browser already has a modern, highly conforming JavaScript engine (V8 in Chromium, JavaScriptCore in Safari, SpiderMonkey in Firefox). Running QuickJS 0.12.1 inside the browser creates a *third* dialect rather than achieving parity with the browser environment.
