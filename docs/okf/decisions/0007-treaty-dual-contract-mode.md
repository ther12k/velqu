---
type: Architecture Decision
title: 'ADR-0007: Dual Treaty Contract Modes'
description: Adopts source inference and published compact contracts, plus distinct
  unit-local and runtime-local testing.
tags:
- adr
- treaty
- client
- contracts
- typescript
status: draft
generated:
  by: openai/gpt-5.6-pro
  at: '2026-08-17T19:27:58+07:00'
sources:
- id: eden-treaty
  resource: https://elysiajs.com/eden/treaty/overview
  title: Eden Treaty overview
---

# ADR-0007: Dual Treaty Contract Modes

## Decision state

Proposed baseline.

## Context

Eden-style source inference offers excellent monorepo ergonomics, but importing a full server type graph can scale poorly and couples independent frontend repositories to server source layout.

## Decision

Provide:

1. **source mode** for immediate monorepo inference;
2. **published compact contract mode** for separate repositories and large APIs.

Both modes expose equivalent route, input, and status-aware response semantics.

## Local testing

Provide a fast JavaScript-local dispatcher for unit tests and a separate actual-runtime mode over loopback. Local dispatch is never labeled native runtime conformance.

## Consequences

- strong early DX without sacrificing publication/versioning;
- additional parity tests and generation tooling;
- contract package becomes a first-class release artifact.

## Rejected alternatives

- generated client only;
- source inference only;
- client importing handler/service implementation;
- throwing all HTTP failures as exceptions.

## Validation

Source/published type snapshots, runtime behavior parity, independent package fixture, status narrowing, and bundle/type-check budgets must pass.
