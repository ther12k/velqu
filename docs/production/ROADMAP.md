# Production-Readiness Roadmap

## Critical path

```text
BASE → M2.3-r2 → M2.4 → M2.5 → M2.6 → M2.7 → M2.8
     → M3 → M4 private alpha → M5 technical production candidate
     → M6 hardened readiness → M7 release candidate → M8 GA
```

No date estimate is embedded in the engineering gate. Progress is determined by evidence, not calendar pressure.

## Parallel lanes

- **Benchmark lane:** real-world harness, competitor fixtures, controlled upstream, and CPU/JIT crossover may advance without introducing out-of-order Velqu runtime features.
- **Documentation lane:** API guides, examples, diagnostics catalog, and operator runbooks can advance against stable milestone outputs.
- **Platform/supply-chain lane:** CI matrix, SBOM, provenance, and reproducibility preparation may begin early, but release claims wait for M6/M7.
- **Owner lane:** repository, license, platform promise, governance, and release authority must be decided by their required gates.

## BASE — Baseline Freeze and Program Integration

The current M2.3-r1 baseline, roadmap authority, evidence model, and review packet format are frozen before further code changes.

**Key work:**
- `BASE-001` Freeze the reviewed source baseline
- `BASE-002` Adopt ADR-0019 production-readiness program
- `BASE-003` Install the production master agent prompt
- `BASE-004` Create machine-readable task and evidence ledgers
- `BASE-005` Strengthen source-to-evidence verification

**Exit gate:** `BASE-GATE` — Authorize implementation only after the baseline, roadmap, prompt, evidence ledger, and review conventions are coherent.

## M23R2 — M2.3-r2 — Exact Numeric Runtime IR and Compiled Router Closure

Current-pack execution is fully numeric, fail-before-ready, route plans are exact, schema identities are operational, and the router resolves directly to a numeric RoutePlan.

**Key work:**
- `M23R2-001` Define explicit numeric and legacy engine load plans
- `M23R2-002` Enforce exact function-vector and manifest equivalence
- `M23R2-003` Enforce exact RoutePlan equivalence
- `M23R2-004` Operationalize RouteId, PolicyId, and SchemaId
- `M23R2-005` Compile FieldNeeds into a verified bitset
- `M23R2-006` Bind the numeric execution graph to pack integrity
- `M23R2-007` Implement numeric terminal router automaton
- `M23R2-008` Remove current-pack string dispatch and duplicate references
- `M23R2-009` Finish terminal settlement retention hardening

**Exit gate:** `M23R2-GATE` — Close exact numeric artifact loading, router scope, evidence, and performance before M2.4 consumes RoutePlan.

## M24 — M2.4 — Zero-Copy Ingress and Worker-Local Request Bridge

Requests route before decoding, unread fields are never materialized, and request state is owned by the QuickJS worker without a process-wide request-store mutex.

**Key work:**
- `M24-001` Freeze ingress ownership and backpressure design
- `M24-002` Route before request materialization
- `M24-003` Introduce worker-local generation-checked request slab
- `M24-004` Capture path parameters as byte ranges
- `M24-005` Implement declared-header lazy access
- `M24-006` Implement lazy query and cookie decoding
- `M24-007` Implement bounded read-once body admission
- `M24-008` Replace per-request JS closure construction with native-backed prototypes
- `M24-009` Add ingress and bridge observability

**Exit gate:** `M24-GATE` — Prove lazy materialization, ownership safety, cancellation, and meaningful fixed-overhead reduction.

## M25 — M2.5 — Schema-Specialized Input and JSON Output Pipeline

The compiler selects measured, schema-aware decoders and encoders while maintaining exact Treaty/OpenAPI/runtime semantics.

