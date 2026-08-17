---
type: Engineering Standard
title: Performance Budgets and Decision Gates
description: Cold-start, bridge, warm runtime, memory, artifact, TypeScript, and compiler
  targets with governance.
tags:
- performance
- budgets
- cold-start
- memory
- typescript
status: draft
generated:
  by: openai/gpt-5.6-pro
  at: '2026-08-17T19:27:58+07:00'
---

# Status

All values are **engineering targets or decision gates**, not observed benchmark results.

Reference hardware and baseline variance must be recorded before thresholds are promoted.

# Cold-start budgets

Candidate initial targets on the designated reference Linux CI machine:

| Metric | Target |
|---|---:|
| C0 native liveness process-to-first response | ≤ 8 ms p95 |
| C1 QuickJS text process-to-first response | ≤ 12 ms p95 |
| C2 QuickJS JSON process-to-first response | ≤ 15 ms p95 |
| C3 validated route process-to-first response | ≤ 18 ms p95 |
| C4 policy-protected route process-to-first response | ≤ 22 ms p95 |
| 1,000-route C3 startup versus 25-route C3 | ≤ 20% increase |
| cold-start failure/timeout samples | 0 in accepted run |

These absolute numbers are aspirational until the chosen CI host proves they are measurable and stable.

Comparative candidate gate:

```text
Project Q C3 and C4 p95
≤ 60% of matched Elysia 2 AOT p95
```

A threshold can be adjusted only before viewing favorable results or through a documented ADR based on measurement noise and product materiality.

# Runtime startup invariants

| Invariant | Target |
|---|---:|
| runtime route compilation | 0 |
| runtime schema compilation | 0 |
| runtime OpenAPI generation | 0 |
| production TypeScript transpilation | 0 |
| initial QuickJS workers | 1 |
| application service connections for unrelated cold route | 0 |
| compiler bytes in production runtime/application pack | 0 |

# Bridge gates

| Metric | Gate |
|---|---|
| cached empty handler throughput | redesign if <50% raw Rust and no compensating route value |
| primitive return | measured separately from object conversion |
| small-object response | target ≤2× raw Rust route latency before network noise |
| lazy unread request fields | 0 fields/bytes materialized |
| host calls for simple route | target 1 handler call plus documented result conversion |
| expired/wrong-owner native handles accepted | 0 |
| native JSON strategy | adopt only if end-to-end faster or materially lower memory than QuickJS strategy |
| response strategy | adopt only with correctness parity and lower complete cost |

The absolute microsecond budget is set after M1 baseline calibration.

# Warm service targets

These are secondary:

| Metric | Target |
|---|---|
| plain QuickJS route | ≥80% of equivalent raw Bun throughput, aspirational |
| validated small JSON route | competitive with matched Elysia 2 within 20%, aspirational |
| p99 queue latency below saturation | explicit and bounded |
| overload memory growth | bounded; no unbounded queue |
| interruption/cancellation leak | 0 retained operations after settle |

QuickJS is not expected to beat JIT-heavy CPU-loop workloads. Such fixtures are reported, not used to redefine the target workload.

# Memory budgets

Candidate:

| Metric | Target |
|---|---:|
| minimal server idle RSS | ≤ 12 MiB p50 |
| framework/engine incremental RSS over raw Rust | ≤ 8 MiB |
| each additional warm worker | measured and budgeted before default enablement |
| retained heap after 10,000 small requests | stable within documented tolerance |
| cancelled operation registry after settle | returns to baseline |
| request handle slots after completion | 0 live |

Platform allocator and OS variance must be published.

# Artifact budgets

Candidate:

| Artifact | Target |
|---|---:|
| runtime binary, stripped, dynamically linked reference | ≤ 8 MiB |
| minimal application pack | ≤ 256 KiB excluding source maps |
| remote Treaty client minified | ≤ 8 KiB before compression |
| compact 1,000-route declaration | measured; target avoids server implementation import |
| unused capability contribution | 0 linked/pack metadata where feasible |

Binary size depends strongly on platform/linking and is never compared without equivalent rules.

# TypeScript budgets

Reference project with generated synthetic routes:

| Routes | Full type check target |
|---:|---:|
| 100 | ≤ 1.5 s |
| 500 | ≤ 3.0 s |
| 1,000 | ≤ 5.0 s |

Also report:

- editor completion latency where measurable;
- max memory;
- declaration size;
- source versus published mode.

# Compiler budgets

Candidate:

| Metric | Target |
|---|---:|
| 25-route clean build | ≤ 1 s excluding Rust runtime build |
| 1,000-route clean contract compile | ≤ 3 s |
| unchanged incremental route edit | ≤ 250 ms target |
| deterministic manifest/hash mismatch | 0 |
| unsupported syntax diagnostic | source-located in same run |

Compiler performance is P1 after correctness, but architecture must avoid per-route quadratic type or graph behavior.

# Budget governance

A budget result records:

```text
target
observed distribution
environment
artifact hashes
versions
sample count
command
status: pass/fail/unexecuted
```

No target is marked met from one run or a different environment without explanation.

# Optimization hierarchy

1. remove startup work;
2. avoid materialization/copies;
3. reduce boundary calls;
4. specialize common declared paths;
5. remove unused capabilities;
6. optimize engine/application load;
7. tune allocator/runtime;
8. consider unsafe/custom primitives only after profiles.

Correctness, security, and traceability cannot be traded away silently.
