---
type: Architecture Decision
title: 'ADR-0008: One QuickJS Runtime per Worker'
description: Defines engine ownership and defers multi-worker/adaptive parallelism
  until the one-worker path is proven.
tags:
- adr
- workers
- concurrency
- ownership
status: draft
generated:
  by: openai/gpt-5.6-pro
  at: '2026-08-17T19:27:58+07:00'
sources:
- id: quickjs
  resource: https://bellard.org/quickjs/quickjs.html
  title: QuickJS documentation
- id: rquickjs
  resource: https://github.com/DelSkayn/rquickjs
  title: rquickjs Rust bindings
---

# ADR-0008: One QuickJS Runtime per Worker

## Decision state

Proposed baseline.

## Context

QuickJS values and contexts have thread/runtime ownership constraints. Sharing JavaScript objects across threads would complicate safety and correctness.

## Decision

Each JavaScript worker owns one QuickJS runtime/context/application instance and its handler references. JavaScript values never cross worker ownership.

M1 starts with exactly one worker. Multi-worker and adaptive growth are later milestones.

## Consequences

- simple ownership and race model;
- asynchronous I/O can overlap, but JavaScript execution is serial per worker;
- multi-core parallelism duplicates application heap;
- module-level state is per worker and non-durable.

## Rejected alternatives

- one shared QuickJS runtime used concurrently;
- per-request runtime creation for normal service mode;
- immediate worker-per-core startup;
- transparent JavaScript object sharing.

## Validation

Wrong-worker/generation access fails safely; promise completion returns to the owner; single-worker fairness and cancellation pass before parallelism work begins.
