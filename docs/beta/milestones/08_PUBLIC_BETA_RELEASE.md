---
type: Milestone Plan
title: Public Beta Readiness and Release
status: draft
tags:
- milestone
- beta
- beta-roadmap

---

# Public Beta Readiness and Release

## Objective

Publish a credible `0.1.0-beta.1` that external developers can install and use for non-critical backend services with clear limitations, operational basics, real-world evidence, and a supportable release packet.

## Why this milestone exists

Public beta is a product gate, not simply the next code tag. It requires a real database story, authentication reference, observability, installability, security baseline, soak evidence, and honest positioning—without claiming GA production readiness.

## Entry criteria

- Required upstream dependencies in the task ledger are PASS.
- Working tree is clean and source/evidence baseline is identified.
- No unresolved upstream P0/P1 invalidates this milestone.

## Tasks

### BETA-001 — Make the real-world benchmark harness executable (P1)

**Dependencies:** G0-GATE

**Objective:** Turn the current SPEC/schema/workloads scaffold into deterministic infrastructure.

**Implementation:**
- Add Postgres compose, seed/reset, controlled upstream, result schema, load generator, and report generator.
- Pin candidate versions.
- Define fairness checks.
- Keep raw samples.

**Acceptance:**
- One command prepares/runs/reports.
- Dataset resets deterministically.
- Candidate failure is retained.
- Protocol records environment and hashes.

**Required evidence:**
- Harness source.
- Smoke results.
- Fairness audit.

### BETA-002 — Implement matched competitor candidates (P1)

**Dependencies:** BETA-001

**Objective:** Provide Raw Rust, Elysia 2, Hono/Bun, and Fastify/Node implementations of identical contracts.

**Implementation:**
- Match SQL, pool, JWT, timeouts, logging, responses, compression, and deployment limits.
- Pin versions.
- Add contract-response verification.
- Document unavoidable differences.

**Acceptance:**
- Candidates are semantically equivalent.
- No framework receives hidden advantages.
- All outputs pass contract fixtures.
- Version/hash metadata is captured.

**Required evidence:**
- Candidate source.
- Parity tests.
- Fairness report.

### BETA-003 — Run controlled I/O and CPU/JIT crossover suites (P1)

**Dependencies:** BETA-001, M28-GATE, M3-GATE

**Objective:** Show where cold start and native infrastructure beat or lose to JIT execution.

**Implementation:**
- Run 0/1/5/10/25ms I/O, payload matrices, and CPU operation levels.
- Measure first request through steady state.
- Calculate cumulative crossover request counts.
- Report losses honestly.

**Acceptance:**
- Crossover method is reproducible.
- Cold, warm, CPU, and I/O are not conflated.
- p50/p95/p99, CPU, RSS, errors are included.
- Positioning follows evidence.

**Required evidence:**
- Raw crossover data.
- Generated report.
- Public wording draft.

### BETA-004 — Implement optional first-party Postgres capability (P0)

**Dependencies:** M27-GATE, BETA-001

**Objective:** Provide a real database story without enlarging core.

**Implementation:**
- Use capability ABI.
- Lazy pool.
- Parameterized queries/transactions.
- Deadline/cancellation/shutdown.
- Pool limits and observability.
- No ORM.

**Acceptance:**
- App without Postgres pays zero dependency/init cost.
- Queries are parameterized.
- Timeout cancels/releases connection safely.
- Pool exhaustion is bounded.
- W1/W2/W3 workloads pass.

**Required evidence:**
- Capability tests.
- Real-world results.
- Cold/RSS cost report.

### BETA-005 — Implement JWT/auth reference package (P0)

**Dependencies:** M27-GATE, M25-GATE

**Objective:** Provide a secure documented policy example and typed errors.

**Implementation:**
- Support one approved JWT algorithm/profile.
- Key loading/rotation hooks.
- Expiry/audience/issuer checks.
- Typed 401/403 problems.
- No secret logging.

**Acceptance:**
- Invalid tokens fail closed.
- Algorithm confusion is impossible.
- Auth policy error appears in Treaty contract.
- Performance/caching is documented.

**Required evidence:**
- Security tests.
- Reference docs.
- W1/W2/W3 integration.

### BETA-006 — Implement beta observability baseline (P0)

**Dependencies:** M3-GATE, M28-GATE

**Objective:** Expose bounded metrics and structured logs sufficient to operate beta services.

**Implementation:**
- Request/route/status/duration.
- Worker queues/quarantine/replacements.
- Fetch and DB pools.
- Memory/tasks/slots.
- Optional trace integration or trace IDs.
- Redaction.

