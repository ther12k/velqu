---
type: Architecture Decision
title: 'ADR-0006: Engine Adapter and QuickJS-NG Initial Target'
description: Uses a narrow internal engine abstraction with QuickJS-NG as the first
  measured implementation.
tags:
- adr
- engine
- quickjs-ng
- rquickjs
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
---

# ADR-0006: Engine Adapter with QuickJS-NG Initial Target

## Decision state

Proposed experimental baseline.

## Context

QuickJS and QuickJS-NG offer compact interpreter designs with different optimization and maintenance characteristics. Binding the public framework directly to one engine would make future comparison or replacement difficult.

## Decision

Implement a private engine adapter. Use QuickJS-NG through `rquickjs` for the first spike, while keeping upstream QuickJS as a benchmark/conformance comparison where practical.

No engine-specific behavior enters the public TypeScript API.

## Consequences

- engine selection remains reversible;
- bytecode/runtime metadata is explicit;
- abstraction overhead and lowest-common-denominator risk must be controlled;
- the initial adapter API stays intentionally small.

## Adapter boundary

The adapter covers worker creation, application load, handler resolution, calls, jobs, promises, interrupts, limits, source locations, and shutdown.

It does not attempt to abstract every engine C API.

## Rejected alternatives

- expose QuickJS C concepts directly to framework users;
- implement a multi-engine framework before one path works;
- choose an engine solely from published microbenchmarks.

## Validation

Compare load/call/async/memory behavior and document the exact engine commit/version. Promote only after bridge and conformance gates pass.
