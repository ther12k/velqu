# Velqu Production-Readiness Task Backlog

Baseline: `velqu-m0-m2-20260819T093558Z.zip` / `03a06bbdcc7b4f7626dd5b287983c4f3b6d26ff82e4895923284d76af92debb5`.

Total tasks and milestone gates: **120**.

Status markers are maintained in `TASKS.json`; this Markdown file is the human-readable execution view.

## BASE — Baseline Freeze and Program Integration

**Required outcome:** The current M2.3-r1 baseline, roadmap authority, evidence model, and review packet format are frozen before further code changes.

### - [ ] BASE-001 — Freeze the reviewed source baseline `P0`

Bind all future work to the exact reviewed archive and a real Git commit.

**Depends on:** none

**Acceptance:**
- SOURCE-BASELINE.md records archive name, SHA-256, source commit, toolchain locks, and known M2.3-r1 gaps.
- A git bundle or repository commit is available; later review never relies only on an unversioned ZIP.

**Required evidence:**
- SOURCE-BASELINE.md
- git rev-parse output
- SHA256SUMS.txt
### - [ ] BASE-002 — Adopt ADR-0019 production-readiness program `P0`

Extend ADR-0018 through technical production candidate, RC, and GA without changing the minimal-core thesis.

**Depends on:** BASE-001

**Acceptance:**
- ADR-0019 is accepted and indexed.
- The roadmap explicitly separates technical production readiness from public GA owner decisions.
- Non-goals remain Node compatibility, hostile-code sandboxing, ORM-in-core, and unauthorized WebSocket/SSE work.

**Required evidence:**
- ADR file
- decision index link
- OKF validation
### - [ ] BASE-003 — Install the production master agent prompt `P0`

Give one coding agent authority to execute the full ordered program while preserving stop conditions and owner boundaries.

**Depends on:** BASE-002

**Acceptance:**
- AGENTS.md points to the new master prompt.
- The former M2/M4 stop language is superseded unambiguously.
- The agent may continue milestone-to-milestone only after local gates pass.

**Required evidence:**
- MASTER_PRODUCTION_AGENT_PROMPT.md
- AGENTS.md diff
### - [ ] BASE-004 — Create machine-readable task and evidence ledgers `P0`

Make status, dependencies, claims, tests, reports, and artifacts auditable by tools and the final reviewer.

**Depends on:** BASE-003

**Acceptance:**
- TASKS.json validates against its schema.
- Every task has acceptance and evidence fields.
- Every completed claim links to source/test/raw evidence rather than prose alone.

**Required evidence:**
- TASKS.json
- TASKS.schema.json
- EVIDENCE_INDEX.json schema
### - [ ] BASE-005 — Strengthen source-to-evidence verification `P0`

Prevent reports from claiming code, tests, or benchmark results absent from the packaged source.

**Depends on:** BASE-004

**Acceptance:**
- verify fails stale generated reports, absent named tests, duplicate current-verification sections, and mismatched artifact hashes.
- Benchmark summaries are generated from raw data.
- Test counts come from captured runner output, not manual arithmetic.

**Required evidence:**
- verify output
- negative fixtures
- generated report hash comparison
### - [ ] BASE-GATE — Pass baseline program gate `P0`

Authorize implementation only after the baseline, roadmap, prompt, evidence ledger, and review conventions are coherent.

**Depends on:** BASE-005

**Acceptance:**
- BASE-001 through BASE-005 are PASS.
- Working tree is clean.
- A checkpoint commit and review packet exist.

**Required evidence:**
- BASE review packet
- commit hash
- source archive + checksum

## M23R2 — M2.3-r2 — Exact Numeric Runtime IR and Compiled Router Closure

**Required outcome:** Current-pack execution is fully numeric, fail-before-ready, route plans are exact, schema identities are operational, and the router resolves directly to a numeric RoutePlan.

### - [ ] M23R2-001 — Define explicit numeric and legacy engine load plans `P0`

Separate current numeric packs from legacy string-table compatibility so no current request pays dual-path cost.

**Depends on:** BASE-GATE

**Acceptance:**
- Numeric mode takes FunctionDecl[] as the only vector contract.
- Numeric mode requires __velquFunctions and rejects absent/mismatched vectors before bind.
- Legacy mode is version-gated and cannot be selected accidentally.

**Required evidence:**
- unit tests
- pack fixtures
- startup failure logs
### - [ ] M23R2-002 — Enforce exact function-vector and manifest equivalence `P0`

Make dense numeric dispatch fail closed for wrong in-range IDs, holes, kinds, duplicates, and length mismatches.

**Depends on:** M23R2-001

**Acceptance:**
- Function IDs are dense and pack-local.
- Each vector index is callable and matches exact key/kind.
- No loader compaction or silent fallback occurs.

**Required evidence:**
- QPack negative tests
- engine load tests
- numeric dispatch counters
### - [ ] M23R2-003 — Enforce exact RoutePlan equivalence `P0`

Cross-check the execution plan against route contracts before readiness.

**Depends on:** M23R2-002

**Acceptance:**
- Declared response status set equals allowedStatuses exactly.
- defaultStatus, deadline, response strategy, policy handler, field needs, and schema bindings match the canonical route.
- Invalid HTTP status values and duplicates are rejected.

**Required evidence:**
- pack validation tests
- golden plans
- tamper fixtures
### - [ ] M23R2-004 — Operationalize RouteId, PolicyId, and SchemaId `P0`

Move all current-pack route, policy, and schema identity to deterministic dense numeric indexes while retaining debug names.

**Depends on:** M23R2-003

**Acceptance:**
- Compiler assigns stable deterministic IDs.
- All schema references are non-null when a schema exists.
- Debug-name tables preserve human-readable diagnostics without entering the hot path.

**Required evidence:**
- compiler golden output
- type/pack tests
- inspect output
### - [ ] M23R2-005 — Compile FieldNeeds into a verified bitset `P0`

Prepare M2.4 request admission decisions from exact route declarations.

**Depends on:** M23R2-004

**Acceptance:**
- FieldNeeds is a compact bitset or equivalent fixed representation.
- Compiler derivation matches route params/query/headers/body/cookies/raw-request requirements.
- Plan tampering or disagreement fails before ready.

**Required evidence:**
- field-needs conformance matrix
- negative pack tests
### - [ ] M23R2-006 — Bind the numeric execution graph to pack integrity `P0`

Detect corruption of function manifests, route plans, and policy-to-handler mappings.

**Depends on:** M23R2-005

**Acceptance:**
- executionGraphSha256 or equivalent covers all execution-critical numeric metadata.
- Public contract hash remains stable when only internal numeric layout changes.
- Tamper fixtures fail before bind.

**Required evidence:**
- integrity tests
- hash fixtures
- pack spec update
### - [ ] M23R2-007 — Implement numeric terminal router automaton `P0`

Replace startup-built candidate scanning with one compiled traversal to a method terminal and RoutePlan.

**Depends on:** M23R2-006

**Acceptance:**
- Router returns RouteId/RoutePlan directly.
- 405 Allow comes from a terminal method mask without repeated matching.
- Collision and shadow detection occur at compile time.
- No Router::build semantic reconstruction is required for current packs.

