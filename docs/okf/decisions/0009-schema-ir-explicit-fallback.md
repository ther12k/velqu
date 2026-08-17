---
type: Architecture Decision
title: 'ADR-0009: Native Schema IR and Explicit Fallback'
description: Uses a bounded schema IR and makes unsupported or JavaScript validation
  paths visible.
tags:
- adr
- schema
- validation
- fallback
status: draft
generated:
  by: openai/gpt-5.6-pro
  at: '2026-08-17T19:27:58+07:00'
---

# ADR-0009: Native Schema IR with Explicit JavaScript Fallback

## Decision state

Proposed baseline.

## Context

Schemas should drive validation, types, Treaty, and OpenAPI. Arbitrary JavaScript transforms and refinements cannot be compiled safely or faithfully into a small native validator.

## Decision

Define a narrow canonical schema IR. Fully representable schemas may use native/generated validation and serialization. Unsupported features either fail or select a visible JavaScript fallback.

No silent downgrade is allowed.

## Consequences

- consistent cross-artifact semantics;
- smaller native core;
- third-party schema compatibility is partial by design;
- build reports become important for understanding route cost.

## Rejected alternatives

- create a complete Zod clone;
- claim Standard Schema compatibility without semantic fidelity;
- run all validation in JavaScript forever;
- force native validation even when conversion makes it slower.

## Validation

Shared fixtures compare types, native/fallback results, OpenAPI, error paths, and semantic diff for every supported feature.
