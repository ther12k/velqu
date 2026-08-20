---
type: Scope
title: Velqu Beta Scope Matrix
status: draft
tags:
- scope
- beta
- non-goals

---

# Beta Scope Matrix

## Included

| Area | Beta commitment |
|---|---|
| HTTP | HTTP/1.1 application runtime behind a reverse proxy/load balancer |
| Routing | Verified numeric RouteId and serialized terminal router |
| Requests | Params, query, selected headers, cookies, JSON/text/bytes, bounded streaming |
| Responses | JSON, text, bytes, headers, cookies, typed statuses, RFC 9457 problems |
| Contracts | Type inference, Treaty, OpenAPI, lock, semantic diff |
| Runtime | QuickJS-NG bytecode, QPack v2, serverless and multi-worker profiles |
| Web APIs | URL, encoding, abort, crypto random, fetch subset |
| I/O | Outbound fetch and optional Postgres capability |
| Auth | Reference JWT policy/package, not a universal identity platform |
| DX | `velqu dev`, build, inspect, contract diff, test, scaffold, package |
| Testing | Unit-local, runtime-local, and remote Treaty modes |
| Operations | Readiness/liveness, metrics/logs, config/secrets, graceful drain |
| Release | Linux beta artifacts, npm prerelease packages, checksums, SBOM |

## Explicitly deferred

- Full Node.js or Bun compatibility.
- CommonJS and native Node addons.
- Express/Elysia compatibility adapters.
- ORM in core.
- WebSocket and SSE.
- Server-side rendering/frontend framework.
- Automatic cloud provisioning, queues, cron, object storage, or pub/sub platform.
- Hostile same-process tenant code.
- Windows/macOS support promise.
- Direct TLS/HTTP2 as mandatory beta ingress; reverse-proxy-first is the working profile.
- Production SLA, GA stability, or universal benchmark claims.

## Scope-control rule

An optional feature may not enter the beta critical path unless it is required by the beta definition, fixes a P0/P1 gate, or is accepted through a new ADR. Everything else becomes post-beta backlog.
