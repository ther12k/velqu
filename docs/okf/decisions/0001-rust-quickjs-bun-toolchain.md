---
type: Architecture Decision
title: 'ADR-0001: Rust Host, QuickJS Engine, Bun Toolchain'
description: Selects the three-layer implementation and clarifies that Bun is development
  tooling rather than production execution.
tags:
- adr
- rust
- quickjs
- bun
- toolchain
status: draft
generated:
  by: openai/gpt-5.6-pro
  at: '2026-08-17T19:27:58+07:00'
sources:
- id: bun-docs
  resource: https://bun.sh/docs
  title: Bun documentation
- id: aws-llrt
  resource: https://github.com/awslabs/llrt
  title: AWS LLRT
- id: quickjs
  resource: https://bellard.org/quickjs/quickjs.html
  title: QuickJS documentation
---

# ADR-0001: Rust Host, QuickJS Engine, Bun Toolchain

## Decision state

Proposed baseline.

## Context

The product needs TypeScript authoring, Elysia-quality contracts, Treaty-style client inference, and substantially lower complete cold start than a general Bun framework. A single existing runtime does not simultaneously optimize all of those goals.

Treating the system as “a Bun framework” would be technically misleading once JavaScript executes in QuickJS rather than JavaScriptCore.

## Decision

Use:

```text
Bun
  → package management, scripts, tests, TypeScript development, bundling

Rust
  → production process, HTTP, routing, limits, bridge, capabilities, scheduling

QuickJS-family engine
  → trusted application business logic
```

The product is a new runtime/framework with a Bun-first development workflow, not a Bun production framework.

## Consequences

Positive:

- development remains familiar to TypeScript/Bun users;
- production can optimize startup, memory, and native boundaries;
- runtime API can remain narrow;
- compiler and host can cooperate on static artifacts.

Negative:

- npm/Bun runtime compatibility is limited;
- two-language implementation raises complexity;
- integration, source maps, and debugging require deliberate work;
- Bun-specific server libraries cannot simply run.

## Rejected alternatives

- Bun/Elysia only: weaker differentiation for the cold-start/runtime thesis.
- Node/V8 host: broader compatibility but heavier product scope and different startup profile.
- Rust-only application framework: loses TypeScript business-logic goal.
- Zig + QuickJS immediately: less mature integration path and an unnecessary second host language.

## Validation

M0/M1 must prove process-to-first-response, bridge cost, memory, source maps, async integration, and test ergonomics before this decision is promoted.
