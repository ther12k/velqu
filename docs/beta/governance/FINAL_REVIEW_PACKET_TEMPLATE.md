---
type: Template
title: Velqu Final Beta Review Packet Template
status: draft
tags:
- review
- template
- beta

---

# Final Beta Review Packet Template

## 1. Identity

```text
Version:
Source commit:
Source ZIP:
Source SHA-256:
Git bundle:
Bundle SHA-256:
Runtime/compiler/engine versions:
Supported platforms:
```

## 2. Gate summary

| Gate | Status | Report | Review index section |
|---|---|---|---|
| G0-GATE | | | |
| M24-GATE | | | |
| M25-GATE | | | |
| M26-GATE | | | |
| M27-GATE | | | |
| M28-GATE | | | |
| M3-GATE | | | |
| M4A-GATE | | | |
| BETA-GATE | | | |

## 3. Verification

- Rust tests:
- TypeScript tests/typecheck:
- Clippy/format:
- Fuzz/conformance:
- Platform/install:
- Soak/chaos:
- Source-to-evidence validation:

Attach captured output; do not manually infer counts.

## 4. Performance evidence

- Warm microbenchmarks.
- Cold start by category and route count.
- Multi-worker scaling.
- Real Postgres/auth/I/O workloads.
- CPU/JIT crossover.
- CPU/RSS/error/queue/pool metrics.
- Known losses and limitations.

## 5. Security and operations

- Threat model and trust boundaries.
- Vulnerability/license scan.
- SSRF/TLS/cancellation results.
- Readiness/drain/rollback runbooks.
- Open findings and waivers.

## 6. Artifact inventory

List every shipped artifact and checksum. Confirm the internal checksum manifest passes from the release directory.

## 7. Owner decisions

List accepted decisions and any unresolved blocker.

## 8. Final assertion

Use exactly one:

- `PUBLIC BETA READY — pending reviewer/owner approval`
- `PUBLIC BETA READY — approved`
- `NOT BETA READY — blockers listed below`

Never use `production ready` or `GA` in this packet.