**Required evidence:**
- router conformance
- 25/1k/10k route fixtures
- allocation profile
### - [ ] M23R2-008 — Remove current-pack string dispatch and duplicate references `P0`

Eliminate __velquRegister, handler BTreeMap, per-request handler/policy strings, and duplicate persistent handles for current packs.

**Depends on:** M23R2-007

**Acceptance:**
- numeric_dispatches > 0 and legacy_map_dispatches == 0 for proof app.
- No registration calls in current generated bundles.
- Legacy support lives only in explicit pack-version compatibility code.

**Required evidence:**
- source inspection check
- runtime counters
- bundle size report
### - [ ] M23R2-009 — Finish terminal settlement retention hardening `P0`

Ensure every timeout, cancel, quarantine, and interrupted watch removes settlement entries and floating operations exactly once.

**Depends on:** M23R2-008

**Acceptance:**
- Settlement table remains zero after repeated terminal cycles.
- Timeout metric increments once.
- Queue-empty-or-quarantined invariant holds at every worker message boundary.

**Required evidence:**
- engine tests
- settlement-table metric
- task/slot gauges
### - [ ] M23R2-GATE — Pass M2.3-r2 closure gate `P0`

Close exact numeric artifact loading, router scope, evidence, and performance before M2.4 consumes RoutePlan.

**Depends on:** M23R2-009

**Acceptance:**
- M23R2-001 through M23R2-009 are PASS.
- Warm c=1/10/50 and 25/1,000-route cold evidence is regenerated with raw files and hashes.
- No current-pack string lookup or runtime route reconstruction remains.
- Treaty/OpenAPI/runtime behavior is unchanged.

**Required evidence:**
- M2.3-r2 report
- raw benchmark data
- fairness audit
- checkpoint archive

## M24 — M2.4 — Zero-Copy Ingress and Worker-Local Request Bridge

**Required outcome:** Requests route before decoding, unread fields are never materialized, and request state is owned by the QuickJS worker without a process-wide request-store mutex.

### - [ ] M24-001 — Freeze ingress ownership and backpressure design `P0`

Define how Hyper request parts and bodies move from HTTP tasks to one owning QuickJS worker without unbounded buffering.

**Depends on:** M23R2-GATE

**Acceptance:**
- Ownership, cancellation, body-stream, and queue invariants are documented in an ADR.
- No JS value or request slot crosses worker ownership ambiguously.

**Required evidence:**
- ADR
- sequence diagrams
- negative design review
### - [ ] M24-002 — Route before request materialization `P0`

Match method/path using borrowed HTTP types before query, headers, cookies, or body are decoded.

**Depends on:** M24-001

**Acceptance:**
- C0/C1 routes do not parse query or clone headers.
- Routes without bodies do not collect bodies.
- Admission limits still apply before unsafe buffering.

**Required evidence:**
- instrumentation counters
- route tests
- allocation traces
### - [ ] M24-003 — Introduce worker-local generation-checked request slab `P0`

Remove the process-wide RequestStore mutex and make the QuickJS worker the sole owner of live request state.

**Depends on:** M24-002

**Acceptance:**
- Request slot allocation/access/settlement is lock-free inside the worker.
- Generation and owner checks reject stale or foreign handles.
- All slots are zero after quiescence.

**Required evidence:**
- bridge tests
- race tests
- slot gauges
### - [ ] M24-004 — Capture path parameters as byte ranges `P0`

Avoid per-request parameter strings until validation or JS access requires them.

**Depends on:** M24-003

**Acceptance:**
- Router captures offsets into the original path.
- Native validators consume slices directly.
- JS strings are created only for requested values.

**Required evidence:**
- param conformance
- allocation benchmark
- Unicode/path tests
### - [ ] M24-005 — Implement declared-header lazy access `P0`

Read only headers named by policies or route schemas and materialize full Headers only through explicit fallback.

**Depends on:** M24-004

**Acceptance:**
- No full header map clone on routes with no/limited header needs.
- Case-insensitive semantics and duplicate-header behavior are specified and tested.
- Raw enumeration is explicitly labeled as generic fallback.

**Required evidence:**
- header matrix
- fallback build report
- allocation counters
### - [ ] M24-006 — Implement lazy query and cookie decoding `P0`

Decode declared keys on demand while retaining raw fallback semantics.

**Depends on:** M24-005

**Acceptance:**
- No query parser runs when FieldNeeds excludes query.
- Duplicate keys, percent encoding, empty values, and invalid encoding have frozen semantics.
- Cookie parsing is bounded and opt-in.

**Required evidence:**
- query/cookie conformance
- fuzz targets
- materialization counters
### - [ ] M24-007 — Implement bounded read-once body admission `P0`

Support bytes/text/JSON body access with limits, cancellation, backpressure, and deterministic second-read behavior.

**Depends on:** M24-006

**Acceptance:**
- Oversize bodies fail without unbounded allocation.
- Request disconnect and route cancellation stop body reads.
- Bytes remain Bytes where possible rather than Vec copies.

**Required evidence:**
- body tests
- slowloris/oversize fixtures
- memory report
### - [ ] M24-008 — Replace per-request JS closure construction with native-backed prototypes `P0`

Use one stable QuickJS object shape and opaque request handle instead of multiple getter closures per invocation.

**Depends on:** M24-007

**Acceptance:**
- Context/request prototypes are cached once.
- Native access validates phase, owner, slot, and generation.
- Retained wrappers throw stable RequestExpiredError.

**Required evidence:**
- shape/cache tests
- bridge microbenchmarks
- expiry tests
### - [ ] M24-009 — Add ingress and bridge observability `P0`

Expose route lookup, queue wait, field materialization, bytes copied, slot count, and body read timing without hot-path logging.

**Depends on:** M24-008

**Acceptance:**
- Metrics are bounded and disableable.
- velqu inspect shows FieldNeeds and expected materialization.
- No string request ID is created in log-off mode.

**Required evidence:**
- metrics tests
- inspect snapshot
- performance overhead A/B
### - [ ] M24-GATE — Pass zero-copy ingress gate `P0`

Prove lazy materialization, ownership safety, cancellation, and meaningful fixed-overhead reduction.

**Depends on:** M24-009

**Acceptance:**
- M24-001 through M24-009 are PASS.
- No global request-store mutex remains.
- Unread fields have zero materialization counters.
- C0 >= 90% of matched raw Rust; C1/C3 p95 do not regress; bridge safety suites pass.

**Required evidence:**
- M2.4 report
- warm/bridge raw evidence
- FFI ownership review
- checkpoint archive

## M25 — M2.5 — Schema-Specialized Input and JSON Output Pipeline

**Required outcome:** The compiler selects measured, schema-aware decoders and encoders while maintaining exact Treaty/OpenAPI/runtime semantics.

### - [ ] M25-001 — Define canonical Schema IR v2 and compatibility rules `P0`

Make one normalized schema graph drive validators, codecs, Treaty, OpenAPI, and contract diff.

**Depends on:** M24-GATE

**Acceptance:**
- Schema IDs and canonical encoding are deterministic.
- Supported/unsupported transforms are explicit.
- IR versioning and migrations are documented.

