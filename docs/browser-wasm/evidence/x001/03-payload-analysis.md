# BWASM-X-001 — Payload and Release Size Budget Analysis

## 1. QuickJS WebAssembly Artifact Measurements

The compiled QuickJS-NG WebAssembly binary (`@jitl/quickjs-ng-wasmfile-release-sync@0.32.0`, compiled via Emscripten with `-Oz -flto`):

| Component | Raw (Bytes) | Gzip -9 (Bytes) | Brotli -11 (Bytes) |
|---|---|---|---|
| `emscripten-module.wasm` | 528,642 | 276,787 | 241,890 |
| JS Glue & FFI Bindings | 15,310 | 5,420 | 4,620 |
| **Total Engine Payload** | **543,952** | **282,207** | **246,510** |

---

## 2. Comparison Against Ratified Release Budgets

The Browser-WASM release budgets were ratified in `docs/browser-wasm/evidence/budgets.json` (BWASM-D-004) based on a target mid-tier 4G mobile baseline:

| Budget Component | Ratified Ceiling (Brotli) | QuickJS-WASM Addition | Impact on Budget |
|---|---|---|---|
| `runtime_js_glue` | **51,200 B** (50 KiB) | 246,510 B | **EXCEEDED BY 4.8x** |
| `base_wasm_kernel` | **512,000 B** (500 KiB) | 241,890 B | **Consumes 47.2% of kernel budget** |
| `per_app_handler_bundle` | **102,400 B** (100 KiB) | N/A | No change |
| `total_initial_transfer` | **1,048,576 B** (1 MiB) | +246,510 B | **Bloats baseline by +58.6%** |

### Budget Impact Breakdown:
1. **The JS Glue Budget**: The entire runtime JS glue budget is 51.2 KiB. Shipping QuickJS-WASM requires 246.5 KiB compressed, immediately violating the glue budget by nearly 500%.
2. **Total Transfer Inflation**:
   - Current clean baseline (`q_browser_kernel_bg.wasm` + `@velqu/browser-runtime` + app): ~420 KiB brotli.
   - Baseline with QuickJS-WASM: ~666.5 KiB brotli.
   - An additional ~246 KiB must be downloaded, decompressed, and compiled over the network on every cold visit.
3. **Core Project Invariant Violation**:
   `packages/browser-runtime/test/budgets.test.ts` (Acceptance criterion 1) explicitly enforces:
   ```typescript
   it("core projects do not download optional SQL or parity-engine assets", () => {
     // files must not contain "quickjs.wasm" or "quickjs-wasm"
   });
   ```
   Making QuickJS-WASM mandatory or default would directly break this ratified regression gate.
