---
type: Milestone Plan
title: M2.5 — Schema-Specialized Input and JSON Output Pipeline
status: draft
tags:
- milestone
- m25
- beta-roadmap

---

# M2.5 — Schema-Specialized Input and JSON Output Pipeline

## Objective

Generate route-specific decoders and encoders from one canonical Schema IR while preserving Treaty, OpenAPI, problem-response, and contract-lock semantics.

## Why this milestone exists

C2 and medium JSON workloads remain dominated by conversion and serialization. The solution must be evidence-selected per route rather than assuming Rust or QuickJS always wins.

## Entry criteria

- Required upstream dependencies in the task ledger are PASS.
- Working tree is clean and source/evidence baseline is identified.
- No unresolved upstream P0/P1 invalidates this milestone.

## Tasks

### M25-001 — Define canonical Schema IR v2 (P0)

**Dependencies:** M24-GATE

**Objective:** Create a versioned normalized schema model suitable for validation, decoding, encoding, OpenAPI, Treaty, and semantic diff.

**Implementation:**
- Specify objects, arrays, unions, literals, enums, formats, defaults, optional/null, transforms, files, and problem schemas.
- Define compatibility and fallback markers.
- Canonicalize ordering and hashing.
- Document unsupported transformations.

**Acceptance:**
- One schema identity produces equivalent runtime and public projections.
- Canonical form is deterministic.
- Unsupported constructs fail or use explicit fallback.
- Schema diff can classify nested changes.

**Required evidence:**
- Schema golden corpus.
- Canonicalization tests.
- Compatibility matrix.

### M25-002 — Build reproducible decoder/encoder strategy benchmark (P1)

**Dependencies:** M25-001

**Objective:** Measure QuickJS and native strategies across realistic payload shapes.

**Implementation:**
- Compare QuickJS parse/stringify, generic Rust conversion, and generated schema-aware codecs.
- Use 256B, 1KB, 16KB, 64KB, nested objects, arrays 100/1,000, optional/null, and problems.
- Capture CPU, allocation, bridge time, and tails.
- Select strategies by evidence.

**Acceptance:**
- Raw and generated results are committed.
- No single strategy is forced globally.
- Compiler decision rules are deterministic.
- Fallback cost is visible in inspect output.

**Required evidence:**
- Benchmark raw data.
- Strategy decision report.
- Artifact hashes.

### M25-003 — Generate params/query/header decoders (P0)

**Dependencies:** M25-001, M24-GATE

**Objective:** Fuse field extraction, coercion, and validation for non-body inputs.

**Implementation:**
- Generate direct decoder programs keyed by SchemaId.
- Validate byte ranges and header/query values without generic object trees.
- Return typed RFC 9457 problems.
- Preserve declared coercion semantics exactly.

**Acceptance:**
- Invalid inputs produce exact declared envelopes.
- No duplicate parse/validation pass.
- Treaty and OpenAPI types agree.
- Decoder programs are bounded and fuzzable.

**Required evidence:**
- Differential tests.
- Malformed corpus.
- Performance profile.

### M25-004 — Generate JSON body decoders (P0)

**Dependencies:** M25-001, M24-007

**Objective:** Parse and validate declared JSON bodies with one route-selected strategy.

**Implementation:**
- Implement generated direct decode where supported.
- Retain QuickJS/generic fallback for unsupported transformations.
- Enforce depth, size, array, string, and numeric limits.
- Propagate cancellation and request deadlines.

**Acceptance:**
- One successful decode representation crosses to JS.
- Oversize/deep inputs fail boundedly.
- No semantic drift from schema.
- Fallback is explicit in build report.

**Required evidence:**
- Fuzz/differential tests.
- Depth/size boundary tests.
- CPU/allocation results.

### M25-005 — Generate status-specific response encoders (P0)

**Dependencies:** M25-001, M25-002

**Objective:** Fuse output validation and serialization for stable response contracts.

**Implementation:**
- Generate per-status encoders.
- Read declared properties in fixed order.
- Handle optional/null/union fields.
- Keep QuickJS stringify or generic fallback when measured better.

**Acceptance:**
- Undeclared status/body remains a contract violation.
- Output is JSON-equivalent to reference serialization.
- One traversal for generated paths.
- No user JS escapes deadline ownership during conversion.

