---
type: Milestone Plan
title: M2.8 — Native Outbound Fetch
status: draft
tags:
- milestone
- m28
- beta-roadmap

---

# M2.8 — Native Outbound Fetch

## Objective

Provide pooled, cancellable, bounded Web fetch with correct DNS/TLS, streaming backpressure, and explicit network egress security.

## Why this milestone exists

A backend beta without outbound HTTP is not practically useful. Fetch must be built on the capability lifecycle rather than as a special scheduler exception.

## Entry criteria

- Required upstream dependencies in the task ledger are PASS.
- Working tree is clean and source/evidence baseline is identified.
- No unresolved upstream P0/P1 invalidates this milestone.

## Tasks

### M28-001 — Accept fetch, TLS, redirect, and SSRF security ADR (P0)

**Dependencies:** M27-GATE

**Objective:** Freeze the public subset, trust boundaries, defaults, and non-goals.

**Implementation:**
- Define URL schemes, redirect policy, DNS rebinding controls, proxy behavior, TLS roots, timeout layers, compression, and body limits.
- Specify reverse-proxy and outbound trust.
- Define unsupported Web features.
- Document same-process trusted-code assumption.

**Acceptance:**
- Security defaults fail closed.
- Private/link-local/metadata behavior is explicit.
- Redirect revalidation is required.
- Direct TLS policy is documented.

**Required evidence:**
- ADR.
- Threat model.
- Security test matrix.

### M28-002 — Select native HTTP client stack from evidence (P1)

**Dependencies:** M28-001

**Objective:** Choose a maintainable implementation based on cold start, size, correctness, streaming, and pooling.

**Implementation:**
- Compare reqwest and lower-level Hyper/Rustls approach.
- Measure dependency/binary/startup cost.
- Test DNS/TLS/pool behavior.
- Record maintenance/security considerations.

**Acceptance:**
- Decision is evidence-backed.
- No framework benchmark alone determines choice.
- Selected stack supports cancellation/backpressure.
- Fallback strategy documented.

**Required evidence:**
- Spike report.
- Raw measurements.
- Decision record.

### M28-003 — Implement connection pooling, DNS, and TLS (P0)

**Dependencies:** M28-002

**Objective:** Create a lazy, bounded outbound client shared safely by native services.

**Implementation:**
- Lazy pool initialization.
- Bound idle/active connections and DNS cache.
- Use verified TLS roots and hostname validation.
- Define keepalive and shutdown.

**Acceptance:**
- App with no fetch pays no pool initialization.
- TLS verification cannot be disabled accidentally.
- Pool exhaustion yields bounded error/backpressure.
- Shutdown releases connections.

**Required evidence:**
- Pool tests.
- TLS negative tests.
- Startup cost evidence.

### M28-004 — Implement Request, Response, and Headers subset (P0)

**Dependencies:** M28-003, M27-005, M27-006

**Objective:** Expose a useful Web-compatible API without materializing unnecessary objects.

**Implementation:**
- Implement method, URL, selected headers, body types, status, and response methods.
- Use lazy native-backed objects.
- Define clone/body-used semantics for beta.
- Keep unsupported API diagnostics explicit.

**Acceptance:**
- Common backend fetch code works.
- Header/body limits are enforced.
- No silent Node-specific behavior.
- WPT subset passes.

**Required evidence:**
- API conformance.
- Body-used tests.
- Allocation profile.

### M28-005 — Propagate AbortSignal and route deadlines (P0)

**Dependencies:** M28-003, M27-007

**Objective:** Ensure request cancellation physically stops outbound work and keeps ownership correct.

**Implementation:**
- Combine explicit abort, route deadline, disconnect, shutdown, and quarantine.
- Use one terminal state for each operation.
- Cancel DNS/connect/body streaming.
- Map failures deterministically.

**Acceptance:**
- No outbound task survives terminal invocation without defer ownership.
- Timeout counted once.
- Cancellation latency is bounded.
- Worker remains reusable.

**Required evidence:**
- Race tests.
- Task accounting.
- Timeout/cancel conformance.

### M28-006 — Implement streaming and strict backpressure (P0)

**Dependencies:** M28-004, M28-005

**Objective:** Support large bodies without unbounded buffering.

**Implementation:**
- Bound read/write buffers.
- Propagate downstream backpressure.
- Cancel on consumer stop/disconnect.
- Define maximum body helper sizes.