**Required evidence:**
- schema spec
- golden fixtures
- compatibility tests
### - [ ] M25-002 — Build reproducible decoder/encoder strategy benchmark `P0`

Compare QuickJS parse/stringify, generic Rust conversion, and generated schema-specialized codecs across realistic shapes.

**Depends on:** M25-001

**Acceptance:**
- Payload matrix includes scalar, nested, arrays 100/1000, 1/16/64/128 KB, invalid and schema-invalid inputs.
- Raw latency, allocation, bytes copied, heap, and correctness are retained.

**Required evidence:**
- raw benchmark data
- strategy decision ADR
- fairness audit
### - [ ] M25-003 — Generate params/query/header decoders `P0`

Fuse extraction, coercion, validation, and error-path reporting for scalar request fields.

**Depends on:** M25-002

**Acceptance:**
- No generic object tree is created when route schemas are supported.
- Validation errors preserve safe paths and stable codes.
- Coercion remains source-specific and explicit.

**Required evidence:**
- decoder tests
- negative fixtures
- allocation profile
### - [ ] M25-004 — Generate JSON body decoders `P0`

Parse, validate, and materialize supported bodies in one measured route-specific path.

**Depends on:** M25-003

**Acceptance:**
- Correct Unicode, number edge cases, required/optional/null, arrays, nesting, and unknown-property policy.
- Body limits and cancellation remain enforced.
- Fallback is visible in build report.

**Required evidence:**
- JSON conformance
- fuzz/property tests
- fallback snapshots
### - [ ] M25-005 — Generate status-specific response encoders `P0`

Fuse response validation and serialization for declared shapes while preserving typed status semantics.

**Depends on:** M25-004

**Acceptance:**
- No duplicate full-object validation plus serialization traversal.
- Every declared status uses the correct schema.
- Undeclared status or incompatible shape fails deterministically.

**Required evidence:**
- encoder tests
- response violation tests
- performance traces
### - [ ] M25-006 — Generate RFC 9457 problem encoders `P0`

Preserve standard fields and custom typed problem fields without leaking secrets.

**Depends on:** M25-005

**Acceptance:**
- Problem schema/type/OpenAPI/Treaty outputs agree.
- Getter/toJSON behavior remains deadline-bounded.
- Redaction tests cover nested causes and source maps.

**Required evidence:**
- problem golden files
- redaction suite
- Treaty type tests
### - [ ] M25-007 — Implement explicit generic and Web fallback paths `P0`

Keep raw Request/Response and unsupported schema semantics available without hiding their cost.

**Depends on:** M25-006

**Acceptance:**
- Every fallback appears in build report and velqu inspect.
- Fallback routes remain bounded and conformance-tested.
- Core optimized routes do not import fallback modules.

**Required evidence:**
- fallback fixtures
- bundle-size report
- inspect output
### - [ ] M25-008 — Unify OpenAPI, Treaty, lock, and runtime schema projection `P0`

Prevent drift between runtime codecs and developer-facing contracts.

**Depends on:** M25-007

**Acceptance:**
- One canonical Schema IR projection feeds all artifacts.
- Semantic diff detects nested object/array/union/enum/constraint changes.
- Published client has no runtime implementation dependency.

**Required evidence:**
- contract parity suite
- OpenAPI validation
- diff fixtures
### - [ ] M25-009 — Add codec fuzzing and differential tests `P0`

Compare generated codecs with a trusted reference and exercise malformed inputs/outputs.

**Depends on:** M25-008

**Acceptance:**
- No panic, UB, unbounded allocation, or divergent accepted value in supported IR subset.
- Fuzz corpora and seeds are retained.
- Failures become regression tests.

**Required evidence:**
- fuzz logs
- corpus archive
- differential report
### - [ ] M25-GATE — Pass schema-specialized pipeline gate `P0`

Select route-level codecs from evidence and close semantic parity.

**Depends on:** M25-009

**Acceptance:**
- M25-001 through M25-009 are PASS.
- C2 reaches the approved engineering threshold or limitations are explicitly accepted.
- Codec choice is inspectable per route.
- No contract drift exists across runtime/Treaty/OpenAPI/lock.

**Required evidence:**
- M2.5 report
- codec raw evidence
- contract parity report
- checkpoint archive

## M26 — M2.6 — Binary QPack v2 and Reproducible Artifact ABI

**Required outcome:** Production startup maps a deterministic binary pack with raw bytecode, precompiled router/runtime IR, strict ABI fingerprinting, and no JSON/base64 reconstruction.

### - [ ] M26-001 — Accept QPack v2 binary format ADR `P0`

Freeze a sectioned, bounds-checkable, versioned production artifact that matches the compiled runtime IR.

**Depends on:** M25-GATE

**Acceptance:**
- Header, section directory, alignment, endianness, size limits, and forward-compatibility rules are specified.
- Development JSON remains inspection-only.

**Required evidence:**
- ADR
- binary layout diagrams
- format spec
### - [ ] M26-002 — Define strict runtime and bytecode fingerprint `P0`

Reject incompatible engine/build/ABI/target artifacts before evaluating code.

**Depends on:** M26-001

**Acceptance:**
- Fingerprint includes QPack format, host ABI, engine/build hash, binding, bytecode format, target, pointer width, endianness, and capability hash.
- Mismatch tests fail before bind.

**Required evidence:**
- fingerprint tests
- upgrade fixtures
- diagnostics
### - [ ] M26-003 — Encode compiled router, RoutePlans, schemas, and functions as binary sections `P0`

Persist the exact M2.3/M2.5 runtime graph without rebuilding it at startup.

**Depends on:** M26-002

**Acceptance:**
- No runtime route collision work or status-string parsing.
- All indexes and section references are bounds-checked.
- Debug names are optional cold data.

**Required evidence:**
- round-trip tests
- malformed section tests
- startup stage trace
### - [ ] M26-004 — Embed raw QuickJS bytecode without base64 `P0`

Remove base64 decode and duplicate production source from the hot startup path.

**Depends on:** M26-003

**Acceptance:**
- Raw bytecode is one section and is hashed.
- Source fallback is explicit and off by default for release.
- No untrusted external bytecode is accepted.

**Required evidence:**
- bytecode parity
- tamper tests
- artifact-size report
### - [ ] M26-005 — Implement zero-copy or bounded-copy pack reader `P0`

Map or read binary sections without reconstructing large owned object graphs.

**Depends on:** M26-004

**Acceptance:**
- Every offset/length is validated before access.
- Peak startup allocations are measured.
- Malformed packs never panic or read out of bounds.

**Required evidence:**
- parser fuzzing
- allocation profile
- security review
### - [ ] M26-006 — Add execution integrity and optional authenticity hooks `P0`

Separate corruption detection from publisher authenticity.

**Depends on:** M26-005

**Acceptance:**
- Canonical section digests cover all executable metadata.
- Optional Ed25519 or equivalent signature verification is pluggable.
- Docs clearly distinguish digest vs signature.

**Required evidence:**
- integrity/signature fixtures
- threat-model update
### - [ ] M26-007 — Guarantee reproducible release packs `P0`

Produce identical pack bytes for identical normalized inputs.

**Depends on:** M26-006

