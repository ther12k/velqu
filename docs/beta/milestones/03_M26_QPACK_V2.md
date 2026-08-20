---
type: Milestone Plan
title: M2.6 — Binary QPack v2 and Reproducible Artifact ABI
status: draft
tags:
- milestone
- m26
- beta-roadmap

---

# M2.6 — Binary QPack v2 and Reproducible Artifact ABI

## Objective

Replace JSON/base64 startup reconstruction with a deterministic sectioned binary pack containing raw bytecode and precompiled runtime IR.

## Why this milestone exists

Current cold start is strong for small apps but route-count scaling still pays JSON parsing, base64 decoding, and owned-graph reconstruction. Beta distribution also needs a stable fail-closed artifact ABI.

## Entry criteria

- Required upstream dependencies in the task ledger are PASS.
- Working tree is clean and source/evidence baseline is identified.
- No unresolved upstream P0/P1 invalidates this milestone.

## Tasks

### M26-001 — Accept QPack v2 format and compatibility ADR (P0)

**Dependencies:** M25-GATE

**Objective:** Freeze the binary format goals, trust model, compatibility, and migration rules.

**Implementation:**
- Define numeric current mode and legacy v1 adapter.
- Specify section directory, alignment, bounds, optional sections, and versioning.
- Separate integrity from authenticity.
- Define debug/source sidecar policy.

**Acceptance:**
- Unknown versions fail closed.
- Current mode has no legacy handler table.
- Compatibility policy is explicit.
- Untrusted arbitrary bytecode is forbidden.

**Required evidence:**
- ADR.
- Binary layout diagrams.
- Compatibility matrix.

### M26-002 — Define strict runtime and bytecode fingerprint (P0)

**Dependencies:** M26-001

**Objective:** Prevent loading bytecode or plans under an incompatible engine/runtime build.

**Implementation:**
- Include runtime ABI, QuickJS-NG version/build hash, rquickjs version, bytecode format, target triple, pointer width, endianness, and capability hash.
- Fail closed on mismatch.
- Provide explicit source rebuild path.
- Never silently fall back.

**Acceptance:**
- Any fingerprint mismatch rejects before ready.
- Error identifies incompatible dimension.
- Engine upgrades require pack rebuild.
- Cross-target packs are rejected.

**Required evidence:**
- Fingerprint tests.
- Cross-build fixtures.
- Upgrade lane documentation.

### M26-003 — Encode compiled router, RoutePlans, schemas, policies, and functions as sections (P0)

**Dependencies:** M26-001, G0-GATE, M25-GATE

**Objective:** Serialize the already verified runtime graph without changing semantics.

**Implementation:**
- Define dense section schemas.
- Store router nodes/edges/terminals, RoutePlans, schema programs, policy plans, function manifest, debug names, and capability inventory.
- Use offsets and bounds checks.
- Bind sections to execution integrity.

**Acceptance:**
- No semantic reconstruction at startup.
- Bounds and index validation reject malformed packs.
- Binary and transitional representations are property-equivalent.
- Debug names are optional and non-hot.

**Required evidence:**
- Round-trip/property tests.
- Mutation fuzzing.
- Section-size report.

### M26-004 — Embed raw QuickJS bytecode without base64 (P0)

**Dependencies:** M26-002, M26-003

**Objective:** Remove base64 storage/decoding and duplicate production source by default.

**Implementation:**
- Store raw module bytecode section.
- Load exactly once.
- Make source optional sidecar/development section.
- Include prelude and handler manifest in the compiled module.

**Acceptance:**
- No base64 decode at startup.
- No source parse in bytecode production mode.
- Tamper/incompatibility rejects.
- Small-app source mode remains explicit if measured faster.

**Required evidence:**
- Bytecode integration tests.
- Tamper tests.
- Pack size/startup evidence.

### M26-005 — Implement zero-copy or bounded-copy pack reader (P0)

**Dependencies:** M26-003

**Objective:** Map and validate the pack without reconstructing large owned trees.

**Implementation:**
- Use mmap/read-only bytes where supported.
- Validate all section bounds before access.
- Avoid unsafe unchecked access unless independently audited.
- Support embedded pack bytes in standalone binary.

**Acceptance:**
- Malformed lengths cannot panic or read out of bounds.
- Startup allocations are measured and bounded.
- Reader works for shared and embedded modes.
- Fuzz parser remains stable.

