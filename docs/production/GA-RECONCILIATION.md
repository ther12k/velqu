---
type: Reconciliation Record
title: GA Reconciliation — Beta + Browser-WASM Evidence Mapped to Production Requirements (M6/M7/M8)
status: active
date: 2026-09-11
baseline: a78459b
---

# GA Reconciliation Pass (M6/M7/M8)

**Purpose.** The owner review of 2026-09-11 directed a single reconciliation
pass instead of mechanically executing `TASKS.production.json` forward: map
the beta (ADR-0020) and Browser-WASM (BWASM) evidence that already exists
onto the M6/M7/M8 production requirements, mark what is genuinely satisfied,
and open work only for the real delta.

**Method.** Every M6/M7/M8 task was checked against committed, verifiable
evidence (reports, JSON evidence, scripts, CI lanes). Verdicts:

- **SATISFIED** — acceptance is met by existing committed evidence; what
  remains is the ledger `evidence_refs` binding (a mechanical, per-task
  follow-up, tracked as one issue, not per-task rework).
- **PARTIAL** — substantial evidence exists; a bounded delta closes it.
- **GAP** — no committed evidence; real work.

The ledger rebind (turning verdicts into `PASS` + `evidence_refs` in
`TASKS.production.json`) is deliberately **not** done in this pass: the
validator requires a concrete ancestor commit plus source/raw/report/test
references per task, which is exactly the follow-up issue's work.

---

## M6 — Security, Reliability, Platform, and Supply-Chain Hardening

| Task | Requirement (abridged) | Verdict | Existing evidence | Delta |
|---|---|---|---|---|
| M6-001 | Current threat model & trust boundaries | **SATISFIED (consolidation delta)** | `docs/reports/beta-009-c-threat-model-review.md` (beta boundary review); ADR-0035/0038 + `docs/browser-wasm/evidence/q-003/threat-model-verification.md` (browser); `docs/okf/decisions/0033`/`0034` (fetch/proxy) | One consolidated native+browser threat-model document so assets/actors/controls are current in a single place |
| M6-002 | Sustained fuzz & property campaigns | **GAP** | `docs/reports/beta-009-a-fuzz-suites.md` — deterministic property/corpus suites across q-pack/router/schema/bridge/http/capabilities, all green | Sustained, duration-configured campaigns (libFuzzer/cargo-fuzz or equivalent) with recorded corpus + findings→regression tests |
| M6-003 | Sanitizers, Miri, concurrency, unsafe audits | **GAP** | none committed (no Miri/ASan/TSan/LSan runs found) | Run sanitizer + Miri campaigns over the workspace; unsafe audit (rquickjs FFI surface) |
| M6-004 | Dependency & license supply-chain policy | **PARTIAL** | `docs/reports/beta-009-b-dependency-vulnerability-license.md` + `scripts/dependency-scan.sh` (fails closed on missing licenses); SBOM license coverage | Write the *policy*: admission rules, upgrade cadence, audit-triggering events (an ADR) |
| M6-005 | SBOM and provenance | **PARTIAL** | `docs/reports/beta-015-f-sbom.md` (CycloneDX, 277 components, commit-bound, deterministic); `docs/browser-wasm/evidence/q-008/sbom-browser-wasm.cdx.json` | Provenance statements (builder, toolchain, parameters) for release artifacts + documented rebuild process |
| M6-006 | Reproducible builds on independent builders | **PARTIAL** | `scripts/compare-builds` in canonical verify: QPack artifacts byte-identical across independent builders, every run; M26-007 reproducible release packs | Reproducibility of the **native binaries** (`velqu-runtime`) across two clean environments with pinned toolchains |
| M6-007 | Supported-platform matrix | **PARTIAL (owner scope decision)** | OD-005 froze Linux x86_64 glibc as the beta promise; `verify (ubuntu-24.04-arm, aarch64)` CI lane is green | Decide the GA matrix (add aarch64-glibc to the promise? more?); then qualification evidence per promised platform |
| M6-008 | Chaos and fault-injection | **SATISFIED (rerun delta)** | `docs/reports/m3-010-b-chaos.md`, `docs/reports/beta-009-d-chaos-tests.md` | None blocking; rerun at RC candidate for freshness |
| M6-009 | Long soak + perf-regression qualification | **GAP (partially covered)** | `docs/reports/beta-013-a/b` soak: 2.43M/4.4M requests, 30-min sustained, flat RSS; benchmark evidence infra exists | 24-hour minimum + selected 72-hour soak; automated perf-regression thresholds blocking merge |

**M6-GATE**: blocked by the two GAP rows (M6-002, M6-003) and the M6-009
long-soak delta; everything else is binding work.

---

## M7 — API/ABI Stabilization and Release Candidate