**Acceptance:**
- Two clean builds produce identical hashes.
- Timestamps and host paths do not contaminate output.
- Build report records all non-reproducible inputs.

**Required evidence:**
- reproducibility script
- two-build evidence
- diffoscope-style report
### - [ ] M26-008 — Provide explicit v1 compatibility and migration tool `P0`

Avoid accidental dual paths inside the current runtime while supporting controlled migration.

**Depends on:** M26-007

**Acceptance:**
- v1 loads only through versioned adapter or conversion command.
- v2 runtime path allocates no v1 maps.
- Unsupported legacy artifacts fail with actionable diagnostics.

**Required evidence:**
- migration fixtures
- compatibility matrix
- CLI tests
### - [ ] M26-009 — Build shared-runtime and standalone deployment artifacts `P0`

Support velqu-runtime + app.qpack and one-file embedded application modes from the same ABI.

**Depends on:** M26-008

**Acceptance:**
- Both modes pass identical conformance.
- Artifact hashes and sizes are recorded.
- Graceful startup/shutdown semantics match.

**Required evidence:**
- packaging tests
- binary size report
- container smoke tests
### - [ ] M26-GATE — Pass QPack v2 gate `P0`

Prove startup reconstruction is gone and artifact trust/versioning is fail-closed.

**Depends on:** M26-009

**Acceptance:**
- M26-001 through M26-009 are PASS.
- 25-route p50 <= approved small-app budget and 1,000-route p50/p95 meet approved scaling budget.
- No JSON parse, base64 decode, string registration, or runtime Router::build in v2 release path.
- Pack parser fuzz/security review passes.

**Required evidence:**
- M2.6 report
- cold raw evidence
- pack spec
- checkpoint archive

## M27 — M2.7 — Capability Linker and Minimal Web Runtime

**Required outcome:** Only declared native capabilities are linked, lifecycle and cancellation are standardized, and a small WinterTC-aligned Web API subset is conformant.

### - [ ] M27-001 — Define capability ABI and lifecycle state machine `P0`

Standardize install, dependency, lazy init, operation ownership, cancellation, shutdown, and version behavior.

**Depends on:** M26-GATE

**Acceptance:**
- Capability identity/version/dependencies are explicit.
- No capability can start native work outside an invocation/deferred owner.
- Shutdown and quarantine semantics are defined.

**Required evidence:**
- ADR
- trait/API spec
- lifecycle tests
### - [ ] M27-002 — Implement compile-time capability dependency resolver `P0`

Link only required modules and reject cycles, conflicts, and missing versions before packaging.

**Depends on:** M27-001

**Acceptance:**
- Dependency graph is deterministic.
- Unused capabilities are absent from pack/binary.
- Capability hash participates in runtime fingerprint.

**Required evidence:**
- resolver tests
- build report
- negative fixtures
### - [ ] M27-003 — Introduce custom QuickJS context profiles `P0`

Measure minimal, Web, and compatibility contexts rather than always loading a full context.

**Depends on:** M27-002

**Acceptance:**
- Profile choice is compiler-derived and inspectable.
- No intrinsic removal without conformance and measurable benefit.
- Missing APIs produce build diagnostics.

**Required evidence:**
- context benchmark
- Test262 subset
- profile manifest
### - [ ] M27-004 — Implement console and timer core capabilities `P0`

Move current natives behind the capability ABI and retain scheduler/cancellation invariants.

**Depends on:** M27-003

**Acceptance:**
- Logging is structured/redacted and mode-controlled.
- Timers are bounded, owned, physically cancellable, and shutdown-safe.

**Required evidence:**
- capability tests
- scheduler regression suite
### - [ ] M27-005 — Implement URL and URLSearchParams `P0`

Provide standards-aligned URL behavior required by backend code and fetch.

**Depends on:** M27-004

**Acceptance:**
- Selected WPT/WinterTC cases pass.
- Parsing limits and error behavior are documented.
- Binary/startup/RSS cost is reported.

**Required evidence:**
- WPT results
- compatibility matrix
- size report
### - [ ] M27-006 — Implement text encoding APIs `P0`

Provide TextEncoder/TextDecoder with explicit encoding support and bounds.

**Depends on:** M27-005

**Acceptance:**
- UTF-8 correctness, invalid sequences, streaming options if supported, and large input limits are tested.
- No hidden Node Buffer dependency.

**Required evidence:**
- WPT/conformance tests
- memory tests
### - [ ] M27-007 — Implement AbortController and AbortSignal `P0`

Create one cancellation contract shared by timers, fetch, and future native operations.

**Depends on:** M27-006

**Acceptance:**
- Abort reasons and event timing are deterministic.
- Route deadlines compose with explicit signals.
- Late completion and listener cleanup are leak-free.

**Required evidence:**
- abort conformance
- race tests
- retention tests
### - [ ] M27-008 — Implement crypto random subset `P0`

Provide getRandomValues and randomUUID from secure native entropy without broad crypto scope.

**Depends on:** M27-007

**Acceptance:**
- Entropy source failures fail closed.
- Input type/length rules are conformant.
- No deterministic fallback exists in production.

**Required evidence:**
- crypto conformance
- failure injection
- security review
### - [ ] M27-009 — Publish capability SDK and inspection surface `P1`

Allow first-party/third-party capability packages without exposing engine internals.

**Depends on:** M27-008

**Acceptance:**
- SDK example compiles and passes lifecycle/cancel tests.
- velqu inspect shows dependencies, costs, and fallbacks.
- ABI stability classification is documented.

**Required evidence:**
- example capability
- SDK docs
- ABI tests
### - [ ] M27-GATE — Pass minimal Web runtime gate `P0`

Prove modularity, lifecycle safety, standards behavior, and cold-start budget.

**Depends on:** M27-009

**Acceptance:**
- M27-001 through M27-009 are PASS.
- Core app pays for no unused capability.
- Selected Test262/WPT/WinterTC matrix passes.
- Startup/RSS deltas are within approved budgets.

**Required evidence:**
- M2.7 report
- capability matrix
- benchmark evidence
- checkpoint archive

## M28 — M2.8 — Native Outbound Fetch

**Required outcome:** Velqu provides bounded, cancellable, pooled Web fetch with TLS/DNS correctness, streaming backpressure, and explicit SSRF policy.

### - [ ] M28-001 — Accept outbound fetch and SSRF security ADR `P0`

Freeze public semantics, security defaults, proxy behavior, and non-goals before implementation.

**Depends on:** M27-GATE

**Acceptance:**
- Web fetch is the public API; node:http/https are out of scope.
- Default network policy and trust boundary are explicit.
- Threat model covers DNS rebinding and redirects.

**Required evidence:**
- ADR
- threat-model update
- API sketch
### - [ ] M28-002 — Select native client stack from evidence `P0`

Compare reqwest versus Hyper-based composition for startup, binary size, pooling, cancellation, and streaming.

**Depends on:** M28-001

**Acceptance:**
- Decision uses matched benchmark and conformance data.
- Rejected alternative and tradeoffs are recorded.

**Required evidence:**
- A/B report
- dependency tree
- ADR addendum
### - [ ] M28-003 — Implement connection pooling, DNS, and TLS `P0`

