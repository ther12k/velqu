---
type: Milestone Plan
title: M2.7 — Capability Linker and Minimal Web Runtime
status: draft
tags:
- milestone
- m27
- beta-roadmap

---

# M2.7 — Capability Linker and Minimal Web Runtime

## Objective

Link only declared native capabilities, standardize lifecycle/cancellation, and provide a small WinterTC-aligned Web API subset without bloating unrelated applications.

## Why this milestone exists

Fetch, Postgres, authentication helpers, and future integrations need one lifecycle and ABI model. Implementing those first would accidentally define the capability architecture piecemeal.

## Entry criteria

- Required upstream dependencies in the task ledger are PASS.
- Working tree is clean and source/evidence baseline is identified.
- No unresolved upstream P0/P1 invalidates this milestone.

## Tasks

### M27-001 — Define capability ABI and lifecycle state machine (P0)

**Dependencies:** M26-GATE

**Objective:** Specify install, lazy init, invocation ownership, cancellation, drain, shutdown, versioning, and errors for native capabilities.

**Implementation:**
- Accept ADR.
- Define CapabilityId/version/dependencies.
- Define native operation owner/deadline state.
- Define lifecycle phases and bounded shutdown.

**Acceptance:**
- No capability can start work outside allowed phase.
- Every op is physically cancellable or explicitly non-cancellable.
- Version conflicts fail before ready.
- Shutdown reaches quiescence or fails closed.

**Required evidence:**
- Lifecycle state tests.
- Capability author guide draft.
- Threat review.

### M27-002 — Implement compile-time capability dependency resolver (P0)

**Dependencies:** M27-001

**Objective:** Resolve exactly which capabilities enter each application artifact.

**Implementation:**
- Build dependency DAG.
- Reject cycles/missing/conflicting versions.
- Emit capability inventory/hash into QPack.
- Remove unused modules.

**Acceptance:**
- Unrelated app pays zero linked capability cost.
- Dependency graph is deterministic.
- Missing capability fails at build or startup.
- `velqu inspect --capabilities` is accurate.

**Required evidence:**
- Resolver tests.
- Binary-size delta report.
- Cold-start delta report.

### M27-003 — Introduce custom QuickJS context profiles (P1)

**Dependencies:** M27-002

**Objective:** Measure minimal/web/full contexts and select only meaningful reductions.

**Implementation:**
- Build configurable intrinsic profiles.
- Compile application requirements.
- Report missing API/intrinsic diagnostics.
- Retain full profile for compatibility testing.

**Acceptance:**
- Chosen profile has measurable startup/RSS benefit or feature is deferred.
- No silent missing intrinsic.
- Conformance passes for selected profile.
- Profile identity enters runtime fingerprint.

**Required evidence:**
- Context benchmark.
- Test262 subset.
- Compatibility report.

### M27-004 — Implement console and timer core capabilities (P0)

**Dependencies:** M27-001, M27-002

**Objective:** Move existing timer behavior under the capability ABI and add bounded console semantics.

**Implementation:**
- Port timer cancellation/accounting.
- Define console levels and redaction.
- Keep logs asynchronous/bounded.
- Support shutdown and quarantine.

**Acceptance:**
- Existing scheduler invariants remain.
- No unbounded logging queue.
- Timers physically cancel.
- Capabilities absent when unused.

**Required evidence:**
- Regression suite.
- Lifecycle tests.
- Overhead measurement.

### M27-005 — Implement URL and URLSearchParams (P1)

**Dependencies:** M27-001, M27-003

**Objective:** Provide interoperable URL behavior for backend libraries and fetch.

**Implementation:**
- Adopt or adapt a proven implementation.
- Run selected WPT/WinterTC cases.
- Define host/path encoding behavior.
- Keep parser limits explicit.

**Acceptance:**
- Selected conformance threshold passes.
- No unbounded input behavior.
- URL behavior matches fetch usage.
- Binary/startup cost recorded.

**Required evidence:**
- WPT report.
- Edge-case fixtures.
- Module cost report.

### M27-006 — Implement TextEncoder and TextDecoder (P1)

**Dependencies:** M27-001, M27-003

**Objective:** Provide bounded text encoding primitives used by modern packages.

