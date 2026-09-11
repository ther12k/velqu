# BWASM-X-001 / OD-054 — Formal GO or NO-GO Decision Record

## Decision Record Details

- **Decision ID**: `BWASM-X-001` / `OD-054`
- **Program Phase**: `07_optional_parity`
- **Title**: QuickJS-NG WebAssembly Engine Parity Spike Evaluation
- **Date**: 2026-09-11
- **Status**: **RESOLVED — VERDICT: UNAMBIGUOUS NO-GO**
- **Authority**: Program rules per `docs/browser-wasm/tasks/07_optional_parity/BWASM-X-001-...md`, ADR-0037 §8, ADR-0038, and `docs/browser-wasm/OWNER_DECISIONS.md`.

---

## 1. Scored Evaluation Matrix

The spike established six quantitative and architectural evaluation dimensions. A **GO** verdict requires achieving at least **24 / 30** points with no single dimension scoring below 3.

| Dimension | Weight | Score (1-5) | Weighted Justification |
|---|---|---|---|
| **1. Engine Version Parity** | High | **1 / 5** | **Critical Failure**. Upstream QuickJS WASM builds are on QuickJS-NG `0.12.1`, while native Velqu pins `0.15.1`. Precompiled `.qpack` bytecode is rejected due to opcode breaking changes. Direct `wasm32-unknown-unknown` Cargo compilation of `q-engine-quickjs` fails at both `rquickjs-sys` (missing wasm32 bindings) and `mio/tokio`. |
| **2. Payload Budget Impact** | High | **1 / 5** | **Critical Failure**. QuickJS-WASM adds 528.6 kB raw / 241.9 kB brotli. This exceeds the ratified `runtime_js_glue` ceiling (51.2 kB) by 4.7x and inflates the total initial transfer by +58.6%, directly violating `budgets.json`. |
| **3. Latency & Startup Performance** | High | **1 / 5** | **Critical Failure**. Measured throughput is **310.3x slower** (497.5 µs vs 1.6 µs per request). Cold startup is **1,475x slower** (59.02 ms vs 0.04 ms). Running an interpreted C engine inside WebAssembly imposes unacceptable CPU overhead. |
| **4. Cancellation & Fault Recovery** | Medium | **3 / 5** | **Marginal**. QuickJS supports `JS_SetInterruptHandler` for loop cancellation, but single-threaded execution blocks incoming abort signals unless hosted in a Worker anyway. Unclean terminations cause assertion aborts in C memory disposal (`list_empty(&rt->gc_obj_list)`). |
| **5. Developer Experience & Debugging** | Medium | **2 / 5** | **Poor**. Code evaluated in QuickJS-WASM linear memory is invisible to browser DevTools. Breakpoints, line-by-line debugging, and CPU profiling are lost. Native browser Workers provide full source-map debugging. |
| **6. Toolchain & Maintenance Risk** | High | **1 / 5** | **Critical Failure**. Adopting QuickJS-WASM would require Velqu to fork and maintain a custom C-to-WASM Emscripten build pipeline for `quickjs-ng 0.15.1`, maintain custom JS bindings, and synchronise dual engine upgrades indefinitely for negative performance. |
| **TOTAL SCORE** | | **9 / 30** | **THRESHOLD NOT MET (Required: >= 24/30)** |

---

## 2. Decision Verdict: NO-GO

Executing TypeScript route handlers in QuickJS-NG compiled to WebAssembly **fails across all critical performance, size, version parity, and maintenance thresholds**.

### Primary Drivers of the NO-GO Verdict:
1. **The "Engine Parity" Hypothesis is Disproved**: Because the WASM build cannot run 0.15.1 bytecode, it must execute JavaScript source text. But modern browser engines (V8, JavaScriptCore, SpiderMonkey) execute the same JavaScript source text with complete ES2023/ES2024 compliance, full DevTools support, and zero extra download bytes.
2. **Catastrophic Performance Degradation**: A 310x latency penalty and 1,475x cold-start penalty are incompatible with Velqu’s core design thesis ("Cold-start first", ADR-0002).
3. **Severe Payload Bloat**: Adding 242 kB of compressed WASM for an engine the browser already contains is a violation of minimal-core discipline.
4. **Maintenance Fork Prohibition**: The task guardrails explicitly prohibit "maintaining an unreviewed permanent fork by default" and "claiming same-engine parity with a different QuickJS-NG version."

---

## 3. Architecture Confirmation

Per the task acceptance criteria:
> *"A NO-GO decision leaves the default Worker-based Browser-WASM target unaffected."*

The ratified **Hybrid Browser-WASM Architecture** (ADR-0037, ADR-0038, BWASM-D-001) is permanently confirmed as the production standard:
- **Rust Kernel (`q-browser-kernel`)**: Compiles to WebAssembly, owning route matching, schema validation, QPack verification, capability authorization, and problem mapping.
- **Handler Execution**: Executes in isolated browser Web Workers using the browser's native, highly optimized JavaScript engine (`@velqu/browser-runtime`).
- **Isolation Boundary**: Governed by the browser origin and sandboxed iframe deployment model (ADR-0038), backed by supervisor kill-and-replace Worker recovery (`WorkerHost`).
- **Zero Runtime Changes**: The repository remains clean; no experimental QuickJS WASM binaries or shims enter the core build.

---

## 4. Disposition of Decisions and Tasks

- **OD-054 (QuickJS Promotion Rule)**: Resolved as **REJECTED** based on measured evidence in `01-toolchain-inventory.md` through `05-cancellation-and-recovery.md`.
- **BWASM-X-001**: Closed as **PASS** (spike completed, evidence gathered, NO-GO verdict rendered, all acceptance criteria satisfied).
