---
type: Milestone Plan
title: M23R2 Gate Closure — Trusted Numeric Artifact and Router
status: draft
tags:
- milestone
- g0
- beta-roadmap

---

# M23R2 Gate Closure — Trusted Numeric Artifact and Router

## Objective

Convert the strong M2.3-r3 implementation into a production-gate-quality numeric artifact whose functions, router, schemas, policies, hashes, evidence, and release metadata are all fail-before-ready and self-consistent.

## Why this milestone exists

M2.4 will trust RouteId, SchemaId, PolicyId, and FieldNeeds to decide what request data crosses into JavaScript. Those identities must be exact before they become admission and security controls.

## Entry criteria

- Authoritative baseline is frozen and review findings are recorded.
- Working tree is clean and source/evidence baseline is identified.
- No unresolved upstream P0/P1 invalidates this milestone.

## Tasks

### G0-001 — Freeze and verify the M2.3-r3 baseline (P0)

**Dependencies:** None

**Objective:** Establish one authoritative source checkpoint for all beta work.

**Implementation:**
- Clone and verify the Git bundle.
- Verify the source ZIP tree against commit e2b379d775a79e619753aaf39eb9ea5f8a763f15.
- Record compiler, Rust, Bun, QuickJS-NG, rquickjs, OS, CPU, and benchmark-tool versions.
- Remove or quarantine stale release metadata from the working tree.

**Acceptance:**
- Clean Git tree at baseline.
- Source tree matches the recorded commit.
- External and internal checksum manifests identify only the current checkpoint.
- Baseline report records known open M23R2 gate findings.

**Required evidence:**
- Captured git bundle verification.
- Tree-diff report.
- Baseline environment manifest.
- Commit-named source archive and SHA-256.

### G0-002 — Make the semantic function manifest mandatory (P0)

**Dependencies:** G0-001

**Objective:** Ensure every numeric function vector entry is semantically tied to its key and route/policy kind.

**Implementation:**
- Require `__velquFunctionManifest` in current numeric mode.
- Reject count-only `__velquFunctions` fallback in current packs.
- Keep any count-only behavior only in an explicit legacy pack adapter.
- Verify exact index, key, kind, and callability before caching.

**Acceptance:**
- Missing semantic manifest rejects before socket bind.
- Swapped callable entries reject.
- Route/policy kind mismatch rejects.
- No numeric request can execute through the legacy map.

**Required evidence:**
- Negative engine-load tests.
- Numeric dispatch counters.
- Startup failure diagnostics snapshot.

### G0-003 — Bind router and schema manifests into the execution graph hash (P0)

**Dependencies:** G0-001

**Objective:** Make any mutation to routing or numeric schema identity detectable before ready.

**Implementation:**
- Define `executionGraphHash` over functions, RoutePlans, policies, schema manifest, capability bindings, and serialized router.
- Recompute and verify it in `QPack::verify()`.
- Retain a separate public contract hash.
- Add tamper fixtures that redirect valid terminals to the wrong valid RouteId.

**Acceptance:**
- Router terminal mutation changes the hash or is rejected.
- Schema manifest mutation changes the hash or is rejected.
- Public contract hash remains unchanged by internal ID reordering.
- Arbitrary supplied hashes are rejected.

**Required evidence:**
- Pack tamper tests.
- Canonicalization golden fixtures.
- Hash separation report.

### G0-004 — Load the serialized router directly (P0)

**Dependencies:** G0-003

**Objective:** Remove runtime semantic route reconstruction for current numeric packs.

**Implementation:**
- Validate serialized nodes, edges, terminals, method masks, path shapes, and RouteId references.
- Load arrays directly with no call to `Router::build` in current numeric mode.
- Keep reference construction only in compiler tests or legacy adapters.
- Add generated-route property comparison against a known-correct matcher.

**Acceptance:**
- Current numeric startup performs zero route parsing/collision reconstruction.
- 404/405 and Allow semantics match the reference matcher.
- Route-specific parameter names are preserved.
- Every compiled route is reachable exactly as intended.

**Required evidence:**
- Startup instrumentation.
- Router property-test corpus.
- 10,000-route load test.

### G0-005 — Complete operational RouteId, PolicyId, and SchemaId usage (P0)

**Dependencies:** G0-002, G0-004

**Objective:** Remove string identity lookup from normal current-pack request execution.

**Implementation:**
- Router returns RouteId.
- RouteId directly indexes RoutePlan.
- PolicyId resolves to a verified policy plan and pre-resolved HandlerId.
- SchemaId directly indexes schema programs.
- Move readable names into debug/inspection tables.