**Acceptance:**
- Large response does not allocate full body unless requested.
- Slow upstream/downstream remains bounded.
- Cancellation releases buffers/connections.
- Streaming errors are typed.

**Required evidence:**
- Streaming load tests.
- Slow consumer tests.
- Memory profile.

### M28-007 — Implement redirect and compression policy (P1)

**Dependencies:** M28-003, M28-004

**Objective:** Handle redirects and encoded responses safely and predictably.

**Implementation:**
- Limit redirect count.
- Reapply SSRF/DNS policy on every hop.
- Define credential/header stripping.
- Bound decompression ratio and output.

**Acceptance:**
- Redirect loops fail boundedly.
- Sensitive headers never leak cross-origin.
- Zip-bomb style expansion is limited.
- Observed URL/status follows documented semantics.

**Required evidence:**
- Security fixtures.
- Compression limits tests.
- Redirect conformance.

### M28-008 — Implement SSRF and network egress controls (P0)

**Dependencies:** M28-001, M28-003, M28-007

**Objective:** Provide explicit controls for metadata, loopback, private networks, and DNS rebinding.

**Implementation:**
- Resolve and validate addresses before connect.
- Revalidate redirects and connection targets.
- Support allow/deny configuration.
- Define proxy interaction.

**Acceptance:**
- Cloud metadata endpoints blocked by safe default.
- DNS rebinding tests fail closed.
- IPv4/IPv6/private ranges handled.
- Policy decisions are observable without logging secrets.

**Required evidence:**
- SSRF test suite.
- Threat-model update.
- Configuration examples.

### M28-009 — Integrate lifecycle, observability, and shutdown (P1)

**Dependencies:** M28-003, M28-005, M28-006

**Objective:** Make fetch operationally diagnosable without hot-path logging cost.

**Implementation:**
- Expose pool wait, DNS, connect, TLS, TTFB, body, errors, cancellations.
- Sample/aggregate metrics.
- Drain pool on shutdown.
- Quarantine rejects new work.

**Acceptance:**
- Metrics are bounded and redacted.
- Shutdown reaches quiescence.
- No task/connection leak after errors.
- Disabled instrumentation overhead is measured.

**Required evidence:**
- Metrics schema.
- Shutdown tests.
- Overhead report.

### M28-010 — Complete fetch conformance and fault testing (P0)

**Dependencies:** M28-004, M28-005, M28-006, M28-007, M28-008

**Objective:** Prove the beta subset across success and failure modes.

**Implementation:**
- Run selected WPT cases.
- Create deterministic DNS/TLS/redirect/slow/body fixtures.
- Fuzz headers and URLs.
- Test proxy and cancellation.

**Acceptance:**
- Documented subset passes.
- No panic/hang/unbounded work.
- All failures map predictably.
- Skips are explicit.

**Required evidence:**
- Conformance report.
- Fixture inventory.
- Fuzz report.

### M28-011 — Run controlled upstream and fan-out benchmarks (P1)

**Dependencies:** M28-009, M28-010

**Objective:** Measure scheduler and pool behavior under realistic I/O.

**Implementation:**
- Run 1/5/10/25ms upstream latency.
- Run one, two, and four parallel calls.
- Mix timeout/success/malformed responses.
- Test concurrency 1/10/50/200.

**Acceptance:**
- Queue and pool wait are reported.
- Tail latency remains bounded.
- Error rate and cancellation are correct.
- Results compare matched Elysia/Hono/Fastify candidates.

**Required evidence:**
- Raw real-world results.
- Generated report.
- Candidate hashes.

## M28-GATE — Exit gate

- [ ] Fetch is useful, Web-compatible within a documented subset, and lazy when unused.
- [ ] DNS/TLS/redirect/SSRF defaults fail closed.
- [ ] Deadlines and AbortSignal physically cancel work.
- [ ] Streaming is bounded and backpressured.
- [ ] Conformance and realistic I/O evidence pass.

## Required benchmark/evidence set

- Controlled upstream 1/5/10/25ms.
- Fan-out 1/2/4.
- Large streaming bodies.
- Pool saturation and cancellation.

## Explicit exclusions

- No node:http/node:https.
- No arbitrary raw sockets.
- No WebSocket.

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
