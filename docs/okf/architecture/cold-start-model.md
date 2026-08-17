---
type: Architecture Specification
title: Cold-Start Definition and Measurement Model
description: Complete process-to-first-response metric, route classes, profiles, fairness
  rules, and evidence gates.
tags:
- cold-start
- performance
- measurement
- serverless
- startup
status: draft
generated:
  by: openai/gpt-5.6-pro
  at: '2026-08-17T19:27:58+07:00'
sources:
- id: elysia-2
  resource: https://elysiajs.com/blog/elysia-20
  title: Elysia 2 beta announcement and AOT design
- id: quickjs
  resource: https://bellard.org/quickjs/quickjs.html
  title: QuickJS documentation
---

# Product metric

Cold start is the elapsed time from launching a previously stopped application process to receiving the first valid response for a defined route class.

It is not:

- QuickJS engine initialization alone;
- socket bind alone;
- a static native response that bypasses the product's intended handler path;
- an already-running process receiving its first request;
- a best-of-one timing.

# Time model

```text
T_first_response =
    T_spawn
  + T_config
  + T_pack_verify
  + T_native_init
  + T_engine_create
  + T_application_load
  + T_handler_cache
  + T_bind_ready
  + T_client_connect
  + T_route_dispatch
  + T_decode_validate
  + T_handler
  + T_encode_write
```

Each benchmark records total time and available stage breakdown.

# Standard route classes

| Class | Description |
|---|---|
| C0 | native liveness/static plaintext; runtime health only |
| C1 | QuickJS plaintext handler |
| C2 | QuickJS small JSON handler |
| C3 | path/query validated JSON handler |
| C4 | policy-protected validated handler |
| C5 | lazy native capability/service initialization route |

Primary product comparison uses C3 and C4. C0 is useful but cannot represent full framework cold start.

# Route-count classes

```text
25 routes
1,000 routes
```

Application code/schema complexity is held equivalent across competitors.

# Profiles

## Serverless profile

Target behavior:

- exactly one initial worker;
- no eager external service connection;
- only required capabilities;
- embedded or memory-mapped pack;
- no production compilation;
- process exits cleanly after test invocation.

## Service profile

Target behavior:

- one initial worker;
- explicit service warm-up policy;
- optional later adaptive workers;
- ready means configured readiness dependencies are satisfied.

# Cold-start budget policy

All numbers in [Performance Budgets](../engineering/performance-budgets.md) are engineering gates, not observed claims.

A comparative release gate should require Project Q to show a material improvement rather than a statistically fragile tiny difference. The exact threshold is selected after baseline variance is measured, but candidate policy is:

```text
Project Q p95 C3/C4 cold start
≤ 60% of matched Elysia 2 AOT p95
```

on the reference environment, with no correctness or functionality asymmetry.

If Elysia 2 or Bun changes, version-pinned results remain valid only for the tested versions.

# Avoiding false wins

The benchmark SHALL NOT:

- omit schema/policy work from Project Q or competitor;
- prewarm files/pages for only one candidate;
- use debug build for a competitor;
- compare a native static response to an Elysia JavaScript route as the primary claim;
- exclude process spawn from only one candidate;
- reuse a server process across samples;
- connect databases in only one fixture;
- report only the fastest sample;
- select a smaller response body or fewer routes for Project Q;
- hide failed or timed-out starts.

# Measurement method

The harness should:

1. build release artifacts before the timed loop;
2. create a fresh process per sample;
3. capture monotonic timestamps from parent and child;
4. detect ready through an explicit protocol;
5. send one request immediately;
6. validate exact response semantics;
7. record process exit and failure;
8. run enough samples to characterize variance;
9. report p50, p95, p99 and distribution;
10. publish environment, commands, artifacts, hashes, and raw data.

Container/serverless platform measurements are separate from local process measurements.

# Startup guards

Compiler/runtime diagnostics protect the budget:

```text
ERROR top-level network I/O detected in recognized initializer
ERROR runtime route generation is unsupported
WARN eager service adds 7.4 ms p95 in measured profile
WARN capability runtime:crypto links 420 KiB
WARN application source load is slower than bytecode target
```

A warning based on measurement must link the report and target environment.

# Ready semantics

`ready` means:

- configuration valid;
- application pack verified;
- required native capabilities available;
- initial engine/application loaded;
- handler table verified;
- listener accepting;
- required eager services complete;
- readiness endpoint can respond.

It does not require optional lazy services or future workers.

# Cold versus warm

Reports distinguish:

- cold process-to-first response;
- warm same-process request latency;
- worker expansion first request;
- lazy service first request;
- post-expansion steady state.

This prevents a low startup number from concealing a large deferred latency spike.

# Exit criterion

The cold-start thesis is validated only when C3/C4 matched tests show:

- a material repeatable p95 advantage;
- acceptable p99/failure rate;
- lower or competitive idle RSS;
- no missing contract/Treaty functionality;
- no reliance on platform cache asymmetry;
- reproducible raw data.