Provide bounded keep-alive pools, DNS cache policy, secure roots, hostname validation, and lazy initialization.

**Depends on:** M28-002

**Acceptance:**
- No fetch capability cost when unused.
- Pool limits/timeouts are configurable and bounded.
- TLS verification cannot be disabled accidentally in production.

**Required evidence:**
- integration tests
- TLS fixtures
- startup/RSS delta
### - [ ] M28-004 — Implement Request/Response/Headers subset `P0`

Expose the required Web-compatible fetch objects without forcing full Web wrappers on inbound routes.

**Depends on:** M28-003

**Acceptance:**
- Header/body semantics and cloning rules are documented.
- Unsupported properties fail clearly.
- Large bodies remain streamed/bounded.

**Required evidence:**
- API conformance
- type tests
- memory tests
### - [ ] M28-005 — Propagate AbortSignal and route deadlines `P0`

Cancel DNS/connect/TLS/write/read and response-body operations when the invocation terminates.

**Depends on:** M28-004

**Acceptance:**
- Cancellation is physical and acknowledged.
- No native task, connection, listener, or Promise remains after quiescence.
- Timeout classification increments once.

**Required evidence:**
- race tests
- task metrics
- chaos tests
### - [ ] M28-006 — Implement streaming and strict backpressure `P0`

Bound request and response buffering across Rust/QuickJS boundaries.

**Depends on:** M28-005

**Acceptance:**
- Slow consumers/producers do not create unbounded queues.
- Reader cancellation releases sockets.
- Streaming route metrics expose buffered bytes.

**Required evidence:**
- stream tests
- slow peer fixtures
- memory profile
### - [ ] M28-007 — Implement redirect and compression policy `P0`

Handle redirects, method/body rewriting, credential stripping, and content decoding consistently.

**Depends on:** M28-006

**Acceptance:**
- Every redirect target is revalidated by SSRF policy.
- Maximum redirects and decompressed-size limits are enforced.
- Cross-origin sensitive headers are stripped per documented rules.

**Required evidence:**
- redirect matrix
- zip-bomb fixture
- security tests
### - [ ] M28-008 — Implement SSRF and network egress controls `P0`

Provide allow/deny policies for schemes, hosts, ports, private/link-local/loopback ranges, and DNS results.

**Depends on:** M28-007

**Acceptance:**
- DNS rebinding and redirect bypass fixtures fail.
- Policy can be configured per deployment/capability.
- Diagnostics redact credentials.

**Required evidence:**
- SSRF suite
- security report
- configuration docs
### - [ ] M28-009 — Complete fetch conformance, observability, and shutdown `P1`

Close Web behavior, pool metrics, graceful drain, source-mapped errors, and selected WPT cases.

**Depends on:** M28-008

**Acceptance:**
- All connections/tasks close on shutdown.
- Metrics show pool wait/connect/TLS/TTFB/body timings.
- Selected WPT and upstream fixtures pass.

**Required evidence:**
- WPT report
- shutdown tests
- operational metrics
### - [ ] M28-GATE — Pass native fetch gate `P0`

Prove useful, secure, cancellable backend I/O without sacrificing the cold-start thesis.

**Depends on:** M28-009

**Acceptance:**
- M28-001 through M28-009 are PASS.
- Controlled upstream matrix 1/5/10/25 ms passes at c=1/10/50/200.
- No unresolved SSRF/cancellation/backpressure P0/P1.
- Capability remains absent from apps that do not use fetch.

**Required evidence:**
- M2.8 report
- raw controlled-I/O data
- security review
- checkpoint archive

## M3 — M3 — Multi-Worker Service Runtime

**Required outcome:** Independent QuickJS workers scale across cores with bounded queues, quarantine/replacement, and distinct serverless/service/throughput profiles.

### - [ ] M3-001 — Freeze independent-worker state semantics `P0`

Document per-worker JS module state, shared native services, routing affinity, and no-JS-value-crossing rules.

**Depends on:** M28-GATE

**Acceptance:**
- One QuickJS runtime is owned by one thread.
- Application authors are warned that module state is per worker.
- Shared service handles expose thread-safe Rust APIs only.

**Required evidence:**
- ADR
- state semantics tests
- docs
### - [ ] M3-002 — Implement bounded worker dispatcher `P0`

Route compiled invocations to workers using bounded queues and measured load selection.

**Depends on:** M3-001

**Acceptance:**
- Queue capacity and rejection behavior are deterministic.
- Power-of-two or selected policy is benchmarked.
- Queue wait is observable.

**Required evidence:**
- dispatcher tests
- load-shedding fixtures
- benchmark
### - [ ] M3-003 — Implement runtime profiles `P0`

Provide serverless, service, and throughput profiles with distinct startup and scaling behavior.

**Depends on:** M3-002

**Acceptance:**
- Serverless starts exactly one worker.
- Service starts ready with one and may add workers.
- Throughput initializes configured workers before readiness.

**Required evidence:**
- profile tests
- cold/RSS report
- configuration docs
### - [ ] M3-004 — Implement deterministic worker initialization and artifact sharing `P0`

Load identical QPack/function/schema state per worker without mutable cross-worker JS state.

**Depends on:** M3-003

**Acceptance:**
- Worker load failures prevent routing to that worker.
- Read-only pack data is shared safely where possible.
- Per-worker memory delta is measured.

**Required evidence:**
- load tests
- memory report
- failure injection
### - [ ] M3-005 — Implement quarantine, replacement, and readiness aggregation `P0`

Remove poisoned workers from service and replace them without continuing semantically broken JS execution.

**Depends on:** M3-004

**Acceptance:**
- Poisoned worker receives no new work.
- Pending requests fail closed or drain according to policy.
- Replacement restores capacity; readiness reflects healthy worker count.

**Required evidence:**
- poison/recycle tests
- readiness tests
- chaos report
### - [ ] M3-006 — Implement adaptive scale-up and scale-down `P0`

Adjust worker count from queue pressure and utilization without oscillation or cold-request surprise.

**Depends on:** M3-005

**Acceptance:**
- Thresholds/hysteresis are documented.
- Scale-down waits for quiescence.
- Serverless profile never scales implicitly.

**Required evidence:**
- adaptive tests
- queue traces
- cold-to-warm benchmark
### - [ ] M3-007 — Implement multi-worker cancellation and graceful shutdown `P0`

Stop admission, drain or cancel requests, close capabilities, and join workers within a bounded deadline.

**Depends on:** M3-006

**Acceptance:**
- SIGTERM behavior is deterministic.
- No task/slot/connection leak after shutdown.
- Forced termination path is logged and bounded.

**Required evidence:**
- shutdown matrix
- task gauges
- container test
### - [ ] M3-008 — Add fairness and overload controls `P0`

Prevent slow/CPU-heavy requests from starving unrelated work and define 429/503 load shedding.

**Depends on:** M3-007

**Acceptance:**
- Per-worker queue, invocation, and native-op limits are enforced.
- Overload errors are typed/observable.
- Slow-request chaos does not deadlock all workers.

**Required evidence:**
- fairness tests
- overload benchmark
- error contract tests
### - [ ] M3-009 — Run scaling, memory, and soak evidence `P1`

