# Browser-WASM Design Freeze — Ratification Report

## Outcome

The Browser-WASM design freeze is **complete and owner-ratified**
(OD-BWASM-001, 2026-09-05): ADR-0037, ADR-0038, and ADR-0039 are
`accepted`. This closes the design phase (BWASM-D-001..004) and opens
the K-phase (kernel implementation) under the owner's phased
registration decision.

## Decision record

`docs/codex-spark-browser-wasm/evidence/design-freeze-owner-decision.md`
— decider (repository owner), method (explicit options in an
interactive review session, no implied acceptance), four decisions,
consequences, alternatives not selected, and the honesty history.

## What was ratified

| ADR | Decision | Key content |
|---|---|---|
| 0037 — product/runtime contract | accepted (remainder confirmed) | hybrid kernel/Worker architecture (invariant owner-specified verbatim), `fetch(Request)` boundary, semantics vocabulary, capability classes, crate boundaries |
| 0038 — threat model | accepted **as corrected** | actors/boundaries; trusted vs untrusted-preview modes; ambient-API honesty: capability bridge cannot prevent direct browser-API calls by trusted handlers — platform backstops (origin, CSP) vs conventions separated |
| 0039 — support matrix + budgets | accepted **as targets** | numeric budgets normative, revisable only by amendment with measured evidence; lanes experimental-untested until BWASM-Q-002; measurement procedures frozen |

## Gate state

- BWASM-D-001..004: **PASS** (D-003/D-004 blockers resolved by
  OD-BWASM-001).
- K-phase registration authorized: **six kernel issues next**
  (BWASM-K-001..006), via `scripts/browser-wasm/create_github_issues.py
  --phase 02_kernel`.
- 27 later-phase issues remain unregistered per the phased decision.

## Verification (this worktree)

- `./scripts/validate-okf` — pass
- `./scripts/verify` — ALL PASS (netns; setup completed before verify)

## Disclosure

Standing CI disclosure applies (zero-step verify workflows since
~#714); local gates are the acceptance basis.