**Acceptance:**
- Disabled overhead measured.
- Enabled overhead budgeted.
- Cardinality is bounded.
- No secrets/PII by default.
- Dashboards/examples exist.

**Required evidence:**
- Metrics schema.
- Overhead benchmark.
- Redaction audit.

### BETA-007 — Implement configuration and secret handling (P0)

**Dependencies:** M27-GATE

**Objective:** Provide typed configuration without top-level network I/O or accidental secret disclosure.

**Implementation:**
- Environment/file configuration.
- Validation at startup.
- Secret value wrapper/redaction.
- Profile-specific settings.
- No dynamic code execution.

**Acceptance:**
- Invalid config fails before ready.
- Secrets never appear in inspect/log/error.
- Defaults are safe.
- Configuration is documented/versioned.

**Required evidence:**
- Config tests.
- Redaction tests.
- Examples.

### BETA-008 — Implement reverse-proxy, drain, and deployment semantics (P0)

**Dependencies:** M3-GATE, BETA-006

**Objective:** Make the beta deployable behind common cloud/reverse-proxy setups.

**Implementation:**
- Trusted proxy configuration.
- Forwarded header policy.
- Liveness/readiness/startup endpoints.
- Graceful drain and termination.
- Container example.

**Acceptance:**
- Spoofed forwarding headers are ignored unless trusted.
- Readiness drops before drain.
- In-flight requests honor deadline.
- Container shutdown exits deterministically.

**Required evidence:**
- Proxy tests.
- Container smoke test.
- Runbook.

### BETA-009 — Run beta security and reliability baseline (P0)

**Dependencies:** M28-GATE, M3-GATE, BETA-004, BETA-005, BETA-007

**Objective:** Establish a credible public-beta safety floor without claiming full GA hardening.

**Implementation:**
- Run fuzz suites for pack/router/schema/bridge/HTTP.
- Dependency vulnerability and license scan.
- Threat-model review.
- Chaos tests for upstream/DB/worker poison.
- No known critical/high exploitable issue.

**Acceptance:**
- All beta trust boundaries documented.
- Critical/high blockers fixed or release blocked.
- Fuzz/chaos findings triaged.
- Same-process code clearly marked trusted.

**Required evidence:**
- Security report.
- Dependency scan.
- Chaos report.
- Known limitations.

### BETA-010 — Create supported beta platform and packaging matrix (P1)

**Dependencies:** M26-GATE, M4A-002

**Objective:** Ship installable binaries/packages for an explicit narrow platform promise.

**Implementation:**
- Linux x86_64 glibc mandatory working assumption.
- Linux ARM64 glibc when CI is available.
- npm packages under beta tag.
- Runtime binary/QPack tools.
- Clean install tests.

**Acceptance:**
- Published platform list is exact.
- Unsupported platforms fail with guidance.
- Packages contain no accidental source/compiler artifacts.
- Install works in clean environment.

**Required evidence:**
- Platform CI.
- Package inventory.
- Install transcript.

### BETA-011 — Automate beta publishing and versioning (P1)

**Dependencies:** M4A-GATE, BETA-010

**Objective:** Produce repeatable pre-release packages without implying API stability.

**Implementation:**
- Use SemVer prerelease.
- Publish `next`/beta tag.
- Generate changelog and migration notes.
- Create GitHub-style release packet.
- Support yanking/rollback.

**Acceptance:**
- Version is consistent across packages/binary/QPack.
- Re-running release does not mutate existing version.
- Rollback procedure is tested.
- Breaking beta changes require notes.

**Required evidence:**
- Dry-run publish.
- Release workflow logs.
- Rollback rehearsal.

### BETA-012 — Complete beta documentation and limitations (P1)

**Dependencies:** M4A-GATE, BETA-004, BETA-005, BETA-008

**Objective:** Make scope, support, and trade-offs impossible to misunderstand.

**Implementation:**
- Installation.
- Quickstart.
- Architecture.
- Contracts/Treaty.
- Fetch/Postgres/auth.
- Deployment.
- Troubleshooting.
- Performance methodology.
- Limitations/non-goals.

**Acceptance:**
- Every command/sample is tested.
- No universal performance claim.
- No production-ready/SLA wording.
- QuickJS bytecode versus JIT is explained accurately.

**Required evidence:**
- Docs CI.
- Link check.
- Example execution.

### BETA-013 — Run beta soak and leak qualification (P0)

**Dependencies:** BETA-004, BETA-005, BETA-006, BETA-008, BETA-009

**Objective:** Prove no obvious unbounded retention before exposing the runtime publicly.

