---
type: Workstream
title: Beta Security and Reliability Program
status: draft
tags:
- security
- reliability
- fuzzing

---

# Beta Security and Reliability Program

## Trust boundaries

- QPack and runtime ABI.
- Compiler-generated router/function/schema graph.
- HTTP request parsing and lazy bridge.
- QuickJS application code, which is trusted same-process code.
- Native capabilities and asynchronous operations.
- Outbound network and SSRF policy.
- Postgres credentials and query inputs.
- Logs, diagnostics, source maps, and release artifacts.

## Required beta activities

- Pack/router/schema/bridge/HTTP fuzzing.
- Property tests for numeric graph, router, slots, cancellation, and codecs.
- Dependency vulnerability and license scan.
- Manual threat-model update.
- Chaos: upstream timeout, DNS/TLS errors, DB pool exhaustion, worker poison/replacement, disconnect, shutdown.
- Two-hour/one-million-request soak.
- Redaction audit.
- No unresolved exploitable critical/high issue.

## Fail-closed invariants

- Invalid pack/policy/schema/router/capability/TLS configuration does not reach ready.
- Queue is empty or worker is quarantined at message boundary.
- Every terminal path settles slot, operations, Promise table, and metrics exactly once.
- Network egress applies policy on original URL, resolved address, and redirects.
- Readiness becomes false when usable capacity is unavailable.

## Deferred to GA hardening

Long multi-platform soak, independent penetration testing, full sanitizer matrix, formal supply-chain provenance/signing, and stable public ABI review.