**Key work:**
- `M25-001` Define canonical Schema IR v2 and compatibility rules
- `M25-002` Build reproducible decoder/encoder strategy benchmark
- `M25-003` Generate params/query/header decoders
- `M25-004` Generate JSON body decoders
- `M25-005` Generate status-specific response encoders
- `M25-006` Generate RFC 9457 problem encoders
- `M25-007` Implement explicit generic and Web fallback paths
- `M25-008` Unify OpenAPI, Treaty, lock, and runtime schema projection
- `M25-009` Add codec fuzzing and differential tests

**Exit gate:** `M25-GATE` — Select route-level codecs from evidence and close semantic parity.

## M26 — M2.6 — Binary QPack v2 and Reproducible Artifact ABI

Production startup maps a deterministic binary pack with raw bytecode, precompiled router/runtime IR, strict ABI fingerprinting, and no JSON/base64 reconstruction.

**Key work:**
- `M26-001` Accept QPack v2 binary format ADR
- `M26-002` Define strict runtime and bytecode fingerprint
- `M26-003` Encode compiled router, RoutePlans, schemas, and functions as binary sections
- `M26-004` Embed raw QuickJS bytecode without base64
- `M26-005` Implement zero-copy or bounded-copy pack reader
- `M26-006` Add execution integrity and optional authenticity hooks
- `M26-007` Guarantee reproducible release packs
- `M26-008` Provide explicit v1 compatibility and migration tool
- `M26-009` Build shared-runtime and standalone deployment artifacts

**Exit gate:** `M26-GATE` — Prove startup reconstruction is gone and artifact trust/versioning is fail-closed.

## M27 — M2.7 — Capability Linker and Minimal Web Runtime

Only declared native capabilities are linked, lifecycle and cancellation are standardized, and a small WinterTC-aligned Web API subset is conformant.

**Key work:**
- `M27-001` Define capability ABI and lifecycle state machine
- `M27-002` Implement compile-time capability dependency resolver
- `M27-003` Introduce custom QuickJS context profiles
- `M27-004` Implement console and timer core capabilities
- `M27-005` Implement URL and URLSearchParams
- `M27-006` Implement text encoding APIs
- `M27-007` Implement AbortController and AbortSignal
- `M27-008` Implement crypto random subset
- `M27-009` Publish capability SDK and inspection surface

**Exit gate:** `M27-GATE` — Prove modularity, lifecycle safety, standards behavior, and cold-start budget.

## M28 — M2.8 — Native Outbound Fetch

Velqu provides bounded, cancellable, pooled Web fetch with TLS/DNS correctness, streaming backpressure, and explicit SSRF policy.

**Key work:**
- `M28-001` Accept outbound fetch and SSRF security ADR
- `M28-002` Select native client stack from evidence
- `M28-003` Implement connection pooling, DNS, and TLS
- `M28-004` Implement Request/Response/Headers subset
- `M28-005` Propagate AbortSignal and route deadlines
- `M28-006` Implement streaming and strict backpressure
- `M28-007` Implement redirect and compression policy
- `M28-008` Implement SSRF and network egress controls
- `M28-009` Complete fetch conformance, observability, and shutdown

**Exit gate:** `M28-GATE` — Prove useful, secure, cancellable backend I/O without sacrificing the cold-start thesis.

## M3 — M3 — Multi-Worker Service Runtime

Independent QuickJS workers scale across cores with bounded queues, quarantine/replacement, and distinct serverless/service/throughput profiles.

**Key work:**
- `M3-001` Freeze independent-worker state semantics
- `M3-002` Implement bounded worker dispatcher
- `M3-003` Implement runtime profiles
- `M3-004` Implement deterministic worker initialization and artifact sharing
- `M3-005` Implement quarantine, replacement, and readiness aggregation
- `M3-006` Implement adaptive scale-up and scale-down
- `M3-007` Implement multi-worker cancellation and graceful shutdown
- `M3-008` Add fairness and overload controls
- `M3-009` Run scaling, memory, and soak evidence

**Exit gate:** `M3-GATE` — Demonstrate bounded scalable service mode while preserving single-worker serverless behavior.

