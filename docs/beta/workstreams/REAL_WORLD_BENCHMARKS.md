---
type: Workstream
title: Real-World Benchmark Program
status: draft
tags:
- benchmark
- real-world
- evidence

---

# Real-World Benchmark Program

## Purpose

Measure the workload boundary where Velqu's startup, memory, native routing, and predictable scheduling outweigh JIT warm execution—and where they do not.

## Workloads

### W0 — Controlled I/O

```text
auth policy → native delay/upstream → dynamic JSON response
latency: 0, 1, 5, 10, 25 ms
payload: 256 B, 1 KB, 16 KB, 64 KB
concurrency: 1, 10, 50, 200
```

### W1 — Authenticated primary-key read

```text
JWT verification → UUID validation → indexed SELECT → 1 KB JSON
```

### W2 — Transactional order write

```text
JWT → 2 KB body → validation → transaction → stock check → order + items → commit → 201
```

### W3 — Paginated join/aggregation

```text
cursor + limit → join/COUNT/AVG → 4/16/64 KB response
```

### W4 — Outbound fan-out

```text
DB read → 1/2/4 upstream fetches → merge → response
success/timeout/malformed combinations
```

### W5 — CPU/JIT crossover

Measure first request, first 10, first 100, after 1,000 warm-up, and steady state across increasing pure-JS work and JSON transformation.

## Candidates

- Velqu serverless and service profiles.
- Raw Rust.
- Elysia 2 on pinned Bun.
- Hono on the same Bun.
- Fastify on pinned Node.

## Fairness

Same schema, seed, SQL, pool size, JWT algorithm/key, timeouts, logging, compression, response bytes, CPU/memory limits, and connection behavior. Candidate-specific differences are documented.

## Required measurements

- p50/p95/p99 and errors.
- throughput and maximum stable throughput.
- CPU per 1,000 requests and requests/vCPU-second.
- idle and loaded RSS.
- worker queue, DB pool, and upstream wait.
- bridge/decode/encode time.
- connection/native task counts.
- cold start categories kept separate.

## Crossover calculation

```text
Velqu(N) = VelquCold + N × VelquPerRequest
JIT(N)   = JITCold   + N × JITPerRequest
N_break_even = (JITCold - VelquCold) / (VelquPerRequest - JITPerRequest)
```

This is reported only when the denominator and measurements are meaningful.
