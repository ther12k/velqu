# Production-Readiness Gates

## Gate semantics

Status values:

```text
PASS      complete with source, tests, and required evidence
FAIL      gate is not satisfied
BLOCKED   external prerequisite is unavailable; independent work may continue
WAIVED    owner/reviewer-approved exception with risk, control, and expiry
```

A milestone passes only when all required P0/P1 tasks are `PASS` or an authorized waiver exists. P2 work may remain in the published backlog.

## Severity

- **P0:** security bypass, memory safety, data loss/corruption, unbounded externally triggerable work, deadlock/hang, false readiness, invalid artifact accepted before ready, or unsupported production claim.
- **P1:** deterministic correctness, cancellation, resource retention, contract/API incompatibility, operational reliability, or material evidence gap.
- **P2:** maintainability, optional performance, documentation polish, or future ecosystem work that does not violate the current milestone contract.

## Cross-cutting technical gates

### Architecture

- Rust routes before JavaScript.
- No app dry-run or production discovery.
- Current packs use verified numeric execution and precompiled runtime IR.
- One worker owns each QuickJS runtime; no cross-worker JS object sharing.
- Optional capabilities are linked by declaration and absent when unused.

### Correctness and contracts

- Runtime, Treaty, OpenAPI, contract lock, semantic diff, and published types derive from one canonical schema/route graph.
- Every status and policy error is declared and narrowed correctly.
- Every terminal path settles request slots, operations, settlement entries, and metrics exactly once.
- Message boundaries satisfy queue-empty-or-quarantined.

### Resource safety

- Body/header/query limits, worker queues, job counts, deadlines, native operations, fetch buffers, DB pools, and deferred work are bounded.
- Cancellation physically releases native work.
- Soak tests show no monotonic retained state.

### Security

- Fail closed on artifact, policy, capability, TLS, and readiness errors.
- No known unresolved critical/high exploitable dependency or code issue.
- Secrets are redacted from logs, errors, reports, and crash output.
- Same-process code is documented as trusted only.

### Performance

Engineering thresholds are decision gates, not public guarantees:

- 25-route release process-to-first-valid-response remains within the approved small-app budget.
- 1,000-route QPack v2 startup meets the approved p50/p95 scaling budget.
- C0 remains close to semantically matched raw Rust.
- C1/C2/C3 meet approved single-worker throughput/p95 thresholds or the product is explicitly narrowed to cold-start workloads.
- Two and four workers meet approved scaling factors without hiding queue latency or memory cost.
- Every public number includes raw data, p50/p95/p99, errors, CPU/RSS, versions, environment, and artifact hashes.

### Operations

- Liveness, readiness, startup, drain, overload, quarantine, and rollback semantics are tested.
- Metrics/logs/traces are bounded and have measured disabled/enabled overhead.
- Runbooks and alerts exist before controlled production.

### Supply chain and release

- Reproducible artifacts, SBOM, provenance, checksums, signatures, and dependency/license policy pass.
- Clean install, upgrade, rollback, and supported-platform tests pass.
- Public APIs/ABIs follow SemVer and migration policy.

## Finish-line gates

### M4 private alpha

Usable by invited developers, actual-runtime dev mode and proof service work, but no production claim.

### M5 technical production candidate

May run controlled canaries with explicit support, observability, runbooks, and rollback. Still not public GA.

### M6 hardened technical readiness

Security, fuzzing, sanitizers, chaos, soak, platform, reproducibility, and supply-chain gates pass.

### M7 release candidate

Public APIs/ABIs and publishing are stable; owner decisions are closed; signed RC artifacts and canary upgrades pass.

### M8 production-ready GA

Formal production-readiness review approved, no open P0/P1, signed artifacts released, SLOs/runbooks/support active, rollback rehearsed.
