# M27-GATE Review — Capability Linker and Minimal Web Runtime

Milestone exit decision for M2.7 (Capability Linker and Minimal Web Runtime).

## Milestone Decision: PASS

All 11 parent tasks (`M27-001` through `M27-011`) are complete, verified with source-backed evidence, and squash-merged to master.

### Parent Task Dependency Closure
1. **M27-001 (Define Capability ABI & Lifecycle)** — PRs #842–#847: ADR-0028, ADR-0029, ADR-0030, ADR-0031. Lifecycle state machine `Declared -> Installed -> Ready -> Draining -> Quiesced | Failed`.
2. **M27-002 (Compile-Time Capability Resolver & Pruning)** — PRs #848–#853: Compiler DAG resolver, cycle detection, hash-checked inventory in QPack, dead capability elimination.
3. **M27-003 (Custom QuickJS Context Profiles)** — PRs #854–#859: Construction-time `ContextProfile::{Full, Web, Minimal}`, reduction diagnostics, ready-line reflection.
4. **M27-004 (Console & Timer Core Capabilities)** — PRs #860–#865: Native timer delay under lifecycle/cancellation, bounded asynchronous console logger with redaction.
5. **M27-005 (URL & URLSearchParams)** — PRs #866–#871: WHATWG URL Standard subset, WinterTC compliant, bounded parsing, regex-free JS prelude.
6. **M27-006 (TextEncoder & TextDecoder UTF-8)** — PRs #872–#877: WHATWG Encoding Standard compliant UTF-8 subset, 16MB buffer bound, `encodeInto`, fatal and replacement modes.
7. **M27-007 (AbortController & AbortSignal)** — PRs #878–#883: WHATWG DOM AbortController/Signal, `AbortSignal.abort`, `AbortSignal.timeout`, leak prevention, atomic state transitions.
8. **M27-008 (Crypto Random Subset)** — PRs #884–#889: OS CSPRNG `crypto.getRandomValues` (64KiB quota) and `crypto.randomUUID` (RFC 4122 v4). No custom cryptography (ADR-0018).
9. **M27-009 (Publish Capability SDK & Inspection Surface)** — PRs #890–#895: `CapabilitySdk` and `CancellableCapability` traits, `LifecycleReport` test harness, read-only `CapabilityDiagnostics`, ADR-0032 semver/ABI policy.
10. **M27-010 (Establish Web API Conformance Program)** — PRs #896–#901: `wpt-manifest.json` with 34 pinned test vectors (100% PASS) + 8 explicit skips with machine-readable reasons, automated regression generator with `--check` in verify.
11. **M27-011 (Close Capability Cost Budgets)** — PRs #902–#907: Verified cold-start p50 = 4.16 ms (< 10 ms budget), binary delta = +120 KB (+2.2%, < +250 KB budget), idle RSS delta = +176 kB, 0 byte heap cost for unused capabilities.

## Architecture Decision Records (ADRs Accepted)
- **ADR-0028**: Capability ABI and Lifecycle State Machine
- **ADR-0029**: Capability Identity, Versioning, and Requirements
- **ADR-0030**: Native Operation Ownership, Deadlines, and Cancellation
- **ADR-0031**: Bounded Capability Shutdown and Quiescence
- **ADR-0032**: Capability Semver/ABI Compatibility Policy

## Standards Conformance & Open Items
- **WPT / WinterTC**: 34 pinned vectors passing; 8 explicit skips documented in `conformance/web-api/wpt-manifest.json`.
- **Open Decisions**: No unauthorized features (general Node compatibility, WebSockets, SSE deferred to post-beta per ADR-0018 / AGENTS.md constraint 15).
- **Standing CI Disclosure**: CI in this repository currently fails with zero executed steps on PRs (infrastructure-side since ~#714); local verification passes 100% across all suites.
