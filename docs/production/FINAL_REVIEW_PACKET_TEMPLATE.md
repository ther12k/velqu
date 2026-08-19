# Final Review Packet Template

## 1. Identity

```text
Project: Velqu / VelquJS
Baseline commit:
Final commit:
Release candidate version:
Source archive:
Source archive SHA-256:
Git bundle:
Artifact checksum file:
```

## 2. Scope and status

- Milestones claimed PASS:
- Tasks BLOCKED/FAIL/WAIVED:
- Open P0/P1/P2:
- Owner decisions:
- Explicit non-goals retained:

## 3. Architecture result

- Compiler and no-dry-run proof:
- Numeric route/runtime IR:
- Request bridge:
- Codec strategy:
- QPack v2/ABI:
- Capabilities/fetch:
- Multi-worker:
- Dev/Treaty:

## 4. Verification commands

Include exact commands, exit codes, captured stdout/stderr, tool versions, and environment.

## 5. Evidence index

Attach `REVIEW_INDEX.json` entries for every production gate:

```json
{
  "gate": "resource-safety",
  "status": "PASS",
  "source": ["..."],
  "tests": ["..."],
  "raw_evidence": ["..."],
  "report": "...",
  "waiver": null
}
```

## 6. Performance

- Raw warm/cold/real-world/crossover data paths
- Candidate versions and fairness audit
- p50/p95/p99, CPU, RSS, errors
- Exact public claim wording
- Negative/inconclusive results

## 7. Security and reliability

- Threat model
- Fuzz/sanitizer/unsafe reports
- Dependency/license audit
- Chaos/soak results
- Security review findings
- Incident/rollback readiness

## 8. Platforms and artifacts

- Supported platform matrix
- Reproducibility evidence
- SBOM/provenance/signatures
- Clean install/upgrade/rollback results

## 9. Operations

- SLOs and alerts
- Runbooks
- Readiness/liveness/drain behavior
- Canary and post-release plan

## 10. Final assertion

Use exactly one:

```text
NOT PRODUCTION READY
TECHNICAL PRODUCTION CANDIDATE
RELEASE CANDIDATE
PRODUCTION-READY GA — pending reviewer and owner approval
PRODUCTION-READY GA — approved
```
