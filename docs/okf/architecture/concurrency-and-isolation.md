---
type: Architecture Specification
title: Concurrency, Workers, and Isolation
description: One-worker baseline, asynchronous scheduling, future parallel workers,
  backpressure, state, and isolation levels.
tags:
- concurrency
- workers
- isolation
- backpressure
- quickjs
status: draft
generated:
  by: openai/gpt-5.6-pro
  at: '2026-08-17T19:27:58+07:00'
sources:
- id: quickjs
  resource: https://bellard.org/quickjs/quickjs.html
  title: QuickJS documentation
- id: rquickjs
  resource: https://github.com/DelSkayn/rquickjs
  title: rquickjs Rust bindings
---

# Purpose

Concurrency must respect QuickJS runtime ownership and distinguish asynchronous I/O concurrency from parallel JavaScript execution.

# M1 baseline

```text
one process
one HTTP listener
one QuickJS runtime/context worker
one bounded invocation queue
many native I/O futures
one JavaScript execution turn at a time
```

This baseline is intentionally simple enough to measure and make race-safe.

# Asynchronous concurrency

While request A awaits native I/O, the worker may execute other ready work according to scheduler policy. JavaScript objects remain owned by the worker.

Fairness controls include:

- maximum JavaScript jobs per turn;
- maximum invocation work before yielding;
- per-request deadline;
- maximum pending operations;
- bounded ready queue;
- cancellation checks.

# Parallelism — later milestone

Service profile may create:

```text
worker 0 → QuickJS runtime/context 0
worker 1 → QuickJS runtime/context 1
...
```

Each worker loads the application and caches its own handler references. JavaScript objects are not shared between workers.

# Worker scheduling options to spike

1. round-robin;
2. least queued;
3. power-of-two choices;
4. connection affinity, only if justified;
5. route/service affinity, only if justified.

The simplest approach that produces acceptable p95/p99 and fairness wins.

# Adaptive workers

Proposed profile:

```text
startup: 1 worker
growth: add worker after sustained queue/latency threshold
maximum: configured or CPU count
shrink: optional, conservative
```

Risks:

- first expansion request pays worker/application load;
- duplicate application/service memory;
- warm state differs across workers;
- expensive service pools may multiply;
- bursts may arrive faster than workers initialize.

Therefore adaptive workers are not part of the initial cold-start claim until expansion latency is separately measured.

# Shared native services

A Rust-native service may be shared safely across workers if its own concurrency contract permits it. JavaScript service objects cannot be directly shared.

Possible models:

- native shared pool exposed by handles;
- per-worker JavaScript wrapper around one native service;
- per-worker independent service;
- external service.

The service package declares its model.

# Backpressure

Queue limits exist at:

```text
process
worker
invocation native operations
outbound response buffering
defer queue
```

Overload behavior is deterministic. The runtime must not accept unlimited work because JavaScript is waiting on I/O.

# CPU-heavy handlers

QuickJS is not expected to beat a JIT runtime for long CPU-bound JavaScript. Project Q should document:

- maximum synchronous execution budget;
- interruption semantics;
- recommendation to move heavy work to native capability, worker process, queue, or external service;
- how interrupted handlers and resources are cleaned up.

# Isolation levels

## Trusted service mode

Application code is trusted and runs in the same process as the Rust host. Resource limits protect availability.

## Process-isolated mode — future

Each application or tenant runs in a separate process with:

- OS user/namespace controls;
- filesystem/network policy;
- memory/CPU limits;
- supervised restart;
- versioned IPC.

## Stronger sandbox — future research

Containers, seccomp, microVMs, or equivalent. This is a separate product concern and not inferred from QuickJS embedding.

# State semantics

Module-level JavaScript state is:

- persistent for a worker lifetime;
- not automatically replicated;
- not consistent across multiple workers;
- unsuitable as durable shared application state.

Documentation and diagnostics should make this explicit.

# Worker failure

When one engine worker fails:

- its in-flight invocations fail safely;
- native operations are cancelled;
- handles are invalidated;
- supervisor policy decides replacement;
- repeated failures may fail readiness or terminate process;
- state loss is visible and expected for non-durable worker state.

No transparent replay occurs for non-idempotent requests.

# Concurrency tests

- overlapping timer/fetch promises;
- cancellation before/after native completion;
- fairness under one slow request;
- queue saturation;
- deadline interruption of CPU loop;
- shutdown with queued and active work;
- handle access from wrong generation/worker;
- future multi-worker state non-sharing;
- worker failure and bounded restart.

# Acceptance gate for multi-worker release

Do not enable multiple workers by default until:

- route correctness and source mapping match one-worker mode;
- p95/p99 improves under concurrent load;
- idle and loaded RSS per worker is reported;
- expansion/warm-up latency is reported;
- no JavaScript value crosses worker ownership;
- service duplication behavior is explicit;
- shutdown and failure recovery pass.
