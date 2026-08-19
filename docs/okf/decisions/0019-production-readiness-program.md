---
type: Architecture Decision Record
title: ADR-0019 — Production Readiness Program and GA Gates
status: draft
tags:
- production-readiness
- roadmap
- release
- security
- operations
---

# ADR-0019 — Production Readiness Program and GA Gates

## Status

Proposed for owner acceptance.

## Context

ADR-0018 authorizes M2.2.1 through M4. The reviewed baseline `velqu-m0-m2-20260819T093558Z.zip` has reached a transitional M2.3-r1 state but is not a production framework. Private alpha also does not close platform, security, reliability, supply-chain, operations, API stability, or public release decisions.

## Decision

Adopt the ordered critical path defined in the production handoff:

```text
M2.3-r2 → M2.4 → M2.5 → M2.6 → M2.7 → M2.8
→ M3 → M4 private alpha → M5 technical production candidate
→ M6 hardening → M7 release candidate → M8 production-ready GA
```

Technical work may proceed continuously when milestone gates pass. Owner decisions remain explicit release gates. Core remains minimal and does not absorb Node compatibility, ORM, WebSocket/SSE, or cloud provisioning by default.

## Consequences

- M4 is no longer mistaken for production readiness.
- Every milestone has a finite frozen review gate.
- Real-world benchmark infrastructure may proceed in parallel; runtime database support waits for the capability ABI.
- Production-ready wording is reserved for M8 approval.
