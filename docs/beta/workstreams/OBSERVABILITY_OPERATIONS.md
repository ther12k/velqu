---
type: Workstream
title: Beta Observability and Operations
status: draft
tags:
- observability
- operations
- deployment

---

# Beta Observability and Operations

## Required semantics

### Health

- Liveness: process/listener alive.
- Readiness: application workers usable and artifact loaded.
- Startup: initial ready transition.
- Quarantine: unhealthy worker removed; capacity aggregated.

### Graceful drain

```text
readiness false
→ stop admission
→ bounded in-flight completion
→ cancel/abort after deadline
→ close pools/workers
→ exit with diagnostic summary
```

### Metrics

- Requests, statuses, durations, errors.
- Route and worker queue time.
- QuickJS invocation/microtask/timeout/quarantine/replacement.
- Request slots, native tasks, deferred tasks.
- Fetch pool/DNS/connect/TLS/TTFB/body.
- Postgres pool wait/query/transaction outcomes.
- RSS/heap and capability profile.

### Logging

Off/errors/full or equivalent modes; asynchronous bounded sink; redaction; correlation IDs; no secret/body logging by default.

## Beta operations artifacts

- Reverse-proxy configuration example.
- Container image/example.
- Startup/readiness/drain runbook.
- Worker quarantine and restart runbook.
- Fetch/DB saturation troubleshooting.
- Rollback instructions for beta packages.
