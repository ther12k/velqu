---
type: Architecture Decision Record
title: "ADR-0042 — GA Platform Promise: Linux x86_64 and aarch64 glibc"
status: accepted
owner-ratified: 2026-09-11 — the repository owner, in the GA-track authorization review (issues #1320/#1322 disposition)
tags:
- platform
- ga
- aarch64
---

# ADR-0042 — GA Platform Promise: Linux x86_64 and aarch64 glibc

## Status

Accepted. Owner-ratified 2026-09-11 in the GA-track authorization review
(issue #1322, verdict APPROVED with the platform set below quoted from
the owner's disposition). Extends OD-005 (beta platform promise:
Linux x86_64 glibc only) for the GA track. Closes the M6-007 scope
decision identified in `docs/production/GA-RECONCILIATION.md`.

## Context

The beta promise (OD-005, `docs/beta/governance/PLATFORM_SUPPORT.md`)
froze Linux x86_64 glibc as the only supported platform. The aarch64
`verify (ubuntu-24.04-arm)` CI lane has been green on every PR (full
canonical verify: Rust workspace, release builds, conformance,
packaging), so aarch64 is exercised but not promised. GA needs an
explicit, honest platform set.

## Decision

1. **GA native promise: Linux x86_64 glibc + Linux aarch64 glibc.**
2. **Not promised at GA:** macOS, Windows, musl-based distributions.
   They remain unverified/unsupported — no implicit claims. Adding any
   of them later is a new owner decision with qualification evidence;
   the matrix is not widened for presentation.
3. **aarch64 promotion requires (tracked in #1322):** packaging
   qualification (publishable tarballs build and install on aarch64),
   clean-install verification, representative conformance (the canonical
   `./scripts/verify` suite, which is the aarch64 CI lane), and bounded
   soak evidence on aarch64 hardware/runners. Promotion is recorded in
   `docs/production/BASELINE_AND_SCOPE.md` and the platform-support
   document when complete.
4. **Browser policy is unchanged by this ADR:** Chromium is the
   tested/required lane; Firefox and WebKit remain experimental
   allowed-failure CI lanes (ADR-0039 + the 2026-09-11 owner direction:
   green CI never promotes a lane; promotion requires an ADR-0039
   amendment). This ADR does not touch the browser matrix.

## Consequences

- Release engineering must produce and verify artifacts for both
  architectures at RC (compiler, runtime binary, npm packages — the npm
  packages are pure JS/TS + wasm assets and are architecture-neutral,
  but their clean-install is verified on both).
- CI keeps the aarch64 verify lane as required (it already is); soak and
  canary programs (#1319/#1321) must cover both promised platforms at
  their respective scales.
- Honest support boundary: bugs on non-promised platforms are
  out-of-support; reports are welcome, no SLA.

## Rejected alternatives

1. **Promise macOS/Windows/musl at GA** — rejected by the owner: no
   qualification evidence exists; a wider matrix presented without
   evidence is a dishonest claim.
2. **Keep aarch64 un-promised despite a permanently green lane** —
   rejected: the lane is exercised on every PR; with packaging,
   clean-install, conformance, and bounded soak evidence it qualifies.
3. **Fold the browser matrix into this ADR** — rejected: browser lane
   promotion is governed separately (ADR-0039 amendment path) and stays
   conservative.