Measure 1/2/4/8 workers with route, JSON, auth, controlled I/O, and mixed workloads.

**Depends on:** M3-008

**Acceptance:**
- 2 workers >= approved scale factor; 4 workers >= approved scale factor.
- Per-worker RSS is reported.
- 24-hour multi-worker soak has no monotonic leak or stuck queue.

**Required evidence:**
- raw scaling data
- soak logs
- memory graphs
### - [ ] M3-GATE — Pass multi-worker service gate `P0`

Demonstrate bounded scalable service mode while preserving single-worker serverless behavior.

**Depends on:** M3-009

**Acceptance:**
- M3-001 through M3-009 are PASS.
- Serverless cold-start budget is unchanged within tolerance.
- Quarantine/replacement, overload, and shutdown gates pass.
- No cross-worker JS ownership violation.

**Required evidence:**
- M3 report
- scaling/fairness evidence
- checkpoint archive

## M4 — M4 — Developer Experience and Private Alpha

**Required outcome:** The actual Rust/QuickJS runtime is pleasant to develop against, Treaty modes are complete, Linux release artifacts exist, and a realistic proof service is private-alpha ready.

### - [ ] M4-001 — Implement actual-runtime velqu dev loop `P1`

Use the real Rust/QuickJS runtime in development to prevent Bun-vs-production semantic drift.

**Depends on:** M3-GATE

**Acceptance:**
- File changes compile incrementally, load a candidate worker, then atomically switch.
- Failed reload leaves the last healthy app serving.
- Source maps and diagnostics point to TypeScript source.

**Required evidence:**
- dev-loop tests
- reload chaos
- developer walkthrough
### - [ ] M4-002 — Complete CLI command surface `P1`

Stabilize velqu dev/build/inspect/contract diff/test/package/doctor commands and exit codes.

**Depends on:** M4-001

**Acceptance:**
- Commands have deterministic machine-readable output.
- Invalid config fails before side effects.
- CLI completion/help docs are generated.

**Required evidence:**
- CLI conformance
- golden outputs
- docs
### - [ ] M4-003 — Implement project scaffolding `P1`

Create a minimal feature-based application with contracts, services, routes, Treaty tests, and deployment config.

**Depends on:** M4-002

**Acceptance:**
- Generated project builds and runs offline after dependency install.
- No demo secrets or unsafe production defaults.
- Template passes current verify.

**Required evidence:**
- create app fixture
- snapshot tests
- tutorial
### - [ ] M4-004 — Complete Treaty unit-local, runtime-local, and remote modes `P1`

Provide typed direct dispatch, real-runtime integration, and network use with identical contracts.

**Depends on:** M4-003

**Acceptance:**
- No public any.
- 2xx/non-2xx/transport errors narrow correctly.
- Unit-local is truly network-free and clearly labeled.

**Required evidence:**
- type-negative tests
- mode parity suite
- published contract fixture
### - [ ] M4-005 — Publish compact contract and SDK artifacts `P1`

Allow separate frontend repositories without importing server implementation.

**Depends on:** M4-004

**Acceptance:**
- Generated package is tree-shakable and versioned.
- Contract hash and mismatch diagnostics are available.
- 100/500/1,000-route TypeScript budgets pass.

**Required evidence:**
- declaration size/typecheck report
- consumer fixture
### - [ ] M4-006 — Finalize diagnostics, source maps, and inspect output `P1`

Make unsupported APIs, route plans, capabilities, fallbacks, performance strategies, and runtime failures actionable.

**Depends on:** M4-005

**Acceptance:**
- Every diagnostic has stable code and source location.
- Secrets remain redacted.
- inspect output matches QPack/runtime graph exactly.

**Required evidence:**
- diagnostic catalog
- redaction tests
- inspect golden files
### - [ ] M4-007 — Implement bounded defer and lifecycle hooks `P1`

Support explicit post-response work without pretending it is durable background processing.

**Depends on:** M4-006

**Acceptance:**
- defer has owner, timeout, queue cap, cancellation, and observability.
- Shutdown waits or cancels according to policy.
- Docs direct durable work to external queues.

**Required evidence:**
- defer tests
- lifecycle matrix
- docs
### - [ ] M4-008 — Build documentation and examples `P1`

Provide installation, contracts, policies, Treaty, capabilities, deployment, limitations, and migration material.

**Depends on:** M4-007

**Acceptance:**
- Docs are versioned and tested.
- Examples cover small API, auth, fetch, streaming, and multi-worker service.
- No unsupported Node compatibility implication.

**Required evidence:**
- docs link check
- example CI
- tutorial review
### - [ ] M4-009 — Build realistic private-alpha proof service `P1`

Exercise 30–50 routes, auth, pagination, typed errors, fetch, lifecycle, observability, and deployment.

**Depends on:** M4-008

**Acceptance:**
- Reference app uses actual runtime, not special benchmark bypasses.
- Operational runbook and load profile exist.
- Known limitations are explicit.

**Required evidence:**
- proof service source
- runtime-local tests
- deployment smoke test
### - [ ] M4-GATE — Pass private-alpha gate `P0`

Make Velqu usable by a small invited group without claiming public production readiness.

**Depends on:** M4-009

**Acceptance:**
- M4-001 through M4-009 are PASS.
- Linux x86_64 and arm64 artifacts pass smoke/conformance or owner-approved scope is recorded.
- No unresolved P0; P1s have explicit alpha disposition.
- Private-alpha docs and support channel exist.

**Required evidence:**
- M4 alpha report
- artifact matrix
- checkpoint archive

## M5 — M5 — Production Operations and Real-World Proof

**Required outcome:** Operational controls, observability, optional Postgres integration, executable cross-framework workloads, and a controlled production canary establish technical production candidacy.

### - [ ] M5-001 — Make the real-world benchmark harness executable `P1`

Turn SPEC/workloads/schema into reproducible containers, datasets, load generator, result schema, and report generator.

**Depends on:** M4-GATE

**Acceptance:**
- W1–W4 run from one command.
- Dataset reset is deterministic.
- CPU/RSS/pool wait/queue wait/p50/p95/p99/errors are captured.

**Required evidence:**
- docker compose
- seed/reset scripts
- raw result schema
### - [ ] M5-002 — Implement matched competitor candidates `P1`

Provide Raw Rust, Elysia 2, Hono/Bun, and Fastify/Node candidates with equivalent semantics.

**Depends on:** M5-001

**Acceptance:**
- Same SQL, schema, JWT, pool size, response bytes, logging, limits, and timeout behavior.
- Fairness audit passes before performance interpretation.

**Required evidence:**
- candidate source
- correctness fixtures
- fairness report
### - [ ] M5-003 — Implement controlled upstream and CPU/JIT crossover suites `P1`

Measure scheduler/I/O behavior separately from database-driver differences and identify cumulative cold-vs-hot crossover.

**Depends on:** M5-001

**Acceptance:**
- Delay matrix 0/1/5/10/25/50 ms and payload 256B/1K/16K/64K.
- CPU levels and first/10/100/1k/10k request phases are captured.
- Break-even analysis uses measured startup and per-request cost.

**Required evidence:**
- upstream fixture
- CPU workload source
- crossover report
### - [ ] M5-004 — Authorize and implement optional Postgres capability `P1`

