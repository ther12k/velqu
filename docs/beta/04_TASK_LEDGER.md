---
type: Task Ledger
title: Velqu Beta Task Ledger
status: draft
tags:
- tasks
- dependencies
- beta

---

# Task Ledger

Status values: `TODO`, `IN_PROGRESS`, `PASS`, `FAIL`, `BLOCKED`, `WAIVED`. A waiver requires owner/reviewer approval, risk, compensating control, and expiry.

| ID | Priority | Status | Dependencies | Milestone | Task |
|---|---|---|---|---|---|
| G0-001 | P0 | PASS | — | G0 | Freeze and verify the M2.3-r3 baseline |
| G0-002 | P0 | PASS | G0-001 | G0 | Make the semantic function manifest mandatory |
| G0-003 | P0 | PASS | G0-001 | G0 | Bind router and schema manifests into the execution graph hash |
| G0-004 | P0 | PASS | G0-003 | G0 | Load the serialized router directly |
| G0-005 | P0 | PASS | G0-002, G0-004 | G0 | Complete operational RouteId, PolicyId, and SchemaId usage |
| G0-006 | P1 | PASS | G0-003, G0-005 | G0 | Separate and verify public contract identity |
| G0-007 | P1 | PASS | G0-002, G0-005 | G0 | Remove duplicate legacy state from current packs |
| G0-008 | P1 | PASS | G0-004, G0-005, G0-007 | G0 | Close canonical performance evidence |
| G0-009 | P1 | PASS | G0-001, G0-008 | G0 | Create self-verifying milestone and evidence indexes |
| G0-GATE | P0 | PASS | G0-001, G0-002, G0-003, G0-004, G0-005, G0-006, G0-007, G0-008, G0-009 | G0 | M23R2 Gate Closure — Trusted Numeric Artifact and Router exit gate |
| M24-001 | P0 | TODO | G0-GATE | M24 | Freeze ingress ownership and backpressure design |
| M24-002 | P0 | TODO | M24-001 | M24 | Route before request materialization |
| M24-003 | P0 | TODO | M24-001, M24-002 | M24 | Implement worker-local generation-checked request slab |
| M24-004 | P1 | TODO | M24-002, M24-003 | M24 | Capture path parameters as byte ranges |
| M24-005 | P0 | TODO | M24-003 | M24 | Implement declared-header lazy access |
| M24-006 | P1 | TODO | M24-003, M24-004 | M24 | Implement lazy query and cookie decoding |
| M24-007 | P0 | TODO | M24-001, M24-003 | M24 | Implement bounded read-once body admission |
| M24-008 | P1 | TODO | M24-003, M24-005, M24-006, M24-007 | M24 | Replace per-request JS closures with native-backed prototypes |
| M24-009 | P1 | TODO | M24-002, M24-003 | M24 | Add ingress and bridge observability |
| M24-010 | P0 | TODO | M24-004, M24-005, M24-006, M24-007, M24-008 | M24 | Complete ingress bridge fuzzing and conformance |
| M24-GATE | P0 | TODO | M24-001, M24-002, M24-003, M24-004, M24-005, M24-006, M24-007, M24-008, M24-009, M24-010 | M24 | M2.4 — Zero-Copy Ingress and Worker-Local Request Bridge exit gate |
| M25-001 | P0 | PASS | M24-GATE | M25 | Define canonical Schema IR v2 |
| M25-002 | P1 | PASS | M25-001 | M25 | Build reproducible decoder/encoder strategy benchmark |
| M25-003 | P0 | PASS | M25-001, M24-GATE | M25 | Generate params/query/header decoders |
| M25-004 | P0 | PASS | M25-001, M24-007 | M25 | Generate JSON body decoders |
| M25-005 | P0 | PASS | M25-001, M25-002 | M25 | Generate status-specific response encoders |
| M25-006 | P0 | PASS | M25-001, M25-005 | M25 | Generate RFC 9457 problem encoders |
| M25-007 | P1 | PASS | M25-003, M25-004, M25-005 | M25 | Implement explicit generic and Web fallback paths |
| M25-008 | P0 | PASS | M25-001, M25-003, M25-004, M25-005, M25-006 | M25 | Unify Treaty, OpenAPI, lock, and runtime schema projection |
| M25-009 | P0 | PASS | M25-003, M25-004, M25-005, M25-006 | M25 | Add codec fuzzing and differential tests |
| M25-010 | P1 | PASS | M25-002, M25-009 | M25 | Close codec performance and cold-start evidence |
| M25-GATE | P0 | PASS | M25-001, M25-002, M25-003, M25-004, M25-005, M25-006, M25-007, M25-008, M25-009, M25-010 | M25 | M2.5 — Schema-Specialized Input and JSON Output Pipeline exit gate |
| M26-001 | P0 | PASS | M25-GATE | M26 | Accept QPack v2 format and compatibility ADR |
| M26-002 | P0 | PASS | M26-001 | M26 | Define strict runtime and bytecode fingerprint |
| M26-003 | P0 | PASS | M26-001, G0-GATE, M25-GATE | M26 | Encode compiled router, RoutePlans, schemas, policies, and functions as sections |
| M26-004 | P0 | PASS | M26-002, M26-003 | M26 | Embed raw QuickJS bytecode without base64 |
| M26-005 | P0 | PASS | M26-003 | M26 | Implement zero-copy or bounded-copy pack reader |
| M26-006 | P1 | PASS | M26-003, M26-004 | M26 | Implement execution integrity and authenticity hooks |
| M26-007 | P1 | PASS | M26-003, M26-004 | M26 | Guarantee reproducible release packs |
| M26-008 | P1 | PASS | M26-001, M26-005 | M26 | Provide explicit v1 compatibility and migration tool |
| M26-009 | P1 | PASS | M26-004, M26-005 | M26 | Build shared-runtime and standalone deployment artifacts |
| M26-010 | P1 | PASS | M26-004, M26-005, M26-009 | M26 | Close route-count cold-start evidence |
| M26-GATE | P0 | PASS | M26-001, M26-002, M26-003, M26-004, M26-005, M26-006, M26-007, M26-008, M26-009, M26-010 | M26 | M2.6 — Binary QPack v2 and Reproducible Artifact ABI exit gate |
| M27-001 | P0 | PASS | M26-GATE | M27 | Define capability ABI and lifecycle state machine |
| M27-002 | P0 | PASS | M27-001 | M27 | Implement compile-time capability dependency resolver |
| M27-003 | P1 | PASS | M27-002 | M27 | Introduce custom QuickJS context profiles |
| M27-004 | P0 | PASS | M27-001, M27-002 | M27 | Implement console and timer core capabilities |
| M27-005 | P1 | PASS | M27-001, M27-003 | M27 | Implement URL and URLSearchParams |
| M27-006 | P1 | PASS | M27-001, M27-003 | M27 | Implement TextEncoder and TextDecoder |
| M27-007 | P0 | PASS | M27-001, M27-003 | M27 | Implement AbortController and AbortSignal |
| M27-008 | P0 | PASS | M27-001, M27-003 | M27 | Implement crypto random subset |
| M27-009 | P1 | PASS | M27-001, M27-002 | M27 | Publish capability SDK and inspection surface |
| M27-010 | P1 | PASS | M27-005, M27-006, M27-007, M27-008 | M27 | Establish Web API conformance program |
| M27-011 | P1 | PASS | M27-002, M27-010 | M27 | Close capability cost budgets |
| M27-GATE | P0 | PASS | M27-001, M27-002, M27-003, M27-004, M27-005, M27-006, M27-007, M27-008, M27-009, M27-010, M27-011 | M27 | M2.7 — Capability Linker and Minimal Web Runtime exit gate |
| M28-001 | P0 | PASS | M27-GATE | M28 | Accept fetch, TLS, redirect, and SSRF security ADR |
| M28-002 | P1 | PASS | M28-001 | M28 | Select native HTTP client stack from evidence |
| M28-003 | P0 | PASS | M28-002 | M28 | Implement connection pooling, DNS, and TLS |
| M28-004 | P0 | PASS | M28-003, M27-005, M27-006 | M28 | Implement Request, Response, and Headers subset |
| M28-005 | P0 | PASS | M28-003, M27-007 | M28 | Propagate AbortSignal and route deadlines |
| M28-006 | P0 | PASS | M28-004, M28-005 | M28 | Implement streaming and strict backpressure |
| M28-007 | P1 | PASS | M28-003, M28-004 | M28 | Implement redirect and compression policy |
| M28-008 | P0 | PASS | M28-001, M28-003, M28-007 | M28 | Implement SSRF and network egress controls |
| M28-009 | P1 | PASS | M28-003, M28-005, M28-006 | M28 | Integrate lifecycle, observability, and shutdown |
| M28-010 | P0 | TODO | M28-004, M28-005, M28-006, M28-007, M28-008 | M28 | Complete fetch conformance and fault testing |
| M28-011 | P1 | TODO | M28-009, M28-010 | M28 | Run controlled upstream and fan-out benchmarks |
| M28-GATE | P0 | TODO | M28-001, M28-002, M28-003, M28-004, M28-005, M28-006, M28-007, M28-008, M28-009, M28-010, M28-011 | M28 | M2.8 — Native Outbound Fetch exit gate |
| M3-001 | P0 | TODO | M28-GATE | M3 | Freeze independent-worker state semantics |
| M3-002 | P0 | TODO | M3-001 | M3 | Implement bounded worker dispatcher |
| M3-003 | P1 | TODO | M3-002 | M3 | Implement serverless, service, and throughput profiles |
| M3-004 | P0 | TODO | M3-002, M26-GATE | M3 | Implement deterministic worker initialization and artifact sharing |
| M3-005 | P0 | TODO | M3-002, M3-004 | M3 | Implement quarantine, replacement, and readiness aggregation |
| M3-006 | P1 | TODO | M3-003, M3-005 | M3 | Implement adaptive scale-up and scale-down |
| M3-007 | P0 | TODO | M3-002, M3-004 | M3 | Implement multi-worker cancellation and graceful shutdown |
| M3-008 | P1 | TODO | M3-002, M3-006 | M3 | Add fairness and overload controls |
| M3-009 | P1 | TODO | M3-003, M3-006, M3-008 | M3 | Close multi-worker scaling and memory evidence |
| M3-010 | P0 | TODO | M3-005, M3-007, M3-009 | M3 | Run multi-worker soak and recovery |
| M3-GATE | P0 | TODO | M3-001, M3-002, M3-003, M3-004, M3-005, M3-006, M3-007, M3-008, M3-009, M3-010 | M3 | M3 — Multi-Worker Service Runtime exit gate |
| M4A-001 | P0 | TODO | M3-GATE | M4A | Implement actual-runtime `velqu dev` loop |
| M4A-002 | P1 | TODO | M4A-001, M26-GATE | M4A | Complete CLI command surface |
| M4A-003 | P1 | TODO | M4A-002 | M4A | Implement project scaffolding |
| M4A-004 | P0 | TODO | M25-GATE, M4A-001 | M4A | Complete Treaty unit-local, runtime-local, and remote modes |
| M4A-005 | P1 | TODO | M4A-004 | M4A | Publish compact contract and SDK artifacts |
| M4A-006 | P0 | TODO | M4A-001, M4A-002 | M4A | Finalize diagnostics, source maps, and inspect output |
| M4A-007 | P0 | TODO | M27-GATE, M3-GATE | M4A | Implement bounded `defer` and lifecycle hooks |
| M4A-008 | P1 | TODO | M4A-002, M4A-004, M4A-006 | M4A | Build documentation and examples |
| M4A-009 | P0 | TODO | M4A-004, M4A-007, M28-GATE | M4A | Build realistic private-alpha proof service |
| M4A-010 | P1 | TODO | M4A-003, M4A-008, M4A-009 | M4A | Run invited developer alpha and close P0/P1 feedback |
| M4A-GATE | P0 | TODO | M4A-001, M4A-002, M4A-003, M4A-004, M4A-005, M4A-006, M4A-007, M4A-008, M4A-009, M4A-010 | M4A | M4A — Developer Preview and Private Alpha exit gate |
| BETA-001 | P1 | TODO | G0-GATE | BETA | Make the real-world benchmark harness executable |
| BETA-002 | P1 | TODO | BETA-001 | BETA | Implement matched competitor candidates |
| BETA-003 | P1 | TODO | BETA-001, M28-GATE, M3-GATE | BETA | Run controlled I/O and CPU/JIT crossover suites |
| BETA-004 | P0 | TODO | M27-GATE, BETA-001 | BETA | Implement optional first-party Postgres capability |
| BETA-005 | P0 | TODO | M27-GATE, M25-GATE | BETA | Implement JWT/auth reference package |
| BETA-006 | P0 | TODO | M3-GATE, M28-GATE | BETA | Implement beta observability baseline |
| BETA-007 | P0 | TODO | M27-GATE | BETA | Implement configuration and secret handling |
| BETA-008 | P0 | TODO | M3-GATE, BETA-006 | BETA | Implement reverse-proxy, drain, and deployment semantics |
| BETA-009 | P0 | TODO | M28-GATE, M3-GATE, BETA-004, BETA-005, BETA-007 | BETA | Run beta security and reliability baseline |
| BETA-010 | P1 | TODO | M26-GATE, M4A-002 | BETA | Create supported beta platform and packaging matrix |
| BETA-011 | P1 | TODO | M4A-GATE, BETA-010 | BETA | Automate beta publishing and versioning |
| BETA-012 | P1 | TODO | M4A-GATE, BETA-004, BETA-005, BETA-008 | BETA | Complete beta documentation and limitations |
| BETA-013 | P0 | TODO | BETA-004, BETA-005, BETA-006, BETA-008, BETA-009 | BETA | Run beta soak and leak qualification |
| BETA-014 | P1 | TODO | BETA-002, BETA-003, BETA-004, BETA-005, BETA-013 | BETA | Publish canonical beta benchmark report |
| BETA-015 | P0 | TODO | BETA-009, BETA-010, BETA-011, BETA-013, BETA-014 | BETA | Generate beta release evidence, SBOM, and checksums |
| BETA-016 | P1 | TODO | BETA-011, BETA-012, BETA-015 | BETA | Run external clean-install and tutorial verification |
| BETA-017 | P0 | PASS | — | BETA | Resolve beta owner decisions |
| BETA-GATE | P0 | TODO | BETA-001, BETA-002, BETA-003, BETA-004, BETA-005, BETA-006, BETA-007, BETA-008, BETA-009, BETA-010, BETA-011, BETA-012, BETA-013, BETA-014, BETA-015, BETA-016, BETA-017 | BETA | Public Beta Readiness and Release exit gate |

## Summary

```text
Milestones: 9
Implementation tasks: 98
Exit gates: 9
Total tracked items: 107
Initial state: all future tasks TODO; baseline achievements are documented separately
```

Each milestone document expands its tasks with implementation, acceptance, tests, and evidence.