**Acceptance:**
- No route, policy, handler, or schema string lookup on the normal numeric path.
- Numeric manifests are dense and complete.
- Invalid IDs reject before ready.
- Diagnostics still report readable names.

**Required evidence:**
- Hot-path counter proving zero legacy lookups.
- Malformed numeric-graph tests.
- `velqu inspect` snapshot.

### G0-006 — Separate and verify public contract identity (P1)

**Dependencies:** G0-003, G0-005

**Objective:** Make the public contract hash represent only externally observable API semantics.

**Implementation:**
- Define a dedicated public canonical model.
- Include method/path, request schemas, content/coercion semantics, responses, public problems, and security requirements.
- Exclude handler names, serializer implementation, numeric IDs, and router layout.
- Verify supplied public hash in QPack.

**Acceptance:**
- Internal HandlerId reorder leaves public hash unchanged.
- Wire-visible schema/security/status changes change the public hash.
- Serializer implementation change leaves public hash unchanged.
- Treaty/OpenAPI/lock all identify the same public graph.

**Required evidence:**
- Hash stability tests.
- Contract projection parity tests.
- Semantic diff fixtures.

### G0-007 — Remove duplicate legacy state from current packs (P1)

**Dependencies:** G0-002, G0-005

**Objective:** Make numeric execution mode explicit and keep legacy compatibility isolated.

**Implementation:**
- Introduce explicit current pack execution mode/version.
- Current numeric packs contain function/schema/policy manifests, RoutePlans, and serialized router.
- Prohibit `handlerTable` and registration metadata in current numeric packs.
- Provide a separately tested v1 compatibility adapter.

**Acceptance:**
- Current pack has zero handlerTable entries.
- Worker allocates no legacy handler cache.
- Legacy pack compatibility is explicit, not inferred.
- Compiler and runtime reject mixed-mode artifacts.

**Required evidence:**
- Pack-format fixtures.
- Memory/startup comparison.
- Legacy migration test.

### G0-008 — Close canonical performance evidence (P1)

**Dependencies:** G0-004, G0-005, G0-007

**Objective:** Replace spot checks with reproducible M23 gate evidence.

**Implementation:**
- Run warm workloads at concurrency 1, 10, 50 with at least five repetitions and randomized candidate order.
- Run cold start at 25, 1,000, and 10,000 routes with fresh processes.
- Capture CPU, RSS, errors, p50/p95/p99, artifact hashes, and machine state.
- Add allocation/profile evidence and raw-to-report generation.

**Acceptance:**
- Markdown reports are generated from current raw data.
- Verifier fails on stale reports.
- No public claim uses a single spot check.
- Any regression is documented rather than hidden.

**Required evidence:**
- Raw benchmark directory.
- Generated report.
- Environment and artifact manifest.
- Ablation results for relevant changes.

### G0-009 — Create self-verifying milestone and evidence indexes (P1)

**Dependencies:** G0-001, G0-008

**Objective:** Make the checkpoint independently reviewable and prevent source/evidence drift.

**Implementation:**
- Create review index mapping gates to source, tests, raw evidence, report, commit, and artifact hash.
- Create a current-only release directory.
- Generate checksums after all artifacts are fixed.
- Make verification reject missing evidence paths and stale milestone labels.

**Acceptance:**
- `sha256sum -c` equivalent passes for the release packet.
- Every PASS task references existing evidence.
- No stale previous bundle is presented as current.
- Git bundle, source ZIP, and source commit agree.

**Required evidence:**
- Review index.
- Evidence index.
- Release packet validation report.

## G0-GATE — Exit gate

- [ ] Current numeric startup requires semantic function identity and accepts no count-only fallback.
- [ ] Serialized router and schema/function plans are integrity-bound and loaded without runtime semantic reconstruction.
- [ ] RouteId, PolicyId, HandlerId, and SchemaId are operational; names are diagnostic only.
- [ ] Public contract and execution graph hashes are separate and independently verified.
- [ ] Canonical warm/cold evidence meets the frozen protocol and reports match raw data.
- [ ] Release packet is self-verifying and task/evidence state is truthful.

## Required benchmark/evidence set

- Warm C0–C3: c=1/10/50, five repetitions.
- Cold: 25/1,000/10,000 routes, fresh processes.
- Allocation and startup-stage profile.
- No regression threshold is silently relaxed.

## Explicit exclusions

- No M2.4 request-slab integration.
- No new capability APIs.
- No database implementation.
- No multi-worker changes.

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