Provide one official database capability after M2.7 lifecycle rules, without adding database code to core.

**Depends on:** M5-002

**Acceptance:**
- Pool is lazy, bounded, cancellable, and gracefully closed.
- Parameterized query API prevents accidental string interpolation.
- Capability absent from unrelated apps.

**Required evidence:**
- ADR
- capability package
- integration tests
### - [ ] M5-005 — Implement auth/JWT reference capability or policy package `P1`

Exercise real cryptographic verification and typed authorization without making auth a core product.

**Depends on:** M5-004

**Acceptance:**
- Pinned algorithms and key handling are secure by default.
- 401/403 distinction is typed.
- Key rotation and clock-skew behavior are documented.

**Required evidence:**
- package source
- security tests
- reference app integration
### - [ ] M5-006 — Implement production configuration and secret handling `P0`

Define files/env/CLI precedence, validation, secret redaction, immutable runtime config, and reload policy.

**Depends on:** M5-005

**Acceptance:**
- Invalid config fails before bind.
- Secrets never appear in logs/reports/crash output.
- Config schema is documented and versioned.

**Required evidence:**
- config tests
- redaction suite
- operator docs
### - [ ] M5-007 — Implement production observability `P1`

Provide structured logs, Prometheus/OpenTelemetry-compatible metrics, traces, request IDs, worker/queue/pool health, and sampling controls.

**Depends on:** M5-006

**Acceptance:**
- Observability can be disabled or sampled with measured overhead.
- Trace context propagates through fetch and DB capability.
- No full per-request JSON allocation when disabled.

**Required evidence:**
- OTel integration tests
- overhead benchmark
- dashboard examples
### - [ ] M5-008 — Implement trusted-proxy, drain, and deployment semantics `P0`

Make reverse-proxy headers, client IP, graceful SIGTERM, readiness, liveness, startup, and rolling deployment behavior explicit.

**Depends on:** M5-007

**Acceptance:**
- Trusted proxy ranges are configured, never assumed.
- Readiness goes false before drain.
- Container exits within configured deadline with no accepted-work loss beyond policy.

**Required evidence:**
- proxy tests
- Kubernetes/container smoke tests
- runbook
### - [ ] M5-009 — Run real-world load, leak, and canary evidence `P1`

Validate W1–W4, controlled I/O, CPU crossover, 1M+ requests, fault recovery, and one controlled production-like canary.

**Depends on:** M5-008

**Acceptance:**
- Raw evidence and environment manifests retained.
- No monotonic slot/task/pool/RSS leak.
- Negative or losing results remain documented.

**Required evidence:**
- real-world report
- canary report
- memory graphs
### - [ ] M5-GATE — Pass technical production candidate gate `P0`

Declare suitability for controlled production only after operations and real workloads pass.

**Depends on:** M5-009

**Acceptance:**
- M5-001 through M5-009 are PASS.
- No unresolved P0; production-affecting P1s have owner-approved disposition.
- Runbook, observability, rollback, and on-call contact exist for canary users.
- No public GA claim is made.

**Required evidence:**
- technical production candidate report
- checkpoint archive

## M6 — M6 — Security, Reliability, Platform, and Supply-Chain Hardening

**Required outcome:** The runtime passes fuzzing, sanitizers, chaos, soak, supported-platform, vulnerability, reproducibility, and artifact-integrity gates.

### - [ ] M6-001 — Update complete threat model and trust boundaries `P0`

Cover compiler, QPack, bytecode, FFI, capabilities, fetch, DB, multi-worker, supply chain, and operator configuration.

**Depends on:** M5-GATE

**Acceptance:**
- Assets/actors/entry points/abuse cases/controls are current.
- Same-process trusted-code limitation is prominent.
- Every high risk maps to a test/control.

**Required evidence:**
- threat model
- security traceability
### - [ ] M6-002 — Run sustained fuzz and property campaigns `P0`

Fuzz HTTP admission, router, QPack, schema codecs, bridge handles, Treaty encoders, fetch parsers, and capability metadata.

**Depends on:** M6-001

**Acceptance:**
- Configured campaign duration and corpus are recorded.
- No crash, panic, UB, or unbounded allocation remains.
- Findings become regression tests.

**Required evidence:**
- fuzz logs
- corpus bundle
- fix reports
### - [ ] M6-003 — Run sanitizers, Miri, concurrency, and unsafe audits `P0`

Exercise FFI and native lifetime invariants beyond normal tests.

**Depends on:** M6-002

**Acceptance:**
- All unsafe blocks have documented invariants.
- Applicable ASan/LSan/TSan/UBSan/Miri/Loom-style checks pass or have explicit platform waivers.
- No known memory-safety P0/P1.

**Required evidence:**
- sanitizer logs
- unsafe inventory
- FFI review
### - [ ] M6-004 — Establish dependency and license supply-chain policy `P0`

Pin, audit, allowlist, and monitor Rust/Bun dependencies and transitive licenses.

**Depends on:** M6-003

**Acceptance:**
- No unresolved critical/high exploitable advisory.
- License policy matches owner decision.
- Engine and binding upgrade lane is tested separately.

**Required evidence:**
- audit output
- license report
- dependency policy
### - [ ] M6-005 — Generate SBOM and provenance `P0`

Produce machine-readable components and build provenance for every release artifact.

**Depends on:** M6-004

**Acceptance:**
- SPDX/CycloneDX SBOM and provenance reference exact source/toolchains.
- Artifacts and metadata are hashed and later signed.
- Rebuild process is documented.

**Required evidence:**
- SBOM
- provenance statement
- verification script
### - [ ] M6-006 — Prove reproducible builds on independent builders `P0`

Build the same source on two clean supported environments and compare normalized outputs.

**Depends on:** M6-005

**Acceptance:**
- QPack and binaries reproduce or all allowed differences are explained.
- Build images/toolchains are pinned.
- Failure blocks RC.

**Required evidence:**
- independent build hashes
- difference report
### - [ ] M6-007 — Complete supported-platform matrix `P0`

Run build/conformance/performance smoke on owner-approved GA platforms.

**Depends on:** M6-006

**Acceptance:**
- At minimum Linux x86_64 and arm64 are decided and tested or narrowed by owner decision.
- Unsupported platforms fail/document clearly.
- Cross-compilation artifacts are exercised on real architecture.

**Required evidence:**
- CI matrix
- platform report
- artifact smoke tests
### - [ ] M6-008 — Run chaos and fault-injection program `P0`

Exercise worker poison, upstream/DB failure, DNS/TLS errors, queue saturation, OOM pressure, process signals, and corrupt artifacts.

**Depends on:** M6-007

**Acceptance:**
- No deadlock or silent auth bypass.
- Readiness/load shedding/rollback behave as documented.
- Recovery time and lost-work policy are measured.

**Required evidence:**
- chaos scenarios
- raw logs
- remediation report
### - [ ] M6-009 — Run long soak and performance-regression qualification `P0`

Prove stable memory/resources and pin benchmark budgets before RC.

**Depends on:** M6-008

**Acceptance:**
- 24-hour minimum and selected 72-hour soak pass.
- At least 10M representative requests or approved equivalent.
- No monotonic RSS/task/slot/connection growth beyond defined tolerance.
- Performance regressions block merge automatically.

