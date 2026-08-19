# ADR-0018 — Post-M2 Architecture Roadmap and Host Pipeline Optimization

- **Status:** Accepted (2026-08-18)
- **Deciders:** Antigravity Engineering, Architecture Review
- **Consulted:** ADR-0001, ADR-0006, ADR-0008, ADR-0014, ADR-0017
- **Informs:** M2.2.1–M2.8, M3, M4

## Context

Milestones M0–M2 proved the fundamental cold-start and memory thesis of Velqu:
1. **Cold Start**: Velqu achieves 2.6–3.0 ms p50 process-to-first-response (30–34× faster than Elysia 2 AOT at 132–145 ms).
2. **Idle RSS**: 6.2 MiB p50 vs 82.6 MiB for Elysia 2.
3. **Bytecode Scaling**: QuickJS module bytecode embedding (`velqu-bytecode embed`, ADR-0017) saves 3.32 ms (−22.2%) at 1,000 routes.
4. **Single-Worker Throughput**: The synchronous JS runner and log modes deliver 118k–130k req/s on native liveness (C0) and ~58k–65k req/s across text, JSON, and parameterized routes.

Performance audits demonstrate that remaining single-worker throughput and route-scaling overheads originate in host-side plumbing rather than the JavaScript engine:
- String handler lookups (`BTreeMap<String, Persistent<Function>>`)
- Global `RequestStore` mutex contention
- Eager query parsing, header cloning, and string request IDs
- Per-request RoutePlan reconstruction and response status string parsing
- JSON pack deserialization and base64 bytecode decoding at startup

QuickJS-NG with `rquickjs` remains the optimal primary engine pairing for Velqu’s cold-start-first, memory-bounded, embeddable, deterministic execution model. Alternative engines (V8, JavaScriptCore, Wasm/Javy, Zig) are rejected for the primary runtime.

## Decision

1. **Retain Primary Stack**: Pinned QuickJS-NG (0.15.1) + `rquickjs` (0.12.2) + Rust / Tokio / Hyper.
2. **Authorize Post-M2 Milestone Sequence**:
   - **M2.2.1 — Scheduler Correctness Closure**: Bounded microtask checkpoints for synchronous handlers scheduling microtasks; preserve the zero-drain fast path for microtask-free handlers.
   - **M2.3 — Compiled Runtime IR & Numeric RoutePlan**: Numeric `HandlerId`, `PolicyId`, `SchemaId`; `Vec<Persistent<Function>>` handler tables; fused route entry functions; compiled terminal router automaton.
   - **M2.4 — Zero-Copy Ingress & Worker-Local Bridge**: Contract-driven field admission (`FieldNeeds` bitset); zero-copy header and query borrowing; worker-local request slab eliminating global mutexes; numeric request IDs.
   - **M2.5 — Schema-Specialized JSON Pipeline**: Schema-generated direct encoders/decoders combining validation and serialization into a single pass; route-level encoder/decoder selection.
   - **M2.6 — Binary QPack v2**: Zero-copy sectioned binary pack format with raw bytecode, compiled route automaton, and runtime fingerprinting.
   - **M2.7 — Capability Linker & Minimal Web Runtime**: Modular host capabilities, WinterTC Web APIs (URL, TextEncoder, AbortController, Crypto random), and custom-context profiles.
   - **M2.8 — Native Outbound Fetch**: High-performance connection pool with DNS/TLS caching, AbortSignal propagation, SSRF protections, and strict stream backpressure.
   - **M3 — Multi-Worker Service Runtime**: Independent QuickJS runtimes per worker thread with serverless (1-worker), service (adaptive), and throughput (N-worker) profiles.
   - **M4 — Developer Experience & Public Alpha**: Actual-runtime `velqu dev` mode, direct Treaty test dispatcher, alpha proof service app, and public release gates.

## Consequences

- Work progresses through incremental, test-backed PRs without architectural pivots.
- The single-worker path is fully optimized and closed before multi-worker scaling is introduced.
- Strict release gates and reproducible benchmark evidence are maintained across all milestones.
