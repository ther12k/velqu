---
type: Engineering Standard
title: Velqu Production Security and Operations Program
status: draft
tags:
- security
- reliability
- operations
- supply-chain
---

# Security and Reliability Plan

## Trust model

Velqu runs trusted application code in-process. Memory/stack/deadline limits reduce accidents and denial-of-service impact; they do not create a hostile multi-tenant sandbox.

## Required security domains

- HTTP parsing and admission limits;
- route/policy fail-closed behavior;
- QPack/bytecode integrity, ABI, and optional signatures;
- FFI/native handle ownership and stale-handle rejection;
- scheduler/job/native-operation bounds;
- capability lifecycle and dependency trust;
- fetch TLS, DNS, redirects, SSRF, streaming, and proxy configuration;
- database parameterization, pool bounds, cancellation, and secret handling;
- source maps, logs, traces, and problem redaction;
- compiler unsupported-import and deterministic-build controls;
- dependency, license, SBOM, provenance, and release signing.

## Test program

```text
unit and negative conformance
property tests
fuzzing with retained corpora
HTTP/QPack/schema/bridge parser fuzz targets
sanitizers and Miri where applicable
concurrency/model tests for ownership state machines
fault injection and chaos
24h/72h soak
10M-request resource-retention run
independent security review before RC
```

## Reliability invariants

After request or worker quiescence:

```text
live request slots == 0
pending Promise invocations == 0
pending native ops == 0
native tasks alive == 0
settlement entries == 0
job queue empty OR worker quarantined
boundary violations == 0
```

After quarantine:

```text
new JS work rejected
readiness false
pending work failed closed
native tasks aborted
worker replaced or process restart requested
liveness may remain true while process is alive
```

## Incident readiness

Before GA:

- security contact and disclosure policy;
- severity and response targets;
- compromised signing-key procedure;
- vulnerable engine/binding upgrade lane;
- bad-release rollback/yank procedure;
- CVE/advisory workflow;
- customer communication template.


# Release and Operations Plan

## Deployment profiles

### Serverless

- one QuickJS worker;
- minimum linked capabilities;
- fastest readiness;
- no implicit adaptive worker creation;
- lazy external pools.

### Service

- ready after one healthy worker;
- bounded adaptive workers;
- graceful drain and replacement;
- shared native pools with per-worker JS state.

### Throughput

- configured workers initialized before readiness;
- higher cold start/RSS accepted and reported;
- best immediate multicore capacity.

## Operational endpoints and states

```text
/health/live     process/listener alive
/health/ready    sufficient healthy runtime capacity
startup          artifact loaded and required eager capabilities ready
```

Runtime-owned paths are reserved and visible in inspect/manifest diagnostics.

## Release artifacts

```text
@velqu/* packages
velqu CLI
velqu-runtime binaries
QPack/bytecode tools
source archive
git bundle
checksums
SBOM
provenance
signatures
release notes
compatibility matrix
```

## Release process

1. clean tagged source;
2. deterministic build on approved builders;
3. full test/security/platform/performance gates;
4. create and sign artifacts;
5. verify install in clean environments;
6. stage/canary deployment;
7. approve release;
8. publish packages/binaries/docs;
9. monitor and retain rollback capability.

## Rollback

- runtime and package releases remain independently versioned but compatibility-tested;
- previous signed release remains available;
- operator runbook covers package rollback, binary rollback, QPack rebuild/migration, and database-independent recovery;
- rollback is rehearsed before RC and GA.

## SLO candidates

Exact values are selected from production-candidate evidence, but must cover:

- successful request availability;
- readiness recovery time;
- queue saturation/load shedding;
- worker quarantine/replacement;
- fetch/DB pool wait and timeouts;
- cold process-to-ready and container-to-ready;
- memory/task/slot retention;
- release rollback time.