**Required evidence:**
- soak report
- resource graphs
- CI regression thresholds
### - [ ] M6-GATE — Pass security and reliability hardening gate `P0`

Reach independently reviewable technical production readiness.

**Depends on:** M6-009

**Acceptance:**
- M6-001 through M6-009 are PASS.
- Independent security review has no unresolved critical/high issue.
- Supported platforms, reproducibility, SBOM, chaos, and soak gates pass.
- Incident response draft exists.

**Required evidence:**
- M6 hardening report
- security review
- checkpoint archive

## M7 — M7 — API/ABI Stabilization and Release Candidate

**Required outcome:** Public APIs, QPack/runtime/capability ABIs, SemVer behavior, migration rules, publishing automation, and RC canaries are stable.

### - [ ] M7-001 — Freeze public TypeScript API and Treaty semantics `P1`

Define stable package exports, error/result shapes, route declarations, schema behavior, and deprecation rules.

**Depends on:** M6-GATE

**Acceptance:**
- No public any or accidental internal types.
- API extractor/baseline detects breaking changes.
- SemVer classification fixtures pass.

**Required evidence:**
- API report
- type tests
- baseline files
### - [ ] M7-002 — Freeze runtime, QPack, and capability ABI policies `P1`

Define compatibility windows, migration tooling, engine fingerprint behavior, and deprecation support.

**Depends on:** M7-001

**Acceptance:**
- Old supported artifacts either load or fail with migration guidance.
- No silent bytecode/ABI fallback.
- Capability SDK has version negotiation rules.

**Required evidence:**
- compatibility matrix
- upgrade tests
- migration docs
### - [ ] M7-003 — Implement release and package publishing automation `P1`

Build, test, sign, publish, and verify @velqu packages, Rust binaries, QPack tools, checksums, SBOM, and provenance.

**Depends on:** M7-002

**Acceptance:**
- Dry-run and staging registry tests pass.
- Published packages install in clean consumers.
- Rollback/yank procedure is documented.

**Required evidence:**
- release workflow
- staging artifacts
- install tests
### - [ ] M7-004 — Complete versioned documentation and migration guides `P1`

Publish API reference, operator guide, capability authoring, troubleshooting, security model, performance methodology, and upgrade paths.

**Depends on:** M7-003

**Acceptance:**
- Docs match RC binaries/packages.
- All code samples run in CI.
- Known limitations and non-goals are prominent.

**Required evidence:**
- docs build
- sample tests
- migration walkthrough
### - [ ] M7-005 — Resolve owner-controlled public release decisions `P0`

Close repository/org, license, supported platform promise, governance, security contact, and release authority.

**Depends on:** M7-004

**Acceptance:**
- OD-003 through OD-006 are DECIDED.
- License headers/package metadata are updated.
- Security policy and code of conduct/governance documents exist as chosen.

**Required evidence:**
- owner decision record
- repository metadata
- license scan
### - [ ] M7-006 — Run RC compatibility and canary program `P1`

Exercise clean installs, upgrades, rollback, real reference apps, platform artifacts, and controlled production canaries.

**Depends on:** M7-005

**Acceptance:**
- At least two independent consumer projects pass.
- Upgrade from prior alpha/beta fixture succeeds or migration path is proven.
- No unresolved RC P0/P1.

**Required evidence:**
- RC canary reports
- consumer feedback
- upgrade logs
### - [ ] M7-007 — Freeze public benchmark and positioning statements `P1`

Approve only fixture-specific claims supported by current raw evidence.

**Depends on:** M7-006

**Acceptance:**
- Every claim names versions, environment, workload, artifacts, and p50/p95/p99.
- Cold start distinguishes engine/process/container/scale-to-zero.
- Negative and crossover results are disclosed.

**Required evidence:**
- claims register
- fairness audit
- owner approval
### - [ ] M7-GATE — Pass release-candidate gate `P0`

Produce a signed, installable, documented, supportable RC with stable APIs and no open release blockers.

**Depends on:** M7-007

**Acceptance:**
- M7-001 through M7-007 are PASS.
- All owner decisions required for publication are closed.
- RC artifacts install and roll back.
- No unresolved P0/P1; P2 backlog is published.

**Required evidence:**
- RC release packet
- signed artifacts
- checkpoint archive

## M8 — M8 — Production-Ready GA Gate

**Required outcome:** All technical and owner gates are closed, signed reproducible artifacts are released, operations and rollback are documented, and the production-readiness review is approved.

### - [ ] M8-001 — Conduct formal production-readiness review `P0`

Review architecture, security, reliability, operations, performance, compatibility, support, and rollback against the frozen gates.

**Depends on:** M7-GATE

**Acceptance:**
- Every gate has source/test/raw-evidence references.
- No unsupported claim or unexecuted required check is hidden.
- Reviewer findings are classified P0/P1/P2 under the frozen scope.

**Required evidence:**
- PRR document
- review index
- finding disposition
### - [ ] M8-002 — Finalize operational SLOs, alerts, and runbooks `P0`

Define supported service indicators and operator actions for startup, readiness, queues, workers, fetch, DB, errors, and resource use.

**Depends on:** M8-001

**Acceptance:**
- SLOs have measurement sources and alert thresholds.
- Runbooks cover common failure modes and rollback.
- Canary dashboards are validated.

**Required evidence:**
- SLO doc
- runbooks
- dashboard screenshots/config
### - [ ] M8-003 — Finalize release signing, rollback, and disaster recovery `P0`

Make every GA artifact verifiable and every release reversible.

**Depends on:** M8-002

**Acceptance:**
- Artifacts/checksums/SBOM/provenance are signed.
- Rollback to previous runtime/package is rehearsed.
- Compromised-key and bad-release procedures are documented.

**Required evidence:**
- signed release set
- rollback drill
- key policy
### - [ ] M8-004 — Publish GA artifacts and versioned documentation `P0`

Release stable packages, binaries, source, QPack tools, docs, examples, benchmark evidence, and support/security contacts.

**Depends on:** M8-003

**Acceptance:**
- Clean install and hello/reference app pass from public artifacts.
- All URLs and package metadata are final.
- Release notes list known limitations and compatibility.

**Required evidence:**
- GA artifacts
- install logs
- release notes
### - [ ] M8-005 — Execute post-release monitoring and response plan `P0`

Closely monitor first production users and define patch/release cadence.

**Depends on:** M8-004

**Acceptance:**
- 24-hour and 7-day review checkpoints exist.
- Issue triage severity and security response timelines are active.
- Performance/regression telemetry is reviewed.

**Required evidence:**
- post-release checklist
- triage board
- monitoring report
### - [ ] M8-GATE — Approve production-ready GA `P0`

Use “production ready” only after technical, owner, release, and operational gates are all satisfied.

**Depends on:** M8-005

**Acceptance:**
- M8-001 through M8-005 are PASS.
- No unresolved P0/P1.
- Production-readiness reviewer and project owner approve the final packet.
- Final source, Git history, artifacts, evidence, and checksums are archived.

**Required evidence:**
- final PRR approval
- GA release packet
- long-term archive
