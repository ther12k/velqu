---
type: Checklist
title: Velqu Public Beta Release Checklist
status: draft
tags:
- checklist
- beta
- release

---

# Public Beta Release Checklist

## Architecture and runtime

- [ ] G0, M24, M25, M26, M27, M28, M3, and M4A gates PASS.
- [ ] Current numeric pack has no legacy map fallback.
- [ ] Queue-empty-or-quarantined and terminal resource invariants pass.
- [ ] Serverless and service profiles are documented.

## Product

- [ ] Clean external install, scaffold, dev, test, build, deploy succeeds.
- [ ] Treaty unit/runtime/remote modes pass.
- [ ] Fetch, Postgres optional capability, and JWT reference work in proof app.
- [ ] Documentation samples execute in CI.

## Security and reliability

- [ ] No known exploitable critical/high issue.
- [ ] Fuzz, chaos, timeout, cancellation, and SSRF/TLS suites pass.
- [ ] Two-hour/one-million-request soak passes.
- [ ] Secrets/redaction audit passes.

## Evidence and performance

- [ ] Repeated randomized benchmark protocol passes.
- [ ] Real-world and CPU/JIT crossover reports include losses.
- [ ] Raw data and reports match.
- [ ] No universal or cloud-cold-start claim from local process data.

## Operations and release

- [ ] Readiness/liveness/drain/proxy semantics pass.
- [ ] Metrics/logging overhead measured.
- [ ] Linux beta artifacts clean-install.
- [ ] Checksums and SBOM included.
- [ ] Source ZIP, Git bundle, review/evidence indexes agree on one commit.
- [ ] Rollback/yank rehearsal passes.

## Owner and wording

- [ ] Repository, license, release authority, security contact, platform promise, and benchmark wording accepted.
- [ ] Release is labeled beta, non-SLA, trusted-code-only, and not production ready.
