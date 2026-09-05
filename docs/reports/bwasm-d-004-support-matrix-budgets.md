# BWASM-D-004 — Support Matrix, Compatibility Claims, and Release Budgets

## Status: BLOCKED (owner decision record missing)

The deliverable set below is complete as **proposed** design material,
but this packet cannot truthfully close: its own acceptance criteria
require an owner decision record for budget/support ratification, and
none exists. Per the task's stop condition, the exact blocker is
recorded here and **issue #1183 stays open**.

### Exact blocker

- **Owner decision record absent.** The owner authorized preparing the
  four design decisions (Browser-WASM packet, standing instruction
  2026-09-05); that authorization is not acceptance of the proposed
  numbers, lanes, or policies. Every budget and lane classification in
  ADR-0039 and `evidence/budgets.json` is marked `proposed-unratified`.
- **Secondary blocker (ADR-0038 correction, tracked there):** the
  threat model's claim that handlers have no ambient fetch/storage
  authority is not enforceable by the runtime in trusted mode — a
  browser Worker exposes browser APIs regardless of the capability
  bridge. Ratifying security budgets and claims on top of an
  uncorrected threat model would compound the error.

## Deliverables (proposed state)

- **ADR-0039** (`docs/okf/decisions/0039-browser-support-matrix-compatibility-claims-and-release-budgets.md`)
  — status **proposed**, owner-acceptance **pending**. Corrected from
  the earlier draft, which overclaimed: lanes are
  `experimental-untested` (no browser-runtime CI exists; no lane may be
  called *tested* until BWASM-Q-002 evidence exists), the platform
  baseline is *proposed* and conditional (Cache/IndexedDB required only
  for offline/persistence capabilities), offline wording is conditional
  on cache population, rollback limits (KV schema/external side effects
  are not undone) are stated, and artifact-version compatibility no
  longer imports the native QuickJS engine fingerprint for
  browser-engine handlers. A **Ratification blockers** section lists
  all of this in-ADR.
- **Machine-readable budgets**: `evidence/budgets.json` — size +
  behavioral budgets, support lanes with promotion requirements,
  measurability prototype, known-limitations baseline; everything
  tagged `proposed-unratified`.
- **Measurability prototype** (procedure validation only, NOT
  performance evidence): a trivial std-linked wasm32 cdylib measures
  **1,474,978 bytes raw / 305,342 bytes gzip-9**
  (sha256 `5f6291a1…169d56`). Findings: std linkage dominates size;
  `wasm-opt` and `brotli` are absent on the measurement host, so
  current size numbers are proxies; latency/memory measurability is
  still unvalidated because no kernel or browser harness exists.
- **Known-limitations baseline**: embedded in `evidence/budgets.json`
  (seven items), including the Worker ambient-API limitation and the
  platform-eviction caveat for offline/persistence.

## Corrections to previously merged design packets

Recorded here because D-004's review surfaced them; the corrections
themselves ship in a follow-up packet touching the ADR files:

1. **ADR-0037/ADR-0038 frontmatter** recorded `owner-acceptance` that
   the evidence does not support. For ADR-0037 the *architecture
   invariant* (hybrid kernel/Worker, `fetch` boundary) is genuinely
   owner-specified verbatim in the packet; the remainder of the ADR
   text is agent-authored. ADR-0038's specifics (CSP baselines,
   preview-origin policy) are agent-authored without owner provenance.
   Both move to `status: proposed` with precise provenance until
   ratified.
2. **ADR-0038 §5 ambient-authority claim** — correction required (see
   blocker above): in trusted mode the capability bridge mediates
   *declared* calls but cannot *prevent* direct browser-API use by
   handler code; the ADR must separate platform-enforceable guarantees
   (origin, CSP `connect-src` backstop) from trusted-code conventions,
   and untrusted-preview isolation must be stated as purely a
   deployment posture.
3. Issue **#1182 (BWASM-D-003) should be reopened** — its PASS rested
   on the unsupported owner-acceptance claim. #1180 (BWASM-D-001)
   carries a correction comment instead: its architecture invariant has
   genuine owner provenance, but full-text ratification is pending.

## Gates (this worktree)

- `./scripts/validate-okf` — pass
- `./scripts/verify` — ALL PASS (netns; doc/evidence-only change, setup
  completed before the run)

## Commands run

- Measurability prototype: exact commands + hash in
  `evidence/budgets.json` (`rustc --target wasm32-unknown-unknown
  --crate-type cdylib -O`, `gzip -9`).
- No kernel code was written; no runtime behavior changed.

## Disclosure

Standing CI disclosure applies (zero-step verify workflows since
~#714); local gates are the acceptance basis.