**Required evidence:**
- Pack fuzz results.
- Allocation profile.
- Platform smoke tests.

### M26-006 — Implement execution integrity and authenticity hooks (P1)

**Dependencies:** M26-003, M26-004

**Objective:** Protect pack corruption now and provide optional publisher signature verification for beta artifacts.

**Implementation:**
- Hash required execution sections.
- Provide Ed25519-compatible signature slot/hook.
- Define key discovery/configuration.
- Keep unsigned local development supported with explicit policy.

**Acceptance:**
- Digest detects corruption.
- Signature verifies publisher when configured.
- Unsigned production policy is explicit.
- No docs conflate digest and authenticity.

**Required evidence:**
- Integrity/signature tests.
- Key rotation notes.
- Threat-model update.

### M26-007 — Guarantee reproducible release packs (P1)

**Dependencies:** M26-003, M26-004

**Objective:** Make identical source/locks/toolchain produce byte-identical packs.

**Implementation:**
- Remove timestamps/non-deterministic map order.
- Pin compiler/runtime versions.
- Canonicalize section ordering and padding.
- Compare independent build outputs.

**Acceptance:**
- Two clean builds produce identical SHA-256.
- Non-reproducibility is diagnosed.
- Build metadata lives outside deterministic payload or is canonical.
- CI verifies reproducibility.

**Required evidence:**
- Independent builder report.
- Artifact hashes.
- Reproducibility test.

### M26-008 — Provide explicit v1 compatibility and migration tool (P1)

**Dependencies:** M26-001, M26-005

**Objective:** Keep old packs supportable without contaminating current hot paths.

**Implementation:**
- Implement separate v1 reader/adapter.
- Provide `velqu pack migrate` or rebuild guidance.
- Deprecate mixed-mode packs.
- Test deterministic failures for unsupported legacy features.

**Acceptance:**
- Current runtime path allocates no legacy structures.
- Supported v1 pack either migrates or loads through adapter.
- Unsupported pack fails with actionable message.
- Migration does not change public contract.

**Required evidence:**
- Compatibility fixtures.
- Migration tests.
- Deprecation documentation.

### M26-009 — Build shared-runtime and standalone deployment artifacts (P1)

**Dependencies:** M26-004, M26-005

**Objective:** Support both small app updates and one-file deployment.

**Implementation:**
- Shared mode: `velqu-runtime` plus app.qpack.
- Standalone mode: embedded qpack executable.
- Ensure exact runtime fingerprint.
- Define source-map/debug sidecars.

**Acceptance:**
- Both modes pass identical conformance.
- Standalone contains no compiler toolchain.
- Shared mode rejects mismatched runtime.
- Startup/RSS differences are measured.

**Required evidence:**
- Artifact smoke tests.
- Size/cold-start report.
- Install guide.

### M26-010 — Close route-count cold-start evidence (P1)

**Dependencies:** M26-004, M26-005, M26-009

**Objective:** Demonstrate flatter startup scaling and preserve small-app behavior.

**Implementation:**
- Measure 25/100/1,000/5,000/10,000 routes.
- At least 100 fresh processes for release evidence.
- Randomize source/bytecode/competitor order.
- Record p50/p95/p99, RSS, stage timings, and hashes.

**Acceptance:**
- No runtime router/schema compilation.
- No base64 decoding.
- 25-route budget is not sacrificed silently.
- 10,000-route scaling is documented honestly.

**Required evidence:**
- Raw cold data.
- Generated report.
- Startup-stage trace.

## M26-GATE — Exit gate

- [ ] QPack v2 is deterministic, fail-closed, and version/fingerprint safe.
- [ ] Production startup maps verified runtime IR and raw bytecode without JSON/base64 reconstruction.
- [ ] Legacy compatibility is isolated.
- [ ] Shared and standalone artifacts pass conformance.
- [ ] Cold-start route scaling evidence is canonical.

## Required benchmark/evidence set

- 25/100/1,000/5,000/10,000 route cold start.
- Shared vs standalone RSS/startup.
- Pack parse/allocation stages.
- Source vs bytecode selection.

## Explicit exclusions

- No full capability ecosystem.
- No Node compatibility.
- No multi-worker yet.

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