| Task | Requirement (abridged) | Verdict | Existing evidence | Delta |
|---|---|---|---|---|
| M7-001 | Freeze public TS API & Treaty semantics | **GAP (policy)** | Contract lock enforces drift-freedom today (`contract.lock.json` preserved in verify); beta makes no stability promise by design | Owner-ratified stability policy: what is frozen at RC, what carries a 1.x deprecation window (ADR) |
| M7-002 | Freeze runtime/QPack/capability ABI policies | **GAP (policy)** | QPack v2 + runtime fingerprint + engine-match (SEC-001) shipped and tested; capability ABI/semver ADRs 0028–0032 exist | The *compatibility promise* ADR: which breakages are acceptable post-GA and how they are versioned |
| M7-003 | Release & publishing automation | **PARTIAL** | `scripts/publish-beta.sh` (idempotent, `--only`, dry-run; #1315); `scripts/build-packages.ts`; yank/rollback rehearsed (`docs/reports/beta-011-e-yank-rollback.md`); clean-install verification (BETA-016) | Signing integration; staging-registry rehearsal; wiring into CI on tagged releases |
| M7-004 | Versioned docs & migration guides | **PARTIAL** | `docs/beta/*` (install, quickstart, troubleshooting, limits, architecture, CLI reference); BWASM Q-006 migration guide | Version-pinned docs surface; code samples executed in CI (samples exist but are not CI-run as docs) |
| M7-005 | Owner-controlled release decisions | **OWNER** | `docs/open-decisions.md` OD-001..010 all decided; GA-specific release decisions pending | Owner session at RC time |
| M7-006 | RC compatibility & canary program | **GAP** | Cleanroom exercises exist (BWASM Q-007; BETA-016) but no *canary* program | Define and run the RC canary: staged rollout to real consumers, upgrade/downgrade matrix |
| M7-007 | Freeze public benchmark statements | **SATISFIED (ratification delta)** | OD-009 decided + `docs/beta/governance/BENCHMARK_WORDING.md`; evidence-bound benchmarks (ADR-0012/0040) | Owner ratifies the GA wording revision at RC |

**M7-GATE**: the two freeze-policy ADRs (M7-001/002) and the canary program
(M7-006) are the real deltas; M7-003/004 need bounded finishing work.

---

## M8 — GA Approval and Operations

| Task | Requirement (abridged) | Verdict | Existing evidence | Delta |
|---|---|---|---|---|
| M8-001 | Formal production-readiness review | **GAP (by design)** | `scripts/release-packet` + REVIEW/EVIDENCE_INDEX machinery exists from the M23R2 program; `docs/production/REVIEW_PROTOCOL.md` + packet template written | Conduct the review itself at RC (the GA event, not earlier work) |
| M8-002 | SLOs, alerts, runbooks | **PARTIAL** | BETA-006 observability baseline (status snapshots, trace ids, redaction); BETA-008 ops (reverse proxy, drain, container example) | Define SLOs + alert thresholds; operator runbook consolidation |
| M8-003 | Signing, rollback, disaster recovery | **PARTIAL** | Yank/rollback rehearsed (beta-011-e); BWASM B-006 upgrade/rollback verified | Artifact **signing** (cosign or similar); DR drill documentation |
| M8-004 | Publish GA artifacts & versioned docs | **Sequencing** | Publishing machinery above | Executed at GA (owner action) |
| M8-005 | Post-release monitoring & response | **GAP** | none committed | Define the plan (who watches what, response ladder) |

---

## Ledger hygiene note (BASE / M23R2)

`TASKS.production.json` still carries `BASE-001..005`, `BASE-GATE`, and
`M23R2-001..009`, `M23R2-GATE` as `IN_PROGRESS`. The corresponding work was
completed and gated under the beta program as G0 (`G0-001..009`, `G0-GATE` —
PASS, frozen numeric RoutePlan baseline) — the ledger rows are stale, not the
work. Rebinding them into `PASS` with `evidence_refs` (concrete ancestor
commits + source/raw/report/test paths, per `scripts/validate-production-plan`)
is tracked as a dedicated follow-up issue; this pass does not touch statuses.

---

## Bottom line

**The delta is small and concrete — not 20 tasks.** Real new work:

1. Sustained fuzz + sanitizer/Miri campaigns (M6-002/M6-003).
2. 24h/72h soak + merge-blocking perf-regression thresholds (M6-009).
3. Freeze-policy ADRs for API/ABI stability (M7-001/M7-002; owner-ratified).
4. RC canary program + SLO/alert definitions + artifact signing
   (M7-006, M8-002 delta, M8-003 delta).
5. Platform-matrix GA decision (M6-007; owner scope decision).

Everything else is evidence binding (mechanical), consolidation (one
threat-model doc), bounded finishing work, or owner-session items. Delta
issues are filed from this document's follow-ups section.
