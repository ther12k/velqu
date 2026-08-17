---
type: Architecture Decision
title: 'ADR-0014: Version-Pinned Trusted Bytecode'
description: Restricts release bytecode to trusted exact-version artifacts and preserves
  source mode for development.
tags:
- adr
- bytecode
- quickjs
- integrity
- versioning
status: draft
generated:
  by: openai/gpt-5.6-pro
  at: '2026-08-17T19:27:58+07:00'
sources:
- id: quickjs
  resource: https://bellard.org/quickjs/quickjs.html
  title: QuickJS documentation
---

# ADR-0014: Version-Pinned Trusted Engine Bytecode

## Decision state

Proposed release direction; source mode remains the M1 fallback.

## Context

QuickJS bytecode can reduce parsing/load work but is engine-version-specific and unsafe as an untrusted interchange format.

## Decision

Development uses bundled source and source maps. Release MAY use bytecode only when:

- produced by the pinned trusted compiler;
- exact engine/ABI metadata is present;
- integrity is verified;
- runtime and pack versions match;
- no external request can supply bytecode.

No silent source fallback occurs in production on mismatch.

## Consequences

- potentially faster application load;
- runtime/application artifacts must be versioned together;
- reproducibility and source maps require care;
- rolling upgrades preserve compatible pairs.

## Rejected alternatives

- accept uploaded bytecode;
- call bytecode a portable artifact;
- load mismatched bytecode optimistically;
- remove source-mode diagnostics prematurely.

## Validation

Source and bytecode conformance parity, tamper/mismatch rejection, load benchmarks, source-map behavior, and rollback packaging must pass.