**Required evidence:**
- Golden JSON corpus.
- Response mismatch tests.
- Mapping deadline tests.

### M25-006 — Generate RFC 9457 problem encoders (P0)

**Dependencies:** M25-001, M25-005

**Objective:** Preserve typed domain and framework errors without generic placeholder shapes.

**Implementation:**
- Generate problem type/status/title/detail/custom-field encoders.
- Redact unexpected failures.
- Ensure policy-provided errors flow into Treaty unions.
- Include content type and instance behavior.

**Acceptance:**
- Custom problem fields survive end-to-end.
- Unexpected errors never expose secrets/stacks in production.
- Error status narrowing is exact.
- OpenAPI problem schemas match runtime.

**Required evidence:**
- Problem fixtures.
- Redaction tests.
- Treaty narrowing tests.

### M25-007 — Implement explicit generic and Web fallback paths (P1)

**Dependencies:** M25-003, M25-004, M25-005

**Objective:** Support advanced cases without hiding performance or semantic costs.

**Implementation:**
- Tag fallback reason in RoutePlan.
- Support raw Response/full Request escape hatches.
- Keep fallback bounded and deadline-aware.
- Expose bridge crossings and codec choice in `velqu inspect`.

**Acceptance:**
- Fallback never activates silently.
- Raw Response bypass behavior is documented.
- No contract claim is generated when adapter lacks required projection.
- Fallback routes pass conformance.

**Required evidence:**
- Inspect snapshots.
- Fallback integration tests.
- Performance delta report.

### M25-008 — Unify Treaty, OpenAPI, lock, and runtime schema projection (P0)

**Dependencies:** M25-001, M25-003, M25-004, M25-005, M25-006

**Objective:** Eliminate projection drift across tooling and runtime.

**Implementation:**
- Generate all projections from canonical IR.
- Add parity checks to verification.
- Publish compact contract metadata.
- Update semantic diff to Schema IR v2.

**Acceptance:**
- Same statuses/fields/security in all projections.
- No hand-written duplicate interface is required.
- Breaking changes are classified correctly.
- Published client does not import server implementation.

**Required evidence:**
- Cross-projection golden tests.
- Contract diff fixtures.
- Typecheck scale results.

### M25-009 — Add codec fuzzing and differential tests (P0)

**Dependencies:** M25-003, M25-004, M25-005, M25-006

**Objective:** Prove generated codecs match reference semantics and remain memory-safe.

**Implementation:**
- Fuzz encoded/decoded values.
- Compare generated output with standards/reference JSON behavior.
- Run malformed and boundary values.
- Minimize failures into permanent fixtures.

**Acceptance:**
- No panic, hang, unbounded output, or semantic mismatch.
- All fuzz findings are triaged.
- Coverage targets are recorded.
- Generated code is deterministic.

**Required evidence:**
- Fuzz summaries.
- Regression corpus.
- Differential report.

### M25-010 — Close codec performance and cold-start evidence (P1)

**Dependencies:** M25-002, M25-009

**Objective:** Prove the selected strategies improve real payloads without inflating startup unacceptably.

**Implementation:**
- Run C2 plus medium/large JSON workloads.
- Measure generated code/pack size.
- Report cold-start delta at 25/1,000 routes.
- Record CPU and RSS.

**Acceptance:**
- C2 materially improves or limitation is documented.
- No unapproved cold-start regression.
- Reports match raw data.
- Route-specific strategy is inspectable.

**Required evidence:**
- Raw performance suite.
- Generated report.
- Decision matrix.

## M25-GATE — Exit gate

- [ ] Canonical Schema IR drives runtime, Treaty, OpenAPI, lock, and diff.
- [ ] Generated decoders/encoders are semantically equivalent and bounded.
- [ ] Fallbacks are explicit and measured.
- [ ] Response errors/problems are exact and redacted correctly.
- [ ] Performance evidence supports route-level strategy selection.

## Required benchmark/evidence set

- C2 small JSON.
- 1KB/16KB/64KB dynamic payloads.
- Arrays 100/1,000.
- Request decode and response encode stage timings.

## Explicit exclusions

- No binary QPack encoding yet.
- No capability API expansion.
- No ORM.

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
