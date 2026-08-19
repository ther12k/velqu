# Baseline and Scope

## Source baseline

```text
velqu-m0-m2-20260819T093558Z.zip
SHA-256 03a06bbdcc7b4f7626dd5b287983c4f3b6d26ff82e4895923284d76af92debb5
```

The starting source has meaningful M2.3-r1 work:

- owner-scoped async Promise response settlement;
- dense function manifest for compiler-generated packs;
- direct `Vec<Persistent<Function>>` lookup when numeric IDs are supplied;
- current generated bundles no longer emit `__velquRegister`;
- 25 ms controlled-I/O workload is present in the real-world specification.

It is **not** accepted as full M2.3 because the reviewed package still needs:

- an explicit numeric load contract independent of the legacy handler table;
- exact RoutePlan/contract equivalence;
- operational `RouteId`, `PolicyId`, and `SchemaId` identities;
- a verified `FieldNeeds` representation;
- a compiler-emitted terminal router rather than runtime candidate reconstruction;
- canonical repeated M2.3 performance and cold-start evidence;
- an executable real-world benchmark harness rather than only specification files.

## Initial GA profile

Unless the project owner changes it by ADR, the plan targets:

```text
trusted TypeScript application code
Linux x86_64 and arm64, subject to owner platform decision
HTTP JSON APIs behind a reverse proxy/load balancer
Rust + QuickJS-NG production runtime
Bun development/package/test tooling
serverless and long-running service profiles
Treaty typed client and OpenAPI/contract artifacts
optional capabilities, including fetch and Postgres, outside core
```

## Core architectural constraints

1. Route in Rust before executing JavaScript.
2. No application dry-run during compilation.
3. No production TypeScript transpilation, route discovery, schema compilation, or plugin discovery.
4. QuickJS runtimes are single-owner; no `JSValue` crosses workers.
5. Request data is lazy, generation-checked, and invalid after settlement.
6. Every queue, body, native operation, job drain, worker, and shutdown path is bounded.
7. Current compiled packs use numeric execution; legacy compatibility is explicit and versioned.
8. One canonical contract drives runtime, Treaty, OpenAPI, and semantic diff.
9. Expected HTTP failures are typed values; unexpected failures are redacted.
10. Performance claims are exact to a pinned workload, environment, and artifact.
11. QuickJS same-process execution is for trusted application code, not hostile tenant sandboxing.
12. Core remains minimal; optional capabilities do not enlarge unrelated applications.

## Scope-change rule

A new feature may enter the critical path only when one of these is true:

- it is required by an existing milestone acceptance gate;
- a security/correctness P0 makes it necessary;
- an owner-approved ADR changes scope.

Interesting features that do not meet those conditions are placed in a post-GA backlog rather than inserted opportunistically.
