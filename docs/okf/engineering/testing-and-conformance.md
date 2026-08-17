---
type: Engineering Standard
title: Testing and Conformance Strategy
description: Unit, type, compiler, actual-runtime, Treaty, schema, baseline, fuzz,
  snapshot, and platform test requirements.
tags:
- testing
- conformance
- type-tests
- fuzzing
- runtime
status: draft
generated:
  by: openai/gpt-5.6-pro
  at: '2026-08-17T19:27:58+07:00'
---

# Test layers

```text
unit
type
component
conformance
runtime integration
baseline correctness
performance regression
security/fuzz
packaging
```

No single layer substitutes for the others.

# Unit tests

Rust:

- route canonicalization;
- pack parsing/versioning;
- handle generation;
- body/limit arithmetic;
- response headers;
- scheduler state transitions;
- error redaction.

TypeScript:

- schema normalization;
- route/module composition;
- policy type merging;
- Treaty URL/request construction;
- semantic diff classification.

# Type tests

Use positive compilation and expected-error fixtures:

- required/missing path/query/header/body;
- transformed input versus handler output type;
- undeclared status;
- policy-provided context;
- status narrowing;
- source/published contract parity;
- reserved Treaty path names;
- 100/500/1,000 route scale.

Expected errors are asserted precisely enough that an unrelated error cannot satisfy the fixture.

# Compiler conformance

Fixtures:

- static route extraction;
- imports/re-exports;
- module prefixes;
- canonical duplicate;
- shadow/wildcard;
- policy cycle/conflict;
- unsupported Node/Bun import;
- dynamic path;
- schema fallback;
- deterministic output;
- no application service execution.

# Runtime-local conformance

Always executes the actual Rust binary.

Routing:

- static/parameter/wildcard;
- 404/405/HEAD;
- malformed URI;
- keep-alive.

Bridge:

- lazy access;
- JSON/text/bytes;
- invalid input;
- typed results;
- expired handle;
- wrong generation;
- throw/source map.

Async:

- resolve;
- reject;
- cancel before completion;
- complete before cancel;
- completion after settlement;
- deadline interrupt;
- shutdown.

Limits:

- headers;
- URI;
- body;
- queue;
- pending operations;
- heap/stack/time.

# Treaty modes

## Unit-local

Confirms:

- contract typing;
- policy/handler business behavior;
- result shape.

It may use generated/local JavaScript dispatch.

## Runtime-local

Confirms:

- actual encoding;
- Rust route/validation/bridge;
- QuickJS execution;
- native capabilities;
- cancellation;
- error/source-map behavior.

Reports always label which mode ran.

# Schema conformance corpus

Each schema case includes:

```text
schema declaration
transport source
valid values
invalid values
normalized handler value
error path/code
OpenAPI snapshot
Treaty type expectation
semantic diff behavior
native/fallback expectation
```

Native and fallback validators must agree within documented differences.

# Baseline correctness

Before performance measurement, each candidate passes the same black-box fixture suite. The suite validates status, body, relevant headers, validation/policy errors, and route count.

A candidate that fails correctness has no performance result for that fixture.

# Fuzz/property tests

Priority targets:

- qpack parser;
- route/path canonicalization;
- URI/query/header handling;
- schema IR decoder;
- native request handle operations;
- JSON/response bridge;
- problem serialization.

Crashes, panics, unbounded allocations, invalid memory access, or inconsistent canonicalization become regression corpus entries.

# Sanitizers and tools

Use available Rust/FFI tooling such as:

- Miri where applicable;
- address/thread/undefined behavior sanitizers for native/FFI builds when supported;
- cargo fuzz/libFuzzer or equivalent;
- dependency/security audit.

Tool limitations and unexecuted platforms are reported.

# Snapshot/golden policy

Golden files are suitable for:

- manifests;
- diagnostics codes/locations;
- OpenAPI;
- compact contracts;
- build reports;
- semantic diffs.

They must be deterministic and reviewed. A blanket snapshot update is not an acceptable fix.

# Test matrix

Initial platforms:

```text
Linux x86_64 — required
Linux aarch64 — packaging later
macOS — development best effort
```

QuickJS source and bytecode modes have separate conformance where bytecode is enabled.

# Verification command

`./scripts/verify` should run all tests required by the current milestone and output a machine-readable summary:

```json
{
  "status": "pass|fail|partial",
  "checks": [
    {
      "id": "runtime-conformance",
      "status": "pass",
      "command": "...",
      "artifact": "..."
    }
  ]
}
```

Unavailable checks are `unexecuted`, never `pass`.

# Coverage principle

Coverage percentage is supplementary. Traceability and boundary/race/failure fixtures are primary.

# M2 conformance exit

- all P0 compiler/runtime/schema/Treaty fixtures pass;
- actual binary tests pass;
- source/published type parity passes;
- no known safety mismatch is waived silently;
- benchmark candidate correctness passes;
- pack tamper/version tests pass;
- OKF links/frontmatter validate;
- clean checkout command is documented.
