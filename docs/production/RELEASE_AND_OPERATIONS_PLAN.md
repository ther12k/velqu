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
