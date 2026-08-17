---
type: Architecture Decision
title: 'ADR-0003: No General Node/Bun Compatibility'
description: Rejects broad Node, Bun, Express, and Elysia runtime compatibility in
  favor of explicit capabilities.
tags:
- adr
- compatibility
- nodejs
- bun
- capabilities
status: draft
generated:
  by: openai/gpt-5.6-pro
  at: '2026-08-17T19:27:58+07:00'
sources:
- id: aws-llrt
  resource: https://github.com/awslabs/llrt
  title: AWS LLRT
---

# ADR-0003: No General Node.js or Bun Runtime Compatibility

## Decision state

Proposed baseline.

## Context

LLRT demonstrates the practicality and continuing cost of implementing selected Node/Web APIs over QuickJS. Attempting broad compatibility would make Project Q a runtime-compatibility project rather than a focused framework.

## Decision

Project Q supports:

- standard ECMAScript available in the selected engine;
- a documented minimal Web-like subset;
- explicit `runtime:*` capabilities;
- Project Q framework contracts.

It does not promise:

- full Node.js built-ins;
- CommonJS;
- Bun runtime globals;
- Express/Elysia application compatibility;
- arbitrary native addons;
- unrestricted npm package compatibility.

Unsupported imports fail during build.

## Consequences

Positive:

- smaller runtime and capability surface;
- faster product progress;
- explicit security/operational authority;
- clearer cold-start budgets.

Negative:

- many packages will not run unchanged;
- adoption requires framework-native choices;
- compatibility documentation and tooling are essential.

## Escape hatches

Packages that use pure ECMAScript and supported APIs may work. Additional APIs can be added as versioned capabilities after conformance, size, security, and maintenance review.

## Rejected alternatives

- replicate all Node APIs;
- silently polyfill unsupported imports;
- claim compatibility based on a few popular packages.

## Validation

The compiler must produce actionable unsupported-API diagnostics and a machine-readable compatibility report.
