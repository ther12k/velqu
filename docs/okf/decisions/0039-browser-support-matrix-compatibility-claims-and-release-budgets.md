---
type: Architecture Decision Record
title: ADR-0039 Browser Support Matrix, Compatibility Claims, and Release Budgets
status: proposed
date: 2026-09-05
implements: BWASM-D-004 (support matrix and budgets), ADR-0037 (browser-wasm product contract), ADR-0038 (threat model), ADR-0023 (canonical hashing)
owner-acceptance: pending; authorization to prepare design documents is not acceptance of their proposed decisions.
---

# ADR-0039: Browser Support Matrix, Compatibility Claims, and Release Budgets

## Context

ADR-0037 and ADR-0038 describe the proposed browser contract and security
boundary; their recorded owner-acceptance claims need explicit confirmation; BWASM-D-002 measured the wasm32 baseline. Before any kernel
implementation merges, the program needs the evidence-bound support
matrix, the compatibility-claims policy, and normative budgets for
size/startup/latency/memory/offline/update — with measurement
procedures attached, so targets and measured results are never
conflated (AGENTS.md constraint 12).

**Everything numeric in this ADR is an unratified proposed target until the
Q-phase (BWASM-Q-002/Q-005) publishes measured evidence against these
procedures.** Budgets are revisable by ADR amendment if measurement
proves them wrong; silently missing a budget is not.

## Decision

### 1. Support matrix

| Lane | Classification | Basis |
|---|---|---|
| Chrome / Edge desktop (last 2 evergreen majors) | **experimental / untested** | Proposed qualification lane; no browser-runtime CI evidence exists yet |
| Firefox desktop (last 2 evergreen majors) | **experimental / untested** | Proposed qualification lane; no browser-runtime CI evidence exists yet |
| Safari desktop (last 2 majors) | **experimental / untested** | Proposed qualification lane; no browser-runtime CI evidence exists yet |
| Chrome Android (last 2 majors) | **experimental / untested** | Requires independent device qualification; desktop results do not establish mobile support |
| Safari iOS (last 2 majors) | **experimental** | Platform quirks (Worker/WASM memory ceilings, storage partitioning) are not exercised by CI; may be promoted with evidence |
| Older evergreen versions | **experimental** | May work; untested; no claims |
| Browsers without WebAssembly + Workers + ES2022 + streams | **out of scope** | Feature-detection must fail closed with an actionable diagnostic (BWASM-R-002) |

**Proposed web platform baseline (not yet qualified):** WebAssembly
(instantiation + streaming compile), Web Workers (module workers),
ES2022, structuredClone, Fetch/Streams, Cache Storage / IndexedDB (for
offline + persistence capabilities). Missing core execution features prevent startup. Cache Storage and IndexedDB
are required only for enabled offline and persistence capabilities; their
absence must not silently disable a declared requirement.

### 2. Compatibility-claims policy

- Public claims may use only the ADR-0037 §7 vocabulary
  (identical / adapted / unsupported / deployment-required), and an
  "identical" claim for a surface requires passing the BWASM-Q-001
  differential suite for that surface on all **tested** lanes.
- No universal performance claim is permitted; any performance
  statement must cite the raw samples and procedure that produced it.
- Engine parity is never claimed for the default Worker profile
  (ADR-0037 §8); `quickjs-wasm` parity claims require BWASM-X-001
  evidence and a recorded owner decision.
- Announcement wording follows the OD-BETA-008 precedent: factual,
  scope-bounded, no superlatives without raw evidence.

### 3. Normative budgets (targets until measured)

| Budget | Target (compressed transfer) | Rationale |
|---|---|---|
| Base WASM kernel (`q-browser-kernel` + glue-free core) | ≤ 500 KiB | Verification/routing/validation/authz only; no engine, no host |
| Runtime JS glue (`@velqu/browser-runtime`) | ≤ 50 KiB | Dispatcher, Worker lifecycle, artifact loader |
| Per-app handler bundle | ≤ 100 KiB | Generated TS handlers for a typical 24-route app |
| Optional capability bundles | ≤ 50 KiB each | Loaded only when the artifact declares the capability |
| **Total initial transfer** | ≤ 1 MiB | Base + glue + one app + declared capabilities, brotli-compressed, cached after first load |

