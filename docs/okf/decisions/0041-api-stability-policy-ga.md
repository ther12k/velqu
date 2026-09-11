---
type: Architecture Decision Record
title: "ADR-0041 — Public API Stability Policy for GA (TypeScript/Treaty Surface)"
status: accepted
owner-ratified: 2026-09-11 — the repository owner, in the GA-track authorization review (issues #1320/#1322 disposition)
tags:
- api-stability
- ga
- semver
---

# ADR-0041 — Public API Stability Policy for GA

## Status

Accepted. Owner-ratified 2026-09-11 in the GA-track authorization review
(issue #1320, verdict APPROVED with the policy below quoted from the
owner's disposition). Closes the M7-001 policy gap identified in
`docs/production/GA-RECONCILIATION.md`.

## Context

The beta (`0.1.0-beta.1`, ADR-0020) deliberately makes no stability
promise. GA requires a ratified stability policy for the public
TypeScript surface (`@velqu/*` packages: core, schema, contract, treaty,
compiler, cli, browser-runtime, browser-pglite). The mechanisms that
make a policy enforceable already exist and are tested: the contract
lock (drift-freedom), generated `contract.d.ts`, and the packaging
qualification suite.

## Decision

1. **Freeze point.** The public GA API freezes at the RC tag. From RC,
   every **documented public export** of every package shipped under the
   GA promise is stable. Internal and experimental surfaces are excluded
   **only** when explicitly labeled (internal modules, `experimental`
   documentation markers, or underscore-prefixed runtime internals).
2. **Semver contract across 1.x.**
   - **Breaking** (requires 2.0): removal of a documented export;
     incompatible signature change; a change to documented semantics of
     an existing export.
   - **Minor** (allowed in 1.x): additive API (new exports, new optional
     parameters, new schema/contract surfaces).
   - **Patch** (allowed in 1.x): bug fixes that are compatible with the
     documented semantics.
3. **Deprecation.** Deprecating a documented export is allowed in 1.x
   (documented notice + replacement pointer). **Removal during 1.x is
   prohibited** — deprecated exports are removed only in 2.0. Because
   removal cannot happen in 1.x, no time-boxed deprecation window
   (e.g. "90 days") is defined; the removal event itself is the window's
   end.
4. **QPack / runtime / engine binding is explicitly NOT covered by API
   semver.** No cross-build portability is promised: the exact
   runtime/engine fingerprint binding and fail-closed mismatch behavior
   (SEC-001, ADR-0014/0026) remain the contract. A pack runs only on the
   exact runtime build it was compiled against, at every version.
5. **Capability ABI** keeps the existing versioned compatibility rules
   (ADR-0028–0032); this ADR does not change them.

## Enforcement

- `contract.lock.json` preservation in `scripts/verify` fails CI on
  undocumented contract drift.
- `packages/publishing/src/publishing.test.ts` fails packaging drift.
- Review rule: any PR that removes or re-semantics a documented export
  must carry a `breaking:` commit/PR tag and targets the 2.0 milestone;
  the release tooling publishes 1.x under `beta` until GA, `latest`
  movement remains owner-gated (OD-050/OD-010).

## Consequences

- Consumers can adopt at GA with upgrade confidence bounded by semver.
- Additive evolution stays cheap (minor); the cost of mistakes is
  concentrated at 2.0 boundaries.
- The engine-fingerprint fail-closed posture is preserved verbatim — no
  hidden compatibility promise is created by this ADR.

## Rejected alternatives

1. **Time-boxed deprecation windows (e.g. 90 days) inside 1.x** —
   rejected by the owner: since removal cannot happen in 1.x anyway, a
   day-count adds ambiguity without adding guarantees.
2. **"Everything documented or not is frozen"** — rejected: it would
   freeze internal/experimental surfaces that were never claimed.
3. **Extending semver coverage to QPack portability** — rejected: it
   contradicts the fingerprint-binding security contract (SEC-001).
