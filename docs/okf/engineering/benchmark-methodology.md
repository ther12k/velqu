---
type: Engineering Standard
title: Reproducible Benchmark Methodology
description: Matched candidates, cold/warm/bridge/type-system protocols, fairness,
  statistics, raw outputs, and interpretation.
tags:
- benchmark
- methodology
- fairness
- statistics
- evidence
status: draft
generated:
  by: openai/gpt-5.6-pro
  at: '2026-08-17T19:27:58+07:00'
sources:
- id: elysia-2
  resource: https://elysiajs.com/blog/elysia-20
  title: Elysia 2 beta announcement and AOT design
- id: elysia-best-practice
  resource: https://elysiajs.com/essential/best-practice
  title: Elysia best-practice guide
---

# Objective

Produce comparisons that can falsify the architecture and can be reproduced by an independent engineer.

# Candidates

Every primary report includes:

```text
raw Rust
raw Bun
Elysia 2 AOT
Project Q
```

Additional runtimes/frameworks are optional. They do not replace the required candidates.

# Version pinning

The run manifest records:

- source repository and commit/tag;
- Rust/Bun/TypeScript/framework/engine versions;
- compiler flags;
- operating system/kernel;
- CPU model/governor;
- memory;
- container/VM status;
- dependency lock hashes;
- application artifact hashes.

Use current official releases at execution time unless a specific beta is the product comparison target. Never silently update previous results.

# Matched application

Primary route classes:

```text
C0 native/static liveness
C1 JS text
C2 JS small JSON
C3 validated params/query/body
C4 policy-protected validated route
C5 lazy capability/service first use
```

C3/C4 schemas, payloads, status behavior, and auth fixture are feature-equivalent.

Exact expected response bytes or semantic canonical values are validated for every sample.

# Cold-start protocol

Parent harness:

1. select an unused port or inherited listener protocol;
2. launch one release process;
3. start monotonic timer before spawn;
4. wait for explicit ready signal or poll under a bounded specified protocol;
5. issue the route-class request immediately;
6. validate status, headers required by fixture, and body;
7. capture first response timestamp;
8. terminate gracefully or through defined cleanup;
9. record child exit, stderr, stage timings, RSS, and failure;
10. use a fresh process for the next sample.

Separate:

- process-to-ready;
- ready-to-first-response;
- process-to-first-response.

# Cache treatment

Run and report at least one controlled condition:

- normal repeated process starts on a stable host.

Optional cold filesystem/page-cache experiments are separate and OS-specific. Do not clear caches for only one candidate.

Run order should be randomized or interleaved to reduce thermal/time drift.

# Samples and statistics

Initial guidance:

- warm-up harness itself;
- at least 200 successful samples per cold candidate/route class when affordable;
- retain all failures/timeouts;
- report p50, p90, p95, p99, mean, standard deviation or bootstrap interval;
- show distribution/box/histogram;
- do not remove outliers unless the rule was defined before run and both raw/filtered data are retained.

# Warm-load protocol

Use a documented load generator with:

- fixed connections/concurrency;
- warm-up period;
- measured interval;
- correctness validation sampling;
- latency distribution;
- throughput;
- CPU;
- RSS;
- allocation/heap metrics where available;
- saturation point and errors.

Test at low concurrency for overhead and increasing concurrency for service behavior.

# Bridge microbenchmarks

Run inside a controlled host process to reduce network noise, then confirm important paths through HTTP.

Operations:

```text
cached no-arg call
native handle scalar access
5 scalar accesses
small JSON input
nested JSON input
invalid/schema-invalid input
primitive result
small/nested object result
array result
promise completion
cancellation race
```

Track:

- duration;
- host calls;
- bytes copied;
- Rust allocations;
- QuickJS heap change;
- strategy;
- correctness.

# Route-count benchmark

Generate 25 and 1,000 equivalent routes with:

- same handler complexity;
- same schema class;
- same policy class where tested;
- deterministic source.

Measure build, application load, handler cache, process-to-first response, and artifact size.

# TypeScript benchmark

Use isolated projects and pinned TypeScript:

- source contract mode;
- published compact mode;
- 100/500/1,000 routes;
- positive and negative client calls;
- cold `tsc --noEmit`;
- incremental check;
- declaration size;
- process memory.

# Memory protocol

Report:

- RSS after ready and stabilization;
- engine heap;
- loaded code/pack size where available;
- first request delta;
- 10k/100k request retained state;
- cancellation/timeout retained state;
- per-worker increment later.

A garbage collection strategy, if invoked, is documented and equivalent where comparison permits.

# Fairness checklist

- release builds for all;
- idiomatic framework implementation;
- same payload and validation semantics;
- same status/body;
- same logging level;
- same compression/TLS setting;
- same eager/lazy external dependencies;
- no hidden pre-running server;
- no static bypass as primary comparison;
- official Elysia best practices applied;
- benchmark-specific tuning disclosed for every candidate.

# Output files

```text
benchmark-manifest.json
raw/cold-start/*.jsonl
raw/warm/*.jsonl
raw/bridge/*.jsonl
summaries/cold-start.json
summaries/warm.json
summaries/bridge.json
benchmark-methodology.md
benchmark-results.md
fairness-audit.md
```

# Interpretation

A result supports only the exact tested workload/version/environment.

Use language such as:

```text
On environment X, artifact Y showed p95 Z for fixture C3,
compared with baseline B at ...
```

Do not generalize to “all APIs” or “always faster.”

# Negative results

Negative and inconclusive results remain in reports. A failed architecture gate triggers an ADR/review rather than changing fixtures after results are known.
