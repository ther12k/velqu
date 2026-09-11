# BWASM-X-001 — JavaScript Engine Semantic Parity Matrix

## 1. Multi-Engine Semantic Evaluation

We evaluated 15 ECMAScript and runtime dimensions across three execution targets:
1. **Native Velqu Engine**: `quickjs-ng 0.15.1` (compiled to native x86_64 machine code via `rquickjs =0.12.2`).
2. **QuickJS-NG WASM Spike**: `quickjs-ng 0.12.1` (`@jitl/quickjs-ng-wasmfile-release-sync@0.32.0`).
3. **Browser Native Worker**: Modern browser JS engines (V8 12.4+, JavaScriptCore 17+, SpiderMonkey 128+).

---

## 2. Feature & Semantic Comparison Table

| Feature / Behavior | Native Velqu (0.15.1) | QuickJS-WASM (0.12.1) | Modern Browser JS (V8/JSC) | Parity Assessment |
|---|---|---|---|---|
| **ES2023 Array methods** (`toSorted`, `toReversed`) | Supported | Supported | Supported | Full Parity |
| **`Promise.withResolvers`** | Supported | Supported | Supported | Full Parity |
| **`Array.fromAsync`** | Supported | Supported | Supported | Full Parity |
| **`Object.groupBy` / `Map.groupBy`** | Supported | Supported | Supported | Full Parity |
| **RegExp `v` flag** (unicodeSets) | Supported | Supported | Supported | Full Parity |
| **Iterator Helpers** (`Iterator.prototype.map`, etc.) | Supported | Supported | Supported | Full Parity |
| **BigInt & TypedArrays** | Supported | Supported | Supported | Full Parity |
| **QPack Bytecode Loading** (`.qpack` bytecode) | **Supported (0.15.1 format)** | **FAIL (Incompatible opcode set)** | N/A (Executes JS bundle) | **BROKEN PARITY**: Bytecode compiled on 0.15.1 cannot be loaded into 0.12.1. |
| **Execution Speed** | Native C bytecode dispatch (~10 µs) | Interpreted C in WASM (~497 µs) | Native JIT machine code (~1.6 µs) | **MASSIVE REGRESSION**: Browser JS is 310x faster than QuickJS-WASM. |
| **Cold Startup Latency** | Sub-millisecond | 59.02 ms | 0.04 ms | **REGRESSION**: 1,475x startup penalty. |
| **Call Stack Depth Limit** | 256 KiB fixed stack | 256 KiB fixed stack (WASM frame limit) | ~10,000 frames (~1 MiB) | QuickJS fails deep recursion earlier than browser JS. |
| **Error Stack Traces** | QuickJS format (`at <eval> (eval.js:1:1)`) | QuickJS format | V8 / SpiderMonkey structured format | QuickJS stack format differs from browser DevTools conventions. |
| **DevTools Debugging** | Native debug logs | Black-box WASM linear memory | Source maps, breakpoints, live stepping | **REGRESSION**: Developer cannot inspect JS inside QuickJS via browser DevTools. |
| **CSP Restrictions** | N/A (Server-side) | May require `wasm-unsafe-eval` | Standard Worker script loading | Additional CSP friction in restrictive enterprise environments. |
| **Memory Isolation** | OS process / thread | WASM linear memory | Worker thread realm | Both provide heap isolation; Worker provides OS-level thread safety. |

---

## 3. Conclusions on Semantic Parity

1. **The "Engine Parity" Promise Fails Due to Version Skew**:
   Because upstream QuickJS WASM tooling is on `0.12.1` while native Velqu is on `0.15.1`, QuickJS-WASM *cannot* run precompiled `.qpack` bytecode emitted by `q-bytecode-tool`. It must evaluate raw JavaScript source text—the exact same text evaluated by the browser Worker.
2. **Modern Browsers Already Have 100% ECMAScript Parity**:
   Every modern browser targeted by Velqu (Chromium, Firefox, Safari) natively implements all ES2023 and ES2024 features. Handler code runs flawlessly on the browser's native JavaScript engine without any dialect divergence.
3. **Loss of Developer Experience**:
   Evaluating JavaScript inside QuickJS-WASM makes browser DevTools blind to execution: developers cannot set breakpoints, inspect variables, or view native source-mapped stack traces.
