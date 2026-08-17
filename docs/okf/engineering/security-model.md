---
type: Security Standard
title: Security Model and Secure Defaults
description: Trust boundaries, FFI/DoS/async/bytecode/secret/SSRF/supply-chain threats,
  capabilities, untrusted-code boundary, and release evidence.
tags:
- security
- threat-model
- ffi
- bytecode
- ssrf
status: draft
generated:
  by: openai/gpt-5.6-pro
  at: '2026-08-17T19:27:58+07:00'
---

# Security position

Initial Project Q runs **trusted application code** in the same process as the Rust host. QuickJS memory, stack, and interrupt limits reduce availability risk but do not make hostile tenant code safe.

# Assets

- host process integrity;
- application pack and bytecode;
- secrets/configuration;
- inbound request data;
- outbound credentials;
- service connections;
- logs/traces;
- route and contract metadata;
- compiler/build environment;
- supply-chain dependencies.

# Trust boundaries

```text
untrusted network request
        ↓
Rust HTTP/admission/parser
        ↓
native request handle
        ↓
trusted application QuickJS
        ↓
native capabilities/services
        ↓
external systems

build source/dependencies
        ↓
trusted compiler
        ↓
signed/hashed application pack
        ↓
runtime loader
```

# Threats and controls

## Malformed/oversized request

Controls:

- header/URI/body/count limits;
- admission before expensive allocation;
- mature HTTP library;
- timeouts;
- fuzzing.

## Route confusion

Controls:

- canonical duplicate/shadow detection;
- method/path normalization;
- no order-dependent replacement;
- 404/405 native behavior.

## FFI use-after-free or wrong owner

Controls:

- opaque handles;
- worker/invocation generations;
- invalidation at settle;
- no raw pointer exposure;
- safe Rust wrappers;
- targeted tests/sanitizers.

## CPU/memory denial of service

Controls:

- request deadlines;
- QuickJS interrupt callback;
- heap/stack limits;
- bounded queues/jobs/operations;
- body/response limits;
- load shedding;
- worker recycle later.

## Async race

Controls:

- operation registry bound to invocation generation;
- cancellation token;
- discard late completion;
- deterministic shutdown;
- race matrix tests.

## Pack/bytecode tampering

Controls:

- format/version checks;
- content hash and future signature;
- exact engine ABI;
- no untrusted bytecode;
- read-only deployment artifact;
- fail before ready.

## Secret disclosure

Controls:

- structured redaction;
- authorization/cookie/body not logged;
- safe Problem Details;
- compiler/build report excludes secrets;
- environment/config handling;
- test fixtures with canary secrets.

## SSRF/outbound misuse

Controls:

- `fetch` is an explicit capability;
- timeouts and cancellation;
- redirect limits;
- future destination policy;
- proxy/DNS trust configuration;
- no automatic forwarding of inbound credentials.

## Supply chain

Controls:

- lockfiles;
- pinned engine/runtime;
- dependency audit;
- SBOM/license report;
- reproducible build work;
- compiler plugin inventory.

## Compiler side effects

Controls:

- static analysis, not app dry-run;
- strict build mode;
- explicit plugin trust;
- no network in reproducible mode;
- generated-source provenance.

# Problem response safety

Client validation problems include safe location and code, not raw body values by default.

Unexpected production failure:

```json
{
  "type": "urn:q:problem:internal",
  "title": "Internal server error",
  "status": 500,
  "requestId": "..."
}
```

Stack/cause appears only in protected logs.

# Capability authority

No ambient filesystem, TCP, or process-spawn API exists.

Each capability has:

- manifest identity/version;
- configuration schema;
- permission model;
- limits;
- cancellation;
- logging/redaction;
- shutdown;
- conformance and threat review.

# Service secrets

Services receive secrets through runtime configuration/secret providers, not route manifests or generated clients.

Secret values are wrapped/redacted where practical and never included in debug serialization automatically.

# Authentication/authorization

Core provides policy composition but not a complete auth product. Security inventory can identify routes without policies.

Application auth packages must define:

- credential transport;
- verification;
- 401 versus 403;
- session/tenant context;
- key rotation;
- audit;
- timing-safe comparison where relevant.

# Untrusted code

Not supported in initial same-process mode.

A future untrusted mode requires:

- process/OS isolation;
- filesystem/network policy;
- resource cgroups/limits;
- IPC contract;
- package provenance;
- supervisor/restart;
- data/tenant boundary threat model.

It is not unlocked by changing one configuration flag.

# Secure defaults

- bind localhost in development unless explicit;
- production configuration validation;
- no detailed production stack response;
- no wildcard outbound authority in high-security profile;
- read-only application pack;
- non-root container;
- bounded bodies/queues/timeouts;
- explicit trusted proxy;
- no dynamic code/bytecode input;
- no hidden compatibility polyfills.

# Security release evidence

- threat model updated;
- FFI ownership audit;
- fuzz/sanitizer report;
- dependency/SBOM/license report;
- secret redaction suite;
- pack tamper/mismatch tests;
- limit/cancellation tests;
- known vulnerabilities/waivers;
- exact scope statement.

# Incident diagnostics

Request IDs and structured errors support investigation without logging raw secrets. Runtime fatal worker failures and pack verification failures are high-signal events with version/hash metadata.
