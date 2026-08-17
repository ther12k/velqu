---
type: Architecture Specification
title: Observability and Diagnostics
description: Stage-aware telemetry, bridge visibility, structured logs, metrics, source
  maps, CLI inspection, and redaction.
tags:
- observability
- diagnostics
- metrics
- tracing
- source-maps
status: draft
generated:
  by: openai/gpt-5.6-pro
  at: '2026-08-17T19:27:58+07:00'
sources:
- id: rfc-9457
  resource: https://www.rfc-editor.org/info/rfc9457/
  title: RFC 9457 Problem Details for HTTP APIs
- id: elysia-best-practice
  resource: https://elysiajs.com/essential/best-practice
  title: Elysia best-practice guide
---

# Purpose

Observability must make the native/JavaScript boundary visible. A framework that claims predictable performance but hides queueing, materialization, and fallback stages is not operationally honest.

# Correlation fields

Every invocation has:

```text
application
application version
contract hash
runtime/compiler version
route ID
request ID
trace ID, when provided
worker ID
pipeline ID
deployment profile
```

Sensitive transport values are not correlation identifiers.

# Stage timing

Optional tracing records:

```text
admission
route match
queue
decode
validation
native policy
JavaScript policy
handler
materialization
serialization
response write
defer
```

The bridge can report:

```text
bytes moved Rust → QuickJS
bytes moved QuickJS → Rust
native handle accesses
JavaScript calls
objects/fields materialized
body/response strategy
```

Production sampling controls overhead.

# Structured logs

Example:

```json
{
  "level": "info",
  "event": "request.complete",
  "routeId": "users.get",
  "status": 200,
  "durationUs": 418,
  "queueUs": 12,
  "javascriptUs": 103,
  "materializedBytes": 64,
  "requestId": "..."
}
```

No raw request body, cookie, authorization value, API key, or response body is logged by default.

# Metrics

Initial metrics:

- requests by route/status;
- request duration histogram;
- queue duration/depth;
- active requests;
- rejected admissions;
- validation failures;
- JavaScript exceptions;
- worker restarts;
- QuickJS heap/limit events;
- native operation counts/durations;
- response bytes;
- cold-start stage durations;
- lazy service initialization;
- deferred task result.

Route labels use stable IDs, not unbounded raw paths.

# Diagnostics

Compiler diagnostics format:

```text
QCOMP3102 error: release route path must be static
  src/modules/users/routes.ts:18:9

  path: process.env.API_PREFIX + "/users"
        ^^^^^^^^^^^^^^^^^^^^^^

Use a literal path and deployment-level base URL, or generate a
static source module before `q build`.
```

Runtime diagnostics identify:

- stage;
- route ID;
- source-mapped handler location;
- expected versus actual contract;
- safe cause;
- request ID;
- corrective documentation code.

# Development inspector

CLI examples:

```text
q inspect routes
q inspect route users.get
q inspect policies
q inspect capabilities
q inspect fallbacks
q inspect cold-start
q contract diff
```

Route output includes:

```text
method/path
input/output schemas
inherited policies
capabilities
native and JavaScript stages
body/response strategy
expected JavaScript boundary calls
raw/Web fallbacks
source location
```

# Source maps

Source maps must cover:

```text
TypeScript source
→ bundled JavaScript
→ QuickJS execution location
```

Generated wrapper frames should be hidden or annotated without losing causal information. Native capability errors include a JavaScript call-site frame where available.

# Error response policy

Declared failures use typed Problem Details. Unexpected failures return a stable redacted problem:

```json
{
  "type": "urn:q:problem:internal",
  "title": "Internal server error",
  "status": 500,
  "requestId": "..."
}
```

Detailed causes remain in protected logs.

# OpenTelemetry

OpenTelemetry compatibility is a P1 optional capability, not a core hard dependency. The internal trace model should map cleanly to spans/metrics without exposing OpenTelemetry types in every handler.

# Build report

Human and machine-readable reports include:

- route count;
- canonical collisions checked;
- schema strategy by route;
- native/JavaScript stages;
- linked capabilities and size;
- raw/Web fallbacks;
- generated client/OpenAPI/contract hashes;
- unsupported API diagnostics;
- reproducibility status;
- performance budget status when evidence exists.

# Acceptance criteria

- route IDs remain stable across logs/metrics/traces/manifests;
- redaction fixtures cover auth/cookie/body/error values;
- source-mapped JavaScript exception points to TypeScript;
- queue/native/JS/materialization timing can be distinguished;
- metrics labels are bounded;
- inspector output matches runtime manifest;
- sampling disabled path has measured low overhead;
- build report exposes every fallback rather than silently optimizing less.
