---
type: Milestone Plan
title: M4A — Developer Preview and Private Alpha
status: draft
tags:
- milestone
- m4a
- beta-roadmap

---

# M4A — Developer Preview and Private Alpha

## Objective

Make the actual Rust/QuickJS runtime pleasant to develop against and validate the framework with invited developers before public beta.

## Why this milestone exists

Runtime performance is not a product. The developer loop, Treaty, diagnostics, packaging, examples, and proof app must work against the real production runtime rather than a Bun-only approximation.

## Entry criteria

- Required upstream dependencies in the task ledger are PASS.
- Working tree is clean and source/evidence baseline is identified.
- No unresolved upstream P0/P1 invalidates this milestone.

## Tasks

### M4A-001 — Implement actual-runtime `velqu dev` loop (P0)

**Dependencies:** M3-GATE

**Objective:** Compile and reload the real QuickJS/QPack runtime with fast feedback and parity.

**Implementation:**
- Watch source and contracts.
- Build incremental temporary QPack.
- Load new worker before switching traffic.
- Drain old worker and surface compile/runtime errors.

**Acceptance:**
- No Bun-only behavior mismatch by default.
- Failed reload keeps prior healthy app.
- Source maps point to TypeScript.
- Reload is bounded and observable.

**Required evidence:**
- Reload conformance.
- Failure recovery tests.
- Developer latency measurements.

### M4A-002 — Complete CLI command surface (P1)

**Dependencies:** M4A-001, M26-GATE

**Objective:** Provide consistent dev/build/inspect/contract/test/package workflows.

**Implementation:**
- Implement and document `velqu dev`, `build`, `inspect`, `contract diff`, `test`, `pack inspect/migrate`, and diagnostics.
- Stable exit codes.
- Machine-readable output option.
- Helpful actionable errors.

**Acceptance:**
- Commands work in clean checkout.
- No compiler in production artifact.
- CI use is documented.
- Invalid inputs fail clearly.

**Required evidence:**
- CLI integration tests.
- Golden output.
- Clean-install demo.

### M4A-003 — Implement project scaffolding (P1)

**Dependencies:** M4A-002

**Objective:** Create a minimal correct project without hidden demo credentials or broad dependencies.

**Implementation:**
- Starter API.
- Treaty client example.
- Testing setup.
- Optional fetch/profile choices.

**Acceptance:**
- Generated project builds/tests/runs.
- Starter follows module/service/contract best practices.
- No database/auth forced into core.
- Dependencies are minimal.

**Required evidence:**
- Scaffold snapshot tests.
- Fresh install test.
- Bundle-size report.

### M4A-004 — Complete Treaty unit-local, runtime-local, and remote modes (P0)

**Dependencies:** M25-GATE, M4A-001

**Objective:** Deliver Eden-quality type-safe clients and distinct test fidelity levels.

**Implementation:**
- Unit-local direct generated dispatcher.
- Runtime-local actual Rust/QuickJS process.
- Remote fetch client.
- Exact method/body/query/status/problem typing.

**Acceptance:**
- No public `any`.
- 2xx data and non-2xx errors narrow correctly.
- Undeclared status is a contract error.
- All modes share the same contract.

**Required evidence:**
- Negative type tests.
- Mode parity tests.
- Typecheck scale benchmark.

### M4A-005 — Publish compact contract and SDK artifacts (P1)

**Dependencies:** M4A-004

**Objective:** Support separate frontend repositories without importing server implementation.

**Implementation:**
- Generate d.ts/client/OpenAPI/contract lock.
- Tree-shakable client.
- Version and public contract hash.
- Package verification.

**Acceptance:**
- Client package contains no server runtime.
- Types remain responsive at large route counts.
- Version mismatch is diagnosable.
- Published artifact is deterministic.

**Required evidence:**
- Package content test.
- Type-scale report.
- Reproducibility check.

### M4A-006 — Finalize diagnostics, source maps, and inspect output (P0)

**Dependencies:** M4A-001, M4A-002

**Objective:** Make compile, startup, contract, capability, and runtime failures actionable.

**Implementation:**
- Structured diagnostic codes.
- Source-map-aware stacks.
- Redaction policy.
- Inspect route plan, fields, codecs, capabilities, crossings, and debug names.

**Acceptance:**
- No secrets in production diagnostics.
- Errors identify route/source/contract cause.
- Source maps are lazy on success path.
- Diagnostic catalog exists.

**Required evidence:**
- Golden diagnostics.
- Redaction tests.
- Source-map tests.

### M4A-007 — Implement bounded `defer` and lifecycle hooks (P0)

**Dependencies:** M27-GATE, M3-GATE

**Objective:** Provide after-response cleanup/best-effort work without pretending it is durable jobs.

**Implementation:**
- Define deferred owner, queue, deadline, cancellation, shutdown.
- Separate cleanup from best-effort work.
- Expose metrics.
- Forbid unbounded recursive spawning.

**Acceptance:**
- Response is not delayed beyond defined handoff.
- Deferred work is bounded.
- Shutdown handles or aborts it deterministically.
- Docs warn against durable-job use.

**Required evidence:**
- Lifecycle tests.
- Load/cleanup tests.
- Operational docs.

### M4A-008 — Build documentation and examples (P1)

**Dependencies:** M4A-002, M4A-004, M4A-006

**Objective:** Provide an honest, runnable learning path.

**Implementation:**
- Quickstart.
- Routes/schemas/policies/services.
- Treaty.
- Fetch/capabilities.
- Runtime profiles.
- Deployment behind reverse proxy.
- Limits and non-goals.

**Acceptance:**
- Every code sample is tested.
- Docs distinguish measured facts from targets.
- No production-ready claim.
- Known limitations are prominent.

**Required evidence:**
- Docs test output.
- Link check.
- Example CI.

### M4A-009 — Build realistic private-alpha proof service (P0)

**Dependencies:** M4A-004, M4A-007, M28-GATE

**Objective:** Validate 30–50 routes, auth, fetch, validation, errors, pagination, and deployment.

**Implementation:**
- Feature modules.
- JWT-like policy reference.
- Controlled upstream.
- Metrics/readiness/shutdown.
- Treaty client.

**Acceptance:**
- Runs entirely on actual runtime.
- No hidden Bun production path.
- All error/status contracts declared.
- Load and failure scenarios pass.

**Required evidence:**
- Proof app source.
- Scenario tests.
- Benchmark report.

### M4A-010 — Run invited developer alpha and close P0/P1 feedback (P1)

**Dependencies:** M4A-003, M4A-008, M4A-009

**Objective:** Find product friction before public beta.

**Implementation:**
- Provide clean install packet.
- Collect task-based feedback.
- Classify P0/P1/P2.
- Fix beta-blocking findings and publish limitations.

**Acceptance:**
- Invited users can install, scaffold, run, test, and build without author intervention.
- No open alpha P0/P1.
- P2 backlog is explicit.
- Docs reflect observed confusion.

**Required evidence:**
- Feedback summary.
- Issue disposition.
- Re-run install evidence.

## M4A-GATE — Exit gate

- [ ] Actual-runtime developer loop works.
- [ ] CLI, scaffolding, Treaty modes, diagnostics, and docs are usable.
- [ ] Proof service demonstrates real framework composition.
- [ ] Invited alpha users complete core tasks.
- [ ] No public beta claim yet.

## Required benchmark/evidence set

- Dev reload latency.
- Typecheck/editor scale.
- Proof-service controlled I/O.
- Install/build artifact sizes.

## Explicit exclusions

- No SLA.
- No public production endorsement.
- Breaking API changes still allowed.

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