## M4 — M4 — Developer Experience and Private Alpha

The actual Rust/QuickJS runtime is pleasant to develop against, Treaty modes are complete, Linux release artifacts exist, and a realistic proof service is private-alpha ready.

**Key work:**
- `M4-001` Implement actual-runtime velqu dev loop
- `M4-002` Complete CLI command surface
- `M4-003` Implement project scaffolding
- `M4-004` Complete Treaty unit-local, runtime-local, and remote modes
- `M4-005` Publish compact contract and SDK artifacts
- `M4-006` Finalize diagnostics, source maps, and inspect output
- `M4-007` Implement bounded defer and lifecycle hooks
- `M4-008` Build documentation and examples
- `M4-009` Build realistic private-alpha proof service

**Exit gate:** `M4-GATE` — Make Velqu usable by a small invited group without claiming public production readiness.

## M5 — M5 — Production Operations and Real-World Proof

Operational controls, observability, optional Postgres integration, executable cross-framework workloads, and a controlled production canary establish technical production candidacy.

**Key work:**
- `M5-001` Make the real-world benchmark harness executable
- `M5-002` Implement matched competitor candidates
- `M5-003` Implement controlled upstream and CPU/JIT crossover suites
- `M5-004` Authorize and implement optional Postgres capability
- `M5-005` Implement auth/JWT reference capability or policy package
- `M5-006` Implement production configuration and secret handling
- `M5-007` Implement production observability
- `M5-008` Implement trusted-proxy, drain, and deployment semantics
- `M5-009` Run real-world load, leak, and canary evidence

**Exit gate:** `M5-GATE` — Declare suitability for controlled production only after operations and real workloads pass.

## M6 — M6 — Security, Reliability, Platform, and Supply-Chain Hardening

The runtime passes fuzzing, sanitizers, chaos, soak, supported-platform, vulnerability, reproducibility, and artifact-integrity gates.

**Key work:**
- `M6-001` Update complete threat model and trust boundaries
- `M6-002` Run sustained fuzz and property campaigns
- `M6-003` Run sanitizers, Miri, concurrency, and unsafe audits
- `M6-004` Establish dependency and license supply-chain policy
- `M6-005` Generate SBOM and provenance
- `M6-006` Prove reproducible builds on independent builders
- `M6-007` Complete supported-platform matrix
- `M6-008` Run chaos and fault-injection program
- `M6-009` Run long soak and performance-regression qualification

**Exit gate:** `M6-GATE` — Reach independently reviewable technical production readiness.

## M7 — M7 — API/ABI Stabilization and Release Candidate

Public APIs, QPack/runtime/capability ABIs, SemVer behavior, migration rules, publishing automation, and RC canaries are stable.

**Key work:**
- `M7-001` Freeze public TypeScript API and Treaty semantics
- `M7-002` Freeze runtime, QPack, and capability ABI policies
- `M7-003` Implement release and package publishing automation
- `M7-004` Complete versioned documentation and migration guides
- `M7-005` Resolve owner-controlled public release decisions
- `M7-006` Run RC compatibility and canary program
- `M7-007` Freeze public benchmark and positioning statements

**Exit gate:** `M7-GATE` — Produce a signed, installable, documented, supportable RC with stable APIs and no open release blockers.

## M8 — M8 — Production-Ready GA Gate

All technical and owner gates are closed, signed reproducible artifacts are released, operations and rollback are documented, and the production-readiness review is approved.

**Key work:**
- `M8-001` Conduct formal production-readiness review
- `M8-002` Finalize operational SLOs, alerts, and runbooks
- `M8-003` Finalize release signing, rollback, and disaster recovery
- `M8-004` Publish GA artifacts and versioned documentation
- `M8-005` Execute post-release monitoring and response plan

**Exit gate:** `M8-GATE` — Use “production ready” only after technical, owner, release, and operational gates are all satisfied.

