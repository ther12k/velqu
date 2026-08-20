---
type: Milestone Plan
title: M2.4 — Zero-Copy Ingress and Worker-Local Request Bridge
status: draft
tags:
- milestone
- m24
- beta-roadmap

---

# M2.4 — Zero-Copy Ingress and Worker-Local Request Bridge

## Objective

Route before decoding, materialize only contract-declared request fields, and move request ownership into the QuickJS worker without a process-wide request-store mutex.

## Why this milestone exists

After G0, RoutePlan and FieldNeeds can safely drive lazy admission. This is the largest remaining fixed-overhead and bridge-ownership optimization before schema codecs.

## Entry criteria

- Required upstream dependencies in the task ledger are PASS.
- Working tree is clean and source/evidence baseline is identified.
- No unresolved upstream P0/P1 invalidates this milestone.

## Tasks

### M24-001 — Freeze ingress ownership and backpressure design (P0)

**Dependencies:** G0-GATE

**Objective:** Define ownership from Hyper ingress through routing, worker queue, slab lifetime, cancellation, and response completion.

**Implementation:**
- Accept an ADR with ownership diagrams and terminal invariants.
- Specify body ownership, queue admission, disconnect cancellation, and request-slot lifecycle.
- Define no-copy and bounded-copy boundaries.
- Define overload responses and metrics.

**Acceptance:**
- No request data is borrowed beyond its owner lifetime.
- Queue/body limits are explicit.
- Cancellation has one owner.
- Design supports one and multiple workers.

**Required evidence:**
- ADR.
- State-machine tests plan.
- Threat/ownership review.

### M24-002 — Route before request materialization (P0)

**Dependencies:** M24-001

**Objective:** Avoid query/header/body work for routes that do not declare it.

**Implementation:**
- Keep Method, Uri, HeaderMap, and body stream in native forms.
- Match RouteId using method/path before creating request metadata.
- Read FieldNeeds from RoutePlan.
- Bypass request-object creation for policy-free routes that need no request fields.

**Acceptance:**
- C0/C1 perform no query parse, header clone, cookie parse, or body collect.
- 404/405 does not materialize request bodies.
- Malformed oversized inputs fail within coarse ingress budgets.
- Routing behavior remains contract-equivalent.

**Required evidence:**
- Admission counters.
- Negative body/header budget tests.
- Perf stage timings.

### M24-003 — Implement worker-local generation-checked request slab (P0)

**Dependencies:** M24-001, M24-002

**Objective:** Eliminate the global request-store mutex and keep lazy request access worker-owned.

**Implementation:**
- Move request slots into each QuickJS worker.
- Use slot plus generation handles.
- Invalidate at settlement, timeout, cancellation, quarantine, and shutdown.
- Reject stale or cross-worker handles deterministically.

**Acceptance:**
- No process-wide request-store mutex on normal access.
- Stale handles always fail.
- No request slot leaks after terminal paths.
- No JS value or request slot crosses worker ownership.

**Required evidence:**
- Race tests.
- Slot accounting metrics.
- Fuzzed stale-handle operations.

### M24-004 — Capture path parameters as byte ranges (P1)

**Dependencies:** M24-002, M24-003

**Objective:** Avoid allocating parameter strings until validation or JavaScript access requires them.

**Implementation:**
- Store capture start/end ranges against the URI path.
- Bind route-specific parameter names after RouteId selection.
- Validate numeric/UUID formats directly from bytes where possible.
- Materialize JS strings lazily.

**Acceptance:**
- Parameterized routes preserve exact names and values.
- No owned parameter string on an unread path.
- Percent-decoding policy is explicit and tested.
- Invalid encodings fail consistently.

**Required evidence:**
- Allocation test.
- Reference router parity.
- Encoding edge-case corpus.

### M24-005 — Implement declared-header lazy access (P0)

**Dependencies:** M24-003

**Objective:** Expose only headers declared by route or policy without cloning the entire HeaderMap.

**Implementation:**
- Compile header-name IDs into RoutePlan.
- Read header values by ID on demand.
- Define duplicate header behavior and byte/string conversion.
- Keep full Headers escape hatch explicit and costed.

**Acceptance:**
- Route declaring no headers copies none.
- Auth route reads only required headers.
- Duplicate/non-UTF8 behavior matches contract.
- Secret headers are redacted in diagnostics.

