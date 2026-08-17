---
type: Engineering Standard
title: Coding and Review Standards
description: Safe Rust, FFI ownership, compiler/static TypeScript rules, errors, performance
  discipline, tests, docs, and commits.
tags:
- coding-standard
- rust
- typescript
- ffi
- review
status: draft
generated:
  by: openai/gpt-5.6-pro
  at: '2026-08-17T19:27:58+07:00'
---

# Engineering values

```text
correctness before benchmark
explicit contracts before magic
small public APIs
bounded resources
evidence before optimization
clear ownership
diagnosable failure
```

# Rust standards

- safe Rust by default;
- `unsafe` confined to minimal reviewed engine FFI modules;
- every unsafe block states invariants in comments;
- no unchecked externally controlled allocation;
- no panic for malformed request/application data;
- errors retain context without leaking secrets;
- cancellation and shutdown are explicit;
- ownership types encode worker/invocation generation where practical;
- hot-path clones/copies require profile justification;
- async locks are not held across JavaScript execution unless proven safe;
- blocking work does not run on the async reactor thread.

# FFI standards

Every native function exposed to JavaScript documents:

- arguments and accepted types;
- ownership;
- lifetime;
- thread/worker affinity;
- cancellation;
- exceptions/errors;
- maximum allocation/work;
- behavior after request completion;
- source-map/diagnostic implications.

No raw pointer is stored directly in a user-visible JavaScript object without an opaque safe indirection.

# TypeScript standards

- route metadata is static and explicit;
- business services accept ordinary typed values;
- avoid importing framework context into domain logic;
- expected HTTP failures are returned as typed values;
- no top-level I/O or service connection;
- no dynamic route registration;
- no `any` at public contract boundaries;
- transformations/coercions are explicit;
- source and published contract tests accompany public type changes;
- runtime-specific APIs come from explicit `runtime:*` modules.

# Compiler standards

- AST analysis is semantic enough to avoid fragile source-string parsing;
- unsupported syntax fails with location and remediation;
- no evaluation of arbitrary handlers;
- normalized IR is versioned;
- diagnostics are stable enough for tests but allow message evolution through codes;
- generated artifacts include source provenance;
- deterministic sorting avoids import-order output drift;
- optimizer passes preserve route/schema IDs.

# Error standards

Errors have stable codes:

```text
QCOMPxxxx compiler
QPACKxxxx application pack
QRUNxxxx runtime
QBRIDGExxxx bridge
QSCHEMAxxxx schema
QTREATYxxxx client
QSECxxxx security
```

Expected client-visible problems use typed status contracts. Internal errors remain protected.

# Performance coding rules

- record a baseline before optimization;
- optimize matched release builds;
- include allocation and memory effects;
- do not replace clear code with unsafe complexity for insignificant wins;
- keep a regression benchmark for accepted optimization;
- isolate optional capability cost;
- distinguish cold/warm/first-use/steady-state;
- never tune competitor defaults asymmetrically.

# Test standards

- tests assert observable behavior, not private implementation where avoidable;
- race and failure cases are first-class;
- time-based tests use deterministic clocks or generous bounded protocols;
- golden files are reviewed and reproducible;
- negative type tests explain expected failure;
- baseline fixtures validate exact response values;
- tests are not weakened to make a benchmark pass;
- skipped/unavailable tests are reported explicitly.

# Documentation standards

Normative words:

- SHALL/MUST — required;
- SHOULD — strong default with documented exception;
- MAY — optional.

Targets are labeled targets. Results include environment, date, artifact hash, and command.

Material design changes update an ADR and traceability, not only code comments.

# Commit standards

- one coherent change;
- tests and docs travel with behavior;
- no unrelated generated churn;
- milestone commits pass authorized checks;
- no secrets;
- commit message identifies requirement/epic when useful;
- clean tree at handoff.

# Review checklist

- contract correctness;
- ownership/lifetime;
- bounded resources;
- cancellation/shutdown;
- error/redaction;
- fallback visibility;
- source map/diagnostics;
- tests/fuzz fixtures;
- benchmark impact where relevant;
- documentation/traceability.
