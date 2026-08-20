---
type: Risk Register
title: Velqu Beta Risk Register
status: draft
tags:
- risk
- mitigation
- beta

---

# Beta Risk Register

| Risk | Severity | Trigger | Mitigation / decision |
|---|---|---|---|
| QuickJS interpreter loses sustained CPU workloads | P1/product | CPU/JIT crossover strongly favors Bun/JSC | Position beta for cold-start and I/O glue; move hot work native; do not claim universal speed |
| Capability scope destroys cold start/RSS | P0/product | Web/Postgres profile exceeds accepted budget | Compile/link only used capabilities; split profiles; defer expensive API |
| Numeric graph mismatch invokes wrong policy/handler | P0 | Manifest/router/schema inconsistency | Mandatory semantic manifest, execution hash, fail-before-ready verification |
| Lazy request bridge skips required validation | P0 | Wrong FieldNeeds/SchemaId | G0 exact equivalence before M2.4; differential/fuzz tests |
| Microtask/native operation escapes owner | P0 | Boundary job/task/slot remains | Existing scheduler invariants, worker-local ownership, quarantine |
| Fetch SSRF/TLS weakness | P0 | Private/metadata access or TLS bypass | Safe defaults, resolved-address checks, redirect revalidation, security ADR/tests |
| Multi-worker hides per-request overhead | P1 | Aggregate RPS rises but queue/p99/RSS collapse | Close single-worker gates first; report queue latency and per-worker memory |
| Postgres becomes core platform scope | P1/product | Core links driver/pool | Optional capability package after ABI; no ORM |
| Bun dev mode diverges from QuickJS production | P0/product | Works in dev, fails at runtime | Actual-runtime `velqu dev`; Bun-local only explicit unit mode |
| Benchmarks are not reproducible | P1/evidence | Single runs/stale reports | Randomized repetitions, raw-to-report generation, stale-report verifier |
| Public beta mistaken for production ready | P0/claim | Marketing/docs imply SLA/GA | Beta definition and limitation banner; owner-reviewed wording |
| Ecosystem too narrow for external users | P1/product | Alpha users cannot build normal service | Fetch, Postgres, JWT reference, proof app, developer feedback before beta |
