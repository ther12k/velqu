---
type: Architecture Decision Record
title: ADR-0020 — Beta-Readiness Program (0.1.0-beta.1) Superseding GA-First Framing
status: accepted
tags:
- beta-readiness
- roadmap
- release
---

# ADR-0020 — Beta-Readiness Program (0.1.0-beta.1)

## Status

Accepted (2026-08-20, owner-supplied beta-readiness handoff `velqu-beta-readiness-handoff-v1.zip`,
SHA-256 `25f9321d449e2166eb8a78551def01b156c37d7ff831619f034e87d9752db08b`).

## Context

ADR-0019 framed the program as a straight march to production-ready GA (M5–M8). The reviewed
reality is that the runtime core is strong while the developer product, capabilities, and
operations layers are incomplete. An intermediate, externally usable beta is the honest and
useful next finish line.

## Decision

Adopt the beta-readiness program under `docs/beta/` as the authoritative forward roadmap:

```text
G0 M23R2 gate closure → M2.4 → M2.5 → M2.6 → M2.7 → M2.8
→ M3 multi-worker → M4A developer preview → 0.1.0-beta.1 public beta
```

- The ADR-0018 technical sequence is preserved, not replaced.
- The finish line is **Velqu 0.1.0-beta.1**: usable by external developers for evaluation,
  staging, internal tools, non-critical services, and benchmarking — with no SLA, no API/ABI
  stability promise, and no universal-performance claims.
- ADR-0019's GA gates (M5–M8) remain the post-beta track, recorded in `docs/production/`.
- Owner decisions stay outside implementation-agent authority.

## Consequences

- "Production ready" wording stays reserved for the GA track.
- Beta scope excludes full Node/Bun compatibility, CommonJS, WebSocket/SSE, ORM-in-core,
  hostile-tenant sandboxing, and stable long-term ABI.
- Each beta milestone keeps a frozen evidence gate; the entry point for agents is
  `docs/beta/MASTER_AGENT_PROMPT.md`.
