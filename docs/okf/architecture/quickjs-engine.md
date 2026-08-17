---
type: Architecture Specification
title: QuickJS Engine Integration
description: Engine abstraction, application loading, promises, limits, modules, bytecode,
  and handler caching.
tags:
- quickjs
- quickjs-ng
- rquickjs
- engine
- bytecode
status: draft
generated:
  by: openai/gpt-5.6-pro
  at: '2026-08-17T19:27:58+07:00'
sources:
- id: quickjs
  resource: https://bellard.org/quickjs/quickjs.html
  title: QuickJS documentation
- id: quickjs-ng
  resource: https://quickjs-ng.github.io/quickjs/diff/
  title: QuickJS-NG differences and goals
- id: rquickjs
  resource: https://github.com/DelSkayn/rquickjs
  title: rquickjs Rust bindings
- id: aws-llrt
  resource: https://github.com/awslabs/llrt
  title: AWS LLRT
---

# Purpose

The engine layer embeds a QuickJS-family interpreter for trusted TypeScript/JavaScript application logic while preventing engine-specific details from becoming the framework API.

# Initial engine choice

QuickJS-NG through `rquickjs` is the proposed initial implementation target because it offers an actively optimized QuickJS-family engine and a Rust integration path. This remains a hypothesis subject to benchmark and conformance evidence.

Upstream QuickJS remains a comparison target where integration cost is reasonable.

# Engine abstraction

Conceptual host interface:

```rust
trait JavaScriptEngine {
    type Worker;
    type Application;
    type FunctionRef;

    fn create_worker(&self, limits: EngineLimits) -> Result<Self::Worker>;
    fn load_application(
        &self,
        worker: &mut Self::Worker,
        artifact: &ApplicationArtifact,
    ) -> Result<Self::Application>;
    fn resolve_handlers(
        &self,
        worker: &mut Self::Worker,
        app: &Self::Application,
        expected: &[HandlerId],
    ) -> Result<Vec<Self::FunctionRef>>;
    fn call(
        &self,
        worker: &mut Self::Worker,
        function: &Self::FunctionRef,
        invocation: InvocationHandle,
    ) -> PendingInvocation;
    fn drain_jobs(&self, worker: &mut Self::Worker) -> Result<JobProgress>;
    fn interrupt(&self, worker: &mut Self::Worker, reason: InterruptReason);
}
```

The public TypeScript API must not depend on QuickJS-specific opcodes, bytecode layouts, C APIs, or non-portable global behavior.

# Application loading

Development:

```text
bundled ESM JavaScript
+ source map
+ manifest
```

Release candidate:

```text
version-pinned bytecode or embedded compiled module
+ source map mapping
+ engine ABI identifier
+ artifact digest
```

The runtime validates exact engine/bytecode compatibility before executing the pack.

# Promise and event-loop integration

The worker event loop combines:

- queued invocation;
- QuickJS pending jobs;
- completed Rust futures;
- timers;
- cancellation/deadline signals;
- deferred callbacks.

A native asynchronous operation creates a JavaScript promise and stores a bounded continuation associated with:

```text
worker ID
invocation generation
capability operation ID
deadline/cancellation token
```

Completion is delivered only to the owning worker. Late completion after cancellation is discarded safely.

# Engine limits

Configurable controls:

- JavaScript heap limit;
- stack limit;
- interrupt/CPU deadline;
- maximum jobs drained per scheduling turn;
- maximum pending promises/native operations;
- maximum module count and application bytes;
- worker/request recycle thresholds.

These controls protect availability but do not constitute hostile-code sandboxing.

# Module loading

Release modules are resolved from the application pack and capability registry. Arbitrary filesystem or network module loading is disabled unless an explicit capability provides it.

Allowed forms:

```ts
import { route } from "@q/core";
import { fetch } from "runtime:fetch";
import { randomUUID } from "runtime:crypto";
```

Unsupported imports fail at build and again at pack verification as defense in depth.

# Globals

The initial global surface should be small:

```text
globalThis
console
setTimeout / clearTimeout
queueMicrotask, if correctly integrated
TextEncoder / TextDecoder
URL / URLSearchParams
AbortController / AbortSignal
fetch, through capability linkage
crypto subset, through capability linkage
```

Project Q framework primitives are build-time imports or frozen runtime modules, not ambient mutable globals.

# Bytecode rules

- bytecode is not accepted from request input;
- application bytecode is produced by a pinned trusted compiler;
- engine version and bytecode ABI are exact-match metadata;
- hash or signature verification happens before load;
- source mode remains available for development and diagnostics;
- fallback to source on a version mismatch is not allowed silently in production.

# Handler cache

The generated application exports a stable handler table. During startup the runtime verifies:

- count;
- IDs;
- callable values;
- contract hash compatibility.

Resolved function references are retained by the owning engine worker for the worker lifetime.

# Exception mapping

Expected business failures are returned as typed results. Thrown exceptions are unexpected unless a documented framework helper converts them.

Development exception output MAY include:

- route ID;
- stage;
- mapped source file/line;
- problem cause chain;
- native operation name;
- request ID.

Production response is redacted. Structured server logs follow the configured redaction policy.

# Engine spike evidence

The spike SHALL record:

- worker creation time;
- source load time;
- bytecode load time;
- handler resolution time;
- empty handler call cost;
- primitive and small-object return cost;
- promise/timer completion cost;
- heap/RSS after load;
- failure/interrupt behavior;
- source-map quality.

QuickJS-NG is not accepted as stable project policy until this evidence and licensing/maintenance review are recorded.
