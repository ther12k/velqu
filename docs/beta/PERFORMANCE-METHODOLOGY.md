# Performance Methodology and Measurement Invariants (public beta)

This document establishes the normative performance methodology, benchmarking standards, and measurement invariants for Velqu during the public beta (`0.1.0-beta.1`).

## Core Invariants

1. **No Universal Performance Claims**: Velqu does not claim universal superiority over Node, Bun, Elysia, or other JavaScript/TypeScript runtimes. Performance is strictly workload-, fixture-, and hardware-dependent.
2. **Separation of Normative Targets vs Measured Results**: Target performance criteria (e.g. startup budgets, memory bounds) must never be presented as measured evidence. All measured claims must point to raw sample artifacts under `benchmarks/raw/` with verifiable checksums in `benchmarks/manifest.json`.
3. **Reproducible Methodology**: Benchmarks must use identical schemas, datasets, seeds, request bodies, timeouts, and connection parameters across all tested candidates.
4. **Complete Distribution Reporting**: Every published benchmark must report sample counts ($n$), mean, p50, p95, and p99 metrics. Single-number or cherry-picked metrics are rejected.

---

## QuickJS Bytecode vs. JIT Compilation

A common misconception is that ahead-of-time bytecode compilation is identical to native JIT (Just-In-Time) compilation.

| Attribute | Velqu (QuickJS-NG Bytecode) | JIT Engines (V8, JavaScriptCore) |
|---|---|---|
| **Compilation Target** | Compact QuickJS-NG bytecode in QPack | Machine code (x86-64, ARM64) generated at runtime |
| **Startup Overhead** | Minimal (no parse/transpile phase at startup) | Higher (warm-up, deoptimizations, JIT compilation pauses) |
| **Memory Footprint** | Bounded (strict 32 MiB heap, 512 KiB stack default) | Higher, dynamic memory growth |
| **Long-Running Steady-State Throughput** | Interpreted bytecode execution; predictable latency | Highly optimized machine code; superior on raw compute |
| **Determinism** | Strict, reproducible execution without JIT deopts | Variable performance profiles during warm-up |

### Summary
Velqu's ahead-of-time bytecode compilation into `app.qpack` eliminates runtime TypeScript transpilation and JavaScript parsing overhead, drastically lowering cold-start latency and resident set size (RSS). However, QuickJS remains a bytecode interpreter; for compute-heavy, CPU-bound workloads where JIT machine code excels, Node/Bun will outpace QuickJS in raw throughput.

---

## Benchmarking Suites & Categories

Velqu maintains three primary benchmark tiers:

### 1. Cold-Start and Startup Profile
- Measures: process exec to `/health/ready` or first request completion.
- Scenarios: route counts from 1 to 1,000 routes.
- Raw evidence: `benchmarks/raw/cold-start/` and `benchmarks/raw/route-count/`.
- Key metric: startup duration (milliseconds) vs route scale.

### 2. Micro-benchmarks & Bridge Efficiency
- Measures: Rust-to-QuickJS boundary crossing, schema validation overhead, and lazy body materialization.
- Raw evidence: `benchmarks/raw/bridge/` and `benchmarks/raw/codec/`.

### 3. Real-World Workload Matrix
Defined in `docs/beta/workstreams/REAL_WORLD_BENCHMARKS.md`:
- **W0 (Controlled I/O)**: auth policy → upstream fetch delay → dynamic JSON response across latency brackets.
- **W1 (Primary-Key Read)**: JWT verification → UUID validation → indexed SELECT → JSON.
- **W2 (Transactional Order Write)**: JWT → validation → multi-table transaction → 201 response.
- **W3 (Paginated Join/Aggregation)**: cursor-based pagination and joined queries.
- **W4 (Outbound Fan-Out)**: DB query + concurrent upstream HTTP fetches.
- **W5 (CPU/JIT Crossover)**: gradual increase in pure JavaScript computation to measure break-even thresholds against JIT engines.

---

## Running Benchmarks Locally

To execute and validate benchmark suites:

```bash
# Validate benchmark evidence and manifest consistency
python3 scripts/validate-benchmark-evidence.py

# Verify manifest against local binaries
./scripts/validate-okf
```

---

## Non-Goals & Disclosures

- **Non-SLA**: Velqu `0.1.0-beta.1` is a public beta release and carries no SLA or production-readiness claim.
- **Trusted Code Execution**: QuickJS executes trusted application code only. It is not an untrusted multi-tenant sandbox.
- **Hardware Variation**: Benchmark reports are tied to the specific hardware on which raw measurements were gathered. Operators should run the benchmark harness on their target infrastructure.