**Implementation:**
- Run at least two-hour mixed workload and at least one million requests on reference platform.
- Include fetch, DB, auth, timeouts, cancellation, worker replacement, and reload.
- Track RSS, heap, slots, tasks, queues, pools, and errors.
- Analyze retained growth.

**Acceptance:**
- No monotonic unbounded growth.
- All resource gauges return near baseline after quiescence.
- No boundary violations.
- Any bounded cache growth is documented.

**Required evidence:**
- Soak raw data.
- Memory graphs.
- Leak analysis.

### BETA-014 — Publish canonical beta benchmark report (P1)

**Dependencies:** BETA-002, BETA-003, BETA-004, BETA-005, BETA-013

**Objective:** Create an honest comparison for beta users.

**Implementation:**
- Include cold start categories, warm microbenchmarks, real DB/auth/I/O, CPU/JIT crossover, cost-normalized metrics, and limitations.
- Pin all candidates/artifacts.
- Retain raw data.
- Have wording reviewed.

**Acceptance:**
- Every number links to raw evidence.
- Fixture-specific wording.
- Velqu losses are included.
- No cloud cold-start claim from local process data.

**Required evidence:**
- Benchmark report.
- Raw archive.
- Methodology review.

### BETA-015 — Generate beta release evidence, SBOM, and checksums (P0)

**Dependencies:** BETA-009, BETA-010, BETA-011, BETA-013, BETA-014

**Objective:** Create a self-verifying public-beta packet.

**Implementation:**
- Source ZIP.
- Git bundle.
- Linux binaries.
- npm package tarballs.
- QPack tools.
- SBOM.
- Checksums.
- Review/evidence indexes.
- Known limitations.

**Acceptance:**
- Checksums verify from release directory.
- Artifacts map to one source commit.
- SBOM identifies dependencies/licenses.
- No stale historical metadata is current.

**Required evidence:**
- Release packet.
- Verification transcript.
- Artifact inventory.

### BETA-016 — Run external clean-install and tutorial verification (P1)

**Dependencies:** BETA-011, BETA-012, BETA-015

**Objective:** Confirm a user outside the repository can complete the intended beta journey.

**Implementation:**
- Fresh Linux VM/container.
- Install CLI/runtime.
- Scaffold app.
- Run tests/dev/build.
- Deploy proof service.
- Use Treaty client.

**Acceptance:**
- No local unpublished dependency.
- Tutorial succeeds verbatim.
- Failures produce actionable diagnostics.
- Artifacts can be rolled back/uninstalled.

**Required evidence:**
- External transcript.
- Environment manifest.
- Issues and resolutions.

### BETA-017 — Resolve beta owner decisions (P0)

**Dependencies:** None

**Objective:** Close only the decisions necessary to publish a public beta.

**Implementation:**
- Repository/organization.
- License/contribution model.
- Release authority.
- Security contact.
- Supported beta platforms.
- Reverse-proxy-first statement.
- Public benchmark wording.

**Acceptance:**
- Decisions are recorded in ADR/open-decision log.
- No agent invents owner authority.
- Security reporting channel exists.
- Platform/support scope is published.

**Required evidence:**
- Accepted decision records.
- Release authorization.
- Contact/support document.

## BETA-GATE — Exit gate

- [ ] No open beta P0/P1 findings or unapproved waivers.
- [ ] A clean external user can install, scaffold, develop, test, build, and deploy a real Velqu app.
- [ ] Fetch, multi-worker service mode, Treaty, optional Postgres, and auth reference work on the actual runtime.
- [ ] Real-world, cold, warm, and CPU/JIT evidence is reproducible and honestly reported.
- [ ] Security baseline, two-hour/one-million-request soak, observability, config, proxy/drain, and clean packaging pass.
- [ ] Release packet is self-verifying and owner decisions are closed.
- [ ] Release is labeled beta, non-SLA, trusted-code-only, and not production-ready GA.

## Required benchmark/evidence set

- Canonical microbenchmarks with repetitions.
- Real PostgreSQL W1/W2/W3.
- Controlled I/O and fan-out.
- CPU/JIT crossover.
- 1/2/4 worker scaling.
- Two-hour/one-million-request soak.

## Explicit exclusions

- No GA/SLA claim.
- No full Node/Bun compatibility.
- No hostile tenant sandbox.
- No WebSocket/SSE.
- No ORM in core.
- No Windows/macOS support promise unless separately accepted.

## Checkpoint deliverables

```text
clean source ZIP
Git bundle or patch history
SOURCE-COMMIT record
SHA-256 manifest
milestone report
review index
evidence index
captured test/typecheck/clippy output
raw benchmark/fuzz/soak evidence where required
known limitations and P2 backlog
```
