---
type: Architecture Decision
title: 'ADR-0011: Typed Status Results and Problem Details'
description: Standardizes declared status/body pairs and typed RFC 9457-compatible
  problem values.
tags:
- adr
- errors
- rfc9457
- responses
- treaty
status: draft
generated:
  by: openai/gpt-5.6-pro
  at: '2026-08-17T19:27:58+07:00'
sources:
- id: rfc-9457
  resource: https://www.rfc-editor.org/info/rfc9457/
  title: RFC 9457 Problem Details for HTTP APIs
- id: eden-treaty
  resource: https://elysiajs.com/eden/treaty/overview
  title: Eden Treaty overview
---

# ADR-0011: Typed Status Results and RFC 9457 Problem Details

## Decision state

Proposed baseline.

## Context

End-to-end client inference needs status-specific response types. Throwing ordinary domain failures obscures declared HTTP behavior and produces weak clients.

## Decision

Routes declare status-to-schema mappings. Expected failures return typed result values. Problem responses follow an RFC 9457-compatible representation.

Unexpected exceptions are redacted internal failures in production.

## Consequences

- Treaty can narrow errors by status;
- OpenAPI and semantic diff are consistent;
- handlers must return declared statuses;
- raw responses are an explicit weaker path.

## Rejected alternatives

- one undifferentiated response envelope;
- exceptions for all non-2xx outcomes;
- status code inferred only at runtime;
- detailed exception messages returned in production.

## Validation

Type tests, runtime undeclared-status tests, RFC-shaped fixtures, error redaction, and Treaty narrowing must pass.
