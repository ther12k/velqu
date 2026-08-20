---
type: Backlog
title: Post-Beta Deferred Work
status: draft
tags:
- post-beta
- backlog
- ga

---

# Post-Beta Deferred Work

This plan deliberately stops at public beta. The following are not required for beta unless promoted by an accepted ADR:

- Formal GA API/QPack/capability ABI freeze.
- Broad multi-platform support and long qualification matrix.
- Signed reproducible provenance as a hard gate.
- Independent penetration test and extended sanitizer/Miri program.
- Multi-day production canaries and SLA/SLO support commitments.
- WebSocket/SSE.
- Full Web Streams compatibility beyond fetch beta needs.
- Additional databases, queues, cron, object storage, pub/sub.
- Bun/JSC generated production target.
- Wasm hostile-code isolate target.
- General Node compatibility.
- ORM or full cloud platform.

Post-beta work must not be smuggled into the beta critical path to make the framework look broader.
