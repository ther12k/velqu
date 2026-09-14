---
type: Architecture Decision Record
title: "ADR-0043 — Dependency and License Supply-Chain Policy"
status: accepted
date: 2026-09-14
implements: M6-004 (production ledger), OD-004 (license decision), ADR-0033 §6 (pinned TLS roots), ADR-0003/ADR-0018 (pinned engine and binding)
---

# ADR-0043 — Dependency and License Supply-Chain Policy

## Context

The GA reconciliation (`docs/production/GA-RECONCILIATION.md`, M6-004) found
the *mechanics* of dependency hygiene in place and green —
`scripts/dependency-scan.sh` produces a reproducible license inventory that
fails closed on missing external license metadata
(`docs/reports/beta-009-b-dependency-vulnerability-license.md`,
verdict `PASS_WITH_DISCLOSURE`); the network stack is pinned to a minimal
feature set with an existing upgrade rule and CVE reviewer checklist
(`docs/reports/m28-002-d-maintenance-security.md`); the engine and binding
are pinned (quickjs-ng 0.15.1 via rquickjs =0.12.2, AGENTS.md constraint 1);
`Cargo.lock` and `bun.lock` are committed and CI installs frozen. What was
missing is the *policy*: admission rules, upgrade cadence, and
audit-triggering events, stated normatively in one place.

This ADR codifies existing practice as policy. It introduces no new tooling
mandate and changes no dependency.

## Decision

### 1. License admission: allowlist, fail closed

An external resolved dependency (Rust crate or npm package) is admissible
only under a permissive license. The allowlist matches the tree as it
stands (beta-009-b scan): MIT, Apache-2.0, BSD-2-Clause, BSD-3-Clause, ISC,
Zlib, BSL-1.0, CC0-1.0, Unlicense, Unicode-3.0/Unicode-DFS-2016,
CDLA-Permissive-2.0 (the Mozilla root-store data license — required by
ADR-0033 §6's webpki-roots mandate), `Apache-2.0 WITH LLVM-exception`, and
explicit public-domain dedications.

For multi-license expressions, an expression is admissible when at least one
**selectable** option is on the allowlist (the consumer may choose it) —
e.g. `MIT OR Apache-2.0 OR LGPL-2.1-or-later` is admissible via its MIT
option; a sole `GPL-3.0-only` is not. Copyleft as the **only** option
(GPL/AGPL/LGPL/SSPL and alike) is **not** admissible without a recorded
owner decision in `docs/open-decisions.md`. Missing or ambiguous license
metadata on an **external** package fails the inventory
(`scripts/dependency-scan.sh` verdict
`BLOCKED_MISSING_EXTERNAL_LICENSES`) — it is never silently tolerated.

The workspace posture stays `UNLICENSED-BEFORE-OWNER-DECISION` in
`Cargo.toml` until the RC publishing packet (M7-005) syncs package metadata
to the owner's MIT decision (OD-004, `LICENSE`); npm packages already carry
`"license": "MIT"`. The field flip is an RC-time publishing step, not an
ad-hoc edit.

### 2. Advisories: review rule and RC blocking

- Any RUSTSEC/GHSA/CVE advisory touching a resolved dependency is reviewed
  against the existing checklist (affected-code reachability, exploitability
  in Velqu's configured feature set, fix version) with the verdict recorded
  in the packet that resolves it.
- An **unresolved critical or high advisory that is exploitable in the
  configured feature set blocks the RC** (M6-004 acceptance). Lower
  severities or unreachable code paths are dispositioned in writing, never
  ignored.
- **Disclosure discipline**: a scan that could not run (no advisory database,
  no network, scanner absent) is recorded as `PASS_WITH_DISCLOSURE` or
  `NOT_RUN` — never as a clean PASS. Absence of evidence is never evidence
  of absence in committed reports (beta-009-b precedent).

### 3. Upgrade cadence

| Object | Rule |
| --- | --- |
| Engine + binding (quickjs-ng / rquickjs) | **Pinned exactly** (AGENTS.md constraint 1). Any bump is an ADR-level change executed as a dedicated packet that re-runs the fuzz, sanitizer, and Miri campaigns plus conformance, and reports cold-start/throughput deltas against the pinned baseline. Never swept in a lockfile refresh. |
| Toolchain (rustc, bun) | Pinned (`rust-toolchain.toml`, CI setup versions). Bumps are dedicated packets running full `scripts/verify`. |
| Network/TLS/crypto stack (hyper, rustls, ring, and equivalents) | Upgrades in dedicated packets re-running full verification and the fetch conformance suite, per the m28-002-d upgrade rule; advisory-triggered upgrades follow §2. |
| Everything else | Reviewed at each RC/candidate via the committed scan report; routine lockfile refreshes are ordinary packets with normal verify. |

### 4. Audit-triggering events

An out-of-cycle dependency audit (scan + advisory review, committed report)
is mandatory when any of these occurs:

1. a security advisory lands against any resolved dependency (§2 checklist);
2. a dependency is **added or bumped into a new capability class** — first
   build-script or proc-macro, first network I/O, first `unsafe`, first
   ambient-environment read;
3. the engine, binding, or toolchain pin moves (§3);
4. a first-party package is published or re-published;
5. the RC candidate is frozen (full audit as part of the M8-001 review).

### 5. Verification lanes

- `scripts/dependency-scan.sh` is the canonical reproducible inventory; its
  machine-readable report is regenerated and committed at every RC and at
  any §4 trigger. It fails closed on missing licenses by construction.
- CI installs with frozen lockfiles (`Cargo.lock`, `bun.lock`) only; no lane
  resolves floating versions.
- The SBOM (beta-015-f, CycloneDX, commit-bound) is regenerated at RC and
  any §4 trigger; the M6-005 provenance statements carry the builder,
  toolchain, and parameter identity.

## Threat model (supply-chain)

| Threat | Mitigation (section) |
| --- | --- |
| Copyleft or unknown-license dependency contaminates distribution | §1 allowlist, fail-closed inventory |
| Exploitable advisory ships at GA | §2 RC blocking rule + reviewer checklist |
| False confidence from skipped scans | §2 disclosure discipline (`PASS_WITH_DISCLOSURE` ≠ PASS) |
| Engine swap via a routine dependency bump | §3 exact-pin + dedicated-packet rule |
| Transitive capability creep (build script, network, `unsafe`) | §4.2 mandatory out-of-cycle audit |
| Drift between lockfile and published artifacts | §5 frozen-lockfile CI + commit-bound SBOM |

## Non-goals

- Vendoring or a monorepo fork of dependencies.
- Mandating a specific scanner (cargo-audit/cargo-deny/osv-scanner) — the
  policy requires *a* scan with disclosed method, not a named tool.
- Renaming, repository, or marketing metadata (owner decisions, AGENTS.md
  constraint 13; OD-004 license decision referenced, not changed).
- `cargo deny` configuration files or CI advisory lanes beyond the existing
  scan script (may be added by a future packet if the owner wants them).

## Consequences

- M6-004's "dependency policy" evidence is this ADR plus the committed scan
  report it mandates; "audit output" and "license report" cite
  `docs/reports/beta-009-b-*` (and its RC-time successor).
- New dependencies need a packet that notes the license and capability class
  — cheap when clean, blocking when it should be.
- The RC freeze (§4.5) gives the M8-001 production-readiness review a fresh,
  committed dependency audit to consume.

## Status

Accepted (M6-004 policy packet). Inventory tool:
`scripts/dependency-scan.sh`. Baseline report:
`docs/reports/beta-009-b-dependency-vulnerability-license.md`.