**Required evidence:**
- Header access tests.
- Allocation profile.
- Security redaction tests.

### M24-006 — Implement lazy query and cookie decoding (P1)

**Dependencies:** M24-003, M24-004

**Objective:** Parse query and cookies only when declared and only to the depth needed.

**Implementation:**
- Compile query/cookie field IDs.
- Provide repeated-key policy.
- Define percent decoding and invalid-byte behavior.
- Cache decoded fields per request slot.

**Acceptance:**
- No query parse on routes without query.
- Repeated and missing values follow schema semantics.
- Cookie parsing is bounded.
- Access remains valid through owner-scoped microtasks.

**Required evidence:**
- Query/cookie conformance.
- Fuzz parser tests.
- Microtask lifetime tests.

### M24-007 — Implement bounded read-once body admission (P0)

**Dependencies:** M24-001, M24-003

**Objective:** Collect or stream request bodies only when declared and under route/global limits.

**Implementation:**
- Drive body behavior from RoutePlan, not HTTP method.
- Use Bytes and avoid Bytes-to-Vec copies.
- Enforce content length and streaming limits.
- Cache one decoded representation and reject incompatible second reads.

**Acceptance:**
- POST with no body contract does not collect body.
- DELETE/body routes work when declared.
- Oversize/slow bodies cancel cleanly.
- Client disconnect releases body work.

**Required evidence:**
- Body-limit tests.
- Slowloris/partial-body tests.
- Cancellation metrics.

### M24-008 — Replace per-request JS closures with native-backed prototypes (P1)

**Dependencies:** M24-003, M24-005, M24-006, M24-007

**Objective:** Keep context shapes stable and avoid constructing getter closures for every request.

**Implementation:**
- Create shared Context/Request prototypes or native classes.
- Store only opaque handle and route plan references per request.
- Cache native capability objects.
- Keep full Web Request construction as explicit fallback.

**Acceptance:**
- Stable hidden-class/object shape.
- No per-field closure allocation on normal routes.
- Fallback semantics documented.
- Stale handle checks remain enforced.

**Required evidence:**
- Heap/allocation profile.
- Bridge conformance.
- Fallback tests.

### M24-009 — Add ingress and bridge observability (P1)

**Dependencies:** M24-002, M24-003

**Objective:** Measure the actual fixed overhead without per-request logging cost.

**Implementation:**
- Add counters/histograms for route, queue, decode, bridge, JS, encode, and write stages.
- Use disabled-by-default or sampled recording.
- Expose slab/queue/body gauges.
- Measure instrumentation overhead.

**Acceptance:**
- Logging off path adds no formatting or timing work beyond approved counters.
- Metrics are bounded and reset/testable.
- Stage timings identify regressions.
- No sensitive request data is emitted.

**Required evidence:**
- Instrumentation overhead benchmark.
- Metrics schema.
- Redaction tests.

### M24-010 — Complete ingress bridge fuzzing and conformance (P0)

**Dependencies:** M24-004, M24-005, M24-006, M24-007, M24-008

**Objective:** Prove lazy materialization is semantically safe under malformed and adversarial inputs.

**Implementation:**
- Fuzz paths, queries, headers, cookies, bodies, handles, and cancellation orderings.
- Differentially compare legacy/reference decoding where applicable.
- Run property tests for slot lifecycle.
- Capture and minimize failures.

**Acceptance:**
- No panic, leak, stale-handle access, or unbounded allocation.
- Queue-empty-or-quarantined remains true.
- All failing cases become regression fixtures.
- Fuzz corpus is committed or reproducibly generated.

**Required evidence:**
- Fuzz summaries.
- Regression corpus.
- Sanitizer-compatible test output.

## M24-GATE — Exit gate

- [ ] Request routing precedes decode/materialization.
- [ ] Unread fields are not materialized.
- [ ] Worker-local slab has no global mutex and is generation-safe.
- [ ] Bodies, queues, parsers, and disconnect handling are bounded.
- [ ] Measured fixed overhead improves without semantic regression.

## Required benchmark/evidence set

- C0/C1 fixed-overhead comparison versus G0 baseline.
- C3 parameter allocation/latency.
- Header/query/body allocation matrix.
- Concurrency 1/10/50 with stage timings.

## Explicit exclusions

- No schema-specialized JSON codecs beyond compatibility hooks.
- No QPack binary format change.
- No multi-worker dispatch.

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
