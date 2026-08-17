---
type: Architecture Decision
title: 'ADR-0012: Evidence Before Performance Claims'
description: Requires matched, reproducible benchmark evidence and preserves negative
  findings.
tags:
- adr
- benchmark
- evidence
- performance
status: draft
generated:
  by: openai/gpt-5.6-pro
  at: '2026-08-17T19:27:58+07:00'
sources:
- id: elysia-2
  resource: https://elysiajs.com/blog/elysia-20
  title: Elysia 2 beta announcement and AOT design
---

# ADR-0012: Evidence Before Performance Claims

## Decision state

Proposed baseline.

## Context

The architecture has plausible advantages but also a costly native/JavaScript boundary. Static native routes, unmatched feature sets, or best-of-one results could easily produce misleading claims.

## Decision

No public “faster than Elysia/Bun” claim is permitted until matched release fixtures, raw results, methodology, versions, and correctness checks satisfy release gates.

Negative results are retained.

## Required baselines

- raw Rust host;
- raw Bun server;
- matched Elysia 2 AOT;
- Project Q.

## Required categories

Cold start, warm latency/throughput, idle/loaded memory, route count, bridge microbenchmarks, artifact size, and TypeScript check cost.

## Consequences

- product messaging follows evidence;
- benchmark harness is part of M0, not a later marketing task;
- optimizations cannot weaken fixtures asymmetrically;
- architecture may be revised or stopped.

## Rejected alternatives

- quote engine startup;
- compare static Rust bypass to validated JavaScript framework route;
- omit failed samples;
- publish only requests/second;
- tune only Project Q.

## Validation

A machine-readable benchmark manifest and independently reproducible command are required before performance status changes from target to measured.