**Implementation:**
- Support UTF-8 baseline.
- Define invalid sequence/replacement behavior.
- Integrate TypedArray ownership.
- Run WPT subset.

**Acceptance:**
- Encoding semantics match selected standard cases.
- Large buffers are bounded.
- No duplicate full-buffer copies without evidence.
- Capability can be tree-linked.

**Required evidence:**
- WPT/conformance.
- Memory tests.
- Benchmark.

### M27-007 — Implement AbortController and AbortSignal (P0)

**Dependencies:** M27-001, M27-003

**Objective:** Create one cancellation primitive shared by fetch and native capabilities.

**Implementation:**
- Define signal state/listeners/reason.
- Bridge route deadline and explicit cancellation.
- Prevent listener leaks.
- Make cancellation idempotent.

**Acceptance:**
- Abort propagates exactly once.
- Late listeners follow defined semantics.
- No cross-invocation ownership.
- Shutdown cancellation is bounded.

**Required evidence:**
- Conformance tests.
- Leak tests.
- Race tests.

### M27-008 — Implement crypto random subset (P0)

**Dependencies:** M27-001, M27-003

**Objective:** Provide secure random bytes and UUID without broad crypto scope.

**Implementation:**
- Implement `getRandomValues` and `randomUUID` through OS CSPRNG.
- Enforce typed-array and size constraints.
- Define unavailable-entropy failure.
- Do not implement custom cryptography.

**Acceptance:**
- Random API fails closed.
- Input limits match intended standard.
- No predictable fallback.
- Security review passes.

**Required evidence:**
- Statistical smoke tests.
- WPT cases.
- Security review note.

### M27-009 — Publish capability SDK and inspection surface (P1)

**Dependencies:** M27-001, M27-002

**Objective:** Make first-party and external capabilities implementable without internal runtime mutation.

**Implementation:**
- Define Rust-side SDK traits and metadata.
- Provide test harness and example capability.
- Expose build/inspect diagnostics.
- Define semver/ABI compatibility.

**Acceptance:**
- Capability does not receive arbitrary mutable app state.
- SDK tests lifecycle/cancel/shutdown.
- Versioning is explicit.
- Example capability remains outside core.

**Required evidence:**
- SDK docs.
- Example package.
- Compatibility tests.

### M27-010 — Establish Web API conformance program (P1)

**Dependencies:** M27-005, M27-006, M27-007, M27-008

**Objective:** Separate standards compatibility from internal framework tests.

**Implementation:**
- Pin WPT/WinterTC subsets.
- Record skips and reasons.
- Automate regression reports.
- Keep unsupported APIs explicit.

**Acceptance:**
- No unsupported API is advertised.
- Pass/fail/skip counts are reproducible.
- Behavioral regressions block relevant gate.
- Reports link to exact runtime build.

**Required evidence:**
- Conformance report.
- Pinned test manifest.
- CI output.

### M27-011 — Close capability cost budgets (P1)

**Dependencies:** M27-002, M27-010

**Objective:** Prove modular capabilities preserve the cold-start and memory thesis.

**Implementation:**
- Measure core, web-minimal, and all-beta profiles.
- Record binary, startup, and idle RSS deltas.
- Identify eager initialization.
- Make expensive modules lazy when safe.

**Acceptance:**
- Core app remains near approved baseline.
- Each capability cost is visible.
- Unused capability cost is zero or explained.
- Budget failures trigger split/defer decisions.

**Required evidence:**
- Cost matrix.
- Cold/RSS raw data.
- Linker report.

## M27-GATE — Exit gate

- [ ] Capability ABI is versioned, bounded, cancellable, and testable.
- [ ] Only declared capabilities are linked.
- [ ] Minimal Web APIs meet documented conformance.
- [ ] Capability cost remains visible and controlled.
- [ ] SDK does not compromise compiler/runtime determinism.

## Required benchmark/evidence set

- Core vs web-minimal startup/RSS.
- Timer/abort overhead.
- URL/encoding throughput and allocation.
- Capability binary-size matrix.

## Explicit exclusions

- No Node module compatibility.
- No filesystem/process APIs for beta.
- No WebSocket/SSE.

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