| Budget | Target | Procedure anchor |
|---|---|---|
| Cold start → first ready | ≤ 2 s on mid-tier 2022 Android-class hardware, throttled 4G | Q-005 procedure §cold |
| Warm start (cached artifact) → first ready | ≤ 500 ms | Q-005 §warm |
| Kernel per-request overhead | ≤ 5 ms p50 added over in-browser handler execution, ≤ 15 ms p99 | Q-005 §latency, matched-candidate design |
| Memory: repeated lifecycle | 100 load/unload cycles: heap flat after warmup within run noise | Q-005 §leak (mirrors BETA-013 soak methodology) |
| Offline readiness | All **tested** lanes: offline-capable routes answer after cache population; network-dependent routes return declared unavailable problems | Q-007 external exercise |

Budget misses are release-blocking findings for the Q-gate, not silent
adjustments; a budget change requires an ADR amendment with the
measured evidence attached.

### 4. Offline, cache, update, and artifact-version compatibility

- **Offline**: artifact + runtime assets cached in Cache Storage keyed
  by canonical content hash (ADR-0023); the Service Worker adapter
  (BWASM-B-004) serves hash-addressed, integrity-verified artifacts;
  first-load-online is required. Offline behavior is conditional on cache
  availability and capability requirements; browsers may evict stored data.
- **Update**: whole-artifact swap to a new content hash; the running
  instance is never hot-patched mid-request; the new artifact boots a
  new kernel instance (ADR-0037 §4).
- **Rollback**: the previous hash remains cached and re-servable;
  rollback selects the prior verified hash if retained. Persistent KV schema
  changes and external side effects are not undone by artifact rollback;
  incompatible storage migrations must block automatic rollback.
- **Artifact-version compatibility**: the browser kernel enforces the
  fail-before-ready checks for the browser target, manifest version, kernel
  ABI, handler-bundle contract, and integrity digests. The hybrid profile
  must not require a native QuickJS engine fingerprint for browser-engine
  handlers. Exact browser artifact fields remain a design dependency; native
  QuickJS bytecode must never be silently accepted as a browser handler bundle.

### 5. Measurement procedures (frozen method outline)

- **Device classes**: desktop reference (4-core, 16 GiB) and mid-tier
  2022 Android-class (≤ 6 GB RAM); hardware named in every report.
- **Network shaping**: throttled 4G (20 Mbps down / 10 Mbps up, 100 ms
  RTT) for transfer numbers; loopback for latency numbers.
- **Repetition and statistics**: ≥ 30 samples per cell; p50/p95/p99
  plus raw per-sample retention; warmup excluded from steady-state
  cells but reported; matched candidates where a comparison is claimed.
- **Raw evidence**: every published number links its raw artifact
  directory and generator commit; claims without matched raw evidence
  are prohibited (AGENTS.md #12).

## Rejected alternatives

1. **"Evergreen means all recent browsers are supported"** — rejected:
   without real-browser CI evidence a lane cannot be `tested`; claims
   follow the lane classification, not optimism.
2. **Aspirational budgets without procedures** — rejected: a target
   nobody knows how to measure is marketing; every budget above binds
   to a procedure and a gate.
3. **Per-file hot patching of artifacts** — rejected: breaks the
   content-hash integrity model and the update coherence rules
   (ADR-0023/0026); whole-artifact swap only.
4. **iOS as `tested` without device CI** — rejected: classified
   `experimental` until device lanes exist; promotion requires evidence.

## Consequences

- BWASM-Q-002/Q-005 own the first measured verdicts against these
  budgets; BWASM-Q-007 owns the offline exercise; the BWASM-GATE cannot
  pass while any budget is unmeasured or missed.
- Browser-lane promotions (e.g. iOS) are evidence-driven ADR-index
  updates, not documentation edits.
- The 33 unregistered kernel/runtime/build issues remain unregistered
  until the owner crosses the design-freeze gate (ADR-0037 frontmatter
  note).

## Ratification blockers

- Owner acceptance of ADR-0037/0038/0039 is not evidenced by a generic
  instruction to continue implementation. Keep the design gate closed.
- ADR-0038 claims handlers have no ambient fetch/storage authority. A normal
  browser Worker exposes browser APIs; a kernel capability bridge alone
  cannot mediate direct calls. Security review must separate enforceable
  origin/CSP restrictions from trusted-handler conventions before ratification.
- No browser-runtime measurements exist. A measurement prototype, exact
  device/browser identities, machine-readable budgets, numerical memory/noise
  criteria, and an owner-approved waiver policy remain required by D-004.
- No production implementation or further issue registration is authorized
  by this draft. The program epic remains open beyond the design phase.
