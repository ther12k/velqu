# BWASM-X-001 — Comparative Benchmark Report: QuickJS-WASM vs Native Browser JS

## Executive Summary

To evaluate whether compiling QuickJS-NG to WebAssembly materially improves Velqu browser deployment, we benchmarked cold startup latency, per-request execution latency, and memory characteristics against native browser JavaScript execution.

**Findings:**
1. **Cold Startup**: QuickJS-in-WASM incurs a **59.02 ms** startup cost (fetching/instantiating WASM, creating a runtime, initializing the context, and parsing bundle source) vs **0.04 ms** for native JavaScript. That is a **1,475x startup penalty**.
2. **Request Throughput**: Under 1,000 handler invocations, QuickJS-in-WASM averaged **497.5 µs/req** vs **1.6 µs/req** for native JavaScript. That is a **310x throughput slowdown**.
3. **Memory Overhead**: QuickJS-in-WASM allocates an initial **16 MiB WebAssembly linear memory pool** plus QuickJS C heap structures, while native Worker handlers utilize the browser's existing, highly optimized JS runtime heap.
4. **Disposal / Lifecycle**: Unclean reference handling during runtime disposal triggers hard Emscripten/WASM aborts (`Assertion failed: list_empty(&rt->gc_obj_list), at: JS_FreeRuntime`), introducing crash risks absent from native Worker execution.

---

## 1. Cold Startup Latency

| Stage | QuickJS-in-WASM | Native Browser JS | Delta |
|---|---|---|---|
| WASM Engine Instantiation | 42.45 ms | 0.00 ms | +42.45 ms |
| Runtime & Context Creation | 9.87 ms | 0.00 ms | +9.87 ms |
| Handler Bundle Evaluation | 6.70 ms | 0.04 ms | +6.66 ms |
| **Total Cold Startup** | **59.02 ms** | **0.04 ms** | **+58.98 ms (1,475x)** |

In mobile or throttled environments (where WASM compilation takes longer), this 59 ms baseline compounds significantly, directly threatening the 500 ms warm / 2,000 ms cold startup budgets set in `budgets.json`.

---

## 2. Request Execution Latency (1,000 Invocations)

The test executed a standard Velqu handler fixture performing parameter extraction, computation, and structured JSON response generation:

```typescript
function handle(ctx) {
  let sum = 0;
  for (let i = 0; i < 100; i++) sum += i;
  return { kind: "response", status: 200, body: { sum, user: ctx.params?.id ?? "anonymous" } };
}
```

### Latency Distribution

| Metric | QuickJS-in-WASM | Native Browser JS | Slowdown Factor |
|---|---|---|---|
| Total (1,000 reqs) | 497.48 ms | 1.60 ms | **310.3x** |
| Mean Latency | 497.5 µs | 1.6 µs | **310.3x** |
| p50 Latency | 485.0 µs | 1.4 µs | **346.4x** |
| p95 Latency | 530.0 µs | 2.1 µs | **252.4x** |
| p99 Latency | 610.0 µs | 3.5 µs | **174.3x** |

### Why QuickJS-in-WASM is 310x Slower
1. **Interpreted handler execution inside WASM**: QuickJS is a C-based bytecode interpreter. Compiled to WebAssembly, the QuickJS interpreter itself runs as WASM code, so every handler bytecode dispatch happens inside that interpreted-in-WASM C loop — instead of letting the browser's optimized JS engine execute the handler directly. (Note: modern browsers typically *compile* WASM, they do not interpret it; the penalty comes from running a second, non-JIT JavaScript engine plus the host/guest boundary, not from WASM interpretation.)
2. **Boundary Serialization**: Data entering QuickJS must be serialized across the host/guest WebAssembly linear memory boundary (JS object -> JSON string or FFI heap -> QuickJS value -> QuickJS execution -> return value -> JS object).
3. **No JIT**: QuickJS has no JIT compiler; modern browser engines (V8 TurboFan/Sparkplug, JSC FTL, SpiderMonkey Ion) compile hot loops to native machine instructions.

### Scope of these numbers

These are **X-001 qualification microbenchmark** results on the tested
QuickJS-WASM implementation (`@jitl/quickjs-ng-wasmfile-release-sync@0.32.0`,
handler-invocation fixture), not end-to-end Velqu request-pipeline
benchmarks. The defensible phrasing is: *in the X-001 qualification
microbenchmark, the tested QuickJS-WASM implementation was ~1,475x
slower at cold initialization and ~310x slower per handler invocation
than direct browser JavaScript.* Do not promote these factors into
product/marketing claims about "Velqu on QuickJS-WASM".

---

## 3. Memory & GC Characteristics

- **Linear Memory Allocation**: WebAssembly modules compiled via Emscripten allocate an initial memory buffer (default 16 MiB, dynamically expandable).
- **Dual Heaps**: The application now maintains two distinct heaps:
  - The Browser V8/JSC JS heap.
  - The QuickJS C runtime heap inside WASM linear memory.
- **Reference Tracking Hazard**: Every handle passed between JS and QuickJS must be manually disposed (`res.value.dispose()`). If any reference remains active when a context or runtime is closed, QuickJS aborts the entire WebAssembly instance:
  ```text
  RuntimeError: Aborted(Assertion failed: list_empty(&rt->gc_obj_list), at: JS_FreeRuntime)
  ```
  In a production web application, this hard crash brings down the entire worker, completely negating any theoretical sandboxing benefit.
